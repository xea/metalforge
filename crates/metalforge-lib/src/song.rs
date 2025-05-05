use crate::asset::AssetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Song {
    pub header: SongHeader,
    pub cover_art: Option<AssetId>,
}

impl Song {
    pub fn id(&self) -> String {
        let mut id_str = format!(
            "{}-{}-{}-{}",
            self.header.title, self.header.artist, self.header.album, self.header.year
        )
        .to_lowercase();
        id_str.retain(|c| c.is_alphanumeric());
        id_str
    }

    pub fn add_arrangement(&mut self, other: Arrangement) -> &Self {
        self.header.arrangements.push(other);
        self
    }
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongHeader {
    pub title: String,
    pub title_sort: String,
    pub album: String,
    pub album_sort: String,
    pub artist: String,
    pub artist_sort: String,
    pub year: u16,
    pub version: u16,
    pub length_sec: u16,
    // Potential others:
    // - Performed by
    // - Genres
    // - Average BPM
    // - Composer
    // - Tuning offset (hz/cents)
    pub cover_art_path: Option<String>,
    pub backing_track_path: Option<String>,
    pub song_preview_path: Option<String>,
    pub arrangements: Vec<Arrangement>,
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrangement {
    pub id: String,
    pub name: String,
    pub instrument: Instrument,
    pub tuning: Option<Tuning>
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Instrument {
    Vocal,
    ElectricBass,
    AcousticGuitar,
    ElectricGuitar,
}
#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Tuning {
    // E2 A2 D3 G3 B3 E4
    Standard,
    // In custom tuning, each element in the vector represents the offset a string/key needs to be tuned
    // with regard to its corresponding standard tuning, in steps of 1/8ths. For example, in the case
    // of a 6-string guitar, the value of [ 0, 0, 0, 0, 0, 0 ] represents standard tuning, while the
    // value [ -4, -4, -4, -4, -4, -4 ] means lowered Eb tuning, and the value [ -8, 0, 0, 0, 0, 0 ]
    // represents Drop D.
    Custom(Vec<u16>)
}
