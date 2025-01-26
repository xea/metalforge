use crate::song::Song;

/// `SongLibrary` manages a collection of loaded songs.
#[derive(Debug)]
pub struct SongLibrary {
    songs: Vec<Song>
}

impl SongLibrary {

    /// Creates a new, empty Song Library.
    pub fn empty() -> SongLibrary {
        Self {
            songs: Vec::new()
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub fn song(&self, idx: usize) -> Option<&Song> {
        self.songs.get(idx)
    }

    pub fn iter(&self) -> std::slice::Iter<Song> {
        self.songs.iter()
    }
}

impl From<Vec<Song>> for SongLibrary {
    fn from(songs: Vec<Song>) -> Self {
        Self {
            songs
        }
    }
}

#[cfg(test)]
mod tests {

}