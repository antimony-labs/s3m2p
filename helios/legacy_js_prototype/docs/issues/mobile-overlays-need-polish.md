# Issue: Mobile overlay polish + scene defaults

## Summary
- Research data overlay still uses fixed-width cards that spill off iPhone viewports, leaving mission status unreadable unless the user pans.
- Safe-area insets are ignored so the overlay can hide behind the dynamic island/status bar on modern devices.
- Scene defaults (distance markers, solar apex, interstellar debris) don't match the LayerControl defaults, so toggling layers on touch screens re-enables elements that were never visible.

## Scope
- Reflow `DataOverlay` into safe-area-aware, full-width cards on narrow screens while preserving the stacked sidebar on desktop.
- Ensure overlay cards disable pointer-events passthrough and adopt rounded, touch-friendly spacing so the controls feel native on phones.
- Align the `createScene` component visibility defaults with the LayerControl initial state so mobile users tap once to reveal a layer instead of toggling a hidden state.

## Acceptance Criteria
- On an iPhone-depth viewport, the overlay stays inside the safe area with no horizontal scroll and all stats readable.
- Overlay cards keep pointer events enabled and can be toggled/selected without affecting OrbitControls.
- Desktop layout remains a 320 px sidebar with the same data blocks.
- `Hero` scene layers reflect the same defaults as the LayerControl UI (helioglow off, distance markers & solar apex off, interstellar objects off).
