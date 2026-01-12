mod recipes;
mod shared;

use std::net::SocketAddr;
use std::sync::Arc;

use recipes::adapters::create_router;
use recipes::application::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListOwnedRecipesUseCase, ListRecipeSharesUseCase, ListSharedRecipesUseCase, SaveRecipeUseCase,
};
use recipes::infrastructure::{OpenAiClient, PgRecipeRepository, PgRecipeShareRepository};
use shared::auth::init_clerk;
use shared::config::AppConfig;
use shared::db::create_pool;
use shared::rate_limit::{RateLimiter, rate_limit_middleware};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("../.env")
        .or_else(|_| dotenvy::dotenv())
        .ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    let config = AppConfig::from_env();

    init_clerk(config.clerk_secret_key.clone());

    let db_pool = create_pool(&config.database_url).await;

    let llm_client = Arc::new(OpenAiClient::new(config.openai_api_key));
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
    let list_recipe_shares_use_case =
        Arc::new(ListRecipeSharesUseCase::new(share_repository.clone()));
    let create_share_use_case = Arc::new(CreateShareUseCase::new(
        recipe_repository.clone(),
        share_repository.clone(),
    ));
    let delete_share_use_case =
        Arc::new(DeleteShareUseCase::new(recipe_repository, share_repository));

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origin
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        })
        .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
            tracing::info!("Incoming request");
        })
        .on_response(
            |_response: &axum::http::Response<_>,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                tracing::info!(
                    status = %_response.status(),
                    latency_ms = latency.as_millis(),
                    "Request completed"
                );
            },
        )
        .on_failure(
            |_error: tower_http::classify::ServerErrorsFailureClass,
             _latency: std::time::Duration,
             _span: &tracing::Span| {
                tracing::error!("Request failed");
            },
        );

    let rate_limiter =
        RateLimiter::new(config.rate_limit_requests, config.rate_limit_duration_secs);

    let app = create_router(
        generate_use_case,
        save_use_case,
        get_use_case,
        list_owned_use_case,
        list_shared_use_case,
        list_recipe_shares_use_case,
        create_share_use_case,
        delete_share_use_case,
    )
    .layer(axum::middleware::from_fn_with_state(
        rate_limiter.clone(),
        rate_limit_middleware,
    ))
    .layer(trace_layer)
    .layer(cors);

    tracing::info!(
        "Rate limiting configured: {} requests per {} seconds per IP address",
        config.rate_limit_requests,
        config.rate_limit_duration_secs
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
