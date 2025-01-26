use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use url::Url;
use metalforge_lib::song::{Song, SongHeader};

/// Library definition is a list of absolute or relative directories in which
/// loadable songs can be found.
#[derive(Serialize, Deserialize)]
pub struct LibraryDef {
    songs: Vec<SongEntry>
}

#[derive(Serialize, Deserialize)]
pub struct SongEntry {
    /// Absolute or relative path to the song's location
    pub path: String
}

pub fn load_library(path: &str) -> std::io::Result<Vec<Song>> {
    let library_def = load_library_def(path)?;

    let mut songs = vec![];

    for song in &library_def.songs {
        match load_song_header(song.path.as_str()) {
            Ok(header) => {
                let song = Song {
                    header,
                    path: Url::parse(song.path.as_str()).unwrap(),
                };

                songs.push(song);
            },
            Err(err) => eprintln!("Failed to load song {}: {}", song.path, err)
        }
    }

    Ok(songs)
}

pub fn load_song_header(path: &str) -> std::io::Result<SongHeader> {
    let bytes = load_file(path)?;

    serde_yaml::from_slice(bytes.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn load_library_def(path: &str) -> std::io::Result<LibraryDef> {
    let bytes = load_file(path)?;

    serde_yaml::from_slice(bytes.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn load_file(path: &str) -> std::io::Result<Vec<u8>> {
    match Url::parse(path) {
        Ok(url) =>
            match url.scheme() {
                "file" => load_file_from_fs(url.path()),
                "http" | "https" => load_file_from_http(&url),
                scheme => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unsupported scheme: {}", scheme)))
            }
        Err(error) => load_file_from_fs(path)
    }
}

fn load_file_from_fs<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![];
    let _ = File::open(path)?.read_to_end(&mut buffer);

    Ok(buffer)
}

fn load_file_from_http(url: &Url) -> std::io::Result<Vec<u8>> {
    todo!()
}
/*
/// Load and parse a library definition from a file
pub fn load_library_def(path: &str) -> std::io::Result<Vec<std::io::Result<Song>>> {
    // TODO: implement loading from URLs, allowing loading from network as well

    serde_yaml::from_reader(std::fs::File::open(path)?)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .map(|library_def| load_library(&library_def))
}

/// Load and parse a song definition from a file
pub fn load_song_def(path: &str) -> std::io::Result<SongEntry> {
    // TODO: implement loading from URLs, allowing loading from network as well
    serde_yaml::from_reader(std::fs::File::open(path)?)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn load_song(song_def: &SongEntry) -> std::io::Result<Song> {
    load_song_header(song_def).map(|header| Song {
        header,
        path: Url::parse(song_def.path.as_str()).unwrap()
    })
}

/// Load and parse a song header from a file
pub fn load_song_header(song_def: &SongEntry) -> std::io::Result<SongHeader> {
    // TODO: implement loading from URLs, allowing loading from network as well
    serde_yaml::from_reader(std::fs::File::open(&song_def.path)?)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn load_library(library_def: &LibraryDef) -> Vec<std::io::Result<Song>> {
    library_def.songs.iter()
        .map(|song| load_song(song))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::loader::{load_library_def, load_song_header, SongEntry};

    #[test]
    fn library_definitions_parse_correctly() {
        let result = load_library_def("../../library/library.yaml");

        assert!(result.is_ok());

        let songs = result.unwrap();

        assert!(!songs.is_empty());
    }

    #[test]
    fn missing_library_definitions_parse_correctly() {
        let result = load_library_def("does_not_exist.yaml");

        assert!(result.is_err());
    }

    #[test]
    fn song_headers_parse_correctly() {
        let result = load_song_header(&SongEntry { path: "../../library/cmajoropen/song.yaml".to_string() });

        assert!(result.is_ok());

        let song = result.unwrap();
        assert_eq!(1, song.arrangements.len());
    }
}
 */