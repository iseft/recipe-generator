use std::sync::Arc;

use axum::Router;
use backend::recipes::adapters::create_router;
use backend::recipes::application::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListOwnedRecipesUseCase, ListSharedRecipesUseCase, SaveRecipeUseCase,
};
use backend::recipes::infrastructure::{PgRecipeRepository, PgRecipeShareRepository};
use backend::shared::auth::init_clerk;
use backend::shared::db::create_pool;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct MockLlmClient;

impl backend::recipes::domain::LlmService for MockLlmClient {
    async fn generate_recipe(
        &self,
        _ingredients: Vec<String>,
        _dietary_restrictions: Option<Vec<String>>,
    ) -> Result<backend::recipes::domain::GeneratedRecipe, backend::recipes::domain::LlmError> {
        Ok(backend::recipes::domain::GeneratedRecipe {
            title: "Mock Recipe".to_string(),
            ingredients: vec!["mock ingredient".to_string()],
            instructions: vec!["mock instruction".to_string()],
            prep_time_minutes: Some(10),
            cook_time_minutes: Some(20),
            servings: Some(4),
        })
    }
}

pub struct FailingLlmClient {
    pub error: backend::recipes::domain::LlmError,
}

impl backend::recipes::domain::LlmService for FailingLlmClient {
    async fn generate_recipe(
        &self,
        _ingredients: Vec<String>,
        _dietary_restrictions: Option<Vec<String>>,
    ) -> Result<backend::recipes::domain::GeneratedRecipe, backend::recipes::domain::LlmError> {
        Err(self.error.clone())
    }
}

pub async fn create_test_app() -> Router {
    create_test_app_with_llm(Arc::new(MockLlmClient)).await
}

pub async fn create_test_app_with_llm<T: backend::recipes::domain::LlmService + 'static>(
    llm_client: Arc<T>,
) -> Router {
    init_clerk("sk_test_dummy_key_for_testing".to_string());

    let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://recipe_user:recipe_password@localhost:5432/recipe_generator_test".to_string()
    });

    let db_pool = create_pool(&database_url).await;
    let recipe_repository = Arc::new(PgRecipeRepository::new(db_pool.clone()));
    let share_repository = Arc::new(PgRecipeShareRepository::new(db_pool));

    let generate_use_case = Arc::new(GenerateRecipeUseCase::new(llm_client));
    let save_use_case = Arc::new(SaveRecipeUseCase::new(recipe_repository.clone()));
    let get_use_case = Arc::new(GetRecipeUseCase::new(
        recipe_repository.clone(),
        share_repository.clone(),
    ));
    let list_owned_use_case = Arc::new(ListOwnedRecipesUseCase::new(recipe_repository.clone()));
    let list_shared_use_case = Arc::new(ListSharedRecipesUseCase::new(recipe_repository.clone()));
    let create_share_use_case = Arc::new(CreateShareUseCase::new(
        recipe_repository.clone(),
        share_repository.clone(),
    ));
    let delete_share_use_case =
        Arc::new(DeleteShareUseCase::new(recipe_repository, share_repository));

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    create_router(
        generate_use_case,
        save_use_case,
        get_use_case,
        list_owned_use_case,
        list_shared_use_case,
        create_share_use_case,
        delete_share_use_case,
    )
    .layer(ServiceBuilder::new().layer(cors).into_inner())
}
