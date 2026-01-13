use crate::components::*;
use bevy::prelude::*;

pub fn melee_combat(
    mut commands: Commands,
    wants_melee_query: Query<(Entity, &WantsToMelee, &Stats)>,
    target_query: Query<&Stats, Without<WantsToMelee>>,
    mut suffer_damage_query: Query<&mut SufferDamage>,
) {
    for (attacker_entity, wants_melee, attacker_stats) in wants_melee_query.iter() {
        if let Ok(target_stats) = target_query.get(wants_melee.target) {
            let damage = (attacker_stats.attack - target_stats.defense).max(0);

            // Try to add damage to existing component, or insert new one
            if let Ok(mut suffer_damage) = suffer_damage_query.get_mut(wants_melee.target) {
                suffer_damage.add_damage(damage);
            } else {
                commands
                    .entity(wants_melee.target)
                    .insert(SufferDamage::new_damage(damage));
            }

            info!(
                "Queued {} damage to target (attack: {}, defense: {})",
                damage, attacker_stats.attack, target_stats.defense
            );
        }

        commands.entity(attacker_entity).remove::<WantsToMelee>();
    }
}

pub fn apply_damage(
    mut commands: Commands,
    mut damage_query: Query<(Entity, &mut Stats, &SufferDamage)>,
) {
    for (entity, mut stats, suffer_damage) in damage_query.iter_mut() {
        let total_damage = suffer_damage.total();
        stats.current_health -= total_damage;

        info!(
            "Applied {} total damage. Health now: {}",
            total_damage, stats.current_health
        );

        commands.entity(entity).remove::<SufferDamage>();
    }
}

pub fn delete_the_dead(
    mut commands: Commands,
    dead_query: Query<(Entity, &Stats, Option<&Player>, Option<&Monster>)>,
) {
    for (entity, stats, player, monster) in dead_query.iter() {
        if stats.health <= 0 {
            if player.is_some() {
                info!("Player has died! Health: {}", stats.health);
                // Don't despawn player - just log death
            } else if monster.is_some() {
                info!("Monster defeated!");
                commands.entity(entity).despawn();
            }
        }
    }
}
