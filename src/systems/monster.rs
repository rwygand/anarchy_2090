use crate::components::{MapDimensions, Monster, Player, TurnTimer};
use crate::helpers::grid_to_world_position;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

pub fn spawn_monsters(
    mut commands: Commands,
    map: Res<MapDimensions>,
    monster_query: Query<&Monster>,
    player_query: Query<&TilePos, With<Player>>,
) {
    if !monster_query.is_empty() {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let mut rng = rand::rng();
    let monster_count = rng.random_range(3..=10);
    let tile_size = map.tile_size;

    info!("Spawning {} monsters", monster_count);

    for _ in 0..monster_count {
        let monster_pos = loop {
            let pos = TilePos {
                x: rng.random_range(0..map.width),
                y: rng.random_range(0..map.height),
            };

            let dx = pos.x.abs_diff(player_pos.x);
            let dy = pos.y.abs_diff(player_pos.y);

            if dx > 1 || dy > 1 {
                break pos;
            }
        };

        let trans = grid_to_world_position(&monster_pos, &map, 10.0);

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..default()
            },
            Transform::from_translation(trans),
            Monster,
            monster_pos,
        ));
    }
}

pub fn monster_turn(
    mut monster_query: Query<(&mut TilePos, &mut Transform), (With<Monster>, Without<Player>)>,
    player_query: Query<&TilePos, With<Player>>,
    turn_timer: Res<TurnTimer>,
    map: Res<MapDimensions>,
) {
    // Only act when turn changes
    if !turn_timer.timer.just_finished() {
        return;
    }

    let player_pos = player_query.single().ok();

    let mut rng = rand::rng();

    for (mut monster_pos, mut transform) in monster_query.iter_mut() {
        // 25% chance to attempt movement
        if rng.random_bool(0.25) {
            let mut new_pos = *monster_pos;

            // Random direction
            match rng.random_range(0..4) {
                0 => new_pos.y += 1,                          // Up
                1 => new_pos.y = new_pos.y.saturating_sub(1), // Down
                2 => new_pos.x = new_pos.x.saturating_sub(1), // Left
                _ => new_pos.x += 1,                          // Right
            }

            // Check bounds
            if new_pos.x >= map.width || new_pos.y >= map.height {
                info!(
                    "Monster at ({}, {}) attempted to move off map to ({}, {})",
                    monster_pos.x, monster_pos.y, new_pos.x, new_pos.y
                );
                continue;
            }

            // Check if player is at target position
            if let Some(p_pos) = player_pos {
                if *p_pos == new_pos {
                    info!(
                        "Monster at ({}, {}) blocked by player at ({}, {})",
                        monster_pos.x, monster_pos.y, new_pos.x, new_pos.y
                    );
                    continue;
                }
            }

            // Apply movement
            let new_trans = grid_to_world_position(&new_pos, &map, 10.0);

            *monster_pos = new_pos;
            transform.translation = new_trans;

            info!("Monster moved to ({}, {})", new_pos.x, new_pos.y);
        }
    }
}
