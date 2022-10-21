use bevy::prelude::*;

// TIME
pub const TIME_STEP: f32 = 1.0/60.0;


// Models
pub const SCOREBOARD_FONT_SIZE: f32 = 20.0;
pub const SCOREBOARD_TEXT_PADDING: f32 = 5.0;
pub const UI_COLOR: Color = Color::Rgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};


pub const ALLY_HITMASK: u8 = 1;  // 0b0001
pub const ENEMY_HITMASK: u8 = 2; // 0b0010


// Wall Constants
pub const WALL_THICKNESS: f32 = 1.0;
// x coordinates_WALL
pub const LEFT_WALL: f32 = -25.;
pub const RIGHT_WALL: f32 = 28.;
// y coordinates
pub const BOTTOM_WALL: f32 = -18.;
pub const TOP_WALL: f32 = 18.;
pub const WALL_COLOR: Color = Color::Rgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 0.00,
};

pub const SPAWN_OFFSET: f32 = 8.0;
pub const RIGHT_SPAWN_X: f32 = RIGHT_WALL+SPAWN_OFFSET;
pub const SPAWN_LOCATIONS: [Vec2; 4] = [
    Vec2::new(RIGHT_SPAWN_X, 0.0),
    Vec2::new(RIGHT_SPAWN_X, 10.0),
    Vec2::new(RIGHT_SPAWN_X, -5.0),
    Vec2::new(RIGHT_SPAWN_X, 5.0)
];
