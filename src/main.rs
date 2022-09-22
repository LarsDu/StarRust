use bevy::prelude::*;

const MOVEMENT_SPEED_X: f32 = 0.10;
const MOVEMENT_SPEED_Y: f32 = 0.10;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.0,0.0,0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_demo_scene)
        .add_system_set(SystemSet::new().with_system(move_hero))
        .run();
}

#[derive(Component)]
struct Hero;

fn setup_demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = asset_server.load("models/basic_hero.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    let hero_transform = Transform::from_xyz(0.0, 0.0, 0.0)
        .with_scale(Vec3::splat(0.95))
        .with_rotation(Quat::from_rotation_y( std::f32::consts::PI * 0.5));
    commands
        .spawn_bundle(SceneBundle {
            scene: my_gltf,
            transform: hero_transform,
            ..Default::default()
        })
        .insert(Hero);

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    /*commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });*/
    // Point light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 400000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn_bundle(
        DirectionalLightBundle{
            directional_light: DirectionalLight { color: Color::OLIVE, ..default() },
            ..default()
        },
    );
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_hero(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Hero>>) {
    let mut hero_transform = query.single_mut();
    let mut direction_x: f32 = 0.0;
    let mut direction_y = 0.0;
    

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= MOVEMENT_SPEED_Y;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += MOVEMENT_SPEED_Y;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x += MOVEMENT_SPEED_X;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x -= MOVEMENT_SPEED_X;
    }

    // Calculate the new horizontal paddle position based on player input
    hero_transform.translation.y = hero_transform.translation.y + direction_y;
    hero_transform.translation.x = hero_transform.translation.x + direction_x;
}
