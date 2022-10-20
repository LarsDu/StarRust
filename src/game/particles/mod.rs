/// Plugin for spawning 3d quad particles
/// with minimal dependencies and should be WASM compatible

use bevy::{prelude::*, time::*};

pub mod components;
use components::*;

pub mod events;
use events::*;

pub struct BasicParticlesPlugin;

impl Plugin for BasicParticlesPlugin{
    fn build(&self, app: &mut App){
        app.add_system(update_particles)
           .add_system(update_emitter);
    }
}

fn update_particles

fn update_emitter(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BasicParticleEmitter)>
) {
    for (entity, emitter) in &mut query{
        emitter.lifetime_timer.tick(time.delta());
        if emitter.lifetime_timer.just_finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}
