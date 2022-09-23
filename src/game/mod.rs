// Game Plugin module
mod constants;
pub use constants::*;

mod player;
pub use player::*;

mod bullet;
pub use bullet::*;

mod components;
pub use components::Player;



use super::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin);
    }
}
