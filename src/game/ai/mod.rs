use super::super::AppState;
use super::components::*;
use bevy::{
    prelude::*,
    time::{FixedTimestep}
};

pub enum AiMode{
    NO_MOVEMENT,
    FORWARD_BACK1,
    CHARGE_LEFT1,
    SINUSOID1,
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                .with_system(update_ai)
        );
    }
}

fn update_ai(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query< (&mut Transform, &Actor, &mut Ai), With<Ai>>
){

    for (transform, actor, mut ai) in &mut query{
        ai.timer.tick(time.delta());

        match ai.mode{
            AiMode::NO_MOVEMENT => {},
            AiMode::FORWARD_BACK1 => {},
            AiMode::CHARGE_LEFT1 => {},
            AiMode::SINUSOID1 => {},
            _ => {}

        }
    }
}

