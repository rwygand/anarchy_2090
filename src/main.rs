use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod helpers;
mod systems;

use helpers::tiled::TiledMapPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Isometric ARPG".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin)
        .add_systems(Startup, (load_map, setup_camera).chain())
        .add_systems(Update, (spawn_player, player_movement, camera_follow_player))
        .run();
}