use bevy::{prelude::*, time::FixedTimestep};

use super::yard::DefaultEnemyShip;
use super::{AiShipBundle, ShipBundle};
use super::constants::*;
use super::super::*;

pub trait SpawnableBundle{
    fn spawn(commands: Commands, asset_server: Res<AssetServer>, position: Vec2);
}

pub struct SpawnSpec{
    frequency: f32,



}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup)
                .with_system(spawn_startup_bundles::<DefaultEnemyShip>)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            );
    }
}

fn setup(
    commands: Commands
){
}


pub fn spawn_startup_bundles<B: SpawnableBundle>(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    B::spawn(commands, asset_server, Vec2::new(23.0, 2.0));
}



fn spawn_bundles() {}
