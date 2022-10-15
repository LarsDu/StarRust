use bevy::{prelude::*, time::FixedTimestep};

use super::ship::yard::*;
use super::constants::*;
use super::components::*;
use super::ship::AiShipBundle;
use super::super::*;

pub mod levels;
use levels::*;


pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup_spawn_points)
                //.with_system(spawn_startup_bundles::<DefaultEnemyShip>)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            );
    }
}

fn setup_spawn_points(
    mut commands: Commands,
    asset_server: Res<AssetServer>

){

    // Note: Rust won't let me set this up 
    let spawn_locations: Vec<Vec2> = (BOTTOM_WALL as usize..TOP_WALL as usize)
        .step_by(4)
        .map(|y| Vec2::new(RIGHT_WALL, (y as f32) ))
        .collect::<Vec<Vec2>>();

    for spawn_location in spawn_locations.iter(){
        commands.spawn(
            Spawner{
                spawn_infos: SpawnSequence::level0(&asset_server, *spawn_location)
            }
        );
    }

}


/*pub fn spawn_startup_bundles<B: SpawnableBundle>(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    B::spawn(commands, asset_server, Vec2::new(23.0, 2.0));
}*/



fn spawn_bundles() {}
