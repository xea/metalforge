#[derive(Clone)]
pub struct Key {
    pub root: NoteClass,
    pub accidental: Accidental,
    pub mode: Mode
}

#[derive(Copy, Clone)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat
}

#[derive(Copy, Clone)]
pub enum NoteClass {
    C, D, E, F, G, A, B
}

#[derive(Copy, Clone)]
pub enum Mode {
    Major,
    Minor,
    // Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    // Aeolian,
    Locrian,
}