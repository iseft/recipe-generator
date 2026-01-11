use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::Recipe;
use crate::domain::repositories::{RecipeRepository, RecipeShareRepository, RepositoryError};
use crate::domain::services::{LlmError, LlmService};
use crate::infrastructure::auth::AuthenticatedUser;

use super::dto::{
    CreateShareRequest, GenerateRecipeRequest, GeneratedRecipeResponse, RecipeResponse,
    SaveRecipeRequest,
};
use super::extractors::ValidatedJson;
use super::state::AppState;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn map_repo_error(e: RepositoryError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, message) = match e {
        RepositoryError::NotFound => (StatusCode::NOT_FOUND, "Recipe not found"),
        RepositoryError::AccessDenied => (StatusCode::FORBIDDEN, "Access denied"),
        RepositoryError::DatabaseError(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    };
    (status, Json(ErrorResponse { error: message.to_string() }))
}

pub async fn generate_recipe<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    _user: AuthenticatedUser,
    ValidatedJson(request): ValidatedJson<GenerateRecipeRequest>,
) -> Result<Json<GeneratedRecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state
        .generate_use_case
        .execute(request.ingredients, request.dietary_restrictions)
        .await
        .map_err(|e| {
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
        })?;

    Ok(Json(recipe.into()))
}

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

    Ok(Json(recipe.into()))
}

pub async fn list_my_recipes<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<RecipeResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipes = state
        .list_use_case
        .execute_owned(&user.user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(Json(recipes.into_iter().map(|r| r.into()).collect()))
}

pub async fn list_shared_recipes<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<RecipeResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipes = state
        .list_use_case
        .execute_shared(&user.user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(Json(recipes.into_iter().map(|r| r.into()).collect()))
}

pub async fn create_share<T: LlmService, R: RecipeRepository, S: RecipeShareRepository>(
    State(state): State<AppState<T, R, S>>,
    user: AuthenticatedUser,
    Path(recipe_id): Path<Uuid>,
    ValidatedJson(request): ValidatedJson<CreateShareRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state
        .create_share_use_case
        .execute(recipe_id, &user.user_id, request.user_id)
        .await
        .map_err(map_repo_error)?;

    Ok(StatusCode::CREATED)
}

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
