use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SongVocals {
    // Note: this struct does not exist in the original spec, I invented it to make it consistent with the rest of the spec
    pub vocals: Vec<SongVocal>
}

#[derive(Serialize, Deserialize)]
pub struct SongVocal {

    #[serde(rename = "Vocal")]
    pub vocal: String,

    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

}

impl SongVocal {

    pub fn end_time(&self) -> f32 {
        self.time_offset
    }

}