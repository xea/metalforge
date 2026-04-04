use serde::{Deserialize, Serialize};
use crate::format::opensongchart::arrangement::SongSection;

#[derive(Serialize, Deserialize)]
pub struct InstrumentPart {

    #[serde(rename = "InstrumentName")]
    pub instrument_name: String,

    #[serde(rename = "InstrumentType")]
    pub instrument_type: InstrumentType,

    #[serde(rename = "ArrangementName")]
    pub arrangement_name: Option<String>,

    #[serde(rename = "SongAudio")]
    pub song_audio: Option<String>,

    #[serde(rename = "SongStem")]
    pub song_stem: Option<String>,

    #[serde(rename = "Tuning")]
    pub tuning: Option<StringTuning>,

    #[serde(rename = "CapoFret", default)]
    pub capo_fret: i16,

    #[serde(rename = "SongDifficulty", default)]
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
    pub time_offset: Option<f32>,

    /// Sustain length of the note in seconds
    #[serde(rename = "TimeLength")]
    pub time_length: Option<f32>,

    /// Fret number of the note - "0" is open string, "-1" is unfretted
    #[serde(rename = "Fret")]
    pub fret: Option<i8>,

    /// String of the note (zero-based)
    #[serde(rename = "String")]
    pub string: Option<i8>,

    /// Array of bend offsets
    #[serde(rename = "CentsOffset", default)]
    pub cents_offset: Vec<CentsOffset>,

    /// Song technique flags
    #[serde(rename = "Techniques", with = "song_techniques", default)]
    pub techniques: Vec<SongNoteTechniques>,

    /// Bottom fret of hand position
    #[serde(rename = "HandFret")]
    pub hand_fret: Option<i8>,

    /// Fret that note slides to over the course of its sustain
    #[serde(rename = "SlideFret")]
    pub slide_fret: Option<i8>,

    /// Index into chord array to use for notes
    #[serde(rename = "ChordID")]
    pub chord_id: Option<i16>,

    /// Index into chord array to use for fingering
    #[serde(rename = "FingerID")]
    pub finger_id: Option<i16>,

}

impl SongNote {

    //public float EndTime => TimeOffset + TimeLength;
}

mod song_techniques {
    use log::warn;
    use serde::{Deserialize, Deserializer, Serializer};
    use crate::format::opensongchart::instrument_part::SongNoteTechniques;

    pub fn serialize<S>(
        techniques: &Vec<SongNoteTechniques>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let s = format!("{}", date.format(FORMAT));
        // let s = techniques.iter().map(|technique| serde_json::to_string(technique)).collect().join(",");
        // serializer.serialize_str(&s)
        unimplemented!()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SongNoteTechniques>, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;

        let mut techniques = vec![];

        for part in s.split(",") {
            let quoted_part = format!("\"{}\"", part.trim());
            let tq_result: serde_json::Result<SongNoteTechniques> = serde_json::from_str(quoted_part.as_str());

            if let Ok(technique) = tq_result {
                techniques.push(technique);
            } else {
                warn!("Failed to parse technique: {}", s);
            }
        }

        Ok(techniques)
    }
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
