mod smither;

use std::fs::read_dir;
use metalforge_lib::library::SongLibrary;
use std::path::Path;
use hashbrown::HashMap;
use crate::smither::load_song_from_psarc;

const EXT_PSARC: &str = "psarc";

/// Attempts to scan a directory on the file system for song files and collect the found songs into
/// a `SongLibrary`. If any of the subdirectories can't be opened or read then the error will be
/// logged but scanning will continue, but if the root directory can't be opened then an error is
/// returned.
pub fn scan_song_directory<P: AsRef<Path>>(path: P) -> std::io::Result<SongLibrary> {
    let mut found_songs = HashMap::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(extension) = entry_path.extension() {
                match extension.to_ascii_lowercase().to_str() {
                    Some(EXT_PSARC) => {
                        if let Ok(songs) = load_song_from_psarc(entry_path) {
                            for song in songs {
                                if found_songs.contains_key(&song.id()) {
                                    println!("Found duplicate song: {}", song.id());
                                } else {
                                    found_songs.insert(song.id(), song);
                                }
                            }
                        };
                    }
                    Some(_other) => {
                        // Ignore unknown extensions
                    }
                    None => {}
                }
            }
        } else if entry_path.is_dir() {
            if let Ok(result) = scan_song_directory(entry_path) {
                // TODO
            }
        }
    }

    Ok(SongLibrary::from(found_songs.into_values().collect::<Vec<_>>()))
}

#[cfg(test)]
mod tests {

}