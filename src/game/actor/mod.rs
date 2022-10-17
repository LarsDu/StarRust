pub mod ship;
pub mod bullet;

use super::components::*;
use super::ai::*;
use super::ai::autofire::*;
use bevy::prelude::*;

#[derive(Bundle, Clone, Default)]
pub struct StarRustSceneBundle{
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
}


#[derive(Bundle, Clone)]
pub struct AiActorBundle {
    pub actor_bundle: ActorBundle,
    pub ai: Ai,
    pub auto_fire: AutoFire,
    pub death_points_awarded: DeathPointsAwarded
}
