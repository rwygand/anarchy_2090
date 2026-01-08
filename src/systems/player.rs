use bevy::asset::{AssetServer, Assets};
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Res, Sprite, Transform};
use bevy_ecs_tilemap::prelude::*;
use crate::components::Player;
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

    // Start at the center of the map
    let player_pos = TilePos {
        x: map.width / 2,
        y: map.height / 2
    };

    let trans = grid_to_world_position(
        &player_pos,
        100.0,
        map.tile_width as f32,
        map.tile_height as f32,
        &TilemapSize { x: map.width, y: map.height }
    );

    info!("Spawning player at grid ({}, {}) world pos ({}, {})", player_pos.x, player_pos.y, trans.x, trans.y);
    info!("Map dimensions: width {}, height {}", map.width, map.height);

    let player_handle = asset_server.load("isometric_player.png");

    commands.spawn((
        Sprite {
            image: player_handle,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_translation(trans),
        Player,
        player_pos,
    ));
}