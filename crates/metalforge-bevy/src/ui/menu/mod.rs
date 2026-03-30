use crate::ui::{despawn_screen, AppState, UIEngine};
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{AppExtStates, Commands, Component, NextState, OnEnter, OnExit, Res, ResMut, Resource, States, Text, Transform};
use bevy::sprite::Text2d;
use bevy::ui::State;
use metalforge_lib::engine::{EngineCommand, EngineEvent};
use metalforge_lib::library::Library;

/// Marker component to indicate what components are visible on the main menu screen
#[derive(Component)]
struct OnMainMenu;

/// Marker component to indicate what components are visible on the "Loading" screen
#[derive(Component)]
struct OnLoading;

#[derive(Resource)]
struct SongLibrary(Library);

#[derive(States, Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum MenuState {
    LibraryRefresh,
    SongBrowser
}

pub fn main_menu(app: &mut App) {
    app
        .insert_state(MenuState::LibraryRefresh)
        .insert_resource(SongLibrary(Library::empty()))
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        .add_systems(OnEnter(MenuState::LibraryRefresh), refresh_library)
        .add_systems(OnExit(MenuState::LibraryRefresh), despawn_screen::<OnLoading>)
        .add_systems(OnEnter(MenuState::SongBrowser), list_songs)
        .add_systems(FixedUpdate, wait_for_ready);
}

fn setup_main_menu(mut commands: Commands, engine: Res<UIEngine>) {
    commands.spawn((
        Text::new("Main Menu"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        OnMainMenu
    ));
}

fn refresh_library(mut commands: Commands, engine: Res<UIEngine>) {
    let paths = engine.config.library.paths.clone();
    engine.send(EngineCommand::ScanLibrary(paths));

    commands.spawn((
        Text2d::new("Loading..."),
        OnMainMenu,
        OnLoading
    ));
}

fn list_songs(mut commands: Commands, library: Res<SongLibrary>) {
    for (idx, song) in library.0.songs.iter().enumerate() {
        commands.spawn((
            Text2d::new(format!("{} - {}", song.artist, song.title)),
            Transform::from_xyz(-50.0, (idx + 1) as f32 * -40.0, 0.0),
            OnMainMenu
        ));
    }
}

fn wait_for_ready(engine_channel: Res<UIEngine>, mut song_library: ResMut<SongLibrary>, mut next_state: ResMut<NextState<MenuState>>) {
    while let Some(event) = engine_channel.channel.try_receive() {
        match event {
            EngineEvent::SongLoaded(song) => {}
            EngineEvent::LibraryUpdated(library) => {
                song_library.0 = library;
                next_state.set(MenuState::SongBrowser);
            }
        }
    }
}