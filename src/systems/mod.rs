use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod camera;
pub mod map;
pub mod movement;
pub mod player;
pub mod monster;
pub mod turn;

pub fn grid_to_world_position(
    tile_pos: &TilePos,
    zindex: f32,
    tile_width: f32,
    tile_height: f32,
    map_size: &TilemapSize,
) -> Vec3 {
    let world_x = (tile_pos.x as f32 - tile_pos.y as f32) * (tile_width / 2.0);
    let world_y = -((tile_pos.x + tile_pos.y) as f32) * (tile_height / 2.0);

    let center_x = (map_size.x as f32 - map_size.y as f32) / 2.0 * (tile_width / 2.0);
    let center_y = -((map_size.x + map_size.y) as f32 / 2.0) * (tile_height / 2.0);

    Vec3::new(world_x - center_x, world_y - center_y, zindex)
}