use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::Player;
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn spawn_player(
    mut commands: Commands,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
    player_query: Query<&Player>,
) {
    if !player_query.is_empty() {
        return;
    }

    let Ok(map_handle) = map_query.single() else {
        return;
    };

    let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
        return;
    };

    let map = &tiled_map.map;
    let tile_size = map.tile_width as f32;
    let half_tile = tile_size / 2.0;

    // Start at the center of the map
    let player_pos = TilePos {
        x: map.width / 2,
        y: map.height / 2
    };

    let world_pos = grid_to_world_position(&player_pos, 10.0, tiled_map);

    info!("Spawning player at grid ({}, {}) world pos ({}, {})", player_pos.x, player_pos.y, world_pos.x, world_pos.y);
    info!("Map dimensions: width {}, height {}", map.width, map.height);

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(half_tile, half_tile)),
            ..default()
        },
        Transform::from_translation(world_pos),
        Player,
        player_pos,
    ));
}