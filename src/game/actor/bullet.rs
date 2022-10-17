use bevy::pbr::NotShadowCaster;
use bevy::{prelude::*, utils::Duration};

use super::super::components::*;
use super::super::spawner::*;
use super::super::events::WeaponFiredEvent;
use super::super::ai::AiMode;
use super::*;
use crate::game::{ALLY_HITMASK, ENEMY_HITMASK, SPAWN_LOCATIONS};

#[derive(Clone, Default)]
pub enum BulletType{
    #[default]
    Standard,
    StandardEnemy
}


#[derive(Bundle, Clone, Default)]
pub struct BulletActorBundle {
    pub actor: Actor,
    pub scene_bundle: StarRustSceneBundle,
    pub collider: Collider,
    pub ai: Ai,
    pub bullet: Bullet,
    pub timed_despawn: TimedDespawn,
}



pub trait AiBulletBundle {
    fn get_bullet_bundle(asset_server: &Res<AssetServer>, weapon_data: &WeaponFiredEvent) -> BulletActorBundle;
}

pub struct StandardBullet;

impl AiBulletBundle for StandardBullet {
    fn get_bullet_bundle(asset_server: &Res<AssetServer>, weapon_data: &WeaponFiredEvent) -> BulletActorBundle {
        return BulletActorBundle {
            actor: Actor {
                speed: Vec2::new(0.25, 0.25),
            },
            scene_bundle: StarRustSceneBundle {
                scene: asset_server.load("models/teal_bolt.glb#Scene0"),
                transform: Transform::from_xyz(weapon_data.translation.x, weapon_data.translation.y, 2.0)
                    .with_rotation(weapon_data.rotation),
                ..default()
            },
            collider: Collider {
                hitmask: weapon_data.hitmask,
                damage: 1,
                rect: Vec2::new(0.2, 0.8)
            },
            ai: Ai {
                mode: AiMode::CHARGE_FORWARD1,
                timer: Timer::default(),
            },
            bullet: Bullet {},
            timed_despawn: TimedDespawn { timer: Timer::from_seconds(2.0, false) }
        }
    }
}


pub struct StandardEnemyBullet;

impl AiBulletBundle for StandardEnemyBullet {
    fn get_bullet_bundle(asset_server: &Res<AssetServer>, weapon_data: &WeaponFiredEvent) -> BulletActorBundle {
        let mut bullet = StandardBullet::get_bullet_bundle(asset_server, weapon_data).clone();
        bullet.scene_bundle.scene = asset_server.load("models/red_bolt.glb#Scene0");
        return bullet;
    }
}