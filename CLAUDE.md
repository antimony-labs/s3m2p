# antimony-labs - AI-Native Development Platform

Rust/WASM monorepo for simulations, visualizations, and full product development.
Deployed to **too.foo**

## Quick Reference

| Command | Description |
|---------|-------------|
| `cargo check --workspace` | Type check all crates |
| `cargo test --workspace` | Run all tests |
| `trunk build SIM/HELIOS/index.html` | Build helios WASM |
| `trunk build SIM/TOOFOO/index.html` | Build too.foo WASM |
| `trunk serve SIM/HELIOS/index.html` | Dev server for helios |
| `./SCRIPTS/deploy.sh toofoo --publish` | Deploy too.foo |
| `./SCRIPTS/worktree.sh create <issue>` | Create worktree for issue |
| `./SCRIPTS/audit.sh` | Security audit |

## Directory Structure

```
S3M2P/
├── DNA/                    # Shared foundation (simulation engine)
├── SIM/                    # Simulations
│   ├── HELIOS/             # Solar system visualization
│   └── TOOFOO/             # Landing page + boid ecosystem
├── SW/                     # Software projects
│   ├── AUTOCRATE/          # ASTM crate generator
│   ├── CHLADNI/            # Wave patterns
│   └── PORTFOLIO/          # Interactive demos
├── HW/                     # Hardware/Embedded [FUTURE]
├── BLOG/                   # Blog platform
├── LEARN/                  # Learning platform
│   └── ML/                 # Machine learning lessons
├── TOOLS/                  # Internal tools
│   ├── SIMULATION_CLI/     # CLI tools
│   └── STORAGE_SERVER/     # Backend persistence
├── DOCS/                   # Documentation
├── SCRIPTS/                # Automation scripts
├── DATA/                   # External data scripts
├── CAD/                    # Mechanical CAD [FUTURE]
├── ECAD/                   # Electronic CAD [FUTURE]
└── MFG/                    # Manufacturing [FUTURE]
```

## Naming Convention

| Level | Case | Example | Rationale |
|-------|------|---------|-----------|
| Category folders (L1) | UPPERCASE | `BLOG/`, `LEARN/`, `SIM/` | Fixed landmarks |
| Project folders (L2) | UPPERCASE | `HELIOS/`, `AUTOCRATE/` | Fixed project names |
| Config folders | lowercase | `src/`, `dist/`, `assets/` | Standard conventions |
| Variable files | lowercase | `main.rs`, `index.html` | Content changes |
| Fixed files | As required | `Cargo.toml`, `CLAUDE.md` | Syntax requirements |

## DNA & CORE Pattern

| Folder | Location | Purpose |
|--------|----------|---------|
| **DNA/** | `/DNA/` (root) | Shared foundation - simulation engine, algorithms |
| **CORE/** | `/SIM/HELIOS/CORE/` | Project-specific Rust logic |

DNA = genetic code shared by all | CORE = heart of each project

## Project Dependencies

```
DNA <── SIM/HELIOS
    <── SIM/TOOFOO
    <── TOOLS/SIMULATION_CLI
    <── TOOLS/STORAGE_SERVER
    <── SW/* (some)
```

## Core Concepts

### BoidArena (DNA)
Fixed-capacity, zero-allocation entity storage using Structure of Arrays (SoA) layout.
- `BoidHandle`: Generational index for safe entity references
- O(1) spawn/kill operations via free list
- Pre-allocated scratch buffers for per-frame computations

### SpatialGrid (DNA)
Spatial partitioning for O(1) neighbor queries.
- Fixed-size cells, no per-cell allocations
- `query_neighbors()` writes to caller-provided buffer

### State Machine (DNA)
Boid behavior states: `Wander`, `Forage`, `Hunt`, `Flee`, `Reproduce`, `Dead`
- State transitions based on energy, threats, and neighbors
- Different flocking forces per state

## Development Workflow

### Starting Work on an Issue

1. Create issue using GitHub templates (enforces project labels)
2. Use `/work <issue-number>` command in Claude Code
3. This creates a worktree and branch automatically
4. Work in isolation, then PR back to main

### Validation Before Commit

Use `/validate` command to run:
- `cargo check` for affected crates
- `cargo test` for test crates
- `trunk build` for WASM crates
- `playwright test` if UI changed

### Creating PRs

Use `/pr` command to:
- Generate PR from current branch
- Link to issue
- Include test plan

## Code Style

- Rust 2021 edition
- No heap allocations in hot paths (simulation loop)
- Use `#[inline]` for small functions called per-entity
- Wrap coordinates at world boundaries (toroidal topology)
- Energy clamped to [0, 200], metabolism affects drain rate

## Testing

- Unit tests in `DNA/src/*.rs` (run with `cargo test -p dna`)
- Visual regression tests in `tests/` (Playwright)
- Snapshot tests for canvas rendering

## Security

- `deny.toml` - Dependency policy (cargo-deny)
- `rust-toolchain.toml` - Pinned toolchain
- `./SCRIPTS/audit.sh` - Security audit script

## Common Pitfalls

1. **Zero-length vectors**: Always check `length() > epsilon` before normalizing
2. **WASM bindings**: Use `wasm-bindgen = "=0.2.93"` (pinned version)
3. **getrandom**: Requires `features = ["js"]` for WASM target
4. **Canvas coordinates**: Y-axis increases downward

## File Patterns

- `src/lib.rs` - Public API exports
- `src/main.rs` - WASM entry point (projects)
- `src/render.rs` - Canvas rendering code
- `src/simulation.rs` - Per-frame update logic
