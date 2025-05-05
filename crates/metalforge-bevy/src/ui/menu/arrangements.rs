use crate::ui::menu::{setup_menu, MenuEvent, MenuItem, MenuState};
use crate::ui::{EngineView, LibraryView};
use bevy::prelude::{Commands, Component, Res, ResMut};

#[derive(Component)]
pub struct OnArrangements;

pub fn setup_arrangement(
    commands: Commands,
    menu_state: ResMut<MenuState>,
    engine: Res<EngineView>,
) {
    let mut menu_items = vec![];

    let song_idx = menu_state.selected_song_idx;
    let library = &engine.0.song_library;

    if let Some(song) = library.iter().nth(song_idx) {
        for (idx, arrangement) in song.header.arrangements.iter().enumerate() {
            let menu_item = MenuItem::from((
                idx,
                arrangement.name.as_str(),
                MenuEvent::ChooseArrangement(idx),
            ));
            menu_items.push(menu_item);
        }
    }

    setup_menu(
        "Arrangements",
        menu_items,
        OnArrangements,
        commands,
        menu_state,
    );
}
