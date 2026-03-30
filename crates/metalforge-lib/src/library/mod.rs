use crate::format::load_dir;
use crate::library::songfile::SongFile;
use std::io::Error;
use std::path::{Iter, Path};
use log::warn;

pub mod songfile;

pub struct Library {
    songs: Vec<SongFile>
}

impl Library {

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

        if let Ok(Some(songfile)) = load_dir(entry.path()) {
            songs.push(songfile);
        }
    }

    Ok(songs)
}