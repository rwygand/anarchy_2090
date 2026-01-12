use crate::components::*;
use bevy::prelude::*;

pub fn melee_combat(
    mut commands: Commands,
    wants_melee_query: Query<(Entity, &WantsToMelee, &Stats), With<WantsToMelee>>,
    mut target_query: Query<&mut Stats, Without<WantsToMelee>>,
) {
    for (attacker_entity, wants_melee, attacker_stats) in wants_melee_query.iter() {
        if let Ok(mut target_stats) = target_query.get_mut(wants_melee.target) {
            let damage = (attacker_stats.attack - target_stats.defense).max(0);
            target_stats.health -= damage;

            info!(
                "Attack dealt {} damage (attack: {}, defense: {}). Target health: {}",
                damage, attacker_stats.attack, target_stats.defense, target_stats.health
            );
        }

        // Remove the WantsToMelee component after processing
        commands.entity(attacker_entity).remove::<WantsToMelee>();
    }
}
