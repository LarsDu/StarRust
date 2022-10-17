use super::super::super::AppState;
use super::super::events::WeaponFiredEvent;
use super::super::collisions::CollisionEvent;
use super::super::components::{AutoFire, Actor, Weapon};
use super::super::constants::*;
//use super::ship::yard::default_enemy_ship_bundle;
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
    mut query: Query<(&Transform, &Actor, &Weapon, &mut AutoFire), With<AutoFire>>,
) {
    for (transform, actor, weapon, mut auto_fire) in &mut query {
        // ref: https://bevy-cheatbook.github.io/features/time.html
        auto_fire.cooldown_timer.tick(time.delta());
        if auto_fire.cooldown_timer.finished() {
            let event = WeaponFiredEvent {
                translation: Vec2::new(
                    transform.translation.x + weapon.offset.x * transform.forward().x,
                    transform.translation.y + weapon.offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: ALLY_HITMASK, // Hurt player only
            };
            bullet_fired_event.send(event);
        }
    }
}
