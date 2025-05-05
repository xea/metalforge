use metalforge_lib::part::{Duration, InstrumentPart, Note, PitchClass};
use metalforge_lib::song::{Arrangement, Instrument, Song, SongHeader, Tuning};
use std::env::args;
use std::fs::{exists, DirBuilder, File};
use std::io::{BufWriter, Error, ErrorKind};
use std::path::Path;

enum Action {
    GenerateConfig,
    GenerateSampleSong,
    ConvertPSARC,
    DumpData,
    ShowHelp,
    None
}

fn main() {
    let mut action = Action::None;
    let mut items = vec![];

    for arg in args().skip(1) {
        match arg.as_str() {
            "generate-config" => action = Action::GenerateConfig,
            "generate-sample-song" => action = Action::GenerateSampleSong,
            "convert-psarc" => action = Action::ConvertPSARC,
            "dump-data" => action = Action::DumpData,
            "help" => action = Action::ShowHelp,
            _ => items.push(arg)
        }
    }

    match action {
        Action::ConvertPSARC => convert_psarc(items),
        Action::DumpData => dump_data(items),
        Action::GenerateConfig => generate_config(),
        Action::GenerateSampleSong => generate_sample_song(),
        Action::ShowHelp | Action::None => print_help()
    }

    /*
    render_sheet(&song);
     */
}

fn generate_config() {

}

fn example_song() -> (SongHeader, Vec<InstrumentPart>) {
    let song_header = SongHeader {
        title: "The Sample Song".to_string(),
        title_sort: "Sample Song, The".to_string(),
        album: "The Sample Album".to_string(),
        album_sort: "Sample Album, The".to_string(),
        artist: "The Sample Artist".to_string(),
        artist_sort: "Sample Artist, The".to_string(),
        year: 2025,
        version: 1,
        length_sec: 30,
        arrangements: vec![
            Arrangement {
                id: "lead_guitar".to_string(),
                name: "Lead Guitar".to_string(),
                instrument: Instrument::ElectricGuitar,
                tuning: Some(Tuning::Standard)
            }
        ],
    };

    let instrument_parts = vec![
        InstrumentPart {
            id: "lead_guitar".to_string(),
            name: "Lead guitar".to_string(),
            notes: vec![
                Note {
                    class: PitchClass::C,
                    octave: 4,
                    time: 1.0,
                    duration: Duration::Whole,
                    velocity: 4,
                    dotted: false,
                    string: 0,
                    fret: 0,
                }
            ],
        }
    ];

    (song_header, instrument_parts)
}

fn generate_sample_song() {
    let (song_header, instrument_parts) = example_song();

    const EXAMPLES_DIR: &str = "examples";
    const SONG_DIR: &str = "examples/sample_song";
    const SONG_YAML: &str = "examples/sample_song/song.yaml";

    // Create main examples directory
    let r = exists(EXAMPLES_DIR)
        .and_then(|exists| match exists {
            true => Ok(()),
            false => std::fs::create_dir(EXAMPLES_DIR)
        })
        // Create the song directory
        .and_then(|_| exists(SONG_DIR))
        .and_then(|exists| match exists {
            true => Ok(()),
            false => std::fs::create_dir(SONG_DIR)
        })
        // Write song.yaml
        .and_then(|_| exists(SONG_YAML))
        .and_then(|exists| match exists {
            true => std::fs::remove_file(SONG_YAML),
            false => Ok(())
        })
        .and_then(|_| File::create(SONG_YAML))
        .and_then(|song_yaml| serde_yaml::to_writer(BufWriter::new(song_yaml), &song_header)
            .map_err(|err| Error::new(ErrorKind::Other, err)))
        // Generate the instrument parts and fail early if there was an error
        .and_then(|_| {
            instrument_parts.iter().fold(Ok(None), |error, part| {
                if let Ok(_) = error {
                    let filename = format!("examples/sample_song/arrangement_{}.yaml", "test_part");

                    exists(&filename)
                        .and_then(|exists| match exists {
                            true => std::fs::remove_file(&filename),
                            false => Ok(())
                        })
                        .and_then(|_| File::create(&filename))
                        .and_then(|file| serde_yaml::to_writer(BufWriter::new(file), part)
                            .map_err(|err| Error::new(ErrorKind::Other, err)))
                        .map(Some)
                } else {
                    error
                }
            })
        });

    match r {
        Ok(_) => println!("Sample song created successfully"),
        Err(error) => {
            eprintln!("Failed to generate sample song: {:?}", error)
        }
    }
}

fn print_help() {
    println!("Metalforge CLI");
    println!("  generate-sample-song            Create a sample song in the examples directory");
    println!("  convert-psarc [FILES or DIRS]   Convert a PSARC file into MFSONG format");
    println!("  help                            Show this help");
}

fn dump_data(items: Vec<String>) {
    let result = load_psarc(items);

    for song_data in result {
        println!("----------");
        println!("Artist: {}", song_data.0.header.artist);
        println!("Title: {}", song_data.0.header.title);
        println!("Album: {}", song_data.0.header.album);
        println!("Year: {}", song_data.0.header.year);
        println!("Length: {} sec", song_data.0.header.length_sec);

        println!("=----------=");

        for arrangement in song_data.0.header.arrangements {
            println!("Arrangement: {} {}", arrangement.id, arrangement.name);
            println!("  Instrument: {:?}", arrangement.instrument);
        }

        for part in &song_data.1 {
            println!("Part: {} Notes: {}", part.name, part.notes.len());

        }
    }
}

fn convert_psarc(items: Vec<String>) {
    let songs_data = load_psarc(items);

    for song in songs_data {
        let id = song_id(song.0.header.artist.as_str(), song.0.header.title.as_str());
        let outdir = format!("out/{}", id);

        if Path::new(outdir.as_str()).exists() {
        }

        if let Ok(_) = DirBuilder::new()
            .recursive(true)
            .create(outdir.as_str()) {

            if let Ok(mfsong_yaml) = File::create(format!("{}/song.yaml", outdir)) {
                let writer = BufWriter::new(mfsong_yaml);

                if let Ok(_) = serde_yaml::to_writer(writer, &song.0.header) {
                    // Ignore
                } else {
                    eprintln!("Failed to write song to output");
                }
            }

            for part in song.1.iter() {
                if let Ok(part_yaml) = File::create(format!("{}/part_{}.yaml", outdir, part.name.as_str())) {
                    let part_writer = BufWriter::new(part_yaml);

                    if let Ok(_) = serde_yaml::to_writer(part_writer, part) {

                    } else {
                        eprintln!("Failed to write part YAML");
                    }
                }
            }
        }
    }
}

fn song_id(p0: &str, p1: &str) -> String {
    let mut id = format!("{}-{}", p0, p1);
    id.push_str(p1);

    id.replace(" ", "_")
}

fn load_psarc(items: Vec<String>) -> Vec<(Song, Vec<InstrumentPart>)>{
    let mut result = vec![];
    for item in items {
        if let Ok(path) = std::fs::canonicalize(item.as_str()) {
            if path.is_file() {
                if let Ok(file) = File::open(path.clone()) {
                    if let Some(pair) = convert_file(file) {
                        result.push(pair);
                    } else {
                        eprintln!("Failed to convert song");
                    }
                } else {
                    eprintln!("Failed to open file: {}", item);
                }
            } else if path.is_dir() {
                let entries = std::fs::read_dir(path)
                    .expect("Failed to read directory")
                    .filter(Result::is_ok)
                    .map(Result::unwrap)
                    .map(|dir_entry| dir_entry.path()
                        .canonicalize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string())
                    .collect();

                let mut r = load_psarc(entries);
                r.drain(..).for_each(|i| result.push(i));
            }
        } else {
            eprintln!("Failed to resolve file: {}", item);
        }
    }

    result
}

fn convert_file(file: File) -> Option<(Song, Vec<InstrumentPart>)> {
    /*
    let mut reader = BufReader::new(file);
    let mut buffer = vec![];
    if let Ok(_) = reader.read_to_end(&mut buffer) {
        if let Ok(result) = convert_psarc_to_mfsong(buffer.as_slice()) {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
    dd
     */
    unimplemented!()
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
