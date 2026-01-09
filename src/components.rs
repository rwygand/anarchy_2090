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

use bevy_ecs_tilemap::prelude::TilePos;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<TilePos>,
    pub range: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            is_dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct Explored(pub bool);

impl Default for Explored {
    fn default() -> Self {
        Self(false)
    }
}
