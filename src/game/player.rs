use super::super::AppState;
use super::bullet::BulletFiredEvent;
use super::components::Player;
use super::constants::*;
use bevy::{prelude::*, time::FixedTimestep};

pub struct PlayerPlugin;

// Plugin definition
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                    .with_system(player_controller)
                    .with_system(fire_controller)

            );
    }
}

// SYSTEMS

// Player spawner system
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let player_model = asset_server.load("models/basic_hero.glb#Scene0");

    commands
        .spawn_bundle(SceneBundle {
            scene: player_model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(0.95))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.5)),
            ..Default::default()
        })
        .insert(Player {
            speed: PLAYER_SPEED,
            gun_offset: PLAYER_GUN_OFFSET,
            hp: 5,
        });
}

// Player controller system
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), With<Player>>,
) {
    for (mut player_transform, player) in &mut query {
        let mut direction_x: f32 = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= player.speed.x;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += player.speed.y;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            direction_x += player.speed.x;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x -= player.speed.x;
        }

        // Calculate the new horizontal paddle position based on player input
        player_transform.translation.y = player_transform.translation.y + direction_y;
        player_transform.translation.x = player_transform.translation.x + direction_x;
    }
}
// Fire controller system
pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut bullet_fired_event: EventWriter<BulletFiredEvent>,
    query: Query<(&Transform, &Player), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (player_transform, player) in &query {
            let event = BulletFiredEvent {
                translation: Vec2::new(
                    player_transform.translation.x + player.gun_offset.x,
                    player_transform.translation.y + player.gun_offset.y,
                ),
                direction: player_transform.forward().truncate(),
                hitmask: 2,
            };
            bullet_fired_event.send(event);
        }
    }
}
