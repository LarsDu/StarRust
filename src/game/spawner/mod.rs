use bevy::{prelude::*, time::FixedTimestep};

use super::AiShipBundle;
use super::constants::*;
use super::super::*;


pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(spawn_bundles),
            );
    }
}

fn setup(
    commands: Commands
){
}


pub fn spawn<T>(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bundle_func: &dyn Fn(Vec2, Res<AssetServer>) -> dyn Bundle
) {
    commands.spawn(
        bundle_func(Vec2::new(25.0, 2.0), asset_server)
    );
}



fn spawn_bundles() {}
