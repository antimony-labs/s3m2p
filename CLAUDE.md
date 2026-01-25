# antimony-labs - AI-Native Development Platform

Rust/WASM monorepo for simulations, visualizations, and full product development.
Deployed to **too.foo**

## Quick Reference

| Command | Description |
|---------|-------------|
| `cargo check --workspace` | Type check all crates |
| `cargo test --workspace` | Run all tests |
| `trunk build HELIOS/index.html` | Build helios WASM |
| `trunk build WELCOME/index.html` | Build WELCOME (too.foo) WASM |
| `./SCRIPTS/dev up <project>` | Dev server (auto-kills existing) |
| `./SCRIPTS/deploy.sh welcome --publish` | Deploy too.foo (WELCOME) |
| `./SCRIPTS/worktree.sh create <issue>` | Create worktree for issue |
| `./SCRIPTS/audit.sh` | Security audit |

## Dev Server

**IMPORTANT: Always use `./SCRIPTS/dev up <project>` to start dev servers. NEVER run `trunk serve` directly.**

The dev script:
- Binds to `0.0.0.0` for local network access (test on phones/tablets)
- Auto-kills existing process on the port
- Sets correct environment variables

```bash
# Correct
./SCRIPTS/dev up blog

# Wrong - don't do this
trunk serve BLOG/index.html --port 8085
```

## Dev Server Ports

Each project has a dedicated port to allow multiple services to run simultaneously.

| Project | Port | URL | Description |
|---------|------|-----|-------------|
| welcome | 8080 | http://127.0.0.1:8080 | too.foo landing page |
| helios | 8081 | http://127.0.0.1:8081 | Solar system |
| chladni | 8082 | http://127.0.0.1:8082 | Chladni patterns |
| autocrate | 8083 | http://127.0.0.1:8083 | Crate generator |
| sensors | 8084 | http://127.0.0.1:8084 | Sensor test (LEARN) |
| blog | 8085 | http://127.0.0.1:8085 | Blog platform |
| learn | 8086 | http://127.0.0.1:8086 | Learning hub |
| arch | 8087 | http://127.0.0.1:8087 | Architecture explorer |
| mcad | 8088 | http://127.0.0.1:8088 | Mechanical CAD modeler |
| pll | 8090 | http://127.0.0.1:8090 | PLL designer |
| power | 8091 | http://127.0.0.1:8091 | Power circuits |
| ai | 8100 | http://127.0.0.1:8100 | AI tutorials |
| ubuntu | 8101 | http://127.0.0.1:8101 | Ubuntu tutorials |
| opencv | 8102 | http://127.0.0.1:8102 | OpenCV tutorials |
| arduino | 8103 | http://127.0.0.1:8103 | Arduino tutorials |
| esp32 | 8104 | http://127.0.0.1:8104 | ESP32 tutorials |
| swarm | 8105 | http://127.0.0.1:8105 | Swarm robotics |
| slam | 8106 | http://127.0.0.1:8106 | SLAM tutorials |
| git | 8107 | http://127.0.0.1:8107 | Git tutorials |
| ds | 8108 | http://127.0.0.1:8108 | Data structures |

## Agent Protocol

### Handling New Tasks (Issue Paste)
When the user pastes a GitHub issue description or URL:
1.  **Extract Issue ID**: Identify the issue number (e.g., `#42` or from URL).
2.  **Setup Worktree**: Execute `./SCRIPTS/worktree.sh create <issue_id>`.
    *   *Note*: This script uses `gh` to fetch details and creates a worktree in `~/worktrees/`.
3.  **Context Switch**: Instruct the user to `cd` into the new worktree path provided by the script output.
4.  **Start Work**: Begin analyzing the codebase context *within that new worktree*.

## Directory Structure

### L1: Main Bubbles (too.foo landing page)
```
S3M2P/
├── DNA/                    # Core algorithms + infrastructure
├── WELCOME/                # Landing page (too.foo)
├── HELIOS/                 # Solar system (helios.too.foo)
├── SIMULATION/             # Simulations (e.g., chladni.too.foo)
├── TOOLS/                  # User-facing tools (autocrate.too.foo, etc.)
├── MCAD/                   # Mechanical CAD (mcad.too.foo)
├── LEARN/                  # Learning tutorials (ai.too.foo, slam.too.foo, etc.)
└── BLOG/                   # Blog platform (blog.too.foo)
```

### L2: Projects within each bubble
```
DNA/
├── src/                    # Core simulation algorithms
├── SIMULATION_CLI/         # CLI for running simulations
├── STORAGE_SERVER/         # Storage backend
└── CLAUDE_AUTOMATION/      # GitHub automation

SIMULATION/
└── CHLADNI/                # Chladni wave patterns (chladni.too.foo)

TOOLS/
├── AUTOCRATE/              # Shipping crate generator (autocrate.too.foo)
├── PLL/                    # PLL designer (pll.too.foo)
├── POWER_CIRCUITS/         # Power circuit designer (power.too.foo)
└── SPICE/                  # SPICE simulation

MCAD/
├── src/                    # WASM frontend
├── CORE/                   # MCAD engines
│   └── CAD_ENGINE/         # B-Rep kernel, solid modeling
└── index.html              # CAD UI (mcad.too.foo)

LEARN/
├── AI/                     # AI/ML tutorials (ai.too.foo)
├── SLAM/                   # SLAM tutorials (slam.too.foo)
├── ESP32/                  # ESP32 tutorials (esp32.too.foo)
├── ARDUINO/                # Arduino tutorials (arduino.too.foo)
├── UBUNTU/                 # Ubuntu tutorials (ubuntu.too.foo)
├── OPENCV/                 # OpenCV tutorials (opencv.too.foo)
├── SWARM_ROBOTICS/         # Swarm robotics (swarm.too.foo)
├── SENSORS/                # Sensor demos (sensors.too.foo)
└── ML/                     # ML fundamentals
```

### Philosophy: Build From Scratch

**Minimize external dependencies** - write our own:
- B-rep kernel for MCAD
- Gerber X2 generator for ECAD
- STEP file writer (ISO 10303-21)
- G-code generator

**Only use external crates for**:
- GPU access (wgpu)
- Math primitives (glam)
- WASM bindings (wasm-bindgen)
- Serialization (serde)

## Naming Convention

| Level | Case | Example | Rationale |
|-------|------|---------|-----------|
| Category folders (L1) | UPPERCASE | `BLOG/`, `LEARN/`, `SIMULATION/` | Fixed landmarks |
| Project folders (L2) | UPPERCASE | `HELIOS/`, `AUTOCRATE/` | Fixed project names |
| Config folders | lowercase | `src/`, `dist/`, `assets/` | Standard conventions |
| Variable files | lowercase | `main.rs`, `index.html` | Content changes |
| Fixed files | As required | `Cargo.toml`, `CLAUDE.md` | Syntax requirements |

## DNA & CORE Pattern

| Folder | Location | Purpose |
|--------|----------|---------|
| **DNA/** | `/DNA/` (root) | Shared foundation - simulation engine, algorithms |
| **CORE/** | Per-project (e.g., `/SIMULATION/CORE/`, `/TOOLS/CORE/`) | Project-specific Rust logic |

DNA = genetic code shared by all | CORE = heart of each project

## Project Dependencies

```
DNA <── HELIOS
    <── SIMULATION/CHLADNI
    <── DNA/SIMULATION_CLI
    <── DNA/STORAGE_SERVER
    <── LEARN/* (some)
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

### Commit Message Guidelines

**Important:** Use `Refs #XX` (not `Closes #XX`) in individual commit messages to link to issues without auto-closing them. GitHub scans ALL commit messages when a PR is merged and will close any issue referenced with closing keywords.

| In Commits | In PR Body |
|------------|------------|
| `Refs #XX` | `Closes #XX` |
| `Related to #XX` | `Fixes #XX` |

**Bad:** `feat(helios): add rotation Closes #22` → Issue closes on any PR merge containing this commit
**Good:** `feat(helios): add rotation Refs #22` → Issue only closes when explicitly linked in PR body

### Preview Deployments

To get a preview URL for your changes:

1. Create a branch named `preview/issue-XX` (where XX is the issue number)
2. Ensure the issue has a `project:*` label (e.g., `project:helios`)
3. Push to the branch
4. The CI workflow will build, deploy to Cloudflare Pages, and post the preview URL as a comment on the issue

```bash
git checkout -b preview/issue-27
# make changes
git push origin preview/issue-27
# → Preview URL will be posted to issue #27
```

Supported projects: helios, welcome, mcad, ecad, chladni, autocrate, portfolio, blog

### Creating PRs

Use `/pr` command to:
- Generate PR from current branch
- Link to issue (use `Closes #XX` only in PR body, not commits)
- Include test plan

## Code Style

- Rust 2021 edition
- No heap allocations in hot paths (simulation loop)
- Use `#[inline]` for small functions called per-entity
- Wrap coordinates at world boundaries (toroidal topology)
- Energy clamped to [0, 200], metabolism affects drain rate
- **Zero Warnings Policy**: All code must compile with `cargo check --workspace` producing NO warnings. Fix unused variables, dead code, and deprecated calls immediately.

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
