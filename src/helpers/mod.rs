use bevy::math::Vec3;
use bevy_ecs_tilemap::prelude::TilePos;
use crate::helpers::tiled::TiledMap;

pub mod tiled;

pub fn grid_to_world_position(
    pos: &TilePos,
    z: f32,
    tiled_map: &TiledMap,
) -> Vec3 {
    let map = &tiled_map.map;
    let tile_width = map.tile_width as f32;
    let tile_height = map.tile_height as f32;

    let x = (pos.x as f32 - map.width as f32 / 2.0) * tile_width + tile_width / 2.0;
    let y = (pos.y as f32 - map.height as f32 / 2.0) * tile_height + tile_height / 2.0;
    Vec3::new(x, y, z)
}