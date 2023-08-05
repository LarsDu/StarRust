use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use super::super::AppState;
use super::actor::bullet::*;
use super::events::WeaponFiredEvent;
use super::models::ModelsAssets;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>()
            .add_systems(Update, on_bullet_fired.run_if(in_state(AppState::InGame)));
    }
}

pub fn on_bullet_fired(
    mut commands: Commands,
    models: Res<ModelsAssets>,
    mut bullet_fired_events: EventReader<WeaponFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, &models, event)
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    models: &Res<ModelsAssets>,
    weapon_data: &WeaponFiredEvent,
) {
    let bullet_bundle = match weapon_data.bullet_type {
        BulletType::StandardEnemy => StandardEnemyBullet::get_bullet_bundle(models, weapon_data),
        _ => StandardBullet::get_bullet_bundle(models, weapon_data),
    };
    commands.spawn((bullet_bundle, NotShadowCaster, NotShadowReceiver));
}
