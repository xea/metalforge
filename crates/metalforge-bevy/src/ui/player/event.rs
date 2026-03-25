use std::ops::Add;
use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Message, MessageReader, MessageWriter, NextState, Res, ResMut, State};
use bevy::time::{Time, Virtual};
use bevy::window::WindowCloseRequested;
use metalforge_lib::engine::{EngineCommand};
use crate::ui::player::CameraPosition;
use crate::ui::player::song_player::{PlayerState, SongPlayer};
use crate::ui::UIEngine;

const SPEED_STEP: f32 = 0.1;
const MIN_SPEED: f32 = 0.1;
const MAX_SPEED: f32 = 2.0;

const ZOOM_STEP: f32 = 0.05;
const MIN_ZOOM: f32 = 0.2;
const MAX_ZOOM: f32 = 5.0;

const FINE_SCROLL_DISTANCE_MILLIS: u64 = 10;
const SCROLL_DISTANCE_MILLIS: u64 = 250;
const JUMP_DISTANCE_MILLIS: u64 = 1000;

#[derive(Copy, Clone)]
pub enum SeekLocation {
    Start,
    Location(Duration),
    RelativeForward(Duration),
    RelativeBackward(Duration),
    PreviousBeat,
    NextBeat
}

#[derive(Message, Copy, Clone)]
pub enum PlayerEvent {
    /// Rewind and play the loaded song from the start, including any lead-in silence
    StartPlaying,
    /// Pause playing at the current position, playing will continue from this position when it's resumed
    PausePlaying,
    /// Resume playing from the last paused position
    ResumePlaying,
    /// Jump ahead in the song by the specified duration
    Seek(SeekLocation),
    /// Increase zoom in player view
    ZoomIn,
    /// Decrease zoom in player view
    ZoomOut,
    /// Reset zoom to original value
    ResetZoom,
    /// Increase playback speed by one increment
    IncreaseSpeed,
    /// Decrease playback speed by one increment
    DecreaseSpeed,
    /// Reset playback speed to its original value
    ResetSpeed,
    /// Create a new marker indicating where playback should start after the next restart
    MarkLoopStart,
    /// Create a new marker indicating where playback should end next
    MarkLoopEnd,
}

pub fn handle_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    player_state: Res<State<PlayerState>>,
    mut player_events: MessageWriter<PlayerEvent>
) {

    // Handle start/stop/pause/restart events
    if input.just_pressed(KeyCode::Space) {
        // Pressing space will start/resume playing from the current position
        if player_state.get() == &PlayerState::Playing {
            player_events.write(PlayerEvent::PausePlaying);

        } else if player_state.get() == &PlayerState::Paused {
            player_events.write(PlayerEvent::ResumePlaying);

        }
    } else if input.just_pressed(KeyCode::KeyR) {
        // Pressing R resets the player
        player_events.write(PlayerEvent::StartPlaying);
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        if input.any_pressed([ KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight ]) {
            // Snap to nearest beat behind
            player_events.write(PlayerEvent::Seek(SeekLocation::PreviousBeat));
        }
    } else if input.pressed(KeyCode::ArrowLeft) {
        // Pressing left jumps back in time
        if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
            // Fast scroll backward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeBackward(Duration::from_millis(JUMP_DISTANCE_MILLIS))));
        } else if input.pressed(KeyCode::AltLeft) || input.pressed(KeyCode::AltRight) {
            // Fine scroll backward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeBackward(Duration::from_millis(FINE_SCROLL_DISTANCE_MILLIS))));
        } else if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]) {
            // Ignore, this is handled separately

        } else {
            // Normal scroll backward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeBackward(Duration::from_millis(SCROLL_DISTANCE_MILLIS))));
        }
    } else if input.just_pressed(KeyCode::ArrowRight) {
        // Snap to nearest beat ahead
        player_events.write(PlayerEvent::Seek(SeekLocation::NextBeat));
    } else if input.pressed(KeyCode::ArrowRight) {
        // Pressing right jumps forward in time
        if input.pressed(KeyCode::ShiftRight) || input.pressed(KeyCode::ShiftLeft) {
            // Fast scroll forward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeForward(Duration::from_millis(JUMP_DISTANCE_MILLIS))));

        } else if input.pressed(KeyCode::AltLeft) || input.pressed(KeyCode::AltRight) {
            // Fine scroll forward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeForward(Duration::from_millis(FINE_SCROLL_DISTANCE_MILLIS))));

        } else if input.any_pressed([ KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight ]) {
            // Ignore, this is handled under 'just_pressed'

        } else {
            // Normal scroll forward
            player_events.write(PlayerEvent::Seek(SeekLocation::RelativeForward(Duration::from_millis(SCROLL_DISTANCE_MILLIS))));

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

    // Handle marker events
    if input.just_pressed(KeyCode::BracketLeft) {
        player_events.write(PlayerEvent::MarkLoopStart);
    } else if input.just_pressed(KeyCode::BracketRight) {
        player_events.write(PlayerEvent::MarkLoopEnd);
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
                seek(&mut engine, &mut player, SeekLocation::Start);
            }
            PlayerEvent::ResumePlaying => {
                resume_play(&mut engine, &mut player, &mut player_state);
            }
            PlayerEvent::PausePlaying => {
                pause_play(&mut engine, &mut player, &mut player_state);
            },
            PlayerEvent::Seek(location) => {
                seek(&mut engine, &mut player, location);
            },
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
                increase_speed(&mut engine, &mut player);
            }
            PlayerEvent::DecreaseSpeed => {
                decrease_speed(&mut engine, &mut player);
            }
            PlayerEvent::ResetSpeed => {
                player.player_speed = 1.0;
                reset_speed(&mut engine, &mut player);
            }
            PlayerEvent::MarkLoopStart => {
                player.start_position = player.song_position;
            }
            PlayerEvent::MarkLoopEnd => {
                player.loop_position = player.song_position;
            }
        }
    }
}

pub fn handle_window_events(mut window_close_events: MessageReader<WindowCloseRequested>, engine: ResMut<UIEngine>) {
    for _event in window_close_events.read() {
        engine.send(EngineCommand::Quit);
    }
}

pub fn handle_engine_events(mut engine: ResMut<UIEngine>) {
    while let Some(event) = engine.channel.try_receive() {
        match event {
        }
    }
}

fn pause_play(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    engine.send(EngineCommand::Pause);
    player.pause();
    player_state.set(PlayerState::Paused);
}

fn resume_play(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    engine.send(EngineCommand::Resume);
    player.resume();
    player_state.set(PlayerState::Playing);
}

fn seek(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, location: SeekLocation) {
    let new_location = match location {
        SeekLocation::Start => Duration::ZERO,
        SeekLocation::Location(location) => location,
        SeekLocation::RelativeBackward(diff) => player.song_position.checked_sub(diff).unwrap_or(Duration::ZERO),
        SeekLocation::RelativeForward(diff) => player.song_position.add(diff).min(player.song_duration),
        SeekLocation::NextBeat => player.current_song.beats.iter()
            .find(|beat| beat.time > player.song_position)
            .map(|b| b.time)
            .unwrap_or(player.song_position),
        SeekLocation::PreviousBeat => {
            let closest_ahead = player.current_song.beats.iter()
                .enumerate()
                .find(|beat| beat.1.time >= player.song_position)
                .map(|beat| beat.0)
                .unwrap_or(1);

            player.current_song.beats.get(closest_ahead.max(1) - 1)
                .map(|beat| beat.time)
                .unwrap_or(Duration::ZERO)
        }
    };

    jump_to(engine, player, &new_location);
}

fn jump_to(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>, location: &Duration) {
    engine.send(EngineCommand::Seek(*location));
    player.seek(location);
}

fn increase_speed(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>) {
    player.player_speed = (player.player_speed + SPEED_STEP).min(MAX_SPEED);
    engine.send(EngineCommand::ChangeSpeed(player.player_speed));
}

fn decrease_speed(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>) {
    player.player_speed = (player.player_speed - SPEED_STEP).max(MIN_SPEED);
    engine.send(EngineCommand::ChangeSpeed(player.player_speed));
}

fn reset_speed(engine: &mut ResMut<UIEngine>, player: &mut ResMut<SongPlayer>) {
    player.player_speed = 1.0;
    engine.send(EngineCommand::ChangeSpeed(player.player_speed));
}
