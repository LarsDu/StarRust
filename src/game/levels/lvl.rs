use bevy::prelude::*;

use super::LevelSpawnInfo;

use crate::game::actor::{ship::*, BundledActor};
use crate::game::constants::SPAWN_LOCATIONS;
use crate::game::{audio::AudioClipAssets, scene::SceneAssets};

pub struct SpawnSequence;

impl SpawnSequence {
    pub fn level0(
        audio_clips: &Res<AudioClipAssets>,
        models: &Res<SceneAssets>,
    ) -> Vec<LevelSpawnInfo> {
        // Note: To make this more flexible in type of item returned, would need to use heap
        return vec![
            LevelSpawnInfo {
                locations: vec![SPAWN_LOCATIONS[0], SPAWN_LOCATIONS[1], SPAWN_LOCATIONS[3]],
                ttl: 10.0,
                frequency: 3.0,
                spawn_func: DefaultEnemyShip::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 15.0,
                frequency: 1.0,
                spawn_func: JetCharger::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 5.0,
                frequency: 2.0,
                spawn_func: SpacePlatformBare::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 30.0,
                frequency: 0.8,
                spawn_func: RaptorSineMovementVariant::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 8.0,
                frequency: 2.0,
                spawn_func: SpacePlatformBare::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 30.0,
                frequency: 0.8,
                spawn_func: RaptorSineMovementVariant::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 15.0,
                frequency: 1.0,
                spawn_func: JetCharger::spawn_bundle,
            },
            LevelSpawnInfo {
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 40.0,
                frequency: 0.8,
                spawn_func: RaptorSineMovementVariant::spawn_bundle,
            },
        ];
    }

    pub fn level0_powerups(
        audio_clips: &Res<AudioClipAssets>,
        models: &Res<SceneAssets>,
    ) -> Vec<LevelSpawnInfo> {
        return vec![LevelSpawnInfo {
            locations: Vec::from(SPAWN_LOCATIONS),
            ttl: 300.0,
            frequency: 0.3,
            spawn_func: Star::spawn_bundle,
        }];
    }
}
