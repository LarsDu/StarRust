/*
TODO:
    - game restart 
    - scoreboard
    - dotted line
    - WASM
*/


use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

/* -- CONSTANTS -- */
// SCREEN
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 640.0;

// PADDLES
const PADDLE_OFFSET: f32 = 80.0;
const LEFT_PADDLE_POS: Vec2 = Vec2::new(-SCREEN_WIDTH / 2.0 + PADDLE_OFFSET, 0.0);
const RIGHT_PADDLE_POS: Vec2 = Vec2::new(SCREEN_WIDTH / 2.0 - PADDLE_OFFSET, 0.0);
const PADDLE_DIMS: Vec2 = Vec2::new(WALL_THICKNESS, 60.0);

// PUCK
const PUCK_DIMS: Vec2 = Vec2::new(WALL_THICKNESS, WALL_THICKNESS);
const PUCK_SPAWN_POS: Vec2 = Vec2::new(0.0, 0.0);
const PUCK_SPEED: f32 = 200.0;
const INITIAL_PUCK_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// WALLS
const WALL_THICKNESS: f32 = 15.0;
const LEFT_RIGHT_WALL_DIMS: Vec2 = Vec2::new(WALL_THICKNESS, SCREEN_HEIGHT - WALL_THICKNESS);
const TOP_BOTTOM_WALL_DIMS: Vec2 = Vec2::new(SCREEN_WIDTH - WALL_THICKNESS, WALL_THICKNESS);

const LEFT_WALL_POS: Vec2 = Vec2::new(WALL_THICKNESS - SCREEN_WIDTH / 2.0, 0.0);
const RIGHT_WALL_POS: Vec2 = Vec2::new(SCREEN_WIDTH / 2.0 - WALL_THICKNESS, 0.0);
const TOP_WALL_POS: Vec2 = Vec2::new(0.0, SCREEN_HEIGHT / 2.0 - WALL_THICKNESS);
const BOTTOM_WALL_POS: Vec2 = Vec2::new(0.0, -SCREEN_HEIGHT / 2.0 + WALL_THICKNESS);

const TOP_BOUND: f32 = TOP_WALL_POS.y - WALL_THICKNESS / 2.0 - PADDLE_DIMS.y / 2.0;
const BOTTOM_BOUND: f32 = BOTTOM_WALL_POS.y + WALL_THICKNESS / 2.0 + PADDLE_DIMS.y / 2.0;

// UPDATE TICK
const TIME_STEP: f32 = 1.0 / 72.0;
const PLAYER_PADDLE_SPEED: f32 = 500.0;

// DIFFICULTY
const AI_PADDLE_BASE_SPEED: f32 = 250.0;
const DIFFICULTY: f32 = 1.0;

// COLORS
const BACKGROUND_COLOR: Color = Color::BLACK;
const WALL_COLOR: Color = Color::WHITE;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Single Page PONG".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_event::<CollisionEvent>()
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_left_paddle.before(check_collisions))
                .with_system(ai_move_right_paddle.before(check_collisions))
                .with_system(apply_velocity.before(check_collisions))
                .with_system(check_collisions)
                //.with_system(play_collision_sound.after(check_collisions))
                .with_system(apply_velocity),
        )
        .run();
}

// EVENTS
#[derive(Default)]
pub struct CollisionEvent;

// COMPONENTS
#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct RightPaddle;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Goal;

//#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

// SYSTEMS

// Startup Systems
fn setup(mut commands: Commands) {
    setup_camera(&mut commands);
    setup_walls(&mut commands);
    setup_paddles(&mut commands);
    setup_puck(&mut commands);
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
fn setup_walls(commands: &mut Commands) {
    commands
        .spawn_bundle(RectBundle::new(LEFT_WALL_POS, LEFT_RIGHT_WALL_DIMS))
        .insert(Goal);
    commands
        .spawn_bundle(RectBundle::new(RIGHT_WALL_POS, LEFT_RIGHT_WALL_DIMS))
        .insert(Goal);
    commands.spawn_bundle(RectBundle::new(TOP_WALL_POS, TOP_BOTTOM_WALL_DIMS));
    commands.spawn_bundle(RectBundle::new(BOTTOM_WALL_POS, TOP_BOTTOM_WALL_DIMS));
}

fn setup_paddles(commands: &mut Commands) {
    commands
        .spawn_bundle(RectBundle::new(LEFT_PADDLE_POS, PADDLE_DIMS))
        .insert(Paddle)
        .insert(LeftPaddle);
    commands
        .spawn_bundle(RectBundle::new(RIGHT_PADDLE_POS, PADDLE_DIMS))
        .insert(Paddle)
        .insert(RightPaddle);
}

fn setup_puck(commands: &mut Commands) {
    commands
        .spawn_bundle(RectBundle::new(PUCK_SPAWN_POS, PUCK_DIMS))
        .insert(Velocity(INITIAL_PUCK_DIRECTION.normalize() * PUCK_SPEED));
}

// Game Logic Systems
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), With<Velocity>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn move_left_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<LeftPaddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * PLAYER_PADDLE_SPEED * TIME_STEP;

    // Keep the paddle movement in bounds

    paddle_transform.translation.y = new_paddle_position.clamp(BOTTOM_BOUND, TOP_BOUND);
}

fn ai_move_right_paddle(
    mut paddle_query: Query<&mut Transform, With<RightPaddle>>,
    mut puck_query: Query<(&mut Transform, &Velocity), (With<Velocity>, Without<RightPaddle>)>,
) {
    let mut paddle_transform = paddle_query.single_mut();
    let (mut puck_transform, puck_velocity) = puck_query.single_mut();
    // Compute y intercept position of the puck
    let puck_direction = puck_velocity.normalize();
    let y_intercept: f32 = puck_transform.translation.y
        + puck_direction.y * (RIGHT_PADDLE_POS.x - puck_transform.translation.x)
            / (puck_direction.x);

    // Move paddle
    if y_intercept > paddle_transform.translation.y + PUCK_DIMS.y/2.0 {
        paddle_transform.translation.y = f32::min(
            paddle_transform.translation.y + AI_PADDLE_BASE_SPEED * TIME_STEP * DIFFICULTY,
            TOP_BOUND,
        );
    } else if y_intercept < paddle_transform.translation.y - PUCK_DIMS.y/2.0 {
        paddle_transform.translation.y = f32::max(
            paddle_transform.translation.y - AI_PADDLE_BASE_SPEED * TIME_STEP * DIFFICULTY,
            BOTTOM_BOUND,
        );
    }
}

fn setup_scoreboard() {}

fn check_collisions(
    mut collision_events: EventWriter<CollisionEvent>,
    mut mover_query: Query<(&mut Transform, &mut Velocity), With<Velocity>>,
    collider_query: Query<&Transform, (With<Collider>, Without<Velocity>)>,
) {
    for (mover_transform, mut mover_velocity) in &mut mover_query {
        for collider_transform in &collider_query {
            let collision = collide(
                mover_transform.translation,
                mover_transform.scale.truncate(),
                collider_transform.translation,
                collider_transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                collision_events.send_default();

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = mover_velocity.x > 0.0,
                    Collision::Right => reflect_x = mover_velocity.x < 0.0,
                    Collision::Top => reflect_y = mover_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = mover_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    mover_velocity.x = -mover_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    mover_velocity.y = -mover_velocity.y;
                }
            }
        }
    }
}

fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

// BUNDLES
#[derive(Bundle)]
struct RectBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl RectBundle {
    fn new(position: Vec2, size: Vec2) -> RectBundle {
        RectBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: size.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

