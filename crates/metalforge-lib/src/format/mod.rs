use std::io::Error;
use std::path::Path;
use crate::format::opensongchart::load_open_song_chart;
use crate::library::songfile::SongFile;

pub mod opensongchart;

pub fn load_dir<P: AsRef<Path>>(path: P) -> Result<Option<SongFile>, Error> {
    load_open_song_chart(path)
}