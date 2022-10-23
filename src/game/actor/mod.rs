pub mod bullet;
pub mod ship;

use crate::AppState;

use super::super::utils::despawn_all;
use super::ai::*;
use super::components::*;
use super::AudioClipAssets;
use super::SceneAssets;
use bevy::prelude::*;

pub trait BundledAsset<T: Bundle> {
    fn get_bundle(audio_clips: &Res<AudioClipAssets>, models: &Res<SceneAssets>) -> T;
    fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        models: &Res<SceneAssets>,
    ) {
        let bundle = Self::get_bundle(audio_clips, models);
        commands.spawn(bundle);
    }
}

#[derive(Bundle, Clone, Default)]
pub struct StarRustSceneBundle {
    pub scene: Handle<Scene>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Clone)]
pub struct ActorBundle {
    pub actor: Actor,
    pub scene_bundle: StarRustSceneBundle,
    pub collider: Collider,
    pub health: Health,
    pub weapon: Weapon,
    pub camera_shake_on_death: CameraShakeOnDeath,
}

#[derive(Bundle, Clone)]
pub struct AiActorBundle {
    pub actor_bundle: ActorBundle,
    pub ai: Ai,
    pub auto_fire: AutoFire,
    pub death_points_awarded: DeathPointsAwarded,
    pub timed_oob_despawn: TimedOobDespawn,
}

pub struct ActorPlugin;
impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(despawn_all::<Actor>))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(despawn_all::<Actor>));
    }
}
