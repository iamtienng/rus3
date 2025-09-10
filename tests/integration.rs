use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rus3::{
    model::{BucketInfo, ObjectInfo},
    router::setup_router,
    service::s3client::S3Client,
    state::AppState,
};
use std::{env, sync::Arc};
use tower::ServiceExt; // for `oneshot`

// Helper to build app (uses real S3 client right now)
async fn build_test_app() -> axum::Router {
    let state = AppState {
        s3_client: Arc::new(S3Client::new().await),
    };

    setup_router(state).await
}

#[tokio::test]
async fn test_list_buckets() {
    let app = build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/buckets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let buckets: Vec<BucketInfo> = serde_json::from_slice(&body).unwrap();

    assert!(buckets.is_empty() || !buckets[0].name.is_empty());
}

#[tokio::test]
async fn test_list_objects() {
    let app = build_test_app().await;

    let test_bucket = env::var("TEST_BUCKET").unwrap_or_default();
    let test_prefix = env::var("TEST_PREFIX").unwrap_or_default();
    let test_uri = format!("/bucket/{}/{}", test_bucket, test_prefix);

    let response = app
        .oneshot(
            Request::builder()
                .uri(test_uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let objects: Vec<ObjectInfo> = serde_json::from_slice(&body).unwrap();

    for obj in objects {
        assert!(!obj.key.is_empty());
    }
}
