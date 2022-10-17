use bevy::prelude::*;

pub struct ScoreEvent{
    pub increment: i32,
}

pub struct AudioEvent;

pub struct WeaponFiredEvent {
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}
