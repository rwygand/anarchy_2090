use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::*;

pub mod camera;
pub mod map;
pub mod movement;
pub mod player;

pub fn grid_to_world_position(
    grid_pos: &GridPosition,
    zindex: f32,
    tile_width: f32,
    tile_height: f32,
    map_size: &TilemapSize,
) -> Vec3 {
    // Isometric coordinate conversion: diamond isometric
    let world_x = (grid_pos.x - grid_pos.y) as f32 * (tile_width / 2.0);
    let world_y = -(grid_pos.x + grid_pos.y) as f32 * (tile_height / 2.0);

    // Calculate the world position of the map's center grid point
    let center_grid_x = map_size.x as f32 / 2.0;
    let center_grid_y = map_size.y as f32 / 2.0;

    let center_world_x = (center_grid_x - center_grid_y) * (tile_width / 2.0);
    let center_world_y = -(center_grid_x + center_grid_y) * (tile_height / 2.0);

    Vec3::new(world_x - center_world_x, world_y - center_world_y, zindex)
}