use std::fs::File;
use std::io::Read;
use std::path::Path;
use rockysmithereens_parser::SongFile;

pub fn load_psarc<P: AsRef<Path>>(path: P) -> std::io::Result<SongFile> {
    File::open(path)
        .and_then(|mut file| read_file(&mut file))
        .and_then(parse_songfile)
}

fn read_file(file: &mut File) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map(|_| bytes)
}

fn parse_songfile(data: Vec<u8>) -> std::io::Result<SongFile> {
    SongFile::parse(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_can_parse_source_items() {
        let r = load_psarc("../../examples/Nokia_Nokia-RingtoneDell_v1_p.psarc");
        assert!(r.is_ok());
    }

    #[test]
    fn when_file_does_not_exist_error_is_returned() {
        let r = load_psarc("./examples/Does_not_exist.psarc");
        assert!(r.is_err());
    }
}
