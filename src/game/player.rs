use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::*;
use super::constants::*;
use super::ships::PLAYER_SHIP;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

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
                    .with_system(fire_controller)
                    .with_system(reflect_from_wall),
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
            transform: Transform::from_xyz(-20.0, 0.0, 2.0)
                .with_scale(Vec3::splat(1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
            ..Default::default()
        })
        .insert(PLAYER_SHIP.clone())
        .insert(Collider {
            damage: 1,
            hitmask: 1,
        })
        .insert(Health { hp: 5 })
        .insert(Player);
}

// Player controller system
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_query: Query<(&mut Transform, &Ship), With<Player>>,
) {
    for (mut ship_transform, ship) in &mut ship_query {
        let mut direction_x: f32 = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= ship.speed.y;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += ship.speed.y;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= ship.speed.x;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += ship.speed.x;
        }

        // Calculate the newosition based on player input
        ship_transform.translation.y = ship_transform.translation.y + direction_y;
        ship_transform.translation.x = ship_transform.translation.x + direction_x;
    }
}

pub fn reflect_from_wall(
    mut ship_query: Query<(&mut Transform, &Ship), With<Player>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>
) {
    for (mut ship_transform, ship) in &mut ship_query {
        // Bounce back on wall collision
        for wall_transform in &wall_query {
            let mut direction_x: f32 = 0.0;
            let mut direction_y = 0.0;

            let collision = collide(
                wall_transform.translation,
                wall_transform.scale.truncate(), //FIXME!
                ship_transform.translation,
                ship_transform.scale.truncate(), //FIXME
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => direction_x += ship.speed.x,
                    Collision::Right => direction_x -= ship.speed.x,
                    Collision::Top => direction_y -= ship.speed.y,
                    Collision::Bottom => direction_y += ship.speed.y,
                    Collision::Inside => { /* do nothing */ }
                }
            }
            // Calculate the new horizontal paddle position based on wall reflection
            ship_transform.translation.y = ship_transform.translation.y + direction_y;
            ship_transform.translation.x = ship_transform.translation.x + direction_x;
        }
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
                    transform.translation.x + ship.gun_offset.x * transform.forward().x,
                    transform.translation.y + ship.gun_offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: 2,
            };
            bullet_fired_event.send(event);
        }
    }
}
