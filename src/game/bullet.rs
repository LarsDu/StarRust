use bevy::{
    prelude::*,
    time::FixedTimestep,
};

use super::super::AppState;
use super::components::{Bullet, Collider};
use super::constants::BULLET_SPEED;

pub struct BulletFiredEvent {
    pub translation: Vec2,
    pub direction: Vec2,
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
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::TEAL.into()),
            transform: Transform::from_xyz(
                bullet_data.translation.x,
                bullet_data.translation.y,
                0.0,
            )
            .with_scale(Vec3 {
                x: 0.6,
                y: 0.12,
                z: 0.12,
            }),
            ..default()
        })
        .insert(Collider{ damage: 1, hitmask: bullet_data.hitmask})
        .insert(Bullet);
}

// BULLET SYSTEMS
fn move_bullets(mut query: Query<&mut Transform, With<Bullet>>) {
    for mut bullet_transform in &mut query {
        bullet_transform.translation.x -= BULLET_SPEED;
    }
}

fn check_collisions(mut commands: Commands) {}
