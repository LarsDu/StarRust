use bevy::{
    prelude::{Component, Vec2},
    time::*,
};

use super::spawner::levels::*;

use super::ship::{yard::*,*};
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct Ship {
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
    pub rect: Vec2,
    pub damage: u8,
    pub hitmask: u8, // Not implemented yet
}

#[derive(Component)]
pub struct FuseTime {
    /// track when the bomb should explode (non-repeating timer)
    pub timer: Timer,
}


#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Spawner{
    pub spawn_infos: Vec<SpawnInfo<AiShipBundle>>
}
