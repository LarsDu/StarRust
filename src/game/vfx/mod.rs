use bevy::{prelude::*, time::FixedTimestep, utils::Duration};
use super::super::AppState;
use super::constants::*;
use super::components::*;
use super::events::*;

pub struct VfxPlugin;

impl Plugin for VfxPlugin{
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
        .add_event::<CameraShakeEvent>()
        .add_event::<ParticleEvent>()
        .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(shake_camera)
                    .with_system(on_particle_event)
            );
    }
}

fn shake_camera(){

}

fn on_particle_event(){

}