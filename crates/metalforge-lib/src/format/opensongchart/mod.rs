use crate::format::opensongchart::arrangement::SongStructure;
use crate::format::opensongchart::drum_part::DrumSongNotes;
use crate::format::opensongchart::instrument_part::{InstrumentType, SongInstrumentNotes};
use crate::format::opensongchart::keyboard_part::SongKeyboardNotes;
use crate::format::opensongchart::song::Song;
use crate::format::opensongchart::vocal_part::SongVocal;
use crate::library::songfile::{Format, SongFile};
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;

pub mod song;
pub mod arrangement;
pub mod instrument_part;
pub mod vocal_part;
pub mod keyboard_part;
pub mod drum_part;

pub trait SongEvent {}

pub struct OpenSongChart {
    song: Song,
    arrangement: SongStructure,
    instrument_parts: Vec<Part>,
    song_path: String
}

pub enum Part {
    InstrumentPart(SongInstrumentNotes),
    KeyboardPart(SongKeyboardNotes),
    DrumPart(DrumSongNotes),
    VocalPart(SongVocal)
}

pub fn load_open_song_chart<P: AsRef<Path>>(dir: P) -> Result<Option<SongFile>, Error> {
    scan_directory(dir).map(|maybe_chart|
        maybe_chart.map(|chart| {
            SongFile {
                title: chart.song.song_name,
                artist: chart.song.artist_name,
                format: Format::OpenSongChart,
                song_path: chart.song_path
            }
        })
    )
}

pub fn scan_directory<P: AsRef<Path>>(dir: P) -> Result<Option<OpenSongChart>, Error> {
    let file = File::open(&dir)?;
    let metadata = file.metadata()?;

    if metadata.is_dir() {
        let mut song_json_path = Path::to_path_buf(dir.as_ref());
        song_json_path.push("song.json");

        if std::fs::exists(song_json_path.as_path())? {
            let song_reader = BufReader::new(File::open(song_json_path)?);
            let song: Song = serde_json::from_reader(song_reader)?;

            // song.json has been read and parsed, look for arrangement.json
            let mut arrangement_json_path = Path::to_path_buf(dir.as_ref());
            arrangement_json_path.push("arrangement.json");

            if std::fs::exists(arrangement_json_path.as_path())? {
                let arrangement_reader = BufReader::new(File::open(arrangement_json_path)?);
                let arrangement: SongStructure = serde_json::from_reader(arrangement_reader)?;

                let mut parts = vec![];

                for part in &song.instrument_parts {
                    let mut part_path = Path::to_path_buf(dir.as_ref());
                    part_path.push(format!("{}.json", part.instrument_name.as_str()));

                    if std::fs::exists(part_path.as_path())? {
                        let part_reader = BufReader::new(File::open(part_path)?);

                        let part = match part.instrument_type {
                            InstrumentType::LeadGuitar | InstrumentType::RhythmGuitar | InstrumentType::BassGuitar => {
                                Part::InstrumentPart(serde_json::from_reader(part_reader)?)
                            }
                            InstrumentType::Keys => {
                                Part::KeyboardPart(serde_json::from_reader(part_reader)?)
                            }
                            InstrumentType::Drums => {
                                Part::DrumPart(serde_json::from_reader(part_reader)?)
                            }
                            InstrumentType::Vocals => {
                                Part::VocalPart(serde_json::from_reader(part_reader)?)
                            }
                        };

                        parts.push(part);
                    }
                }

                let mut ogg_path = Path::to_path_buf(dir.as_ref());
                ogg_path.push("song.ogg");

                if std::fs::exists(ogg_path.as_path())? {
                    return Ok(Some(OpenSongChart {
                        song,
                        arrangement,
                        instrument_parts: parts,
                        song_path: ogg_path.to_str().unwrap().to_string(),
                    }));
                }
            }
        }

        Ok(None)
    } else {
        Ok(None)
    }
}