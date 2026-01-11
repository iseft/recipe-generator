use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;

use crate::application::use_cases::GenerateRecipeUseCase;
use crate::domain::services::{LlmError, LlmService};

use super::dto::{GenerateRecipeRequest, GeneratedRecipeResponse};
use super::extractors::ValidatedJson;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn generate_recipe<T: LlmService>(
    State(use_case): State<Arc<GenerateRecipeUseCase<T>>>,
    ValidatedJson(request): ValidatedJson<GenerateRecipeRequest>,
) -> Result<Json<GeneratedRecipeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let recipe = use_case
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
