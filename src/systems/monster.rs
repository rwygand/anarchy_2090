use crate::components::{
    BlocksMovement, FieldOfView, MapDimensions, Monster, Player, TickTimer, Visible,
};
use crate::helpers::grid_to_world_position;
use crate::map_builder::MapBuilder;
use bevy::log::info;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

pub fn spawn_monsters(
    mut commands: Commands,
    map: Res<MapDimensions>,
    map_builder: Res<MapBuilder>,
    monster_query: Query<&Monster>,
    player_query: Query<&TilePos, With<Player>>,
    blocking_query: Query<&TilePos, With<BlocksMovement>>,
) {
    if !monster_query.is_empty() {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let mut rng = rand::rng();
    let monster_count = rng.random_range(5..=10);
    let tile_size = map.tile_size;

    info!("Spawning {} monsters", monster_count);

    // Get list of rooms excluding first room (player's room)
    let available_rooms: Vec<_> = map_builder.rooms.iter().skip(1).collect();

    if available_rooms.is_empty() {
        warn!("No rooms available for monster spawning");
        return;
    }

    for _ in 0..monster_count {
        // Pick a random room
        let room = available_rooms[rng.random_range(0..available_rooms.len())];

        // available_rooms will never contain the player, we can skip player collision checks
        let monster_pos = TilePos {
            x: rng.random_range(room.x1 + 1..room.x2),
            y: rng.random_range(room.y1 + 1..room.y2),
        };

        let trans = grid_to_world_position(&monster_pos, &map, 10.0);

        let monster_type = rng.random_range(0..=1);
        let (glyph, color) = match monster_type {
            0 => ("o", Color::srgb(1.0, 0.0, 0.0)), // Orc
            _ => ("g", Color::srgb(0.0, 1.0, 0.0)), // Goblin
        };

        commands.spawn((
            Text2d::new(glyph),
            TextFont {
                font_size: tile_size,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(trans),
            Monster,
            monster_pos,
            BlocksMovement,
            FieldOfView::new(6),
            Visible::default(),
        ));
    }

    info!(
        "Spawned {} monsters across {} rooms",
        monster_count,
        available_rooms.len()
    );
}

pub fn monster_turn(
    mut monster_query: Query<(&mut TilePos, &mut Transform), (With<Monster>, Without<Player>)>,
    blocking_query: Query<&TilePos, (With<BlocksMovement>, Without<Monster>)>,
    turn_timer: Res<TickTimer>,
    map: Res<MapDimensions>,
) {
    // Only act when turn changes
    if !turn_timer.timer.just_finished() {
        return;
    }

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

            // Check for any blocking entity
            if blocking_query.iter().any(|pos| *pos == new_pos) {
                info!(
                    "Monster at ({}, {}) blocked at ({}, {})",
                    monster_pos.x, monster_pos.y, new_pos.x, new_pos.y
                );
                continue;
            }

            // Apply movement
            let new_trans = grid_to_world_position(&new_pos, &map, 10.0);

            *monster_pos = new_pos;
            transform.translation = new_trans;

            info!("Monster moved to ({}, {})", new_pos.x, new_pos.y);
        }
    }
}
