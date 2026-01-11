use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::application::use_cases::{
    GenerateRecipeUseCase, GetRecipeUseCase, ListRecipesUseCase, SaveRecipeUseCase,
};
use crate::domain::repositories::RecipeRepository;
use crate::domain::services::LlmService;

use super::handlers;
use super::state::AppState;

pub fn create_router<T: LlmService + 'static, R: RecipeRepository + 'static>(
    generate_use_case: Arc<GenerateRecipeUseCase<T>>,
    save_use_case: Arc<SaveRecipeUseCase<R>>,
    get_use_case: Arc<GetRecipeUseCase<R>>,
    list_use_case: Arc<ListRecipesUseCase<R>>,
) -> Router {
    let state = AppState {
        generate_use_case,
        save_use_case,
        get_use_case,
        list_use_case,
    };

    Router::new()
        .route("/api/recipes/generate", post(handlers::generate_recipe))
        .route(
            "/api/recipes",
            post(handlers::save_recipe).get(handlers::list_recipes),
        )
        .route("/api/recipes/{id}", get(handlers::get_recipe))
        .with_state(state)
}
