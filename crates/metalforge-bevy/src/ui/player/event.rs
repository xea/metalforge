use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Message, MessageReader, MessageWriter, NextState, Res, ResMut, State};
use bevy::time::{Time, Virtual};
use bevy::window::WindowCloseRequested;
use metalforge_lib::engine::EngineCommand;
use crate::ui::player::CameraPosition;
use crate::ui::player::song_player::{PlayerState, SongPlayer};
use crate::ui::UIEngine;

const SPEED_STEP: f32 = 0.1;
const MIN_SPEED: f32 = 0.1;
const MAX_SPEED: f32 = 2.0;

const ZOOM_STEP: f32 = 0.05;
const MIN_ZOOM: f32 = 0.2;
const MAX_ZOOM: f32 = 5.0;

#[derive(Message, Copy, Clone)]
pub enum PlayerEvent {
    /// Rewind and play the loaded song from the start, including any lead-in silence
    StartPlaying,
    /// Pause playing at the current position, playing will continue from this position when it's resumed
    PausePlaying,
    /// Resume playing from the last paused position
    ResumePlaying,
    /// Jump ahead in the song by the specified duration
    JumpForwards(Duration),
    /// Jump back in the song by the specified duration
    JumpBackwards(Duration),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    IncreaseSpeed,
    DecreaseSpeed,
    ResetSpeed
}

pub fn handle_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    player_state: Res<State<PlayerState>>,
    mut player_events: MessageWriter<PlayerEvent>
) {
    const SCROLL_DISTANCE_MILLIS: u64 = 250;
    const JUMP_DISTANCE_MILLIS: u64 = 1000;

    // Handle start/stop/pause/restart events
    if input.just_pressed(KeyCode::Space) {
        // Pressing space will start/resume playing from the current position
        if player_state.get() == &PlayerState::Playing {
            player_events.write(PlayerEvent::PausePlaying);

        } else if player_state.get() == &PlayerState::Paused {
            player_events.write(PlayerEvent::ResumePlaying);

        }
    } else if input.pressed(KeyCode::KeyR) {
        // Pressing R resets the player
        player_events.write(PlayerEvent::StartPlaying);

    } else if input.pressed(KeyCode::ArrowLeft) {
        // Pressing left jumps back in time
        if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            player_events.write(PlayerEvent::JumpBackwards(Duration::from_millis(JUMP_DISTANCE_MILLIS)));
        } else {
            player_events.write(PlayerEvent::JumpBackwards(Duration::from_millis(SCROLL_DISTANCE_MILLIS)));
        }
    } else if input.pressed(KeyCode::ArrowRight) {
        // Pressing right jumps forward in time
        if input.pressed(KeyCode::ShiftRight) || input.pressed(KeyCode::ShiftLeft) {
            player_events.write(PlayerEvent::JumpForwards(Duration::from_millis(JUMP_DISTANCE_MILLIS)));
        } else {
            player_events.write(PlayerEvent::JumpForwards(Duration::from_millis(SCROLL_DISTANCE_MILLIS)));
        }
    }

    // Handle speed events
    if input.just_pressed(KeyCode::ArrowUp) {
        player_events.write(PlayerEvent::IncreaseSpeed);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        player_events.write(PlayerEvent::DecreaseSpeed);
    } else if input.just_pressed(KeyCode::Slash) {
        player_events.write(PlayerEvent::ResetSpeed);
    }

    // Handle zoom events
    if input.pressed(KeyCode::Equal) {
        player_events.write(PlayerEvent::ZoomIn);
    } else if input.pressed(KeyCode::Minus) {
        player_events.write(PlayerEvent::ZoomOut);
    } else if input.pressed(KeyCode::Digit0) {
        player_events.write(PlayerEvent::ResetZoom);
    }
}

pub fn handle_events(
    mut events: MessageReader<PlayerEvent>,
    mut engine: ResMut<UIEngine>,
    mut player: ResMut<SongPlayer>,
    mut camera: ResMut<CameraPosition>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut time: ResMut<Time<Virtual>>
) {
    for event in events.read() {
        match *event {
            PlayerEvent::StartPlaying => {
                rewind_player(&mut player, &mut time);
                resume_play(&mut engine, &mut player, &mut player_state);
            }
            PlayerEvent::ResumePlaying => {
                resume_play(&mut engine, &mut player, &mut player_state);
            }
            PlayerEvent::PausePlaying => {
                pause_play(&mut engine, &mut player, &mut player_state);
            },
            PlayerEvent::JumpForwards(diff) => {
                jump_forwards(&mut engine, &mut player, &diff);
            },
            PlayerEvent::JumpBackwards(diff) => {
                jump_backwards(&mut engine, &mut player, &diff);
            }
            PlayerEvent::ZoomIn => {
                camera.zoom = (camera.zoom - ZOOM_STEP).max(MIN_ZOOM);
            }
            PlayerEvent::ZoomOut => {
                camera.zoom = (camera.zoom + ZOOM_STEP).min(MAX_ZOOM);
            }
            PlayerEvent::ResetZoom => {
                camera.zoom = 1.0;
            }
            PlayerEvent::IncreaseSpeed => {
                player.player_speed = (player.player_speed + SPEED_STEP).min(MAX_SPEED);
            }
            PlayerEvent::DecreaseSpeed => {
                player.player_speed = (player.player_speed - SPEED_STEP).max(MIN_SPEED);
            }
            PlayerEvent::ResetSpeed => {
                player.player_speed = 1.0;
            }
        }
    }
}

pub fn handle_window_events(mut window_close_events: MessageReader<WindowCloseRequested>, engine: ResMut<UIEngine>) {
    for _event in window_close_events.read() {
        engine.channel.send(EngineCommand::Quit);
    }
}

fn rewind_player(player: &mut ResMut<SongPlayer>, _time: &mut ResMut<Time<Virtual>>) {
    player.rewind();
}

fn pause_play(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    engine.channel.send(EngineCommand::Pause);
    player.pause();
    player_state.set(PlayerState::Paused);
}

fn resume_play(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    engine.channel.send(EngineCommand::Resume);
    player.resume();
    player_state.set(PlayerState::Playing);
}

fn jump_forwards(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, diff: &Duration) {
    player.jump_forwards(diff);
    engine.channel.send(EngineCommand::Seek(player.song_position))
}

fn jump_backwards(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, diff: &Duration) {
    player.jump_backwards(diff);
    engine.channel.send(EngineCommand::Seek(player.song_position))
}
