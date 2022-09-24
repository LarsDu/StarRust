use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::{Collider, Enemy, FuseTime, Health, Ship};
use super::ships::DEFAULT_ENEMY;
use super::constants::*;
use bevy::{prelude::*, time::*, asset};
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
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(enemy_controller), //.with_system(fire_controller)
            );
    }
}

// SYSTEMS

pub fn spawn(time: Res<Time>, commands: Commands, asset_server: Res<AssetServer>) {
    spawn_at(Vec2::new(-15.0, 2.0), commands, asset_server);
}

// Enemy spawner system
pub fn spawn_at(position: Vec2, mut commands: Commands, asset_server: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    

    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("models/basic_enemy.glb#Scene0"),
            transform: Transform::from_xyz(position.x, position.y, 0.0)
                .with_scale(Vec3::splat(0.95))
                .with_rotation(Quat::from_rotation_y(1.5 * PI)),
            ..Default::default()
        })
        .insert(DEFAULT_ENEMY.clone())
        .insert(Collider{ damage:1, hitmask:1})
        .insert(Health { hp: 2 })
        .insert(Enemy);
}

// Enemy controller system
fn enemy_controller(time: Res<Time>, mut query: Query<(&mut Transform, &Enemy), With<Enemy>>) {


}

// Fire controller system
pub fn fire_controller(
    time: Res<Time>,
    mut bullet_fired_event: EventWriter<BulletFiredEvent>,
    query: Query<(&Transform, &Ship, &mut FuseTime), With<Enemy>>,
) {
    for (transform, ship, fuse_timer) in &query {
        /*fuse_timer.timer.tick(time.delta());
        if (fuse_timer.timer.finished()){
            let event = BulletFiredEvent {
                translation: Vec2::new(
                    transform.translation.x + ship.gun_offset.x,
                    transform.translation.y + ship.gun_offset.y,
                ),
                direction: transform.forward().truncate(),
                hitmask: 1,
            };
            bullet_fired_event.send(event);
        }*/
    }
}
