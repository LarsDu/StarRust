// Game Plugin module
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

mod ships;
pub use ships::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(CollisionPlugin);
    }
}
