use super::actor::bullet::BulletType;
use bevy::prelude::*;

#[derive(Message)]
pub struct LevelEndEvent;

#[derive(Default, Message)]
pub struct PlayerDeathEvent;

#[derive(Message)]
pub struct ScoreEvent {
    pub increment: i32,
}

#[derive(Message)]
pub struct AudioEvent {
    pub clip: Handle<AudioSource>,
}

#[derive(Message)]
pub struct WeaponFiredEvent {
    pub bullet_type: BulletType,
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}

#[derive(Message)]
pub struct ExplosionEvent {
    pub position: Vec3,
    pub lifetime: f32,
}

#[derive(Message)]
pub struct CameraShakeEvent {
    pub magnitude: f32,
    pub duration_secs: f32,
}
