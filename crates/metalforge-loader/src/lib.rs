mod smither;
pub mod explorer;

use crate::smither::load_psarc;
use metalforge_lib::{Song, SongInfo};
use rockysmithereens_parser::SongFile;
use std::path::Path;

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
                info: SongInfo {
                    title: attributes.song_name.clone(),
                    artist: attributes.artist_name.clone(),
                    album: attributes.album_name.clone(),
                    release_year: attributes.song_year,
                    length: attributes.song_length as u16,
                },
                tracks: vec![],
            }
        }).collect()
    }
}
