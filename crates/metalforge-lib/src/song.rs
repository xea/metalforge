use url::Url;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Song {
    pub header: SongHeader,
    pub path: Url
}

impl Song {
    pub fn id(&self) -> String {
        let mut id_str = format!("{}-{}-{}-{}", self.header.title, self.header.artist, self.header.album, self.header.year).to_lowercase();
        id_str.retain(|c| c.is_alphanumeric());
        id_str
    }

    pub fn add_arrangement(&mut self, other: Arrangement) -> &Self {
        self.header.arrangements.push(other);
        self
    }
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq)]
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
    pub arrangements: Vec<Arrangement>
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq)]
pub struct Arrangement {
    pub name: String,
    pub instrument: Instrument
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq)]
pub enum Instrument {
    /// 6-string guitar
    Guitar6,
}