pub mod bullet;
pub mod ship;

use crate::AppState;

use crate::game::audio::AudioClipAssets;
use crate::game::components::*;
use crate::game::models::ModelsAssets;
use crate::utils::despawn_all;
use bevy::prelude::*;

pub trait BundledActor<T: Bundle> {
    fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        models: &Res<ModelsAssets>,
        spawn_pos: Vec2,
    ) -> T;
    fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        models: &Res<ModelsAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, models, spawn_pos));
    }
}

#[derive(Bundle, Clone, Default)]
pub struct StarRustSceneBundle {
    pub scene: Handle<Scene>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
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
pub struct PlayerActorBundle {
    pub player: Player,
    pub actor_bundle: ActorBundle,
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
        app.add_systems(OnEnter(AppState::InGame), despawn_all::<Actor>)
            .add_systems(OnExit(AppState::InGame), despawn_all::<Actor>);
    }
}
