use super::arena::BoidArena;
use super::types::{BoidRole, BoidState};

/// Process predation: Carnivores attack and consume other boids
pub fn process_predation<const CAP: usize>(arena: &mut BoidArena<CAP>) {
    const PREDATION_RADIUS: f32 = 15.0; // Close contact required
    const PREDATION_DAMAGE: f32 = 20.0;
    const PREDATION_GAIN: f32 = 30.0;

    for idx in 0..CAP {
        if !arena.alive[idx] || arena.roles[idx] != BoidRole::Carnivore {
            continue;
        }

        let pos = arena.positions[idx];

        // Find nearby prey
        for other_idx in 0..CAP {
            if !arena.alive[other_idx] || other_idx == idx {
                continue;
            }

            // Carnivores don't eat other carnivores
            if arena.roles[other_idx] == BoidRole::Carnivore {
                continue;
            }

            let other_pos = arena.positions[other_idx];
            let dist_sq = pos.distance_squared(other_pos);

            if dist_sq < PREDATION_RADIUS * PREDATION_RADIUS {
                // Attack!
                let damage = PREDATION_DAMAGE * arena.genes[idx].strength;
                arena.energy[other_idx] -= damage;

                // Carnivore gains energy
                arena.energy[idx] = (arena.energy[idx] + PREDATION_GAIN).min(200.0);

                // Kill prey if energy depleted
                if arena.energy[other_idx] <= 0.0 {
                    arena.kill(other_idx);
                }
            }
        }
    }
}

/// Process scavenging: Scavengers consume dead boids
pub fn process_scavenging<const CAP: usize>(arena: &mut BoidArena<CAP>) {
    const SCAVENGING_RADIUS: f32 = 20.0;
    const SCAVENGING_GAIN: f32 = 15.0;

    for idx in 0..CAP {
        if !arena.alive[idx] || arena.roles[idx] != BoidRole::Scavenger {
            continue;
        }

        let pos = arena.positions[idx];

        // Find dead boids nearby
        for other_idx in 0..CAP {
            if arena.alive[other_idx] || arena.states[other_idx] != BoidState::Dead {
                continue;
            }

            let other_pos = arena.positions[other_idx];
            let dist_sq = pos.distance_squared(other_pos);

            if dist_sq < SCAVENGING_RADIUS * SCAVENGING_RADIUS {
                // Consume corpse
                arena.energy[idx] = (arena.energy[idx] + SCAVENGING_GAIN).min(200.0);
                // Remove corpse (fully free the slot)
                arena.states[other_idx] = BoidState::Wander;
            }
        }
    }
}
