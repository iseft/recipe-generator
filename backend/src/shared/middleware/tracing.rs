use axum::Router;
use tower_http::trace::TraceLayer;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();
}

pub fn apply_tracing<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
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

    router.layer(trace_layer)
}
