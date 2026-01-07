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
        tiled_map: TiledMapHandle(asset_server.load("map.tmx")),
        ..Default::default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_query: Query<&TiledMapHandle>,
    tiled_maps: Res<Assets<TiledMap>>,
    player_query: Query<&Player>,
    map_dimensions: Option<Res<MapDimensions>>,
) {
    if !player_query.is_empty() || map_dimensions.is_some() {
        return;
    }

    let Ok(map_handle) = map_query.single() else {
        info!("no map handle found");
        return;
    };

    let Some(tiled_map) = tiled_maps.get(&map_handle.0) else {
        info!("tiled map not yet loaded");
        return;
    };

    let map = &tiled_map.map;

    commands.insert_resource(MapDimensions {
        width: map.width as i32,
        height: map.height as i32,
        tile_width: map.tile_width as f32,
        tile_height: map.tile_height as f32,
    });

    // Start the player in the center of the map
    let start_grid_x = map.width as i32 / 2;
    let start_grid_y = map.height as i32 / 2;

    let player_pos = GridPosition { x: start_grid_x, y: start_grid_y };

    // Convert grid position to world position
    let tile_size = map.tile_width as f32; // height and width are the same for square tiles

    let trans = grid_to_world_position(&player_pos, 100.0, tile_size, &TilemapSize { x: map.width as u32, y: map.height as u32 });

    info!("Spawning player at grid ({}, {}) world pos ({})", start_grid_x, start_grid_y, trans);
    info!("Map dimensions: width {}, height {}", map.width, map.height);

    let player_handle = asset_server.load("player.png");

    commands.spawn((
        Sprite {
            image: player_handle,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_translation(trans),
        Player,
        GridPosition { x: start_grid_x, y: start_grid_y },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut GridPosition, &mut Transform), With<Player>>,
    map_dim: If<Res<MapDimensions>>,

) {
    if let Ok((mut position, mut transform)) = player_query.single_mut() {

            let mut new_x = position.x;
            let mut new_y = position.y;

            if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
                new_y += 1;
            }
            if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
                new_y -= 1;
            }
            if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                new_x -= 1;
            }
            if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
                new_x += 1;
            }

            if new_y != position.y || new_x != position.x {
                if new_x < map_dim.width && new_x >= 0 && new_y < map_dim.height && new_y >= 0 {
                    position.x = new_x;
                    position.y = new_y;
                    info!("Player moved to grid position: ({}, {})", position.x, position.y);
                } else {
                    info!("Movement out of bounds: attempted to move to ({}, {})", new_x, new_y);
                }

                let trans = grid_to_world_position(
                    &position,
                    transform.translation.z,
                    map_dim.tile_width,
                    &TilemapSize{ x: map_dim.width as u32, y: map_dim.height as u32 }
                );

                transform.translation = trans;
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
    tile_size: f32,
    map_size: &TilemapSize,
) -> Vec3 {
    // Convert grid coordinates to world coordinates
    // Assuming the tilemap is centered and uses the standard bevy_ecs_tilemap coordinate system
    let world_x = (grid_pos.x as f32 - map_size.x as f32 / 2.0) * tile_size + tile_size / 2.0;
    let world_y = (grid_pos.y as f32 - map_size.y as f32 / 2.0) * tile_size + tile_size / 2.0;

    Vec3::new(world_x, world_y, zindex)
}