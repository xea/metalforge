use metalforge_lib::part::{Duration, InstrumentPart, Note, PitchClass};
use metalforge_lib::song::Tuning::Standard;
use metalforge_lib::song::{Arrangement, Instrument, Song, SongHeader};
use rockysmithereens_parser::SongFile as RSSongFile;
use std::io::{Error, ErrorKind};

pub fn convert_psarc_to_mfsong(data: &[u8]) -> std::io::Result<(Song, Vec<InstrumentPart>)> {
    /*
    let psarc = RSSongFile::parse(data)
        .map_err(|_rs| Error::from(ErrorKind::InvalidData))?;

    let mut song = Song {
        header: SongHeader {
            title: "".to_string(),
            title_sort: "".to_string(),
            album: "".to_string(),
            album_sort: "".to_string(),
            artist: "".to_string(),
            artist_sort: "".to_string(),
            year: 0,
            version: 0,
            length_sec: 0,
            arrangements: vec![],
        },
    };

    let mut parts = vec![];

    for (idx, manifest) in psarc.manifests.iter().enumerate() {
        let attributes = manifest.attributes();

        if let Ok(rs_song) = psarc.parse_song_info(idx)
            .map_err(|_rs| Error::from(ErrorKind::InvalidData)) {

            let instrument = match attributes.arrangement_name.as_str() {
                "Lead" => Instrument::ElectricGuitar,
                "Rhythm" => Instrument::ElectricGuitar,
                "Bass" => Instrument::ElectricBass,
                _ => Instrument::ElectricGuitar
            };

            // Update existing values
            // attributes.dlc_key = MetallicaForWhomtheBellTolls
            if song.header.artist != attributes.artist_name {
                song.header.artist = attributes.artist_name.clone();
            }

            if song.header.artist_sort != attributes.artist_name_sort {
                song.header.artist_sort = attributes.artist_name_sort.clone();
            }

            if song.header.title != attributes.song_name {
                song.header.title = attributes.song_name.clone();
            }

            if song.header.title_sort != attributes.song_name_sort {
                song.header.title_sort = attributes.song_name_sort.clone();
            }

            if song.header.album != attributes.album_name {
                song.header.album = attributes.album_name.clone();
            }

            if song.header.album_sort != attributes.album_name_sort {
                song.header.album_sort = attributes.album_name_sort.clone();
            }

            if song.header.year != attributes.song_year {
                song.header.year = attributes.song_year;
            }

            if song.header.length_sec != attributes.song_length as u16 {
                song.header.length_sec = attributes.song_length as u16;
            }

            // Populate arrangements
            let arrangement = Arrangement {
                // Maybe use the persistent id?
                id: attributes.full_name.to_string(),
                name: attributes.arrangement_name.to_string(),
                instrument,
                tuning: Some(Standard)
            };

            song.header.arrangements.push(arrangement);

            for level in &rs_song.levels {
                let mut part = InstrumentPart {
                    id: "instrument_id".to_string(),
                    name: format!("{}_diff_{}", attributes.full_name.to_string(), level.difficulty),
                    notes: vec![],
                };

                // TODO store tuning
                // let tuning = attributes.tuning;
                // Tuning pitch

                for note in &level.notes {
                    part.notes.push(Note {
                        class: PitchClass::E,
                        octave: 0,
                        time: note.time * 10.0,
                        duration: Duration::Whole,
                        velocity: 0,
                        dotted: false,
                        string: note.string,
                        fret: note.fret,
                    });
                }

                parts.push(part);
            }
        }
    }
    
     */

    /*
    for note in song.notes_iter() {
        let string = note.string;
        let fret   = note.fret;
        let time  = note.time;
        let sustain = note.sustain;
        let chord = note.chord;
        let bend = note.bend;
        let mute = note.mute;
        let show = note.show;
        let slide_to_next = note.slide_to_next;
        // let vibrato;
        // harmonics;
        // tremolo_pick;
        // dynamics;
    }

     */
    Ok((song, parts))
}


#[cfg(test)]
mod test {


}