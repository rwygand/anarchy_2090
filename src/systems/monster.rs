use crate::components::*;
use crate::helpers::grid_to_world_position;
use crate::map_builder::MapBuilder;
use bevy::log::info;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use pathfinding::prelude::astar;
use rand::Rng;
use std::collections::HashSet;

pub fn spawn_monsters(
    mut commands: Commands,
    map: Res<MapDimensions>,
    map_builder: Res<MapBuilder>,
    monster_query: Query<&Monster>,
) {
    if !monster_query.is_empty() {
        return;
    }

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
            Actor,
            Monster,
            monster_pos,
            BlocksMovement,
            FieldOfView::new(6),
            Visible::default(),
            PlayerDetected::default(),
            Stats::default(),
        ));
    }

    info!(
        "Spawned {} monsters across {} rooms",
        monster_count,
        available_rooms.len()
    );
}

pub fn monster_turn(
    mut commands: Commands,
    mut monster_query: Query<
        (
            Entity,
            &mut TilePos,
            &mut Transform,
            &PlayerDetected,
            &Stats,
        ),
        (With<Monster>, Without<Player>),
    >,
    blocking_query: Query<(Entity, &TilePos), (With<BlocksMovement>, Without<Monster>)>,
    player_query: Query<(Entity, &TilePos), With<Player>>,
    turn_timer: Res<TickTimer>,
    map: Res<MapDimensions>,
) {
    if !turn_timer.timer.just_finished() {
        return;
    }

    let Ok((player_entity, player_pos)) = player_query.single() else {
        return;
    };

    let mut rng = rand::rng();

    for (monster_entity, mut monster_pos, mut transform, detected, _stats) in
        monster_query.iter_mut()
    {
        // If player is detected, try to attack or move towards them
        if detected.0 {
            // Check if player is adjacent
            let dx = (player_pos.x as i32 - monster_pos.x as i32).abs();
            let dy = (player_pos.y as i32 - monster_pos.y as i32).abs();

            if dx <= 1 && dy <= 1 && (dx + dy) > 0 {
                // Attack the player
                commands.entity(monster_entity).insert(WantsToMelee {
                    target: player_entity,
                });
                info!(
                    "Monster at ({}, {}) attacks player!",
                    monster_pos.x, monster_pos.y
                );
                continue;
            }

            // Find path to player using A*
            let blocked_tiles: HashSet<_> = blocking_query
                .iter()
                .filter(|(entity, _)| *entity != monster_entity)
                .map(|(_, pos)| *pos)
                .collect();

            let result = astar(
                &*monster_pos,
                |pos| {
                    let mut neighbors = Vec::new();
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let new_x = pos.x as i32 + dx;
                            let new_y = pos.y as i32 + dy;

                            if new_x >= 0
                                && new_y >= 0
                                && new_x < map.width as i32
                                && new_y < map.height as i32
                            {
                                let neighbor = TilePos {
                                    x: new_x as u32,
                                    y: new_y as u32,
                                };
                                if !blocked_tiles.contains(&neighbor) || neighbor == *player_pos {
                                    neighbors.push((neighbor, 1));
                                }
                            }
                        }
                    }
                    neighbors
                },
                |pos| {
                    ((pos.x as i32 - player_pos.x as i32).abs()
                        + (pos.y as i32 - player_pos.y as i32).abs()) as u32
                },
                |pos| *pos == *player_pos,
            );

            if let Some((path, _)) = result {
                if path.len() > 1 {
                    let next_pos = path[1];
                    let new_trans = grid_to_world_position(&next_pos, &map, 10.0);
                    *monster_pos = next_pos;
                    transform.translation = new_trans;
                    info!(
                        "Monster chasing player, moved to ({}, {})",
                        next_pos.x, next_pos.y
                    );
                    continue;
                }
            }
        }

        // Random movement (existing code)
        if rng.random_bool(0.25) {
            let mut new_pos = *monster_pos;

            match rng.random_range(0..4) {
                0 => new_pos.y += 1,
                1 => new_pos.y = new_pos.y.saturating_sub(1),
                2 => new_pos.x = new_pos.x.saturating_sub(1),
                _ => new_pos.x += 1,
            }

            if new_pos.x >= map.width || new_pos.y >= map.height {
                continue;
            }

            if blocking_query.iter().any(|(_, pos)| *pos == new_pos) {
                continue;
            }

            let new_trans = grid_to_world_position(&new_pos, &map, 10.0);
            *monster_pos = new_pos;
            transform.translation = new_trans;
        }
    }
}

pub fn monster_ai(
    mut monster_query: Query<(&TilePos, &FieldOfView, &mut PlayerDetected), With<Monster>>,
    player_query: Query<&TilePos, With<Player>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (monster_pos, fov, mut detected) in monster_query.iter_mut() {
        let can_see = fov.visible_tiles.contains(player_pos);

        if can_see && !detected.0 {
            info!(
                "Monster at ({}, {}) spotted player at ({}, {})!",
                monster_pos.x, monster_pos.y, player_pos.x, player_pos.y
            );
        }

        detected.0 = can_see;
    }
}
