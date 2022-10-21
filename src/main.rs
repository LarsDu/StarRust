use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
//use bevy_hanabi::prelude::*;
mod menus;
use menus::MenuPlugin;

mod game;
use game::{components::CameraShaker, GamePlugin};

mod constants;
<<<<<<< HEAD
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
=======
use constants::{SCREEN_HEIGHT,SCREEN_WIDTH, CAMERA_FAR};

>>>>>>> larsdu/particlesystembundle

pub mod utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Menu,
    Paused,
}
fn main() {
    App::new()
<<<<<<< HEAD
    .add_state(AppState::Menu)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
=======
        //.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
>>>>>>> larsdu/particlesystembundle
        .insert_resource(WindowDescriptor {
            title: "StarRust".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
<<<<<<< HEAD
=======
        .add_state(AppState::Menu)
>>>>>>> larsdu/particlesystembundle
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    /*commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
    );*/
    // Bevy 2d camera is at Z=999.9
<<<<<<< HEAD
    commands
        .spawn(Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 0.05,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 999.0).looking_at(Vec3::ZERO, Vec3::Y),
            /*transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),*/
            ..default()
        })
        .insert(CameraShaker { ..default() });
=======
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {  ..default()},
        camera: Camera {priority: 1, ..default()},
        projection: Projection::Orthographic(OrthographicProjection {
            //far: -1000.0,
            //near: 0.0,
            scale: 0.05,
           ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-1.0).looking_at(Vec3::ZERO, Vec3::Y),
        /*transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),*/
        ..default()
    })
    //.insert(Camera2d{..default()})
    .insert(CameraShaker{..default()});
>>>>>>> larsdu/particlesystembundle
}
