use crate::track::Track;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use url::Url;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Song {
    pub header: SongHeader,
    pub path: Url,
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

    pub fn load_track(&self, arrangement: &Arrangement) -> Result<Track, ()> {
        let mut file_path = self.path.to_file_path().expect("Failed to convert URL to file path");
        file_path.pop();
        file_path.push(format!("arrangement_{}.yaml", arrangement.id.as_str()));

        let reader = BufReader::new(File::open(file_path)
            .expect("Failed to open track file: {}"));
        let content = serde_yaml::from_reader(reader)
            .expect("Failed to parse track file: {}");

        Ok(content)
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
    pub arrangements: Vec<Arrangement>,
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrangement {
    pub id: String,
    pub name: String,
    pub instrument: Instrument,

}


#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Instrument {
    ElectricBass,
    AcousticGuitar,
    ElectricGuitar,
}

