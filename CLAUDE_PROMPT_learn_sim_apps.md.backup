## CLAUDE ORCHESTRATION BRIEF — LEARN simulation apps (Rust/WASM)

### Mission
Build **four independent Trunk + Rust/WASM tutorial apps** that teach via **interactive simulations** (not static pages):
- **ML**: `LEARN/` (Zero to AGI curriculum)
- **SLAM**: `LEARN/SLAM/`
- **ESP32**: `LEARN/ESP32/`
- **Ubuntu**: `LEARN/UBUNTU/`

Each app must ship:
- **Home**: lesson/lab list
- **Lesson view**: short explanation + **controls** + **live canvas simulation** with **play/pause/step/reset**

### Quick start (for implementer)
1. **Start with Milestone 1** (shared crates foundation)
   - Create `LEARN/learn_core/` and `LEARN/learn_web/` crates
   - Wire them as path dependencies in all four apps' `Cargo.toml`
   - Test by creating a minimal "simulation runner" pattern and wiring it into ML app (even if demo is placeholder)

2. **Then proceed sequentially** through milestones 2-5
   - Each milestone should be fully working before moving to the next
   - Test each app independently: `cd LEARN && trunk serve --open` (port 8086)

3. **Verify MVP criteria** after milestone 5
   - All apps load, navigate, and run simulations
   - Controls are responsive (no lag)
   - Core logic has unit tests

### Key constraints
- **Rust-first simulations**: simulation/state logic in Rust.
- **No real shell execution** in Ubuntu app (use an in-memory model).
- **Client-side only** (static hosting compatible).
- Prefer **2D canvas** first; WebGL is optional.

### Reuse existing patterns (do not reinvent)
- **Hash routing + UI state patterns**: `WELCOME/src/main.rs`
- **Hi‑DPI canvas resizing + render loop**: `LEARN/SENSORS/src/lib.rs`
- **ML lesson shell already exists** (cards + lesson view + KaTeX + canvas placeholder):
  - `LEARN/src/lib.rs`
  - `LEARN/src/render.rs`
  - `LEARN/src/lessons.rs`
- **ML “source of truth” implementations + lesson JSON artifacts** exist in `LEARN/ML/` (use for reference only).

### Architectural decision (important)
These are **separate apps**. However, avoid copy/paste by adding shared crates used by all four.

### Shared crates to create
Create two small path-dependency crates:

#### 1) `LEARN/learn_core/` (pure Rust, no `web-sys`)
**Purpose**: Pure simulation logic that can be tested without WASM.

**Contents**:
- **RNG**: Deterministic seeded RNG (LCG or similar). Must support `seed(u64)` and `gen::<f64>()`, `gen_range()`. See `WELCOME/src/main.rs` for `rand` usage patterns.
- **Math helpers**: 
  - `Vec2 { x: f32, y: f32 }` with basic ops (add, sub, mul, dot, length, normalize)
  - `clamp`, `lerp`, `smoothstep` utilities
  - Keep it minimal; don't pull in `glam` or `nalgebra` unless absolutely needed
- **Simulation trait**:
  ```rust
  pub trait Sim {
      type State;
      fn reset(&mut self, seed: u64) -> Self::State;
      fn step(&mut self, state: &mut Self::State, dt: f32);
      // Note: rendering is separate (handled by learn_web)
  }
  ```
- **Unit tests**: All math/RNG logic must have tests. Run with `cargo test` (native).

**Dependencies**: Only `rand` (or implement minimal LCG). No `wasm-bindgen`, no `web-sys`.

#### 2) `LEARN/learn_web/` (WASM glue, minimal DOM helpers)
**Purpose**: Thin WASM bindings for common patterns (avoid copy/paste across apps).

**Contents**:
- **Hi‑DPI canvas wrapper**: 
  - Pattern from `LEARN/SENSORS/src/lib.rs` lines 416-439
  - Handles `devicePixelRatio` scaling automatically
  - Returns `CanvasRenderingContext2d` ready to use
- **Animation loop runner**:
  - `AnimationLoop::new(callback)` with `start()`, `stop()`, `pause()`
  - Uses `requestAnimationFrame` internally
  - Pattern similar to `WELCOME/src/main.rs` lines 1013-1014, 1786-1788
- **DOM utilities**:
  - `get_element_by_id<T>()` with type casting
  - `set_text_content(id, text)`, `set_attribute(id, key, value)`
  - `wire_input_range(id, callback)` for sliders
  - `wire_button(id, callback)` for buttons
- **Hash routing helpers**:
  - `get_current_route() -> Option<Route>` (parse `window.location.hash`)
  - `navigate_to(route)` (set hash, trigger event)
  - Pattern from `WELCOME/src/routing.rs` (if exists) or `WELCOME/src/main.rs` lines 729-757

**Dependencies**: `wasm-bindgen`, `web-sys` (minimal feature set), `js-sys`.

### Per-app code layout (target shape)
For each app folder (`LEARN/`, `LEARN/SLAM/`, `LEARN/ESP32/`, `LEARN/UBUNTU/`):
- `src/lessons.rs`: lesson metadata + which demo to run
- `src/render.rs` (or expand `src/lib.rs`): render home/lesson views + controls
- `src/demos/`: one Rust demo per lesson

### Simulation runner pattern (critical)
Each app needs a **reusable simulation runner** that handles:
1. **State management**: Keep simulation state alive across navigation (don't recreate on every render)
2. **Control binding**: Wire sliders/buttons to simulation parameters
3. **Animation loop**: Start/stop/pause/step the simulation
4. **Canvas rendering**: Call demo's render function with current state

**Suggested structure** (implement once in `learn_web`, reuse everywhere):
```rust
pub struct SimRunner<Sim: learn_core::Sim> {
    sim: Sim,
    state: Sim::State,
    loop_handle: AnimationLoop,
    canvas: CanvasWrapper,
    is_paused: bool,
    step_mode: bool, // If true, advance one step per button click
}

impl<Sim: learn_core::Sim> SimRunner<Sim> {
    pub fn new(sim: Sim, canvas_id: &str) -> Result<Self, JsValue>;
    pub fn reset(&mut self, seed: u64);
    pub fn set_param(&mut self, param_name: &str, value: f32);
    pub fn toggle_pause(&mut self);
    pub fn step_once(&mut self); // For step mode
    pub fn render(&mut self); // Called by animation loop
}
```

**Usage in app**:
- Create `SimRunner` when lesson loads
- Store in `Rc<RefCell<SimRunner>>` or thread-local (like `LEARN/SENSORS/src/lib.rs` line 73-75)
- Wire controls to `set_param()` / `reset()` / `toggle_pause()`
- Animation loop calls `render()` which calls demo's render function

**Reference**: Look at `WELCOME/src/main.rs` for persistent state pattern (lines 986-1003, 1031-1784).

### MVP milestone order (do in this sequence)

#### Milestone 1 — Shared foundation
- Add `LEARN/learn_core` + `LEARN/learn_web` and wire them as path deps.
- Implement the “simulation lesson runner” once (controls + loop + render + routing) and reuse it.

#### Milestone 2 — ML app: first real interactive demo
- Upgrade `LEARN/` so demos are not “coming soon”.
- Implement **Lesson 1: Linear Regression (gradient descent)** demo:
  - Controls: learning rate, noise, seed/reset, play/pause/step
  - Visualization: points + fitted line + loss plot/trace

#### Milestone 3 — SLAM app: flagship sim
- Replace “Coming Soon” in `LEARN/SLAM/` with home + lesson view.
- Implement **2D robot localization (Particle Filter)**:
  - Controls: particle count, motion noise, sensor noise, resample threshold
  - Visualization: true pose, noisy odom, particles, estimated pose

#### Milestone 4 — ESP32 app: flagship sim
- Replace “Coming Soon” in `LEARN/ESP32/` with home + lesson view.
- Implement **GPIO button debouncing** lab:
  - Simulate a bouncing signal; show raw vs debounced + LED state timeline
  - Controls: bounce severity, sampling interval, debounce window

#### Milestone 5 — Ubuntu app: safe terminal simulation
- Replace “Coming Soon” in `LEARN/UBUNTU/` with home + lesson view.
- Implement **permissions/ownership** lab:
  - In-memory FS model (users, groups, perms)
  - Command subset: `ls -l`, `chmod`, `chown`, `mkdir`, `touch`, `cat` (simulated)
  - Exercises: scripted prompts + immediate feedback

### Dependency version alignment
**Critical**: All apps must use **consistent versions** to avoid conflicts.

- **wasm-bindgen**: Use `=0.2.93` (matches existing `LEARN/ESP32/Cargo.toml`, `LEARN/UBUNTU/Cargo.toml`)
- **web-sys**: Use `0.3` (matches existing)
- **js-sys**: Use `0.3` (matches existing)
- **rand**: Use `0.8` (matches `WELCOME/Cargo.toml`) or implement minimal LCG in `learn_core`
- **console_error_panic_hook**: Use `0.1` (matches existing)

**For `learn_core`**: Only `rand` (or no deps if implementing LCG). No WASM deps.

**For `learn_web`**: `wasm-bindgen = "=0.2.93"`, `web-sys = "0.3"`, `js-sys = "0.3"`.

**For each app**: Depend on `learn_core = { path = "../learn_core" }` and `learn_web = { path = "../learn_web" }`.

### Build/run reminders
Each is an independent Trunk project:
- `LEARN/` serves on **8086** (`LEARN/Trunk.toml`)
- `LEARN/UBUNTU/` serves on **8101**
- `LEARN/ESP32/` serves on **8104**
- `LEARN/SLAM/` serves on **8106`

**Commands**:
```bash
# ML app
cd LEARN && trunk serve --open

# SLAM app
cd LEARN/SLAM && trunk serve --open

# ESP32 app
cd LEARN/ESP32 && trunk serve --open

# Ubuntu app
cd LEARN/UBUNTU && trunk serve --open
```

**Note**: Trunk will auto-rebuild on file changes. Check browser console for WASM errors.

### Testing strategy

#### Unit tests (required)
- **`learn_core`**: All math/RNG logic must have unit tests
  - Run with `cargo test` (native, no WASM)
  - Test deterministic RNG with fixed seeds
  - Test Vec2 operations, clamp, lerp edge cases
  - Example: `assert_eq!(rng.seed(42).gen::<f64>(), expected_value)`

- **Demo logic**: Each demo's core algorithm should be testable
  - Extract pure functions (e.g., `gradient_descent_step()`, `particle_filter_update()`)
  - Test with known inputs → expected outputs
  - Mock canvas if needed (pass a trait object or `()` renderer)

#### Integration tests (manual, for now)
- **Each app**: Manual testing checklist
  - [ ] App loads without console errors
  - [ ] Home page shows lesson list
  - [ ] Clicking lesson navigates correctly
  - [ ] Simulation starts automatically (or on play)
  - [ ] Controls update simulation in real-time
  - [ ] Reset button works
  - [ ] Pause/play works
  - [ ] Step mode works (if implemented)
  - [ ] Simulation runs at ~60fps (check with browser DevTools Performance tab)
  - [ ] No memory leaks (check heap snapshot over 1 minute)

#### Performance targets
- **Frame time**: < 16ms per frame (60fps)
- **WASM size**: Keep each app under 500KB (gzipped) if possible
- **Memory**: Avoid per-frame allocations; use object pools or pre-allocated buffers
- **Controls**: Slider changes should reflect in simulation within 1 frame

### Definition of done (MVP)
- All four apps:
  - Home ⇄ Lesson navigation works
  - Simulation runs at interactive framerate (~60fps)
  - Controls include reset + at least **two** tunable params
- Core logic has tests in `LEARN/learn_core` (run `cargo test` to verify)
- No placeholder “Coming Soon” remains for the above flagship lessons.

### Notes
- Keep lesson text brief; the simulation should teach by doing.
- Prefer deterministic sims (seedable) so behavior is reproducible.
- Keep WASM allocations low; avoid per-frame heap churn.
- Use existing code patterns from `WELCOME/` and `LEARN/SENSORS/` as reference.


### Common pitfalls to avoid

#### Architecture mistakes
- ❌ **Don't recreate canvas/DOM helpers** that exist in `learn_web` (check first!)
- ❌ **Don't put simulation logic in WASM-only code** (use `learn_core` for pure Rust)
- ❌ **Don't recreate simulation state on every render** (use persistent state like `Rc<RefCell<>>` or thread-local)
- ❌ **Don't mix routing logic** — use `learn_web` hash router consistently

#### Performance mistakes
- ❌ **Don't allocate per-frame** (e.g., `Vec::new()` in render loop)
  - ✅ Pre-allocate buffers or use object pools
  - ✅ Reuse `Vec` with `clear()` instead of creating new ones
- ❌ **Don't update DOM every frame** (batch updates or throttle)
  - ✅ Only update text/numbers when values actually change
  - ✅ Use `requestAnimationFrame` for canvas, not DOM
- ❌ **Don't forget Hi‑DPI scaling** (use `learn_web` canvas wrapper)

#### Testing mistakes
- ❌ **Don't forget to test deterministic RNG** with fixed seeds
- ❌ **Don't test WASM code** in unit tests (extract pure functions to `learn_core`)
- ❌ **Don't skip edge cases** (zero values, negative values, NaN handling)

#### WASM-specific gotchas
- ❌ **Don't use `println!`** in WASM (use `web_sys::console::log_1`)
- ❌ **Don't forget `console_error_panic_hook::set_once()`** in `start()` function
- ❌ **Don't use `std::time`** (use `web_sys::Performance` or `js_sys::Date`)
- ❌ **Don't forget to handle `JsValue` errors** (unwrap carefully or use `?`)

### Future enhancements (post-MVP)
These are **not required** for MVP but good to keep in mind:

#### User experience
- **Progress tracking**: Save completed lessons to `localStorage`
- **Code syntax highlighting**: Add syntax highlighting to code examples in lesson text (use `highlight.js` or similar)
- **Export/import simulation state**: Allow users to save/load simulation configurations
- **Mobile touch controls**: Optimize controls for touch devices (larger hit areas, swipe gestures)

#### Technical improvements
- **WebGL renderer**: Optional WebGL2 renderer for complex 3D visualizations (SLAM could benefit)
- **Web Workers**: Offload heavy computation to workers (e.g., particle filter with 10k+ particles)
- **Shared state between apps**: Allow linking from one app's lesson to another (e.g., ML → SLAM)
- **Accessibility**: Add ARIA labels, keyboard navigation, screen reader support

#### Content expansion
- **More lessons per app**: Expand each app beyond the flagship demo
- **Interactive exercises**: Add "challenges" where users must tune parameters to achieve goals
- **Explanatory overlays**: Click-to-reveal tooltips explaining concepts as simulation runs
- **Comparison mode**: Side-by-side comparison of different algorithms (e.g., Q-Learning vs Policy Gradients)

### Implementation tips

#### Getting started checklist
1. [ ] Read `WELCOME/src/main.rs` to understand routing + state patterns
2. [ ] Read `LEARN/SENSORS/src/lib.rs` to understand canvas + animation loop
3. [ ] Create `LEARN/learn_core/Cargo.toml` with minimal deps
4. [ ] Create `LEARN/learn_web/Cargo.toml` with WASM deps
5. [ ] Wire both into ML app's `Cargo.toml` as path deps
6. [ ] Implement minimal `SimRunner` in `learn_web`
7. [ ] Test by creating a trivial demo (e.g., bouncing ball) in ML app
8. [ ] Once pattern works, proceed to Milestone 2

#### Debugging WASM
- Use `console.log()` via `web_sys::console::log_1(&format!(...).into())`
- Check browser DevTools → Sources → `wasm://` for WASM source maps
- Use `console_error_panic_hook` to see Rust panics in console
- Profile with Chrome DevTools → Performance tab (record while simulation runs)

#### Code organization
- Keep demos **small and focused** (one concept per demo)
- Extract reusable algorithms to `learn_core` (e.g., `gradient_descent()`, `particle_filter()`)
- Use `mod` blocks to organize code (`src/demos/mod.rs` exports all demos)
- Follow existing file header format (see `LEARN/src/lib.rs` for example)

### Questions to resolve during implementation
- Should `SimRunner` be generic over `Sim` trait, or should each app have its own runner?
  - **Recommendation**: Start generic, but if it gets too complex, specialize per app
- Should controls be auto-generated from simulation parameters, or manually defined?
  - **Recommendation**: Manual for MVP (more control over UX), auto-gen later if needed
- Should we support multiple simulations per lesson (e.g., tabs)?
  - **Recommendation**: No for MVP, one sim per lesson is simpler
- How to handle responsive design (mobile vs desktop)?
  - **Recommendation**: Use CSS media queries, make controls stack vertically on mobile
