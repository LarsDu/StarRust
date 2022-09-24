use std::cmp::max;
use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::components::{Bullet, Collider, Health, Ship};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::*,
};

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
                    .with_system(check_health_collision)
                    .with_system(check_bullet_collision),
            );
    }
}

pub fn check_health_collision(query: Query<(&Collider, &Health), With<Health>>) {}

pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_event: EventWriter<CollisionEvent>,
    bullet_query: Query<(Entity, &Bullet, &Collider, &Transform), With<Bullet>>,
    mut ship_query: Query<(Entity, &Collider, &mut Health, &Transform), With<Ship>>,
) {
    // This would be more efficient with quadtrees fyi
    for (bullet_entity, bullet, bullet_collider, bullet_transform) in &bullet_query {
        for (ship_entity, ship_collider, mut ship_health, ship_transform) in &mut ship_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(), //FIXME!
                ship_transform.translation,
                Vec2::new(1.0, 1.0),
            );
            if collision.is_some() {
                //println!("Contact!");
                commands.entity(bullet_entity).despawn();
                ship_health.hp = max(ship_health.hp - bullet_collider.damage, 0);
                if (ship_health.hp == 0){
                    // TODO kill the ship
                    println!("Despawn entity {}!", ship_entity.id());
                    commands.entity(ship_entity).despawn_recursive();
                }

                collision_event.send_default();

            }
        }
    }
}
