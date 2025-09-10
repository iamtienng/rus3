use crate::{
    handler::{bucket, object},
    state::AppState,
};
use axum::{Router, routing::get};

pub async fn setup_router(state: AppState) -> Router {
    Router::new()
        .route("/buckets", get(bucket::list_buckets))
        .route("/bucket/{bucket}", get(object::list_objects))
        .route("/bucket/{bucket}/{*prefix}", get(object::list_objects))
        .with_state(state)
}
