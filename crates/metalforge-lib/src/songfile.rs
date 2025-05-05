use crate::song::{Song, SongHeader};
use log::trace;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::asset::AssetId;

#[derive(Serialize, Deserialize)]
pub struct SongEntry {
    /// Absolute or relative path to the song's location
    pub path: String,
}

pub fn load_song<P: AsRef<Path>>(song_path: P) -> std::io::Result<Song> {
    let reader = BufReader::new(File::open(song_path.as_ref())?);

    let song = load_song_header(reader)
    .map(|header| {
        let cover_art = header.cover_art_path.clone()
            .map(|asset_path| AssetId::from_paths(song_path.as_ref(), asset_path.as_ref()));
        
        Song {
            header,
            cover_art,
        }
    })?;

    Ok(song)
}

pub fn load_song_header<R: Read>(reader: R) -> std::io::Result<SongHeader> {
    serde_yaml::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(test)]
mod tests {

}
