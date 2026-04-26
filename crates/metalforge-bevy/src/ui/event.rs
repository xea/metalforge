use std::time::Duration;
use crate::ui::menu::{populate_song_browser, MenuId, MenuState, MenuStructure, SongLibrary};
use crate::ui::player::song_player::SongPlayer;
use crate::ui::{AppState, UIEngine};
use bevy::prelude::{NextState, Res, ResMut};
use log::info;
use metalforge_lib::engine::EngineEvent;

pub fn handle_engine_event(
    engine_channel: Res<UIEngine>,
    mut song_player: ResMut<SongPlayer>,
    mut song_library: ResMut<SongLibrary>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut menu: ResMut<MenuStructure>
) {
    while let Some(event) = engine_channel.channel.try_receive() {
        match event {
            EngineEvent::SongLoaded(song) => {
                song_player.reset(song);
                next_app_state.set(AppState::Player);
                next_menu_state.set(MenuState::HideMenu);
            }
            EngineEvent::LibraryUpdated(library) => {
                info!("Updating library");
                song_library.0 = library;

                if let Some(browser_menu) = menu.menus.get_mut(&MenuId::Browser) {
                    populate_song_browser(browser_menu, &song_library.0.songs);
                }

                next_menu_state.set(MenuState::ShowMenu);
            }
            EngineEvent::SongUnloaded => {
                next_menu_state.set(MenuState::ShowMenu);
                next_app_state.set(AppState::MainMenu);
                song_player.current_song = None;
                song_player.playing = false;
                menu.pop_menu();
            }
        }
    }
}

