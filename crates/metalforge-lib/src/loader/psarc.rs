use std::collections::HashMap;
use rockysmithereens_parser::SongFile as RSSongFile;
use rockysmithereens_parser::song::Song as RSSong;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::Path;
use crate::asset::AssetId;
use crate::loader::AssetLoader;
use crate::part::{Duration, InstrumentPart, Note, PitchClass};
use crate::song::{Arrangement, Instrument, Song, SongHeader, SongType, Tuning};

pub struct PSARCAssetLoader;

impl AssetLoader for PSARCAssetLoader {
    fn load_instrument_part(asset_id: &AssetId) -> std::io::Result<InstrumentPart> {
        let song_file = load_raw_psarc(asset_id.parent_id.as_str())?;
        if let Some(parsed_song) = song_file.entities.iter().enumerate()
            .find(|(_idx, entity)| entity.id.to_ascii_lowercase() == asset_id.id.to_ascii_lowercase())
            .and_then(|(idx, _entity)| song_file.parse_song_info(idx).ok()) {

            Ok(InstrumentPart {
                id: asset_id.id.to_string(),
                name: "".to_string(),
                notes: parse_instrument_part(&parsed_song),
            })
        } else {
            Err(std::io::Error::new(ErrorKind::NotFound, "Asset for instrument part not found"))
        }
    }
}

pub(crate) fn load_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Song>> {
    load_raw_psarc(path.as_ref())
        .and_then(|song_file| parse_song(song_file, path.as_ref()))
}

fn load_raw_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<RSSongFile> {
    File::open(path.as_ref())
        .map(BufReader::new)
        .and_then(|mut reader| {
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes)
                .map(|_| bytes)
        })
        .and_then(|bytes| parse_songfile(bytes.as_slice()))
}

/// Attempt to parse the raw contents of a file as a PSARC song or return with an error in case of
/// a failure
fn parse_songfile(data: &[u8]) -> std::io::Result<RSSongFile> {
    RSSongFile::parse(data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

/// Convert a PSARC song file into a Metalforge song file or files.
fn parse_song<P: AsRef<Path>>(song_file: RSSongFile, song_path: P) -> std::io::Result<Vec<Song>> {
    // let mut songs: Vec<Song> = vec![];
    let mut songs: HashMap<&str, Song> = HashMap::new();
    
    for manifest in song_file.manifests.iter() {
        let attributes = manifest.attributes();

        // attributes.arrangement_properties gives various flags like bends, bass_pick, sustain, barre_chords,
        // double_stops, finger_picking, pick_direction, etc.
        let instrument = if attributes.arrangement_properties.path_lead > 0 || attributes.arrangement_properties.path_bass > 0 {
            Instrument::ElectricGuitar
        } else {
            Instrument::ElectricBass
        };

        let tuning = vec![
            attributes.tuning.string_0,
            attributes.tuning.string_1,
            attributes.tuning.string_2,
            attributes.tuning.string_3,
            attributes.tuning.string_4,
            attributes.tuning.string_5,
        ];

        let arrangement = Arrangement {
            id: attributes.arrangement_name.to_string(),
            asset_id: AssetId::from_paths(song_path.as_ref(), attributes.persistent_id.as_str()),
            name: attributes.arrangement_name.to_string(),
            instrument,
            tuning: Some(Tuning::Custom(tuning))
        };

        let song_key = attributes.song_key.as_str();

        if let Some(song) = songs.get_mut(song_key) {
            song.header.arrangements.push(arrangement);
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
                song_type: SongType::PSARC,
                cover_art: Some(AssetId::from_paths(song_path.as_ref(), attributes.album_art.as_str())),
                song: Some(AssetId::from_paths(song_path.as_ref(), attributes.song_bank.as_str())),
                preview: Some(AssetId::from_paths(song_path.as_ref(), attributes.preview_bank_path.as_ref())),
            };

            songs.insert(song_key, song);
        }
    }

    Ok(songs.into_values().collect())
}

fn parse_instrument_part(song: &RSSong) -> Vec<Note> {
    let mut notes = vec![];

    for level in &song.levels {
        // Ignore difficulty for now
        for note in &level.notes {
            let new_note = Note {
                class: PitchClass::C,
                octave: 0,
                time: note.time * 4.0,
                sustain: note.sustain.unwrap_or(0.0),
                duration: Duration::Whole,
                velocity: 0,
                string: note.string,
                fret: note.fret,
            };

            notes.push(new_note);
        }
    }

    notes
}

#[cfg(test)]
mod tests {

}
