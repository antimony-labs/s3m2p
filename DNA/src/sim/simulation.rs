use rand::Rng;

use super::arena::BoidArena;
use super::interactions::{process_predation, process_scavenging};
use super::spatial_grid::SpatialGrid;
use super::state_machine::update_states;
use super::types::BoidState;

pub struct SimConfig {
    pub carrying_capacity: usize,
    pub reproduction_threshold: f32,
    pub reproduction_cost: f32,
    pub max_age: f32,
    pub base_mortality: f32,
    pub starvation_threshold: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            carrying_capacity: 800,
            reproduction_threshold: 120.0, // Easier to reproduce
            reproduction_cost: 40.0,       // Cheaper reproduction
            max_age: 2000.0,               // Longer lifespan
            base_mortality: 0.00002,       // Much gentler base mortality
            starvation_threshold: 10.0,    // Only die when very low energy
        }
    }
}

/// Single simulation step - zero heap allocations
/// Now integrates state machine, predation, and scavenging
pub fn simulation_step<const CAP: usize, const CELL_CAP: usize>(
    arena: &mut BoidArena<CAP>,
    grid: &SpatialGrid<CELL_CAP>,
    config: &SimConfig,
    width: f32,
    height: f32,
    dt: f32,
) -> (usize, usize) {
    // returns (births, deaths)
    let mut rng = rand::thread_rng();
    let mut births = 0usize;
    let mut deaths = 0usize;
    let population = arena.alive_count;

    // Phase 0: Update states based on environment
    update_states(arena, grid);

    // Collect reproduction candidates first (to avoid borrowing issues)
    let mut reproduce_indices = [0u16; 128];
    let mut reproduce_count = 0;

    // Phase 1: Apply forces and update physics
    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }

        // Apply acceleration
        let accel = arena.scratch_accel[idx] * 0.05;
        arena.velocities[idx] += accel;

        // Limit speed (guard against zero division)
        let max_speed = arena.genes[idx].max_speed;
        let speed = arena.velocities[idx].length();
        if speed > max_speed && speed > 0.0001 {
            arena.velocities[idx] = arena.velocities[idx] / speed * max_speed;
        }

        // Update position
        arena.positions[idx] += arena.velocities[idx] * dt;

        // Wrap around
        if arena.positions[idx].x < 0.0 {
            arena.positions[idx].x += width;
        }
        if arena.positions[idx].x >= width {
            arena.positions[idx].x -= width;
        }
        if arena.positions[idx].y < 0.0 {
            arena.positions[idx].y += height;
        }
        if arena.positions[idx].y >= height {
            arena.positions[idx].y -= height;
        }

        // Metabolism (size affects cost)
        let size_cost = arena.genes[idx].size;
        let metabolism_cost = speed * 0.002 * arena.genes[idx].metabolism * size_cost;
        arena.energy[idx] -= metabolism_cost;

        // Aging
        arena.age[idx] += dt;

        // Check reproduction (only in Reproduce state and high energy)
        if arena.states[idx] == BoidState::Reproduce
            && arena.energy[idx] > config.reproduction_threshold
            && reproduce_count < 128
            && population + reproduce_count < config.carrying_capacity
        {
            reproduce_indices[reproduce_count] = idx as u16;
            reproduce_count += 1;
        }
    }

    // Phase 2: Process interactions (predation, scavenging)
    process_predation(arena);
    process_scavenging(arena);

    // Phase 3: Reproduction (separate pass to avoid borrow conflicts)
    for &parent_idx_u16 in reproduce_indices.iter().take(reproduce_count) {
        let parent_idx = parent_idx_u16 as usize;
        if arena.alive[parent_idx] && arena.energy[parent_idx] > config.reproduction_threshold {
            // Check for mate nearby
            let mut has_mate = false;
            let pos = arena.positions[parent_idx];
            let mut neighbors = [0u16; 64];
            let neighbor_count = grid.query_neighbors(pos, 30.0, arena, parent_idx, &mut neighbors);

            for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                let other_idx = neighbor_idx as usize;
                if arena.alive[other_idx]
                    && arena.roles[other_idx] == arena.roles[parent_idx]
                    && arena.states[other_idx] == BoidState::Reproduce
                    && other_idx != parent_idx
                {
                    has_mate = true;
                    break;
                }
            }

            if has_mate {
                let handle = arena.spawn_child(parent_idx);
                if handle.is_valid() {
                    births += 1;
                }
            }
        }
    }

    // Phase 4: Death checks
    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }

        let should_die =
            // Starvation - primary death cause
            arena.energy[idx] <= config.starvation_threshold ||
            // Old age guaranteed death
            arena.age[idx] > config.max_age ||
            // Carrying capacity pressure only when significantly over
            {
                let over_capacity = population > config.carrying_capacity;
                let pop_excess = if over_capacity {
                    (population - config.carrying_capacity) as f32 / config.carrying_capacity as f32
                } else { 0.0 };

                // Only apply population pressure, age mortality is gentle
                let death_prob = config.base_mortality + pop_excess * 0.02;

                rng.gen::<f32>() < death_prob
            };

        if should_die {
            arena.kill(idx);
            deaths += 1;
        }
    }

    (births, deaths)
}
