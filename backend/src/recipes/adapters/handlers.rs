use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::recipes::domain::{
    LlmError, LlmService, Recipe, RecipeRepository, RecipeShareRepository, RepositoryError,
};
use crate::shared::auth::AuthenticatedUser;

use super::dto::{
    CreateShareRequest, GenerateRecipeRequest, GeneratedRecipeResponse, RecipeResponse,
    SaveRecipeRequest, ShareResponse,
};
use super::extractors::ValidatedJson;
use super::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

fn map_repo_error(e: RepositoryError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, message) = match e {
        RepositoryError::NotFound => (StatusCode::NOT_FOUND, "Recipe not found"),
        RepositoryError::AccessDenied => (StatusCode::FORBIDDEN, "Access denied"),
        RepositoryError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
    };
    (
        status,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}

fn map_llm_error(e: LlmError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, user_message, log_message) = match &e {
        LlmError::ApiError(msg) => (
            StatusCode::BAD_GATEWAY,
            "Failed to reach AI service. Please try again later.",
            format!("AI API error: {}", msg),
        ),
        LlmError::ParseError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to process AI response. Please try again.",
            format!("AI response parse error: {}", msg),
        ),
    };
    eprintln!("Error: {}", log_message);
    (
        status,
        Json(ErrorResponse {
            error: user_message.to_string(),
        }),
    )
}

fn map_auth_lookup_error(e: String) -> (StatusCode, Json<ErrorResponse>) {
    eprintln!("Failed to lookup user by email: {}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: "Failed to lookup user".to_string(),
        }),
    )
}

#[utoipa::path(
    post,
    path = "/api/recipes/generate",
    summary = "Generate a recipe using AI",
    description = "Generates a recipe based on provided ingredients and optional dietary restrictions. Uses AI to create a complete recipe with instructions, prep time, cook time, and serving size.",
    request_body = GenerateRecipeRequest,
    responses(
        (status = 200, description = "Recipe generated successfully", body = GeneratedRecipeResponse),
        (status = 400, description = "Invalid request - ingredients list is empty or invalid", body = ErrorResponse),
        (status = 502, description = "AI service error - failed to reach or process AI service", body = ErrorResponse),
    ),
    tag = "Recipes"
)]
pub async fn generate_recipe<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    ValidatedJson(request): ValidatedJson<GenerateRecipeRequest>,
) -> Result<Json<GeneratedRecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state
        .generate_use_case
        .execute(request.ingredients, request.dietary_restrictions)
        .await
        .map_err(map_llm_error)?;

    Ok(Json(recipe.into()))
}

#[utoipa::path(
    post,
    path = "/api/recipes",
    summary = "Save a recipe",
    description = "Saves a recipe to the user's collection. The recipe will be associated with the authenticated user and visible in 'My Recipes'.",
    request_body = SaveRecipeRequest,
    responses(
        (status = 200, description = "Recipe saved successfully", body = RecipeResponse),
        (status = 400, description = "Invalid request - missing required fields or invalid data", body = ErrorResponse),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 500, description = "Database error - failed to save recipe", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Recipes"
)]
pub async fn save_recipe<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    ValidatedJson(request): ValidatedJson<SaveRecipeRequest>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = Recipe::from_generated(request.into(), user.user_id);

    state
        .save_use_case
        .execute(recipe.clone())
        .await
        .map_err(map_repo_error)?;

    Ok(Json(recipe.into()))
}

#[utoipa::path(
    get,
    path = "/api/recipes/{id}",
    summary = "Get a recipe by ID",
    description = "Retrieves a recipe by its ID. The user must be the owner of the recipe or have the recipe shared with them.",
    params(
        ("id" = Uuid, Path, description = "Recipe UUID")
    ),
    responses(
        (status = 200, description = "Recipe retrieved successfully", body = RecipeResponse),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 403, description = "Access denied - user does not have permission to view this recipe", body = ErrorResponse),
        (status = 404, description = "Recipe not found", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Recipes"
)]
pub async fn get_recipe<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state
        .get_use_case
        .execute(id, &user.user_id)
        .await
        .map_err(map_repo_error)?;

    let mut response: RecipeResponse = recipe.into();

    if response.owner_id != user.user_id {
        if let Ok(Some(email)) = crate::shared::auth::get_user_email_by_id(&response.owner_id).await
        {
            response = response.with_owner_email(Some(email));
        }
    }

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/recipes",
    summary = "List user's recipes",
    description = "Returns all recipes owned by the authenticated user, ordered by creation date (newest first).",
    responses(
        (status = 200, description = "List of user's recipes", body = [RecipeResponse]),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 500, description = "Database error - failed to retrieve recipes", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Recipes"
)]
pub async fn list_my_recipes<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<RecipeResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipes = state
        .list_owned_use_case
        .execute(&user.user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(Json(recipes.into_iter().map(|r| r.into()).collect()))
}

#[utoipa::path(
    get,
    path = "/api/recipes/shared",
    summary = "List recipes shared with user",
    description = "Returns all recipes that have been shared with the authenticated user by other users, ordered by share date (newest first).",
    responses(
        (status = 200, description = "List of recipes shared with user", body = [RecipeResponse]),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 500, description = "Database error - failed to retrieve shared recipes", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Recipes"
)]
pub async fn list_shared_recipes<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<RecipeResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipes = state
        .list_shared_use_case
        .execute(&user.user_id)
        .await
        .map_err(map_repo_error)?;

    let mut responses: Vec<RecipeResponse> = Vec::new();
    for recipe in recipes {
        let mut response: RecipeResponse = recipe.into();
        if let Ok(Some(email)) = crate::shared::auth::get_user_email_by_id(&response.owner_id).await
        {
            response = response.with_owner_email(Some(email));
        }
        responses.push(response);
    }

    Ok(Json(responses))
}

#[utoipa::path(
    post,
    path = "/api/recipes/{id}/shares",
    summary = "Share a recipe with another user",
    description = "Shares a recipe with another user by their email address. Only the recipe owner can share their recipes.",
    params(
        ("id" = Uuid, Path, description = "Recipe UUID")
    ),
    request_body = CreateShareRequest,
    responses(
        (status = 201, description = "Recipe shared successfully"),
        (status = 400, description = "Invalid request - invalid email format", body = ErrorResponse),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 403, description = "Access denied - user is not the owner of this recipe", body = ErrorResponse),
        (status = 404, description = "Recipe or user not found - recipe doesn't exist or user with email not found", body = ErrorResponse),
        (status = 500, description = "Database error - failed to create share", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Sharing"
)]
pub async fn create_share<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    Path(recipe_id): Path<Uuid>,
    ValidatedJson(request): ValidatedJson<CreateShareRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let user_id = crate::shared::auth::get_user_id_by_email(&request.email)
        .await
        .map_err(map_auth_lookup_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "User with this email not found".to_string(),
                }),
            )
        })?;

    state
        .create_share_use_case
        .execute(recipe_id, &user.user_id, user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    delete,
    path = "/api/recipes/{recipe_id}/shares/{user_id}",
    summary = "Unshare a recipe",
    description = "Removes sharing access for a specific user. Only the recipe owner can unshare their recipes.",
    params(
        ("recipe_id" = Uuid, Path, description = "Recipe UUID"),
        ("user_id" = String, Path, description = "User ID to remove sharing access from")
    ),
    responses(
        (status = 204, description = "Share removed successfully"),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 403, description = "Access denied - user is not the owner of this recipe", body = ErrorResponse),
        (status = 404, description = "Share not found - recipe is not shared with this user", body = ErrorResponse),
        (status = 500, description = "Database error - failed to remove share", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Sharing"
)]
pub async fn delete_share<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    Path((recipe_id, shared_user_id)): Path<(Uuid, String)>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state
        .delete_share_use_case
        .execute(recipe_id, &user.user_id, &shared_user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/recipes/{id}/shares",
    summary = "List users a recipe is shared with",
    description = "Returns a list of users (with emails) that the recipe is shared with. Only the recipe owner can view this list.",
    params(
        ("id" = Uuid, Path, description = "Recipe UUID")
    ),
    responses(
        (status = 200, description = "List of users the recipe is shared with", body = [ShareResponse]),
        (status = 401, description = "Unauthorized - authentication token missing or invalid"),
        (status = 403, description = "Access denied - user is not the owner of this recipe", body = ErrorResponse),
        (status = 404, description = "Recipe not found", body = ErrorResponse),
        (status = 500, description = "Database error", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Sharing"
)]
pub async fn list_recipe_shares<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    Path(recipe_id): Path<Uuid>,
) -> Result<Json<Vec<ShareResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state
        .get_use_case
        .execute(recipe_id, &user.user_id)
        .await
        .map_err(map_repo_error)?;

    if recipe.owner_id != user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Access denied".to_string(),
            }),
        ));
    }

    let shares = state
        .list_recipe_shares_use_case
        .execute(recipe_id)
        .await
        .map_err(map_repo_error)?;

    let mut responses: Vec<ShareResponse> = Vec::new();
    for share in shares {
        if let Ok(Some(email)) = crate::shared::auth::get_user_email_by_id(&share.user_id).await {
            responses.push(ShareResponse {
                user_id: share.user_id,
                email,
                created_at: share.created_at,
            });
        }
    }

    Ok(Json(responses))
}
