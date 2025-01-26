mod smither;
mod loader;

use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry};
use metalforge_lib::library::SongLibrary;
use std::path::Path;
use metalforge_lib::song::Song;
use crate::loader::load_library;
use crate::smither::load_song_from_psarc;

/// Metalforge song file type extension
const EXT_MFSONG: &str = "mfsong";
/// Metalforge song library file type extension
const EXT_MFLIB: &str = "mflib";
/// PSARC file format extension
const EXT_PSARC: &str = "psarc";
/// Song library descriptor file name
const LIBRARY_DESCRIPTOR: &str = "library.yaml";
/// Song descriptor file name
const SONG_DESCRIPTOR: &str = "song.yaml";

/// Attempts to scan a directory on the file system for song files and collect the found songs into
/// a `SongLibrary`. If any of the subdirectories can't be opened or read then the error will be
/// logged but scanning will continue, but if the root directory can't be opened then an error is
/// returned.
pub fn scan_song_directory<P: AsRef<Path>>(path: P) -> std::io::Result<SongLibrary> {
    let mut library = SongLibrary::empty();

    for entry in read_dir(path)? {
        let dir_entry = entry?;

        scan_dir_entry(&dir_entry, &mut library)?;
    }

    Ok(library)
}

fn scan_dir_entry(entry: &DirEntry, found_songs: &mut SongLibrary) -> std::io::Result<()> {
    let entry_path = entry.path();

    if entry_path.is_file() {
        if let Some(filename) = entry_path.file_name().and_then(OsStr::to_str) {
            match filename {
                LIBRARY_DESCRIPTOR => {
                    scan_library(entry, found_songs)?;
                }
                SONG_DESCRIPTOR => {
                    scan_song(entry, found_songs)?;
                }
                _ => {
                    // Filename wasn't a known one, look for known extensions
                    if let Some(extension) = entry_path.extension().and_then(OsStr::to_str) {
                        match extension {
                            EXT_PSARC => {
                                scan_psarc(entry, found_songs)?;
                            }
                            _ => {
                                // Unknown extension, ignore
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn scan_psarc(entry: &DirEntry, library: &mut SongLibrary) -> std::io::Result<()> {
    let psarc_songs = load_song_from_psarc(entry.path())?;

    for song in psarc_songs {
        library.add_song(song);
    }

    Ok(())
}

fn scan_song(entry: &DirEntry, library: &mut SongLibrary) -> std::io::Result<()> {
    if let Some(path) = entry.path().to_str() {
    }

    Ok(())
}

fn scan_library(entry: &DirEntry, library: &mut SongLibrary) -> std::io::Result<()> {
    if let Some(path) = entry.path().to_str() {
        let songs = load_library(path)?;

        songs.into_iter().for_each(|song| library.add_song(song));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

}