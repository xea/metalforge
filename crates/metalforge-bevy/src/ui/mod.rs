use bevy::prelude::{Resource, States};
use metalforge_loader::explorer::SongRef;

pub(crate) mod player;
pub(crate) mod menu;

#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    SongLibrary,
    Arrangements,
    Player
}

#[derive(Resource, Debug)]
pub struct Library {
    songs: Vec<SongRef>
}

#[derive(Resource, Debug, Default)]
pub struct RunState {
}

impl Library {
    pub fn new(songs: Vec<SongRef>) -> Self {
        Self { songs }
    }
}