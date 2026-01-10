use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;

use crate::application::use_cases::GenerateRecipeUseCase;
use crate::domain::services::LlmService;

use super::dto::{GenerateRecipeRequest, RecipeResponse};
use super::extractors::ValidatedJson;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn generate_recipe<T: LlmService>(
    State(use_case): State<Arc<GenerateRecipeUseCase<T>>>,
    ValidatedJson(request): ValidatedJson<GenerateRecipeRequest>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = use_case
        .execute(request.ingredients, request.dietary_restrictions)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("{:?}", e),
                }),
            )
        })?;

    Ok(Json(RecipeResponse {
        title: recipe.title,
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        prep_time_minutes: recipe.prep_time_minutes,
        cook_time_minutes: recipe.cook_time_minutes,
        servings: recipe.servings,
    }))
}
