use crate::SmithereenLoader;
use metalforge_lib::SongInfo;
use std::fs;
use std::path::{Path, PathBuf};
use crate::smither::create_psarc_ref;

const EXT_PSARC: &str = "psarc";

#[derive(Debug)]
pub struct SongRef {
    pub path: PathBuf,
    pub song_info: SongInfo
}

pub fn find_songs<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<SongRef>> {
    let mut songs = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                match extension.to_str() {
                    Some(EXT_PSARC) => {
                        // Load
                        if let Ok(mut read_songs) = create_psarc_ref(path) {
                            songs.append(&mut read_songs);
                        } else {
                            // Error loading manifest
                        }
                    }
                    Some(_) => {
                        // Unsupported extension, ignore
                    }
                    None => {
                        // Ignore for now
                    }
                }
            }
        } else {
            // Ignore recursion for now
        }
    }

    Ok(songs)
}

#[cfg(test)]
mod tests {
    use crate::explorer::find_songs;

    #[test]
    fn songs_in_example_finds_only_valid_songs() {
        let songs = find_songs("../../../examples");
    }
}