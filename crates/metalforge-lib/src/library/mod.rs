use crate::format::load_dir;
use crate::library::songfile::{SongFile};
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
            Ok(None) => {
                if entry.metadata()?.is_dir() {
                    let mut sub_result = scan_directory(entry.path().as_path())?;
                    songs.append(&mut sub_result);
                }
            },
            Err(error) => error!("Failed to scan library {:?}: {:?}", entry.path(), error),
        }
    }

    songs.sort_by(|a, b| {
        let artist_cmd = a.song.metadata.artist.as_str().cmp(b.song.metadata.artist.as_str());
        let title_cmd = a.song.metadata.title.as_str().cmp(b.song.metadata.title.as_str());

        artist_cmd.then(title_cmd)
    });

    Ok(songs)
}