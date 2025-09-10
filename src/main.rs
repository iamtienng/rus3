use rus3::{router::setup_router, service::s3client::S3Client, state::AppState};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let client = S3Client::new();

    let state = AppState {
        s3_client: Arc::new(client.await),
    };

    let app = setup_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
