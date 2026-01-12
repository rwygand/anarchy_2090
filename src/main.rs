use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod fov;
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
                    visibility::mark_fov_dirty,
                    player::player_movement,
                    monster::monster_turn,
                    combat::melee_combat,
                    combat::apply_damage,
                    combat::delete_the_dead,
                    visibility::update_fov,
                    monster::monster_ai,
                    visibility::update_visibility,
                    camera::follow_player,
                )
                    .chain(),
            );
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
