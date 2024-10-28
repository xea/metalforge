use crate::piano::PitchClass;

pub struct Song {
    pub info: SongInfo,
    pub tracks: Vec<Track>,
}

pub struct SongInfo {
    pub title: String,
    pub artist: String,
}

pub struct Track {
    pub instrument: Instrument
}

pub enum Instrument {
    Guitar(Vec<guitar::Sound>),
    Piano(Vec<piano::Sound>)
}

pub struct Chord {
    _chord_type: PitchClass,
    _base_note: Option<PitchClass>,
    _mods: Vec<ChordMods>,
    _octave: i8
}

pub enum ChordMods {
    Major,
    Minor,
    Aug,
    Dim,
    Sus2,
    Sus4,
    Pow5,
    Add7,
    Add9,
    Add11,
    Add13,
}

pub mod piano {
    use std::time::Duration;

    pub enum Sound {
        Note(Note),
        Chord(Vec<Note>)
    }

    pub struct Note {
        pub class: PitchClass,
        pub octave: u8,
        pub duration: Duration,
        // Velocity of the note, 0.. quiet, ..255 loud
        pub velocity: u8
    }

    pub enum PitchClass {
        C, CSharp, CFlat,
        D, DSharp, DFlat,
        E, ESharp, EFlat,
        F, FSharp, FFlat,
        G, GSharp, GFlat,
        A, ASharp, AFlat,
        B, BSharp, BFlat
    }
}

pub mod guitar {
    use std::time::Duration;

    pub enum Sound {
        Note(Note),
        Chord(Vec<Note>),
        HammerOn(Note, Note, Vec<Note>),
        Slide(Note, Note, Vec<Note>)
    }

    pub struct Note {
        // 0 - High E, 5 - Low E, 6 - Low B
        pub string: u8,
        // 0 - open string, 12 - octave up
        pub fret: u8,
        pub duration: Duration,
        // multiples of eighth notes, i.e. 8 = full note, 4 = half note, 2 = quarter note
        pub bend: i8
    }
}

