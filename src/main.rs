use bevy::prelude::*;
mod game;
use game::GamePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::InGame)
        .add_plugin(GamePlugin)
        .run();
}

