# Performance and Optimization Plan (Platform-wide)

Purpose
- Build a complete understanding of the codebase and runtime behavior.
- Establish performance baselines and budgets.
- Improve real and perceived performance across all surfaces.
- Extract reusable, performance-safe libraries for long-term velocity.

Scope (all user-facing surfaces)
- WELCOME, HELIOS, ARCH, BLOG, LEARN, MCAD, ATLAS, SIMULATION/CHLADNI
- TOOLS/* (AUTOCRATE, PLL, POWER_CIRCUITS, SPICE, SIMULATION_CLI if UI)
- SIMULATIONS/* (HANDTRACK, POWERLAW, etc)
- Shared DNA/CORE libraries and SCRIPTS build/deploy tooling

Artifacts and tracking
- DOCS/perf_ledger.md (baseline metrics table and notes).
- DOCS/perf_hotspots.md (Top 10 list with ROI score).
- DOCS/perf_measurement_guide.md (how to capture baseline metrics).
- Performance budget policy (added to this doc after Phase 1).

Non-goals
- No optimization without a recorded baseline.
- No behavior changes that alter visual output without explicit sign-off.

----------------------------------------------------------------
Codebase understanding (map)

Architecture layers
- DNA: shared algorithms, math, physics, exporters, simulation primitives
- CORE: domain engines (project-specific reusable logic)
- Surfaces: WASM apps per project (web-sys + canvas or WebGL)

Runtime model (common pattern)
- Trunk builds a Rust WASM app that owns a render loop (requestAnimationFrame)
- Canvas or WebGL draws per frame; DOM is used for UI overlays and routing
- Some surfaces also parse content at runtime (BLOG, LEARN)

Shared performance risks
- Per-frame allocations inside render loops (Vec::new, collect, format)
- Repeated DOM reads and layout thrash
- Full-canvas redraws each frame even when static
- Unbounded growth of data structures (nodes, particles, logs)
- Large WASM or asset payloads (fonts, images, large JSON)

Measurement approach (baseline)
- Define a standard scenario per surface (idle + typical interaction).
- Use a consistent device matrix (low-end phone, mid laptop, desktop).
- Capture load: first paint, first interaction, total load time.
- Capture runtime: average FPS + 1% low, memory at 60s/120s.
- Record wasm/JS/CSS/fonts/images sizes and cache headers.
- Store raw numbers + notes in the perf ledger.

Triage rubric (Top 10 list)
- Impact (1-5): how much user experience improves.
- Effort (1-5): estimate of change complexity.
- Risk (1-5): likelihood of regression.
- ROI score: Impact / Effort, then adjust down for high risk.

----------------------------------------------------------------
Surface inventory with likely hotspots

WELCOME (landing)
- Stack: Rust/WASM + Canvas2D + DOM routing
- Heavy loop: boids + fungal network + background + telemetry
- Risks: per-frame allocations, random generator in loops, redundant color math
- Key files: WELCOME/src/main.rs, WELCOME/src/fungal.rs, WELCOME/src/shader.rs

HELIOS (solar system)
- Stack: Rust/WASM + Canvas2D
- Heavy loop: star rendering + celestial projection + overlays
- Risks: star list rebuild, color conversions per frame, expensive text overlays
- Key files: HELIOS/src/main.rs, HELIOS/src/render.rs, HELIOS/src/star_data.rs

ARCH (architecture explorer)
- Stack: Rust/WASM + Canvas2D
- Risks: full canvas redraw on every pointer move, large node graph
- Key files: ARCH/src/lib.rs, ARCH/src/graph.rs

BLOG
- Stack: Rust/WASM + DOM rendering, markdown
- Risks: markdown parse time, large post rendering, image payloads
- Key files: BLOG/src/lib.rs, BLOG/src/render.rs, BLOG/posts/*

LEARN (hub + tutorials)
- Stack: Rust/WASM + DOM + per-demo Canvas/WebGL
- Risks: demo code running even when offscreen, heavy math in tutorials
- Key files: LEARN/index.html, LEARN/AI, LEARN/SLAM, LEARN/ESP32, etc

MCAD
- Stack: Rust/WASM + WebGL, geometry kernel
- Risks: geometry rebuilds, mesh allocations, GPU upload churn
- Key files: MCAD/src, MCAD/CORE/CAD_ENGINE

ATLAS (vector maps)
- Stack: Rust/WASM + Canvas2D
- Risks: path tessellation, redraw on pan/zoom, large datasets
- Key files: ATLAS/src

SIMULATION/CHLADNI
- Stack: Rust/WASM + WebGL
- Risks: shader complexity, particle count, CPU-GPU sync
- Key files: SIMULATION/CHLADNI/src

TOOLS/*
- AUTOCRATE: UI + geometry calculation, mostly CPU-bound
- PLL: math heavy, plots, dynamic UI
- POWER_CIRCUITS: likely graph rendering and interaction
- SPICE: eventual heavy solver + plotting
- Risks: UI input handling, graph redraws, CPU-bound solvers

SIMULATIONS/*
- Experimental demos, often heavy per-frame loops
- Risks: not tuned for low-end devices, large assets

----------------------------------------------------------------
Phase 1 - Inventory and baseline (documentation-driven)

1) Build a performance ledger
- For each surface, record:
  - wasm size, JS size, CSS size, fonts/images
  - first paint, first interaction, total load time
  - FPS under standard scenario
  - memory usage after 60s and 120s
- Store results in a table (new doc: DOCS/perf_ledger.md)
 - Include the scenario definition and device used for each measurement

2) Identify top 10 hotspots
- Use code review + lightweight instrumentation
- Prioritize by user impact and ease of fix

Deliverables
- PERF ledger (baseline metrics)
- Top 10 improvement list with ROI score

Exit criteria
- All priority surfaces measured at least once.
- Top 10 list agreed with a clear next surface owner.

----------------------------------------------------------------
Phase 2 - Build and asset pipeline optimization

Targets
- Smaller payloads, faster first render
- Consistent caching strategy across all surfaces

Checklist
- Trunk config audit (WASM opt-level, wasm-bindgen settings)
- Enable compression (gzip + brotli) for wasm/assets
- Font subsetting and modern formats
- Image format migration (AVIF/WEBP) where possible
- Split large content (BLOG posts, LEARN lessons)

Deliverables
- Reduced asset size and faster first paint
- Build docs updated with new asset rules

----------------------------------------------------------------
Phase 3 - Runtime hot-path optimization (per surface)

Common patterns to apply
- Reuse scratch vectors and buffers
- Avoid per-frame string formatting
- Cache expensive computations (colors, shapes, text)
- Gate DOM reads to user events or 1Hz intervals
- Use offscreen canvas for static layers
- Implement frame-skipping for non-critical layers

Instrumentation quick wins (during Phase 3)
- Add lightweight FPS and frame-time sampling to each render loop.
- Track allocation-heavy paths with simple counters or debug toggles.
- Log steady-state memory after 60s/120s for validation.

Surface-specific initial targets
- WELCOME: remove per-frame Vec allocs, single RNG per frame, cache colors
- HELIOS: star list caching and LOD gating, reduce text/overlay redraws
- ARCH: draw only on state change, cache static graph
- BLOG: precompute markdown, lazy-load images
- LEARN: pause demos when hidden, reduce redraw rate offscreen
- MCAD: batch GPU uploads, cache tessellations
- ATLAS: tile rendering and LOD for vector data
- CHLADNI: decouple simulation step rate from render

Deliverables
- FPS improvements per surface
- Memory stability improvements (no growth in steady state)

----------------------------------------------------------------
Phase 4 - UX performance (perceived speed)

- Skeleton and progressive rendering for large pages
- Idle-time prefetch of the next likely surface
- Reduced-motion mode for low-power devices
- Graceful fallback for low-end GPUs

Deliverables
- Documented UX perf patterns
- Consistent feel across all surfaces

----------------------------------------------------------------
Phase 5 - Reusable performance utilities

Extract common utilities into DNA/CORE or a shared crate
- rng.rs (fast deterministic RNG for visuals)
- ring_buffer.rs (telemetry, FPS, rolling stats)
- frame_budget.rs (frame timing and throttling)
- draw_cache.rs (offscreen canvas caching helpers)
- asset_loader.rs (lazy load images/fonts)

Deliverables
- Shared crate modules used across multiple surfaces

----------------------------------------------------------------
Phase 6 - Guardrails and budgets

Budgets to enforce
- wasm size per surface
- total asset payload per surface
- max frame time for main loop
- memory growth threshold per 60s
- budgets set after Phase 1 baseline

Automated checks
- simple script to fail build if budgets exceeded
- optional Playwright smoke perf checks

Deliverables
- Performance budget policy
- CI or local scripts for guardrails

----------------------------------------------------------------
Tech stack evaluation (HTML/CSS/Node question)

Current stack strengths
- Rust/WASM is excellent for heavy simulation, geometry, and deterministic logic
- Shared DNA/CORE libraries are a long-term asset
- Canvas/WebGL lets us do rich visuals without heavy JS frameworks

Where a lighter stack helps
- Content-heavy pages (BLOG, LEARN hub) can be pure HTML/CSS/JS
- Node is useful for build tooling and static generation
- Static content reduces JS/WASM payloads and speeds first render

Recommended direction
- Keep Rust/WASM for heavy interactive surfaces (WELCOME, HELIOS, MCAD, CHLADNI)
- Use static HTML/CSS/JS (or minimal JS) for content surfaces where possible
- Consider generating BLOG and LEARN content at build time to avoid runtime parse
- Avoid a full rewrite; do a hybrid approach to keep momentum and reuse

Decision gate
- After baseline metrics, decide per surface whether a lighter stack materially
  improves UX. Only migrate surfaces that are content-first and not simulation-first.

----------------------------------------------------------------
Mobile and readability priority

- Mobile-first layout should be the default; desktop is an enhancement.
- Reduce motion and background noise on small screens.
- Provide readable typography (16-18px base, 1.6-1.8 line height).
- Use stacked widgets for learning content and progressive disclosure.
- Ensure all controls are reachable (44px target size).

See also
- DOCS/codebase_map.md (repo overview)
- DOCS/ux_mobile_plan.md (mobile + readability plan)
