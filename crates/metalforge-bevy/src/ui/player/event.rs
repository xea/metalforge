use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Message, MessageReader, MessageWriter, NextState, Res, ResMut, State};
use bevy::time::{Time, Virtual};
use crate::ui::player::song_player::{PlayerState, SongPlayer};

#[derive(Message, Copy, Clone)]
pub enum PlayerEvent {
    /// Rewind and play the loaded song from the start, including any lead-in silence
    StartPlaying,
    /// Pause playing at the current position, playing will continue from this position when it's resumed
    PausePlaying,
    /// Resume playing from the last paused position
    ResumePlaying,
    JumpForwards(Duration),
    JumpBackwards(Duration),
}

pub fn handle_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    player_state: Res<State<PlayerState>>,
    mut player_events: MessageWriter<PlayerEvent>
) {
    const SCROLL_DISTANCE: u64 = 50;
    const JUMP_DISTANCE: u64 = 500;

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
            player_events.write(PlayerEvent::JumpBackwards(Duration::from_millis(JUMP_DISTANCE)));
        } else {
            player_events.write(PlayerEvent::JumpBackwards(Duration::from_millis(SCROLL_DISTANCE)));
        }
    } else if input.pressed(KeyCode::ArrowRight) {
        // Pressing right jumps forward in time
        if input.pressed(KeyCode::ShiftRight) || input.pressed(KeyCode::ShiftLeft) {
            player_events.write(PlayerEvent::JumpForwards(Duration::from_millis(JUMP_DISTANCE)));
        } else {
            player_events.write(PlayerEvent::JumpForwards(Duration::from_millis(SCROLL_DISTANCE)));
        }
    }
}

pub fn handle_events(
    mut events: MessageReader<PlayerEvent>,
    mut player: ResMut<SongPlayer>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut time: ResMut<Time<Virtual>>
) {
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
                pause_play(&mut player, &mut player_state);
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

fn rewind_player(player: &mut ResMut<SongPlayer>, _time: &mut ResMut<Time<Virtual>>) {
    player.rewind();
}

fn pause_play(player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    player.pause();
    player_state.set(PlayerState::Paused);
}

fn resume_play(player: &mut ResMut<SongPlayer>, player_state: &mut ResMut<NextState<PlayerState>>) {
    player.resume();
    player_state.set(PlayerState::Playing);
}

fn jump_forwards(player: &mut ResMut<SongPlayer>, diff: &Duration) {
    player.jump_forwards(diff);
}

fn jump_backwards(player: &mut ResMut<SongPlayer>, diff: &Duration) {
    player.jump_backwards(diff);
}
