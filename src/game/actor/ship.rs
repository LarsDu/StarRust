use bevy::{prelude::*, time::*, utils::Duration};

use super::super::components::*;
use super::*;
use super::super::spawner::*;
use crate::game::{ALLY_HITMASK, ENEMY_HITMASK, SPAWN_LOCATIONS};



pub fn player_ship(spawn_position: Vec2, asset_server: Res<AssetServer>) -> ActorBundle {
    return ActorBundle {
        actor: Actor {
            speed: Vec2::new(0.5, 0.5),
            gun_offset: Vec2::new(1.0, -0.32),
        },
        scene_bundle: StarRustSceneBundle {
            scene: asset_server.load("models/basic_hero.glb#Scene0"),
            transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                .with_scale(Vec3::splat(0.95))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
            ..default()
        },
        collider: Collider {
            rect: Vec2::new(1.5, 1.0),
            damage: 1,
            hitmask: ALLY_HITMASK,
        },
        health: Health { hp: 5 },
    };
}

pub struct DefaultEnemyShip;

impl BundledAsset for DefaultEnemyShip {
    fn get_bundle(asset_server: &Res<AssetServer>) -> AiActorBundle {
        let spawn_position = SPAWN_LOCATIONS[0];
        return AiActorBundle{
            ai: Ai{
               mode: AiMode::CHARGE_FORWARD1,
               timer: Timer::default()
            },
            actor_bundle: ActorBundle {
                actor: Actor {
                    speed: Vec2::new(0.1, 0.1),
                    gun_offset: Vec2::new(1.0, 0.0),
                },
                scene_bundle: StarRustSceneBundle {
                    scene: asset_server.load("models/basic_enemy.glb#Scene0"),
                    transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
                    ..default()
                },
                collider: Collider {
                    rect: Vec2::new(1.0, 1.0),
                    damage: 1,
                    hitmask: ENEMY_HITMASK,
                },
                health: Health { hp: 1 },
            },
            auto_fire: AutoFire {
                cooldown_timer: Timer::new(Duration::from_secs_f32(1.0), true),
            },
        };

    }
}


pub struct RaptorSineMovementVariant;

impl BundledAsset for RaptorSineMovementVariant {
    fn get_bundle(asset_server: &Res<AssetServer>) -> AiActorBundle {
        let mut variant = DefaultEnemyShip::get_bundle(asset_server).clone();
        variant.ai.mode = AiMode::SINUSOID1;
        return variant;
    }

}