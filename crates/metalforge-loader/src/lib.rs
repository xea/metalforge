mod loader;
mod smither;

use crate::loader::{load_library, load_song};
use crate::smither::load_psarc;
use log::trace;
use metalforge_lib::library::SongLibrary;
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::path;
use url::Url;

/// Metalforge song file type extension
const EXT_MFSONG: &str = "mfsong";
/// Metalforge song library file type extension
const EXT_MFLIB: &str = "mflib";
/// PSARC file format extension
const EXT_PSARC: &str = "psarc";
/// Song library descriptor file name
const LIBRARY_DESCRIPTOR: &str = "library.yaml";
/// Song descriptor file name
const SONG_DESCRIPTOR: &str = "song.yaml";

/// Attempts to scan the configured list of library paths and collect the songs found in them into
/// a single `SongLibrary`.
pub fn scan_libraries(paths: &Vec<String>) -> std::io::Result<SongLibrary> {
    let mut library = SongLibrary::empty();

    for library_path in paths {
        trace!("Scanning library root {}", library_path);

        let url = build_url(library_path)?;

        trace!("Library resolved to URL: {}", url);

        let mut new_library = scan_library_url(&url)?;

        library.merge(&mut new_library);
    }

    trace!("Loaded libraries with a total of {} songs", library.iter().len());

    Ok(library)
}

/// Take a string slice representing some path that may be an absolute or a relative path or a URL
/// and attempt to parse it into a `Url` instance
///
/// Note: the current implementation is prone to detecting drive letters in absolute paths on Windows
/// as URL schemes. (TODO: fix it at some point it becomes important)
fn build_url(path: &str) -> std::io::Result<Url> {
    let url_result = Url::parse(path)
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidInput, err));

    let absolute_path = path::absolute(path)
        .and_then(|abs_path| Url::from_file_path(abs_path)
            .map_err(|_err| std::io::Error::from(ErrorKind::InvalidInput)));

    url_result.or(absolute_path)
}

/// Scan the given URL for library descriptors, song descriptor or song archives. If the URI points
/// to multiple songs (such as a library) then all songs will be added to the library.
fn scan_library_url(url: &Url) -> std::io::Result<SongLibrary> {
    match url.scheme() {
        "file" => scan_library_dir(url),
        _ => Ok(SongLibrary::empty()) // Ignore URL, scheme unsupported or unknown,
    }
}

/// Scan a filesystem directory for files that are recognised as songs or libraries. Any items found
/// will be read into a new `SongLibrary`.
fn scan_library_dir(url: &Url) -> std::io::Result<SongLibrary> {
    trace!("Scanning library directory root {}", url);

    let url_path = url.to_file_path().expect("Failed to convert url to path");

    if url_path.exists() {
        if url_path.is_file() {
            return scan_library_dir_entry(url);

        } else if url_path.is_dir() {
            return scan_library_dir_entries(url);

        }
    }

    Ok(SongLibrary::empty())
}

fn scan_library_dir_entries(url: &Url) -> std::io::Result<SongLibrary> {
    trace!("Scanning library directory {}", url);

    let mut library = SongLibrary::empty();

    let url_path = url.to_file_path().expect("Failed to convert url to path");

    for entry_result in url_path.read_dir()? {
        trace!("Found directory entry {:?}", entry_result);

        if let Ok(dir_entry) = entry_result {
            let entry_path = dir_entry.path();

            if entry_path.is_file() {
                // Append file name to directory URL
                let mut entry_url = url.clone();
                let mut entry_path = entry_url.to_file_path().expect("Failed to convert url to path");
                entry_path.push(dir_entry.file_name());
                entry_url.set_path(entry_path.to_str().unwrap());

                let mut new_library = scan_library_dir_entry(&entry_url)?;

                library.merge(&mut new_library);
            } else {
                // Currently ignore, but entertain the possibility to do recursion
            }
        }
    }

    Ok(library)
}

fn scan_library_dir_entry(url: &Url) -> std::io::Result<SongLibrary> {
    trace!("Scanning library directory entry {}", url);

    let entry_path = url.to_file_path().expect("Failed to convert url to path");

    println!("Scanning {:?}", url);

    if let Some(filename) = entry_path.file_name().and_then(OsStr::to_str) {
        match filename {
            LIBRARY_DESCRIPTOR => {
                return load_library(url);
            }
            SONG_DESCRIPTOR => {
                return load_song(url);
            }
            _ => {
                // Filename wasn't a known one, look for known extensions
                if let Some(extension) = entry_path.extension().and_then(OsStr::to_str) {
                    match extension {
                        EXT_MFLIB => {
                            todo!()
                        }
                        EXT_MFSONG => {
                            todo!()
                        }
                        EXT_PSARC => {
                            return load_psarc(url);
                        }
                        _ => {
                            // Unknown extension, ignore
                        }
                    }
                }
            }
        }
    }

    Ok(SongLibrary::empty())
}

#[cfg(test)]
mod tests {
    use crate::{build_url, scan_libraries, scan_library_url};
    use env_logger::Builder;
    use log::LevelFilter;
    use std::path;
    use url::Url;

    #[test]
    fn if_no_library_paths_are_given_then_empty_library_is_created() {
        let result = scan_libraries(&vec![]);

        assert!(result.is_ok());

        assert_eq!(0, result.unwrap().iter().len());
    }

    #[test]
    fn when_given_relative_filesystem_paths_file_scheme_urls_are_built() {
        let url_result = build_url("filename.txt");

        assert!(url_result.is_ok());

        let url = url_result.unwrap();

        assert_eq!(url.scheme(), "file");

        let last_item = url.path_segments().unwrap().last();

        assert_eq!(Some("filename.txt"), last_item);
    }

    #[test]
    fn when_given_absolute_filesystem_paths_file_scheme_urls_are_built() {
        let url = if cfg!(windows) {
            "\\windows\\system32\\filename.txt"
        } else {
            "/system32/filename.txt"
        };

        let url_result = build_url(url);

        assert!(url_result.is_ok());

        let url = url_result.unwrap();

        assert_eq!(url.scheme(), "file");

        let last_item = url.path_segments().unwrap().last();

        assert_eq!(Some("filename.txt"), last_item);
    }

    #[test]
    fn when_given_an_url_the_url_scheme_is_detected() {
        let url_result = build_url("https://localhost/filename.txt");

        assert!(url_result.is_ok());

        let url = url_result.unwrap();

        assert_eq!(url.scheme(), "https");

        let last_item = url.path_segments().unwrap().last();

        assert_eq!(Some("filename.txt"), last_item);
    }

    #[test]
    fn when_an_url_scheme_is_not_known_an_empty_library_is_returned() {
        let library_result = scan_library_url(&Url::parse("unknown://something").unwrap());

        assert!(library_result.is_ok());

        let library = library_result.unwrap();

        assert_eq!(0, library.iter().len());
    }

    #[test]
    fn all_scanned_libraries_are_merged_together() {
        Builder::new()
            .filter(None, LevelFilter::max())
            .init();

        let library_result = scan_libraries(&vec![ "../../library".to_string() ]);

        assert!(library_result.is_ok());

        let library = library_result.unwrap();

        assert_eq!(1, library.iter().len());
    }

    #[test]
    fn when_the_library_url_points_to_filesystem_the_contents_from_disk_are_read() {
        let path = path::absolute("../../library").unwrap();
        let url = Url::from_file_path(path).unwrap();
        let library_result = scan_libraries(&vec![ url.to_string() ]);

        assert!(library_result.is_ok());

        let library = library_result.unwrap();

        assert_eq!(1, library.iter().len());
    }
}
