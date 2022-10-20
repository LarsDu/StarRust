use bevy::prelude::*;

use super::*;

pub struct SpawnSequence;


impl SpawnSequence {
    pub fn level0(audio_clips: &Res<AudioClipAssets>, models: &Res<game::SceneAssets>) -> Vec<LevelSpawnInfo<AiActorBundle>>{
        return vec![
            LevelSpawnInfo::<AiActorBundle>{
                locations: vec![SPAWN_LOCATIONS[0], SPAWN_LOCATIONS[1]],
                ttl: 15.0,
                frequency: 2.0,
                bundle: DefaultEnemyShip::get_bundle(audio_clips, models),
   
            },
            LevelSpawnInfo::<AiActorBundle>{
                locations: Vec::from(SPAWN_LOCATIONS),
                ttl: 30.0,
                frequency: 1.0,
                bundle: RaptorSineMovementVariant::get_bundle(audio_clips, models),
            },
        ];
    }
}