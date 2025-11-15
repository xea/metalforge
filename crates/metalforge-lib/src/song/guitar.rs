use std::time::Duration;

pub struct GuitarTuning {
    /// Represents the number of strings the guitar part was written for and their tunings, expressed
    /// as the number of semitones from E2. I.e. low E would be 0, A2 would be 5, D2 would be 10, etc.
    pub string_offsets: Vec<i8>
}

impl From<Vec<i8>> for GuitarTuning {
    fn from(value: Vec<i8>) -> Self {
        Self {
            string_offsets: value
        }
    }
}

impl From<CommonTunings> for GuitarTuning {
    fn from(value: CommonTunings) -> Self {
        value.to_tuning()
    }
}

pub enum CommonTunings {
    EStandard
}

impl CommonTunings {
    pub fn to_tuning(&self) -> GuitarTuning {
        let offsets = match &self {
            CommonTunings::EStandard => vec![ 0, 5, 10, 15, 19, 24 ]
        };

        GuitarTuning::from(offsets)
    }
}

pub struct GuitarPart {
    /// The notes and chords to be played during this part
    pub notes: Vec<GuitarNote>,
    /// The tuning that should be used for this guitar part
    pub tuning: GuitarTuning,
    /// The index of the fret the capo should be placed at. 0 means open strings, i.e. no capo
    pub capo: u8
}

pub struct GuitarNote {
    /// The index of the string the note is played on. 0 means the lowest string on the current instrument
    pub string: u8,
    /// The index of the fret the note is played at, with 0 meaning open string, 12 being the octave, etc.
    pub fret: u8,
    ///The index of the finger that should hold the string for this note. The values are:
    /// - 0 = thumb
    /// - 1 = index
    /// - 2 = middle
    /// - 3 = ring
    /// - 4 = little
    pub finger: u8,
    /// The amount of time since the start of the song to play this note.
    pub time: Duration,
    /// The duration for which the note should be held
    pub length: Duration,
    /// The technique that should be used when playing this note
    pub technique: GuitarTechnique,
    /// The index of the fret this note should slide to
    pub slide_to: u8,
}

pub enum GuitarTechnique {
    None,
    HammerOn,
    PullOff,
    PalmMute,
    FretHandMute,
    Slide,
    Bend,
    Tremolo,
    Vibrato,
    Harmonic,
    PinchHarmonic,
    Tap,
    Chord,
    ChordNote,
    Continued,
    Arpeggio,
    // Bass techniques
    Slap,
    Pop,
}