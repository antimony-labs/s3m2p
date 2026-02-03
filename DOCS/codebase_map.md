# Codebase Map (S3M2P)

Purpose
- Quick, human-readable map of the repo and how each surface runs.
- Intended for onboarding and architectural context.

Top-level layout
- DNA: shared algorithms, physics, math, exporters (Rust library)
- WELCOME: landing page (WASM + Canvas2D + DOM)
- HELIOS: solar system visualization (WASM + Canvas2D)
- SIMULATION/CHLADNI: chladni plate visualization (WASM + WebGL)
- TOOLS: user-facing tools (AUTOCRATE, PLL, POWER_CIRCUITS, SPICE)
- LEARN: learning hub and tutorials
- BLOG: markdown blog engine
- ARCH: architecture explorer visualization
- ATLAS: vector map system
- MCAD: mechanical CAD (WASM + WebGL)
- SIMULATIONS: experimental demos

Common runtime pattern
- Trunk builds Rust to WASM
- Each surface owns a render loop (requestAnimationFrame)
- Canvas/WebGL for visuals; DOM for controls and routing

Surface notes

WELCOME
- Purpose: landing + navigation hub
- Entry: WELCOME/index.html, WELCOME/src/main.rs
- Stack: Canvas2D + DOM routing + WASM
- Heavy systems: boids, fungal network, background shader

HELIOS
- Purpose: heliosphere visualization
- Entry: HELIOS/index.html, HELIOS/src/main.rs
- Stack: Canvas2D + WASM
- Heavy systems: star data manager, projection, overlays

SIMULATION/CHLADNI
- Purpose: wave pattern simulation
- Entry: SIMULATION/CHLADNI/index.html, SIMULATION/CHLADNI/src/lib.rs
- Stack: WebGL + WASM

TOOLS
- Purpose: interactive tools (design, simulation)
- Entry: per tool index.html + src/lib.rs
- Stack: Canvas2D/WebGL + WASM + DOM
- Notable: AUTOCRATE (crate geometry), PLL (math + plots), POWER_CIRCUITS

LEARN
- Purpose: educational hub and lessons
- Entry: LEARN/index.html, LEARN/*/index.html
- Stack: DOM + WASM; some demos use Canvas/WebGL
- Goal: readability-first, mobile friendly content

BLOG
- Purpose: markdown posts + routing
- Entry: BLOG/index.html, BLOG/src/lib.rs
- Stack: DOM + WASM

ARCH
- Purpose: dependency graph visualization
- Entry: ARCH/index.html, ARCH/src/lib.rs
- Stack: Canvas2D + WASM

ATLAS
- Purpose: vector maps
- Entry: ATLAS/index.html, ATLAS/src
- Stack: Canvas2D + WASM

MCAD
- Purpose: mechanical CAD
- Entry: MCAD/index.html, MCAD/src
- Stack: WebGL + WASM

SIMULATIONS
- Purpose: experimental demos
- Entry: per simulation index.html

Development
- Dev server: ./SCRIPTS/dev up <project>
- Build: trunk build <PROJECT>/index.html

Notes on reuse
- DNA: pure logic only, no UI
- CORE: domain engines that can be reused across tools
- Surfaces: thin UI shells on top of DNA/CORE
