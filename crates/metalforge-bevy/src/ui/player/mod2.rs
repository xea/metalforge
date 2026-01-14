use crate::ui::{AppState, UIEngine};
use bevy::app::{App, FixedUpdate, Startup, Update};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::camera::Camera2d;
use bevy::color::Luminance;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{default, in_state, AppExtStates, Bundle, Circle, Color, ColorMaterial, Commands, Component, Fixed, IntoScheduleConfigs, KeyCode, MeshMaterial2d, Message, MessageReader, MessageWriter, NextState, OnEnter, Query, Rectangle, Res, ResMut, Resource, State, States, Text, Transform, With};
use bevy::sprite::{Sprite, Text2d};
use bevy::text::{Font, TextColor, TextFont};
use bevy::time::{Time, Virtual};
use metalforge_lib::song::guitar::{GuitarNote, GuitarTuning};
use metalforge_lib::song::instrument_part::InstrumentPartType;
use metalforge_lib::song::song::Song;
use std::time::{Duration, Instant};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use metalforge_lib::engine::EngineCommand;

/// The base unit used to calculate distances visually. 1 Unit represents 1 millisecond of time passed.
/// This setting determines the length of rendered notes and scroll speed as well.
const BASE_MILLI_LENGTH_UNIT: f32 = 0.2;
const STRING_SPACING: f32 = 40.0;

#[derive(Message, Copy, Clone)]
pub enum PlayerEvent {
    StartPlaying,
    PausePlaying,
    ResumePlaying,
    JumpForwards(Duration),
    JumpBackwards(Duration),
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Playing,
    Paused,
}

#[derive(Resource)]
pub struct Player {
    // We may want to use Bevy's Virtual clock as it can be paused/unpaused/sped up and slowed down.
    last_start: Instant,
    song_position: Duration
}

#[derive(Component)]
struct CameraPosition {
    position: Vec3
}

#[derive(Component)]
struct OldCameraPosition {
    position: Vec3
}

impl Player {
    pub fn rewind(&mut self) {
        self.song_position = Duration::ZERO;
        self.last_start = Instant::now();
    }

    pub fn resume(&mut self) {
        self.last_start = Instant::now();
    }

    pub fn jump_forwards(&mut self, diff: &Duration) {
        self.song_position += *diff;
    }

    pub fn jump_backwards(&mut self, diff: &Duration) {
        self.song_position -= Duration::min(self.song_position, *diff);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            last_start: Instant::now(),
            song_position: Duration::ZERO
        }
    }
}

pub fn player_plugin(app: &mut App) {
    app
        .add_systems(Startup, setup2d)
        .add_systems(Update, (scroll_camera));
}

fn scroll_camera(mut query: Query<&mut Transform, With<Camera2d>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += time.delta().as_secs_f32() * 400.0;
        // transform.rotation.z += time.elapsed().as_secs_f32() * -1.0;
    }
}

fn setup2d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let mesh = meshes.add(Rectangle::new(9.0, 9.0));

    for x in -0..960 {
        for y in -5..5 {
            let color = Color::srgb_u8((x + 100) as u8, (y + 100) as u8, 255);
            let material = materials.add(color);
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material),
                Transform::from_xyz(x as f32 * 10.0, y as f32 * 10.0, 0.0)
            ));
        }
    }

    commands.spawn(Camera2d::default());
}


pub fn player_plugin2(app: &mut App) {
    app
        .insert_state(PlayerState::Playing)
        .insert_resource(Player::default())
        // .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_millis(100)))
        // .insert_resource(Time::<Fixed>::from_hz(10.0))
        .add_message::<PlayerEvent>()
        .add_systems(OnEnter(AppState::Player), setup_player)
        .add_systems(OnEnter(AppState::Player), setup_limiter)
        .add_systems(Update, handle_keyboard)
        .add_systems(Update, handle_events)
        .add_systems(FixedUpdate, update_player.run_if(in_state(AppState::Player)))
        .add_systems(FixedUpdate, (move_cursor, move_camera).run_if(in_state(AppState::Player)));
        // .add_systems(FixedUpdate, move_camera.run_if(in_state(AppState::Player)));
        // .add_systems(FixedUpdate, update_camera.run_if(in_state(AppState::Player)));
}

/// Initialise the tab player screen
fn setup_player(mut commands: Commands, engine: ResMut<UIEngine>, mut message_writer: MessageWriter<PlayerEvent>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, _asset_server: Res<AssetServer>) {
    // Load song
    let song = Song::test_song();
    let instrument = song.instrument_parts.first().expect("An instrument part was expected");
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

    // Start playing
    message_writer.write(PlayerEvent::StartPlaying);
    engine.engine.send(EngineCommand::PlaySong);
}

fn setup_limiter(mut framepace_settings: ResMut<FramepaceSettings>) {
    framepace_settings.limiter = Limiter::from_framerate(60.0)
}

#[derive(Component)]
pub struct Cursor;

#[derive(Bundle)]
pub struct CursorBundle {
    sprite: Sprite,
    transform: Transform,
    cursor: Cursor
}

impl CursorBundle {
    pub fn new() -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 280.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            cursor: Cursor
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
    commands.spawn(CursorBundle::new());
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

    // Render text on top of the notes
    commands.spawn((
        Text2d::new(format!("{}", note.fret)),
        Transform::from_xyz(position, note_offset, 0.3)
    ));
}

fn update_player(mut player: ResMut<Player>, player_state: Res<State<PlayerState>>, time: Res<Time>) {
    if player_state.get() == &PlayerState::Playing {
        let delta = Duration::from_millis(35); //time.delta();
        player.song_position += delta;
    }
}

fn move_camera(mut query: Query<&mut Transform, With<Camera2d>>, player: Res<Player>, time: Res<Time>) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    camera.translation.x += time.delta().as_secs_f32() * 200.0;
    //camera.translation.x = player.song_position.as_millis() as f32 * BASE_MILLI_LENGTH_UNIT;
}

fn move_cursor(mut query: Query<&mut Transform, With<Cursor>>, player: Res<Player>) {
    query.single_mut().expect("").translation.x = player.song_position.as_millis() as f32 * BASE_MILLI_LENGTH_UNIT;
}

fn update_camera(time: Res<Time>, mut camera_pos: Query<(&mut CameraPosition, &mut OldCameraPosition)>) {
    for (mut position, mut old_position) in &mut camera_pos {
        let position = &mut *position;

        old_position.position = position.position;
        position.position += time.delta_secs() * BASE_MILLI_LENGTH_UNIT;
    }
}

fn handle_keyboard(input: Res<ButtonInput<KeyCode>>, player_state: Res<State<PlayerState>>, mut player_events: MessageWriter<PlayerEvent>) {
}

fn handle_events(mut events: MessageReader<PlayerEvent>, mut player: ResMut<Player>, mut player_state: ResMut<NextState<PlayerState>>, mut time: ResMut<Time<Virtual>>) {
    for event in events.read() {
        match *event {
            PlayerEvent::StartPlaying => {
                rewind_player(&mut player, &mut time);
                resume_play(&mut player, &mut player_state);
            }
            PlayerEvent::ResumePlaying => {
                resume_play(&mut player, &mut player_state);
            }
            PlayerEvent::PausePlaying => {
                player_state.set(PlayerState::Paused);
                println!("Playing paused");
            },
            PlayerEvent::JumpForwards(diff) => {
                jump_forwards(&mut player, &diff);
            },
            PlayerEvent::JumpBackwards(diff) => {
                jump_backwards(&mut player, &diff);
            }
        }
    }
}

fn rewind_player(player: &mut ResMut<Player>, _time: &mut ResMut<Time<Virtual>>) {
    player.rewind();
}

fn resume_play(player: &mut ResMut<Player>, player_state: &mut ResMut<NextState<PlayerState>>) {
    player.resume();
    player_state.set(PlayerState::Playing);
    println!("Resume playing");
}

fn jump_forwards(player: &mut ResMut<Player>, diff: &Duration) {
    player.jump_forwards(diff);
}

fn jump_backwards(player: &mut ResMut<Player>, diff: &Duration) {
    player.jump_backwards(diff);
}