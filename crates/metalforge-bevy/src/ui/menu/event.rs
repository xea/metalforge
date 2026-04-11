use crate::ui::menu::{Menu, MenuId, MenuState, MenuStructure, SongLibrary};
use crate::ui::UIEngine;
use bevy::app::AppExit;
use bevy::prelude::{Message, MessageReader, MessageWriter, NextState, Res, ResMut};
use log::info;
use metalforge_lib::engine::EngineCommand;

#[derive(Message, Hash, Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum MenuEvent {
    PrevItemSelected,
    NextItemSelected,
    PushMenu(MenuId),
    PopMenu,
    PlaySong(usize),
    ExitSong,
    ExitApp,
    ShowMenu,
    HideMenu,
    Noop
}

pub(crate) fn handle_menu_events(
    mut events: MessageReader<MenuEvent>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu: ResMut<MenuStructure>,
    engine: Res<UIEngine>,
    mut next_state: ResMut<NextState<MenuState>>,
    library: Res<SongLibrary>
) {
    for event in events.read() {
        match event {
            MenuEvent::PrevItemSelected => {
                menu.select_prev();
            }
            MenuEvent::NextItemSelected => {
                menu.select_next();
            }
            MenuEvent::PushMenu(menu_id) => {
                menu.push_menu(*menu_id);
                next_state.set(MenuState::ShowMenu);
            }
            MenuEvent::PopMenu => {
                if menu.pop_menu() {
                    next_state.set(MenuState::ShowMenu);
                }
            }
            MenuEvent::PlaySong(song_idx) => {
                info!("Playing song (idx: {})", *song_idx);
                if let Some(song_file) = library.0.songs.get(*song_idx) {
                    engine.send(EngineCommand::LoadSong(song_file.clone()))
                }
            }
            MenuEvent::ShowMenu => {
                next_state.set(MenuState::ShowMenu);
            }
            MenuEvent::HideMenu => {
                next_state.set(MenuState::HideMenu);
            },
            MenuEvent::ExitApp => {
                info!("ExitApp menu event received, quitting engine");
                app_exit_writer.write(AppExit::Success);
                engine.send(EngineCommand::Quit);
            }
            MenuEvent::ExitSong => {
                info!("Exiting song");
                engine.send(EngineCommand::UnloadSong);
            }
            MenuEvent::Noop => {},
        }
    }
}
