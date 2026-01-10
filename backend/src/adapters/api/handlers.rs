use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;

use crate::application::use_cases::GenerateRecipeUseCase;
use crate::domain::services::LlmService;

use super::dto::{GenerateRecipeRequest, RecipeResponse};

pub async fn generate_recipe<T: LlmService>(
    State(use_case): State<Arc<GenerateRecipeUseCase<T>>>,
    Json(request): Json<GenerateRecipeRequest>,
) -> Result<Json<RecipeResponse>, (StatusCode, String)> {
    let recipe = use_case
        .execute(request.ingredients, request.dietary_restrictions)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    Ok(Json(RecipeResponse {
        title: recipe.title,
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        prep_time_minutes: recipe.prep_time_minutes,
        cook_time_minutes: recipe.cook_time_minutes,
        servings: recipe.servings,
    }))
}
