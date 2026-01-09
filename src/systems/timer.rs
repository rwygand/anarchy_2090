use crate::components::TickTimer;
use bevy::prelude::*;

pub fn tick(time: Res<Time>, mut tick_timer: ResMut<TickTimer>) {
    if tick_timer.timer.tick(time.delta()).just_finished() {
        tick_timer.count += 1;
        info!("TickTimer count {} begins", tick_timer.count);

        // This is where you'd trigger turn-based events:
        // - Monster AI
        // - Status effects
        // - Resource regeneration
        // etc.
    }
}
