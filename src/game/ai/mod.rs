use crate::game::collisions::check_collisions;
use crate::game::components::*;
use crate::AppState;
use bevy::prelude::*;

pub mod autofire;
use autofire::AutoFirePlugin;

#[derive(Copy, Clone, Default)]
pub enum AiMode {
    #[default]
    NoMovement,
    ChargeForward1,
    Sinusoid1,
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutoFirePlugin).add_systems(
            FixedUpdate,
            update_ai
                .before(check_collisions)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_ai(time: Res<Time>, mut query: Query<(&mut Transform, &Actor, &mut Ai), With<Ai>>) {
    for (transform, actor, mut ai) in &mut query {
        ai.timer.tick(time.delta());
        match ai.mode {
            AiMode::NoMovement => {}
            AiMode::ChargeForward1 => charge_forward(transform, actor.speed.length()),
            AiMode::Sinusoid1 => sine_charge(&time, transform, actor.speed.length(), 1.5, 1.0),
        }
    }
}

fn charge_forward(mut t: Mut<Transform>, speed: f32) {
    t.translation = t.translation + t.forward() * speed;
}

fn sine_charge(
    time: &Res<Time>,
    mut t: Mut<Transform>,
    forward_speed: f32,
    amplitude: f32,
    frequency: f32,
) {
    let forward = t.translation + t.forward() * forward_speed;
    let up_down = t.up() * amplitude * (time.elapsed_secs() * frequency).sin();
    t.translation = forward + up_down;
}
