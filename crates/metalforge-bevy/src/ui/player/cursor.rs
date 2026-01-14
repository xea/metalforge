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
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 280.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            cursor: Cursor
        }
    }
}

