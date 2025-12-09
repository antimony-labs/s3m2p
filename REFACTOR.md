# REFACTOR.md - DNA/CORE/SRC Architecture

## Vision

**AI-Native Engineering Platform** where LLMs and humans collaborate to create powerful engineering tools.

```
DNA/          WORLD, PHYSICS, MATH, DATA - Atomic algorithms
    ↓
CORE/         Engines - Domain-specific systems
    ↓
SRC/          Projects - Application-specific code
```

---

## DNA Architecture

### DNA/WORLD - The Stage (Where Things Exist)

```
DNA/src/world/
├── coordinates/           # Cartesian, spherical, cylindrical, polar
├── transforms/            # Astronomical, geodetic, projection, rotation
├── topology/              # Toroidal, bounded, infinite
├── grid/                  # Uniform, adaptive, unstructured
└── units.rs               # Type-safe physical units
```

### DNA/PHYSICS - The Rules (How Things Behave)

```
DNA/src/physics/
├── core/                  # Units, constants, quantities
├── mechanics/             # Particle, rigid_body, collision, constraint
├── fields/                # Scalar, vector, tensor, wave
├── electromagnetics/      # Maxwell, FDTD, lumped circuits
├── fluids/                # Euler, Navier-Stokes, SPH, Lattice-Boltzmann
├── thermal/               # Conduction, convection, radiation
├── orbital/               # Kepler, N-body, perturbation
└── solvers/
    ├── ode/               # Euler, RK4, Verlet, adaptive
    ├── pde/               # FDM, FEM, spectral
    ├── linear/            # Dense, sparse, iterative, eigensolver
    └── nonlinear/         # Newton, bisection, optimization
```

### DNA/MATH - The Language (Pure Mathematics)

```
DNA/src/math/
├── vec.rs                 # Vec2, Vec3, Vec4
├── mat.rs                 # Mat2, Mat3, Mat4
├── quaternion.rs          # Rotation representation
├── complex.rs             # Complex arithmetic
├── polynomial.rs          # Evaluation, roots
├── interpolation.rs       # Linear, cubic, spline
├── statistics.rs          # Mean, variance, distributions
└── random.rs              # PCG RNG
```

### DNA/DATA - Data Structures

```
DNA/src/data/
├── arena.rs               # Generalized BoidArena pattern
├── spatial_grid.rs        # O(1) neighbor queries
├── quadtree.rs            # Hierarchical 2D
├── octree.rs              # Hierarchical 3D
├── mesh.rs                # Triangle/quad mesh
└── graph.rs               # Node/edge graph
```

---

## CORE Engines

```
CORE/
├── SIMULATION_ENGINE/     # Boid/particle runtime
├── SPICE_ENGINE/          # Circuit simulation (DC, AC, transient)
├── PLL_ENGINE/            # PLL design automation
├── CAD_ENGINE/            # B-rep kernel (geometry, topology, boolean ops)
├── EXPORT_ENGINE/         # Gerber, PDF, STEP, G-code
├── WAVE_ENGINE/           # Wave/field simulation
└── HELIOSPHERE_ENGINE/    # Solar system simulation
```

---

## File Header Standard

Every file has a human-readable header at the top:

```rust
//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: particle.rs
//! PATH: DNA/src/physics/mechanics/particle.rs
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PURPOSE: Point mass dynamics using Newton's laws (F = ma)
//!
//! LAYER: DNA → PHYSICS → MECHANICS
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA DEFINED                                                                │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Particle          Position, velocity, mass of a point mass                  │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA FLOW                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ CONSUMES:  Vec3 (forces), f64 (dt), Integrator                              │
//! │ PRODUCES:  Vec3 (position), Vec3 (velocity), f64 (energy)                   │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DEPENDS ON:
//!   • math/vec.rs         → Vec3
//!   • physics/solvers/ode → Integrator trait
//!
//! USED BY:
//!   • CORE/SIMULATION_ENGINE
//!   • TOOLS/PHYSICS_SIM
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// CODE BELOW - Optimized for ML development
// ─────────────────────────────────────────────────────────────────────────────────
```

### Required Header Sections

| Section | Purpose |
|---------|---------|
| `FILE` | Filename |
| `PATH` | Full path from repo root |
| `PURPOSE` | One sentence description |
| `LAYER` | Position in architecture |
| `DATA DEFINED` | Structs/enums/traits |
| `DATA FLOW` | CONSUMES → PRODUCES |
| `DEPENDS ON` | Import sources |
| `USED BY` | Reverse dependencies |

### Optional Sections

| Section | When to Use |
|---------|-------------|
| `PHYSICS` | Physics equations |
| `UNITS` | Physical units |
| `ALGORITHM` | Known algorithm name |
| `REFERENCE` | External sources |

---

## Migration Phases

### Phase 1: Infrastructure Setup
**PR: `refactor/dna-directory-structure`**
- [ ] Create `DNA/src/world/mod.rs`
- [ ] Create `DNA/src/physics/mod.rs`
- [ ] Create `DNA/src/math/mod.rs`
- [ ] Create `DNA/src/data/mod.rs`
- [ ] Update `DNA/src/lib.rs` exports
- [ ] `cargo check --workspace` passes

### Phase 2: MATH Module
**PR: `refactor/dna-math-module`**
- [ ] Move `mat2.rs` → `math/mat.rs`
- [ ] Move `random.rs` → `math/random.rs`
- [ ] Move `statistics.rs` → `math/statistics.rs`
- [ ] Create `math/complex.rs`
- [ ] Add compatibility re-exports

### Phase 3: DATA Module
**PR: `refactor/dna-data-module`**
- [ ] Create `data/arena.rs`
- [ ] Create `data/spatial_grid.rs`
- [ ] Create `data/mesh.rs` scaffold
- [ ] Create `data/graph.rs` scaffold

### Phase 4: WORLD Module
**PR: `refactor/dna-world-module`**
- [ ] Move `coordinates.rs` → `world/transforms/astronomical.rs`
- [ ] Create `world/coordinates/cartesian.rs`
- [ ] Create `world/coordinates/spherical.rs`
- [ ] Create `world/topology/toroidal.rs`
- [ ] Create `world/units.rs`

### Phase 5: PHYSICS Scaffolds
**PR: `refactor/dna-physics-scaffolds`**
- [ ] Create all physics domain scaffolds
- [ ] Create solver scaffolds (ODE, linear, nonlinear)
- [ ] Add doc comments and test stubs

### Phase 6: PHYSICS Migration
**PR: `refactor/dna-physics-migration`**
- [ ] Migrate `spice/` → `physics/electromagnetics/`
- [ ] Migrate `sim/chladni.rs` → `physics/fields/wave.rs`
- [ ] Migrate `wave_field/fft.rs` → `physics/solvers/pde/spectral.rs`
- [ ] Migrate `ekf.rs` → `physics/solvers/`

### Phase 7: CORE Structure
**PR: `refactor/core-engine-structure`**
- [ ] Create `CORE/` directory
- [ ] Create engine crate scaffolds
- [ ] Add to workspace

### Phase 8: CORE Migration
**PR: `refactor/core-engine-migration`**
- [ ] Move boid simulation → SIMULATION_ENGINE
- [ ] Move PLL logic → PLL_ENGINE
- [ ] Move export code → EXPORT_ENGINE

### Phase 9: CAD Engine (B-Rep)
**PR: `feat/cad-engine-brep`**
- [ ] Create geometry module (point, curve, surface)
- [ ] Create topology module (vertex, edge, face, shell)
- [ ] Implement boolean operations

### Phase 10: ARCH Project
**PR: `feat/arch-project`**
- [ ] Create ARCH project structure
- [ ] Implement AST parser
- [ ] Build interactive explorer
- [ ] Deploy to arch.too.foo

### Phase 11: Cleanup
**PR: `refactor/cleanup-reexports`**
- [ ] Remove compatibility re-exports
- [ ] Update all dependencies
- [ ] Update documentation

---

## Testing Requirements

Each phase must:
1. `cargo check --workspace` - Type check
2. `cargo test --workspace` - All tests pass
3. `trunk build` - WASM builds for affected projects
4. Visual verification of deployed sites

---

## File Creation Patterns

### When to create in physics/mechanics/
- New physical object (particle, rigid body, rope, cloth)
- New force model (gravity, spring, friction, drag)
- New constraint (joint, contact, distance)

### When to create in world/coordinates/
- New coordinate system
- New coordinate conversion

### When to create CORE/ engine
- Domain-specific system composing multiple DNA modules
- Simulation runtime
- Export pipeline

---

## Dependency Graph

```
         PROJECTS (WELCOME, HELIOS, TOOLS/*)
              │
              ▼
           CORE (Engines)
              │
              ▼
    ┌─────────┼─────────┐
    │         │         │
    ▼         ▼         ▼
  WORLD    PHYSICS    MATH    DATA
    └─────────┴─────────┴───────┘
                  DNA
```

---

## Success Metrics

- [ ] All 97,000+ lines migrated
- [ ] DNA = atomic algorithms only
- [ ] CORE = domain engines
- [ ] arch.too.foo live with auto-generated architecture
- [ ] All tests passing
- [ ] No performance regressions
