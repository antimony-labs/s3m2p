# Issue: Improve Mobile Viewport, Aspect Ratio, and UV Defaults

## Summary
- The heliosphere canvas currently relies on `100vh`, which behaves poorly on iOS browsers, and the OrbitControls camera feels cramped in portrait layouts.
- Controls/Layers UI does not reflow on smaller screens, making it hard to interact with on iPhone-sized viewports.
- The UV/helioglow layer renders too dark for the mobile display pipeline and should default to off until we tune it.

## Scope
- Introduce a dynamic viewport height variable and resize logic that relies on `visualViewport` so the canvas honors iPhone safe areas and keeps a stable aspect ratio.
- Clamp the WebGL renderer pixel ratio and tweak camera/FOV offsets when the viewport is portrait to reduce GPU pressure and keep the heliosphere in frame.
- Reflow header controls + safe-area padding for touch targets on narrow widths.
- Default the UV layer (`helioglow`) to off across the scene + UI so it no longer ships enabled.

## Acceptance Criteria
- Canvas fills the *dynamic* viewport height on an iPhone simulator (address bar collapse should no longer pop the layout) and the heliosphere remains centered without severe stretch.
- Renderer pixel ratio is capped (≤1.6) on mobile while desktop still renders at higher density, and camera adjusts automatically when aspect ratio < 1.
- Controls/Layer UI stacks vertically on narrow screens with comfortable tap targets and respects safe-area top padding.
- Layer menu + scene visibility both report `helioglow: false` on initial load, though the layer can still be toggled on manually.
- Documented in repo (issue + PR notes) for future reference.
