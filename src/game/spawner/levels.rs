use bevy::{prelude::*, time::*, utils::Duration};

use super::*;
use super::constants::*;
use super::super::ai::*;
use super::super::actor::{ship::*, *};
pub struct SpawnSequence;


impl SpawnSequence {
    pub fn level0(asset_server: &Res<AssetServer>) -> Vec<SpawnInfo<AiActorBundle>>{
        return vec![
            SpawnInfo::<AiActorBundle>{
                locations: vec![SPAWN_LOCATIONS[0]],
                ttl_timer: Timer::new( Duration::from_secs(4), false),
                frequency_timer: Timer::new( Duration::from_secs(1), true),
                bundle: DefaultEnemyShip::get_bundle(asset_server),
   
            },
            SpawnInfo::<AiActorBundle>{
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl_timer: Timer::new( Duration::from_secs(4), false),
                frequency_timer: Timer::new(Duration::from_secs(1), true),
                bundle: DefaultEnemyShip::get_bundle(asset_server),
            },
        ];
    }
}