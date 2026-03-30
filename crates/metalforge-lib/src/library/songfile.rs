use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SongFile {
    pub title: String,
    pub artist: String,
    pub format: Format,
    pub song_path: String
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    OpenSongChart
}
