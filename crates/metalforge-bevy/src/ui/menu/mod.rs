mod event;

use std::collections::HashMap;
use std::time::Instant;
use crate::ui::menu::event::{handle_keyboard_events, handle_menu_events, MenuEvent};
use crate::ui::{despawn_screen, exit_menu, AppState, UIEngine};
use bevy::app::{App, FixedUpdate};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{in_state, AppExtStates, Commands, Component, IntoScheduleConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, States, Text, Transform, Update};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::TextColor;
use bevy::ui::{px, Node};
use bevy::utils::default;
use metalforge_lib::engine::{EngineCommand, EngineEvent};
use metalforge_lib::library::Library;

const KEYSTEP_MILLIS: u32 = 100;

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
    LoadData,
    ShowMenu,
    SwitchMenu,
}

#[derive(Resource)]
pub(crate) struct MenuStructure {
    // Menu definitions
    menus: HashMap<MenuId, Menu>,
    // Which menu is currently selected
    menu_stack: Vec<(MenuId, usize)>,
    requested_menu: Option<MenuId>,

    // The currently selected menu item
    pub selected_idx: usize,
    // The menu item the user requested to be selected next
    requested_idx: usize,

    // The last time the previous/next menu item was selected. Used for rate limiting keyboard actions
    last_update: Instant,
}

impl Default for MenuStructure {

    fn default() -> Self {
        Self {
            selected_idx: usize::MAX,
            requested_idx: 0,
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
                    items: vec![
                        MenuItem { label: "Item 1".to_string(), action: MenuEvent::PlaySong },
                        MenuItem { label: "Item 2".to_string(), action: MenuEvent::PlaySong },
                        MenuItem { label: "Item 3".to_string(), action: MenuEvent::PlaySong },
                        MenuItem { label: "Item 4".to_string(), action: MenuEvent::PlaySong },
                        MenuItem { label: "Item 5".to_string(), action: MenuEvent::Noop },
                        MenuItem { label: "Item 6".to_string(), action: MenuEvent::Noop },
                        MenuItem { label: "Item 7".to_string(), action: MenuEvent::Noop },
                        MenuItem { label: "Item 8".to_string(), action: MenuEvent::Noop },
                        MenuItem { label: "Item 9".to_string(), action: MenuEvent::Noop },
                        MenuItem { label: "Item 10".to_string(), action: MenuEvent::Noop },
                    ]
                })
            ]),
            menu_stack: vec![ (MenuId::MainMenu, 0) ],
            requested_menu: Some(MenuId::MainMenu),
        }
    }
}

impl MenuStructure {

    pub fn select_next(&mut self) {
        let now = Instant::now();

        let item_count = self.current_menu().map(|menu| menu.items.len()).unwrap_or(0);

        if now.duration_since(self.last_update).as_millis() >= KEYSTEP_MILLIS as u128 {
            if self.requested_idx + 1 < item_count {
                self.requested_idx += 1;
            } else {
                self.requested_idx = 0;
            }

            self.last_update = now;
        }
    }

    pub fn select_prev(&mut self) {
        let now = Instant::now();


        if now.duration_since(self.last_update).as_millis() >= KEYSTEP_MILLIS as u128 {
            if self.requested_idx > 0 {
                self.requested_idx -= 1;
            } else {
                self.requested_idx = self.current_menu().map(|menu| menu.items.len() - 1).unwrap_or(0);
            }

            self.last_update = now;
        }

    }

    pub fn current_menu(&self) -> Option<&Menu> {
        self.menu_stack.last().and_then(|(id, idx)| self.menus.get(id))
    }

    pub fn current_item(&self) -> Option<&MenuItem> {
        let menu = self.current_menu();
        let current_item = menu.unwrap().items.get(self.selected_idx);

        current_item
    }

    pub fn push_menu(&mut self, menu_id: MenuId) {
        self.menu_stack.push((menu_id, self.requested_idx));
        self.requested_idx = 0;
        self.selected_idx = usize::MAX;
        self.requested_menu = Some(menu_id);
    }

    pub fn pop_menu(&mut self) -> bool {
        if self.menu_stack.len() > 1 {
            if let Some((menu_id, last_idx)) = self.menu_stack.pop() {
                self.requested_menu = Some(menu_id);
                self.requested_idx = last_idx;
                self.selected_idx = usize::MAX;
            }

            true
        } else {
            false
        }
    }
}

#[derive(Component)]
struct MenuIdx(usize);

pub fn main_menu(app: &mut App) {
    app
        .add_message::<MenuEvent>()
        .insert_state(MenuState::LoadData)
        .insert_resource(SongLibrary(Library::empty()))
        .insert_resource(MenuStructure::default())
        // Main menu systems
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)

        .add_systems(OnEnter(MenuState::LoadData), refresh_library)
        .add_systems(OnExit(MenuState::LoadData), despawn_screen::<OnLoading>)
        .add_systems(FixedUpdate, handle_engine_event
            .run_if(in_state(MenuState::LoadData)))

        .add_systems(OnEnter(MenuState::SwitchMenu),
                     |mut next_state: ResMut<NextState<MenuState>>| next_state.set(MenuState::ShowMenu))

        .add_systems(OnEnter(MenuState::ShowMenu), display_menu)
        .add_systems(OnExit(MenuState::ShowMenu), exit_menu::<OnMainMenu>)
        .add_systems(Update, (handle_keyboard_events, handle_menu_events, highlight_selection).chain()
            .run_if(in_state(MenuState::ShowMenu)));
        /*
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        // Loading screen systems
        .add_systems(OnEnter(MenuState::LibraryRefresh), refresh_library)
        .add_systems(OnExit(MenuState::LibraryRefresh), despawn_screen::<OnLoading>)
        .add_systems(FixedUpdate, wait_for_ready
            .run_if(in_state(MenuState::LibraryRefresh)))
        // Song browser systems
        .add_systems(OnEnter(MenuState::SongBrowser), list_songs)
        .add_systems(Update, (handle_keyboard_events, handle_menu_events, highlight_selection)
            .run_if(in_state(MenuState::SongBrowser)));
         */
}

fn display_menu(
    mut commands: Commands,
    menu_struct: Res<MenuStructure>,
) {
    let current_menu = menu_struct.current_menu();

    if let Some(menu) = current_menu {
        commands.spawn((
            Text::new(menu.title.as_str()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            OnMainMenu
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
                OnMainMenu
            ));
        }
    }
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

fn list_songs(
    mut commands: Commands,
    library: Res<SongLibrary>,
    mut menu: ResMut<MenuStructure>,
    _ui: Res<UIEngine>
) {
    commands.spawn(());

    for (idx, song) in library.0.songs.iter().enumerate() {
        commands.spawn((
            Text::new(format!("{} - {}", song.artist, song.title)),
            TextColor::from(Color::WHITE),
            Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::new(30.0, 10.0)),
            Node {
                top: px((idx + 2) * 25),
                left: px(25),
                ..default()
            },
            MenuIdx(idx),
            OnMainMenu
        ));
    }
}

fn handle_engine_event(engine_channel: Res<UIEngine>, mut song_library: ResMut<SongLibrary>, mut next_state: ResMut<NextState<MenuState>>) {
    while let Some(event) = engine_channel.channel.try_receive() {
        match event {
            EngineEvent::SongLoaded(_song) => {}
            EngineEvent::LibraryUpdated(library) => {
                song_library.0 = library;
                next_state.set(MenuState::ShowMenu);
            }
        }
    }
}

fn highlight_selection(mut menu: ResMut<MenuStructure>, mut item_q: Query<(&mut TextColor, &MenuIdx)>) {
    if menu.requested_idx != menu.selected_idx || menu.requested_menu.is_some() {
        for (mut color, idx) in item_q.iter_mut() {
            if idx.0 == menu.requested_idx {
                // This item is the selected one
                color.0 = Color::srgba(0.3, 0.3, 1.0, 1.0);
            } else {
                // Not selected
                color.0 = Color::WHITE;
            }
        }

        menu.selected_idx = menu.requested_idx;
        menu.requested_menu = None;
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum MenuId {
    MainMenu,
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
