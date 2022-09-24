use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::{Collider, Health, Player, Ship};
use super::ships::PLAYER_SHIP;
use super::constants::*;
use bevy::{prelude::*, time::FixedTimestep};

pub struct PlayerPlugin;

// Plugin definition
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(player_controller)
                    .with_system(fire_controller),
            );
    }
}

// SYSTEMS

// Player spawner system
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let scene_model = asset_server.load("models/basic_hero.glb#Scene0");

    commands
        .spawn_bundle(SceneBundle {
            scene: scene_model,
            transform: Transform::from_xyz(10.0, 0.0, 0.0)
                .with_scale(Vec3::splat(0.95))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
            ..Default::default()
        })
        .insert(PLAYER_SHIP.clone())
        .insert(Collider{ damage:1, hitmask:1 })
        .insert(Health { hp: 5 })
        .insert(Player);
}

// Player controller system
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Ship), With<Player>>,
) {
    for (mut transform, ship) in &mut query {
        let mut direction_x: f32 = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= ship.speed.x;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += ship.speed.y;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            direction_x += ship.speed.x;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x -= ship.speed.x;
        }

        // Calculate the new horizontal paddle position based on player input
        transform.translation.y = transform.translation.y + direction_y;
        transform.translation.x = transform.translation.x + direction_x;
    }
}
// Fire controller system
pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut bullet_fired_event: EventWriter<BulletFiredEvent>,
    query: Query<(&Transform, &Ship), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (transform, ship) in &query {
            let event = BulletFiredEvent {
                translation: Vec2::new(
                    transform.translation.x + ship.gun_offset.x,
                    transform.translation.y + ship.gun_offset.y,
                ),
                rotation: transform.rotation,
                hitmask: 2,
            };
            bullet_fired_event.send(event);
        }
    }
}
