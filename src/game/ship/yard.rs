use bevy::{prelude::*, time::*, utils::Duration};

use super::super::components::*;
use super::*;
use crate::game::{ALLY_HITMASK, ENEMY_HITMASK};


pub trait AiShipSpawn{
    fn spawn(asset_server: &Res<AssetServer>, position: Vec2) -> AiShipBundle;
}


pub fn player_ship(spawn_position: Vec2, asset_server: Res<AssetServer>) -> ShipBundle {
    return ShipBundle {
        ship: Ship {
            speed: Vec2::new(0.5, 0.5),
            gun_offset: Vec2::new(1.0, -0.32),
        },
        scene_bundle: SceneBundle {
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

impl AiShipSpawn for DefaultEnemyShip {
    fn spawn(asset_server: &Res<AssetServer>, spawn_position: Vec2) -> AiShipBundle {
        return AiShipBundle{
            ship_bundle: ShipBundle {
                ship: Ship {
                    speed: Vec2::new(0.2, 0.2),
                    gun_offset: Vec2::new(1.0, 0.0),
                },
                scene_bundle: SceneBundle {
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
                health: Health { hp: 2 },
            },
            fuse_timer: FuseTime {
                timer: Timer::new(Duration::from_secs(1), true),
            },
        };

    }
}
