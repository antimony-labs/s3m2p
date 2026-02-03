# Performance Ledger (Baseline Metrics)

Purpose
- Capture repeatable baseline performance metrics for each surface.
- Provide a single place to compare improvements over time.

How to use
- Add a row per surface, per device, per scenario.
- Record date, build/commit, and any special conditions.
- Re-run after major changes and add a new row.

Device matrix (recommended; adjust to available hardware)
- Low-end phone: Android 12, 4GB RAM, Snapdragon 6xx class, Chrome stable, 60Hz.
- Mid laptop: 4-8 core CPU, 16GB RAM, integrated GPU (e.g., Intel i5-1135G7 or M1 Air), Chrome stable.
- Desktop: 8-core CPU, 32GB RAM, discrete GPU (e.g., RTX 3060 class), Chrome stable.

Scenario definitions (baseline; use cold cache unless noted)
- WELCOME: cold load, idle 10s, open menu/CTA, idle 20s.
- HELIOS: cold load, idle 10s, zoom/pan for 10s, idle 20s.
- ARCH: cold load, open graph, drag a node for 10s, idle 20s.
- BLOG: cold load a long post, scroll to mid, idle 10s.
- LEARN: cold load lesson, expand a section, start demo, idle 20s.
- MCAD: cold load default model, orbit for 10s, idle 20s.
- ATLAS: cold load map, pan/zoom for 10s, idle 20s.
- SIMULATION/CHLADNI: cold load, run default sim for 30s, idle 10s.
- TOOLS/*: cold load default tool view, change inputs, idle 10s.

Baseline table

| Surface | Scenario | Device | Date | Build | wasm KB | JS KB | CSS KB | Fonts/Imgs KB | First paint ms | First interaction ms | Total load ms | FPS avg | FPS 1% low | Mem 60s MB | Mem 120s MB | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| WELCOME |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| HELIOS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ARCH |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| BLOG |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LEARN |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| MCAD |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ATLAS |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| SIMULATION/CHLADNI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| TOOLS/* |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
