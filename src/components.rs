use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Resource)]
pub struct TurnTimer {
    pub timer: Timer,
    pub turn_number: u32,
}

impl Default for TurnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            turn_number: 0,
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
