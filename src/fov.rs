use bevy_ecs_tilemap::prelude::TilePos;
use std::collections::HashSet;

pub fn compute_fov(
    tiles: &[bool],
    width: u32,
    height: u32,
    origin: &TilePos,
    range: i32,
) -> HashSet<TilePos> {
    let mut visible = HashSet::new();
    visible.insert(*origin);

    let range_squared = range * range;
    let ox = origin.x as i32;
    let oy = origin.y as i32;

    // Cast rays in all directions
    for angle in 0..360 {
        let rad = (angle as f32).to_radians();
        let dx = rad.cos();
        let dy = rad.sin();

        for step in 1..=range {
            let x = ox + (dx * step as f32).round() as i32;
            let y = oy + (dy * step as f32).round() as i32;

            // Check bounds
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                break;
            }

            let pos = TilePos {
                x: x as u32,
                y: y as u32,
            };

            // Check distance
            let dist_squared = (x - ox) * (x - ox) + (y - oy) * (y - oy);
            if dist_squared > range_squared {
                break;
            }

            visible.insert(pos);

            // Stop if we hit a wall
            let idx = (y * width as i32 + x) as usize;
            if !tiles[idx] {
                break;
            }
        }
    }

    visible
}
