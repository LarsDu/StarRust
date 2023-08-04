use bevy::{prelude::*, utils::Duration};

use crate::utils::despawn_all;

use super::super::menus::MenuState;
use super::actor::{ship::*, *};
use super::components::*;
use super::events::LevelEndEvent;
use super::{super::*, models::ModelsAssets, AudioClipAssets};
use fastrand;
pub mod lvl;
use lvl::*;

// FIXME: Use enum rather than bundle here to make this
// capable of spawning any type of bundle!
pub struct LevelSpawnInfo {
    pub locations: Vec<Vec2>,
    pub ttl: f32,
    pub frequency: f32,
    pub spawn_func: fn(&mut Commands, &Res<AudioClipAssets>, &Res<ModelsAssets>, Vec2),
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEndEvent>()
            .add_systems(OnEnter(AppState::InGame), setup_level)
            .add_systems(OnExit(AppState::InGame), despawn_all::<AiActorSpawner>)
            .add_systems(Update(AppState::InGame), (level_periodic_spawn, level_ender));
    }
}

fn setup_level(
    mut commands: Commands,
    audio_clips: Res<AudioClipAssets>,
    models: Res<ModelsAssets>,
) {
    commands.spawn(AiActorSpawner::new(SpawnSequence::level0(
        &audio_clips,
        &models,
    )));
    commands.spawn(AiActorSpawner::new(SpawnSequence::level0_powerups(
        &audio_clips,
        &models,
    )));
}

fn level_periodic_spawn(
    mut commands: Commands,
    time: Res<Time>,
    models: Res<ModelsAssets>,
    audio_clips: Res<AudioClipAssets>,
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
                spawner.index = 0;
                level_end_event.send(LevelEndEvent {});
            }
        }

        if spawner.frequency_timer.finished() {
            let spawn_info = &spawner.spawn_infos[spawner.index as usize];

            spawn_from_spawn_info(&mut commands, spawn_info, &audio_clips, &models);
        }
    }
}

fn spawn_from_spawn_info(
    commands: &mut Commands,
    spawn_info: &LevelSpawnInfo,
    audio_clips: &Res<AudioClipAssets>,
    models: &Res<ModelsAssets>,
) {
    // Read from spawn info
    let rng = fastrand::Rng::new();
    let spawn_pos = spawn_info.locations[rng.usize(0..spawn_info.locations.len())];
    // Note: function must be wrapped in parenthesis
    // ref: https://stackoverflow.com/questions/37370120/
    (spawn_info.spawn_func)(commands, &audio_clips, &models, spawn_pos);
}

fn level_ender(
    mut events: EventReader<LevelEndEvent>,
    mut game_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if !events.is_empty() {
        // Use overwrite_set for events, since events may register over multiple frames
        menu_state.set(MenuState::LevelEnd);
        game_state.set(AppState::Menu);
        events.clear();
    }
}
