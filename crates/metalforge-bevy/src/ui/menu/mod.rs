mod song_library;
mod main_menu;
mod settings;

use crate::ui::menu::main_menu::{setup_main_menu, OnMainMenu};
use crate::ui::menu::song_library::{setup_song_library, OnSongLibrary};
use crate::ui::AppState;
use bevy::app::{App, AppExit, Update};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild, DespawnRecursiveExt};
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, Changed, Commands, Component, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, State, With};
use bevy::text::TextColor;
use bevy_ui::prelude::{Button, Text};
use bevy_ui::{FlexDirection, Interaction, Node, Val};
use crate::ui::menu::settings::{setup_settings, OnSettingsMenu};

pub const NORMAL_COLOR: Color = Color::srgb(1., 1., 1.);
pub const HOVERED_COLOR: Color = Color::srgb(0.6, 0.6, 1.);

pub fn menu_plugin(app: &mut App) {
    let menu_handlers = (highlight_menu, handle_menu_mouse, handle_menu_keys, handle_menu_events);

    app
        .insert_resource(MenuState::default())
        .add_event::<MenuEvent>()
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        .add_systems(OnEnter(AppState::SongLibrary), setup_song_library)
        .add_systems(OnExit(AppState::SongLibrary), despawn_screen::<OnSongLibrary>)
        .add_systems(OnEnter(AppState::SettingsMenu), setup_settings)
        .add_systems(OnExit(AppState::SettingsMenu), despawn_screen::<OnSettingsMenu>)
        .add_systems(Update, menu_handlers.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, menu_handlers.run_if(in_state(AppState::SettingsMenu)))
        .add_systems(Update, menu_handlers.run_if(in_state(AppState::SongLibrary)))
    ;
}

#[derive(Event, Copy, Clone, Default)]
enum MenuEvent {
    #[default]
    OpenMainMenu,
    OpenSettingsMenu,
    PrevMenuItem,
    NextMenuItem,
    FocusMenuItem(usize),
    Play,
    Quit,
    Todo,
    Ignore
}

#[derive(Component, Debug)]
struct MenuIdx(usize);

#[derive(Resource, Default)]
pub(crate) struct MenuState {
    previous_idx: usize,
    selected_idx: usize,
    menu_len: usize,
    current_action: MenuEvent
}

fn highlight_menu(mut query: Query<(&MenuEvent, &mut TextColor, &MenuIdx)>, mut menu_state: ResMut<MenuState>) {
    if menu_state.selected_idx != menu_state.previous_idx {
        for (event, mut color, menu_idx) in query.iter_mut() {
            if menu_idx.0 == menu_state.selected_idx {
                color.0 = HOVERED_COLOR;
                menu_state.previous_idx = menu_idx.0;
                menu_state.current_action = *event;
            } else {
                color.0 = NORMAL_COLOR;
            }
        }
    }
}

fn handle_menu_mouse(
    mut events: EventWriter<MenuEvent>,
    mut interactions: Query<(&Interaction, &MenuIdx, &MenuEvent), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, idx, event) in interactions.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                events.send(*event);
            }
            Interaction::Hovered => {
                events.send(MenuEvent::FocusMenuItem(idx.0));
            }
            _ => { /* Ignore */ }
        }
    }
}

fn handle_menu_keys(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut events: EventWriter<MenuEvent>,
    menu_state: ResMut<MenuState>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::MainMenu => {
                events.send(MenuEvent::Quit);
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
        events.send(menu_state.current_action);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        events.send(MenuEvent::NextMenuItem);
    } else if input.just_pressed(KeyCode::ArrowUp) {
        events.send(MenuEvent::PrevMenuItem);
    }
}

fn handle_menu_events(
    mut events: EventReader<MenuEvent>,
    mut menu_state: ResMut<MenuState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            MenuEvent::OpenMainMenu => {
                app_state.set(AppState::MainMenu);
            }
            MenuEvent::OpenSettingsMenu => {
                app_state.set(AppState::SettingsMenu);
            }
            MenuEvent::PrevMenuItem => {
                if menu_state.selected_idx > 0 {
                    menu_state.selected_idx -= 1;
                }
            }
            MenuEvent::NextMenuItem => {
                if menu_state.menu_len > 0 && menu_state.selected_idx < menu_state.menu_len - 1 {
                    menu_state.selected_idx += 1;
                }
            }
            MenuEvent::FocusMenuItem(idx) => {
                if menu_state.menu_len > *idx {
                    menu_state.selected_idx = *idx;
                }
            }
            MenuEvent::Play => {
                menu_state.selected_idx = 0;
                app_state.set(AppState::SongLibrary);
            }
            MenuEvent::Quit => {
                println!("Exiting app...");
                app_exit_events.send(AppExit::Success);
            }
            MenuEvent::Todo => {
                println!("TODO: Feature not implemented yet");
            }
            MenuEvent::Ignore => { /* Ignore */ }
        }
    }
}

fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuItem {
    idx: MenuIdx,
    title: String,
    event: MenuEvent
}

impl From<(usize, &str, MenuEvent)> for MenuItem {
    fn from(value: (usize, &str, MenuEvent)) -> Self {
        MenuItem {
            idx: MenuIdx(value.0),
            title: value.1.to_string(),
            event: value.2
        }
    }
}

pub(crate) fn setup_menu<T: Component>(menu_items: Vec<MenuItem>, tag: T, mut commands: Commands, mut state: ResMut<MenuState>) {
    state.selected_idx = 0;
    state.current_action = menu_items.get(0).map(|item| item.event).unwrap_or(MenuEvent::Ignore);
    state.menu_len = menu_items.len();

    // Container defining the overall outline of the menu, including tagging required for screen de-spawning
    commands.spawn((tag, Node {
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    })).with_children(|parent| {
        // Create a button for each menu item, highlighting the first element in the list
        for menu_item in menu_items.into_iter() {
            let text_color = if menu_item.idx.0 == state.selected_idx {
                HOVERED_COLOR
            } else {
                NORMAL_COLOR
            };

            parent.spawn((
                Button,
                menu_item.idx,
                menu_item.event,
                Text(menu_item.title),
                TextColor(text_color),
                //BackgroundColor(Color::from(RED)),
                Node {
                    width: Val::Percent(50.0),
                    ..default()
                }
            ));
        }
    });
}
