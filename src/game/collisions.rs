use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::components::{Bullet, Collider, Health, Player, Actor, Wall};
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
        app.add_event::<BulletFiredEvent>()
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
    ship_a_query: Query<(Entity, &Transform, &Collider), With<Actor>>,
    mut ship_b_query: Query<(Entity, &mut Health, &Transform, &Collider), With<Actor>>,
) {
    // TODO: Use quadtrees for more efficient collision resolution
    for (ship_a_entity, ship_a_transform, ship_a_collider) in &ship_a_query {
        for (ship_b_entity, mut ship_b_health, ship_b_transform, ship_b_collider) in
            &mut ship_b_query
        {
            // Skip self-collisions
            if ship_a_entity.id() == ship_b_entity.id() {
                continue;
            }
            let collision = collide(
                ship_a_transform.translation,
                ship_a_collider.rect,
                ship_b_transform.translation,
                ship_b_collider.rect,
            );
            if collision.is_some() {
                ship_b_health.hp = max(ship_b_health.hp - ship_a_collider.damage, 0);
                /*println!(
                    "ship A({}) hit ship B({}) which has hp {}",
                    ship_a_entity.id(),
                    ship_b_entity.id(),
                    ship_b_health.hp
                );
                */
                if ship_b_health.hp == 0 {
                    commands.entity(ship_b_entity).despawn_recursive();
                }

                collision_event.send_default();
            }
        }
    }
}

pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_event: EventWriter<CollisionEvent>,
    bullet_query: Query<(Entity, &Bullet, &Collider, &Transform), With<Bullet>>,
    mut ship_query: Query<(Entity, &Collider, &mut Health, &Transform), With<Actor>>,
) {
    // This would be more efficient with quadtrees fyi
    for (bullet_entity, bullet, bullet_collider, bullet_transform) in &bullet_query {
        for (ship_entity, ship_collider, mut ship_health, ship_transform) in &mut ship_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_collider.rect,
                ship_transform.translation,
                ship_collider.rect,
            );
            if collision.is_some() && ((bullet_collider.hitmask & ship_collider.hitmask) > 0){
                //println!("Contact!");
                commands.entity(bullet_entity).despawn();
                ship_health.hp = max(ship_health.hp - bullet_collider.damage, 0);
                if ship_health.hp == 0 {
                    // TODO kill the ship
                    println!("Despawn entity {}!", ship_entity.id());
                    commands.entity(ship_entity).despawn_recursive();
                }

                collision_event.send_default();
            }
        }
    }
}
