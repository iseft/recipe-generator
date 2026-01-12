use axum::{
    Router,
    routing::{delete, get, post},
};
use std::sync::Arc;

use crate::recipes::application::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListOwnedRecipesUseCase, ListSharedRecipesUseCase, SaveRecipeUseCase,
};
use crate::recipes::domain::{LlmService, RecipeRepository, RecipeShareRepository};
use crate::shared::auth::create_clerk_layer;

use super::handlers;
use super::state::AppState;

async fn health() -> &'static str {
    "OK"
}

pub fn create_router<
    T: LlmService + 'static,
    R: RecipeRepository + 'static,
    S: RecipeShareRepository + 'static,
>(
    generate_use_case: Arc<GenerateRecipeUseCase<T>>,
    save_use_case: Arc<SaveRecipeUseCase<R>>,
    get_use_case: Arc<GetRecipeUseCase<R, S>>,
    list_owned_use_case: Arc<ListOwnedRecipesUseCase<R>>,
    list_shared_use_case: Arc<ListSharedRecipesUseCase<R>>,
    create_share_use_case: Arc<CreateShareUseCase<R, S>>,
    delete_share_use_case: Arc<DeleteShareUseCase<R, S>>,
) -> Router {
    let state = AppState {
        generate_use_case,
        save_use_case,
        get_use_case,
        list_owned_use_case,
        list_shared_use_case,
        create_share_use_case,
        delete_share_use_case,
    };

    let public_routes = Router::new()
        .route("/health", get(health))
        .route("/api/recipes/generate", post(handlers::generate_recipe));

    let protected_routes = Router::new()
        .route("/api/recipes/shared", get(handlers::list_shared_recipes))
        .route(
            "/api/recipes",
            post(handlers::save_recipe).get(handlers::list_my_recipes),
        )
        .route("/api/recipes/{id}", get(handlers::get_recipe))
        .route("/api/recipes/{id}/shares", post(handlers::create_share))
        .route(
            "/api/recipes/{recipe_id}/shares/{user_id}",
            delete(handlers::delete_share),
        )
        .layer(create_clerk_layer());

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
