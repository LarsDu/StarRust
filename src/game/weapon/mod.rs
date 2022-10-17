use bevy::{prelude::*, time::FixedTimestep};

use super::super::AppState;
use super::components::*;
use super::actor::bullet::*;
use super::events::WeaponFiredEvent;



pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_system(on_bullet_fired)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    //.with_system(move_bullets),
            );
    }
}


pub fn on_bullet_fired(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bullet_fired_events: EventReader<WeaponFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, &asset_server, event)
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    weapon_data: &WeaponFiredEvent,
) {
    commands.spawn(
        StandardBullet::get_bullet_bundle(asset_server, weapon_data)
    );
}

