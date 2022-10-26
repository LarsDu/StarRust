use crate::menus::MenuState;

use super::super::AppState;
use super::actor::ship::PlayerShipDefault;
use super::actor::BundledActor;
use super::actor::PlayerActorBundle;
use super::collisions::check_collisions;
use super::collisions::CollisionEvent;
use super::components::*;
use super::constants::PLAYER_SPAWN_POS;
use super::events::WeaponFiredEvent;
use super::events::{AudioEvent, PlayerDeathEvent};
use super::scene;
use super::AudioClipAssets;
use super::SceneAssets;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

pub struct PlayerPlugin;

// Plugin definition
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(scene::setup_resources) //why does this need to be explicitly specified?
            .add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<AudioEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    //.with_run_criteria(FixedTimestep::step(TIMESTEPas f64))
                    .with_system(player_controller.before(check_collisions))
                    .with_system(fire_controller)
                    .with_system(
                        reflect_from_wall
                            .before(check_collisions)
                            .after(player_controller),
                    ),
            )
            .add_system(on_player_death);
    }
}

// SYSTEMS
// Player spawner system
pub fn spawn_player(
    mut commands: Commands,
    audio_clips: Res<AudioClipAssets>,
    models: Res<SceneAssets>,
) {
    commands.spawn(PlayerShipDefault::get_bundle(
        &audio_clips,
        &models,
        PLAYER_SPAWN_POS,
    ));
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
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut audio_event: EventWriter<AudioEvent>,
    mut bullet_fired_event: EventWriter<WeaponFiredEvent>,
    mut query: Query<(&Transform, &mut Weapon, &Collider), With<Player>>,
) {
    for (transform, mut weapon, collider) in &mut query {
        if keyboard_input.just_pressed(KeyCode::Space) {
            weapon.cooldown_timer.reset();
            weapon.cooldown_timer.set_mode(TimerMode::Repeating);
            weapon.cooldown_timer.unpause();
        } else if keyboard_input.just_released(KeyCode::Space) {
            weapon.cooldown_timer.pause()
        }
        weapon.cooldown_timer.tick(time.delta());

        if weapon.cooldown_timer.just_finished() {
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
            audio_event.send(AudioEvent {
                clip: weapon.firing_audio_clip.clone(),
            })
        }
    }
}

fn on_player_death(
    death_events: EventReader<PlayerDeathEvent>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<AppState>>,
    //query: Query<Entity, With<Player>>
) {
    if !death_events.is_empty() {
        // Currently panics
        menu_state.overwrite_set(MenuState::PlayerDeath).unwrap();
        game_state.overwrite_set(AppState::Menu).unwrap();
        death_events.clear();
    }
}
