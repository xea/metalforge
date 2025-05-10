use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AssetId {
    pub id: String,
    pub parent_id: String,
}

impl AssetId {
    pub fn from_paths<P: AsRef<Path>>(song_path: P, asset_path: &str) -> AssetId {
        AssetId { parent_id: song_path.as_ref().to_str().unwrap().to_string(), id: asset_path.to_string() }
    }
}