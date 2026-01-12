pub mod adapters;
pub mod application;
pub mod dependencies;
pub mod domain;
pub mod infrastructure;

use axum::Router;
use sqlx::PgPool;

use crate::shared::config::AppConfig;

pub fn create_router(config: &AppConfig, db_pool: PgPool) -> Router {
    let dependencies = dependencies::RecipeDependencies::new(config, db_pool);

    adapters::create_router(
        dependencies.generate_use_case,
        dependencies.save_use_case,
        dependencies.get_use_case,
        dependencies.list_owned_use_case,
        dependencies.list_shared_use_case,
        dependencies.list_recipe_shares_use_case,
        dependencies.create_share_use_case,
        dependencies.delete_share_use_case,
    )
}
