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