use bevy::{prelude::*, time::FixedTimestep};

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

fn spawn_bundles() {}
