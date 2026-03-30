use crate::format::load_dir;
use crate::library::songfile::SongFile;
use log::{error, warn};
use std::io::Error;
use std::path::{Path};

pub mod songfile;

pub struct Library {
    pub songs: Vec<SongFile>
}

impl Library {

    pub fn empty() -> Self {
        Self {
            songs: vec![]
        }
    }

    pub fn scan_directories<P: AsRef<Path>>(paths: Vec<P>) -> Library {
        let mut songs = vec![];

        for path in paths {
            if let Ok(mut path_songs) = scan_directory(&path) {
                songs.append(&mut path_songs);
            } else {
                warn!("Failed to read songs at {:?}", path.as_ref());
            }
        }

        Library {
            songs
        }
    }
}

pub fn scan_directory<P: AsRef<Path>>(path: P) -> Result<Vec<SongFile>, Error> {
    let mut songs = vec![];

    for maybe_entry in std::fs::read_dir(path)? {
        let entry = maybe_entry?;

        match load_dir(entry.path()) {
            Ok(Some(songfile)) => songs.push(songfile),
            Ok(None) => {},
            Err(error) => error!("Failed to scan library {:?}: {:?}", entry.path(), error),
        }
    }

    Ok(songs)
}