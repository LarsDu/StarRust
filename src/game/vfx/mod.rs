use std::f32::consts::PI;

use crate::constants::CAMERA_DEPTH;
use bevy::{prelude::*, time::FixedTimestep, utils::Duration};
use bevy_hanabi::ParticleEffect;
use bevy_hanabi::ParticleLifetimeModifier;
use bevy_hanabi::PositionCircleModifier;
use bevy_hanabi::SizeOverLifetimeModifier;
use rand::{thread_rng, Rng};

use super::super::AppState;
use super::components::*;
use super::constants::*;
use super::events::*;
use bevy_hanabi::{
    AccelModifier, ColorOverLifetimeModifier, EffectAsset, Gradient,
    ShapeDimension, Spawner,
};

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_event::<CameraShakeEvent>()
            .add_event::<ExplosionEvent>()
            .add_system(on_explosion_event)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(shake_camera),
            );
    }
}

fn shake_camera(
    time: Res<Time>,
    mut shake_events: EventReader<CameraShakeEvent>,
    mut camera_query: Query<(&mut CameraShaker, &mut Transform), With<Camera>>,
) {
    for (mut shaker, mut t) in &mut camera_query {
        if !shake_events.is_empty() {
            for shake_event in shake_events.iter() {
                shaker.timer.reset();
                shaker
                    .timer
                    .set_duration(Duration::from_secs_f32(shake_event.duration_secs));
                shaker.magnitude = shake_event.magnitude
            }
        }
        // Tick the shaker
        shaker.timer.tick(time.delta());

        // Shake time goes from higher to lower as the shake progresses and should land at around 0.0
        let shake_time = shaker.timer.duration().as_secs_f32() - shaker.timer.elapsed_secs();
        if shake_time > 0.001 {
            let mut rng = thread_rng();
            let magnitude_at_time = shaker.magnitude * shake_time;
            let theta = magnitude_at_time * rng.gen_range(0.0..1.0) * 2.0 * PI;
            t.translation = Vec3::new(theta.cos(), theta.sin(), CAMERA_DEPTH);
        }
    }
}

// TODO: Hanabi doesn't work with wasm
fn on_explosion_event(
    mut events: EventReader<ExplosionEvent>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    if events.is_empty() {
        return;
    }
    for explosion_data in &mut events.iter() {
        // Define a color gradient
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1., 0.8, 0., 1.)); // Orange
        gradient.add_key(1.0, Vec4::ZERO); // Transparent black

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2::new(0.5, 0.5));
        //size_gradient.add_key(0.8, Vec2::new(0.4,0.4));
        size_gradient.add_key(1.0, Vec2::new(0.05,0.05));
        let effect = effects.add(
            EffectAsset {
                name: "Blast".to_string(),
                // Maximum number of particles alive at a time
                capacity: 2000,
                // Spawn at a rate of 25 particles per second
                spawner: Spawner::rate(25.0.into()),
                ..Default::default()
            }
            // On spawn, randomly initialize the position and velocity
            // of the particle over a sphere of radius 0.5 units, with a
            // radial initial velocity of 6 units/sec away from the
            // sphere center.
            .init(PositionCircleModifier {
                center: Vec3::ZERO,
                radius: 0.05,
                dimension: ShapeDimension::Surface,
                axis: Vec3::new(0.0, 0.0, 1.0),
                speed: 12.0.into(),
            })
            .init(ParticleLifetimeModifier {
                lifetime: explosion_data.lifetime,
            })
            // Every frame, add a gravity-like acceleration downward
            .update(AccelModifier {
                accel: Vec3::new(0., -0.2, 0.),
            })
            // Render the particles with a color gradient over their
            // lifetime.
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
            }),
        );

        commands
            .spawn(bevy_hanabi::ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                transform: Transform::from_translation(explosion_data.position),
                ..Default::default()
            })
            .insert(TimedDespawn {
                timer: Timer::new(Duration::from_secs_f32(explosion_data.lifetime), false),
            });
    }
}
