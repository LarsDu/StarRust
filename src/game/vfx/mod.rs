use std::f32::consts::PI;

use crate::constants::CAMERA_FAR;
use bevy::prelude::*;
use std::time::Duration;

use fastrand;

use super::components::*;
use super::events::*;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CameraShakeEvent>()
            .add_message::<ExplosionEvent>()
            .add_systems(Update, shake_camera);
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
    asset_server: Res<AssetServer>,
) {
    if events.is_empty() {
        return;
    }
}
