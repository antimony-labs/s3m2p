## CLAUDE ORCHESTRATION BRIEF — LEARN ESP32 (Rust/WASM)

### Mission
Build a **beginner → advanced** ESP32 learning track inside **`LEARN/ESP32/`** that teaches through **interactive labs + live canvas simulations**, not static docs.

Users should be able to:
- Start from **“blink + serial logs”**
- Progress to **GPIO/PWM/ADC/I2C/SPI/UART**
- Reach **Wi‑Fi/MQTT/OTA**, **FreeRTOS concurrency**, **power management**, and **production-ish** concerns (debugging, partitioning, basics of security)

### Quick start (for implementer)
- Run the ESP32 app locally:

```bash
cd LEARN/ESP32
trunk serve --open --port 8104
```

### Current state (ground truth in this repo)
The ESP32 app is already a polished UI shell, but the “curriculum” is minimal:
- **Lessons are just metadata**: [`LEARN/ESP32/src/lessons.rs`](LEARN/ESP32/src/lessons.rs) defines 4 short lessons.
- **Home + lesson rendering**: [`LEARN/ESP32/src/render.rs`](LEARN/ESP32/src/render.rs) renders a home grid and a lesson view.
  - Home currently shows a single phase header (“GPIO & Digital I/O”) even though lessons span multiple topics.
  - Lesson view includes a canvas slot (`#lesson-canvas`) and demo controls (currently only for lesson `id == 0`).
- **WASM entry + navigation**: [`LEARN/ESP32/src/lib.rs`](LEARN/ESP32/src/lib.rs) exposes `go_to_lesson()` / `go_home()` and starts a demo after rendering.
- **Only one real interactive demo runner** exists: [`LEARN/ESP32/src/demo_runner.rs`](LEARN/ESP32/src/demo_runner.rs) runs a GPIO debounce visualization.
- The debounce simulation logic lives in `learn_core` and is already testable:
  - [`LEARN/learn_core/src/demo.rs`](LEARN/learn_core/src/demo.rs) defines the pure-Rust `Demo` trait + `ParamMeta`.
  - [`LEARN/learn_core/src/demos/gpio_debounce.rs`](LEARN/learn_core/src/demos/gpio_debounce.rs) implements `GpioDebounceDemo` with unit tests.

### Key constraints (do not violate)
- **Rust-first simulations**: put algorithms/state in `LEARN/learn_core` (no `web-sys` there).
- **WASM is glue only**: `LEARN/ESP32` should render DOM + wire controls + draw to canvas.
- **Deterministic**: demos should support seeding (`reset(seed)`).
- **Performance**: avoid per-frame allocations; keep DOM updates minimal; draw to canvas each frame.
- **Client-side only**: no servers required for lessons.

### Reuse existing patterns (do not reinvent)
- Shared web utilities exist in `learn_web`:
  - Canvas wrapper: [`LEARN/learn_web/src/canvas.rs`](LEARN/learn_web/src/canvas.rs)
  - RAF animation loop: [`LEARN/learn_web/src/animation.rs`](LEARN/learn_web/src/animation.rs)
  - Slider/button wiring: [`LEARN/learn_web/src/controls.rs`](LEARN/learn_web/src/controls.rs)
  - Hash routing helpers (optional): [`LEARN/learn_web/src/routing.rs`](LEARN/learn_web/src/routing.rs)
- Shared pure-Rust foundations exist in `learn_core`:
  - RNG/math/etc: [`LEARN/learn_core/src/lib.rs`](LEARN/learn_core/src/lib.rs)
  - Demo trait + param metadata: [`LEARN/learn_core/src/demo.rs`](LEARN/learn_core/src/demo.rs)

### Architectural decision (important)
Treat **ESP32 as its own app**, but keep simulation logic reusable by implementing new demos under:
- `LEARN/learn_core/src/demos/<demo_name>.rs`
- and exporting them from [`LEARN/learn_core/src/demos/mod.rs`](LEARN/learn_core/src/demos/mod.rs)

Then, in the ESP32 web app, build lightweight runners (like `GpioDebounceDemoRunner`) that:
- Create a `learn_web::Canvas`
- Run a `learn_web::AnimationLoop`
- Wire sliders/buttons to `demo.set_param(...)` and `demo.reset(...)`
- Render from demo state each frame

### Target UX (what “good” looks like)
Each lesson has:
- **Short explanation** (why it matters)
- **Controls** (2–5 parameters + reset + play/pause/step)
- **Live visualization** (canvas)
- **Exercises** (small challenges) + **troubleshooting checklist**

Home page should show:
- Clear **phases** (beginner → advanced)
- Progress cues (optional: localStorage)

### Curriculum roadmap (simple → complex)
Use phases to guide progression. Aim for ~12–20 lessons total.

#### Phase 0 — Onboarding & Tooling
- Board + pinout basics (ESP32 variants)
- Install toolchain (ESP-IDF / Arduino / Rust notes)
- Flashing + serial monitor + common errors

#### Phase 1 — Digital I/O (GPIO)
- Blink (output)
- Button input (pull-ups/pull-downs)
- Debounce (already implemented as a sim demo)
- Interrupts vs polling (concept + sim)

#### Phase 2 — Timers & PWM (LEDC)
- PWM duty cycle intuition + waveform visualization
- Frequency vs resolution tradeoffs
- Servo control / motor control basics

#### Phase 3 — Analog (ADC)
- Quantization + resolution intuition
- ADC noise + averaging / simple filters
- Sensor patterns (potentiometer, thermistor)

#### Phase 4 — Buses (I2C/SPI/UART)
- I2C addressing + ACK/NAK (transaction visualization)
- SPI framing + chip-select etiquette
- UART baud/framing/errors

#### Phase 5 — Networking & OTA
- Wi‑Fi join states + retries
- HTTP vs MQTT patterns
- OTA concepts (partitioning + rollback, high level)

#### Phase 6 — Concurrency (FreeRTOS)
- Tasks vs interrupts mental model
- Queues/ring buffers
- Timing/jitter + debouncing in the presence of load

#### Phase 7 — Power management
- Deep sleep basics
- Wake sources
- Measuring “duty-cycled” power (conceptual)

#### Phase 8 — Production-ish engineering
- Logging + observability
- NVS storage patterns
- Security overview: TLS, secure boot / flash encryption (conceptual, not a full guide)

### Per-lesson content template (keep consistent)
For each lesson, include:
- **Goal**: one sentence
- **Why it matters**: real-world use case
- **Prereqs**: tools, parts, prior lessons
- **Wiring**: minimal diagram/notes
- **Lab steps**: numbered, copy/paste friendly
- **Interactive demo**: what the canvas shows + what each control does
- **Common failure modes**: symptoms → likely cause → fix
- **Exercises**: 2–5 challenges
- **Next**: link to next lesson phase

### Interactive demo backlog (ESP32)
Add at least 3 beyond debounce:
- **PWM waveform + duty**:
  - Visualize square wave, averaged power, perceived brightness
  - Controls: duty, frequency, resolution (conceptual), load model (LED vs motor)
- **ADC quantization + noise**:
  - Visualize analog signal → sampled values → filtered output
  - Controls: bits, sampling rate, noise level, filter window
- **I2C bus transactions**:
  - Visualize start/stop, address, data bytes, ACK/NAK
  - Controls: address, clock rate (conceptual), error injection (NAK, clock stretch)

(Optional advanced)
- **UART framing** (start/stop bits, baud mismatch)
- **RTOS scheduler toy model** (tasks, priorities, jitter)

### Milestones (ship incrementally)
#### Milestone 1 — Phase-based curriculum structure
- Refactor lesson metadata so home page can render **phases** (not one bucket).
- Add lesson ordering + difficulty tags (beginner/intermediate/advanced).

#### Milestone 2 — Add 3 new demos (core + web runners)
- Implement pure-Rust demos in `learn_core` (PWM, ADC, I2C).
- Add ESP32 runners + controls + canvas rendering for each.

#### Milestone 3 — Strengthen “guide” content
- Apply the per-lesson template across lessons (prereqs, lab steps, pitfalls, exercises).
- Add troubleshooting sections with real error patterns (flashing, serial, wiring, power).

#### Milestone 4 — Polish
- Make play/pause/step/reset consistent across all lessons.
- Add light progress tracking (optional) and improve navigation between phases.

### Definition of done
- Home page shows **multiple phases** and a clear beginner→advanced path.
- At least **12 lessons** using the lesson template.
- At least **3 interactive demos beyond debounce**.
- `trunk build` / `trunk serve` for `LEARN/ESP32` works without regressions.


