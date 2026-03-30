use crate::ui::{despawn_screen, AppState, UIEngine};
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Commands, Component, OnEnter, OnExit, Res, ResMut, Resource};
use bevy::sprite::Text2d;
use metalforge_lib::engine::EngineEvent;
use metalforge_lib::library::Library;

#[derive(Component)]
struct OnMainMenu;

#[derive(Resource)]
struct SongLibrary(Library);

pub fn main_menu(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        .add_systems(FixedUpdate, wait_for_ready);
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Loading..."),
        OnMainMenu
    ));
}

fn wait_for_ready(engine_channel: Res<UIEngine>, mut song_library: ResMut<SongLibrary>) {
    while let Some(event) = engine_channel.channel.try_receive() {
        match event {
            EngineEvent::SongLoaded(song) => {}
            EngineEvent::LibraryReady(library) => {
                song_library.0 = library;
            }
        }
    }
}