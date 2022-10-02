use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::*;
use super::constants::*;
use super::ship::yard::player_ship_bundle;
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
    commands
        .spawn(player_ship_bundle(Vec2::new(-20.0, 0.0), asset_server))
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
    mut ship_query: Query<(&mut Transform, &Collider, &Ship), With<Player>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
) {
    for (mut ship_transform, ship_collider, ship) in &mut ship_query {
        // Bounce back on wall collision
        for wall_transform in &wall_query {
            let mut direction_x: f32 = 0.0;
            let mut direction_y = 0.0;

            let collision = collide(
                wall_transform.translation,
                wall_transform.scale.truncate(),
                ship_transform.translation,
                ship_collider.rect,
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
                hitmask: ENEMY_HITMASK, //Hurt enemies only
            };
            bullet_fired_event.send(event);
        }
    }
}
