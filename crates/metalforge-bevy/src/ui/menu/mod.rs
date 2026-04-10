pub(crate) mod event;

use std::collections::HashMap;
use std::time::Instant;
use crate::ui::menu::event::{handle_menu_keyboard_events, handle_menu_events, MenuEvent};
use crate::ui::{despawn_screen, exit_menu, AppState, UIEngine};
use bevy::app::{App, FixedUpdate};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{in_state, AppExtStates, Commands, Component, IntoScheduleConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, States, Text, Transform, Update};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::TextColor;
use bevy::ui::{px, Node};
use bevy::utils::default;
use log::info;
use metalforge_lib::engine::{EngineCommand, EngineEvent};
use metalforge_lib::library::Library;
use crate::ui::player::song_player::SongPlayer;

const KEYSTEP_MILLIS: u32 = 100;

/// Marker component to indicate what components are visible on the main menu screen
#[derive(Component)]
pub(crate) struct OnMenu;

/// Marker component to indicate what components are visible on the "Loading" screen
#[derive(Component)]
struct OnLoading;

#[derive(Resource)]
pub(crate) struct SongLibrary(Library);

#[derive(States, Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub(crate) enum MenuState {
    // Preparation phase, data loading, etc.
    LoadData,
    // The main menu state
    ShowMenu,
    // This is a virtual, marker state, to allow for state transitions between menus
    SwitchMenu,
}

pub fn main_menu(app: &mut App) {
    app
        .add_message::<MenuEvent>()
        .insert_state(MenuState::LoadData)
        .insert_resource(SongLibrary(Library::empty()))
        .insert_resource(MenuStructure::default())

        // Main menu systems
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMenu>)

        // Loading screen systems
        .add_systems(OnEnter(MenuState::LoadData), refresh_library)
        .add_systems(OnExit(MenuState::LoadData), despawn_screen::<OnLoading>)

        .add_systems(FixedUpdate, handle_engine_event)

        .add_systems(OnEnter(MenuState::SwitchMenu),
                     |mut next_state: ResMut<NextState<MenuState>>| next_state.set(MenuState::ShowMenu))

        .add_systems(OnEnter(MenuState::ShowMenu), display_menu)
        .add_systems(OnExit(MenuState::ShowMenu), exit_menu::<OnMenu>)
        .add_systems(Update, (handle_menu_keyboard_events, handle_menu_events, highlight_selection).chain()
            .run_if(in_state(MenuState::ShowMenu)));
}

#[derive(Resource)]
pub(crate) struct MenuStructure {
    // Menu definitions
    menus: HashMap<MenuId, Menu>,
    // Which menu is currently selected
    menu_stack: Vec<(MenuId, usize)>,
    // The next selected menu, if there is one
    requested_menu: Option<MenuId>,

    // The currently selected menu item
    pub selected_idx: usize,
    // The menu item the user requested to be selected next
    requested_idx: Option<usize>,

    // The last time the previous/next menu item was selected. Used for rate limiting keyboard actions
    last_update: Instant,
}

impl Default for MenuStructure {

    fn default() -> Self {
        Self {
            selected_idx: 0,
            requested_idx: Some(0),
            last_update: Instant::now(),
            menus: HashMap::from([
                (MenuId::MainMenu, Menu {
                    title: "Main Menu".to_string(),
                    items: vec![
                        MenuItem { label: "Browser".to_string(), action: MenuEvent::PushMenu(MenuId::Browser) },
                        MenuItem { label: "Settings".to_string(), action: MenuEvent::PushMenu(MenuId::Settings) },
                        MenuItem { label: "Exit".to_string(), action: MenuEvent::ExitApp }
                    ]
                }),
                (MenuId::Settings, Menu {
                    title: "Settings".to_string(),
                    items: vec![
                        MenuItem { label: "Debug".to_string(), action: MenuEvent::Noop }
                    ]
                }),
                (MenuId::Browser, Menu {
                    title: "Browser".to_string(),
                    items: vec![]
                }),
                (MenuId::PlayerMenu, Menu {
                    title: "Song Player".to_string(),
                    items: vec![
                        MenuItem { label: "Exit Song".to_string(), action: MenuEvent::ExitSong }
                    ]
                }),
            ]),
            menu_stack: vec![ (MenuId::MainMenu, 0) ],
            requested_menu: Some(MenuId::MainMenu),
        }
    }
}

impl MenuStructure {

    pub fn select_next(&mut self) {
        let now = Instant::now();

        let item_count = self.current_menu()
            .map(|menu| menu.items.len())
            .unwrap_or(0);

        if now.duration_since(self.last_update).as_millis() >= KEYSTEP_MILLIS as u128 {
            let new_idx = self.requested_idx
                .map(|idx| idx + 1)
                .filter(|new_idx| *new_idx < item_count)
                .unwrap_or(0);

            self.requested_idx = Some(new_idx);
            self.last_update = now;
        }
    }

    pub fn select_prev(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_update).as_millis() >= KEYSTEP_MILLIS as u128 {
            let new_idx = self.requested_idx
                .filter(|idx| *idx > 0)
                .map(|idx| idx - 1)
                .unwrap_or_else(|| self.current_menu()
                    .map(|menu| menu.items.len() - 1)
                    .unwrap_or(0));

            self.requested_idx = Some(new_idx);
            self.last_update = now;
        }
    }

    pub fn current_menu(&self) -> Option<&Menu> {
        self.menu_stack.last().and_then(|(id, _idx)| self.menus.get(id))
    }

    pub fn current_item(&self) -> Option<&MenuItem> {
        let menu = self.current_menu();
        menu.unwrap().items.get(self.selected_idx)
    }

    pub fn push_menu(&mut self, menu_id: MenuId) {
        self.menu_stack.push((menu_id, self.selected_idx));
        self.requested_idx = Some(0);
        self.selected_idx = 0;
        self.requested_menu = Some(menu_id);
    }

    pub fn pop_menu(&mut self) -> bool {
        if self.menu_stack.len() > 1 {
            if let Some((menu_id, last_idx)) = self.menu_stack.pop() {
                self.requested_menu = Some(menu_id);
                self.requested_idx = Some(last_idx);
                self.selected_idx = 0;
            }

            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub(crate) struct MenuIdx(usize);

pub(crate) fn display_menu(
    mut commands: Commands,
    menu_struct: Res<MenuStructure>,
) {
    let current_menu = menu_struct.current_menu();

    if let Some(menu) = current_menu {
        commands.spawn((
            Text::new(menu.title.as_str()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            OnMenu
        ));

        for (idx, item) in menu.items.iter().enumerate() {
            commands.spawn((
                Text::new(item.label.as_str()),
                TextColor::from(Color::WHITE),
                Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::new(30.0, 10.0)),
                Node {
                    top: px((idx + 2) * 25),
                    left: px(25),
                    ..default()
                },
                MenuIdx(idx),
                OnMenu
            ));
        }
    }
}

fn refresh_library(mut commands: Commands, engine: Res<UIEngine>) {
    info!("Loading song library");

    let paths = engine.config.library.paths.clone();
    engine.send(EngineCommand::ScanLibrary(paths));

    commands.spawn((
        Text2d::new("Loading..."),
        OnMenu,
        OnLoading
    ));
}

fn handle_engine_event(
    engine_channel: Res<UIEngine>,
    mut commands: Commands,
    mut song_library: ResMut<SongLibrary>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut menu: ResMut<MenuStructure>
) {
    while let Some(event) = engine_channel.channel.try_receive() {
        match event {
            EngineEvent::SongLoaded(song) => {
                commands.insert_resource(SongPlayer::new(song));
                next_app_state.set(AppState::Player);
            }
            EngineEvent::LibraryUpdated(library) => {
                info!("Updating library");
                song_library.0 = library;

                if let Some(browser_menu) = menu.menus.get_mut(&MenuId::Browser) {
                    browser_menu.items.clear();

                    if song_library.0.songs.is_empty() {
                        browser_menu.items.push(MenuItem {
                            label: "[No songs found]".to_string(),
                            action: MenuEvent::Noop,
                        });
                    } else {
                        for (song_idx, song_file) in song_library.0.songs.iter().enumerate() {
                            browser_menu.items.push(
                                MenuItem {
                                    label: format!("{} - {}", song_file.song.metadata.artist, song_file.song.metadata.title),
                                    action: MenuEvent::PlaySong(song_idx),
                                });
                        }
                    }
                }

                next_state.set(MenuState::ShowMenu);
            }
            EngineEvent::SongUnloaded => {
                menu.pop_menu();
                next_state.set(MenuState::SwitchMenu);
                next_app_state.set(AppState::MainMenu);
                // commands.remove_resource::<SongPlayer>();
            }
        }
    }
}

pub(crate) fn highlight_selection(
    mut menu: ResMut<MenuStructure>,
    mut item_q: Query<(&mut TextColor, &MenuIdx)>
) {
    if let Some(new_idx) = menu.requested_idx {
        for (mut color, idx) in item_q.iter_mut() {
            if idx.0 == new_idx {
                // This item is the selected one
                color.0 = Color::srgba(0.3, 0.3, 1.0, 1.0);
            } else {
                // Not selected
                color.0 = Color::WHITE;
            }
        }

        menu.selected_idx = menu.requested_idx.unwrap_or(0);
        menu.requested_menu = None;

    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum MenuId {
    MainMenu,
    PlayerMenu,
    Browser,
    Settings
}

pub struct Menu {
    title: String,
    items: Vec<MenuItem>
}

#[derive(Hash)]
pub struct MenuItem {
    label: String,
    action: MenuEvent
}
