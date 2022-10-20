use bevy::prelude::*;

pub enum EmitterMode{
    Circle,
    Sphere,
}

#[derive(Component, Clone)]
pub struct BasicParticle{
    pub start_color: Color,
    pub end_color: Color,
    pub direction: Vec2,
    pub start_speed: Vec2,
    pub scale: f32,
    lifetime_timer: Timer,
}

impl Default for BasicParticle{
    fn default() -> Self {
        
    }
}

#[derive(Component, Clone)]
pub struct BasicParticleEmitter{
    pub mode: EmitterMode,
    frequency_timer: Timer,
    lifetime_timer: Timer
}
impl BasicParticleEmitter{
    fn new(&self, lifetime: f32, frequency: f32, mode: EmitterMode) -> Self{
        Self { 
            frequency_timer: Timer::from_seconds(frequency, true),
            lifetime_timer: Timer::from_seconds(lifetime, true),
            mode: mode
        }

    }
}
impl Default for BasicParticleEmitter{
    fn default() -> Self {
        Self::new(10.0, 0.05, EmitterMode::Circle);
    }
}