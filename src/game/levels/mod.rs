use bevy::{prelude::*, time::FixedTimestep, utils::Duration};

use crate::utils::despawn_all;

use super::super::menus::MenuState;
use super::actor::{ship::*, *};
use super::components::*;
use super::constants::*;
use super::events::LevelEndEvent;
use super::{super::*, scene, AudioClipAssets, SceneAssets};
use fastrand;
pub mod lvl;
use lvl::*;

pub struct LevelSpawnInfo<B: Bundle> {
    pub locations: Vec<Vec2>,
    pub ttl: f32,
    pub frequency: f32,
    pub bundle: B,
}
pub trait BundledAsset {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle;
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(scene::setup_resources)
            .add_event::<LevelEndEvent>()    
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(despawn_all::<AiActorSpawner>))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(despawn_all::<AiActorSpawner>))
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(setup_level), //.with_system(spawn_startup_bundles::<Spawn>)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(level_periodic_spawn),
            )
            .add_system(level_ender);
    }
}

fn setup_level(
    mut commands: Commands,
    audio_clips: Res<AudioClipAssets>,
    models: Res<SceneAssets>,
) {
    commands.spawn(AiActorSpawner::new(SpawnSequence::level0(
        &audio_clips,
        &models,
    )));
}

fn level_periodic_spawn(
    mut commands: Commands,
    time: Res<Time>,
    //mut menu_state: ResMut<State<MenuState<>>>,
    mut level_end_event: EventWriter<LevelEndEvent>,
    mut query: Query<&mut AiActorSpawner, With<AiActorSpawner>>,
) {
    // Run logic for each Spawner Component
    for mut spawner in &mut query {
        let n_spawn_infos = spawner.spawn_infos.len() as i32;
        // Tick spawn timer
        spawner.frequency_timer.tick(time.delta());
        spawner.ttl_timer.tick(time.delta());

        // Fixme: Make this more functional
        if spawner.ttl_timer.just_finished() {
            spawner.index += 1;
            if spawner.index < n_spawn_infos {
                // Get the next spawn info and set the frequency and ttl durations
                let next_ttl = spawner.spawn_infos[spawner.index as usize].ttl;
                let next_frequency = spawner.spawn_infos[spawner.index as usize].frequency;
                spawner
                    .ttl_timer
                    .set_duration(Duration::from_secs_f32(next_ttl));
                spawner
                    .frequency_timer
                    .set_duration(Duration::from_secs_f32(next_frequency));
                    
                spawner.ttl_timer.reset();
            } else {
                //TODO: Level is over. Progress into success or failure states
                spawner.index = 0;
                level_end_event.send(LevelEndEvent {});
            }
        }

        if spawner.frequency_timer.finished() {
            let spawn_info = &spawner.spawn_infos[spawner.index as usize];
            //commands.spawn(DefaultEnemyShip::get_bundle(&asset_server));
            spawn_from_spawn_info(&mut commands, spawn_info);
        }
    }
}

fn spawn_from_spawn_info(commands: &mut Commands, spawn_info: &LevelSpawnInfo<AiActorBundle>) {
    // Read from spawn info
    let mut bundle = spawn_info.bundle.clone();
    let rng = fastrand::Rng::new();
    let spawn_pos = spawn_info.locations[rng.usize(0..spawn_info.locations.len())];
    bundle.actor_bundle.scene_bundle.transform.translation = spawn_pos.extend(0.0);
    commands.spawn(bundle); // <--The bundle is behind a mutable reference
}

fn level_ender(
    mut events: EventReader<LevelEndEvent>,
    mut game_state: ResMut<State<AppState>>,
    mut menu_state: ResMut<State<MenuState>>,
) {
    if !events.is_empty() {
        // Use overwrite_set for events, since events may register over multiple frames
        menu_state.overwrite_set(MenuState::LevelEnd).unwrap();
        game_state.overwrite_set(AppState::Menu).unwrap();
        events.clear();
    }
}
