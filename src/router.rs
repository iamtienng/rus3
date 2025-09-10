use crate::{
    handler::{bucket, object},
    state::AppState,
};
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub async fn setup_router(state: AppState) -> Router {
    Router::new()
        .route("/buckets", get(bucket::list_buckets))
        .route("/bucket/{bucket}", get(object::list_objects))
        .route("/bucket/{bucket}/{*prefix}", get(object::list_objects))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http-request",
                        method = %request.method(),
                        uri = %request.uri()
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = %response.status(),
                            latency_ms = %latency.as_millis(),
                            "response finished"
                        );
                    },
                ),
        )
}
