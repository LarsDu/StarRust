use bevy::prelude::*;
//use bevy_hanabi::prelude::*;
mod menus;
use menus::MenuPlugin;

mod game;
use game::{GamePlugin,components::CameraShaker};

mod constants;
use constants::{SCREEN_HEIGHT,SCREEN_WIDTH, CAMERA_FAR};


pub mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Menu,
    Paused,
}
fn main() {
    App::new()
    .add_state(AppState::Menu)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.27)))
        .insert_resource(WindowDescriptor {
            title: "StarRust".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        camera: Camera {priority: 0, ..default()},
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
    );
    // Bevy 2d camera is at Z=999.9
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {  ..default()},
        camera: Camera {priority: 1, ..default()},
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
           ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(CameraShaker{..default()});
}
