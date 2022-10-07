pub mod yard;

use super::components::*;
use bevy::{prelude::*, time::Timer};

#[derive(Bundle)]
pub struct ShipBundle {
    ship: Ship,
    scene_bundle: SceneBundle,
    collider: Collider,
    health: Health,
}

/*impl SpawnableBundle for ShipBundle{
    fn spawn(position: Vec2, asset_server: Res<AssetServer>) -> ShipBundle {
        ShipBundle {
            ship: ship,
            scene_bundle: scene_bundle,
            collider: collider,
            health: health
        }
    }

}*/

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
    ship_bundle: ShipBundle,
    fuse_timer: FuseTime,
}
