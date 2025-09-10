use crate::error::AppError;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    types::{Bucket, Object},
};

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        S3Client { client }
    }

    pub async fn list_buckets(&self) -> Result<Option<Vec<Bucket>>, AppError> {
        let buckets = self.client.list_buckets().send().await;

        Ok(buckets.unwrap().buckets)
    }

    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<Option<Vec<Object>>, AppError> {
        let resp = self
            .client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .send()
            .await;

        Ok(resp.unwrap().contents)
    }
}
