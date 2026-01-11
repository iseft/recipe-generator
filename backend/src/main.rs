mod adapters;
mod application;
mod domain;
mod infrastructure;

use std::net::SocketAddr;
use std::sync::Arc;

use adapters::api::routes::create_router;
use application::use_cases::{
    GenerateRecipeUseCase, GetRecipeUseCase, ListRecipesUseCase, SaveRecipeUseCase,
};
use infrastructure::config::AppConfig;
use infrastructure::db::create_pool;
use infrastructure::llm::OpenAiClient;
use infrastructure::repositories::PgRecipeRepository;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();

    let db_pool = create_pool(&config.database_url).await;

    let llm_client = Arc::new(OpenAiClient::new(config.openai_api_key));
    let recipe_repository = Arc::new(PgRecipeRepository::new(db_pool));

    let generate_use_case = Arc::new(GenerateRecipeUseCase::new(llm_client));
    let save_use_case = Arc::new(SaveRecipeUseCase::new(recipe_repository.clone()));
    let get_use_case = Arc::new(GetRecipeUseCase::new(recipe_repository.clone()));
    let list_use_case = Arc::new(ListRecipesUseCase::new(recipe_repository));

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origin
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router(
        generate_use_case,
        save_use_case,
        get_use_case,
        list_use_case,
    )
    .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
