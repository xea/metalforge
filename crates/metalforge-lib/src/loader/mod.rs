use crate::song::Song;
use log::trace;
use psarc::load_psarc;
use songfile::load_song;
use std::ffi::OsStr;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use url::Url;
use crate::asset::AssetId;
use crate::part::InstrumentPart;

pub mod psarc;
pub mod songfile;

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

pub trait AssetLoader {
    fn load_instrument_part(asset_id: &AssetId) -> std::io::Result<InstrumentPart>;
}

/// Attempts to scan the configured list of library paths and collect the songs found in them into
/// a single `SongLibrary`.
pub fn scan_libraries<P: AsRef<Path>>(library_root_paths: &Vec<P>) -> std::io::Result<Vec<Song>> {
    let mut songs = vec![];

    for library_root_path in library_root_paths {
        trace!("Scanning library root {}", library_root_path.as_ref().to_string_lossy());

        let mut new_songs = scan_library_dir(library_root_path)?;

        new_songs.drain(..).for_each(|song| songs.push(song));
    }

    trace!("Loaded libraries with a total of {} songs", songs.len());

    Ok(songs)
}

/// Scan a filesystem directory for files that are recognised as songs or libraries.
fn scan_library_dir<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    trace!("Scanning library directory root {}", path.as_ref().to_string_lossy());

    if path.as_ref().exists() {
        if path.as_ref().is_file() {
            return scan_library_dir_entry(path)
                .map(|r| r.unwrap_or_else(|| vec![]));

        } else if path.as_ref().is_dir() {
            return scan_library_dir_entries(path);
        }
    }

    Err(std::io::Error::new(ErrorKind::NotFound, "Specified path does not exist"))
}

fn scan_library_dir_entries<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    trace!("Scanning library directory {}", path.as_ref().to_string_lossy());

    let mut songs = vec![];

    for entry_result in path.as_ref().read_dir()? {
        trace!("Found directory entry {:?}", entry_result);

        if let Ok(dir_entry) = entry_result {
            let entry_path = dir_entry.path();

            if entry_path.is_file() {
                // Append file name to directory URL
                if let Some(mut new_songs) = scan_library_dir_entry(entry_path)? {
                    new_songs.drain(..).for_each(|song| songs.push(song));
                }

            } else if entry_path.is_dir() {
                scan_library_dir_entries(entry_path)?.drain(..).for_each(|song| songs.push(song));
            }
        }
    }

    Ok(songs)
}

fn scan_library_dir_entry<P: AsRef<Path>>(path: P) -> std::io::Result<Option<Vec<Song>>> {
    trace!("Scanning library directory entry {}", path.as_ref().to_string_lossy());

    if let Some(filename) = path.as_ref().file_name().and_then(OsStr::to_str) {
        match filename {
            SONG_DESCRIPTOR => {
                return load_song(path)
                    .map(|song| Some(vec! [ song ]));
            }
            _ => {
                // Filename wasn't known, look for known extensions
                if let Some(extension) = path.as_ref().extension().and_then(OsStr::to_str) {
                    match extension {
                        EXT_MFLIB => {
                            todo!()
                        }
                        EXT_MFSONG => {
                            todo!()
                        }
                        EXT_PSARC => {
                            return load_psarc(path).map(Some);
                        }
                        _ => {
                            // Unknown extension, ignore
                        }
                    }
                }
            }
        }
    }

    Ok(None)
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
