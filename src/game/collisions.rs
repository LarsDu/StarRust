use crate::game::components::*;
use crate::game::events::*;
use bevy::prelude::*;
use std::cmp::max;

#[derive(Default, Message)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WeaponFiredEvent>()
            .add_message::<CollisionEvent>()
            .add_message::<PlayerDeathEvent>()
            .add_systems(FixedUpdate, check_collisions);
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut audio_event: MessageWriter<AudioEvent>,
    mut collision_event: MessageWriter<CollisionEvent>,
    mut camera_shake_event: MessageWriter<CameraShakeEvent>,
    mut explosion_event: MessageWriter<ExplosionEvent>,
    mut player_death_event: MessageWriter<PlayerDeathEvent>,
    mut score_event: MessageWriter<ScoreEvent>,
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
            if a_entity.index() == b_entity.index()
                || (a_collider.hitmask ^ b_collider.hitmask) == 0
            {
                continue;
            }

            let collision = check_aabb_collision(
                a_transform.translation,
                a_collider.rect,
                b_transform.translation,
                b_collider.rect,
            );
            if collision.is_some() {
                if let Some(_) = a_bullet {
                    // If a is a bullet, despawn it on impact
                    commands.entity(a_entity).despawn();
                }
                b_health.hp = max(b_health.hp - a_collider.damage, 0);

                // Play damage sound
                if a_collider.damage > 0 {
                    //audio_event.write(AudioEvent {
                    //    clip: b_health.damage_sound.clone(),
                    //});
                }

                if b_health.hp == 0 {
                    if let Some(d) = b_death_points {
                        score_event.write(ScoreEvent {
                            increment: d.points,
                        });
                    }
                    if let Some(s) = b_camera_shake {
                        camera_shake_event.write(CameraShakeEvent {
                            magnitude: s.magnitude,
                            duration_secs: s.duration_secs,
                        });
                        explosion_event.write(ExplosionEvent {
                            position: b_transform.translation,
                            lifetime: 0.25,
                        });
                    }

                    if let Some(_) = b_player {
                        player_death_event.write(PlayerDeathEvent::default());
                    }

                    // Play death sound
                    audio_event.write(AudioEvent {
                        clip: b_health.death_sound.clone(),
                    });

                    commands.entity(b_entity).despawn();
                }

                collision_event.write_default();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

pub struct CollisionBox {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl CollisionBox {
    pub fn new(pos: Vec3, size: Vec2) -> Self {
        Self {
            top: pos.y + size.y / 2.,
            bottom: pos.y - size.y / 2.,
            left: pos.x - size.x / 2.,
            right: pos.x + size.x / 2.,
        }
    }
}

// From https://github.com/bevyengine/bevy/blob/6a3b059db917999b15ca032a4cab8cd31569b896/crates/bevy_sprite/src/collide_aabb.rs
pub fn check_aabb_collision(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
) -> Option<Collision> {
    let a = CollisionBox::new(a_pos, a_size);
    let b = CollisionBox::new(b_pos, b_size);

    // check to see if the two rectangles are intersecting
    if a.left < b.right && a.right > b.left && a.bottom < b.top && a.top > b.bottom {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a.left < b.left && a.right > b.left && a.right < b.right {
            (Collision::Left, b.left - a.right)
        } else if a.left > b.left && a.left < b.right && a.right > b.right {
            (Collision::Right, a.left - b.right)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a.bottom < b.bottom && a.top > b.bottom && a.top < b.top {
            (Collision::Bottom, b.bottom - a.top)
        } else if a.bottom > b.bottom && a.bottom < b.top && a.top > b.top {
            (Collision::Top, a.bottom - b.top)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth.abs() < x_depth.abs() {
            Some(y_collision)
        } else {
            Some(x_collision)
        }
    } else {
        None
    }
}
