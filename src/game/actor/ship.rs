use bevy::{prelude::*, utils::Duration};
use super::super::components::*;
use super::super::spawner::*;
use super::super::actor::bullet::BulletType;
use super::*;
use crate::game::AudioClipAssets;
use crate::game::SceneAssets;
use crate::game::{ALLY_HITMASK, ENEMY_HITMASK, SPAWN_LOCATIONS};

pub fn player_ship(spawn_position: Vec2, audio_clips: Res<AudioClipAssets>, models: Res<SceneAssets>) -> ActorBundle {
    return ActorBundle {
        actor: Actor {
            speed: Vec2::new(0.5, 0.5),
        },
        scene_bundle: StarRustSceneBundle {
            scene: models.default_player.clone(),
            transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                .with_scale(Vec3::splat(0.95))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
            ..default()
        },
        weapon: Weapon {
            firing_audio_clip: audio_clips.laser_shot_silenced.clone(),
            bullet_type: BulletType::Standard,
            offset: Vec2::new(1.0, -0.32),
        },
        collider: Collider {
            rect: Vec2::new(1.5, 1.5),
            damage: 1,
            hitmask: ALLY_HITMASK,
            ..default()
        },
        health: Health { 
            hp: 10,
            death_sound: audio_clips.light_explosion.clone(),
            damage_sound: audio_clips.light_pow.clone()
        },
        camera_shake_on_death: CameraShakeOnDeath { ..default() }
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
                    speed: Vec2::new(0.1, 0.1),
                },
                scene_bundle: StarRustSceneBundle {
                    scene: models.default_enemy.clone(),
                    transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 2.0)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
                    ..default()
                },
                collider: Collider {
                    rect: Vec2::new(1.5, 1.5),
                    damage: 1,
                    hitmask: ENEMY_HITMASK,
                    ..default()
                },
                health: Health { 
                    hp: 1,
                    death_sound: audio_clips.laser_shot_silenced.clone(),
                    damage_sound: audio_clips.no_sound.clone()
                },
                weapon: Weapon {
                    firing_audio_clip: audio_clips.laser_shot_silenced.clone(),
                    bullet_type: BulletType::StandardEnemy,
                    offset: Vec2::new(1.0, 0.0),
                },
                camera_shake_on_death: CameraShakeOnDeath { ..default() }
            },
            auto_fire: AutoFire {
                cooldown_timer: Timer::new(Duration::from_secs_f32(1.0), true),
            },
            death_points_awarded: DeathPointsAwarded { points: 20 },
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
