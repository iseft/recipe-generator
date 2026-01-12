use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::shared::config::AppConfig;

pub fn create_cors_layer(config: &AppConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            config
                .cors_origin
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any)
}

pub fn apply_cors<S>(router: Router<S>, config: &AppConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(create_cors_layer(config))
}
