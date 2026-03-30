use serde::{Deserialize, Serialize};
use crate::format::opensongchart::arrangement::SongSection;

#[derive(Serialize, Deserialize)]
pub struct InstrumentPart {

    #[serde(rename = "InstrumentName")]
    pub instrument_name: String,

    #[serde(rename = "InstrumentType")]
    pub instrument_type: InstrumentType,

    #[serde(rename = "ArrangementName")]
    pub arrangement_name: String,

    #[serde(rename = "SongAudio")]
    pub song_audio: String,

    #[serde(rename = "SongStem")]
    pub song_stem: String,

    #[serde(rename = "Tuning")]
    pub tuning: StringTuning,

    #[serde(rename = "CapoFret")]
    pub capo_fret: i16,

    #[serde(rename = "SongDifficulty")]
    pub song_difficulty: f32
}

#[derive(Serialize, Deserialize)]
pub enum InstrumentType {
    LeadGuitar,
    RhythmGuitar,
    BassGuitar,
    Keys,
    Drums,
    Vocals
}

#[derive(Serialize, Deserialize)]
pub struct StringTuning {
    #[serde(rename = "StringSemitoneOffsets")]
    string_semitone_offsets: Vec<i16>
}

impl StringTuning {
    //TODO
}

#[derive(Serialize, Deserialize)]
pub struct SongInstrumentNotes {

    #[serde(rename = "Sections")]
    pub sections: Vec<SongSection>,

    #[serde(rename = "Chords")]
    pub chords: Vec<SongChord>,

    #[serde(rename = "Notes")]
    pub notes: Vec<SongNote>
}

#[derive(Serialize, Deserialize)]
pub struct SongChord {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Fingers")]
    pub fingers: Vec<i8>,

    #[serde(rename = "Frets")]
    pub frets: Vec<i8>
}

#[derive(Serialize, Deserialize)]
pub struct SongNote {

    /// Start offset of the note in seconds
    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

    /// Sustain length of the note in seconds
    #[serde(rename = "TimeLength")]
    pub time_length: f32,

    /// Fret number of the note - "0" is open string, "-1" is unfretted
    #[serde(rename = "Fret")]
    pub fret: i8,

    /// String of the note (zero-based)
    #[serde(rename = "String")]
    pub string: i8,

    /// Array of bend offsets
    #[serde(rename = "CentsOffset")]
    pub cents_offset: Vec<CentsOffset>,

    /// Song technique flags
    #[serde(rename = "Techniques")]
    pub techniques: SongNoteTechniques,

    /// Bottom fret of hand position
    #[serde(rename = "HandFret")]
    pub hand_fret: i8,

    /// Fret that note slides to over the course of its sustain
    #[serde(rename = "SlideFret")]
    pub slide_fret: i8,

    /// Index into chord array to use for notes
    #[serde(rename = "ChordID")]
    pub chord_id: i16,

    /// Index into chord array to use for fingering
    #[serde(rename = "FingerID")]
    pub finger_id: i8,

}

impl SongNote {

    //public float EndTime => TimeOffset + TimeLength;
}

/// Offset structure for bends
#[derive(Serialize, Deserialize)]
pub struct CentsOffset {

    /// Time offset of the bend position
    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

    /// Amount of the bend, in cents (100th of a semitone)
    #[serde(rename = "Cents")]
    pub cents: i16
}

#[derive(Serialize, Deserialize)]
pub enum SongNoteTechniques {
    HammerOn = 1 << 1,
    PullOff = 1 << 2,
    Accent = 1 << 3,
    PalmMute = 1 << 4,
    FretHandMute = 1 << 5,
    Slide = 1 << 6,
    Bend = 1 << 7,
    Tremolo = 1 << 8,
    Vibrato = 1 << 9,
    Harmonic = 1 << 10,
    PinchHarmonic = 1 << 11,
    Tap = 1 << 12,
    Slap = 1 << 13,
    Pop = 1 << 14,
    Chord = 1 << 15,
    ChordNote = 1 << 16,
    Continued = 1 << 17,
    Arpeggio = 1 << 18
}
