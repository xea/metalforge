use crate::ui::AppState;
use bevy::app::{App, FixedUpdate, Update};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::camera::Camera2d;
use bevy::math::Vec2;
use bevy::mesh::Mesh;
use bevy::prelude::{default, in_state, AppExtStates, Bundle, Color, ColorMaterial, Commands, IntoScheduleConfigs, Message, OnEnter, Query, Res, ResMut, Resource, States, SystemCondition, Text, Transform, With};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::{Font, TextColor, TextFont};
use bevy::time::Time;
use bevy::ui::BackgroundColor;
use std::time::Duration;
use metalforge_lib::note::Note;

#[derive(Message, Copy, Clone)]
pub enum PlayerEvent {
    StartPlaying,
    PausePlaying,
    ResumePlaying
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Playing,
    Paused,
}

#[derive(Resource, Default)]
pub struct Player {
    song_position: Duration
}

pub fn player_plugin(app: &mut App) {
    app
        .insert_state(PlayerState::Playing)
        .insert_resource(Player::default())
        .add_message::<PlayerEvent>()
        .add_systems(OnEnter(AppState::Player), setup_player)
        .add_systems(FixedUpdate, update_player.run_if(in_state(AppState::Player)))
        .add_systems(Update, move_camera.run_if(in_state(AppState::Player).and(in_state(PlayerState::Playing))));
}

/// Initialise the tab player screen
fn setup_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    // Load song
    let notes = [
        Note {}
    ];


    // Prepare assets
    // - fonts can be loaded using asset_server.load("fonts/LelandText.otf");

    // Draw vertical cursor
    create_cursor(&mut commands);

    // Draw tab string lines
    create_string_lines(&mut commands);

    // Draw the individual notes

    for note in notes.iter() {
        create_note(&mut commands, &note);
    }

    /*
    let shape = meshes.add(Rectangle::new(40.0, 30.0));
    let color = Color::srgb(0.7, 0.7, 0.7);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(30.0, 0.0, 0.0)
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("C#"),
                ));
        });

     */
}

#[derive(Bundle)]
struct Cursor {
    sprite: Sprite,
    transform: Transform
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 120.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Bundle)]
struct TitleText {
    text: Text,
    text2: Text2d,
    font: TextFont,
    color: TextColor
}

impl TitleText {
    pub fn new(_font_handle: Handle<Font>) -> Self {
        Self {
            text: Text::new("MetaL Bundle 𝅘𝅥𝅰 \\m/"),
            text2: Text2d::new("MetaL Bundle 2D"),
            font: TextFont {
                // font: font_handle, but it's not used because the text doesn't show when it is and I don't feel like debugging this right now.
                font_size: 30.0,
                ..default()
            },
            color: TextColor(Color::srgb(0.7, 0.7, 0.7)),
        }
    }
}

fn create_cursor(commands: &mut Commands) {
    commands.spawn(Cursor::new());
}

fn create_string_lines(commands: &mut Commands) {
    let string_e4 = (
        Sprite::from_color(Color::srgb(1.0, 0.0, 1.0), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, 45.0, -1.0)
    );

    let string_b3 = (
        Sprite::from_color(Color::srgb(0.2, 1.0, 0.2), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, 30.0, -1.0)
    );

    let string_g3 = (
        Sprite::from_color(Color::srgb(0.8, 0.4, 0.0), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, 15.0, -1.0)
    );

    let string_d3 = (
        Sprite::from_color(Color::srgb(0.2, 0.2, 1.0), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, 0.0, -1.0)
    );

    let string_a2 = (
        Sprite::from_color(Color::srgb(0.8, 0.8, 0.0), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, -15.0, -1.0)
    );

    let string_e2 = (
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(150.0, 2.0)),
        Transform::from_xyz(75.0, -30.0, -1.0)
    );

    commands.spawn(string_e4);
    commands.spawn(string_b3);
    commands.spawn(string_g3);
    commands.spawn(string_d3);
    commands.spawn(string_a2);
    commands.spawn(string_e2);
}

fn create_note(commands: &mut Commands, note: &Note) {

}

fn update_player(mut player: ResMut<Player>, time: Res<Time>) {
    player.song_position = time.elapsed();
}

fn move_camera(mut query: Query<&mut Transform, With<Camera2d>>, player: Res<Player>) {
    /*
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    let prev_x = camera.translation.x;
    camera.translation.x = player.song_position.as_millis() as f32 / 10.0;
     */


    /*
    if player_state.song_playing {
        // camera.translation.x += 1.0;
    }
     */
}
