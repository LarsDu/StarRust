use bevy::prelude::*;
use super::components::Ship;


pub const PLAYER_SHIP: Ship  = Ship{
    size:  Vec2::new(5.0,5.0),
    speed: Vec2::new(0.1, 0.1),
    gun_offset: Vec2::new(-1.0, -0.32)
};

pub const DEFAULT_ENEMY: Ship = Ship{
    size:  Vec2::new(5.0,5.0),
    speed: Vec2::new(0.2, 0.2),
    gun_offset: Vec2::new(1.0, 0.0)
};