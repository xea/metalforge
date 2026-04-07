use crate::song::guitar::GuitarPart;

#[derive(Clone)]
pub struct InstrumentPart {
    pub name: String,
    pub instrument_part_type: InstrumentPartType
}

#[derive(Clone)]
pub enum InstrumentPartType {
    LeadGuitar(GuitarPart),
    RhythmGuitar(GuitarPart),
    BassGuitar(GuitarPart),
    Keyboard,
    Drums,
    Vocals
}