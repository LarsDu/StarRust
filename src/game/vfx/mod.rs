use std::f32::consts::PI;

use crate::constants::CAMERA_FAR;
use bevy::{prelude::*, utils::Duration};

use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient, Module,
    ParticleEffect, ParticleEffectBundle, SetAttributeModifier, SetPositionCircleModifier,
    ShapeDimension, SizeOverLifetimeModifier, Spawner, SetVelocityCircleModifier
};

use fastrand;

use super::components::*;
use super::events::*;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraShakeEvent>()
            .add_event::<ExplosionEvent>()
            .add_systems(Update, (shake_camera, on_explosion_event));
    }
}

fn shake_camera(
    time: Res<Time>,
    mut shake_events: EventReader<CameraShakeEvent>,
    mut camera_query: Query<(&mut CameraShaker, &mut Transform), With<Camera>>,
) {
    for (mut shaker, mut t) in &mut camera_query {
        if !shake_events.is_empty() {
            for shake_event in shake_events.read() {
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

        if shake_time > 0.01 {
            let magnitude_at_time = shaker.magnitude * shake_time;
            let theta = fastrand::f32() * 2.0 * PI;
            t.translation = Vec3::new(
                magnitude_at_time * theta.cos(),
                magnitude_at_time * theta.sin(),
                CAMERA_FAR,
            );
        }
    }
}

fn on_explosion_event(
    mut events: EventReader<ExplosionEvent>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    if events.is_empty() {
        return;
    }

    for explosion_data in &mut events.read() {
        // Define a color gradient
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1., 0.8, 0., 1.)); // Orange
        gradient.add_key(1.0, Vec4::ZERO); // Transparent black
        let mut module = Module::default();

        let init_pos = SetPositionCircleModifier {
            center: module.lit(Vec3::ZERO),
            radius: module.lit(0.5),
            dimension: ShapeDimension::Surface,
            axis: module.lit(Vec3::new(0.0, 0.0, 1.0)),
        };

          // Also initialize a radial initial velocity
        // away from the (same) sphere center.
        let init_vel = SetVelocityCircleModifier {
            center: module.lit(Vec3::ZERO),
            axis: module.lit(Vec3::new(0., 0., 1.)),
            speed: module.lit(1000.0),
        };


        // Initialize the total lifetime of the particle, that is
        // the time for which it's simulated and rendered. This modifier
        // is almost always required, otherwise the particles won't show.
        let lifetime = module.lit(explosion_data.lifetime); // literal value "10.0"
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Every frame, add a gravity-like acceleration downward
        let accel = module.lit(Vec3::new(0., -1., 0.));
        let update_accel = AccelModifier::new(accel);

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2::new(3.0, 3.0));
        //size_gradient.add_key(0.8, Vec2::new(0.4,0.4));
        size_gradient.add_key(1.0, Vec2::new(0.05, 0.05));
        let effect = EffectAsset::new(vec![2048], Spawner::rate(40.0.into()), module)// This rate is the count
            .with_name("Blast")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .update(update_accel)
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                ..default()
            });

        let effect_handle = effects.add(effect);
        commands
            .spawn(ParticleEffectBundle {
                effect: ParticleEffect::new(effect_handle),
                transform: Transform::from_translation(explosion_data.position),
                ..Default::default()
            })
            .insert(TimedDespawn {
                timer: Timer::new(
                    Duration::from_secs_f32(explosion_data.lifetime),
                    TimerMode::Once
                ),
            });
    }
}
