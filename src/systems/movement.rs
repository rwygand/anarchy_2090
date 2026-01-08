use crate::components::{MapDimensions, Monster, Player};
use crate::helpers::grid_to_world_position;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut TilePos, &mut Transform), (With<Player>, Without<Monster>)>,
    monster_query: Query<&TilePos, With<Monster>>,
    map: Res<MapDimensions>,
) {
    let Ok((mut player_pos, mut transform)) = player_query.single_mut() else {
        return;
    };

    let mut new_pos = *player_pos;

    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        new_pos.y += 1;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        new_pos.y = new_pos.y.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::KeyA)
    {
        new_pos.x = new_pos.x.saturating_sub(1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD)
    {
        new_pos.x += 1;
    }

    if new_pos.x >= map.width || new_pos.y >= map.height {
        return;
    }

    // Check for monster collision
    if monster_query
        .iter()
        .any(|monster_pos| *monster_pos == new_pos)
    {
        info!(
            "Player blocked by monster at ({}, {})",
            new_pos.x, new_pos.y
        );
        return;
    }

    if new_pos != *player_pos {
        let new_trans = grid_to_world_position(&new_pos, &map, 10.0);
        *player_pos = new_pos;
        transform.translation = new_trans;
    }
}
