pub(crate) mod event;

use crate::ui::menu::event::{handle_menu_events, MenuEvent};
use crate::ui::{despawn_screen, exit_menu, AppState, UIEngine};
use bevy::app::App;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{in_state, AppExtStates, Commands, Component, IntoScheduleConfigs, OnEnter, OnExit, Query, Res, ResMut, Resource, States, Text, Transform, Update};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::TextColor;
use bevy::ui::{px, Node};
use bevy::utils::default;
use log::info;
use metalforge_lib::engine::{EngineCommand};
use metalforge_lib::library::songfile::SongFile;
use metalforge_lib::library::Library;
use std::collections::HashMap;
use std::time::Instant;

const KEYSTEP_MILLIS: u32 = 100;

/// Marker component to indicate what components are visible on the main menu screen
#[derive(Component)]
pub(crate) struct OnMenu;

/// Marker component to indicate what components are visible on the "Loading" screen
#[derive(Component)]
struct OnLoading;

#[derive(Resource)]
pub(crate) struct SongLibrary(pub(crate) Library);

#[derive(States, Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub(crate) enum MenuState {
    // Preparation phase, data loading, etc.
    LoadData,
    // Menu is showing
    ShowMenu,
    // Menu is hidden
    HideMenu,
}

pub fn main_menu(app: &mut App) {
    app
        .add_message::<MenuEvent>()
        .insert_state(MenuState::LoadData)
        .insert_resource(SongLibrary(Library::empty()))

        // Main menu systems
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMenu>)

        // Loading screen systems
        .add_systems(OnEnter(MenuState::LoadData), refresh_library)
        .add_systems(OnExit(MenuState::LoadData), despawn_screen::<OnLoading>)

        .add_systems(OnEnter(MenuState::ShowMenu), show_menu)
        .add_systems(OnExit(MenuState::ShowMenu), exit_menu::<OnMenu>)
        .add_systems(Update, (handle_menu_events, highlight_selection).chain()
            .run_if(in_state(MenuState::ShowMenu)));
}

#[derive(Resource)]
pub(crate) struct MenuStructure {
    // Menu definitions
    pub menus: HashMap<MenuId, Menu>,
    // Which menu is currently selected
    pub menu_stack: Vec<(MenuId, usize)>,
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
                        MenuItem {
                            label: "Browser".to_string(),
                            action: MenuEvent::PushMenu(MenuId::Browser),
                        },
                        MenuItem {
                            label: "Settings".to_string(),
                            action: MenuEvent::PushMenu(MenuId::Settings),
                        },
                        MenuItem {
                            label: "Exit".to_string(),
                            action: MenuEvent::ExitApp,
                        }
                    ],
                    pop_action: MenuEvent::Noop,
                }),
                (MenuId::Settings, Menu {
                    title: "Settings".to_string(),
                    items: vec![
                        MenuItem {
                            label: "Debug".to_string(),
                            action: MenuEvent::Noop,
                        }
                    ],
                    pop_action: MenuEvent::PopMenu,
                }),
                (MenuId::Browser, Menu {
                    title: "Browser".to_string(),
                    items: vec![],
                    pop_action: MenuEvent::PopMenu,
                }),
                (MenuId::PlayerMenu, Menu {
                    title: "Song Player".to_string(),
                    items: vec![
                        MenuItem {
                            label: "Exit Song".to_string(),
                            action: MenuEvent::ExitSong,
                        }
                    ],
                    pop_action: MenuEvent::HideMenu,
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

    pub fn current_menu_id(&self) -> Option<MenuId> {
        self.menu_stack.last().map(|(id, _idx)| *id)
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

pub(crate) fn show_menu(
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

pub fn populate_song_browser(browser_menu: &mut Menu, songs: &Vec<SongFile>) {
    browser_menu.items.clear();

    if songs.is_empty() {
        reset_songs(browser_menu);
    } else {
        for (song_idx, song_file) in songs.iter().enumerate() {
            browser_menu.items.push(song_to_menu(song_idx, song_file));
        }
    }
}

fn song_to_menu(song_idx: usize, song_file: &SongFile) -> MenuItem {
    MenuItem {
        label: format!("{} - {}", song_file.song.metadata.artist, song_file.song.metadata.title),
        action: MenuEvent::PlaySong(song_idx),
    }
}

fn reset_songs(browser_menu: &mut Menu) {
    browser_menu.items.push(MenuItem {
        label: "[No songs found]".to_string(),
        action: MenuEvent::Noop,
    });
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum MenuId {
    MainMenu,
    PlayerMenu,
    Browser,
    Settings,
}

#[derive(Debug)]
pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub pop_action: MenuEvent
}

#[derive(Hash, Debug)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuEvent,
}
