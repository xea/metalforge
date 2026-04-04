use bevy::app::AppExit;
use crate::ui::menu::{MenuId, MenuState, MenuStructure};
use crate::ui::{AppState, UIEngine};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Message, MessageReader, MessageWriter, NextState, Res, ResMut};
use metalforge_lib::engine::EngineCommand;

#[derive(Message, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub(crate) enum MenuEvent {
    PrevItemSelected,
    NextItemSelected,
    PushMenu(MenuId),
    PopMenu,
    PlaySong,
    ExitApp,
    Noop
}

pub(crate) fn handle_keyboard_events(
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
        menu_events.write(MenuEvent::PopMenu);
    }
}

pub(crate) fn handle_menu_events(
    mut events: MessageReader<MenuEvent>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu: ResMut<MenuStructure>,
    engine: Res<UIEngine>,
    mut app_state: ResMut<NextState<AppState>>,
    mut next_state: ResMut<NextState<MenuState>>
) {
    for event in events.read() {
        match event {
            MenuEvent::PrevItemSelected => {
                menu.select_prev();
            }
            MenuEvent::NextItemSelected => {
                menu.select_next();
            }
            MenuEvent::ExitApp => {
                app_exit_writer.write(AppExit::Success);
                engine.send(EngineCommand::Quit);
            }
            MenuEvent::PushMenu(menu_id) => {
                menu.push_menu(*menu_id);
                next_state.set(MenuState::SwitchMenu);
            }
            MenuEvent::PopMenu => {
                if menu.pop_menu() {
                    next_state.set(MenuState::SwitchMenu);
                }
            }
            MenuEvent::PlaySong => {
                app_state.set(AppState::Player);
            }
            MenuEvent::Noop => {},

        }
    }
}
