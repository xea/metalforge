use crate::scan_library_url;
use log::trace;
use metalforge_lib::library::SongLibrary;
use metalforge_lib::song::{Song, SongHeader};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use url::Url;

/// Library definition is a list of absolute or relative directories in which
/// loadable songs can be found.
#[derive(Serialize, Deserialize)]
pub struct LibraryDef {
    songs: Vec<SongEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct SongEntry {
    /// Absolute or relative path to the song's location
    pub path: String,
}

pub fn load_library(url: &Url) -> std::io::Result<SongLibrary> {
    let mut library = SongLibrary::empty();

    trace!("Loading library definition {}", url);

    let library_def = load_library_def(url)?;

    trace!("Library definition loaded with {} song entries", library_def.songs.len());

    for song_entry in &library_def.songs {
        // Build the URL to the song entry
        let mut song_url = url.clone();
        let mut song_path = song_url.to_file_path().expect("Failed to convert song URL to file path");
        // Remove the library descriptor file name from the path
        song_path.pop();
        // Replace it with the file name of the current entry
        song_path.push(song_entry.path.as_str());
        //song_path.push(SONG_DESCRIPTOR);

        song_url.set_path(song_path.to_str().expect("Failed to convert path to URL"));

        let mut new_library = scan_library_url(&song_url)?;
        library.merge(&mut new_library);
    }

    Ok(library)
}

pub fn load_song(song_url: &Url) -> std::io::Result<SongLibrary> {
    let mut library = SongLibrary::empty();

    let song = load_song_header(&song_url)
    .map(|header| Song {
        header,
        path: song_url.clone()
    })?;

    library.add_song(song);

    Ok(library)
}

pub fn load_song_header(url: &Url) -> std::io::Result<SongHeader> {
    let bytes = load_file_contents(url)?;

    serde_yaml::from_slice(bytes.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}


fn load_library_def(url: &Url) -> std::io::Result<LibraryDef> {
    let bytes = load_file_contents(url)?;

    serde_yaml::from_slice(bytes.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn load_file_contents(url: &Url) -> std::io::Result<Vec<u8>> {
    trace!("Loading file {}", url);

    match url.scheme() {
        "file" => load_file_from_fs(url.to_file_path().expect("Failed to convert url to file path")),
        scheme => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unsupported scheme: {}", scheme)))
    }
}

fn load_file_from_fs<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    println!("Loading file: {:?}", path.as_ref());

    let mut buffer = vec![];

    match File::open(&path) {
        Ok(mut file) => {
            let _ = file.read_to_end(&mut buffer);
            Ok(buffer)
        }
        Err(error) => {
            trace!("Failed to load file {:?} ({})", path.as_ref().to_str(), error);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_library() {}
}
