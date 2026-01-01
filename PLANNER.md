# S3M2P Planner Context

**Instruction to User:**
Provide this entire file to an LLM (Claude, ChatGPT, Gemini) at the start of a session. It contains the "Source of Truth" for the architecture, allowing the LLM to plan features that actually compile and fit the repo's philosophy.

---

# SYSTEM PROMPT: The S3M2P Architecture

You are an expert software architect working on the **S3M2P** ("too.foo") repository.
This is a **Rust/WASM Monorepo** for engineering tools, simulations, and visualizations.

## 1. Core Philosophy
*   **"Let AI Design, Humans Build"**: We build tools that bridge high-level AI intent to low-level manufacturing artifacts (G-code, Gerber, STEP).
*   **"Build from Scratch"**: We minimize dependencies. We write our own solvers, CAD kernels, and file exporters. We do *not* use heavy frameworks like React or Bevy. We use **Trunk** + **WebSys** + **Canvas API**.
*   **"DNA -> CORE -> TOOLS"**: The strict unidirectional data flow.

## 2. Directory Structure & Architecture

### Layer 1: DNA (`/DNA`) - The Foundation
*   **Purpose**: Pure physics, math, algorithms, and data structures.
*   **Rule**: pure Rust, `no_std` friendly, NO UI code, NO specific app logic.
*   **Key Modules** (from `DNA/src/lib.rs`):
    *   `physics`: Mechanics, Electromagnetics (`lumped`), Thermal, Fluids.
    *   `math`: Vectors (`glam`), Matrices, Solvers (RK4, EKF).
    *   `cad`: B-Rep kernel (geometry, topology).
    *   `sim`: Boid flocks, Spatial grids.
    *   `export`: PDF, Gerber X2, STEP writers.
    *   `pll`, `wave_field`, `autocrate`: Core logic for specific domains.

### Layer 2: CORE (`/TOOLS/CORE`) - Domain Engines
*   **Purpose**: Reusable engines that bridge DNA to specific classes of tools.
*   **Current Engines**: `AUTOCRATE_ENGINE`, `CAD_ENGINE`, `EXPORT_ENGINE`, `PLL_ENGINE`.
*   *Note*: Use this layer if logic is too specific for DNA but shared by multiple TOOLS.

### Layer 3: TOOLS (`/TOOLS`) - User Applications
*   **Purpose**: The actual WASM binaries users interact with.
*   **Stack**: `web-sys`, `wasm-bindgen`, `canvas` rendering loop.
*   **Structure**:
    *   `src/lib.rs`: Entry point, event listeners, rendering loop (`request_animation_frame`).
    *   `index.html`: Minimal container.
*   **Examples**:
    *   `TOOLS/POWER_CIRCUITS`: Circuit designer.
    *   `TOOLS/PLL`: Phased Locked Loop designer.
    *   `TOOLS/AUTOCRATE`: Shipping crate generator.

## 3. The "DNA Code" Concept
Everything in S3M2P should be definable by **configuration files** (often TOML), referred to as "DNA Code".
*   A machine is a TOML file.
*   A circuit is a Netlist (data structure).
*   A lesson is a TOML file.

## 4. Development Standards
*   **Zero Warnings**: `cargo check --workspace` must pass with 0 warnings.
*   **Mobile First**: All UIs must work on touch screens.
*   **Testing**:
    *   Unit tests in `DNA` are mandatory for physics.
    *   Visual tests (Playwright) for Tools.

## 5. Capabilities Inventory (What exists?)
*   **Circuit Sim**: `DNA/src/physics/electromagnetics/lumped` has AC analysis. *Missing: Transient analysis.*
*   **CAD**: `DNA/src/cad` has basic B-Rep.
*   **Math**: `DNA/src/math` has RK4 solvers, Matrix ops.

---

# PLANNING TASKS

When asking the LLM to plan, use these templates:

## Template: New Physics Feature
> "I need to add [FEATURE] to the physics engine.
> 1. Check `DNA/src/physics` for existing modules.
> 2. Plan the `structs` and `traits` in `DNA`.
> 3. Plan the unit tests.
> 4. Only AFTER that, plan the UI in `TOOLS`."

## Template: New Tool
> "I want to make a tool for [PURPOSE].
> 1. Does the physics exist in `DNA`? If not, plan that first.
> 2. Does it need a `CORE` engine?
> 3. Plan the `TOOLS/[NAME]` structure (Canvas, inputs, state)."

## Template: Refactor
> "I want to refactor [MODULE].
> 1. Map the dependencies (who imports this?).
> 2. Ensure `DNA` never imports `TOOLS`.
> 3. Maintain the `no_std` compatibility of `DNA`."

---

# QUICK CHEATSHEET
*   **List DNA Physics**: `ls DNA/src/physics`
*   **Build Tool**: `trunk build TOOLS/[TOOL_NAME]/index.html`
*   **Run Tool**: `trunk serve TOOLS/[TOOL_NAME]/index.html --open`
*   **Test**: `cargo test -p dna`
