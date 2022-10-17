use super::super::AppState;
use super::events::WeaponFiredEvent;
use super::components::{Actor, Bullet, Collider, DeathPointsAwarded, Enemy, Health, Wall};
use super::events::ScoreEvent;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::*,
};
use std::cmp::max;

#[derive(Default)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(check_ship_collision)
                    .with_system(check_bullet_collision), //.with_system(check_player_ship_wall_collision)
            );
    }
}

pub fn check_ship_collision(
    mut commands: Commands,
    mut collision_event: EventWriter<CollisionEvent>,
    mut score_event: EventWriter<ScoreEvent>,
    a_query: Query<(Entity, &Transform, &Collider, Option<&Bullet>), With<Actor>>,
    mut b_query: Query<
        (
            Entity,
            &mut Health,
            &Transform,
            &Collider,
            Option<&Bullet>,
            Option<&DeathPointsAwarded>,
        ),
        With<Actor>,
    >,
) {
    // TODO: Use quadtrees for more efficient collision resolution
    for (a_entity, a_transform, a_collider, a_bullet) in &a_query {
        for (b_entity, mut b_health, b_transform, b_collider, b_bullet, death_points) in
            &mut b_query
        {
            // Skip self-collisions and identical hitmasks
            if a_entity.id() == b_entity.id()
                || (a_collider.hitmask & b_collider.hitmask) == 0
            {
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
                    println!("Despawn bullet a");
                    commands.entity(a_entity).despawn_recursive();
                }
                if let Some(_) = b_bullet {
                    // If a is a bullet, despawn it on impact
                    println!("Despawn bullet b");
                    commands.entity(b_entity).despawn_recursive();
                }
                b_health.hp = max(b_health.hp - a_collider.damage, 0);
                if b_health.hp == 0 {
                    if let Some(d) = death_points {
                        score_event.send(ScoreEvent{increment: d.points});
                    }
                    commands.entity(b_entity).despawn_recursive();
                }

                collision_event.send_default();
            }
        }
    }
}

// TODO: Get rid of this collision check entirely. Replace bullets with actors
pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_event: EventWriter<CollisionEvent>,
    mut score_event: EventWriter<ScoreEvent>,
    bullet_query: Query<(Entity, &Bullet, &Collider, &Transform), With<Bullet>>,
    mut ship_query: Query<(Entity, &Collider, &mut Health, &Transform, Option<&DeathPointsAwarded>), With<Actor>>,
) {
    // This would be more efficient with quadtrees fyi
    for (bullet_entity, bullet, bullet_collider, bullet_transform) in &bullet_query {
        for (ship_entity, ship_collider, mut ship_health, ship_transform, death_points) in
            &mut ship_query
        {
            let collision = collide(
                bullet_transform.translation,
                bullet_collider.rect,
                ship_transform.translation,
                ship_collider.rect,
            );
            if collision.is_some() && ((bullet_collider.hitmask & ship_collider.hitmask) > 0) {
                //println!("Contact!");
                commands.entity(bullet_entity).despawn();
                ship_health.hp = max(ship_health.hp - bullet_collider.damage, 0);
                if ship_health.hp == 0 {
                    // TODO kill the ship
                    if let Some(d) = death_points {
                        score_event.send(ScoreEvent{increment: d.points});
                    }
                    println!("Despawn entity {}!", ship_entity.id());
                    commands.entity(ship_entity).despawn_recursive();
                }

                collision_event.send_default();
            }
        }
    }
}
