use bevy::prelude::{Resource, States};
use metalforge_loader::explorer::SongRef;

pub(crate) mod player;
pub(crate) mod menu;

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    SongLibrary,
    Player
}

#[derive(Resource, Debug)]
pub struct Library {
    songs: Vec<SongRef>
}

#[derive(Resource, Debug, Default)]
pub struct RunState {
    pub current_menu_idx: usize
}

impl Library {
    pub fn new(songs: Vec<SongRef>) -> Self {
        Self { songs }
    }
}