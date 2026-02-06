# SLAM - Simultaneous Localization and Mapping Tutorials

> Codex CLI note: See `/AGENTS.md` for repo-wide instructions and best practices. This file is project-specific context.

Interactive SLAM tutorials with 5 demos built in Rust/WASM. Deployed to slam.too.foo.

## Build & Run

```bash
# Development (hot reload) - from repo root
./SCRIPTS/dev up slam

# Production build
trunk build --release LEARN/SLAM/index.html
```

## Architecture

```
LEARN/SLAM/
  src/
    lib.rs           # WASM entry, lesson/demo routing
    lessons.rs       # Lesson content and metadata
    demo_runner.rs   # Interactive demo logic (5 demos)
    render.rs        # Canvas rendering for demos
  index.html         # Entry point with lesson navigation
```

## Demos

1. **Odometry** - Dead reckoning with drift visualization
2. **Lidar Scanning** - 2D lidar beam simulation
3. **EKF Localization** - Extended Kalman Filter for robot pose
4. **Particle Filter** - Monte Carlo localization
5. **Dark Hallway** - Navigation under uncertainty

## Dependencies

Uses shared learning infrastructure:
- `learn_core` - Demo abstractions, sensor demos (EKF, particle filter)
- `learn_web` - Canvas utilities, routing, controls

## Testing

No Playwright or unit tests yet. Testing is manual via dev server.
