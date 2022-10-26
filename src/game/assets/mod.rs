use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};

use crate::AppState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .with_collection::<GltfAssets>()
                .with_collection::<AudioClipAssets>()
        );
    }
}

#[derive(AssetCollection)]
pub struct AudioClipAssets{
    #[asset(path = "")]
    pub no_sound: Handle<AudioSource>,
    #[asset(path = "audio/clips/laser_shot.ogg")]
    pub laser_shot: Handle<AudioSource>,
    #[asset(path = "audio/clips/laser_shot_silenced.ogg")]
    pub laser_shot_silenced: Handle<AudioSource>,
    #[asset(path = "audio/clips/light_pow.ogg")]
    pub light_pow: Handle<AudioSource>,
    #[asset(path = "audio/clips/light_explosion.ogg")]
    pub light_explosion: Handle<AudioSource>,
    #[asset(path = "audio/clips/collection1.ogg")]
    pub collection1: Handle<AudioSource>,
    #[asset(path = "audio/clips/point_counter.ogg")]
    pub point_counter: Handle<AudioSource>,
    #[asset(path = "audio/clips/salt_explosion.ogg")]
    pub salt_explosion: Handle<AudioSource>,
    #[asset(path = "audio/clips/sputter_rocket.ogg")]
    pub sputter_rocket: Handle<AudioSource>,
    #[asset(path = "audio/clips/coin_larry.ogg")]
    pub coin_larry: Handle<AudioSource>,
    #[asset(path = "audio/clips/event_slam.ogg")]
    pub event_slam: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct GltfAssets{
    #[asset(path = "models/basic_hero.glb#Scene0")]
    pub default_player: Handle<Scene>,
    #[asset(path = "models/basic_enemy.glb#Scene0")]
    pub default_enemy: Handle<Scene>,
    #[asset(path = "models/jet_charger.glb#Scene0")]
    pub jet_charger: Handle<Scene>,
    #[asset(path = "models/basic_boss.glb#Scene0")]
    pub basic_boss: Handle<Scene>,
    #[asset(path = "models/teal_bolt.glb#Scene0")]
    pub default_bullet: Handle<Scene>,
    #[asset(path = "models/red_bolt.glb#Scene0")]
    pub default_enemy_bullet: Handle<Scene>,
    #[asset(path = "models/asteroid.glb#Scene0")]
    pub asteroid: Handle<Scene>,
    #[asset(path = "models/space_platform.glb#Scene0")]
    pub space_platform: Handle<Scene>,
    #[asset(path = "models/turret_base.glb#Scene0")]
    pub turret_base: Handle<Scene>,
    #[asset(path = "models/turret_barrel1.glb#Scene0")]
    pub turret_barrel: Handle<Scene>,
    #[asset(path = "models/powerup_star.glb#Scene0")]
    pub powerup_star: Handle<Scene>,
    #[asset(path = "models/powerup_ico.glb#Scene0")]
    pub powerup_ico: Handle<Scene>

}



