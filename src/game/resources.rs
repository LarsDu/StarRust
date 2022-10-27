use bevy::prelude::*;

// Scoreboard resource
#[derive(Resource)]
pub struct Scoreboard {
    pub score: i32,
}

#[derive(Resource)]
struct HealthIndicator {
    hp: i8,
}
