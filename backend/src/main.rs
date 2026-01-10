mod adapters;
mod application;
mod domain;
mod infrastructure;

use std::net::SocketAddr;
use std::sync::Arc;

use adapters::api::routes::create_router;
use application::use_cases::GenerateRecipeUseCase;
use infrastructure::config::AppConfig;
use infrastructure::llm::OpenAiClient;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();

    let llm_client = Arc::new(OpenAiClient::new(config.openai_api_key));
    let use_case = Arc::new(GenerateRecipeUseCase::new(llm_client));

    let app = create_router(use_case);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
