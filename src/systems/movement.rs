use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{KeyCode, Query, Res, Transform, With};
use bevy_ecs_tilemap::prelude::*;
use crate::components::Player;
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut TilePos, &mut Transform), With<Player>>,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
) {
    let Ok((mut tile_pos, mut transform)) = player_query.single_mut() else {
        return;
    };

    let Ok(map_handle) = map_query.single() else {
        return;
    };

    let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
        return;
    };

    let map = &tiled_map.map;
    let max_x = map.width - 1;
    let max_y = map.height - 1;

    let mut moved = false;

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if tile_pos.y < max_y {
            tile_pos.y += 1;
            moved = true;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
        if tile_pos.y > 0 {
            tile_pos.y -= 1;
            moved = true;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        if tile_pos.x > 0 {
            tile_pos.x -= 1;
            moved = true;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
        if tile_pos.x < max_x {
            tile_pos.x += 1;
            moved = true;
        }
    }

    if moved {
        info!("Player moved to tile position: ({}, {})", tile_pos.x, tile_pos.y);

        transform.translation = grid_to_world_position(
            &tile_pos,
            transform.translation.z,
            map.tile_width as f32,
            map.tile_height as f32,
            &TilemapSize { x: map.width, y: map.height }
        );
    }
}
