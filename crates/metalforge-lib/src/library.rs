use crate::song::Song;

/// `SongLibrary` manages a collection of loaded songs.
#[derive(Debug, Default)]
pub struct SongLibrary {
    songs: Vec<Song>,
}

impl SongLibrary {
    /// Creates a new, empty Song Library.
    pub fn empty() -> SongLibrary {
        Self { songs: Vec::new() }
    }

    pub fn song(&self, idx: usize) -> Option<&Song> {
        self.songs.get(idx)
    }

    pub fn iter(&self) -> std::slice::Iter<Song> {
        self.songs.iter()
    }

    pub fn merge(&mut self, other: &mut SongLibrary) {
        for song in other.songs.drain(..other.songs.len()) {
            self.songs.push(song);
        }
    }
}

impl From<Vec<Song>> for SongLibrary {
    fn from(songs: Vec<Song>) -> Self {
        Self { songs }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;
    use crate::library::SongLibrary;
    use crate::song::{Song, SongHeader};

    #[test]
    fn merging_two_libraries_moves_the_songs_in_other_to_self() {
        let songs = vec![
            Song {
                header: SongHeader {
                    title: "Test Song".to_string(),
                    title_sort: "Test Song, The".to_string(),
                    album: "Album".to_string(),
                    album_sort: "Album, The".to_string(),
                    artist: "Artist".to_string(),
                    artist_sort: "Artist, The".to_string(),
                    year: 2024,
                    version: 1,
                    length_sec: 30,
                    arrangements: vec![],
                },
                cover_art: None,
            }
        ];
        let mut library = SongLibrary::empty();
        let mut other = SongLibrary::from(songs);

        library.merge(&mut other);

        assert_eq!(1, library.songs.len());
        assert_eq!(0, other.songs.len());
    }

}
