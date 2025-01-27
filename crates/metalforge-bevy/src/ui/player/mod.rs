use crate::ui::menu::{MenuEvent, MenuState};
use crate::ui::{AppState, LibraryView};
use bevy::app::AppExit;
use bevy::color::palettes::basic::{PURPLE, RED};
use bevy::input::ButtonInput;
use bevy::prelude::{
    default, in_state, App, AssetServer, Assets, BuildChildren, Camera2d, ChildBuild, Color,
    ColorMaterial, Commands, Component, Event, EventReader, EventWriter, IntoSystemConfigs,
    KeyCode, Mesh, Mesh2d, MeshMaterial2d, NextState, OnEnter, Query, Rectangle, Res, ResMut,
    Resource, Sprite, State, Text2d, TextFont, Time, Transform, Update, Vec2, Vec3, With,
};
use bevy::text::TextBounds;

#[derive(Event, Copy, Clone)]
pub enum PlayerEvent {}

#[derive(Resource)]
struct PlayerState {}

pub fn player_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Player), setup_player)
        .add_systems(
            Update,
            handle_player_events.run_if(in_state(AppState::Player)),
        )
        .add_systems(Update, handle_keyboard.run_if(in_state(AppState::Player)))
        .add_systems(Update, scroll_nodes.run_if(in_state(AppState::Player)))
        .add_systems(Update, move_camera.run_if(in_state(AppState::Player)));
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    menu_state: Res<MenuState>,
    library: Res<LibraryView>,
) {
    let song = library
        .song_library
        .song(menu_state.selected_song_idx)
        .expect("Unable to open selected song");

    let arrangement = song
        .header
        .arrangements
        .get(menu_state.selected_arrangement_idx)
        .expect("Unable to find selected arrangement");

    commands.spawn((
        Note2d {
            kind: "C".to_string(),
        },
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::new(64., 32., 16.)),
    ));

    commands.spawn((
        Note2d {
            kind: "D".to_string(),
        },
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(RED))),
        // Move this mesh slightly to the left
        Transform::from_xyz(-64., 0., -1.).with_scale(Vec3::splat(32.)),
    ));

    // Text 2D
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        ..default()
    };

    commands.spawn((Text2d::new("C"), text_font.clone()));

    //
    let box_size = Vec2::new(60., 30.);

    commands
        .spawn((
            Note2d {
                kind: "E".to_string(),
            },
            Sprite::from_color(Color::srgb(0.2, 0.3, 0.7), box_size),
            Transform::from_translation(Vec3::new(0.0, 30.0, 0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("D"),
                text_font.clone(),
                TextBounds::from(box_size),
                Transform::from_translation(Vec3::Z),
            ));
        });
}

fn handle_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut events: EventWriter<crate::ui::menu::MenuEvent>,
    mut menu_state: ResMut<MenuState>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::Player => {
                events.send(MenuEvent::OpenMainMenu);
            }
            _ => {
                println!("Ignored escape");
            }
        }
    }
}

// TODO refactor event handler to make it more modular
fn handle_player_events(
    mut events: EventReader<PlayerEvent>,
    mut menu_state: ResMut<MenuState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct Note2d {
    pub kind: String,
}

fn scroll_nodes(mut query: Query<&mut Transform, With<Note2d>>, _time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 0.3;
    }
}

fn move_camera(mut query: Query<&mut Transform, With<Camera2d>>) {
    let Ok(mut camera) = query.get_single_mut() else {
        return;
    };

    camera.translation.x += 0.3;
}

#[cfg(test)]
mod tests {}
