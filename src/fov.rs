use bevy_ecs_tilemap::prelude::*;
use std::collections::HashSet;

fn bresenham_line(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    points
}

pub fn compute_fov(
    tiles: &[bool],
    width: u32,
    height: u32,
    origin: &TilePos,
    radius: u32,
) -> HashSet<TilePos> {
    let mut visible = HashSet::new();
    let origin_i = (origin.x as i32, origin.y as i32);

    for y in 0..height {
        for x in 0..width {
            let target = (x as i32, y as i32);
            let dist_sq = (target.0 - origin_i.0).pow(2) + (target.1 - origin_i.1).pow(2);

            if (dist_sq as f32).sqrt() > radius as f32 {
                continue;
            }

            // Check line of sight using Bresenham's line algorithm
            let line = bresenham_line(origin_i, target);
            let mut blocked = false;

            for (lx, ly) in line.iter().skip(1) {
                if *lx == target.0 && *ly == target.1 {
                    break; // Reached target
                }
                let idx = (*ly as u32 * width + *lx as u32) as usize;
                if tiles.get(idx) == Some(&false) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                visible.insert(TilePos { x, y });
            }
        }
    }

    visible
}
