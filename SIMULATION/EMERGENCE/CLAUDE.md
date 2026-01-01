# EMERGENCE - Boids Ecosystem Simulation

Rust/WASM interactive boids simulation demonstrating emergent behavior in multi-agent systems.
Features herbivores, carnivores, scavengers, fungal networks, and real-time telemetry.

## Build & Run

```bash
# Development (hot reload)
trunk serve SIMULATION/EMERGENCE/index.html --open

# Or use the dev script
./SCRIPTS/dev up emergence

# Production build
trunk build --release SIMULATION/EMERGENCE/index.html

# Output in SIMULATION/EMERGENCE/dist/
```

## Access

- **Local dev**: http://localhost:8089 or http://emergence.local.too.foo (via Caddy)
- **Production**: https://emergence.too.foo

## Architecture

```
SIMULATION/EMERGENCE/
├── Cargo.toml       # Dependencies: simulation-engine, wasm-bindgen
├── Trunk.toml       # Build config (port 8089)
├── index.html       # UI with canvas, controls, telemetry bar
└── src/
    ├── lib.rs       # Main simulation loop, boid rendering
    └── telemetry.rs # Performance metrics and sparklines
```

## Core Concepts

### Boid Roles
- **Herbivore** (green): Forage for food, seed fungal network
- **Carnivore** (red): Hunt herbivores, predatory behavior
- **Scavenger** (blue): Feed on dead organisms

### Emergent Behaviors
- **Flocking**: Cohesion, separation, alignment forces
- **Predator-prey dynamics**: Energy transfer through food chain
- **Fungal network**: Organic growth visualization seeded by herbivores
- **Chakravyu mechanics**: Deadly central trap with rush mechanics

### Telemetry System
Real-time performance dashboard with:
- Population counter (POP)
- Generation tracker (GEN)
- FPS monitor
- Birth/death sparklines (10-second history)
- Role distribution (H/C/S counts)
- Genetic diversity metric

## Controls

### Mouse
- **Click + Drag**: Pan viewport
- **Scroll**: Zoom in/out

### Keyboard
- **Space**: Pause/Resume simulation
- **R**: Reset simulation
- **Arrow Keys**: Manual pan

## Configuration

Key constants (in `src/lib.rs`):
```rust
const ARENA_CAPACITY: usize = 4096;  // Max boids
const CELL_SIZE: usize = 64;         // Spatial grid cell size
const VISION_RADIUS: f32 = 60.0;     // Neighbor detection range
const BOID_SIZE: f32 = 6.0;          // Boid rendering size
```

## Features

- **Zero-allocation simulation loop**: Pre-allocated arena and spatial grid
- **Viewport-adaptive spawn rates**: Different max populations for mobile/desktop
- **Fungal network visualization**: Organic branching patterns
- **Background shader effects**: Grid, digital rain, vignette
- **Responsive telemetry**: Tap-to-peek detailed metrics

## Development

### Adding new boid roles
1. Add variant to `BoidRole` enum (in simulation-engine)
2. Implement rendering in `draw_organism()`
3. Update telemetry role counters

### Tuning performance
- Adjust `ARENA_CAPACITY` for target population
- Modify `spawn_rate_multiplier` for faster/slower growth
- Tune `VISION_RADIUS` to balance realism vs performance

## Testing

Visual testing:
```bash
npx playwright test tests/emergence.spec.ts
```

Manual checklist:
- [ ] Simulation runs at 60 FPS
- [ ] Boids exhibit flocking behavior
- [ ] Predator-prey interactions work
- [ ] Fungal network grows correctly
- [ ] Telemetry updates in real-time
- [ ] Responsive on mobile

## Links

- [Landing page](https://too.foo)
- [Simulation engine](../CORE/SIMULATION_ENGINE)
- [Chladni simulation](../CHLADNI)
