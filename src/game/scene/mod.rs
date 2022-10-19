use bevy::prelude::*;

use crate::AppState;
pub struct ScenePlugin;

#[derive(Resource)]
pub struct SceneAssets{
    pub default_player: Handle<Scene>,
    pub default_enemy: Handle<Scene>,
    pub default_bullet: Handle<Scene>,
    pub default_enemy_bullet: Handle<Scene>

}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
        .add_system_set(SystemSet::on_enter(AppState::InGame));
    }
}

pub fn setup_resources(mut commands: Commands, asset_server: ResMut<AssetServer>){
    let scene_assets = SceneAssets{
        default_player: asset_server.load("models/basic_hero.glb#Scene0"),
        default_enemy: asset_server.load("models/basic_enemy.glb#Scene0"),
        default_bullet: asset_server.load("models/teal_bolt.glb#Scene0"),
        default_enemy_bullet: asset_server.load("models/red_bolt.glb#Scene0")
    };
    commands.insert_resource(scene_assets);
}