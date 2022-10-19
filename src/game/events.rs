use bevy::prelude::*;
use super::actor::bullet::BulletType;

pub struct ScoreEvent{
    pub increment: i32,
}

pub struct AudioEvent{
    pub clip: Handle<AudioSource>,
}


pub struct WeaponFiredEvent {
    pub bullet_type: BulletType,
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}

pub struct ExplosionEvent{
    pub position: Vec3,
    pub lifetime: f32
}


pub struct CameraShakeEvent{
    pub magnitude: f32,
    pub duration_secs: f32
}
