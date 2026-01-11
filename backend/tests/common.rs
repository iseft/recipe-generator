use std::sync::Arc;

use axum::Router;
use backend::adapters::api::routes::create_router;
use backend::application::use_cases::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListRecipesUseCase, SaveRecipeUseCase,
};
use backend::infrastructure::db::create_pool;
use backend::infrastructure::repositories::{PgRecipeRepository, PgRecipeShareRepository};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct MockLlmClient;

impl backend::domain::services::LlmService for MockLlmClient {
    async fn generate_recipe(
        &self,
        _ingredients: Vec<String>,
        _dietary_restrictions: Option<Vec<String>>,
    ) -> Result<backend::domain::entities::GeneratedRecipe, backend::domain::services::LlmError>
    {
        Ok(backend::domain::entities::GeneratedRecipe {
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
    pub error: backend::domain::services::LlmError,
}

impl backend::domain::services::LlmService for FailingLlmClient {
    async fn generate_recipe(
        &self,
        _ingredients: Vec<String>,
        _dietary_restrictions: Option<Vec<String>>,
    ) -> Result<backend::domain::entities::GeneratedRecipe, backend::domain::services::LlmError>
    {
        Err(self.error.clone())
    }
}

pub async fn create_test_app() -> Router {
    create_test_app_with_llm(Arc::new(MockLlmClient)).await
}

pub async fn create_test_app_with_llm<T: backend::domain::services::LlmService + 'static>(
    llm_client: Arc<T>,
) -> Router {
    let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://recipe_user:recipe_password@localhost:5432/recipe_generator_test".to_string()
    });

    let db_pool = create_pool(&database_url).await;
    let recipe_repository = Arc::new(PgRecipeRepository::new(db_pool.clone()));
    let share_repository = Arc::new(PgRecipeShareRepository::new(db_pool));

    let generate_use_case = Arc::new(GenerateRecipeUseCase::new(llm_client));
    let save_use_case = Arc::new(SaveRecipeUseCase::new(recipe_repository.clone()));
    let get_use_case = Arc::new(GetRecipeUseCase::new(recipe_repository.clone()));
    let list_use_case = Arc::new(ListRecipesUseCase::new(recipe_repository.clone()));
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
        list_use_case,
        create_share_use_case,
        delete_share_use_case,
    )
    .layer(ServiceBuilder::new().layer(cors).into_inner())
}
