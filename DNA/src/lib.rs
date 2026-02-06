//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | DNA/src/lib.rs
//! PURPOSE: Foundation library root - physics, math, world, data structures
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

// Suppress wasm_bindgen cfg warnings from macro expansion
#![allow(unexpected_cfgs)]

// ============================================================================
// NEW ARCHITECTURE MODULES (DNA/CORE/SRC Refactor)
// See REFACTOR.md for full architecture documentation
// ============================================================================

/// WORLD - The Stage (coordinate systems, topology, grids)
pub mod world;

/// PHYSICS - The Rules (mechanics, fields, solvers)
pub mod physics;

/// MATH - The Language (vectors, matrices, complex numbers)
pub mod math;

/// DATA - Data Structures (arena, spatial grid, mesh, graph)
pub mod data;

// ============================================================================
// SIMULATION PRIMITIVES
// ============================================================================

// Note: coordinates module moved to world::transforms::astronomical
// Re-export for backward compatibility
pub use world::transforms::astronomical as coordinates;

pub mod sim;

// Re-export core simulation types at crate root for backward compatibility
pub use sim::{
    apply_predator_zones, compute_diversity, compute_flocking_forces, feed_from_sources,
    get_boid_color, process_predation, process_scavenging, simulation_step, trigger_earthquake,
    trigger_mass_extinction, trigger_migration, update_states, BoidArena, BoidHandle, BoidRole,
    BoidState, FoodSource, Genome, Obstacle, PredatorZone, SeasonCycle, SimConfig, SpatialGrid,
    WorldEvent,
};

/// Spatial indexing for cube-sphere LOD data (stars, planets, etc.)
pub mod spatial;
pub use spatial::*;

// ============================================================================
// SHARED UTILITY MODULES (Used by too.foo, helios, future projects)
// ============================================================================

/// Export utilities (STEP, CAD formats)
pub mod export;
pub use export::*;

/// Zone and exclusion area utilities
pub mod zones;
pub use zones::*;

/// Entity interaction effects
pub mod interaction;
pub use interaction::*;

// Note: random module moved to math::random
// Re-export for backward compatibility (don't glob re-export to avoid rand collision)
pub use math::random::{
    random_angle, random_direction, random_in_annulus, random_in_circle, random_in_rect,
    random_in_rect_with_margin, random_index, random_velocity, random_velocity_range, roll_chance,
};

/// Population statistics and metrics
pub mod statistics;
pub use statistics::*;

/// Power law network effects simulation
pub mod powerlaw;

/// Color management and theme utilities
pub mod color;
pub use color::*;

/// Wave field simulation with FFT
/// Note: FFT migrated to physics/solvers/pde/spectral, Chladni to physics/fields/wave
pub mod wave_field;
pub use wave_field::*;

// Also export from new locations
pub use physics::fields::wave::{ChladniMode, PlateMode, WaveSimulation};
pub use physics::solvers::pde::FFT2D;

/// PLL (Phase-Locked Loop) circuit design
pub mod pll;
pub use pll::*;

/// Power supply design (Buck, Boost, LDO)
pub mod power;
pub use power::{
    design_boost, design_buck, design_ldo, quick_boost, quick_buck, quick_ldo, recommend_topology,
    BoostDesign, BoostRequirements, BuckDesign, BuckRequirements, DesignPriority, DesignWarning,
    EfficiencyBreakdown, LDODesign, LDORequirements, OperatingMode, RippleSpec, SelectedComponent,
    ThermalAnalysis, TopologyRecommendation, TopologyType, VoltageRange,
};

/// SPICE circuit simulation engine
/// DEPRECATED: Use `physics::electromagnetics::lumped` or `spice_engine` crate
#[deprecated(
    since = "0.1.0",
    note = "use `physics::electromagnetics::lumped` instead"
)]
pub mod spice;

/// Autocrate crate generation algorithms
pub mod autocrate;

// ============================================================================
// MATH AND FILTERING MODULES
// ============================================================================

// Mat2 canonical location
pub use math::mat::Mat2;

/// Extended Kalman Filter for state estimation
/// DEPRECATED: Use `physics::solvers::filters::EKF`
#[deprecated(since = "0.1.0", note = "use `physics::solvers::filters::EKF` instead")]
pub mod ekf;

// Canonical export from new location
pub use physics::solvers::filters::{smooth_trajectory, EKF};

/// A* and grid-based pathfinding
pub mod pathfinding;
pub use pathfinding::{astar, GridMap, Heuristic, PathResult};

/// CAD module (B-Rep solid modeling)
pub mod cad;

/// Security module (secrets and PII detection)
pub mod security;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn test_arena_spawn_kill() {
        let mut arena: BoidArena<100> = BoidArena::new();
        assert_eq!(arena.alive_count, 0);
        assert_eq!(arena.remaining_capacity(), 100);

        let h1 = arena.spawn(Vec2::new(10.0, 10.0), Vec2::ZERO, Genome::random());
        assert!(h1.is_valid());
        assert_eq!(arena.alive_count, 1);

        arena.kill(h1.index as usize);
        assert_eq!(arena.alive_count, 0);
        assert!(!arena.is_alive(h1)); // Handle invalidated by generation bump
    }

    #[test]
    fn test_spatial_grid() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let h1 = arena.spawn(Vec2::new(10.0, 10.0), Vec2::ZERO, Genome::random());
        let _h2 = arena.spawn(Vec2::new(15.0, 10.0), Vec2::ZERO, Genome::random());
        let _h3 = arena.spawn(Vec2::new(100.0, 100.0), Vec2::ZERO, Genome::random());

        let mut grid: SpatialGrid<16> = SpatialGrid::new(200.0, 200.0, 50.0);
        grid.build(&arena);

        let count = grid.count_neighbors(Vec2::new(10.0, 10.0), 20.0, &arena, h1.index as usize);
        assert_eq!(count, 1); // Should find boid at (15, 10) but not self
    }

    #[test]
    fn test_genome_defaults() {
        let genome = Genome::random();
        assert!(genome.max_speed >= 2.0 && genome.max_speed <= 4.0);
        assert!(genome.agility >= 0.5 && genome.agility <= 2.0);
        assert!(genome.size >= 0.5 && genome.size <= 2.0);
        assert!(genome.strength >= 0.5 && genome.strength <= 2.0);
        assert!(genome.sensor_radius >= 40.0 && genome.sensor_radius <= 120.0);
        assert!(genome.metabolism >= 0.7 && genome.metabolism <= 1.3);
    }

    #[test]
    fn test_mutation_gigantism() {
        let parent = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };

        // Force gigantism by testing multiple mutations until we get one
        let mut found_gigantism = false;
        for _ in 0..100 {
            let child = parent.mutate();
            if child.size > parent.size * 1.15 && child.strength > parent.strength * 1.15 {
                assert!(
                    child.max_speed < parent.max_speed * 1.05
                        || child.max_speed <= parent.max_speed
                );
                found_gigantism = true;
                break;
            }
        }
        assert!(found_gigantism, "Should find gigantism mutation");
    }

    #[test]
    fn test_role_inheritance() {
        let parent = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };

        let mut role_changes = 0;
        for _ in 0..100 {
            let child = parent.mutate();
            if child.role != parent.role {
                role_changes += 1;
            }
        }

        // Speciation should be rare (~1%)
        assert!(
            role_changes <= 5,
            "Role mutation rate should be low, got {} changes",
            role_changes
        );
    }

    #[test]
    fn test_state_transition_flee() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let mut grid: SpatialGrid<16> = SpatialGrid::new(200.0, 200.0, 60.0);

        let herb_genes = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };
        let herb_idx = arena
            .spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, herb_genes)
            .index as usize;

        let carn_genes = Genome {
            role: BoidRole::Carnivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (0, 80),
        };
        let _carn_idx = arena
            .spawn(Vec2::new(60.0, 50.0), Vec2::ZERO, carn_genes)
            .index as usize;

        grid.build(&arena);
        update_states(&mut arena, &grid);

        assert_eq!(
            arena.states[herb_idx],
            BoidState::Flee,
            "Herbivore should flee from nearby Carnivore"
        );
    }

    #[test]
    fn test_state_transition_hunt() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let mut grid: SpatialGrid<16> = SpatialGrid::new(200.0, 200.0, 60.0);

        let carn_genes = Genome {
            role: BoidRole::Carnivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (0, 80),
        };
        let carn_idx = arena
            .spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, carn_genes)
            .index as usize;

        let herb_genes = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };
        let _herb_idx = arena
            .spawn(Vec2::new(60.0, 50.0), Vec2::ZERO, herb_genes)
            .index as usize;

        grid.build(&arena);
        update_states(&mut arena, &grid);

        assert_eq!(
            arena.states[carn_idx],
            BoidState::Hunt,
            "Carnivore should hunt nearby Herbivore"
        );
    }

    #[test]
    fn test_state_forage() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let grid: SpatialGrid<16> = SpatialGrid::new(200.0, 200.0, 60.0);

        let genes = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;

        arena.energy[idx] = 40.0;

        update_states(&mut arena, &grid);

        assert_eq!(
            arena.states[idx],
            BoidState::Forage,
            "Low energy Herbivore should forage"
        );
    }

    #[test]
    fn test_predation_damage() {
        let mut arena: BoidArena<100> = BoidArena::new();

        let carn_genes = Genome {
            role: BoidRole::Carnivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.5,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (0, 80),
        };
        let carn_idx = arena
            .spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, carn_genes)
            .index as usize;
        arena.energy[carn_idx] = 100.0;

        let herb_genes = Genome {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        };
        let herb_idx = arena
            .spawn(Vec2::new(51.0, 50.0), Vec2::ZERO, herb_genes)
            .index as usize;
        let initial_herb_energy = 100.0;
        arena.energy[herb_idx] = initial_herb_energy;

        process_predation(&mut arena);

        assert!(
            arena.energy[herb_idx] < initial_herb_energy,
            "Herbivore should lose energy"
        );
        assert!(
            arena.energy[carn_idx] > 100.0 || arena.energy[carn_idx] == 100.0,
            "Carnivore should gain energy or stay at max"
        );
    }

    #[test]
    fn test_death_cleanup() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::random();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;

        arena.kill(idx);

        assert_eq!(
            arena.states[idx],
            BoidState::Dead,
            "Dead boid should be marked as Dead"
        );
        assert!(!arena.alive[idx], "Dead boid should not be alive");
    }

    #[test]
    fn test_arena_capacity_4096() {
        let mut arena: BoidArena<4096> = BoidArena::new();
        let mut grid: SpatialGrid<32> = SpatialGrid::new(1000.0, 1000.0, 60.0);

        for i in 0..4096 {
            let x = (i % 100) as f32 * 10.0;
            let y = (i / 100) as f32 * 10.0;
            arena.spawn(Vec2::new(x, y), Vec2::ZERO, Genome::random());
        }

        assert_eq!(arena.alive_count, 4096);

        grid.build(&arena);

        let config = SimConfig::default();
        let (births, deaths) = simulation_step(&mut arena, &grid, &config, 1000.0, 1000.0, 1.0);

        let _ = (births, deaths);
    }

    // ============================================================================
    // WORLD & ECOSYSTEM TESTS
    // ============================================================================

    #[test]
    fn test_feed_from_sources() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;
        arena.energy[idx] = 50.0;

        let mut food_sources = vec![FoodSource::new(50.0, 50.0)];
        let season = SeasonCycle::new();

        feed_from_sources(&mut arena, &mut food_sources, &season);

        assert!(
            arena.energy[idx] > 50.0,
            "Boid inside food source should gain energy"
        );
    }

    #[test]
    fn test_feed_from_sources_out_of_range() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(500.0, 500.0), Vec2::ZERO, genes).index as usize;
        arena.energy[idx] = 50.0;

        let mut food_sources = vec![FoodSource::new(50.0, 50.0)];
        let season = SeasonCycle::new();

        feed_from_sources(&mut arena, &mut food_sources, &season);

        assert_eq!(
            arena.energy[idx], 50.0,
            "Boid far from food should not gain energy"
        );
    }

    #[test]
    fn test_apply_predator_zones() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;
        arena.energy[idx] = 100.0;

        let predators = vec![PredatorZone::new(50.0, 50.0)];
        apply_predator_zones(&mut arena, &predators);

        assert!(
            arena.energy[idx] < 100.0,
            "Boid inside predator zone should lose energy"
        );
    }

    #[test]
    fn test_trigger_migration() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;

        let direction = Vec2::new(1.0, 0.0);
        trigger_migration(&mut arena, direction, 5.0);

        assert!(
            arena.velocities[idx].x > 0.0,
            "Migration should push boid in direction"
        );
    }

    #[test]
    fn test_trigger_earthquake() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;
        arena.energy[idx] = 100.0;

        trigger_earthquake(&mut arena);

        assert!(
            arena.energy[idx] < 100.0,
            "Earthquake should drain energy"
        );
        // Velocity should be randomized (unlikely to stay at exactly zero)
        let vel = arena.velocities[idx];
        assert!(
            vel.x != 0.0 || vel.y != 0.0,
            "Earthquake should randomize velocity"
        );
    }

    #[test]
    fn test_compute_diversity_balanced() {
        let mut arena: BoidArena<100> = BoidArena::new();

        // Spawn balanced population: 10 of each role
        for _ in 0..10 {
            let mut g = Genome::default();
            g.role = BoidRole::Herbivore;
            g.max_speed = 2.5;
            arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, g);
        }
        for _ in 0..10 {
            let mut g = Genome::default();
            g.role = BoidRole::Carnivore;
            g.max_speed = 3.5;
            arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, g);
        }
        for _ in 0..10 {
            let mut g = Genome::default();
            g.role = BoidRole::Scavenger;
            g.max_speed = 4.0;
            arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, g);
        }

        let diversity = compute_diversity(&arena);
        assert!(
            diversity > 0.5,
            "Balanced 3-role population should have high diversity, got {}",
            diversity
        );
    }

    #[test]
    fn test_compute_diversity_monoculture() {
        let mut arena: BoidArena<100> = BoidArena::new();

        // All herbivores, same speed
        for _ in 0..30 {
            let g = Genome::default(); // All herbivore, max_speed=3.0
            arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, g);
        }

        let diversity = compute_diversity(&arena);
        assert!(
            diversity < 0.3,
            "Monoculture should have low diversity, got {}",
            diversity
        );
    }

    #[test]
    fn test_trigger_mass_extinction() {
        let mut arena: BoidArena<100> = BoidArena::new();

        for _ in 0..50 {
            arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, Genome::random());
        }
        assert_eq!(arena.alive_count, 50);

        trigger_mass_extinction(&mut arena, 0.8, 200.0, 200.0);

        // Should kill ~80% but spawn founders, so alive_count should be much less than 50
        assert!(
            arena.alive_count < 30,
            "Mass extinction should reduce population significantly, got {}",
            arena.alive_count
        );
    }

    #[test]
    fn test_get_boid_color() {
        let mut arena: BoidArena<100> = BoidArena::new();
        let genes = Genome::default();
        let idx = arena.spawn(Vec2::new(50.0, 50.0), Vec2::ZERO, genes).index as usize;
        arena.energy[idx] = 100.0;

        let (hue, sat, lightness) = get_boid_color(&arena, idx);

        // Herbivore default: hue ~120 (green), sat ~70
        assert!(hue >= 100 && hue <= 160, "Herbivore hue should be green range, got {}", hue);
        assert!(sat >= 50 && sat <= 100, "Saturation should be in valid range, got {}", sat);
        assert!(lightness >= 25 && lightness <= 80, "Lightness should scale with energy, got {}", lightness);
    }

    #[test]
    fn test_food_source_depletion_and_regen() {
        let mut food = FoodSource::new(0.0, 0.0);
        let initial = food.energy;

        // Consume most of it
        for _ in 0..1000 {
            food.consume(1.0);
        }
        assert!(food.is_depleted(), "Food should be depleted after heavy consumption");

        // Regenerate
        for _ in 0..200 {
            food.regenerate(1.0, 1.0);
        }
        assert!(
            food.energy > initial * 0.1,
            "Food should regenerate over time"
        );
    }

    #[test]
    fn test_season_cycle() {
        let mut season = SeasonCycle::new();
        assert_eq!(season.season_name(), "SPRING");

        // Advance to summer (period/4)
        season.update(season.period * 0.25 + 1.0);
        assert_eq!(season.season_name(), "SUMMER");

        // Summer should have high food multiplier
        let mult = season.food_multiplier();
        assert!(mult > 1.0, "Summer food multiplier should be high, got {}", mult);
    }

    #[test]
    fn test_predator_zone_lifecycle() {
        let mut zone = PredatorZone::new(50.0, 50.0);
        assert!(zone.active);

        // Advance past lifetime
        for _ in 0..1000 {
            zone.update(1.0);
        }
        assert!(!zone.active, "Predator zone should deactivate after lifetime");
    }
}
