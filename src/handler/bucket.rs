use crate::{error::AppError, model::BucketInfo, state::AppState};
use axum::{Json, extract::State};

pub async fn list_buckets(
    State(state): State<AppState>,
) -> Result<Json<Vec<BucketInfo>>, AppError> {
    let buckets = state.s3_client.list_buckets().await?;
    Ok(Json(
        buckets
            .unwrap_or_default()
            .iter()
            .map(|b| BucketInfo {
                name: b.name().unwrap_or_default().to_string(),
                creation_date: b
                    .creation_date()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            })
            .collect(),
    ))
}
