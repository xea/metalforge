use serde::{Deserialize, Serialize};
use crate::format::opensongchart::instrument_part::InstrumentPart;

#[derive(Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "SongName")]
    pub song_name: String,

    #[serde(rename = "ArtistName")]
    pub artist_name: String,

    #[serde(rename = "AlbumName")]
    pub album_name: String,

    #[serde(rename = "SongYear")]
    pub song_year: i16,

    #[serde(rename = "SongLengthSeconds")]
    pub song_length_seconds: f32,

    #[serde(rename = "A440CentsOffset", default)]
    pub a440_cent_offset: i16,

    #[serde(rename = "InstrumentParts")]
    pub instrument_parts: Vec<InstrumentPart>
}

