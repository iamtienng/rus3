use rus3::{router::setup_router, service::s3client::S3Client, state::AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    info!("Starting rus3 server...");

    let client = S3Client::new();

    let state = AppState {
        s3_client: Arc::new(client.await),
    };

    let app = setup_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);

    if let Err(err) = axum_server::bind(addr)
        .serve(app.await.into_make_service())
        .await
    {
        error!("Server error: {:?}", err);
    }
}
