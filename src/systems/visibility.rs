use crate::components::{Explored, FieldOfView, MapDimensions, Monster, Player, Visible, Wall};
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
    mut wall_query: Query<(&TilePos, &mut Visible, &mut Explored, &mut TextColor), With<Wall>>,
    mut monster_query: Query<
        (&TilePos, &mut Visible, &mut TextColor),
        (With<Monster>, Without<Wall>),
    >,
) {
    let Ok(player_fov) = player_query.single() else {
        return;
    };

    // Update walls - mark as explored and adjust alpha
    for (pos, mut visible, mut explored, mut color) in wall_query.iter_mut() {
        let is_visible = player_fov.visible_tiles.contains(pos);
        visible.0 = is_visible;

        if is_visible {
            // In FOV - full visibility
            explored.0 = true;
            color.0.set_alpha(1.0);
        } else if explored.0 {
            // Previously seen but not in FOV - reduced visibility
            color.0.set_alpha(0.25);
        } else {
            // Never seen - hidden
            color.0.set_alpha(0.0);
        }
    }

    // Update monsters - only visible in current FOV
    for (pos, mut visible, mut color) in monster_query.iter_mut() {
        let is_visible = player_fov.visible_tiles.contains(pos);
        visible.0 = is_visible;

        if is_visible {
            color.0.set_alpha(1.0);
        } else {
            color.0.set_alpha(0.0);
        }
    }
}
