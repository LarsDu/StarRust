use bevy::prelude::*;
//use bevy_hanabi::prelude::*;
mod menus;
use menus::MenuPlugin;

mod game;
use game::components::CameraShaker;
use game::ui::UiPlugin;
use game::background::BackgroundPlugin;
use game::audio::AudioPlugin;
use game::ai::AiPlugin;
use game::actor::ActorPlugin;
use game::walls::WallPlugin;
use game::player::PlayerPlugin;
use game::weapon::WeaponPlugin;
use game::collisions::CollisionPlugin;
use game::despawner::DespawnerPlugin;
use game::vfx::VfxPlugin;
use game::levels::LevelPlugin;

mod constants;
use constants::{CAMERA_FAR, SCREEN_HEIGHT, SCREEN_WIDTH};

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "StarRust".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(BackgroundPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(DespawnerPlugin)
        .add_plugin(VfxPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup_camera)
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
                priority: 0,
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
