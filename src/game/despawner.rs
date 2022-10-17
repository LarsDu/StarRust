use bevy::{prelude::*, time::*};
use super::constants::*;
use super::components::*;
use super::super::AppState;

pub struct DespawnerPlugin;

impl Plugin for DespawnerPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                .with_system(timed_despawn)
        );
    }
}

fn timed_despawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut TimedDespawn), With<TimedDespawn>>
){
    for (entity, transform, mut despawner) in &mut query{
        
        let pos = transform.translation;
        if pos.x < LEFT_WALL || pos.x > RIGHT_WALL + SPAWN_OFFSET || pos.y > TOP_WALL || pos.y < BOTTOM_WALL{
            despawner.timer.tick(time.delta());
            if despawner.timer.finished(){
                //println!("Despawning {}", entity.id());
                commands.entity(entity).despawn();
            }
        } else {
            despawner.timer.reset();
        }
    }

}