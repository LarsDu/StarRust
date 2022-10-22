use std::time::Duration;

use super::super::actor::bullet::BulletType;
use super::super::components::*;
use super::super::levels::*;
use super::*;
use crate::game::constants::ASSET_SCALE;
use crate::game::player::spawn;
use crate::game::AudioClipAssets;
use crate::game::SceneAssets;
use crate::game::{ALLY_HITMASK, ENEMY_HITMASK, SPAWN_LOCATIONS};
use bevy::prelude::*;

pub fn player_ship(
    spawn_position: Vec2,
    audio_clips: Res<AudioClipAssets>,
    models: Res<SceneAssets>,
) -> ActorBundle {
    return ActorBundle {
        actor: Actor {
            speed: Vec2::new(8.0, 8.0),
        },
        scene_bundle: StarRustSceneBundle {
            scene: models.default_player.clone(),
            transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                .with_scale(Vec3::splat(ASSET_SCALE))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
            ..default()
        },
        weapon: Weapon::new(
            BulletType::Standard,
            Vec2::new(20.0, -10.0),
            audio_clips.laser_shot.clone(),
            0.15,
        ),
        collider: Collider {
            rect: Vec2::new(30.0, 30.0),
            damage: 1,
            hitmask: ALLY_HITMASK,
            ..default()
        },
        health: Health {
            hp: 20,
            death_sound: audio_clips.light_explosion.clone(),
            damage_sound: audio_clips.light_pow.clone(),
        },
        camera_shake_on_death: CameraShakeOnDeath { ..default() },
    };
}

pub struct DefaultEnemyShip;

impl BundledAsset for DefaultEnemyShip {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle {
        let spawn_position = SPAWN_LOCATIONS[0];
        return AiActorBundle {
            ai: Ai {
                mode: AiMode::ChargeForward1,
                timer: Timer::default(),
            },
            actor_bundle: ActorBundle {
                actor: Actor {
                    speed: Vec2::new(2.0, 2.0),
                },
                scene_bundle: StarRustSceneBundle {
                    scene: models.default_enemy.clone(),
                    transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                        .with_scale(Vec3::splat(23.0))
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
                    ..default()
                },
                collider: Collider {
                    rect: Vec2::new(35.0, 35.0),
                    damage: 1,
                    hitmask: ENEMY_HITMASK,
                    ..default()
                },
                health: Health {
                    hp: 1,
                    death_sound: audio_clips.light_explosion.clone(),
                    damage_sound: audio_clips.no_sound.clone(),
                },
                weapon: Weapon::new(
                    BulletType::StandardEnemy,
                    Vec2::new(20.0, 0.0),
                    audio_clips.laser_shot.clone(),
                    0.5,
                ),
                camera_shake_on_death: CameraShakeOnDeath { ..default() },
            },
            auto_fire: AutoFire {},
            death_points_awarded: DeathPointsAwarded { points: 20 }, //FIXME: Gets doubled
        };
    }
}

pub struct RaptorSineMovementVariant;

impl BundledAsset for RaptorSineMovementVariant {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle {
        let mut variant = DefaultEnemyShip::get_bundle(audio_clips, models).clone();
        variant.ai.mode = AiMode::Sinusoid1;
        return variant;
    }
}

pub struct JetCharger;

impl BundledAsset for JetCharger {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle {
        let mut variant = DefaultEnemyShip::get_bundle(audio_clips, models).clone();
        variant.actor_bundle.scene_bundle.scene = models.jet_charger.clone();
        variant.actor_bundle.actor.speed = Vec2::new(3.5, 3.5);
        variant.ai.mode = AiMode::ChargeForward1;
        // Disable Weapon
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_mode(TimerMode::Once);
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_duration(Duration::from_secs(300));
        return variant;
    }
}

pub struct SpacePlatformBare;

impl BundledAsset for SpacePlatformBare {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle {
        let mut variant = DefaultEnemyShip::get_bundle(audio_clips, models).clone();
        variant.actor_bundle.scene_bundle.scene = models.space_platform.clone();
        variant.actor_bundle.actor.speed = Vec2::new(2.0, 2.0);
        variant.actor_bundle.health.hp = 100;
        variant.actor_bundle.collider.rect = Vec2::new(100.0, 40.0);
        variant.ai.mode = AiMode::ChargeForward1;
        // Disable Weapon
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_mode(TimerMode::Once);
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_duration(Duration::from_secs(300));
        return variant;
    }
}

// FIXME: Replace with dedicated powerup system, bundles, and spawnpoints!!
pub struct Star;

impl BundledAsset for Star {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> AiActorBundle {
        let mut variant = DefaultEnemyShip::get_bundle(audio_clips, models).clone();
        variant.actor_bundle.scene_bundle.scene = models.powerup_star.clone();
        variant.actor_bundle.collider.damage = 0;
        variant.death_points_awarded.points = 1;
        variant.actor_bundle.actor.speed = Vec2::new(4.0, 4.0);
        variant.actor_bundle.health.hp = 1;
        variant.actor_bundle.health.death_sound = audio_clips.coin_larry.clone();
        variant.actor_bundle.collider.rect = Vec2::new(10.0, 10.0);
        variant.ai.mode = AiMode::ChargeForward1;
        // Disable Weapon
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_mode(TimerMode::Once);
        variant
            .actor_bundle
            .weapon
            .cooldown_timer
            .set_duration(Duration::from_secs(300));
        return variant;
    }
}
