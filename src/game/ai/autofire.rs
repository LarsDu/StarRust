use crate::game::events::AudioEvent;

use super::super::super::AppState;
use super::super::collisions::CollisionEvent;
use super::super::components::{AutoFire, Collider, Weapon};
use super::super::events::WeaponFiredEvent;
use bevy::prelude::*;

pub struct AutoFirePlugin;

// Plugin definition
impl Plugin for AutoFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_systems(Update, fire_controller.run_if(in_state(AppState::InGame)));
    }
}

// Fire controller system
pub fn fire_controller(
    time: Res<Time>,
    mut bullet_fired_event: EventWriter<WeaponFiredEvent>,
    mut audio_event: EventWriter<AudioEvent>,
    mut query: Query<(&Transform, &Collider, &mut Weapon), With<AutoFire>>,
) {
    for (transform, collider, mut weapon) in &mut query {
        // ref: https://bevy-cheatbook.github.io/features/time.html
        weapon.cooldown_timer.tick(time.delta());
        if weapon.cooldown_timer.finished() {
            let event = WeaponFiredEvent {
                bullet_type: weapon.bullet_type.clone(),
                translation: Vec2::new(
                    transform.translation.x + weapon.offset.x * transform.forward().x,
                    transform.translation.y + weapon.offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: collider.hitmask, // Hurt player only
            };
            bullet_fired_event.send(event);

            audio_event.send(AudioEvent {
                clip: weapon.firing_audio_clip.clone(),
            })
        }
    }
}
