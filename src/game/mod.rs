use bevy::prelude::*;

// Game Plugin modules
mod ai;
pub use ai::*;

mod constants;
pub use constants::*;

mod player;
pub use player::*;

mod enemy;
pub use enemy::*;

mod bullet;
pub use bullet::*;

mod collisions;
pub use collisions::*;

mod health;
pub use health::*;

mod components;
pub use components::Player;

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BackgroundPlugin)
            .add_plugin(WallPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(SpawnerPlugin);
    }
}
