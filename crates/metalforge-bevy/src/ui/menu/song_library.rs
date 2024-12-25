use crate::ui::menu::{setup_menu, MenuEvent, MenuItem, MenuState};
//use crate::ui::menu::OnSongLibrary;
use crate::ui::LibraryView;
use bevy::prelude::{Commands, Component, Res, ResMut};

#[derive(Component)]
pub struct OnSongLibrary;

pub fn setup_song_library(
    commands: Commands,
    library: Res<LibraryView>,
    state: ResMut<MenuState>
) {
    let mut menu_items = vec![];

    for (idx, song) in library.iter().enumerate() {
        let title = format!("{} by {}", song.header.title, song.header.artist);
        //let title = format!("{} by {} [{}]", song.song_info.title, song.song_info.artist, song.path.to_str().unwrap_or("[Unknown]"));
        let menu_item = MenuItem::from((idx, title, MenuEvent::ChooseSong(idx)));
        menu_items.push(menu_item);
    }

    if menu_items.is_empty() {
        menu_items.push(MenuItem::from((0, "<Empty library>", MenuEvent::Ignore)));
    }

    setup_menu("Song library", menu_items, OnSongLibrary, commands, state);
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
