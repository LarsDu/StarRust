use bevy::{prelude::*, time::FixedTimestep, utils::Duration};

use rand::{thread_rng, Rng};

use super::{super::*, AudioClipAssets, SceneAssets, scene};
use super::actor::{ship::*, *};
use super::components::*;
use super::constants::*;

pub mod levels;
use levels::*;

pub struct SpawnInfo<B: Bundle> {
    pub locations: Vec<Vec2>,
    pub ttl: f32,
    pub frequency: f32,
    pub bundle: B,
}
pub trait BundledAsset {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle;
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(scene::setup_resources)
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup_level), //.with_system(spawn_startup_bundles::<Spawn>)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(periodic_spawn),
        );
    }
}

fn setup_level(mut commands: Commands, audio_clips: Res<AudioClipAssets>, models: Res<SceneAssets>) {
    commands.spawn(AiActorSpawner::new(SpawnSequence::level0(&audio_clips, &models)));
}

fn periodic_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut AiActorSpawner, With<AiActorSpawner>>,
) {
    // Run logic for each Spawner Component
    for mut spawner in &mut query {
        let n_spawn_infos = spawner.spawn_infos.len() as i32;
        // Tick spawn timer
        spawner.frequency_timer.tick(time.delta());
        spawner.ttl_timer.tick(time.delta());

        if spawner.ttl_timer.finished() {
            let new_index = i32::clamp(spawner.index + 1, 0, n_spawn_infos);
            if new_index < n_spawn_infos {
                spawner.index = new_index;
                // Get the next spawn info and set the frequency and ttl durations
                let next_ttl = spawner.spawn_infos[spawner.index as usize].ttl;
                let next_frequency = spawner.spawn_infos[spawner.index as usize].frequency;
                spawner
                    .ttl_timer
                    .set_duration(Duration::from_secs_f32(next_ttl));
                spawner
                    .frequency_timer
                    .set_duration(Duration::from_secs_f32(next_frequency));
            } else {
                //TODO: Level is over. Progress into success or failure states
                //spawner.index = 0;
            }
        }

        if spawner.frequency_timer.finished() {
            let spawn_info = &spawner.spawn_infos[spawner.index as usize];
            //commands.spawn(DefaultEnemyShip::get_bundle(&asset_server));
            spawn_from_spawn_info(&mut commands, spawn_info);
        }
    }
}

fn spawn_from_spawn_info(commands: &mut Commands, spawn_info: &SpawnInfo<AiActorBundle>) {
    // Read from spawn info
    let mut bundle = spawn_info.bundle.clone();
    let mut rng = thread_rng();
    let spawn_pos = spawn_info.locations[rng.gen_range(0..spawn_info.locations.len())];
    bundle.actor_bundle.scene_bundle.transform.translation = spawn_pos.extend(0.0);
    commands.spawn(bundle); // <--The bundle is behind a mutable reference
}
