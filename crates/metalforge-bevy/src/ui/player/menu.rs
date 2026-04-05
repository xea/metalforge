use crate::ui::menu::{MenuId, MenuStructure};
use bevy::prelude::ResMut;

pub(crate) fn setup_song_menu(
    mut menu_struct: ResMut<MenuStructure>
) {
    menu_struct.push_menu(MenuId::PlayerMenu);
}