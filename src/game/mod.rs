use bevy::{prelude::*, scene::ScenePlugin};
//use bevy_hanabi::prelude::HanabiPlugin; <-- No WASM support
//use bevy_particle_systems::ParticleSystemPlugin;

mod ai;
pub use ai::AiPlugin;

mod actor;
use actor::ActorPlugin;

mod audio;
pub use audio::*;

mod constants;
use constants::*;

mod player;
use player::*;

mod collisions;
pub use collisions::*;

mod health;
pub use health::*;

pub mod components;
pub use components::Player;

mod events;

mod resources;

mod ui;
use ui::UiPlugin;

mod despawner;
use despawner::DespawnerPlugin;

mod background;
pub use background::BackgroundPlugin;

mod walls;
pub use walls::WallPlugin;

mod levels;
pub use levels::LevelPlugin;

mod weapon;
pub use weapon::WeaponPlugin;

mod vfx;
pub use vfx::VfxPlugin;

pub mod models;
pub use models::ModelsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ModelsPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(WallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(DespawnerPlugin)
            .add_plugin(VfxPlugin)
            .add_plugin(ActorPlugin); // currently for cleaning up entities
    }
}
