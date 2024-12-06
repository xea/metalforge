use crate::ui::menu::{setup_menu, MenuEvent, MenuItem, MenuState};
use bevy::prelude::{Commands, Component, ResMut};

#[derive(Component)]
pub struct OnSettingsMenu;

pub fn setup_settings(
    commands: Commands,
    menu_state: ResMut<MenuState>
) {
    let menu_items = vec![
        MenuItem::from((0, "Display settings", MenuEvent::Todo)),
        MenuItem::from((1, "Debug settings", MenuEvent::Todo)),
    ];

    setup_menu(menu_items, OnSettingsMenu, commands, menu_state)
}

