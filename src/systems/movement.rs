use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::{Player, Monster};
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut TilePos, &mut Transform), (With<Player>, Without<Monster>)>,
    monster_query: Query<&TilePos, With<Monster>>,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
) {
    let Ok((mut player_pos, mut transform)) = player_query.single_mut() else {
        return;
    };

    let Ok(map_handle) = map_query.single() else {
        return;
    };

    let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
        return;
    };

    let map = &tiled_map.map;
    let mut new_pos = *player_pos;

    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        new_pos.y += 1;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
        new_pos.y = new_pos.y.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA) {
        new_pos.x = new_pos.x.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD) {
        new_pos.x += 1;
    }

    if new_pos.x >= map.width || new_pos.y >= map.height {
        return;
    }

    // Check for monster collision
    if monster_query.iter().any(|monster_pos| *monster_pos == new_pos) {
        info!("Player blocked by monster at ({}, {})", new_pos.x, new_pos.y);
        return;
    }

    if new_pos != *player_pos {
        let new_trans = grid_to_world_position(
            &new_pos,
            100.0,
            map.tile_width as f32,
            map.tile_height as f32,
            &TilemapSize { x: map.width, y: map.height }
        );
        *player_pos = new_pos;
        transform.translation = new_trans;
    }
}
