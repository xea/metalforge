use crate::ui::debug::event::DebugEvent;
use crate::ui::menu::event::MenuEvent;
use crate::ui::menu::{MenuId, MenuState, MenuStructure};
use crate::ui::player::event::{PlayerEvent, SeekLocation, FINE_SCROLL_DISTANCE_MILLIS, JUMP_DISTANCE_MILLIS, SCROLL_DISTANCE_MILLIS};
use crate::ui::player::song_player::PlayerState;
use crate::ui::AppState;
use bevy::input::ButtonInput;
use bevy::prelude::{in_state, App, Commands, IntoScheduleConfigs, KeyCode, MessageWriter, NextState, Res, ResMut, State, Update};
use std::time::Duration;
use log::{info, trace, warn};

pub fn handle_key_input(app: &mut App) {
    app
        .add_systems(Update, handle_debug_keys)
        .add_systems(Update, handle_player_keys.run_if(in_state(AppState::Player)))
        .add_systems(Update, handle_menu_keys.run_if(in_state(MenuState::ShowMenu)));
}

pub fn handle_debug_keys(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    _player_state: Res<State<PlayerState>>,
) {
    if input.just_pressed(KeyCode::F1) {
        commands.trigger(DebugEvent::ToggleDebugInfo)
    }
}

pub(crate) fn handle_menu_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_events: MessageWriter<MenuEvent>,
    menu: Res<MenuStructure>,
) {
    if input.just_pressed(KeyCode::ArrowDown) {
        menu_events.write(MenuEvent::NextItemSelected);

    } else if input.just_pressed(KeyCode::ArrowUp) {
        menu_events.write(MenuEvent::PrevItemSelected);

    } else if input.pressed(KeyCode::ArrowDown) {
        // Down key has been pressed for a while
        menu_events.write(MenuEvent::NextItemSelected);

    } else if input.pressed(KeyCode::ArrowUp) {
        // Up key has been pressed for a while
        menu_events.write(MenuEvent::PrevItemSelected);

    } else if input.just_pressed(KeyCode::Enter) {
        if let Some(item) = menu.current_item() {
            menu_events.write(item.action);
        }

    } else if input.just_pressed(KeyCode::Escape) {
        if let Some(menu) = menu.current_menu() {
            menu_events.write(menu.pop_action);
        } else {
            menu_events.write(MenuEvent::PopMenu);
        }
    }
}

fn handle_player_keys(
    input: Res<ButtonInput<KeyCode>>,
    player_state: Res<State<PlayerState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_events: MessageWriter<PlayerEvent>,
    mut menu_events: MessageWriter<MenuEvent>,
    mut menu_structure: ResMut<MenuStructure>
) {
    if player_state.get() == &PlayerState::Menu {
        if input.just_pressed(KeyCode::Escape) {
            // Release menu and resume playing
            trace!("Release player menu");
            menu_structure.pop_menu();
            next_player_state.set(PlayerState::Playing)
        }
    } else {

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
            if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]) {
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
            } else if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]) {
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

        if input.just_pressed(KeyCode::Escape) {
            trace!("Show player menu");
            menu_structure.push_menu(MenuId::PlayerMenu);
            next_menu_state.set(MenuState::ShowMenu);
            next_player_state.set(PlayerState::Menu);
            menu_events.write(MenuEvent::ShowMenu);
        }
    }

}