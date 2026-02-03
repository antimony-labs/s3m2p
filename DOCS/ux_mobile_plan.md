# Mobile UX and Readability Plan

Goal
- Make the platform pleasant on phones and tablets.
- Reduce cognitive load and improve reading flow.
- Preserve the visual identity without sacrificing usability.

Success criteria (definition of done)
- No horizontal scroll at 320px viewport.
- Tap targets >= 44px with visible pressed/active states.
- Body text 16-18px, line height 1.6-1.8, line length ~45-75 chars.
- Primary action is visible in the first viewport on key entry pages.
- prefers-reduced-motion is honored; no continuous motion on mobile by default.
- Demos and heavy visuals never block first interaction.

Scope and priorities
- Priority order: LEARN, BLOG, WELCOME, HELIOS, ARCH, ATLAS, SIMULATION/CHLADNI, TOOLS/*.
- Content-first surfaces get the first full pass.
- Simulation-first surfaces get a mobile safety pass (reduced motion, low detail).

Non-goals
- No brand redesign or new UI framework.
- No heavy JS for layout; use HTML/CSS first.
- No feature expansion unless required to make mobile usable.

Artifacts (docs and templates)
- DOCS/mobile_tokens.md (type scale, spacing, layout tokens).
- DOCS/mobile_audit.md (per-surface audit checklist + results).

Global UX principles
- Mobile-first layout: column-first, large tap targets, single primary action.
- Legibility: 16-18px base text, strong contrast, generous line height.
- Consistent spacing scale: 8px or 12px grid to keep rhythm.
- Content density: reduce simultaneous visual noise on small screens.
- Motion discipline: avoid continuous motion that steals attention from content.

Platform-wide mobile fixes
- Add a global responsive layout policy (max width, padding scales).
- Use CSS clamp() for typography scaling.
- Provide reduced motion mode for low-end devices.
- Ensure all CTA buttons are 44px min height.
- Avoid hover-only affordances (touch needs visible cues).

Audit checklist (per surface)
- Viewports: 320/375/414/768 widths with no horizontal scroll.
- Safe-area handling on iOS (padding for notches).
- Tap targets, spacing, and focus states for all actions.
- Hover-only UI has a visible touch fallback.
- Typography scale matches reading rhythm (headers vs body).
- Demos or canvases do not auto-run when offscreen.

Execution roadmap
Phase 0 - Foundations
- Define mobile tokens (type scale, spacing, max widths).
- Apply global layout container + typography scaling.
- Add reduced-motion support and demo pausing behavior.

Phase 1 - High traffic surfaces
- LEARN: widget layout, reading mode, demo gating, compact nav.
- BLOG: single column, larger type, lazy images, simplified chrome.
- WELCOME: reduce background intensity, simplify overlays, CTA above fold.

Phase 2 - Simulation-first surfaces
- HELIOS: low detail mode, reduce overlays, throttle labels.
- ARCH: draw-on-change, tap focus, disable drag redraw when static.
- ATLAS: simplify controls, reduce layer count on mobile.
- SIMULATION/CHLADNI: reduce particle count on mobile, pause when hidden.
- TOOLS/*: single-column forms, large inputs, clear validation states.

Phase 3 - QA and polish
- QA pass on phones/tablets and document fixes.
- Close remaining audit checklist gaps and re-test.

LEARN (priority)

Current issues
- Dense cards, small text, and wide grids on small screens
- Long lessons lack clear structure and reading rhythm
- Demos compete with content on mobile

Design direction: widget-based reading
- Use stacked widgets: Overview, Concepts, Math, Demo, Summary.
- Each widget is a card with a clear heading and optional icon.
- Progressive disclosure: hide advanced sections behind "Expand".
- Reading mode: single-column, narrow max width, large line height.

Layout spec
- Mobile:
  - 1 column grid
  - 16-18px body text, 1.7 line height
  - 24-32px section headers
  - 12-16px vertical rhythm
- Tablet:
  - 2 column grid for cards, but lesson detail remains single column
- Desktop:
  - 2-3 column grid for cards
  - lesson detail can add a right rail for navigation

Interaction spec
- When a lesson opens, show a short summary + prerequisites first
- Provide a "Try demo" button that opens the demo in its own block
- Add "Quick checks" (mini widgets) to reinforce understanding
- Support a compact navigation bar for jumping between sections

Common components (mobile-first)
- Stacked card widgets with clear headings and optional icons.
- Inline callouts: Note, Warning, Example, Summary.
- Sticky mini-nav for section jumps (collapsed by default).
- Expand/collapse for advanced content and long derivations.

Performance tie-in
- Only mount/animate demos when the demo block is visible
- Pause simulations when offscreen or when the user scrolls past

Other surfaces (mobile priorities)
- WELCOME: reduce background intensity on mobile, simplify overlays
- HELIOS: add a low-detail mode and limit overlays
- BLOG: switch to single column, larger typography, image lazy loading
- ARCH: disable drag redraw loops when static; allow tap to focus

Decision gate
- If a surface is content-first, favor HTML/CSS and pre-rendering.
- If a surface is simulation-first, keep Rust/WASM but tune for mobile.
