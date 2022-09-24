use bevy::{
    prelude::{Component, Vec2},
    time::*,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct Ship {
    pub size: Vec2,
    pub speed: Vec2,
    pub gun_offset: Vec2,
}

#[derive(Component)]
pub struct Health {
    pub hp: u8,
}
#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Collider {
    pub damage: u8,
    pub hitmask: u8
}

#[derive(Component)]
pub struct FuseTime {
    /// track when the bomb should explode (non-repeating timer)
    pub timer: Timer,
}
