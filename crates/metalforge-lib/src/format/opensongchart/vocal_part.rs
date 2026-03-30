use serde::{Deserialize, Serialize};

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