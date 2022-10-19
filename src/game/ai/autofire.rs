use crate::game::events::AudioEvent;

use super::super::super::AppState;
use super::super::events::WeaponFiredEvent;
use super::super::collisions::CollisionEvent;
use super::super::components::{AutoFire, Weapon, Collider};
use super::super::constants::*;
use bevy::{prelude::*, time::*};

pub struct AutoFirePlugin;


// Plugin definition
impl Plugin for AutoFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(fire_controller),
            );
    }
}


// Fire controller system
pub fn fire_controller(
    time: Res<Time>,
    mut bullet_fired_event: EventWriter<WeaponFiredEvent>,
    mut audio_event: EventWriter<AudioEvent>,
    mut query: Query<(&Transform, &Collider, &Weapon, &mut AutoFire)>,
) {
    for (transform, collider, weapon, mut auto_fire) in &mut query {
        // ref: https://bevy-cheatbook.github.io/features/time.html
        auto_fire.cooldown_timer.tick(time.delta());
        if auto_fire.cooldown_timer.finished() {
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
            audio_event.send(AudioEvent { clip: weapon.firing_audio_clip.clone()})
        }
    }
}
