

use bevy::{
    prelude::*,
    render::camera::*,
};
use super::super::AppState;
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_lights)
                .with_system(setup_camera)
                .with_system(setup_starfield)
        );
    }
}

pub fn setup_lights(mut commands: Commands){
    // Point light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 400000.0,
            radius: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 16000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn setup_camera(mut commands: Commands){
    commands.spawn_bundle(Camera2dBundle{
        ..default()
    }
    );

    // Bevy 2d camera is at Z=999.9
    commands.spawn_bundle(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection{scale: 0.05, ..default()}),
        transform: Transform::from_xyz(0.0, 0.0, 999.0).looking_at(Vec3::ZERO, Vec3::Y),
        /*transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),*/
        ..default()
    });
}

pub fn setup_starfield(mut commands: Commands){

}