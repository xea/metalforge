mod song_player;
mod event;
mod cursor;
mod info;

use crate::ui::player::cursor::{Cursor, CursorBundle};
use crate::ui::player::event::{handle_events, handle_keyboard, handle_window_events, PlayerEvent};
use crate::ui::player::info::{setup_info, update_info};
use crate::ui::player::song_player::{PlayerState, SongPlayer};
use bevy::app::{App, FixedUpdate, Startup, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::camera::{Camera2d, ClearColor, Projection};
use bevy::color::{Color, Luminance};
use bevy::math::{Vec2, Vec3};
use bevy::mesh::Mesh;
use bevy::prelude::{AppExtStates, Commands, Query, Res, ResMut, Resource, Sprite, Transform, With, Without};
use bevy::sprite::{BorderRect, SpriteImageMode, TextureSlicer};
use bevy::sprite_render::ColorMaterial;
use bevy::time::{Fixed, Time};
use bevy::utils::default;
use metalforge_lib::song::guitar::GuitarPart;
use metalforge_lib::song::instrument_part::InstrumentPartType;
use metalforge_lib::song::song::Song;
use std::time::Duration;

const SCROLL_SPEED: f32 = 100.0;
const PIXELS_PER_MILLIS: f32 = SCROLL_SPEED / 1000.0;
const STRING_SPACING: f32 = 45.0;

/// The main song player plugin, this method is responsible for setting up the camera, rendering the
/// song view, etc.
pub fn player_plugin(app: &mut App) {
    app
        .insert_state(PlayerState::Paused)
        .insert_resource(SongPlayer::default())
        .insert_resource(CameraPosition::default())
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.10)))
        .add_systems(Startup, (setup_player, setup_info, create_camera))
        .add_systems(Update, (handle_keyboard, handle_events))
        .add_systems(FixedUpdate, (update_position, update_info))
        .add_systems(FixedUpdate, handle_window_events)
        .add_systems(Update, update_camera)
        .add_message::<PlayerEvent>()
    ;
}

/// This structure is responsible for tracking the camera and translating the player state into camera
/// coordinates. `current`, `previous`, and `velocity` are used for frame interpolation/extrapolation
#[derive(Resource)]
struct CameraPosition {
    current: Vec3,
    previous: Vec3,
    velocity: Vec3,
    zoom: f32
}

impl Default for CameraPosition {
    fn default() -> Self {
        Self {
            current: Vec3::ZERO,
            previous: Vec3::ZERO,
            velocity: Vec3::ZERO,
            zoom: 1.0
        }
    }
}

fn setup_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    assert_server: Res<AssetServer>
) {
    let song = Song::test_song();
    let instrument = song.instrument_parts.first()
        .expect("Instrument part could not be found");

    let part = match &instrument.instrument_type {
        InstrumentPartType::LeadGuitar(part) => part,
        InstrumentPartType::RhythmGuitar(part) => part,
        InstrumentPartType::BassGuitar(part) => part,
    };

    let num_strings = part.tuning.string_offsets.len();
    let duration = song.metadata.length;
    let track_length_px = duration.as_millis() as f32 * PIXELS_PER_MILLIS;

    create_background(&mut commands, num_strings, track_length_px);
    create_strings(&mut commands, num_strings, track_length_px);
    create_guide_lines(&mut commands, &song, part);
    create_note_sprites(&mut commands, &assert_server, part);
    create_cursor(&mut commands);
}

fn create_note_sprites(commands: &mut Commands, asset_server: &Res<AssetServer>, part: &GuitarPart) {
    let handle = asset_server.load("images/8BitButton-64x36.png");

    let scale_mode = SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::all(8.0),
        ..default()
    });

    for note in &part.notes {
        let len = PIXELS_PER_MILLIS * note.length.as_millis() as f32;
        let size = Vec2::new(len, 36.0);

        let x = PIXELS_PER_MILLIS * note.time.as_millis() as f32;
        let y = string_y_offset(note.string as usize, part.tuning.string_offsets.len());

        commands.spawn((
            Sprite {
                image: handle.clone(),
                custom_size: Some(size),
                image_mode: scale_mode.clone(),
                ..default()
            },
            Transform::from_xyz(x + (len / 2.0), y, 0.0),
        ));

    }
}

fn create_guide_lines(commands: &mut Commands, song: &Song, part: &GuitarPart) {
    let notch_width_px = 1.0;
    let short_notch_height_px = 10.0;
    let tall_notch_height_px = short_notch_height_px * 2.0;

    let notch_count = song.metadata.length.as_millis() / 10;
    let total_height_px = (part.tuning.string_offsets.len() as f32 + 1.5) * STRING_SPACING;

    let y = -total_height_px / 2.0;
    let z = 0.1;

    for i in 0..notch_count {
        let x = i as f32 * 10.0;
        let notch_height_px = if i % 10 == 0 {
            tall_notch_height_px
        } else {
            short_notch_height_px
        };

        commands.spawn((
            Sprite::from_color(
                Color::srgba(1.0, 1.0, 1.0, 0.85),
                Vec2::new(notch_width_px, notch_height_px)
            ),
            Transform::from_xyz(x, y, z)
        ));

    }
}

fn create_background(commands: &mut Commands, num_strings: usize, track_length_px: f32) {
    let total_height_px = (num_strings as f32 + 1.5) * STRING_SPACING;
    let background_width_px = track_length_px + 2000.0;

    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.08, 0.09, 0.16, 0.85),
            Vec2::new(background_width_px, total_height_px)
        ),
        Transform::from_xyz(track_length_px / 2.0, 0.0, -3.0)
    ));
}

fn create_strings(commands: &mut Commands, num_strings: usize, track_length_px: f32) {
    for i in 0..num_strings {
        let y = string_y_offset(i, num_strings);
        let color = string_color(i).lighter(0.1);

        commands.spawn((
            Sprite::from_color(color, Vec2::new(track_length_px, 3.0)),
            Transform::from_xyz(track_length_px / 2.0, y, -1.0)
        ));
    }
}

fn string_y_offset(string_index: usize, num_strings: usize) -> f32 {
    let total_height = (num_strings - 1) as f32 * STRING_SPACING;
    string_index as f32 * STRING_SPACING - total_height / 2.0
}

fn string_color(index: usize) -> Color {
    match index % 6 {
        0 => Color::srgb(0.91, 0.28, 0.33),
        1 => Color::srgb(0.98, 0.86, 0.36),
        2 => Color::srgb(0.19, 0.52, 0.99),
        3 => Color::srgb(1.0, 0.50, 0.07),
        4 => Color::srgb(0.18, 0.77, 0.71),
        _ => Color::srgb(0.75, 0.35, 0.90)
    }
}

fn create_cursor(commands: &mut Commands) {
    commands.spawn(CursorBundle::new());
}

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
    ));
}

/// Updates the camera position at a fixed frame rate
fn update_position(time: Res<Time>, mut camera_position: ResMut<CameraPosition>, mut player: ResMut<SongPlayer>) {
    let position = &mut *camera_position;

    // Swap the previous position to the current, preparing for the next frame
    position.previous = position.current;

    // Calculate new position based on player speed (may be slowed down or sped up)
    if player.playing() {
        let offset = time.delta_secs() * player.player_speed;
        player.song_position += Duration::from_secs_f32(offset);
    }

    position.velocity.x = player.player_speed;
    position.current.x = player.song_position.as_secs_f32();
}

/// Calculates and adjusts the position for the camera for each frame, interpolating and extrapolating
/// per frame as needed.
fn update_camera(time: Res<Time<Fixed>>, camera_position: Res<CameraPosition>, mut q_camera: Query<(&mut Transform, &mut Projection), (With<Camera2d>, Without<Cursor>)>, mut q_cursor: Query<&mut Transform, (With<Cursor>, Without<Camera2d>)>) {
    let f = time.overstep_fraction();

    let mut camera = q_camera.single_mut().unwrap();
    let mut cursor = q_cursor.single_mut().unwrap();

    camera.0.translation = SCROLL_SPEED * camera_position.previous.lerp(camera_position.current, f);
    cursor.translation = camera.0.translation;

    if let Projection::Orthographic(ref mut ortho) = *camera.1 {
        ortho.scale = camera_position.zoom;
    }
}