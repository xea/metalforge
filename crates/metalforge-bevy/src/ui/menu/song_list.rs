use crate::ui::menu::OnSongSelectScreen;
use crate::ui::{Library, RunState};
use bevy::color::palettes::css::{NAVY, RED};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::prelude::{default, Commands, Res};
use bevy_ui::widget::Text;
use bevy_ui::{BackgroundColor, FlexDirection, Node, Outline, Val};

pub fn setup_song_library(
    mut commands: Commands,
    library: Res<Library>,
) {
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
