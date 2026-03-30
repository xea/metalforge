use serde::{Deserialize, Serialize};
use crate::format::opensongchart::arrangement::SongSection;

#[derive(Serialize, Deserialize)]
pub struct DrumSongNotes {

    #[serde(rename = "Sections")]
    pub sections: Vec<SongSection>,

    #[serde(rename = "Notes")]
    pub notes: Vec<SongDrumNote>
}

#[derive(Serialize, Deserialize)]
pub struct SongDrumNote {

    #[serde(rename = "TimeOffset")]
    pub time_offset: f32,

    #[serde(rename = "KitPiece")]
    pub kit_piece: KitPiece,

    #[serde(rename = "Articulation")]
    pub articulation: DrumArticulation
}

impl SongDrumNote {
    
}

#[derive(Serialize, Deserialize)]
pub enum KitPiece {
    None,
    Kick,
    Snare,
    HiHat,
    Crash,
    Crash2,
    Crash3,
    Ride,
    Ride2,
    Tom1,
    Tom2,
    Tom3,
    Tom4,
    Tom5,
    Flexi1,
    Flexi2,
    Flexi3,
    Flexi4
}

pub enum DrumKitPieceType {
    None,
    Kick,
    Snare,
    HiHat,
    Crash,
    Ride,
    Tom,
    Flexi
}

#[derive(Serialize, Deserialize)]
pub enum DrumArticulation {
    None,
    DrumHead,
    DrumHeadEdge,
    DrumRim,
    SideStick,
    HiHatClosed,
    HiHatOpen,
    HiHatChick,
    HiHatSplash,
    CymbalEdge,
    CymbalBow,
    CymbalBell,
    CymbalChoke,
    FlexiA,
    FlexiB,
    FlexiC
}