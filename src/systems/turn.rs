use bevy::prelude::*;
use crate::components::TurnTimer;

pub fn tick(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
) {
    if turn_timer.timer.tick(time.delta()).just_finished() {
        turn_timer.turn_number += 1;
        info!("Turn {} begins", turn_timer.turn_number);

        // This is where you'd trigger turn-based events:
        // - Monster AI
        // - Status effects
        // - Resource regeneration
        // etc.
    }
}