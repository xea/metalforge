use std::time::Duration;
use crate::ui::AppState;
use bevy::app::{App, FixedUpdate, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::camera::Camera2d;
use bevy::math::Vec2;
use bevy::prelude::{default, in_state, AppExtStates, Bundle, Color, Commands, DespawnOnExit, IntoScheduleConfigs, Message, OnEnter, Query, Res, ResMut, Resource, States, SystemCondition, Text, Transform, With};
use bevy::sprite::Sprite;
use bevy::text::{Font, TextColor, TextFont};
use bevy::time::Time;
use bevy::ui::{percent, AlignItems, BackgroundColor, JustifyContent, Node};
use bevy::ui::Val::Px;

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

fn update_player(mut player: ResMut<Player>, time: Res<Time>) {
    player.song_position = time.elapsed();
}

fn move_camera(mut query: Query<&mut Transform, With<Camera2d>>, player: Res<Player>) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    camera.translation.x = player.song_position.as_millis() as f32 / 10.0;

    /*
    if player_state.song_playing {
        // camera.translation.x += 1.0;
    }
     */
}

/// Initialise the tab player screen
fn setup_player(mut commands: Commands,  asset_server: Res<AssetServer>) {
    // Prepare assets
    // - fonts can be loaded using asset_server.load("fonts/LelandText.otf");

    // Draw the individual notes
    commands.spawn(Cursor::new());

    /*
    for i in 0..10 {
        commands.spawn((Node {
            width: Px(30.0),
            height: Px(30.0),
            ..default()
        }, Transform::from_xyz(i as f32 * 12.0, i as f32 * 15.0, 0.0), BackgroundColor(Color::srgb(0.8, 0.3, 0.3))));
    }
     */

    /*
    commands.spawn((
        DespawnOnExit(AppState::Player),
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        Node {
            // Make the root node fill out the entire screen
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(TitleText::new(font.clone()));
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
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 60.0)),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
        }
    }
}

#[derive(Bundle)]
struct TitleText {
    text: Text,
    font: TextFont,
    color: TextColor
}

impl TitleText {
    pub fn new(_font_handle: Handle<Font>) -> Self {
        Self {
            text: Text::new("MetaL Bundle 𝅘𝅥𝅰 \\m/"),
            font: TextFont {
                // font: font_handle, but it's not used because the text doesn't show when it is and I don't feel like debugging this right now.
                font_size: 30.0,
                ..default()
            },
            color: TextColor(Color::srgb(0.7, 0.7, 0.7)),
        }
    }
}