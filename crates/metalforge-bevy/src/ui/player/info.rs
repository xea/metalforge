use bevy::prelude::{default, Commands, Component, Query, Res, Text, With};
use bevy::ui::{percent, px, Node, PositionType};
use crate::ui::player::song_player::SongPlayer;

#[derive(Component)]
pub struct UILabel;

pub fn setup_info(mut commands: Commands) {
    commands.spawn((
        Text::new("0:00:00.000".to_string()),
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            right: px(10),
            ..default()
        },
        UILabel
    ));
}

pub fn update_info(player: Res<SongPlayer>, mut query: Query<&mut Text, With<UILabel>>) {
    let time = player.song_position.as_secs_f32();
    let time_label = format!("{:01}:{:02}:{:02}.{:03}",
                             (time / 3600.0) as u8,
                             (time % 3600.0 / 60.0) as u8,
                             (time % 60.0) as u8,
                             (time.fract() * 1000.0) as u16);

    for mut text in query.iter_mut() {
        text.0.clear();
        text.0.push_str(&time_label);
    }
}