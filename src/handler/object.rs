use crate::{error::AppError, model::ObjectInfo, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list_objects(
    State(state): State<AppState>,
    Path((bucket, prefix)): Path<(String, String)>,
) -> Result<Json<Vec<ObjectInfo>>, AppError> {
    let objects = state.s3_client.list_objects(&bucket, &prefix).await?;
    Ok(Json(
        objects
            .unwrap_or_default()
            .iter()
            .map(|o| ObjectInfo {
                key: o.key().unwrap_or_default().to_string(),
                size: o.size().unwrap_or_default(),
                last_modified: o
                    .last_modified()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            })
            .collect(),
    ))
}
