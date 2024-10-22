use bevy::prelude::*;
//use bevy_hanabi::prelude::HanabiPlugin; <-- TODO: Add this back in

mod ai;
pub use ai::AiPlugin;

mod actor;
use actor::ActorPlugin;

mod audio;
pub use audio::*;

mod constants;
use bevy_hanabi::HanabiPlugin;
use constants::*;

mod player;
use player::*;

mod collisions;
pub use collisions::*;

mod health;

pub mod components;

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
        app.add_plugins((
            HanabiPlugin,
            ModelsPlugin,
            BackgroundPlugin,
            UiPlugin,
            AiPlugin,
            AudioPlugin,
            WallPlugin,
            PlayerPlugin,
            WeaponPlugin,
            CollisionPlugin,
            LevelPlugin,
            DespawnerPlugin,
            VfxPlugin,
            ActorPlugin,
        )); // currently for cleaning up entities
    }
}
