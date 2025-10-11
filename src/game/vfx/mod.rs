use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient as HanabiGradient;
use fastrand;

use crate::constants::CAMERA_FAR;

use super::components::*;
use super::events::*;

const EXPLOSION_PARTICLE_LIFETIME: f32 = 0.6;
const EXPLOSION_SPAWN_COUNT: f32 = 160.0;
const EXPLOSION_RADIUS: f32 = 6.0;
const EXPLOSION_BASE_SPEED: f32 = 42.0;

#[derive(Resource, Clone)]
struct ExplosionEffectHandle(pub Handle<EffectAsset>);

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CameraShakeEvent>()
            .add_message::<ExplosionEvent>()
            .add_systems(Startup, setup_explosion_effect)
            .add_systems(Update, (shake_camera, on_explosion_event));
    }
}

fn shake_camera(
    time: Res<Time>,
    mut shake_events: MessageReader<CameraShakeEvent>,
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
    mut events: MessageReader<ExplosionEvent>,
    mut commands: Commands,
    effect: Res<ExplosionEffectHandle>,
) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        let lifetime = event
            .lifetime
            .max(EXPLOSION_PARTICLE_LIFETIME)
            .max(f32::EPSILON);

        commands.spawn((
            ParticleEffect::new(effect.0.clone()),
            EffectProperties::default(),
            Transform::from_translation(event.position),
            GlobalTransform::default(),
            TimedDespawn {
                timer: Timer::from_seconds(lifetime, TimerMode::Once),
            },
            Name::new("ExplosionVfx"),
        ));
    }
}

fn setup_explosion_effect(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient = HanabiGradient::<Vec4>::new();
    color_gradient.add_key(0.0, Vec4::new(2.5, 1.4, 0.4, 1.0));
    color_gradient.add_key(0.15, Vec4::new(1.8, 0.6, 0.1, 0.9));
    color_gradient.add_key(0.45, Vec4::new(0.9, 0.2, 0.05, 0.5));
    color_gradient.add_key(1.0, Vec4::ZERO);

    let mut size_gradient = HanabiGradient::<Vec3>::new();
    size_gradient.add_key(0.0, Vec3::splat(0.4));
    size_gradient.add_key(0.3, Vec3::splat(0.8));
    size_gradient.add_key(1.0, Vec3::splat(0.05));

    let writer = ExprWriter::new();

    let init_position = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(EXPLOSION_RADIUS).expr(),
        dimension: ShapeDimension::Surface,
    };
    let init_velocity = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(EXPLOSION_BASE_SPEED).expr(),
    };
    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(EXPLOSION_PARTICLE_LIFETIME).expr(),
    );
    let drag = LinearDragModifier::new(writer.lit(8.0).expr());

    let effect = EffectAsset::new(
        512,
        SpawnerSettings::once(EXPLOSION_SPAWN_COUNT.into()),
        writer.finish(),
    )
    .with_name("Explosion")
    .init(init_position)
    .init(init_velocity)
    .init(init_age)
    .init(init_lifetime)
    .update(drag)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient,
        blend: ColorBlendMode::Add,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false,
    });

    let handle = effects.add(effect);
    commands.insert_resource(ExplosionEffectHandle(handle));
}
