use metalforge_cpal::run_demo;
// use metalforge_lib::{guitar, piano, Instrument, Song, SongHeader, Arrangement};
// use metalforge_lib::guitar::Sound;
// use metalforge_lib::guitar::Sound::{Chord, Note};

fn main() {
    run_demo();

    /*
    let song = Song {
        info: SongHeader {
            artist: "TestUser".to_string(),
            album: "TestAlbum".to_string(),
            release_year: 2024,
            title: "TestSong".to_string(),
            length: 5,
        },
        arrangements: vec![
            Arrangement {
                instrument: Instrument::Guitar(vec![
                    Note(guitar::Note {
                        string: 5,
                        fret: 3,
                        duration: Default::default(),
                        bend: 0,
                    }),
                    Note(guitar::Note {
                        string: 4,
                        fret: 2,
                        duration: Default::default(),
                        bend: 0,
                    }),
                    Chord(vec![
                        guitar::Note {
                            string: 5,
                            fret: 3,
                            duration: Default::default(),
                            bend: 0,
                        },
                        guitar::Note {
                            string: 4,
                            fret: 2,
                            duration: Default::default(),
                            bend: 0,
                        },
                        guitar::Note {
                            string: 0,
                            fret: 3,
                            duration: Default::default(),
                            bend: 0,
                        }
                    ])
                ]),
            }
        ]
    };

    render_sheet(&song);
     */
}

/*
fn render_sheet(song: &Song) {
    for track in &song.arrangements {
        match &track.instrument {
            Instrument::Guitar(sounds) => render_guitar(sounds),
            Instrument::Piano(sounds) => render_piano(sounds)
        }
    }
}

fn render_guitar(sounds: &Vec<guitar::Sound>) {
    for sound in sounds {
        let mut strings = [ 0, 0, 0, 0, 0, 0 ];

        match sound {
            Sound::Note(note) => {
                strings[note.string as usize] = note.fret;
            }
            Sound::Chord(chord) => {
                for chord_note in chord {
                    if chord_note.string < 6 {
                        strings[chord_note.string as usize] = chord_note.fret;
                    }
                }
            }
            Sound::HammerOn(_, _, _) => {}
            Sound::Slide(_, _, _) => {}
        }

        println!("{} {} {} {} {} {}", strings[5], strings[4], strings[3], strings[2], strings[1], strings[0]);
    }
}

fn render_piano(_sounds: &Vec<piano::Sound>) {

}
 */
