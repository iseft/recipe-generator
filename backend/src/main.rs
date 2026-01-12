mod recipes;
mod shared;

use std::net::SocketAddr;

use shared::auth::init_clerk;
use shared::config::AppConfig;
use shared::db::create_pool;
use shared::middleware::apply_middleware;
use shared::middleware::tracing::init_tracing;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("../.env")
        .or_else(|_| dotenvy::dotenv())
        .ok();

    init_tracing();

    let config = AppConfig::from_env();
    init_clerk(config.clerk_secret_key.clone());

    let db_pool = create_pool(&config.database_url).await;

    let app = recipes::create_router(&config, db_pool);
    let app = apply_middleware(app, &config);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
