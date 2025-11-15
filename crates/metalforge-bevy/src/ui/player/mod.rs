use crate::ui::AppState;
use bevy::app::{App, FixedUpdate, Update};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::camera::Camera2d;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{default, in_state, AppExtStates, Bundle, Circle, Color, ColorMaterial, Commands, IntoScheduleConfigs, MeshMaterial2d, Message, OnEnter, Query, Rectangle, Res, ResMut, Resource, States, SystemCondition, Text, Transform, With};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::{Font, TextColor, TextFont};
use bevy::time::Time;
use metalforge_lib::song::guitar::{GuitarNote, GuitarTuning};
use metalforge_lib::song::instrument_part::InstrumentPartType;
use metalforge_lib::song::song::Song;
use std::time::Duration;
use bevy::color::Luminance;

/// The base unit used to calculate distances visually. 1 Unit represents 1 millisecond of time passed.
/// This setting determines the length of rendered notes and scroll speed as well.
const BASE_MILLI_LENGTH_UNIT: f32 = 0.1;
const STRING_SPACING: f32 = 40.0;

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
    let song = Song::test_song();
    let instrument = song.instrument_parts.get(0).expect("An instrument part was expected");
    let part = match &instrument.instrument_type {
        InstrumentPartType::LeadGuitar(part) => part,
        InstrumentPartType::RhythmGuitar(part) => part,
        InstrumentPartType::BassGuitar(part) => part
    };

    let notes = &part.notes;

    // Prepare assets
    // - fonts can be loaded using asset_server.load("fonts/LelandText.otf");

    // Draw vertical cursor
    create_cursor(&mut commands);

    // Draw tab string lines
    create_string_lines(&mut commands, &part.tuning, &song.metadata.length);

    // Draw the individual notes

    for note in notes.iter() {
        create_note(&mut commands, &mut meshes, &mut materials, note, part.tuning.string_offsets.len());
    }

}

#[derive(Bundle)]
struct Cursor {
    sprite: Sprite,
    transform: Transform
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 280.0)),
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

fn string_offset(idx: usize, strings: usize) -> f32 {
    let height = (strings - 1) as f32 * STRING_SPACING;
    let offset = height / -2.0;
    idx as f32 * STRING_SPACING + offset
}

fn create_string_lines(commands: &mut Commands, tuning: &GuitarTuning, duration: &Duration) {
    let strings = tuning.string_offsets.len();
    let length = duration.as_millis() as f32 * BASE_MILLI_LENGTH_UNIT;

    let colors = [
        Color::srgb(1.0, 0.0, 0.0),
        Color::srgb(0.8, 0.8, 0.0),
        Color::srgb(0.2, 0.2, 1.0),
        Color::srgb(0.8, 0.4, 0.0),
        Color::srgb(0.2, 1.0, 0.2),
        Color::srgb(1.0, 0.0, 1.0),
    ];

    for string in tuning.string_offsets.iter().enumerate() {
        colors.get(string.0).map(|color| {
            let string_y = string_offset(string.0, strings);

            commands.spawn((
                Sprite::from_color(*color, Vec2::new(length, 2.0)),
                Transform::from_xyz(75.0, string_y, -1.0)
            ));
        });
    }
}

fn create_note(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, note: &GuitarNote, strings: usize) {
    let width = BASE_MILLI_LENGTH_UNIT * note.length.as_millis() as f32;
    let color = Color::srgb(0.7, 0.7, 0.7);
    let color_light = color.lighter(0.1);
    let position = BASE_MILLI_LENGTH_UNIT * note.time.as_millis() as f32;

    let material = materials.add(color);
    let material_light = materials.add(color_light);
    let mesh_note_tail = meshes.add(Rectangle::new(width, 30.0));
    let mesh_circle = meshes.add(Circle::new(20.0));

    let note_offset = string_offset(note.string as usize, strings);

    // Base circle goes to the bottom
    commands.spawn((
        Mesh2d(mesh_circle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(position, note_offset, 0.0)
    ));

    // Next is the tail of the note, same colour as the base circle
    commands.spawn((
        Mesh2d(mesh_note_tail),
        MeshMaterial2d(material),
        Transform::from_xyz(position + width / 2.0, note_offset, 0.0)
    ));

    // Then the lighter overlay circle covering parts of the base circle
    commands.spawn((
        Mesh2d(mesh_circle),
        MeshMaterial2d(material_light),
        Transform::from_xyz(position, note_offset, 0.1)
            .with_scale(Vec3::new(0.8, 0.8, 1.0)),
    ));

    // Render text on top of it all
    commands.spawn((
        Text2d::new("C#"),
        Transform::from_xyz(position, note_offset, 0.3)
    ));
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
