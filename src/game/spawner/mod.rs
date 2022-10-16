use bevy::{
    prelude::*,
    time::{FixedTimestep, Timer},
};
use core::cmp::min;
use super::super::*;
use super::actor::{ship::*, *};
use super::components::*;
use super::constants::*;

pub mod levels;
use levels::*;

pub struct SpawnInfo<B: Bundle> {
    pub locations: Vec<Vec2>,
    pub ttl_timer: Timer,
    pub frequency_timer: Timer,
    pub bundle: B,
}
pub trait BundledAsset {
    fn get_bundle(asset_server: &Res<AssetServer>) -> AiActorBundle;
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup_level), //.with_system(spawn_startup_bundles::<Spawn>)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(periodic_spawn),
        );
    }
}

fn setup_level(time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AiActorSpawner {
        index: 0,
        spawn_infos: SpawnSequence::level0(&asset_server),
    });
}

fn periodic_spawn(mut commands: Commands, time: Res<Time>, mut query: Query<&mut AiActorSpawner, With<AiActorSpawner>>) {
    // Run logic for each Spawner Component
    for mut spawner in &mut query {
        let n_spawn_infos = spawner.spawn_infos.len() as i32;
        let cur_index = (spawner.index % n_spawn_infos) as usize;
        let spawn_info = &mut spawner.spawn_infos[cur_index];
        
        // Tick spawn timer
        spawn_info.frequency_timer.tick(time.delta());
        spawn_info.ttl_timer.tick(time.delta());

        if spawn_info.ttl_timer.finished() {
            spawner.index = min((cur_index + 1) as i32, n_spawn_infos); // <-- Second mutable borrow occurs here
        }

        if spawn_info.frequency_timer.finished(){
            commands.spawn(spawn_info.bundle); // <--The bundle is behind a mutable reference
        }
    }
}


/*
pub fn spawn_startup_bundles<B: Spawn>(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
) {
    let bundle = B::get_bundle(asset_server, Vec2::new(23.0, 2.0));
    commands.spawn(bundle);
}
*/
