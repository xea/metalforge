use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// An instrument contains the data for the recorded music of a given instrument in a specific
/// arrangement.
#[derive(Serialize, Deserialize)]
pub struct InstrumentPart {
    pub id: String,
    pub name: String,
    // TODO
    // capo
    pub notes: Vec<Note>
}

impl InstrumentPart {
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub class: PitchClass,
    pub octave: u8,
    pub time: f32,
    pub sustain: f32,
    pub duration: Duration,
    // Velocity of the note, 0.. quiet, ..255 loud
    #[serde(default)]
    pub velocity: u8,
    // 0 - High E, 5 - Low E, 6 - Low B
    pub string: u8,
    // 0 - open string, 12 - octave up
    pub fret: u8,

    // Other possible stuff
    // vibrato
    // harmonics
    // tremolo_pick
    // dynamics
    // let sustain = note.sustain;
    // let chord = note.chord;
    // let bend = note.bend;
    // let mute = note.mute;
    // let show = note.show;
    // let slide_to_next = note.slide_to_next;
}

#[derive(Debug, Hash, Eq, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum PitchClass {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c#")]
    CSharp,
    #[serde(rename = "cb")]
    CFlat,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "d#")]
    DSharp,
    #[serde(rename = "db")]
    DFlat,
    #[serde(rename = "e")]
    E,
    #[serde(rename = "e#")]
    ESharp,
    #[serde(rename = "eb")]
    EFlat,
    #[serde(rename = "f")]
    F,
    #[serde(rename = "f#")]
    FSharp,
    #[serde(rename = "fb")]
    FFlat,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "g#")]
    GSharp,
    #[serde(rename = "gb")]
    GFlat,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "a#")]
    ASharp,
    #[serde(rename = "ab")]
    AFlat,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "b#")]
    BSharp,
    #[serde(rename = "bb")]
    BFlat,
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PitchClass::C      => f.write_str("C"),
            PitchClass::CSharp => f.write_str("C#"),
            PitchClass::CFlat  => f.write_str("Cb"),
            PitchClass::D      => f.write_str("D"),
            PitchClass::DSharp => f.write_str("D#"),
            PitchClass::DFlat  => f.write_str("Db"),
            PitchClass::E      => f.write_str("E"),
            PitchClass::ESharp => f.write_str("E#"),
            PitchClass::EFlat  => f.write_str("Eb"),
            PitchClass::F      => f.write_str("F"),
            PitchClass::FSharp => f.write_str("F#"),
            PitchClass::FFlat  => f.write_str("Fb"),
            PitchClass::G      => f.write_str("G"),
            PitchClass::GSharp => f.write_str("G#"),
            PitchClass::GFlat  => f.write_str("Gb"),
            PitchClass::A      => f.write_str("A"),
            PitchClass::ASharp => f.write_str("A#"),
            PitchClass::AFlat  => f.write_str("Ab"),
            PitchClass::B      => f.write_str("B"),
            PitchClass::BSharp => f.write_str("B#"),
            PitchClass::BFlat  => f.write_str("Bb"),
        }
    }
}

impl PitchClass {

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Duration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
}

/*
pub struct GuitarTrack {
    pub notes: Vec<Note>,
}

pub struct Note {
    // 0 - High E, 5 - Low E, 6 - Low B
    pub string: u8,
    // 0 - open string, 12 - octave up
    pub fret: u8,
    pub duration: Duration,
    // multiples of eighth notes, i.e. 8 = full note, 4 = half note, 2 = quarter note
    pub bend: i8
}
 */
