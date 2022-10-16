use super::super::super::AppState;
use super::super::bullet::BulletFiredEvent;
use super::super::collisions::CollisionEvent;
use super::super::components::{AutoFire, Actor};
use super::super::constants::*;
//use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*};

pub struct AutoFirePlugin;


// Plugin definition
impl Plugin for AutoFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
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
    mut bullet_fired_event: EventWriter<BulletFiredEvent>,
    mut query: Query<(&Transform, &Actor, &mut AutoFire), With<AutoFire>>,
) {
    for (transform, actor, mut auto_fire) in &mut query {
        // ref: https://bevy-cheatbook.github.io/features/time.html
        auto_fire.cooldown_timer.tick(time.delta());
        if auto_fire.cooldown_timer.finished() {
            let event = BulletFiredEvent {
                translation: Vec2::new(
                    transform.translation.x + actor.gun_offset.x * transform.forward().x,
                    transform.translation.y + actor.gun_offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: ALLY_HITMASK, // Hurt player only
            };
            bullet_fired_event.send(event);
        }
    }
}
