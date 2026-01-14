mod song_player;
mod event;
mod cursor;
mod info;

use crate::ui::player::cursor::CursorBundle;
use crate::ui::player::event::{handle_events, handle_keyboard, PlayerEvent};
use crate::ui::player::song_player::{PlayerState, SongPlayer};
use bevy::app::{App, FixedUpdate, Startup, Update};
use bevy::asset::Assets;
use bevy::camera::Camera2d;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{AppExtStates, Commands, MeshMaterial2d, Query, Rectangle, Res, ResMut, Resource, Transform, With};
use bevy::sprite_render::ColorMaterial;
use bevy::time::{Fixed, Time};
use rand::random;
use std::time::Duration;
use crate::ui::player::info::{setup_info, update_info};

/// The main song player plugin, this method is responsible for setting up the camera, rendering the
/// song view, etc.
pub fn player_plugin(app: &mut App) {
    app
        .insert_state(PlayerState::Paused)
        .insert_resource(SongPlayer::default())
        .insert_resource(CameraPosition::default())
        .add_systems(Startup, (setup_player, setup_info, create_camera))
        .add_systems(Update, (handle_keyboard, handle_events))
        .add_systems(FixedUpdate, (update_position, update_info))
        .add_systems(Update, update_camera)
        .add_message::<PlayerEvent>()
    ;
}

/// This structure is responsible for tracking the camera and translating the player state into camera
/// coordinates. `current`, `previous`, and `velocity` are used for frame interpolation/extrapolation
#[derive(Resource, Default)]
struct CameraPosition {
    current: Vec3,
    previous: Vec3,
    velocity: Vec3
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    create_grid(&mut commands, &mut meshes, &mut materials);
    create_cursor(&mut commands);
}

fn create_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(19.0, 19.0));
    let material = materials.add(ColorMaterial::from_color(Color::srgb_u8(255, 255, 255)));

    for x in 0..1000 {
        for y in -5..5 {
            let r = random::<u16>();
            let color = Color::srgb((x as f32 / 10.0).sin(), (y as f32 / 10.0).cos(), ((y) as f32 / 10.0).sin());
            let material = materials.add(ColorMaterial::from_color(color));

            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(x as f32 * 20.0, y as f32 * 20.0, 0.0)
            ));
        }
    }
}

fn create_cursor(commands: &mut Commands) {
    commands.spawn(CursorBundle::new());
}

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
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
fn update_camera(time: Res<Time<Fixed>>, camera_position: Res<CameraPosition>, mut q_camera: Query<&mut Transform, With<Camera2d>>) {
    let f = time.overstep_fraction();
    const SCROLL_SPEED: f32 = 100.0;

    for mut camera in &mut q_camera {
        camera.translation = SCROLL_SPEED * camera_position.previous.lerp(camera_position.current, f);
    }
}
