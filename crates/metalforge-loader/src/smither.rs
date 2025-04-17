use crate::loader::load_file_contents;
use metalforge_lib::library::SongLibrary;
use metalforge_lib::song::{Arrangement, Instrument, Song, SongHeader};
use rockysmithereens_parser::SongFile as RSSongFile;
use std::io::{Error, ErrorKind, };
use url::Url;

const DEFAULT_URL: &str = "file:///";
const ARRANGEMENT_VOCALS: &str = "Vocals";

pub(crate) fn load_psarc(url: &Url) -> std::io::Result<SongLibrary> {
    let bytes = load_file_contents(url)?;
    let songs = load_song_from_psarc(&bytes, url)?;

    Ok(SongLibrary::from(songs))
}

fn load_song_from_psarc(bytes: &[u8], url: &Url) -> std::io::Result<Vec<Song>> {
    parse_songfile(bytes)
        .and_then(|song_file| parse_song(song_file, url))
}

/// Attempt to parse the raw contents of a file as a PSARC song or return with an error in case of
/// a failure
fn parse_songfile(data: &[u8]) -> std::io::Result<RSSongFile> {
    RSSongFile::parse(data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

/// Convert a PSARC song file into a Metalforge song file or files.
fn parse_song(song_file: RSSongFile, file_url: &Url) -> std::io::Result<Vec<Song>> {
    let mut songs: Vec<Song> = vec![];

    for manifest in song_file.manifests.iter() {
        let attributes = manifest.attributes();
        println!("{:#?}", attributes);

        let candidate_result = songs.iter_mut().find(|candidate| {
            candidate.header.artist == attributes.artist_name
                && candidate.header.album == attributes.album_name
                && candidate.header.title == attributes.song_name
                && candidate.header.year == attributes.song_year
        });

        let instrument = Instrument::ElectricGuitar;

        let arrangement = Arrangement {
            id: attributes.arrangement_name.to_string(),
            name: attributes.arrangement_name.to_string(),
            instrument,
        };

        if let Some(candidate) = candidate_result {
            if manifest.attributes().arrangement_name == ARRANGEMENT_VOCALS {
                println!("Ignoring vocal arrangement")
            } else {
                candidate.add_arrangement(arrangement);
            }
        } else if manifest.attributes().arrangement_name == ARRANGEMENT_VOCALS {
            println!("Ignoring vocal arrangement")
        } else {
            let song = Song {
                header: SongHeader {
                    title: attributes.song_name.to_string(),
                    title_sort: attributes.song_name_sort.to_string(),
                    album: attributes.album_name.to_string(),
                    album_sort: attributes.album_name_sort.to_string(),
                    artist: attributes.artist_name.to_string(),
                    artist_sort: attributes.artist_name_sort.to_string(),
                    year: attributes.song_year,
                    version: manifest.iteration_version,
                    length_sec: attributes.song_length as u16,
                    arrangements: vec![arrangement],
                },
                path: file_url.clone(),
            };

            songs.push(song);
        }
    }

    Ok(songs)
}

#[cfg(test)]
mod tests {
    use crate::loader::load_file_contents;
    use crate::smither::load_song_from_psarc;
    use std::path::PathBuf;
    use url::Url;

    #[test]
    fn parses_known_good_archives_without_errors() {
        let file_url = Url::from_file_path(PathBuf::from("../../examples/Test-Artist_Test-Song_v1_p.psarc").canonicalize().unwrap()).unwrap();
        let bytes = load_file_contents(&file_url).unwrap();
        let result = load_song_from_psarc(&bytes, &file_url);

        assert!(
            result.is_ok(),
            "Failed to load songs from psarc: {:?}",
            result
        );
    }

    #[test]
    fn path_to_song_file_is_preserved_in_headers() {
        let file_url = Url::from_file_path(PathBuf::from("../../examples/Test-Artist_Test-Song_v1_p.psarc").canonicalize().unwrap()).unwrap();
        let bytes = load_file_contents(&file_url).unwrap();
        let mut result = load_song_from_psarc(&bytes, &file_url)
            .expect("Failed to load songs from psarc");

        assert_eq!(1, result.len(), "No songs were loaded");

        let song = result.remove(0);

        assert!(song.path.as_str().starts_with("file://"));
        assert!(song
            .path
            .as_str()
            .ends_with("/examples/Test-Artist_Test-Song_v1_p.psarc"));
    }

    #[test]
    fn multiple_arrangements_are_merged_into_the_arrangements_of_the_song() {
        let file_url = Url::from_file_path(PathBuf::from("../../examples/Test-Artist_Test-Song_v1_p.psarc").canonicalize().unwrap()).unwrap();
        let bytes = load_file_contents(&file_url).unwrap();
        let mut result = load_song_from_psarc(&bytes, &file_url)
            .expect("Failed to load songs from psarc");

        assert_eq!(1, result.len());

        let song = result.remove(0);

        assert!(song.header.arrangements.len() > 1);
    }
}
