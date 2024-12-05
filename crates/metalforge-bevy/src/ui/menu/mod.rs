//mod song_list;
// mod main_menu;
// mod settings;

use bevy::color::Color;
// use crate::ui::menu::main_menu::setup_main_menu;
// use crate::ui::menu::song_list::setup_song_library;
use crate::ui::AppState;
use bevy::color::palettes::css::{GOLD, ORANGE, RED, WHITE};
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, App, AppExit, AppExtStates, AssetServer, BuildChildren, Changed, ChildBuild, Children, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, State, States, Update, With};
use bevy::text::TextColor;
use bevy_ui::prelude::{Button, ImageNode, Text};
use bevy_ui::{BackgroundColor, FlexDirection, Interaction, Node, Val};
// use crate::ui::menu::settings::setup_settings;

pub fn menu_plugin(app: &mut App) {
    app
        .insert_resource(MenuState::default())
        .add_event::<MenuEvent>()
        .add_systems(OnEnter(AppState::MainMenu), setup_test_menu)
        // .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        // .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        // .add_systems(OnEnter(AppState::SongLibrary), setup_song_library)
        // .add_systems(OnExit(AppState::SongLibrary), despawn_screen::<OnSongLibrary>)
        // .add_systems(OnEnter(AppState::SettingsMenu), setup_settings)
        // .add_systems(OnExit(AppState::SettingsMenu), despawn_screen::<OnSettingsMenu>)
        .add_systems(Update, menu_interactions.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_menu_input.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_menu_events.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, highlight_menu.run_if(in_state(AppState::MainMenu)))
    ;
}

#[derive(Event)]
enum MenuEvent {
    PrevMenuItem,
    NextMenuItem,
    Play,
    Settings,
    Quit
}

#[derive(Component)]
struct MenuIdx(usize);

#[derive(Resource, Default)]
struct MenuState {
    previous_idx: usize,
    selected_idx: usize,
    menu_len: usize
}

fn setup_test_menu(mut commands: Commands, mut state: ResMut<MenuState>) {
    let menu_items = vec![
        (MenuIdx(0), "Play", MenuEvent::Play),
        (MenuIdx(1), "Settings", MenuEvent::Settings),
        (MenuIdx(2), "Quit", MenuEvent::Quit)
    ];

    state.menu_len = menu_items.len();

    commands.spawn(Node {
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }).with_children(|parent| {
        for (idx, title, event) in menu_items.into_iter() {
            let text_color = if idx.0 == state.selected_idx {
                Color::srgb(0.6, 0.6, 1.0)
            } else {
                Color::WHITE
            };

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    ..default()
                },
                BackgroundColor(Color::from(RED)),
            )).with_children(|parent| {
                parent.spawn((
                    Button,
                    idx, event,
                    Text(title.to_string()),
                    TextColor(text_color),
                ));
            });
            /*
            parent.spawn((
                idx, event,
                Button,
                Text(title.to_string()),
                TextColor(text_color),
                BackgroundColor(Color::from(RED)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    ..default()
                }
            ));

             */
        }
    });
}

fn highlight_menu(mut query: Query<(&MenuEvent, &Text, &mut TextColor, &MenuIdx)>, mut menu_state: ResMut<MenuState>) {
    if menu_state.selected_idx != menu_state.previous_idx {
        for ((event, text, mut color, menu_idx)) in query.iter_mut() {
            if menu_idx.0 == menu_state.selected_idx {
                color.0 = Color::srgb(0.6, 0.6, 1.0);
                menu_state.previous_idx = menu_idx.0;
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
}

fn menu_interactions(
    menu_state: Res<MenuState>,
    mut interactions: Query<(&Interaction, &Children, &Text), (Changed<Interaction>, With<Button>)>
) {
    for interaction in interactions.iter_mut() {
        println!("{:?}", interaction);
    }
}

/*
fn menu_interactions(
    menu_items: Query<&MenuEvent>,
    menu_state: Res<MenuState>,
    mut interaction_query: Query<
        (&Interaction, &Children, &mut TextColor),
        // (&Interaction, &Children, &mut ImageNode),
        (Changed<Interaction>, With<MenuEvent>)
    >
) {
    for (interaction, children, mut image) in interaction_query.iter_mut() {
        println!("Interaction: {:?}", interaction);
        match interaction {
            Interaction::Pressed => {
                println!("Pressed");
            }
            Interaction::Hovered => {
                println!("Hovered");
                for (idx, item) in menu_items.iter().enumerate() {
                    if menu_state.selected_idx == idx {
                        println!("Selected menu item #{}", idx);
                    }
                }
            }
            _ => { /* Ignore */ }
        }
    }
}
 */

fn handle_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut events: EventWriter<MenuEvent>,
    mut menu_state: ResMut<MenuState>,
) {
    if input.just_pressed(KeyCode::Escape) {
        /*
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

         */
    } else if input.just_pressed(KeyCode::Enter) {
        // events.send(MenuEvent::SelectMenuItem);
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
            // MenuEvent::SelectMenuItem => {}
            // MenuEvent::ExitApp => {
            //     println!("Exiting app...");
            //     app_exit_events.send(AppExit::Success);
            // }
            // MenuEvent::OpenMainMenu => {
            //     app_state.set(AppState::MainMenu);
            // }
            MenuEvent::Play => {}
            MenuEvent::Settings => {}
            MenuEvent::Quit => {}
        }
    }
}

/*
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
struct OnMainMenu;

#[derive(Component)]
struct OnSongLibrary;

#[derive(Component)]
struct OnSettingsMenu;

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
 */