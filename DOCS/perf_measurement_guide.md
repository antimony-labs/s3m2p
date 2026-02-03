# Performance Measurement Guide

Purpose
- Ensure baselines are captured consistently across surfaces.
- Standardize scenario, device, and reporting fields.

Before you measure
- Pick the surface and scenario from `DOCS/perf_ledger.md`.
- Record the device name and OS.
- Use a clean browser profile (no extensions).
- Disable throttling unless explicitly testing it.

What to capture
- Load: first paint, first interaction, total load time.
- Runtime: average FPS + 1% low during the scenario.
- Memory: at 60s and 120s into the scenario.
- Payload sizes: wasm, JS, CSS, fonts, images.
- Cache headers for key assets (wasm, main JS, CSS).

Where to capture (suggested tools)
- Browser DevTools Performance panel for load + runtime.
- Performance monitor (FPS + memory).
- Network panel for asset sizes and headers.

Recording
- Add a new row per device/scenario in `DOCS/perf_ledger.md`.
- Include date and build/commit identifier.
- Add notes if the scenario deviates or if there are anomalies.

