use bevy::asset::Assets;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;
use crate::components::{Monster, Player};
use crate::helpers::tiled::{TiledMap, TiledMapHandle};
use crate::systems::grid_to_world_position;

pub fn spawn_monsters(
    mut commands: Commands,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
    monster_query: Query<&Monster>,
    player_query: Query<&TilePos, With<Player>>,
) {
    if !monster_query.is_empty() {
        return;
    }

    let Ok(map_handle) = map_query.single() else {
        return;
    };

    let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
        return;
    };

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let map = &tiled_map.map;
    let mut rng = rand::rng();
    let monster_count = rng.random_range(3..=10);

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

        let trans = grid_to_world_position(
            &monster_pos,
            100.0,
            map.tile_width as f32,
            map.tile_height as f32,
            &TilemapSize { x: map.width, y: map.height }
        );

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_translation(trans)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            Monster,
            monster_pos,
        ));
    }
}
