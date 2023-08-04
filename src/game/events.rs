use super::actor::bullet::BulletType;
use bevy::prelude::*;

#[derive(Event)]
pub struct LevelEndEvent;

#[derive(Default, Event)]
pub struct PlayerDeathEvent;

#[derive(Event)]
pub struct ScoreEvent {
    pub increment: i32,
}

#[derive(Event)]
pub struct AudioEvent {
    //pub clip: Handle<AudioSource>,
    pub audio_file: String,
}

#[derive(Event)]
pub struct WeaponFiredEvent {
    pub bullet_type: BulletType,
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}

#[derive(Event)]
pub struct ExplosionEvent {
    pub position: Vec3,
    pub lifetime: f32,
}

#[derive(Event)]
pub struct CameraShakeEvent {
    pub magnitude: f32,
    pub duration_secs: f32,
}
