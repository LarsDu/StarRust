use super::super::AppState;
use super::events::*;
use super::constants::*;
use bevy::{prelude::*, time::*};

pub struct EnemyPlugin;


// Plugin definition
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
        .add_event::<ScoreEvent>()
        .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            );
    }
}
