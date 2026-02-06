use super::arena::BoidArena;
use super::spatial_grid::SpatialGrid;
use super::types::{BoidRole, BoidState};

/// Update states for all alive boids based on energy, neighbors, and threats
pub fn update_states<const CAP: usize, const CELL_CAP: usize>(
    arena: &mut BoidArena<CAP>,
    grid: &SpatialGrid<CELL_CAP>,
) {
    let mut neighbors = [0u16; 64];

    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }

        let pos = arena.positions[idx];
        let role = arena.roles[idx];
        let energy = arena.energy[idx];
        let sensor_radius = arena.genes[idx].sensor_radius;

        // Check for predators (Carnivores) in sensor range
        let mut has_predator = false;
        let neighbor_count = grid.query_neighbors(pos, sensor_radius, arena, idx, &mut neighbors);

        for &neighbor_idx in neighbors.iter().take(neighbor_count) {
            let other_idx = neighbor_idx as usize;
            if arena.roles[other_idx] == BoidRole::Carnivore && role != BoidRole::Carnivore {
                has_predator = true;
                break;
            }
        }

        // State transition logic
        if has_predator {
            arena.states[idx] = BoidState::Flee;
        } else if role == BoidRole::Carnivore {
            // Check for prey
            let mut has_prey = false;
            for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                let other_idx = neighbor_idx as usize;
                if arena.roles[other_idx] != BoidRole::Carnivore && arena.alive[other_idx] {
                    has_prey = true;
                    break;
                }
            }
            if has_prey {
                arena.states[idx] = BoidState::Hunt;
            } else if energy > 160.0 {
                arena.states[idx] = BoidState::Reproduce;
            } else {
                arena.states[idx] = BoidState::Wander;
            }
        } else if energy < 80.0 {
            // Low energy -> Forage
            arena.states[idx] = BoidState::Forage;
        } else if energy > 160.0 {
            arena.states[idx] = BoidState::Reproduce;
        } else {
            arena.states[idx] = BoidState::Wander;
        }
    }
}
