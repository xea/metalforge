mod song_list;
mod main_menu;
mod settings;

use crate::ui::menu::main_menu::setup_main_menu;
use crate::ui::menu::song_list::setup_song_library;
use crate::ui::AppState;
use bevy::color::palettes::css::{GOLD, ORANGE, WHITE};
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, App, AppExit, AppExtStates, AssetServer, BuildChildren, Changed, ChildBuild, Children, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, State, States, Update, With};
use bevy_ui::prelude::{Button, ImageNode};
use bevy_ui::Interaction;
use crate::ui::menu::settings::setup_settings;

pub fn menu_plugin(app: &mut App) {
    app
        .add_event::<MenuEvent>()
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(AppState::SongLibrary), setup_song_library)
        .add_systems(OnExit(AppState::SongLibrary), despawn_screen::<OnSongSelectScreen>)
        .add_systems(OnEnter(AppState::SettingsMenu), setup_settings)
        .add_systems(Update, buttons.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_menu_input.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_menu_events.run_if(in_state(AppState::MainMenu)))
    ;
}

#[derive(Component)]
enum MenuButtonAction {
    OpenLibrary,
    PlaySong,
    Settings,
    BackToMenu,
    Quit,
}

#[derive(Event)]
pub enum MenuEvent {
    OpenMainMenu,
    PrevMenuItem,
    NextMenuItem,
    SelectMenuItem,
    ExitApp,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSongSelectScreen;

fn buttons(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut ImageNode),
        (Changed<Interaction>, With<Button>)
    >
) {
    for (interaction, children, mut image) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                image.color = GOLD.into()
            }
            Interaction::Hovered => {
                image.color = ORANGE.into()
            }
            Interaction::None => {
                image.color = WHITE.into()
            }
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut events: EventWriter<MenuEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            match action {
                MenuButtonAction::OpenLibrary => {
                    app_state.set(AppState::SongLibrary);
                }
                MenuButtonAction::PlaySong => {
                    app_state.set(AppState::Player);
                }
                MenuButtonAction::Settings => {
                    app_state.set(AppState::SettingsMenu);
                }
                MenuButtonAction::BackToMenu => {
                    unimplemented!()
                }
                MenuButtonAction::Quit => {
                    events.send(MenuEvent::ExitApp);
                }
            }
        }
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut events: EventWriter<MenuEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::MainMenu => {
                events.send(MenuEvent::ExitApp);
            }
            AppState::SettingsMenu => {
                events.send(MenuEvent::OpenMainMenu);
            }
            AppState::SongLibrary => {
                events.send(MenuEvent::OpenMainMenu);
            }
            AppState::Player => {
                unimplemented!()
            }
        }
    } else if input.just_pressed(KeyCode::Enter) {
        events.send(MenuEvent::SelectMenuItem);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        events.send(MenuEvent::NextMenuItem);
    } else if input.just_pressed(KeyCode::ArrowUp) {
        events.send(MenuEvent::PrevMenuItem);
    }
}

fn handle_menu_events(
    mut events: EventReader<MenuEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            MenuEvent::PrevMenuItem => {}
            MenuEvent::NextMenuItem => {}
            MenuEvent::SelectMenuItem => {}
            MenuEvent::ExitApp => {
                println!("Exiting app...");
                app_exit_events.send(AppExit::Success);
            }
            MenuEvent::OpenMainMenu => {
                app_state.set(AppState::MainMenu);
            }
        }
    }
}