use std::env::args;
use std::fs::{DirBuilder, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;
use metalforge_loader::converter::convert_psarc_to_mfsong;
use url::Url;
use metalforge_lib::song::Song;
use metalforge_lib::track::Track;

enum Action {
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
            "convert-psarc" => action = Action::ConvertPSARC,
            "dump-data" => action = Action::DumpData,
            "help" => action = Action::ShowHelp,
            _ => items.push(arg)
        }
    }

    match action {
        Action::ConvertPSARC => convert_psarc(items),
        Action::DumpData => dump_data(items),
        Action::ShowHelp | Action::None => print_help()
    }

    /*
    render_sheet(&song);
     */
}
fn print_help() {
    println!("Metalforge CLI");
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

        for track in &song_data.1 {
            println!("Track: {} Notes: {}", track.name, track.notes.len());

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

            for track in song.1.iter() {
                if let Ok(track_yaml) = File::create(format!("{}/arrangement_{}.yaml", outdir, track.name.as_str())) {
                    let track_writer = BufWriter::new(track_yaml);

                    if let Ok(_) = serde_yaml::to_writer(track_writer, track) {

                    } else {
                        eprintln!("Failed to write tack YAML");
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
/*
fn convert_psarc0(url: Url, file: File) {
    if let Some((song, tracks)) = convert_file(url.clone(), file) {

        /*
        let out = File::create("out/out.yaml")
            .expect("Failed to open output file");

        let writer = BufWriter::new(out);

        let _ = serde_yaml::to_writer(writer, &song.header);
         */
    } else {
        eprintln!("Failed to convert PSARC file: {}", url);
    }
}

 */

fn load_psarc(items: Vec<String>) -> Vec<(Song, Vec<Track>)>{
    let mut result = vec![];
    for item in items {
        if let Ok(path) = std::fs::canonicalize(item.as_str()) {
            if path.is_file() {
                if let Ok(file) = File::open(path.clone()) {
                    if let Ok(url) = Url::from_file_path(path) {
                        if let Some(pair) = convert_file(url, file) {
                            result.push(pair);
                        } else {
                            eprintln!("Failed to convert song");
                        }
                    } else {
                        eprintln!("Failed to convert path to URL");
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

fn convert_file(url: Url, file: File) -> Option<(Song, Vec<Track>)> {
    let mut reader = BufReader::new(file);
    let mut buffer = vec![];
    if let Ok(_) = reader.read_to_end(&mut buffer) {
        if let Ok(result) = convert_psarc_to_mfsong(url, buffer.as_slice()) {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
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
