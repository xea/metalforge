use crate::asset::AssetId;
use crate::song::{Arrangement, Instrument, Song, SongHeader, SongType, Tuning};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Defines what metadata of a `Song` is stored in Songfiles.
#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SongDef {
    pub title: String,
    pub title_sort: String,
    pub album: String,
    pub album_sort: String,
    pub artist: String,
    pub artist_sort: String,
    pub year: u16,
    pub version: u16,
    pub length_sec: u16,
    pub cover_art_path: Option<String>,
    pub song_path: Option<String>,
    pub song_preview_path: Option<String>,
    pub arrangements: Vec<ArrangementDef>
}

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrangementDef {
    pub id: String,
    pub name: String,
    pub instrument: Instrument,
    pub tuning: Option<Tuning>
}

impl From<&Song> for SongDef {
    fn from(song: &Song) -> Self {
        let header = &song.header;
        Self {
            title: header.title.to_string(),
            title_sort: header.title_sort.to_string(),
            album: header.album.to_string(),
            album_sort: header.album_sort.to_string(),
            artist: header.artist.to_string(),
            artist_sort: header.artist_sort.to_string(),
            year: header.year,
            version: header.version,
            length_sec: header.length_sec,
            cover_art_path: song.cover_art.as_ref().map(|asset_id| asset_id.id.to_string()),
            song_path: song.song.as_ref().map(|asset_id| asset_id.id.to_string()),
            song_preview_path: song.preview.as_ref().map(|asset_id| asset_id.id.to_string()),
            arrangements: header.arrangements.iter().map(|arrangement| {
                ArrangementDef {
                    id: arrangement.id.to_string(),
                    name: arrangement.name.to_string(),
                    instrument: arrangement.instrument.clone(),
                    tuning: arrangement.tuning.clone(),
                }
            }).collect(),
        }
    }
}

// Loads a file from a specified path and parses it into a `Song`
pub fn load_song<P: AsRef<Path>>(song_path: P) -> std::io::Result<Song> {
    let reader = BufReader::new(File::open(song_path.as_ref())?);

    parse_song(song_path, reader)
}

// Parse the contents of a song definition into a `Song`
fn parse_song<P: AsRef<Path>, R: Read>(song_path: P, read: R) -> std::io::Result<Song> {
    let song_def = load_song_def(read)?;

    let arrangements = song_def.arrangements.iter()
        .map(|arrangement_def| load_arrangement(song_path.as_ref(), arrangement_def))
        .collect();

    let cover_asset_id = song_def.cover_art_path.as_ref()
        .map(|asset_path| AssetId::from_paths(song_path.as_ref(), asset_path));
    let preview_asset_id = song_def.song_preview_path.as_ref()
        .map(|asset_path| AssetId::from_paths(song_path.as_ref(), asset_path));
    let song_asset_id = song_def.song_path.as_ref()
        .map(|asset_path| AssetId::from_paths(song_path.as_ref(), asset_path));

    let song = Song {
        header: load_song_header(song_def, arrangements),
        song_type: SongType::Unpacked,
        cover_art: cover_asset_id,
        preview: preview_asset_id,
        song: song_asset_id
    };

    Ok(song)
}

// Takes a song definition and a list of arrangements and combines them into a song header
fn load_song_header(song_def: SongDef, arrangements: Vec<Arrangement>) -> SongHeader {
    SongHeader {
        title: song_def.title,
        title_sort: song_def.title_sort,
        album: song_def.album,
        album_sort: song_def.album_sort,
        artist: song_def.artist,
        artist_sort: song_def.artist_sort,
        year: song_def.year,
        version: song_def.version,
        length_sec: song_def.length_sec,
        arrangements,
    }
}

// Creates a new `Arrangement` instance from a base path and an arrangement definition
fn load_arrangement<P: AsRef<Path>>(song_path: P, arrangement_def: &ArrangementDef) -> Arrangement {
    let asset_id = AssetId::from_paths(song_path.as_ref(), format!("arrangement_{}.yaml", arrangement_def.id.as_str()).as_str());

    Arrangement {
        id: arrangement_def.id.to_string(),
        asset_id,
        name: arrangement_def.name.to_string(),
        instrument: arrangement_def.instrument,
        tuning: arrangement_def.tuning.clone(),
    }
}

/// Attempts to parse a song definition from a `Reader`, typically representing the contents of a file.
fn load_song_def<R: Read>(reader: R) -> std::io::Result<SongDef> {
    serde_yaml::from_reader(reader)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
}

#[cfg(test)]
mod tests {
    use crate::asset::AssetId;
    use crate::loader::songfile::{load_arrangement, load_song_def, parse_song, ArrangementDef, SongDef};
    use crate::song::{Instrument, Tuning};
    use std::io::{BufReader, ErrorKind};

    #[test]
    fn load_invalid_song_def_from_yaml() {
        let invalid_input = "---\nso\tng:\n";
        let result = load_song_def(BufReader::new(invalid_input.as_bytes()));

        assert!(result.is_err());
        assert_eq!(Some(ErrorKind::Other), result.err().map(|e| e.kind()))
    }

    #[test]
    fn all_arrangement_fields_are_loaded_correctly() {
        let arrangement_def = ArrangementDef {
            id: "test-id".to_string(),
            name: "test-name".to_string(),
            instrument: Instrument::ElectricGuitar,
            tuning: Some(Tuning::Custom(vec![ 0, 1, 2, 3 ])),
        };

        let arrangement = load_arrangement("", &arrangement_def);

        assert_eq!("test-id", arrangement.id);
        assert_eq!("test-name", arrangement.name);
        assert_eq!(Instrument::ElectricGuitar, arrangement.instrument);
        assert_eq!(Some(Tuning::Custom(vec![ 0, 1, 2, 3 ])), arrangement.tuning);
    }

    #[test]
    fn arrangement_asset_id_is_generated_from_the_base_path() {
        let arrangement_def = ArrangementDef {
            id: "test-id".to_string(),
            name: "test-name".to_string(),
            instrument: Instrument::AcousticGuitar,
            tuning: None
        };

        let arrangement = load_arrangement("/base/path", &arrangement_def);

        assert_eq!(AssetId::from_paths("/base/path", "arrangement_test-id.yaml"), arrangement.asset_id);
    }

    #[test]
    fn asset_paths_are_converted_into_asset_id() {
        let song_def = SongDef {
            cover_art_path: Some("assets/art.jpg".to_string()),
            song_preview_path: Some("assets/preview.ogg".to_string()),
            song_path: Some("assets/song.ogg".to_string()),
            .. Default::default()
        };

        let yaml = serde_yaml::to_string(&song_def).unwrap();

        let song = parse_song("/base/path", BufReader::new(yaml.as_bytes())).unwrap();

        assert_eq!(Some(AssetId::from_paths("/base/path", "assets/art.jpg")), song.cover_art);
        assert_eq!(Some(AssetId::from_paths("/base/path", "assets/preview.ogg")), song.preview);
        assert_eq!(Some(AssetId::from_paths("/base/path", "assets/song.ogg")), song.song);
    }
}
