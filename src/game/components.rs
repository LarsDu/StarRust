use bevy::{
    prelude::*,
    time::*,
};

use super::spawner::{SpawnInfo, levels::*};
use super::ai::AiMode;

use super::actor::{ship::*,*};
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct Actor {
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
pub struct WeaponCooldown {
    /// Used for timed weapon shots
    pub timer: Timer,
}


#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct AiActorSpawner{
    pub index: i32,
    pub spawn_infos: Vec<SpawnInfo<AiActorBundle>>
}

#[derive(Component)]
pub struct Ai{
    pub mode: AiMode,
    pub timer: Timer
}

#[derive(Component)]
pub struct TimedDespawn{
    pub timer: Timer,
}


