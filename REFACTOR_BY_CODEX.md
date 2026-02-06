# REFACTOR_BY_CODEX.md — Comprehensive Refactoring Instructions

> **Purpose**: This document is the complete instruction set for Codex (or any LLM agent) to refactor the S3M2P monorepo. Each task is self-contained with exact file paths, expected outcomes, and validation commands. Execute tasks in order — later tasks depend on earlier ones.
>
> **Repo**: `github.com/Shivam-Bhardwaj/S3M2P` — Rust/WASM monorepo deployed to `too.foo`
> **Branch**: `main` (only branch)
> **Total codebase**: ~148,000 lines of Rust across 47 workspace members
> **Current state**: Compiles with warnings. No test failures blocking.

## COMPLETION STATUS (2026-02-06)

The following tasks have been **completed** and should be **skipped** by Codex:

| Section | Status | Summary |
|---------|--------|---------|
| 1. Critical Fixes | DONE | Edition 2024→2021 fixed in 4 Cargo.toml files |
| 2. Remove Dead Weight | DONE | cloudflared.deb, COMING_SOON/, DOCS/, stale blog posts, HELIOS/simulation-cli deleted |
| 3.1 Workspace members | DONE | HANDTRACK + POWERLAW added to workspace, standalone [workspace] removed |
| 3.2 Profile consolidation | DONE | [profile.release] moved to root, removed from 10 sub-crates |
| 4.1 SIMULATIONS→SIMULATION | DONE | Merged, paths updated in Cargo.toml + config.sh |
| 5. DNA lib.rs decomposition | DONE | Split into sim/{types,arena,spatial_grid,flocking,state_machine,interactions,simulation,world}.rs. lib.rs now ~130 lines of re-exports + tests |
| 6. Dependency standardization | DONE | wasm-bindgen pinned to =0.2.93 in all 4 files |
| 7. Gitignore hardening | DONE | Comprehensive .gitignore rewritten |
| 8. CI/CD improvements | DONE | Added fmt/clippy/test to ci.yml, fixed SIMULATIONS→SIMULATION path, added CSP meta to WELCOME |
| 9. Documentation | DONE | DNA CLAUDE.md updated with new sim/ module structure |
| 10. Code quality | DONE | clippy --fix + cargo fmt applied |

**What Codex should verify**: Run `cargo check --workspace && cargo test -p dna` to confirm everything compiles and tests pass. Then review any remaining warnings from `cargo clippy --workspace`.

---

---

## TABLE OF CONTENTS

1. [Critical Fixes](#1-critical-fixes)
2. [Remove Dead Weight](#2-remove-dead-weight)
3. [Cargo Workspace Cleanup](#3-cargo-workspace-cleanup)
4. [Directory Structure Normalization](#4-directory-structure-normalization)
5. [DNA lib.rs Decomposition](#5-dna-librs-decomposition)
6. [Dependency Standardization](#6-dependency-standardization)
7. [Gitignore Hardening](#7-gitignore-hardening)
8. [CI/CD Improvements](#8-cicd-improvements)
9. [Documentation Consolidation](#9-documentation-consolidation)
10. [Code Quality Sweep](#10-code-quality-sweep)
11. [Security Hardening](#11-security-hardening)
12. [Validation Checklist](#12-validation-checklist)

---

## 1. CRITICAL FIXES

These block compilation or cause broken builds. Fix first.

### 1.1 Fix invalid Rust edition "2024"

Rust edition 2024 does not exist on stable. Change to `"2021"`.

**Files to edit:**
```
BLOG/Cargo.toml                    line 10: edition = "2024" → edition = "2021"
HELIOS/simulation-cli/Cargo.toml   line 10: edition = "2024" → edition = "2021"
LEARN/ML/Cargo.toml                line  9: edition = "2024" → edition = "2021"
LEARN/ML/antimony-core/Cargo.toml  line  9: edition = "2024" → edition = "2021"
```

**Validation:**
```bash
cargo check --workspace 2>&1 | grep -i "edition"
# Should produce no edition-related errors
```

### 1.2 Remove `[profile.release]` from non-root Cargo.toml files

Workspace members cannot define profiles — these are silently ignored but generate 10 warnings on every build. Remove the `[profile.release]` sections from these files:

**Files to edit (remove the `[profile.release]` block and its contents from each):**
```
BLOG/Cargo.toml                     lines 43-45
SIMULATION/CHLADNI/Cargo.toml       lines 51-53
TOOLS/PLL/Cargo.toml                line 40+
TOOLS/POWER_CIRCUITS/Cargo.toml     line 36+
LEARN/SENSORS/Cargo.toml            lines 53-55
LEARN/AI/Cargo.toml                 lines 35-37
LEARN/OPENCV/Cargo.toml             line 45+
LEARN/ARDUINO/Cargo.toml            line 20+
LEARN/SWARM_ROBOTICS/Cargo.toml     line 33+
LEARN/DATA_STRUCTURES/Cargo.toml    line 35+
```

**Add a single shared profile to the root `Cargo.toml`** (after `[workspace.lints.clippy]`):
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

**Validation:**
```bash
cargo check --workspace 2>&1 | grep "profiles for the non root"
# Should produce ZERO lines
```

---

## 2. REMOVE DEAD WEIGHT

### 2.1 Remove `LEARN/ML/cloudflared.deb` (20 MB binary in git)

This is a Cloudflare tunnel binary that should never be tracked in git.

```bash
git rm LEARN/ML/cloudflared.deb
```

Add to root `.gitignore`:
```
*.deb
```

### 2.2 Remove `nerd-dictation/` directory

This is an untracked 4.6 GB directory containing speech recognition models. It's not part of the project.

```bash
rm -rf nerd-dictation/
```

Add to root `.gitignore`:
```
nerd-dictation/
```

### 2.3 Remove `COMING_SOON/` project

This is a legacy placeholder page with test files and stale artifacts. It's not in the workspace and serves no purpose.

**Files to delete:**
```
COMING_SOON/          (entire directory)
```

Remove any references in `SCRIPTS/config.sh` (port 8109 "coming_soon" entry).

### 2.4 Remove `HELIOS/simulation-cli/`

This is an orphaned scaffold — not in workspace, has invalid edition "2024", contains no real code.

```bash
rm -rf HELIOS/simulation-cli/
```

### 2.5 Remove `DOCS/` directory (stale planning docs)

These are planning documents from early development that are now outdated:
```
DOCS/codebase_map.md
DOCS/deployment.md
DOCS/mobile_audit.md
DOCS/mobile_tokens.md
DOCS/perf_hotspots.md
DOCS/perf_ledger.md
DOCS/perf_measurement_guide.md
DOCS/performance_plan.md
DOCS/tdd.md
DOCS/traceability.md
DOCS/ux_mobile_plan.md
DOCS/workflow.md
```

```bash
git rm -r DOCS/
```

### 2.6 Remove stale blog posts

These blog posts were auto-generated content with no real value:
```
BLOG/posts/fcc-class-b-robotics-guide.md
BLOG/posts/iso-13482-personal-care-robots.md
BLOG/posts/robot-characterization-manufacturing-qa.md
BLOG/posts/vla-revolution.md
```

Also remove their image assets if present:
```
BLOG/images/fcc-guide/     (entire directory if exists)
```

Update `BLOG/posts/index.json` to remove entries for deleted posts.

### 2.7 Clean up ATLAS assets (large GeoJSON files)

Check if `ATLAS/assets/*.geojson` files are actually used by the current ATLAS WASM app. If ATLAS is currently just a scaffold, remove the large data files:
```
ATLAS/assets/countries_10m.geojson
ATLAS/assets/countries_50m.geojson
ATLAS/assets/countries_110m.geojson
ATLAS/assets/lakes_10m.geojson
ATLAS/assets/places_10m.geojson
ATLAS/assets/rivers_10m.geojson
ATLAS/assets/states_10m.geojson
```

**Validation:**
```bash
# After all removals
cargo check --workspace
cargo test --workspace
```

---

## 3. CARGO WORKSPACE CLEANUP

### 3.1 Add SIMULATIONS/HANDTRACK and SIMULATIONS/POWERLAW to workspace or exclude

Currently `SIMULATIONS/HANDTRACK/` declares its own `[workspace]` (standalone) and `SIMULATIONS/POWERLAW/` is not in the workspace either.

**Option A (recommended): Add to workspace**

Edit root `Cargo.toml`, add to `members`:
```toml
    # Simulations (L2 under Simulations bubble)
    "SIMULATION/CHLADNI",
    "SIMULATIONS/HANDTRACK",
    "SIMULATIONS/POWERLAW",
```

Then in `SIMULATIONS/HANDTRACK/Cargo.toml`, remove the `[workspace]` line (line 5). Update the `dna` dependency path to `../../DNA`.

In `SIMULATIONS/POWERLAW/Cargo.toml`, no `[workspace]` line to remove — it already uses relative path for `dna`.

**Option B: Exclude explicitly**

Add to root `Cargo.toml`:
```toml
exclude = ["HW/*", "SIMULATIONS/HANDTRACK", "SIMULATIONS/POWERLAW"]
```

### 3.2 Remove `HW/*` exclusion (directory doesn't exist)

The root `Cargo.toml` line 65 excludes `HW/*` but this directory does not exist. Remove:
```toml
# Before
exclude = ["HW/*"]
# After (remove the line entirely, or update if using Option B above)
```

### 3.3 Audit workspace members for orphaned LEARN/ML crates

`LEARN/ML/` and `LEARN/ML/antimony-core/` exist on disk but are NOT listed in workspace members. They have broken edition = "2024". Either:
- Add to workspace (after fixing edition)
- Or add to `exclude` list

**Validation:**
```bash
cargo check --workspace 2>&1 | head -20
# Should show zero "profiles" warnings and zero "not a member" warnings
```

---

## 4. DIRECTORY STRUCTURE NORMALIZATION

### 4.1 Merge `SIMULATIONS/` into `SIMULATION/`

Two directories exist with almost the same name:
```
SIMULATION/          # Contains CHLADNI/ and CORE/
SIMULATIONS/         # Contains HANDTRACK/ and POWERLAW/
```

Move `SIMULATIONS/HANDTRACK/` → `SIMULATION/HANDTRACK/`
Move `SIMULATIONS/POWERLAW/` → `SIMULATION/POWERLAW/`
Delete empty `SIMULATIONS/` directory.

**Update:**
- Root `Cargo.toml` workspace members (if added in 3.1)
- Any `dna` path dependencies in the moved Cargo.toml files
- `SCRIPTS/config.sh` (any references to SIMULATIONS path)
- Any imports or path references in `.rs` files

**Validation:**
```bash
cargo check --workspace
ls SIMULATIONS/ 2>&1  # Should say "No such file or directory"
```

### 4.2 Remove `temp/` directory reference in `.gitignore`

`.gitignore` already ignores `temp/`, so if `temp/AutoCrate/` exists on disk, it won't be tracked. Verify it's not tracked:
```bash
git ls-files temp/
# Should return nothing
```

If it IS tracked, run `git rm -r --cached temp/`.

---

## 5. DNA lib.rs DECOMPOSITION

### Current state
`DNA/src/lib.rs` is 1,813 lines. It contains:
- Module declarations and re-exports (lines 1-137)
- Inline struct definitions: `Obstacle`, `BoidRole`, `BoidState`, `Genome` (lines 142-500+)
- Full implementation blocks with logic (Genome::random, mutate, etc.)
- `BoidHandle`, `BoidArena`, `SpatialGrid` implementations
- Full simulation update logic

### Goal
`lib.rs` should ONLY contain `pub mod` declarations and `pub use` re-exports. All types and logic should live in their respective modules.

### Migration steps

**Step 1: Move core types to `DNA/src/sim/types.rs`**

Extract from `lib.rs`:
- `struct Obstacle`
- `enum BoidRole`
- `enum BoidState`
- `struct Genome` + all impl blocks
- `struct BoidHandle`

Create `DNA/src/sim/types.rs` with these types. Add `pub mod types;` to `DNA/src/sim/mod.rs`. Re-export from `lib.rs`:
```rust
pub use sim::types::{Obstacle, BoidRole, BoidState, Genome, BoidHandle};
```

**Step 2: Move BoidArena to `DNA/src/sim/boid_arena.rs`** (if not already there)

Check if `DNA/src/sim/boid_arena.rs` already has the implementation. If `lib.rs` has a duplicate or extended version, consolidate into the module file.

**Step 3: Move remaining inline code**

Any simulation update logic, helper functions, or constants in `lib.rs` should move to appropriate `sim/` submodules.

**Step 4: Clean up deprecated re-exports**

Remove the `#[deprecated]` module declarations for `spice` and `ekf` once all consumers are updated to use the new paths.

**Target: `lib.rs` should be under 200 lines** — only `pub mod` and `pub use` statements.

**Validation:**
```bash
cargo check --workspace
cargo test --workspace
wc -l DNA/src/lib.rs
# Should be < 200 lines
```

---

## 6. DEPENDENCY STANDARDIZATION

### 6.1 Pin wasm-bindgen to exact version everywhere

Three crates use unpinned `wasm-bindgen = "0.2"` instead of `"=0.2.93"`:

**Files to check and fix:**
```
BLOG/Cargo.toml               → change wasm-bindgen to use workspace version
SIMULATION/CHLADNI/Cargo.toml → change wasm-bindgen to use workspace version
LEARN/SENSORS/Cargo.toml      → change wasm-bindgen to use workspace version
```

Best approach: use `wasm-bindgen.workspace = true` in each crate's `[dependencies]`:
```toml
wasm-bindgen = { workspace = true }
```

### 6.2 Use workspace dependencies consistently

All crates should use `{ workspace = true }` for shared dependencies instead of specifying versions locally. The workspace already defines:
- `glam`, `rand`, `getrandom`, `wasm-bindgen`, `web-sys`, `serde`, `serde_json`, `dna`

Audit each `Cargo.toml` in the workspace and replace local version specs with workspace references where applicable.

**Validation:**
```bash
cargo check --workspace
# Verify no version conflict warnings
```

---

## 7. GITIGNORE HARDENING

### Current `.gitignore` (40 lines) is too permissive. Add:

```gitignore
# Binaries that should never be committed
*.deb
*.rpm
*.exe
*.dll
*.so
*.dylib

# Speech/ML models
nerd-dictation/
*.vosk
*.onnx

# Environment files
.env
.env.*
!.env.example

# Build caches
.trunk/
*.wasm.d

# Coverage
lcov.info
tarpaulin-report.html

# Editor state
.vscode/
!.vscode/settings.json
!.vscode/extensions.json

# Playwright
test-results/
playwright-report/
```

---

## 8. CI/CD IMPROVEMENTS

### 8.1 Add clippy to CI

Edit `.github/workflows/ci.yml` to add a clippy step:

```yaml
- name: Clippy
  run: cargo clippy --workspace -- -D warnings
```

This enforces the zero-warnings policy documented in CLAUDE.md.

### 8.2 Add format check to CI

```yaml
- name: Format check
  run: cargo fmt --check
```

### 8.3 Add cargo-deny to CI

```yaml
- name: Dependency audit
  run: |
    cargo install cargo-deny --locked
    cargo deny check
```

---

## 9. DOCUMENTATION CONSOLIDATION

### 9.1 Update CLAUDE.md port table

The port table in `CLAUDE.md` is stale. `SCRIPTS/config.sh` is the source of truth. Update the table to match:

**Ports to add/fix in CLAUDE.md:**
```
| python    | 8110 | http://127.0.0.1:8110 | Python tutorials        |
| handtrack | 8121 | http://127.0.0.1:8121 | Hand tracking           |
```

Remove the `coming_soon` port entry.

### 9.2 Update AGENTS.md directory structure

Add `SIMULATIONS/` (after merge) to the directory listing. Remove any references to worktrees (the project no longer uses them).

### 9.3 Remove REFACTOR.md after migration is complete

The existing `REFACTOR.md` describes a DNA architecture migration that is partially complete. After tasks in section 5 are done, update or remove this file.

### 9.4 Consolidate CLAUDE.md files

Every project has its own `CLAUDE.md`. These are useful but some are stale. For each, verify:
- Port numbers match `SCRIPTS/config.sh`
- Dependency lists are accurate
- Build commands work

**Files to audit:**
```
CLAUDE.md              (root)
AGENTS.md              (root)
DNA/CLAUDE.md
WELCOME/CLAUDE.md
HELIOS/CLAUDE.md
LEARN/CLAUDE.md
BLOG/CLAUDE.md
SIMULATION/CHLADNI/CLAUDE.md
TOOLS/AUTOCRATE/CLAUDE.md
ARCH/CLAUDE.md
```

---

## 10. CODE QUALITY SWEEP

### 10.1 Fix all clippy warnings

Run and fix:
```bash
cargo clippy --workspace -- -D warnings 2>&1
```

Known warnings (from current build):
- `python-learn`: 7 warnings (manual prefix stripping, split_once, for loop, unit let-bindings)
- `sensors`: 5 warnings (complex types, loop indexing)
- `opencv-learn`: 13 warnings (deref, loop indexing, Range::contains, prefix stripping)
- Various: derivable impls, needless reference borrows

**Auto-fix where possible:**
```bash
cargo clippy --fix --workspace --allow-dirty
```

Then manually review and fix what auto-fix couldn't handle.

### 10.2 Run cargo fmt

```bash
cargo fmt --all
```

### 10.3 Audit unsafe blocks

4 files use `unsafe`:
```
MCAD/src/renderer/buffers.rs
HELIOS/src/render_gl.rs
TOOLS/AUTOCRATE/src/render/mesh.rs
SIMULATION/CHLADNI/src/renderer.rs
```

These are likely WebGL buffer operations. Each `unsafe` block MUST have a `// SAFETY:` comment explaining why it's safe. Add comments where missing.

### 10.4 Address TODO/FIXME comments

There are 81 TODO/FIXME/HACK comments in the codebase. Triage each:
- If the TODO is done → remove the comment
- If the TODO is still valid → create a GitHub issue and reference it: `// TODO(#XX): description`
- If the TODO is stale → remove it

---

## 11. SECURITY HARDENING

### 11.1 Add Content Security Policy to all index.html files

Every WASM project's `index.html` should include:
```html
<meta http-equiv="Content-Security-Policy"
  content="default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self' https://*.too.foo; img-src 'self' data: blob:;">
```

**Files to update (all index.html files in WASM projects):**
```
WELCOME/index.html
HELIOS/index.html
BLOG/index.html
ARCH/index.html
MCAD/index.html
ATLAS/index.html
SIMULATION/CHLADNI/index.html
SIMULATIONS/HANDTRACK/index.html
SIMULATIONS/POWERLAW/index.html
TOOLS/AUTOCRATE/index.html
TOOLS/PLL/index.html
TOOLS/POWER_CIRCUITS/index.html
TOOLS/SPICE/index.html
LEARN/index.html
LEARN/AI/index.html
LEARN/UBUNTU/index.html
LEARN/OPENCV/index.html
LEARN/ARDUINO/index.html
LEARN/ESP32/index.html
LEARN/SWARM_ROBOTICS/index.html
LEARN/SLAM/index.html
LEARN/GIT/index.html
LEARN/DATA_STRUCTURES/index.html
LEARN/PYTHON/index.html
LEARN/SENSORS/index.html
```

### 11.2 Update deny.toml

Add explicit ban rules:
```toml
[bans]
multiple-versions = "warn"
deny = [
    # No runtime dependency on openssl (use rustls)
    { name = "openssl-sys" },
]
```

---

## 12. VALIDATION CHECKLIST

After ALL tasks are complete, run this full validation:

```bash
# 1. Type check (zero warnings)
cargo check --workspace 2>&1 | grep -c "warning:"
# Expected: 0

# 2. Clippy (zero warnings)
cargo clippy --workspace -- -D warnings

# 3. Format check
cargo fmt --check

# 4. Tests pass
cargo test --workspace

# 5. Security audit
cargo deny check

# 6. WASM builds (spot check key projects)
trunk build WELCOME/index.html
trunk build HELIOS/index.html
trunk build BLOG/index.html

# 7. No large files tracked
git ls-files | xargs ls -la 2>/dev/null | awk '{if ($5 > 1000000) print $5, $9}'
# Expected: empty (no files > 1MB)

# 8. No stale warnings
cargo check --workspace 2>&1 | grep "profiles for the non root"
# Expected: empty

# 9. Verify directory structure
ls SIMULATIONS/ 2>&1   # Should not exist
ls COMING_SOON/ 2>&1   # Should not exist
ls DOCS/ 2>&1           # Should not exist
ls HELIOS/simulation-cli/ 2>&1  # Should not exist

# 10. Verify lib.rs size
wc -l DNA/src/lib.rs
# Expected: < 200 lines
```

---

## EXECUTION ORDER

For Codex, execute in this order (each task should be a separate commit):

| Order | Section | Commit message prefix | Risk |
|-------|---------|----------------------|------|
| 1 | 1.1 | `fix(cargo): correct Rust edition to 2021` | Low |
| 2 | 1.2 | `fix(cargo): move profiles to workspace root` | Low |
| 3 | 2.1-2.7 | `chore: remove dead files and stale content` | Low |
| 4 | 7 | `chore: harden .gitignore` | Low |
| 5 | 3.1-3.3 | `fix(cargo): normalize workspace members` | Medium |
| 6 | 4.1 | `refactor: merge SIMULATIONS into SIMULATION` | Medium |
| 7 | 6.1-6.2 | `fix(deps): standardize workspace dependencies` | Medium |
| 8 | 10.1-10.2 | `style: fix clippy warnings and format` | Low |
| 9 | 10.3 | `docs: add SAFETY comments to unsafe blocks` | Low |
| 10 | 5 | `refactor(dna): decompose lib.rs into modules` | High |
| 11 | 8.1-8.3 | `ci: add clippy, fmt, and deny checks` | Low |
| 12 | 11.1-11.2 | `security: add CSP headers and update deny.toml` | Low |
| 13 | 9.1-9.4 | `docs: consolidate and update documentation` | Low |
| 14 | 10.4 | `chore: triage TODO/FIXME comments` | Low |

**After each commit, run:** `cargo check --workspace && cargo test --workspace`

---

## CONSTRAINTS FOR CODEX

1. **Never modify simulation logic** — this is a refactor, not a feature change
2. **Never change public API signatures** — downstream code must still compile
3. **Always use backward-compatible re-exports** when moving types between modules
4. **One commit per logical change** — atomic, reviewable, revertible
5. **Run `cargo check --workspace` after every file edit** — catch breakage immediately
6. **Do not add new dependencies** — the project philosophy is minimal external deps
7. **Do not create new documentation files** unless explicitly listed above
8. **Preserve all file headers** (the `//! ═══` banner pattern) when moving code
9. **Use `Refs #XX`** in commits (not `Closes #XX`) to avoid auto-closing issues
