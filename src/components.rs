use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Resource)]
pub struct MapDimensions {
    pub width: i32,
    pub height: i32,
    pub tile_width: f32,
    pub tile_height: f32,
}