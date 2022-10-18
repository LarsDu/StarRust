use bevy::prelude::*;
use super::actor::bullet::BulletType;
use super::audio::clip::AudioClipEnum;

pub struct ScoreEvent{
    pub increment: i32,
}

pub struct AudioEvent{
    pub clip: AudioClipEnum,
}


pub struct WeaponFiredEvent {
    pub bullet_type: BulletType,
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}

pub struct ParticleEvent{

}


pub struct CameraShakeEvent{
    pub magnitude: f32,
    pub duration_secs: f32
}
