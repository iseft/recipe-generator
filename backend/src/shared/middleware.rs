use axum::Router;

use crate::shared::config::AppConfig;
use crate::shared::middleware::cors::apply_cors;
use crate::shared::middleware::rate_limit::{apply_rate_limit, create_rate_limiter};
use crate::shared::middleware::tracing::apply_tracing;

pub mod cors;
pub mod rate_limit;
pub mod tracing;

pub fn apply_middleware<S>(router: Router<S>, config: &AppConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let rate_limiter =
        create_rate_limiter(config.rate_limit_requests, config.rate_limit_duration_secs);

    let router = apply_rate_limit(router, rate_limiter);
    let router = apply_tracing(router);
    apply_cors(router, config)
}
