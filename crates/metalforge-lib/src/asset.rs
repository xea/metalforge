use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::part::InstrumentPart;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AssetId {
    id: String,
    parent_id: String,
}

impl AssetId {
    pub fn from_paths<P: AsRef<Path>>(song_path: P, asset_path: &str) -> AssetId {
        AssetId { parent_id: song_path.as_ref().to_str().unwrap().to_string(), id: asset_path.to_string() }
    }
}

pub fn load_instrument_part(asset_id: &AssetId) -> std::io::Result<InstrumentPart> {
    load_asset(asset_id)
        .and_then(|asset| serde_yaml::from_reader(asset)
            .map_err(|e| Error::new(ErrorKind::Other, e)))
}

pub fn load_asset(asset_id: &AssetId) -> std::io::Result<impl Read> {
    let path = PathBuf::from(&asset_id.parent_id);
    
    Ok(BufReader::new(File::open(path)?))
}