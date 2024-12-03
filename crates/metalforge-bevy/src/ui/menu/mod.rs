mod song_list;
mod main_menu;

use crate::ui::menu::main_menu::setup_main_menu;
use crate::ui::menu::song_list::setup_songlist;
use crate::ui::AppState;
use bevy::color::palettes::css::{GOLD, ORANGE, WHITE};
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, App, AppExit, AppExtStates, AssetServer, BuildChildren, Changed, ChildBuild, Children, Commands, Component, DespawnRecursiveExt, Entity, EventWriter, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, State, States, Update, With};
use bevy_ui::prelude::{Button, ImageNode};
use bevy_ui::Interaction;

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnEnter(MenuState::Main), setup_main_menu)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::SongList), setup_songlist)
        .add_systems(OnExit(MenuState::SongList), despawn_screen::<OnSongSelectScreen>)
        .add_systems(OnEnter(MenuState::Settings), setup_settings)
        .add_systems(Update, buttons.run_if(in_state(AppState::Menu)))
        .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
        .add_systems(Update, handle_menu_input.run_if(in_state(AppState::Menu)));
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, States)]
enum MenuState {
    #[default]
    Main,
    SongList,
    Settings,
    None
}

#[derive(Component)]
enum MenuButtonAction {
    ChooseSong,
    PlaySong,
    Settings,
    BackToMenu,
    Quit,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSongSelectScreen;

fn setup_menu(
    mut menu_state: ResMut<NextState<MenuState>>
) {
    menu_state.set(MenuState::Main);
}

fn setup_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

}

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
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            match action {
                MenuButtonAction::ChooseSong => {
                    menu_state.set(MenuState::SongList);
                }
                MenuButtonAction::PlaySong => {
                    app_state.set(AppState::Player);
                    menu_state.set(MenuState::None);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::BackToMenu => {
                    unimplemented!()
                }
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
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
    current_menu_state: Res<State<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::Menu => match current_menu_state.get() {
                MenuState::Main => {
                    println!("Exiting app...");
                    menu_state.set(MenuState::None);
                    app_exit_events.send(AppExit::Success);
                }
                MenuState::SongList => {
                    menu_state.set(MenuState::Main);
                }
                MenuState::Settings => {
                    menu_state.set(MenuState::Main);
                }
                MenuState::None => {}
            }
            AppState::Player => {}
        }
    } else if input.just_pressed(KeyCode::Enter) {
        println!("Enter");
    } else if input.just_pressed(KeyCode::ArrowDown) {
        println!("ArrowDown");
    } else if input.just_pressed(KeyCode::ArrowUp) {
        println!("ArrowUp");
    }
}
