use bevy::prelude::{Resource, States};
use metalforge_lib::engine::Engine;
use metalforge_lib::library::SongLibrary;
use metalforge_lib::song::Song;
// use metalforge_loader::explorer::SongRef;

pub(crate) mod menu;
pub(crate) mod player;

#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    SongLibrary,
    Arrangements,
    Player,
}

#[derive(Resource)]
pub struct EngineView(pub Engine);

#[derive(Resource, Debug)]
pub struct LibraryView {
    song_library: SongLibrary,
}

impl LibraryView {
    pub fn iter(&self) -> impl Iterator<Item = &Song> {
        self.song_library.iter()
    }
}

impl From<SongLibrary> for LibraryView {
    fn from(song_library: SongLibrary) -> Self {
        Self { song_library }
    }
}

