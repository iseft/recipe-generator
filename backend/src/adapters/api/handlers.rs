use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;

use crate::application::use_cases::GenerateRecipeUseCase;
use crate::domain::services::LlmService;

use super::dto::{GenerateRecipeRequest, RecipeResponse};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn generate_recipe<T: LlmService>(
    State(use_case): State<Arc<GenerateRecipeUseCase<T>>>,
    Json(request): Json<GenerateRecipeRequest>,
) -> Result<Json<RecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ingredients: Vec<String> = request
        .ingredients
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if ingredients.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "At least one ingredient is required".to_string(),
            }),
        ));
    }

    let recipe = use_case
        .execute(ingredients, request.dietary_restrictions)
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
