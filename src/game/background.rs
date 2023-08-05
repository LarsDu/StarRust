use super::super::AppState;
use bevy::prelude::*;
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
           OnEnter(AppState::InGame), (setup_lights, setup_starfield)
        );
    }
}

pub fn setup_lights(mut commands: Commands) {
    // Point light
    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 30000.0,
            radius: 150000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });*/

    // Directional Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn setup_starfield(commands: Commands) {}
