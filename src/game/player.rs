use super::super::AppState;
use super::events::AudioEvent;
use super::events::WeaponFiredEvent;
use super::collisions::CollisionEvent;
use super::components::*;
use super::constants::*;
use super::collisions::check_collisions;
use super::actor::ship::player_ship;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};


pub struct PlayerPlugin;

// Plugin definition
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<AudioEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(player_controller.before(check_collisions))
                    .with_system(fire_controller)
                    .with_system(reflect_from_wall.before(check_collisions).after(player_controller)),
            );
    }
}

// SYSTEMS

// Player spawner system
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    commands
        .spawn(player_ship(Vec2::new(-20.0, 0.0), asset_server))
        .insert(Player);
}

// Player controller system
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_query: Query<(&mut Transform, &Actor), With<Player>>,
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
        
        // Calculate the new position based on player input
        ship_transform.translation.y = ship_transform.translation.y + direction_y;
        ship_transform.translation.x = ship_transform.translation.x + direction_x;
    }
}

pub fn reflect_from_wall(
    mut ship_query: Query<(&mut Transform, &Collider, &Actor), With<Player>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
) {
    for (mut ship_transform, ship_collider, ship) in &mut ship_query {
        // FIXME: Call this from player_controller to eliminate redundant transform query
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
    mut audio_event: EventWriter<AudioEvent>,
    mut bullet_fired_event: EventWriter<WeaponFiredEvent>,
    query: Query<(&Transform, &Weapon, &Collider), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (transform, weapon, collider) in &query {
            let event = WeaponFiredEvent {
                bullet_type: weapon.bullet_type.clone(),
                translation: Vec2::new(
                    transform.translation.x + weapon.offset.x * transform.forward().x,
                    transform.translation.y + weapon.offset.y * transform.forward().y,
                ),
                rotation: transform.rotation,
                hitmask: collider.hitmask, // Bullets have the same hitmask as the collider attached to the firer
            };
            bullet_fired_event.send(event);
            audio_event.send(AudioEvent { clip: weapon.firing_audio_clip})

        }
    }
}
