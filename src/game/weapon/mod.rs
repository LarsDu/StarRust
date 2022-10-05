use bevy::{prelude::*, time::FixedTimestep};
use super::constants::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(
            SystemSet::new().with_run_criteria(
                FixedTimestep::step(TIME_STEP as f64)
            ).with_system(weapon_firing)
        );

    }

}

pub fn weapon_firing(){}