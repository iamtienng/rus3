use crate::service::s3client::S3Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub s3_client: Arc<S3Client>,
}
