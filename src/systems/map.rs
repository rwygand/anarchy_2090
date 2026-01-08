use bevy::asset::AssetServer;
use bevy::math::UVec2;
use bevy::prelude::{Commands, Res};
use bevy_ecs_tilemap::map::TilemapRenderSettings;
use crate::helpers::tiled::{TiledMapBundle, TiledMapHandle};

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