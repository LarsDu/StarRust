use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::{Enemy, FuseTime, Health, Ship};
use super::constants::*;
use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*, utils::Duration};
use std::f32::consts::PI;

pub struct EnemyPlugin;


// Plugin definition
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(movement)
                    .with_system(fire_controller),

            );
    }
}

// SYSTEMS

pub fn spawn(time: Res<Time>, commands: Commands, asset_server: Res<AssetServer>) {
    spawn_at(Vec2::new(25.0, 2.0), commands, asset_server);
}

// Enemy spawner system
pub fn spawn_at(position: Vec2, mut commands: Commands, asset_server: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    commands
        .spawn(
            default_enemy_ship_bundle(position, asset_server)
        )    
        .insert(Enemy);
        
}

// Enemy controller system
fn movement(time: Res<Time>, query: Query<(&mut Transform, &Enemy), With<Enemy>>) {

}

// Fire controller system
pub fn fire_controller(
    time: Res<Time>,
    mut bullet_fired_event: EventWriter<BulletFiredEvent>,
    mut query: Query<(&Transform, &Ship, &mut FuseTime), With<Enemy>>,
) {
    for (transform, ship, mut fuse_timer) in &mut query {
        // ref: https://bevy-cheatbook.github.io/features/time.html
        fuse_timer.timer.tick(time.delta());
        if fuse_timer.timer.finished() {
            let event = BulletFiredEvent {
                translation: Vec2::new(
                    transform.translation.x + ship.gun_offset.x * transform.forward().x,
                    transform.translation.y + ship.gun_offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: ALLY_HITMASK, // Hurt player only
            };
            bullet_fired_event.send(event);
        }
    }
}
