use bevy::{prelude::*, window::WindowResolution};
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
    InGame,
    #[default]
    Menu,
    Paused,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.27)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "StarRust".to_string(),
                resolution: WindowResolution::new(
                    SCREEN_WIDTH as u32,
                    SCREEN_HEIGHT as u32,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GamePlugin, MenuPlugin))
        .init_state::<AppState>() //https://github.com/bevyengine/bevy/issues/14151
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
        .spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
            Projection::from(OrthographicProjection {
                scale: 1.0,
                far: CAMERA_FAR,
                ..OrthographicProjection::default_3d()
            }),
            Transform::from_xyz(0.0, 0.0, CAMERA_FAR - 0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(CameraShaker { ..default() });
}
