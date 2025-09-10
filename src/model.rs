use serde::Serialize;

#[derive(Serialize)]
pub struct BucketInfo {
    pub name: String,
    pub creation_date: String,
}

#[derive(Serialize)]
pub struct ObjectInfo {
    pub key: String,
    pub size: i64,
    pub last_modified: String,
}
