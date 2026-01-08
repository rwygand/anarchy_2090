use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod helpers;
mod systems;

use systems::*;
use components::TurnTimer;
use helpers::tiled::TiledMapPlugin;

pub struct AnarchyTwentyNinetyPlugin;

impl Plugin for AnarchyTwentyNinetyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TurnTimer>()
            .add_plugins(TilemapPlugin)
            .add_plugins(TiledMapPlugin)
            .add_systems(Startup, (
                camera::setup,
                map::load_map,
            ))
            .add_systems(Update, (
                player::spawn_player,
                monster::spawn_monsters,
                turn::tick,
                monster::monster_turn,
            ).before(movement::player_movement))
            .add_systems(Update, movement::player_movement.before(camera::follow_player))
            .add_systems(Update, camera::follow_player);
    }
}

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
        .add_plugins(AnarchyTwentyNinetyPlugin)
        .run();
}