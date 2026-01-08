use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{KeyCode, Query, Res, Transform, With};
use bevy_ecs_tilemap::map::TilemapSize;
use crate::components::{GridPosition, Player};
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut GridPosition, &mut Transform), With<Player>>,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
) {
    if let Ok((mut position, mut transform)) = player_query.single_mut() {
        let Ok(map_handle) = map_query.single() else {
            return;
        };

        let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
            return;
        };

        let map = &tiled_map.map;

        let mut new_x = position.x;
        let mut new_y = position.y;

        if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
            new_y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
            new_y += 1;
        }
        if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            new_x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
            new_x += 1;
        }

        if new_y != position.y || new_x != position.x {
            let max_x = map.width as i32;
            let max_y = map.height as i32;

            if new_x >= 0 && new_x < max_x && new_y >= 0 && new_y < max_y {
                position.x = new_x;
                position.y = new_y;
                info!("Player moved to grid position: ({}, {})", position.x, position.y);

                let trans = grid_to_world_position(
                    &position,
                    transform.translation.z,
                    map.tile_width as f32,
                    map.tile_height as f32,
                    &TilemapSize { x: map.width, y: map.height }
                );

                transform.translation = trans;
            } else {
                info!("Movement out of bounds: attempted to move to ({}, {}) (max: {}, {})",
                      new_x, new_y, max_x - 1, max_y - 1);
            }
        }
    }
}
