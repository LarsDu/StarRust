use crate::utils::despawn_all;
use bevy::{prelude::*, time::*};

use super::super::AppState;
use super::components::*;
use super::constants::*;

pub struct DespawnerPlugin;

impl Plugin for DespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                //.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(timed_oob_despawn)
                .with_system(timed_despawn),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(despawn_all::<TimedDespawn>)
                .with_system(despawn_all::<TimedOobDespawn>),
        );
    }
}

fn timed_despawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TimedDespawn)>,
) {
    for (entity, mut despawner) in &mut query {
        despawner.timer.tick(time.delta());
        if despawner.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn timed_oob_despawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut TimedOobDespawn)>,
) {
    // Timed despawn for entities that go out of bounds
    for (entity, transform, mut despawner) in &mut query {
        let pos = transform.translation;
        if pos.x < LEFT_WALL
            || pos.x > RIGHT_WALL + SPAWN_OFFSET
            || pos.y > TOP_WALL
            || pos.y < BOTTOM_WALL
        {
            despawner.timer.tick(time.delta());
            if despawner.timer.finished() {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            despawner.timer.reset();
        }
    }
}
