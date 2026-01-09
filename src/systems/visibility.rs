use crate::components::{FieldOfView, MapDimensions, Monster, Player, Visible, Wall};
use crate::fov::compute_fov;
use crate::map_builder::MapBuilder;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn update_fov(
    mut query: Query<(&TilePos, &mut FieldOfView), Or<(With<Player>, Changed<TilePos>)>>,
    map_builder: Res<MapBuilder>,
    map_dims: Res<MapDimensions>,
) {
    for (pos, mut fov) in query.iter_mut() {
        if fov.is_dirty {
            fov.visible_tiles = compute_fov(
                &map_builder.tiles,
                map_dims.width,
                map_dims.height,
                pos,
                fov.range,
            );
            fov.is_dirty = false;
        }
    }
}

pub fn mark_fov_dirty(mut query: Query<&mut FieldOfView, Changed<TilePos>>) {
    for mut fov in query.iter_mut() {
        fov.is_dirty = true;
    }
}

pub fn update_visibility(
    player_query: Query<&FieldOfView, With<Player>>,
    mut entity_query: Query<
        (&TilePos, &mut Visible, &mut TextColor),
        Or<(With<Wall>, With<Monster>)>,
    >,
) {
    let Ok(player_fov) = player_query.single() else {
        return;
    };

    for (pos, mut visible, mut color) in entity_query.iter_mut() {
        let is_visible = player_fov.visible_tiles.contains(pos);
        visible.0 = is_visible;

        // Hide entities not in FOV
        if is_visible {
            color.0.set_alpha(1.0);
        } else {
            color.0.set_alpha(0.0);
        }
    }
}
