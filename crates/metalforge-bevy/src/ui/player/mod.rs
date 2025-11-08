use crate::ui::AppState;
use bevy::app::App;
use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{default, AppExtStates, Bundle, Color, Commands, DespawnOnExit, Message, OnEnter, Res, States, Text};
use bevy::text::{Font, TextColor, TextFont};
use bevy::ui::{percent, AlignItems, BackgroundColor, JustifyContent, Node};

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

pub fn player_plugin(app: &mut App) {
    app
        .insert_state(PlayerState::Playing)
        .add_message::<PlayerEvent>()
        .add_systems(OnEnter(AppState::Player), setup_player);
}

/// Initialise the tab player screen
fn setup_player(mut commands: Commands,  asset_server: Res<AssetServer>) {
    // Prepare assets
    let font = asset_server.load("fonts/LelandText.otf");

    // Draw the individual notes

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