// Ripped straight from the breakout example https://bevyengine.org/examples/games/breakout/

use bevy::prelude::*;
use crate::utils::despawn_all;

use super::super::AppState;
use super::components::{Collider, Wall};
use super::constants::*;
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_walls)
            .add_systems(OnExit(AppState::InGame), despawn_all::<Wall>);
    }
}

fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(WallBundle::new(
        WallLocation::Left,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Right,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Bottom,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Top,
        &mut meshes,
        &mut materials,
    ));
}

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    //#[bundle]
    //sprite_bundle: SpriteBundle,
    pbr_bundle: PbrBundle,
    collider: Collider,
    wall: Wall,
}
impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(
        location: WallLocation,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> WallBundle {
        WallBundle {
            /*
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },*/
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {
                    half_size: location.size().extend(1.0) / 2.0,
                })),
                material: materials.add(WALL_COLOR),
                transform: Transform::from_translation(location.position().extend(1.0))
                    .with_scale(location.size().extend(1.0)),
                ..default()
            },
            collider: Collider {
                rect: location.size(),
                damage: 0,
                hitmask: 0,
            },
            wall: Wall,
        }
    }
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
