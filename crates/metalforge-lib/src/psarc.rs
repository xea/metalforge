use crate::song::Song;
use rockysmithereens_parser::SongFile as RSSongFile;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::Path;

const DEFAULT_URL: &str = "file:///";
const ARRANGEMENT_VOCALS: &str = "Vocals";

pub(crate) fn load_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    File::open(path.as_ref())
        .map(BufReader::new)
        .and_then(|mut reader| {
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes)
                .map(|_| bytes)
        })
        .and_then(|bytes| load_song_from_psarc(&bytes, path))
}

fn load_song_from_psarc<P: AsRef<Path>>(bytes: &[u8], path: P) -> std::io::Result<Vec<Song>> {
    parse_songfile(bytes)
        .and_then(|song_file| parse_song(song_file, path))
}

/// Attempt to parse the raw contents of a file as a PSARC song or return with an error in case of
/// a failure
fn parse_songfile(data: &[u8]) -> std::io::Result<RSSongFile> {
    RSSongFile::parse(data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

/// Convert a PSARC song file into a Metalforge song file or files.
fn parse_song<P: AsRef<Path>>(song_file: RSSongFile, _path: P) -> std::io::Result<Vec<Song>> {
    let mut songs: Vec<Song> = vec![];
    

    /*
    for manifest in song_file.manifests.iter() {
        let attributes = manifest.attributes();

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
            tuning: Some(Standard)
        };

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
            cover_art: None,
        };

        songs.push(song);
    }
     */

    Ok(songs)
}

#[cfg(test)]
mod tests {}
