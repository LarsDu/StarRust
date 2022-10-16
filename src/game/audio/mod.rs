use super::super::AppState;
use super::events::*;
use super::constants::*;

//use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_event::<AudioEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}