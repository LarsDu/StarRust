use bevy::{
    prelude::*,
    render::camera::*,
};
mod game;
use game::GamePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_demo_scene)
        .add_state(AppState::InGame)
        .add_plugin(GamePlugin)
        //.add_system_set(SystemSet::new().with_system(move_hero))
        .run();
}

fn setup_demo_scene(mut commands: Commands) {
    /*
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });*/
    // Point light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 400000.0,
            radius: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, -12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 16000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // camera

    commands.spawn_bundle(Camera3dBundle {
       projection: Projection::Orthographic(OrthographicProjection{scale: 0.05, ..default()}),
       transform: Transform::from_xyz(0.0, 0.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        /*transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),*/
        ..default()
    });


}
