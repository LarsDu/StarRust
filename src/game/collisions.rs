use super::super::AppState;
use super::components::*;
use super::constants::TIME_STEP;
use super::events::*;
use bevy::{prelude::*, sprite::collide_aabb::collide, time::*};
use std::cmp::max;

#[derive(Default)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)) //FIXME: can potentially break WASM builds
                    .with_system(check_collisions),
            );
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut audio_event: EventWriter<AudioEvent>,
    mut collision_event: EventWriter<CollisionEvent>,
    mut camera_shake_event: EventWriter<CameraShakeEvent>,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut player_death_event: EventWriter<PlayerDeathEvent>,
    mut score_event: EventWriter<ScoreEvent>,
    a_query: Query<(Entity, &Transform, &Collider, Option<&Bullet>)>,
    mut b_query: Query<
        (
            Entity,
            &mut Health,
            &Transform,
            &Collider,
            Option<&DeathPointsAwarded>,
            Option<&CameraShakeOnDeath>,
            Option<&Player>,
        ),
        With<Actor>,
    >,
) {
    // TODO: Use quadtrees for more efficient collision resolution
    // TODO: Find a way to break up this giant function
    for (a_entity, a_transform, a_collider, a_bullet) in &a_query {
        for (
            b_entity,
            mut b_health,
            b_transform,
            b_collider,
            b_death_points,
            b_camera_shake,
            b_player,
        ) in &mut b_query
        {
            // Skip self-collisions and identical hitmasks
            if a_entity.index() == b_entity.index() || (a_collider.hitmask ^ b_collider.hitmask) == 0 {
                continue;
            }

            let collision = collide(
                a_transform.translation,
                a_collider.rect,
                b_transform.translation,
                b_collider.rect,
            );
            if collision.is_some() {
                if let Some(_) = a_bullet {
                    // If a is a bullet, despawn it on impact
                    commands.entity(a_entity).despawn_recursive();
                }
                b_health.hp = max(b_health.hp - a_collider.damage, 0);

                // Play damage sound
                if a_collider.damage > 0 {
                    audio_event.send(AudioEvent {
                        clip: b_health.damage_sound.clone(),
                    });
                }

                if b_health.hp == 0 {
                    if let Some(d) = b_death_points {
                        score_event.send(ScoreEvent {
                            increment: d.points,
                        });
                    }
                    if let Some(s) = b_camera_shake {
                        camera_shake_event.send(CameraShakeEvent {
                            magnitude: s.magnitude,
                            duration_secs: s.duration_secs,
                        });
                        explosion_event.send(ExplosionEvent {
                            position: b_transform.translation,
                            lifetime: 0.25,
                        });
                    }
                    if let Some(_) = b_player {
                        player_death_event.send(PlayerDeathEvent::default())
                    }

                    // Play death sound
                    audio_event.send(AudioEvent {
                        clip: b_health.death_sound.clone(),
                    });

                    commands.entity(b_entity).despawn_recursive();
                }

                collision_event.send_default();
            }
        }
    }
}
