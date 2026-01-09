use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> TilePos {
        TilePos {
            x: (self.x1 + self.x2) / 2,
            y: (self.y1 + self.y2) / 2,
        }
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

#[derive(Resource)]
pub struct MapBuilder {
    pub width: u32,
    pub height: u32,
    pub rooms: Vec<Rect>,
    pub tiles: Vec<bool>, // true = floor, false = wall
}

impl MapBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            rooms: Vec::new(),
            tiles: vec![false; (width * height) as usize], // Start with all walls
        }
    }

    fn apply_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = true;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: u32, x2: u32, y: u32) {
        for x in x1.min(x2)..=x1.max(x2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = true;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: u32, y2: u32, x: u32) {
        for y in y1.min(y2)..=y1.max(y2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = true;
            }
        }
    }

    fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn is_floor(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.tiles[self.xy_idx(x, y)]
    }

    pub fn build_rooms_and_corridors(&mut self) {
        let mut rng = rand::rng();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.random_range(MIN_SIZE..=MAX_SIZE);
            let h = rng.random_range(MIN_SIZE..=MAX_SIZE);
            let x = rng.random_range(1..self.width - w - 1);
            let y = rng.random_range(1..self.height - h - 1);
            let new_room = Rect::new(x, y, w, h);

            let ok = !self.rooms.iter().any(|other| new_room.intersect(other));

            if ok {
                self.apply_room(&new_room);

                if !self.rooms.is_empty() {
                    let new_center = new_room.center();
                    let prev_center = self.rooms[self.rooms.len() - 1].center();

                    if rng.random_bool(0.5) {
                        self.apply_horizontal_tunnel(prev_center.x, new_center.x, prev_center.y);
                        self.apply_vertical_tunnel(prev_center.y, new_center.y, new_center.x);
                    } else {
                        self.apply_vertical_tunnel(prev_center.y, new_center.y, prev_center.x);
                        self.apply_horizontal_tunnel(prev_center.x, new_center.x, new_center.y);
                    }
                }

                self.rooms.push(new_room);
            }
        }
    }
}
