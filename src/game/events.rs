use bevy::prelude::*;
use super::actor::bullet::BulletType;

pub struct ScoreEvent{
    pub increment: i32,
}

pub struct AudioEvent;

pub struct WeaponFiredEvent {
    pub bullet_type: BulletType,
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}
