use std::f32::consts::PI;

use bevy::{prelude::*, time::FixedTimestep, utils::Duration};
use rand::{Rng, thread_rng};
use crate::constants::CAMERA_DEPTH;

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

fn shake_camera(
    time: Res<Time>,
    mut shake_events: EventReader<CameraShakeEvent>,
    mut camera_query: Query<(&mut CameraShaker, &mut Transform), With<Camera>>
){

    for (mut shaker, mut t) in &mut camera_query{
        if !shake_events.is_empty(){
            for shake_event in shake_events.iter(){
                shaker.timer.reset();
                shaker.timer.set_duration(Duration::from_secs_f32(shake_event.duration_secs));
                shaker.magnitude = shake_event.magnitude
            }
        }
        // Tick the shaker
        shaker.timer.tick(time.delta());

        // Shake time goes from higher to lower as the shake progresses and should land at around 0.0
        let shake_time = shaker.timer.duration().as_secs_f32() - shaker.timer.elapsed_secs();
        if shake_time > 0.001{
            let mut rng = thread_rng();
            let magnitude_at_time = shaker.magnitude * shake_time;
            let theta = magnitude_at_time * rng.gen_range(0.0..1.0) * 2.0 * PI;
            t.translation = Vec3::new(theta.cos(), theta.sin(), CAMERA_DEPTH);
        }

    }
}

fn on_particle_event(){

}