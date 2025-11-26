# PR: Align mobile overlays + scene defaults

## Related Issue
- [docs/issues/mobile-overlays-need-polish.md](../issues/mobile-overlays-need-polish.md)

## Changes
- Reflowed `DataOverlay` into safe-area-aware, full-width cards on mobile while keeping the compact 320 px sidebar on desktop, added pointer-event isolation, and stacked grids for legible stats.
- Synced `createScene` visibility defaults with the LayerControl UI so helioglow/distance markers/solar apex/interstellar objects all start disabled across devices.

## Testing
- Not run (UI-only changes).
