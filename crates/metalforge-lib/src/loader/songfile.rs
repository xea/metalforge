use crate::asset::AssetId;
use crate::song::{Instrument, Song, SongHeader, Tuning};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Defines what metadata of a `Song` is stored in Songfiles.
#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongDef {
    pub title: String,
    pub title_sort: String,
    pub album: String,
    pub album_sort: String,
    pub artist: String,
    pub artist_sort: String,
    pub year: u16,
    pub version: u16,
    pub length_sec: u16,
    pub cover_art_path: Option<String>,
    pub backing_track_path: Option<String>,
    pub song_preview_path: Option<String>,
    pub arrangements: Vec<ArrangementDef>
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrangementDef {
    pub id: String,
    pub name: String,
    pub instrument: Instrument,
    pub tuning: Tuning
}

impl From<&Song> for SongDef {
    fn from(value: &Song) -> Self {
        todo!()
    }
}

impl From<&SongHeader> for SongDef {
    fn from(value: &SongHeader) -> Self {
        todo!()
    }
}

pub fn load_song<P: AsRef<Path>>(song_path: P) -> std::io::Result<Song> {
    let reader = BufReader::new(File::open(song_path.as_ref())?);
    let def = load_song_def(reader)?;

    let song = Song {
        header: SongHeader {
            title: def.title,
            title_sort: def.title_sort,
            album: def.album,
            album_sort: def.album_sort,
            artist: def.artist,
            artist_sort: def.artist_sort,
            year: def.year,
            version: def.version,
            length_sec: def.length_sec,
            arrangements: vec![],
        },
        cover_art: None,
    };

    Ok(song)
}

fn load_song_def<R: Read>(reader: R) -> std::io::Result<SongDef> {
    serde_yaml::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(test)]
mod tests {

}
