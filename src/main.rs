use bevy::prelude::*;
//use bevy_hanabi::prelude::*;
mod menus;
use menus::MenuPlugin;

mod game;
use game::{components::CameraShaker, GamePlugin};

mod constants;
use constants::{CAMERA_FAR, SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod utils;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
   #[default]
    InGame,
    Menu,
    Paused,
}
fn main() {
    App::new()
        //.add_state(AppState::Menu)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.27)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "StarRust".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
          }))
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    /*commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        camera: Camera {priority: 1, ..default()},
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
    ); // FIXME: this does not render at all*/
    // Bevy 2d camera is at Z=999.9
    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d { ..default() },
            camera: Camera {
                order: 0,
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR - 0.1)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraShaker { ..default() });
}
