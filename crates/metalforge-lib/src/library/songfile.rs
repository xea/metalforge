use crate::format::opensongchart::instrument_part::{InstrumentType, SongNote, SongNoteTechniques};
use crate::format::opensongchart::{OpenSongChart, Part};
use crate::song::guitar::{BendPoint, GuitarNote, GuitarPart, GuitarTechnique, GuitarTuning};
use crate::song::instrument_part::{InstrumentPart, InstrumentPartType};
use crate::song::metadata::Metadata;
use crate::song::{Beat, Song};
use std::time::Duration;

#[derive(Clone)]
pub struct SongFile {
    pub format: Format,
    pub song_path: String,
    pub song: Song
}

impl From<OpenSongChart> for SongFile {
    fn from(chart: OpenSongChart) -> Self {
        // Convert beats, tracking measure and beat-within-measure
        let mut measure = 1usize;
        let mut beat_in_measure = 1u8;
        let beats: Vec<Beat> = chart.arrangement.beats.iter().map(|b| {
            if b.is_measure {
                measure += 1;
                beat_in_measure = 1;
            } else {
                beat_in_measure += 1;
            }
            Beat {
                time: Duration::from_secs_f32(b.time_offset),
                measure,
                beat_in_measure,
            }
        }).collect();

        let mut instrument_parts: Vec<InstrumentPart> = vec![];

        for part_def in chart.song.instrument_parts.iter() {
            let matched_part = chart.instrument_parts.iter()
                .find(|p| p.has_part_id(part_def.instrument_name.as_str()));

            let instrument_type = match &part_def.instrument_type {
                guitar_type @ (InstrumentType::LeadGuitar | InstrumentType::RhythmGuitar | InstrumentType::BassGuitar) => {
                    let notes_data = matched_part.and_then(|p| {
                        if let Part::InstrumentPart(_, notes) = p { Some(notes) } else { None }
                    });

                    let tuning = part_def.tuning.as_ref()
                        .map(|t| GuitarTuning {
                            string_offsets: t.string_semitone_offsets.iter().map(|&v| v as i8).collect()
                        })
                        .unwrap_or_else(|| GuitarTuning { string_offsets: vec![0, 5, 10, 15, 19, 24] });

                    let guitar_notes = notes_data.map(|nd| {
                        nd.notes.iter().filter_map(|note| {
                            let string = note.string? as u8;
                            let fret_raw = note.fret?;
                            if fret_raw < 0 { return None; }
                            let fret = fret_raw as u8;
                            let time = Duration::from_secs_f32(note.time_offset.unwrap_or(0.0));
                            let length = Duration::from_secs_f32(note.time_length.unwrap_or(0.1));

                            // Resolve finger: finger_id indexes into chords array, then index by string
                            let finger = note.finger_id
                                .and_then(|fid| nd.chords.get(fid as usize))
                                .and_then(|chord| chord.fingers.get(string as usize))
                                .copied()
                                .unwrap_or(0)
                                .max(0) as u8;

                            let technique = note.techniques.iter()
                                .filter_map(|t| map_technique(t, note))
                                .collect();

                            Some(GuitarNote { string, fret, finger: Some(finger), time, length, technique })
                        }).collect::<Vec<_>>()
                    }).unwrap_or_default();

                    let guitar_part = GuitarPart {
                        notes: guitar_notes,
                        tuning,
                        capo: part_def.capo_fret.max(0) as u8,
                    };

                    match guitar_type {
                        InstrumentType::LeadGuitar => Some(InstrumentPartType::LeadGuitar(guitar_part)),
                        InstrumentType::RhythmGuitar => Some(InstrumentPartType::RhythmGuitar(guitar_part)),
                        InstrumentType::BassGuitar => Some(InstrumentPartType::BassGuitar(guitar_part)),
                        _ => unreachable!()
                    }
                }
                InstrumentType::Keys | InstrumentType::Drums | InstrumentType::Vocals => None,
            };

            if let Some(t) = instrument_type {
                instrument_parts.push(InstrumentPart {
                    name: part_def.instrument_name.clone(),
                    instrument_part_type: t,
                });
            }
        }

        SongFile {
            format: Format::OpenSongChart,
            song_path: chart.song_path,
            song: Song {
                metadata: Metadata {
                    title: chart.song.song_name.clone(),
                    artist: chart.song.artist_name.clone(),
                    album: chart.song.album_name.clone(),
                    year: chart.song.song_year as u16,
                    length: Duration::from_secs_f32(chart.song.song_length_seconds),
                    key: None
                },
                instrument_parts,
                beats,
                sections: vec![],
                a440_offset_cents: 0.0,
            }
        }
    }
}

fn map_technique(t: &SongNoteTechniques, note: &SongNote) -> Option<GuitarTechnique> {
    match t {
        SongNoteTechniques::HammerOn => Some(GuitarTechnique::HammerOn),
        SongNoteTechniques::PullOff => Some(GuitarTechnique::PullOff),
        SongNoteTechniques::PalmMute => Some(GuitarTechnique::PalmMute),
        SongNoteTechniques::FretHandMute => Some(GuitarTechnique::FretHandMute),
        SongNoteTechniques::Slide => {
            let to_fret = note.slide_fret.unwrap_or(0).max(0) as u8;
            Some(GuitarTechnique::Slide { to_fret })
        }
        SongNoteTechniques::Bend => {
            let points = note.cents_offset.iter().map(|co| BendPoint {
                time_offset: Duration::from_secs_f32(co.time_offset),
                cents: co.cents,
            }).collect();
            Some(GuitarTechnique::Bend { points })
        }
        SongNoteTechniques::Tremolo => Some(GuitarTechnique::Tremolo),
        SongNoteTechniques::Vibrato => Some(GuitarTechnique::Vibrato),
        SongNoteTechniques::Harmonic => Some(GuitarTechnique::Harmonic),
        SongNoteTechniques::PinchHarmonic => Some(GuitarTechnique::PinchHarmonic),
        SongNoteTechniques::Tap => Some(GuitarTechnique::Tap),
        SongNoteTechniques::Slap => Some(GuitarTechnique::Slap),
        SongNoteTechniques::Pop => Some(GuitarTechnique::Pop),
        // These no longer exist in the native format — drop them
        SongNoteTechniques::Chord
        | SongNoteTechniques::ChordNote
        | SongNoteTechniques::Continued
        | SongNoteTechniques::Arpeggio
        | SongNoteTechniques::Accent => None,
    }
}

#[derive(Copy, Clone)]
pub enum Format {
    OpenSongChart
}