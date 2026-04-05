use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SongFile {
    pub title: String,
    pub artist: String,
    pub format: Format,
    pub song_path: String
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Format {
    OpenSongChart
}
