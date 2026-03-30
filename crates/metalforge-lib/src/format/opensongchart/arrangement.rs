use serde::{Deserialize, Serialize};
use crate::format::opensongchart::SongEvent;

#[derive(Serialize, Deserialize)]
pub struct SongStructure {

    #[serde(rename = "Sections")]
    pub sections: Vec<SongSection>,

    #[serde(rename = "Beats")]
    pub beats: Vec<SongBeat>

}

#[derive(Serialize, Deserialize)]
pub struct SongBeat {

    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

    #[serde(rename = "IsMeasure")]
    pub is_measure: bool,

    #[serde(rename = "EndTime")]
    pub end_time: f32
}

impl SongEvent for SongBeat {}

#[derive(Serialize, Deserialize)]
pub struct SongSection {

    #[serde(rename = "Beats")]
    pub name: String,

    #[serde(rename = "StartTime")]
    pub start_time: f32,

    #[serde(rename = "EndTime")]
    pub end_time: f32
}

