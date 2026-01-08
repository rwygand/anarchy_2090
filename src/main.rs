use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod helpers;
mod systems;

use systems::{player, map, camera, movement};

use helpers::tiled::TiledMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Anarchy 2090".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin)
        .add_systems(Startup, (map::load_map, camera::setup).chain())
        .add_systems(Update, (player::spawn_player, movement::player_movement, camera::follow_player))
        .run();
}