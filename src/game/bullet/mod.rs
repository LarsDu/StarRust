use bevy::{prelude::*, time::FixedTimestep};
pub mod variants;
use variants::*;

use super::super::AppState;
use super::components::{Bullet, Collider};
use super::constants::BULLET_SPEED;

pub struct BulletFiredEvent {
    pub translation: Vec2,
    pub rotation: Quat,
    pub hitmask: u8,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_system(on_bullet_fired)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(move_bullets),
            );
    }
}

#[derive(Bundle)]
pub struct BulletBundle{
    #[bundle]
    pbr_bundle: PbrBundle,
    collider: Collider,
    bullet: Bullet,
}

pub fn on_bullet_fired(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bullet_fired_events: EventReader<BulletFiredEvent>,
) {
    for event in bullet_fired_events.iter() {
        spawn_bullet(&mut commands, &mut meshes, &mut materials, event)
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    bullet_data: &BulletFiredEvent,
) {
    commands
        .spawn_bundle(standard_bullet(meshes, materials, bullet_data));
}

// BULLET SYSTEMS
fn move_bullets(mut query: Query<&mut Transform, With<Bullet>>) {
    for mut bullet_transform in &mut query {
        bullet_transform.translation =
            bullet_transform.translation + BULLET_SPEED * bullet_transform.forward();
    }
}
