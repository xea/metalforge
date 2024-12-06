use crate::ui::menu::{setup_menu, MenuEvent, MenuItem, MenuState};
use bevy::prelude::{Commands, Component, ResMut};

#[derive(Component)]
pub struct OnMainMenu;

pub fn setup_main_menu(commands: Commands, state: ResMut<MenuState>) {
    let menu_items = vec![
        MenuItem::from((0, "Play", MenuEvent::Play)),
        MenuItem::from((1, "Settings", MenuEvent::OpenSettingsMenu)),
        MenuItem::from((2, "Quit", MenuEvent::Quit))
    ];

    setup_menu(menu_items, OnMainMenu, commands, state);
}
