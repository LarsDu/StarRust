use bevy::prelude::*;

use crate::AppState;
pub struct ScenePlugin;

#[derive(Resource)]
pub struct SceneAssets{
    pub default_player: Handle<Scene>,
    pub default_enemy: Handle<Scene>,
    pub jet_charger: Handle<Scene>,
    pub basic_boss: Handle<Scene>,
    pub default_bullet: Handle<Scene>,
    pub default_enemy_bullet: Handle<Scene>,
    pub asteroid: Handle<Scene>,
    pub space_platform: Handle<Scene>,
    pub turret_base: Handle<Scene>,
    pub turret_barrel: Handle<Scene>,
    pub powerup_star: Handle<Scene>,
    pub powerup_ico: Handle<Scene>

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
        jet_charger: asset_server.load("models/jet_charger.glb#Scene0"),
        basic_boss: asset_server.load("models/basic_boss.glb#Scene0"),
        default_bullet: asset_server.load("models/teal_bolt.glb#Scene0"),
        default_enemy_bullet: asset_server.load("models/red_bolt.glb#Scene0"),
        asteroid: asset_server.load("models/asteroid.glb#Scene0"),
        space_platform: asset_server.load("models/space_platform.glb#Scene0"),
        turret_base: asset_server.load("models/turret_base.glb#Scene0"),
        turret_barrel: asset_server.load("models/turret_barrel.glb#Scene0"),
        powerup_star: asset_server.load("models/powerup_star.glb#Scene0"),
        powerup_ico: asset_server.load("models/powerup_ico.glb#Scene0"),
    };
    commands.insert_resource(scene_assets);
}