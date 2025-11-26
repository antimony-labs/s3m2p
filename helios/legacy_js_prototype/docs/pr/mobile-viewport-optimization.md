# PR: Stabilize Mobile Viewport + Disable UV Layer

## Related Issue
- [docs/issues/mobile-viewport-optimization.md](../issues/mobile-viewport-optimization.md)

## Changes
- Added viewport snapshot plumbing to `Hero`/`createScene` so mobile devices rely on `visualViewport`, clamp DPR, and retune the camera/FOV based on aspect ratio.
- Clamped renderer pixel ratio + enabled GPU-friendly styles (`translateZ(0)`, `will-change`) for the canvas while adding safe-area aware height variables in global CSS and layout components.
- Reflowed the header controls stack (safe-area padding, flex wrapping, larger tap targets) so iPhone users can reach sliders/toggles without horizontal scrolling.
- Defaulted the helioglow/UV layer to `false` across the scene + LayerControl UI per design feedback.
- Captured the work as a local issue/PR artifact per workflow request.

## Testing
- `npm run test` *(fails in existing suites: `tests/components/Controls.test.tsx` “handles errors when updating scene on pause” and several `tests/integration/researchHero.test.tsx` cases that already fail on main — see CLI output in ChatGPT response for details).* 
