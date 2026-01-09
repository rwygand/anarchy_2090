use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod helpers;
mod map_builder;
mod systems;

use components::TickTimer;
use systems::*;

pub struct AnarchyTwentyNinetyPlugin;

impl Plugin for AnarchyTwentyNinetyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickTimer>()
            .add_plugins(TilemapPlugin)
            .add_systems(Startup, (camera::setup, map::generate_map))
            .add_systems(
                Update,
                (
                    player::spawn_player,
                    monster::spawn_monsters,
                    timer::tick,
                    monster::monster_turn,
                )
                    .before(player::player_movement),
            )
            .add_systems(
                Update,
                player::player_movement.before(camera::follow_player),
            )
            .add_systems(Update, camera::follow_player);
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anarchy 2090".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AnarchyTwentyNinetyPlugin)
        .run();
}
