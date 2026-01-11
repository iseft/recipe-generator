use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::Recipe;
use crate::domain::repositories::RecipeRepository;
use crate::domain::services::{LlmError, LlmService};

use super::dto::{
    GenerateRecipeRequest, GeneratedRecipeResponse, RecipeResponse, SaveRecipeRequest,
};
use super::extractors::ValidatedJson;
use super::state::AppState;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn generate_recipe<T: LlmService, R: RecipeRepository>(
    State(state): State<AppState<T, R>>,
    ValidatedJson(request): ValidatedJson<GenerateRecipeRequest>,
) -> Result<Json<GeneratedRecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state
        .generate_use_case
        .execute(request.ingredients, request.dietary_restrictions)
        .await
        .map_err(|e| {
            let (status, message) = match e {
                LlmError::ApiError(_) => (
                    StatusCode::BAD_GATEWAY,
                    "Failed to reach AI service".to_string(),
                ),
                LlmError::ParseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse AI response".to_string(),
                ),
            };
            (status, Json(ErrorResponse { error: message }))
        })?;

    Ok(Json(recipe.into()))
}

pub async fn save_recipe<T: LlmService, R: RecipeRepository>(
    State(state): State<AppState<T, R>>,
    ValidatedJson(request): ValidatedJson<SaveRecipeRequest>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = Recipe::from_generated(request.into());

    state
        .save_use_case
        .execute(recipe.clone())
        .await
        .map_err(|e| {
            let (status, message) = match e {
                crate::domain::repositories::RepositoryError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to save recipe".to_string(),
                ),
                crate::domain::repositories::RepositoryError::NotFound => {
                    (StatusCode::NOT_FOUND, "Recipe not found".to_string())
                }
            };
            (status, Json(ErrorResponse { error: message }))
        })?;

    Ok(Json(recipe.into()))
}

pub async fn get_recipe<T: LlmService, R: RecipeRepository>(
    State(state): State<AppState<T, R>>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = state.get_use_case.execute(id).await.map_err(|e| {
        let (status, message) = match e {
            crate::domain::repositories::RepositoryError::NotFound => {
                (StatusCode::NOT_FOUND, "Recipe not found".to_string())
            }
            crate::domain::repositories::RepositoryError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch recipe".to_string(),
            ),
        };
        (status, Json(ErrorResponse { error: message }))
    })?;

    Ok(Json(recipe.into()))
}

pub async fn list_recipes<T: LlmService, R: RecipeRepository>(
    State(state): State<AppState<T, R>>,
) -> Result<Json<Vec<RecipeResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let recipes = state.list_use_case.execute().await.map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to fetch recipes".to_string(),
            }),
        )
    })?;

    Ok(Json(recipes.into_iter().map(|r| r.into()).collect()))
}
