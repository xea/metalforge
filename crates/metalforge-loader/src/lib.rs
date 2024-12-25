mod smither;
pub mod explorer;

use std::fs::read_dir;
use metalforge_lib::library::SongLibrary;
use std::path::Path;
use hashbrown::HashMap;
use crate::smither::load_song_from_psarc;

pub fn scan_song_directory<P: AsRef<Path>>(path: P) -> std::io::Result<SongLibrary> {
    let mut found_songs = HashMap::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                match extension.to_ascii_lowercase().to_str() {
                    Some("psarc") => {
                        if let Ok(songs) = load_song_from_psarc(path) {
                            for song in songs {
                                if found_songs.contains_key(&song.id()) {
                                    println!("Found duplicate song: {}", song.id());
                                } else {
                                    found_songs.insert(song.id(), song);
                                }
                            }
                        };
                    }
                    Some(other) => {
                        // Ignore unknown extensions
                    }
                    None => {}
                }
            }
        }
    }

    Ok(SongLibrary::from(found_songs.into_values().collect::<Vec<_>>()))
}

/*
pub struct SmithereenLoader;

impl SmithereenLoader {
    fn load_file<P: AsRef<Path>>(source: P) -> std::io::Result<Vec<Song>> {
        load_psarc(source)
            .map(|song_file| Self::convert_psarc(&song_file))
    }

    fn convert_psarc(source: &SongFile) -> Vec<Song> {
        source.manifests.iter().map(|manifest| {
            let attributes = manifest.attributes();

            Song {
                info: SongHeader {
                    title: attributes.song_name.clone(),
                    artist: attributes.artist_name.clone(),
                    album: attributes.album_name.clone(),
                    release_year: attributes.song_year,
                    length: attributes.song_length as u16,
                },
                arrangements: vec![],
            }
        }).collect()
    }
}
 */