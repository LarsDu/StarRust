use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::collisions::CollisionEvent;
use super::components::{Enemy, AutoFire, Health, Actor};
use super::constants::*;
//use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*, utils::Duration};
use std::f32::consts::PI;

pub struct EnemyPlugin;


// Plugin definition
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            );
    }
}
