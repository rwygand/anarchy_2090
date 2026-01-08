use crate::components::{MapDimensions, Player};
use crate::helpers::grid_to_world_position;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn spawn_player(mut commands: Commands, player_query: Query<&Player>, map: Res<MapDimensions>) {
    if !player_query.is_empty() {
        return;
    }

    let tile_size = map.tile_size;

    // Start at the center of the map
    let player_pos = TilePos {
        x: map.width / 2,
        y: map.height / 2,
    };

    let world_pos = grid_to_world_position(&player_pos, &map, 10.0);

    info!(
        "Spawning player at grid ({}, {}) world pos ({}, {})",
        player_pos.x, player_pos.y, world_pos.x, world_pos.y
    );
    info!("Map dimensions: width {}, height {}", map.width, map.height);

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..default()
        },
        Transform::from_translation(world_pos),
        Player,
        player_pos,
    ));
}
