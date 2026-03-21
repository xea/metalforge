use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Transform};
use bevy::sprite::Sprite;

#[derive(Component)]
pub struct Cursor;

#[derive(Bundle)]
pub struct CursorBundle {
    sprite: Sprite,
    transform: Transform,
    cursor: Cursor
}

impl CursorBundle {
    pub fn new() -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgba(0.4, 0.9, 1.0, 0.9), Vec2::new(2.5, 320.0)),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            cursor: Cursor
        }
    }
}

