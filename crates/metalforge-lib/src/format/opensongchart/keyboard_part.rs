use serde::{Deserialize, Serialize};
use crate::format::opensongchart::arrangement::SongSection;

#[derive(Serialize, Deserialize)]
pub struct SongKeyboardNotes {

    #[serde(rename = "Sections")]
    pub sections: Vec<SongSection>,

    #[serde(rename = "Notes")]
    pub notes: Vec<SongKeyboardNote>
}

#[derive(Serialize, Deserialize)]
pub struct SongKeyboardNote {

    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

    #[serde(rename = "TimeLength")]
    pub time_length: f32,

    #[serde(rename = "Note")]
    pub note: u16,

    #[serde(rename = "Velocity")]
    pub velocity: u16
}
