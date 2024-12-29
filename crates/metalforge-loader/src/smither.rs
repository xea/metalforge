use metalforge_lib::song::{Arrangement, Instrument, Song, SongHeader};
use rockysmithereens_parser::SongFile as RSSongFile;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use url::Url;

const DEFAULT_URL: &str = "file:///";
const ARRANGEMENT_VOCALS: &str = "Vocals";

pub fn load_song_from_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    let url = path_to_url(path.as_ref())?;

    File::open(path)
        .and_then(|mut file| read_file(&mut file))
        .and_then(parse_songfile)
        .and_then(parse_song)
        .and_then(|mut songs| {
            update_path(&mut songs, url);
            Ok(songs)
        })
}

fn read_file(file: &mut File) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map(|_| bytes)
}

fn parse_songfile(data: Vec<u8>) -> std::io::Result<RSSongFile> {
    RSSongFile::parse(&data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

fn parse_song(song_file: RSSongFile) -> std::io::Result<Vec<Song>> {
    let mut songs: Vec<Song> = vec![];

    for manifest in song_file.manifests.iter() {
        let attributes = manifest.attributes();
        println!("{:#?}", attributes);

        let candidate_result = songs.iter_mut().find(|candidate| {
            candidate.header.artist == attributes.artist_name &&
            candidate.header.album == attributes.album_name &&
            candidate.header.title == attributes.song_name &&
            candidate.header.year == attributes.song_year
        });

        let instrument = Instrument::Guitar6;

        let arrangement = Arrangement {
            name: attributes.arrangement_name.to_string(),
            instrument,
        };

        if let Some(candidate) = candidate_result {
            if manifest.attributes().arrangement_name == ARRANGEMENT_VOCALS {
                println!("Ignoring vocal arrangement")
            } else {
                candidate.add_arrangement(arrangement);
            }
        } else {
            if manifest.attributes().arrangement_name == ARRANGEMENT_VOCALS {
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
                        arrangements: vec![ arrangement ],
                    },
                    path: Url::parse(DEFAULT_URL).unwrap(),
                };

                songs.push(song);
            }
        }
    }

    Ok(songs)
}

fn path_to_url<P: AsRef<Path>>(path: P) -> std::io::Result<Url> {
    std::fs::canonicalize(path)
        .and_then(|cp| {
            Url::from_file_path(cp)
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid path"))
    })
}

fn update_path(songs: &mut Vec<Song>, url: Url) -> () {
    songs.iter_mut().for_each(|song| song.path = url.clone());
}

#[cfg(test)]
mod tests {
    use crate::smither::load_song_from_psarc;

    #[test]
    fn parses_known_good_archives_without_errors() {
        let result = load_song_from_psarc("../../examples/Nokia_Nokia-RingtoneDell_v1_p.psarc");

        assert!(result.is_ok(), "Failed to load songs from psarc: {:?}", result);
    }

    #[test]
    fn parsing_missing_files_yields_error() {
        let result = load_song_from_psarc("../does_not_exist.psarc");

        assert!(result.is_err(), "Expected missing archive but it existed");
    }

    #[test]
    fn path_to_song_file_is_preserved_in_headers() {
        let mut result = load_song_from_psarc("../../examples/Nokia_Nokia-RingtoneDell_v1_p.psarc")
            .expect("Failed to load songs from psarc");

        assert_eq!(1, result.len(), "No songs were loaded");

        let song = result.remove(0);

        assert!(song.path.as_str().starts_with("file://"));
        assert!(song.path.as_str().ends_with("/examples/Nokia_Nokia-RingtoneDell_v1_p.psarc"));
    }

    #[test]
    fn multiple_arrangements_are_merged_into_the_arrangements_of_the_song() {
        let mut result = load_song_from_psarc("../../examples/Nokia_Nokia-RingtoneDell_v1_p.psarc")
            .expect("Failed to load songs from psarc");

        assert_eq!(1, result.len());

        let song = result.remove(0);

        assert!(song.header.arrangements.len() > 1);
    }

}