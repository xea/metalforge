use crate::song::guitar::{Beat, CommonTunings, GuitarNote, GuitarPart, GuitarTechnique};
use crate::song::instrument_part::{InstrumentPart, InstrumentPartType};
use crate::song::key::Key;
use crate::song::metadata::Metadata;
use std::time::Duration;
use rand::{RngExt};

pub struct Song {
    pub metadata: Metadata,
    pub instrument_parts: Vec<InstrumentPart>,
    pub beats: Vec<Beat>,
    pub a440_offset: f32
}

impl Song {
    pub fn empty() -> Song {
        Self {
            metadata: Metadata {
                title: "N/A".to_string(),
                artist: "N/A".to_string(),
                album: "N/A".to_string(),
                year: 1970,
                length: Default::default(),
                key: Key::CMajor,
            },
            instrument_parts: vec![],
            beats: vec![],
            a440_offset: 0.0,
        }
    }

    pub fn test_song() -> Self {
        let mut rng = rand::rng();

        let notes = (0..100).map(|i| {
            GuitarNote {
                string: rng.random_range(0..6),
                fret: rng.random_range(0..25),
                finger: rng.random_range(0..5),
                time: Duration::from_millis(i * 1000),
                // length: Duration::from_millis(rng.random_range(0..30) * 100),
                length: Duration::from_secs(1),
                technique: GuitarTechnique::None,
                slide_to: 0
            }
        }).collect();

        let beats = (0..100).map(|i| {
            Beat {
                time: Duration::from_secs(i),
                measure: if i % 4 == 0 { Some(i as usize / 4) } else { None }
            }
        }).collect();

        Self {
            metadata: Metadata {
                title: "Test Song".to_string(),
                artist: "Test Artist".to_string(),
                album: "Test Album".to_string(),
                year: 2025,
                length: Duration::from_mins(3),
                key: Key::CMajor,
            },
            a440_offset: 0.0,
            instrument_parts: vec![
                InstrumentPart {
                    name: "Lead Guitar".to_string(),
                    instrument_type: InstrumentPartType::LeadGuitar(GuitarPart {
                        tuning: CommonTunings::EStandard.into(),
                        capo: 0,
                        notes,
                    }),
                }
            ],
            beats
        }
    }
}