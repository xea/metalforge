use metalforge_lib::song::{Arrangement, Instrument, Song, SongHeader};
use rockysmithereens_parser::SongFile as RSSongFile;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use url::Url;

pub fn load_song_from_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    File::open(path)
        .and_then(|mut file| read_file(&mut file))
        .and_then(parse_songfile)
        .and_then(parse_song)
}

fn read_file(file: &mut File) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map(|_| bytes)
}

fn parse_songfile(data: Vec<u8>) -> std::io::Result<RSSongFile> {
    RSSongFile::parse(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn parse_song(song_file: RSSongFile) -> std::io::Result<Vec<Song>> {
    let url_result = std::path::absolute(song_file.song_path())
        .and_then(|path| Url::from_file_path(path)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid path")));

    let mut songs: Vec<Song> = vec![];

    if let Ok(url) = url_result {
        for manifest in song_file.manifests.iter() {
            let attributes = manifest.attributes();
            println!("{:#?}", attributes);

            let mut candidate_result = songs.iter_mut().find(|candidate| {
                candidate.path == url.clone() &&
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
                if manifest.attributes().arrangement_name == "Vocals" {
                    println!("Ignoring vocal arrangement")
                } else {
                    candidate.add_arrangement(arrangement);
                }
            } else {
                if manifest.attributes().arrangement_name == "Vocals" {
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
                        path: url.clone(),
                    };

                    songs.push(song);
                }
            }
        }
    } else {
        println!("Unable to open archive: {}", song_file.song_path());
    }

    Ok(songs)
}