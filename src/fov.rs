use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use pathfinding::prelude::bfs_reach;
use std::collections::HashSet;

pub fn compute_fov(
    tiles: &[bool],
    width: u32,
    height: u32,
    origin: &TilePos,
    radius: u32,
) -> HashSet<TilePos> {
    let mut visible = HashSet::new();

    // Get all transparent tiles within radius
    let transparent_tiles: Vec<_> = bfs_reach((origin.x, origin.y), |&(x, y)| {
        [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .iter()
        .filter_map(|(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                let idx = (ny as u32 * width + nx as u32) as usize;
                if tiles.get(idx) == Some(&true) {
                    let dist_sq = (nx - origin.x as i32).pow(2) + (ny - origin.y as i32).pow(2);
                    if (dist_sq as f32).sqrt() <= radius as f32 {
                        return Some((nx as u32, ny as u32));
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
    })
    .collect();

    // Add all transparent tiles
    for (x, y) in &transparent_tiles {
        visible.insert(TilePos { x: *x, y: *y });
    }

    // Add walls adjacent to visible tiles
    for (x, y) in &transparent_tiles {
        for (dx, dy) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ] {
            let nx = *x as i32 + dx;
            let ny = *y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                let idx = (ny as u32 * width + nx as u32) as usize;
                if tiles.get(idx) == Some(&false) {
                    // Wall tile
                    visible.insert(TilePos {
                        x: nx as u32,
                        y: ny as u32,
                    });
                }
            }
        }
    }

    visible
}
