# Claude Prompt — Mobile landing polish for too.foo (WELCOME)

You are **Claude (programmer)** working in the repo at `S3M2P/`. Your job is to implement the attached plan **exactly** (mobile-first clarity, keep the constellation), while keeping changes minimal and production-safe.

## Non‑negotiables
- **Do NOT edit** the plan file under `.cursor/plans/…` (read-only reference).
- The todo items already exist. **Do not create new todos.** Mark them **in_progress** as you start each one, in this exact order:
  1) `add-social-meta`
  2) `mission-overlay`
  3) `telemetry-mobile`
  4) `bubble-a11y`
  5) `update-visual-tests`
  6) `verify`
- Keep the landing **constellation-first** (no redesign into a scroll landing).
- Prioritize **mobile first-time visitor clarity**.
- Follow repo standards: **zero warnings** on `cargo check`, avoid unnecessary dependencies.

## High-level intent (copy direction)
Mission direction is **hybrid**:
- “Antimony Labs builds open-source engineering tools, simulations, and manufacturing compilers — in Rust/WASM/WebGPU.”
- Mission tagline: **“Let AI design, humans build.”**
- Instructional line: **“Tap a bubble to open a project.”**

## Files you will modify
- `WELCOME/index.html`
- `WELCOME/src/main.rs`
- `TESTS/specs/visual.spec.ts`

You will also add one asset:
- `WELCOME/assets/og.png`

## Task 1 — add-social-meta
### Goal
LinkedIn/X previews should look intentional.

### Implementation
In `WELCOME/index.html` `<head>`:
- Set `<title>` to **`too.foo — Antimony Labs`**.
- Add `meta name="description"` using the hybrid mission.
- Add **Open Graph** tags: `og:type`, `og:site_name`, `og:title`, `og:description`, `og:url`, `og:image`.
- Add **Twitter Card** tags: `twitter:card` (use `summary_large_image`), `twitter:title`, `twitter:description`, `twitter:image`.
- Add `meta name="theme-color"` matching the UI (e.g. `#050508`).

Use an **absolute** `og:image` URL for production: `https://too.foo/assets/og.png`.

### Add the OG image file
Create `WELCOME/assets/og.png` by copying an existing repo PNG (don’t try to hand-author binary):
- Source: `TESTS/specs/visual.spec.ts-snapshots/landing-layout-chromium-linux.png`
- Destination: `WELCOME/assets/og.png`

(If you can easily crop/resize to 1200×630 using available tools, do it; otherwise just ship the copied PNG.)

## Task 2 — mission-overlay
### Goal
On mobile first visit, the user immediately understands what too.foo is and what to do.

### Implementation
In `WELCOME/index.html`, add a lightweight overlay **inside the `#ui-layer`** (or as a fixed-position element) with:
- Headline / identity: **Antimony Labs / too.foo**
- 1–2 lines explaining the hybrid mission
- Tagline: **Let AI design, humans build.**
- Instruction: **Tap a bubble to open a project.**
- 3 big CTAs:
  - **Explore Tools** → `#/tools`
  - **Open Helios** → `https://helios.too.foo`
  - **Read Vision** → `https://github.com/Shivam-Bhardwaj/S3M2P/blob/main/VISION.md`

Behavior:
- **Show by default on first visit**.
- Dismissible via close button.
- Persist dismissal in `localStorage` (e.g., key `toofoo_intro_dismissed_v1`).
- Add an always-available **Mission/Menu** button to reopen the overlay.
- Respect iOS safe areas (`env(safe-area-inset-top/right/bottom/left)`).

Test stability requirement:
- Add support for **query param** `intro=false` that forces the overlay hidden (used by Playwright).

## Task 3 — telemetry-mobile
### Goal
On small screens, the bottom bar shouldn’t look like unexplained jargon, and the user must still have a visible entry point to the mission/menu.

### Implementation
In `WELCOME/index.html` CSS:
- In the existing `@media (max-width: 600px)` block, **de-emphasize or collapse** `#stats-row` (hide it or reduce it), and ensure the **Mission/Menu** control remains visible.
- Add safe-area padding for the telemetry bar on mobile so it doesn’t clash with the home indicator.

Keep the overall “tech UI” vibe.

## Task 4 — bubble-a11y
### Goal
Long-press tooltips and screen readers give meaningful labels.

### Implementation
In `WELCOME/src/main.rs` inside `render_bubbles()` when creating each `.monolith` link:
- Set `title` and `aria-label` using label + description, e.g. `"Tools — Engineering Apps"`.
- For `target="_blank"` links, add `rel="noopener noreferrer"`.

## Task 5 — update-visual-tests
### Goal
CI/test suite stays stable after adding the overlay.

### Implementation
In `TESTS/specs/visual.spec.ts`:
- Change navigation to include the new param: `/?paused=true&intro=false`.
- Fix the text expectations to match actual home bubbles (e.g. Helios / Learn / Tools / Simulations / Blog / About Me).
- Update the screenshot baseline using Playwright’s update-snapshots flow.

## Task 6 — verify
Run these commands and fix anything that breaks:
- `cargo check -p welcome`
- `trunk build WELCOME/index.html`
- `npx playwright test TESTS/specs/visual.spec.ts` (and update snapshots if needed)

Also do a quick manual smoke check at a mobile viewport (320–375px):
- Overlay readable and dismissible
- Mission/Menu button always reachable
- Bubbles still tappable
- No layout overlap with telemetry bar

## Output
When done, report:
- Files changed
- Commands run + results
- Any tradeoffs (e.g. if OG image isn’t perfectly sized)
