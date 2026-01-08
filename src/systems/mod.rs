use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod camera;
pub mod map;
pub mod movement;
pub mod player;
pub mod monster;
pub mod turn;

pub fn grid_to_world_position(
    pos: &TilePos,
    z: f32,
    tile_width: f32,
    tile_height: f32,
    tilemap_size: &TilemapSize,
) -> Vec3 {
    let x = (pos.x as f32 - tilemap_size.x as f32 / 2.0) * tile_width + tile_width / 2.0;
    let y = (pos.y as f32 - tilemap_size.y as f32 / 2.0) * tile_height + tile_height / 2.0;
    Vec3::new(x, y, z)
}