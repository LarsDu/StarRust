pub mod ship;

use super::components::*;
use super::ai::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    scene_bundle: SceneBundle,
    collider: Collider,
    health: Health,
}


#[derive(Bundle)]
pub struct AiActorBundle {
    actor_bundle: ActorBundle,
    ai: Ai,
    weapon_cooldown: WeaponCooldown,
}
