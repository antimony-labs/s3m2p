use glam::Vec2;

use super::arena::BoidArena;
use super::spatial_grid::SpatialGrid;
use super::types::{BoidRole, BoidState, Obstacle};

/// Calculate flocking forces using spatial grid, writes to arena's scratch buffer
/// Now state-aware: different behaviors for Hunt, Flee, Forage, etc.
pub fn compute_flocking_forces<const CAP: usize, const CELL_CAP: usize>(
    arena: &mut BoidArena<CAP>,
    grid: &SpatialGrid<CELL_CAP>,
    vision_radius: f32,
    obstacles: &[Obstacle],
) {
    let mut neighbors = [0u16; 64]; // Stack-allocated neighbor buffer

    for idx in 0..CAP {
        if !arena.alive[idx] {
            arena.scratch_accel[idx] = Vec2::ZERO;
            continue;
        }

        let pos = arena.positions[idx];
        let state = arena.states[idx];
        let role = arena.roles[idx];
        let sensor_radius = arena.genes[idx].sensor_radius;
        let effective_radius = sensor_radius.max(vision_radius);

        let neighbor_count =
            grid.query_neighbors(pos, effective_radius, arena, idx, &mut neighbors);

        // Store density for population dynamics
        arena.scratch_density[idx] = neighbor_count.min(255) as u8;

        let mut force = Vec2::ZERO;

        match state {
            BoidState::Flee => {
                // Flee from predators
                let mut flee_force = Vec2::ZERO;
                for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                    let other_idx = neighbor_idx as usize;
                    if arena.roles[other_idx] == BoidRole::Carnivore && arena.alive[other_idx] {
                        let diff = pos - arena.positions[other_idx];
                        let dist = diff.length();
                        if dist > 0.001 {
                            flee_force += (diff / dist) * (100.0 / dist);
                        }
                    }
                }
                force += flee_force * 3.0; // Strong flee response
            }
            BoidState::Hunt => {
                // Seek nearest prey
                let mut seek_force = Vec2::ZERO;
                let mut closest_dist = f32::MAX;
                let mut closest_prey = None;

                for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                    let other_idx = neighbor_idx as usize;
                    if arena.roles[other_idx] != BoidRole::Carnivore && arena.alive[other_idx] {
                        let diff = arena.positions[other_idx] - pos;
                        let dist = diff.length();
                        if dist < closest_dist {
                            closest_dist = dist;
                            closest_prey = Some(other_idx);
                        }
                    }
                }

                if let Some(prey_idx) = closest_prey {
                    let diff = arena.positions[prey_idx] - pos;
                    let dist = diff.length();
                    if dist > 0.001 {
                        seek_force = diff / dist * 2.0;
                    }
                }
                force += seek_force;
            }
            BoidState::Forage => {
                // Wander with slight cohesion to same-species
                if neighbor_count > 0 {
                    let mut cohesion = Vec2::ZERO;
                    let mut separation = Vec2::ZERO;
                    let mut same_species_count = 0;

                    for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                        let other_idx = neighbor_idx as usize;
                        if arena.roles[other_idx] == role && arena.alive[other_idx] {
                            let diff = arena.positions[other_idx] - pos;
                            let dist = diff.length();
                            cohesion += arena.positions[other_idx];
                            if dist > 0.001 && dist < 30.0 {
                                separation -= diff / dist;
                            }
                            same_species_count += 1;
                        }
                    }

                    if same_species_count > 0 {
                        cohesion = (cohesion / same_species_count as f32) - pos;
                        force += cohesion * 0.5 + separation * 2.0;
                    }
                }
            }
            BoidState::Reproduce => {
                // Seek same-species mates
                if neighbor_count > 0 {
                    let mut mate_force = Vec2::ZERO;
                    for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                        let other_idx = neighbor_idx as usize;
                        if arena.roles[other_idx] == role
                            && arena.alive[other_idx]
                            && arena.states[other_idx] == BoidState::Reproduce
                        {
                            let diff = arena.positions[other_idx] - pos;
                            let dist = diff.length();
                            if dist > 0.001 {
                                mate_force += diff / dist * 1.5;
                            }
                        }
                    }
                    force += mate_force;
                }
            }
            BoidState::Wander | BoidState::Dead => {
                // Standard flocking behavior
                if neighbor_count > 0 {
                    let mut cohesion = Vec2::ZERO;
                    let mut alignment = Vec2::ZERO;
                    let mut separation = Vec2::ZERO;
                    let mut same_species_count = 0;

                    for &neighbor_idx in neighbors.iter().take(neighbor_count) {
                        let other_idx = neighbor_idx as usize;
                        if arena.roles[other_idx] == role && arena.alive[other_idx] {
                            let diff = arena.positions[other_idx] - pos;
                            let dist = diff.length();

                            cohesion += arena.positions[other_idx];
                            alignment += arena.velocities[other_idx];
                            if dist > 0.001 {
                                // Inverse square law for strong close-range repulsion
                                let repulsion_strength = (20.0 / dist).powi(2).min(50.0);
                                separation -= (diff / dist) * repulsion_strength;
                            }
                            same_species_count += 1;
                        }
                    }

                    if same_species_count > 0 {
                        let n = same_species_count as f32;
                        cohesion = (cohesion / n - pos) * 0.8; // Reduced cohesion
                        alignment /= n;
                        separation /= n;
                        force += cohesion + alignment * 1.0 + separation * 2.5; // Increased separation weight
                    }
                }
            }
        }

        // Apply agility multiplier
        let agility_mult = arena.genes[idx].agility;
        force *= agility_mult;

        // Always avoid obstacles
        let avoidance = compute_obstacle_avoidance(pos, obstacles);
        force += avoidance;

        arena.scratch_accel[idx] = force;
    }
}

#[inline]
fn compute_obstacle_avoidance(pos: Vec2, obstacles: &[Obstacle]) -> Vec2 {
    let mut force = Vec2::ZERO;
    const BUFFER: f32 = 50.0;

    for obs in obstacles {
        let diff = pos - obs.center;
        let d = diff.length();
        if d < obs.radius + BUFFER && d > 0.001 {
            let repulsion = diff / d; // Safe normalization
            force += repulsion * (100.0 / d);
        }
    }
    force
}
