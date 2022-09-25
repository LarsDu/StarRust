pub mod yard;

use super::components::*;
use bevy::{prelude::*, time::Timer};

#[derive(Bundle)]
pub struct ShipBundle {
    ship: Ship,
    #[bundle]
    scene_bundle: SceneBundle,
    collider: Collider,
    health: Health,
}

/*
impl ShipBundle {
    fn new(
        ship: Ship,
        scene_bundle: SceneBundle,
        collider: Collider,
        health: Health,
    ) -> ShipBundle {
        ShipBundle {
            ship: ship,
            scene_bundle: scene_bundle,
            collider: collider,
            health: health
        }
    }
}
*/

#[derive(Bundle)]
pub struct AiShipBundle {
    #[bundle]
    ship_bundle: ShipBundle,
    fuse_timer: FuseTime,
}
