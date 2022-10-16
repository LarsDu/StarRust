use super::super::AppState;
use super::components::*;
use bevy::{
    prelude::*,
    time::{FixedTimestep}
};
pub mod autofire;
use autofire::AutoFirePlugin;

#[derive(Copy,Clone)]
pub enum AiMode{
    NO_MOVEMENT,
    FORWARD_BACK1,
    CHARGE_FORWARD1,
    SINUSOID1,
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App){
        app.add_plugin(AutoFirePlugin)
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                .with_system(update_ai)
        );
    }
}

fn update_ai(
    time: Res<Time>,
    mut query: Query< (&mut Transform, &Actor, &mut Ai), With<Ai>>
){
    for (transform, actor, mut ai) in &mut query{
        ai.timer.tick(time.delta());
        match ai.mode{
            AiMode::NO_MOVEMENT => {},
            AiMode::FORWARD_BACK1 => {},
            AiMode::CHARGE_FORWARD1 => charge_forward(transform,actor.speed.length()),
            AiMode::SINUSOID1 => sine_charge(&time, transform, actor.speed.length(), 0.05, 1.0),
            _ => {}
        }
    }
}

fn charge_forward(
    mut t: Mut<Transform>,
    speed: f32
){
    t.translation = t.translation + speed * t.forward();  
}

fn sine_charge(
    time: &Res<Time>,
    mut t: Mut<Transform>,
    forward_speed: f32,
    amplitude: f32, frequency: f32){
    
    let forward = t.translation + forward_speed * t.forward();  
    let up_down = t.up() * amplitude * (time.seconds_since_startup() as f32 * frequency).sin();
    t.translation = forward + up_down;

}

