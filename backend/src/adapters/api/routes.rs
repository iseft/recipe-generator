use axum::{Router, routing::post};
use std::sync::Arc;

use crate::application::use_cases::GenerateRecipeUseCase;
use crate::domain::services::LlmService;

use super::handlers;

pub fn create_router<T: LlmService + 'static>(use_case: Arc<GenerateRecipeUseCase<T>>) -> Router {
    Router::new()
        .route(
            "/api/recipes/generate",
            post(handlers::generate_recipe::<T>),
        )
        .with_state(use_case)
}
