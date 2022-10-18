use bevy::prelude::*;
//use bevy_hanabi::prelude::HanabiPlugin;
// Game Plugin modules
mod ai;
pub use ai::*;

mod audio;
pub use audio::*;

mod constants;
pub use constants::*;

mod player;
pub use player::*;

mod enemy;
pub use enemy::*;

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
use despawner::*;

mod actor;
pub use actor::*;

mod background;
pub use background::*;

mod walls;
pub use walls::*;

mod spawner;
pub use spawner::*;

mod weapon;
pub use weapon::*;

mod vfx;
pub use vfx::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugin(HanabiPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(WallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(DespawnerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(VfxPlugin);
    }
}
