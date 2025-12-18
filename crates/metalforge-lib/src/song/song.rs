use crate::song::guitar::{CommonTunings, GuitarNote, GuitarPart, GuitarTechnique};
use crate::song::instrument_part::{InstrumentPart, InstrumentPartType};
use crate::song::key::Key;
use crate::song::metadata::Metadata;
use std::time::Duration;

pub struct Song {
    pub metadata: Metadata,
    pub instrument_parts: Vec<InstrumentPart>,
    pub a440_offset: f32
}

impl Song {
    pub fn test_song() -> Self {
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
                        notes: vec![
                            GuitarNote {
                                string: 0, fret: 0, finger: 0,
                                time: Duration::from_millis(0), length: Duration::ZERO,
                                technique: GuitarTechnique::None,
                                slide_to: 0,
                            },
                            GuitarNote {
                                string: 3, fret: 0, finger: 0,
                                time: Duration::from_millis(1000), length: Duration::from_millis(1000),
                                technique: GuitarTechnique::None,
                                slide_to: 0,
                            },
                            GuitarNote {
                                string: 5, fret: 2, finger: 0,
                                time: Duration::from_millis(2000), length: Duration::from_millis(500),
                                technique: GuitarTechnique::None,
                                slide_to: 0,
                            }
                        ],
                    }),
                }
            ]
        }
    }
}