use super::*;
use super::super::ship::{yard::*, *};



pub struct SpawnSequence;

pub struct SpawnInfo<B: Bundle>{
    pub frequency: f32,
    pub duration: f32,
    pub bundle: B
}


impl SpawnSequence {
    pub fn level0(asset_server: &Res<AssetServer>, spawn_position: Vec2) -> Vec<SpawnInfo<AiShipBundle>>{
        return vec![
            SpawnInfo::<AiShipBundle>{
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip::spawn(asset_server, spawn_position)
            },
            SpawnInfo::<AiShipBundle>{
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip::spawn(asset_server, spawn_position)
            },
        ]
    }
}