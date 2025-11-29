# S3M2P Traceability Matrix

This document maps the purpose of each folder and file in the codebase for maintainability and auditing.

## Folder Structure

| Folder | Purpose | Dependencies |
|--------|---------|--------------|
| `/core` | Shared simulation engine, algorithms, and utilities | None (foundation) |
| `/too.foo` | 2D boid ecosystem visualization (WASM) | core |
| `/helios` | 3D heliosphere visualization (WASM/WebGPU) | core |
| `/PROJECT_N` | Reserved for future projects | core |
| `/storage-server` | Spatial data pre-processing server | core |
| `/simulation-cli` | Data generation CLI tools | core |

## Core Library (`/core`)

### Domain Modules (Heliosphere/Astronomy)

| File | Purpose | Tests |
|------|---------|-------|
| `heliosphere.rs` | Heliosphere data types and structures | 0 |
| `coordinates.rs` | Celestial coordinate transformations (HGI, HCI, HEE) | 3 |
| `solar_wind.rs` | Parker spiral, magnetic field modeling | 4 |
| `heliosphere_model.rs` | Heliosphere boundary calculations | 2 |
| `spatial.rs` | Hierarchical spatial indexing (SpatialKey, SpatialStore) | 5 |

### Shared Utility Modules

| File | Purpose | Used By | Tests |
|------|---------|---------|-------|
| `zones.rs` | Exclusion zone primitives | too.foo (fungal, boids) | 5 |
| `interaction.rs` | Entity interaction effects | too.foo (fungal network) | 2 |
| `random.rs` | RNG helpers (angles, directions, circles) | All simulations | 7 |
| `statistics.rs` | Population metrics and diversity | too.foo (status display) | 5 |
| `color.rs` | HSL/RGB conversion, gradients, themes | All visualizations | 7 |

### Boid Simulation Engine (in `lib.rs`)

| Component | Lines | Purpose | Tests |
|-----------|-------|---------|-------|
| `Genome` | 75-178 | Genetic traits and mutation | 2 |
| `BoidArena<CAP>` | 206-365 | Fixed-capacity entity storage (SoA) | 3 |
| `SpatialGrid<CELL_CAP>` | 371-494 | Cache-friendly neighbor queries | 2 |
| `compute_flocking_forces()` | 502-659 | State-aware flocking behavior | 0 (covered by integration) |
| `update_states()` | 681-738 | FSM state transitions | 2 |
| `process_predation()` | 745-786 | Carnivore attack mechanics | 1 |
| `process_scavenging()` | 789-817 | Scavenger feeding | 0 (covered by integration) |
| `simulation_step()` | 847-978 | Main simulation loop | 1 |
| `FoodSource` | 984-1033 | Resource nodes | 0 (covered by integration) |
| `PredatorZone` | 1036-1065 | Dangerous zones | 1 |
| `SeasonCycle` | 1078-1121 | Seasonal food multipliers | 2 |
| `compute_diversity()` | 1232-1280 | Shannon entropy diversity | 2 |
| `trigger_mass_extinction()` | 1283-1318 | Population reset event | 2 |

## too.foo (`/too.foo`)

| File | Purpose | Core Deps |
|------|---------|-----------|
| `main.rs` | WASM entry point, render loop, UI | BoidArena, SpatialGrid, SimConfig |
| `fungal.rs` | Fungal network growth simulation | InteractionResult (implicit pattern) |
| `shader.rs` | Background visual effects | None |

## helios (`/helios`)

| File | Purpose | Core Deps |
|------|---------|-----------|
| `main.rs` | WebGPU 3D renderer entry | SpatialKey, DataLayer |
| `streaming.rs` | Spatial data streaming controller | SpatialStore, SpatialKey |

## Testing Architecture

### Test Categories

| Category | Location | Count | Purpose |
|----------|----------|-------|---------|
| Unit Tests | `core/src/*.rs` | 44 | Module-level functionality |
| Comprehensive | `core/tests/comprehensive_tests.rs` | 23 | Edge cases, numerical stability |
| Simulation | `core/tests/simulation_tests.rs` | 4 | Long-running integration tests |
| too.foo | `too.foo/src/fungal.rs` | 4 | Fungal network tests |
| PROJECT_N | `PROJECT_N/src/lib.rs` | 1 | Placeholder test |

**Total Tests: 83**

### Critical Test Coverage

| Risk Area | Tests | Confidence |
|-----------|-------|------------|
| NaN/Infinity positions | `test_position_nan_handling` | High |
| Zero division | `test_zero_division_safety` | High |
| Arena overflow | `test_full_capacity_arena` | High |
| State machine FSM | `test_all_state_transitions` | High |
| Predator-prey dynamics | `test_predator_prey_dynamics` | Medium |
| Diversity calculation | `test_diversity_*` (4 tests) | High |
| Mass extinction | `test_mass_extinction_*` (2 tests) | Medium |

## Dependencies Graph

```
                    ┌─────────────┐
                    │    core     │
                    │  (shared)   │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
   ┌──────────┐     ┌──────────┐     ┌──────────┐
   │ too.foo  │     │  helios  │     │PROJECT_N │
   │  (2D)    │     │  (3D)    │     │ (future) │
   └──────────┘     └──────────┘     └──────────┘
         │                 │
         ▼                 ▼
   ┌──────────┐     ┌──────────┐
   │   WASM   │     │  WebGPU  │
   │  Canvas  │     │  WASM    │
   └──────────┘     └──────────┘
```

## Build Commands

```bash
# Run all tests
cargo test --workspace

# Build WASM (too.foo)
cd too.foo && trunk build --release

# Build WASM (helios)
cd helios && trunk build --release

# Run storage server
cargo run -p storage-server

# Generate synthetic data
cargo run -p simulation-cli -- generate -c 1000000
```

## Version History

| Date | Change | Tests Affected |
|------|--------|----------------|
| 2024-XX | Initial refactor: antimony-core → core | All |
| 2024-XX | Added zones, interaction, random, statistics modules | +19 tests |
| 2024-XX | Numerical stability fixes (NaN guards) | +2 tests |
