use crate::song::guitar::GuitarPart;

pub struct InstrumentPart {
    pub name: String,
    pub instrument_type: InstrumentPartType
}

pub enum InstrumentPartType {
    LeadGuitar(GuitarPart),
    RhythmGuitar(GuitarPart),
    BassGuitar(GuitarPart),
}