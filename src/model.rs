use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BucketInfo {
    pub name: String,
    pub creation_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectInfo {
    pub key: String,
    pub size: i64,
    pub last_modified: String,
}
