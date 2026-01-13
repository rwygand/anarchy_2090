use bevy::prelude::*;

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
    pub count: u32,
}

impl Default for TickTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            count: 0,
        }
    }
}

#[derive(Resource)]
pub struct MapDimensions {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
}

impl Default for MapDimensions {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            tile_size: 16.0,
        }
    }
}

use bevy_ecs_tilemap::prelude::TilePos;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<TilePos>,
    pub range: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            is_dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct Explored(pub bool);

impl Default for Explored {
    fn default() -> Self {
        Self(false)
    }
}

// In src/components.rs
#[derive(Component)]
pub struct PlayerDetected {
    pub is_detected: bool,
    pub was_detected_last_frame: bool,
}

impl Default for PlayerDetected {
    fn default() -> Self {
        Self {
            is_detected: false,
            was_detected_last_frame: false,
        }
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Stats {
    // Core stats
    pub muscle: i32,
    pub brains: i32,
    pub skill: i32,
    pub cool: i32,
    pub fitness: i32,

    // TODO: These are temp stats that will eventually come from equipment or buffs
    pub attack: i32,
    pub defense: i32,

    // Derived stats
    pub health: i32,
    pub current_health: i32,
    pub stamina: i32,
    pub current_stamina: i32,
    pub load: i32,
    pub current_load: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            muscle: 1,
            brains: 1,
            skill: 1,
            cool: 1,
            fitness: 1,
            attack: 2,
            defense: 0,
            health: 10,
            current_health: 10,
            stamina: 10,
            current_stamina: 10,
            load: 10,
            current_load: 10,
        }
    }
}

#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Default)]
pub struct SufferDamage {
    pub amounts: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(amount: i32) -> Self {
        Self {
            amounts: vec![amount],
        }
    }

    pub fn add_damage(&mut self, amount: i32) {
        self.amounts.push(amount);
    }

    pub fn total(&self) -> i32 {
        self.amounts.iter().sum()
    }
}

#[derive(Component)]
pub struct InfoPanelBorder;

#[derive(Component)]
pub struct HealthLabel;

#[derive(Component)]
pub struct StaminaLabel;

#[derive(Component)]
pub struct LoadLabel;

#[derive(Component)]
pub struct StatDisplay {
    pub current: i32,
    pub max: i32,
}

impl StatDisplay {
    pub fn new(current: i32, max: i32) -> Self {
        Self { current, max }
    }

    pub fn format_values(&self) -> String {
        format!(" [ {} / {} ]", self.current, self.max)
    }
}

#[derive(Component)]
pub struct HealthStat;

#[derive(Component)]
pub struct StaminaStat;

#[derive(Component)]
pub struct LoadStat;

#[derive(Resource, Default)]
pub struct GameLog {
    pub messages: Vec<String>,
}

impl GameLog {
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
}

#[derive(Component)]
pub struct GameLogText;
