use crate::menus::MenuState;

use super::super::AppState;
use super::actor::ship::PlayerShipDefault;
use super::actor::BundledActor;
use super::collisions::check_collisions;
use super::collisions::{check_aabb_collision, Collision, CollisionEvent};
use super::components::*;
use super::constants::PLAYER_SPAWN_POS;
use super::events::WeaponFiredEvent;
use super::events::{AudioEvent, PlayerDeathEvent};
use super::models::{setup_resources, ModelsAssets};

use super::AudioClipAssets;
use bevy::prelude::*;

pub struct PlayerPlugin;

// Plugin definition
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_resources)
            .add_message::<WeaponFiredEvent>()
            .add_message::<CollisionEvent>()
            .add_message::<AudioEvent>()
            .add_message::<PlayerDeathEvent>()
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                (
                    player_controller.before(check_collisions),
                    fire_controller,
                    reflect_from_wall
                        .before(check_collisions)
                        .after(player_controller),
                    on_player_death,
                ),
            );
    }
}

// SYSTEMS
// Player spawner system
pub fn spawn_player(
    mut commands: Commands,
    audio_clips: Res<AudioClipAssets>,
    models: Res<ModelsAssets>,
) {
    commands.spawn(PlayerShipDefault::get_bundle(
        &audio_clips,
        &models,
        PLAYER_SPAWN_POS,
    ));
}

// Player controller system
fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ship_query: Query<(&mut Transform, &Actor), With<Player>>,
) {
    for (mut ship_transform, ship) in &mut ship_query {
        let mut direction_x: f32 = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction_y -= ship.speed.y;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction_y += ship.speed.y;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction_x -= ship.speed.x;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
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

            let collision = check_aabb_collision(
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
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bullet_fired_event: MessageWriter<WeaponFiredEvent>,
    mut audio_event: MessageWriter<AudioEvent>,
    mut query: Query<(&Transform, &mut Weapon, &Collider), With<Player>>,
) {
    for (transform, mut weapon, collider) in &mut query {
        if keyboard_input.just_pressed(KeyCode::Space) {
            send_projectile_spawn_event(
                transform,
                collider,
                &weapon,
                &mut bullet_fired_event,
                &mut audio_event,
            );
            weapon.cooldown_timer.reset();
            weapon.cooldown_timer.set_mode(TimerMode::Repeating);
            weapon.cooldown_timer.unpause();
        } else if keyboard_input.just_released(KeyCode::Space) {
            weapon.cooldown_timer.pause()
        }
        weapon.cooldown_timer.tick(time.delta());
        if weapon.cooldown_timer.just_finished() {
            send_projectile_spawn_event(
                transform,
                collider,
                &weapon,
                &mut bullet_fired_event,
                &mut audio_event,
            );
        }
    }
}

fn send_projectile_spawn_event(
    transform: &Transform,
    collider: &Collider,
    weapon: &Weapon,
    bullet_fired_event: &mut MessageWriter<WeaponFiredEvent>,
    audio_event: &mut MessageWriter<AudioEvent>,
) {
    let event = WeaponFiredEvent {
        bullet_type: weapon.bullet_type.clone(),
        translation: Vec2::new(
            transform.translation.x + weapon.offset.x * transform.forward().x,
            transform.translation.y + weapon.offset.y * transform.forward().y,
        ),
        rotation: transform.rotation,
        hitmask: collider.hitmask, // Bullets have the same hitmask as the collider attached to the firer
    };
    bullet_fired_event.write(event);
    audio_event.write(AudioEvent {
        clip: weapon.firing_audio_clip.clone(),
    }); // TODO: Perhaps tie this audio event to the bullet fired event rather than with the player controls!
}

fn on_player_death(
    mut death_events: MessageReader<PlayerDeathEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    if !death_events.is_empty() {
        // See: https://bevyengine.org/examples/Games/game-menu/
        menu_state.set(MenuState::PlayerDeath);
        game_state.set(AppState::Menu);
        death_events.clear();
    }
}
