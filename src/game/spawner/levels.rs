use super::*;
use super::constants::*;
use super::super::ship::{yard::*, *};
pub struct SpawnSequence;

pub struct SpawnInfo<B: Bundle>{
    pub location: Vec3,
    pub frequency: f32,
    pub duration: f32,
    pub bundle: B
}


impl SpawnSequence {
    pub fn level0(asset_server: &Res<AssetServer>, spawn_position: Vec2) -> Vec<SpawnInfo<AiShipBundle>>{
        return vec![
            SpawnInfo::<AiShipBundle>{
                location: SPAWN_LOCATIONS[0],
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip::spawn(asset_server, spawn_position)
            },
            SpawnInfo::<AiShipBundle>{
                location: SPAWN_LOCATIONS[1],
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip::spawn(asset_server, spawn_position)
            },
        ]
    }
}