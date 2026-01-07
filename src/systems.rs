use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::*;
use crate::helpers::tiled::{TiledMap, TiledMapBundle, TiledMapHandle};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(TiledMapBundle {
        tiled_map: TiledMapHandle(asset_server.load("isometric_map.tmx")),
        render_settings: TilemapRenderSettings {
            render_chunk_size: UVec2::new(2, 2),
            y_sort: false,
        },
        ..Default::default()
    });
}

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
    let player_pos = GridPosition {
        x: map.width as i32 / 2,
        y: map.height as i32 / 2
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

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.single(), camera_query.single_mut()) {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn grid_to_world_position(
    grid_pos: &GridPosition,
    zindex: f32,
    tile_width: f32,
    tile_height: f32,
    map_size: &TilemapSize,
) -> Vec3 {
    // Isometric coordinate conversion: diamond isometric
    let world_x = (grid_pos.x - grid_pos.y) as f32 * (tile_width / 2.0);
    let world_y = -(grid_pos.x + grid_pos.y) as f32 * (tile_height / 2.0);

    // Calculate the world position of the map's center grid point
    let center_grid_x = map_size.x as f32 / 2.0;
    let center_grid_y = map_size.y as f32 / 2.0;

    let center_world_x = (center_grid_x - center_grid_y) * (tile_width / 2.0);
    let center_world_y = -(center_grid_x + center_grid_y) * (tile_height / 2.0);

    Vec3::new(world_x - center_world_x, world_y - center_world_y, zindex)
}