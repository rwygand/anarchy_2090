use crate::components::MapDimensions;
use bevy::math::Vec3;
use bevy_ecs_tilemap::prelude::TilePos;

pub fn grid_to_world_position(tile_pos: &TilePos, map_dims: &MapDimensions, z: f32) -> Vec3 {
    let tile_size = map_dims.tile_size;
    // Calculate position with bottom-left origin
    let world_x = (tile_pos.x as f32 * tile_size) - (map_dims.width as f32 * tile_size) / 2.0;
    let world_y = (tile_pos.y as f32 * tile_size) - (map_dims.height as f32 * tile_size) / 2.0;
    Vec3::new(world_x, world_y, z)
}
