use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}