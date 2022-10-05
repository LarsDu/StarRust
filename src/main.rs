use bevy::{prelude::*, render::camera::*};

mod menus;
use menus::MenuPlugin;

mod game;
use game::GamePlugin;

mod constants;
use constants::{SCREEN_HEIGHT,SCREEN_WIDTH};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Menu,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "StarRust".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        
        .add_state(AppState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    /*commands.spawn(Camera2dBundle{
        ..default()
    }
    );*/
    // Bevy 2d camera is at Z=999.9
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.05,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 999.0).looking_at(Vec3::ZERO, Vec3::Y),
        /*transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),*/
        ..default()
    });
}
