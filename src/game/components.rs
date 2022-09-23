use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Player {
    pub speed: Vec2,
    pub gun_offset: Vec2,
    pub hp: i8,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;
