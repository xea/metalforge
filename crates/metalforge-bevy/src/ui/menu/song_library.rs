//use crate::ui::menu::OnSongLibrary;
use crate::ui::{Library, RunState};
use bevy::color::palettes::css::{NAVY, RED};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::prelude::{default, Commands, Component, Res, ResMut};
use bevy_ui::widget::Text;
use bevy_ui::{BackgroundColor, FlexDirection, Node, Outline, Val};
use crate::ui::menu::{setup_menu, MenuEvent, MenuItem, MenuState};
use crate::ui::menu::main_menu::OnMainMenu;

#[derive(Component)]
pub struct OnSongLibrary;

pub fn setup_song_library(
    commands: Commands,
    library: Res<Library>,
    state: ResMut<MenuState>
) {
    let menu_items = vec![
        MenuItem::from((0, "BackToMain", MenuEvent::OpenMainMenu)),
        MenuItem::from((1, "Quit", MenuEvent::Quit)),
    ];

    setup_menu(menu_items, OnSongLibrary, commands, state);
    /*
    commands.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }, BackgroundColor(Color::from(NAVY)), OnSongSelectScreen)
    ).with_children(|parent| {
        for song in &library.songs {
            parent.spawn((
                Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(15.),
                    ..default()
                },
                Outline {
                    width: Val::Px(1.),
                    // offset: Val::Px(6.),
                    color: Color::WHITE,
                    ..default()
                },
                BackgroundColor(Color::from(RED)),
            )).with_children(|parent| {
                println!("[{}]", song.song_info.title.as_str());
                parent.spawn((
                    Text::new(song.song_info.title.as_str())
                ));
            });
        }
    });
     */
}
