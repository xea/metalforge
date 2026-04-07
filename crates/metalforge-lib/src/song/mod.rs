use crate::song::instrument_part::InstrumentPart;
use crate::song::key::{Accidental, Key, Mode, NoteClass};
use crate::song::metadata::Metadata;
use std::time::Duration;

pub mod guitar;
pub mod instrument_part;
pub mod key;
pub mod metadata;

#[derive(Clone)]
pub struct Song {
    pub metadata: Metadata,
    pub instrument_parts: Vec<InstrumentPart>,
    pub beats: Vec<Beat>,
    pub sections: Vec<Section>,
    pub a440_offset_cents: f32
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
                key: None,
            },
            instrument_parts: vec![],
            beats: vec![],
            sections: vec![],
            a440_offset_cents: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Beat {
    /// The start time of this section
    pub time: Duration,
    /// Which measure this beat belongs to, indexed from 1
    pub measure: usize,
    /// Which beat in the measure, indexed from 1
    pub beat_in_measure: u8
}

#[derive(Clone)]
pub struct Section {
    pub name: String,
    pub time: Duration,
    // pub length: Duration,
}