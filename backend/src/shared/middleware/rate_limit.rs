use axum::extract::ConnectInfo;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Router;
use dashmap::DashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            max_requests,
            window_seconds,
        }
    }

    pub fn check_rate_limit(&self, key: &str) -> Result<(), StatusCode> {
        let now = Instant::now();
        let window = Duration::from_secs(self.window_seconds);

        let mut requests = self
            .requests
            .entry(key.to_string())
            .or_insert_with(Vec::new);

        requests.retain(|&timestamp| now.duration_since(timestamp) < window);

        if requests.len() >= self.max_requests as usize {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        requests.push(now);

        Ok(())
    }

    fn get_client_ip(
        headers: &HeaderMap,
        connect_info: Option<&ConnectInfo<SocketAddr>>,
    ) -> String {
        if let Some(forwarded_for) = headers.get("x-forwarded-for") {
            if let Ok(ip_str) = forwarded_for.to_str() {
                if let Some(first_ip) = ip_str.split(',').next() {
                    return first_ip.trim().to_string();
                }
            }
        }

        if let Some(real_ip) = headers.get("x-real-ip") {
            if let Ok(ip_str) = real_ip.to_str() {
                return ip_str.to_string();
            }
        }

        if let Some(ConnectInfo(addr)) = connect_info {
            return addr.ip().to_string();
        }

        "unknown".to_string()
    }
}

async fn rate_limit_middleware(
    axum::extract::State(rate_limiter): axum::extract::State<RateLimiter>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let connect_info = req.extensions().get::<ConnectInfo<SocketAddr>>().cloned();
    let client_ip = RateLimiter::get_client_ip(req.headers(), connect_info.as_ref());

    match rate_limiter.check_rate_limit(&client_ip) {
        Ok(_) => next.run(req).await,
        Err(status) => (
            status,
            [("X-RateLimit-Limit", rate_limiter.max_requests.to_string())],
            "Too Many Requests",
        )
            .into_response(),
    }
}

pub fn create_rate_limiter(max_requests: u32, window_seconds: u64) -> RateLimiter {
    let rate_limiter = RateLimiter::new(max_requests, window_seconds);

    tracing::info!(
        "Rate limiting configured: {} requests per {} seconds per IP address",
        max_requests,
        window_seconds
    );

    rate_limiter
}

pub fn apply_rate_limit<S>(router: Router<S>, rate_limiter: RateLimiter) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(axum::middleware::from_fn_with_state(
        rate_limiter.clone(),
        rate_limit_middleware,
    ))
}
