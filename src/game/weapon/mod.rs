use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::{prelude::*, time::FixedTimestep};

use super::super::AppState;
use super::actor::bullet::*;
use super::scene::GltfAssets;
use super::events::WeaponFiredEvent;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_system(on_bullet_fired)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    //.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}

pub fn on_bullet_fired(
    mut commands: Commands,
    models: Res<GltfAssets>,
    mut bullet_fired_events: EventReader<WeaponFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, &models, event)
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    models: &Res<GltfAssets>,
    weapon_data: &WeaponFiredEvent,
) {
    let bullet_bundle = match weapon_data.bullet_type {
        BulletType::StandardEnemy => {
            StandardEnemyBullet::get_bullet_bundle(models, weapon_data)
        },
        _ => StandardBullet::get_bullet_bundle(models, weapon_data),
    };
    commands.spawn((bullet_bundle, NotShadowCaster, NotShadowReceiver));
}
