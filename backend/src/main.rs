mod recipes;
mod shared;

use std::net::SocketAddr;

use recipes::adapters::create_router;
use shared::app::AppDependencies;
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
    let dependencies = AppDependencies::new(&config, db_pool);

    let app = create_router(
        dependencies.generate_use_case,
        dependencies.save_use_case,
        dependencies.get_use_case,
        dependencies.list_owned_use_case,
        dependencies.list_shared_use_case,
        dependencies.list_recipe_shares_use_case,
        dependencies.create_share_use_case,
        dependencies.delete_share_use_case,
    );

    let app = apply_middleware(app, &config);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
