use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
    pub count: u32,
}

impl Default for TickTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            count: 0,
        }
    }
}

#[derive(Resource)]
pub struct MapDimensions {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
}

impl Default for MapDimensions {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            tile_size: 16.0,
        }
    }
}
