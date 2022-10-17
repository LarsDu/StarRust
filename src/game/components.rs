use bevy::{
    prelude::*,
    time::Timer,
};

use super::ENEMY_HITMASK;
use super::spawner::{SpawnInfo, levels::*};
use super::actor::bullet::*;
use super::ai::AiMode;

use super::actor::{ship::*,*};
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone, Default)]
pub struct Actor {
    pub speed: Vec2,
}

#[derive(Component, Clone, Default)]
pub struct Health {
    pub hp: i32,
}
#[derive(Component, Clone, Default)]
pub struct Bullet;

#[derive(Component, Clone)]
pub struct Collider {
    pub rect: Vec2,
    pub damage: i32,
    pub hitmask: u8,
}

impl Default for Collider{
    fn default() -> Self {
        return Collider { rect: Vec2::new(3.0, 3.0), damage: 0, hitmask: ENEMY_HITMASK }
    }
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub bullet_type: BulletType,
    pub offset: Vec2
}

#[derive(Component, Clone)]
pub struct AutoFire {
    /// Used for timed weapon shots
    pub cooldown_timer: Timer,
}


#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct AiActorSpawner{
    pub index: i32,
    pub spawn_infos: Vec<SpawnInfo<AiActorBundle>>,
    pub ttl_timer: Timer, // init from spawn_infos
    pub frequency_timer: Timer //init from spawn_infos
}

impl AiActorSpawner{
    pub fn new(spawn_infos: Vec<SpawnInfo<AiActorBundle>>) -> Self{
        return AiActorSpawner{
            index: 0,
            ttl_timer: Timer::from_seconds(spawn_infos[0].ttl, false),
            frequency_timer: Timer::from_seconds(spawn_infos[0].frequency, true),
            spawn_infos: spawn_infos
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Ai{
    pub mode: AiMode,
    pub timer: Timer
}

#[derive(Component, Clone, Default)]
pub struct TimedDespawn{
    pub timer: Timer,
}

// The following gets attached to the scor
#[derive(Component)]
pub struct PlayerScoreBoard;

#[derive(Component, Clone)]
pub struct DeathPointsAwarded{
    pub points: i32
}