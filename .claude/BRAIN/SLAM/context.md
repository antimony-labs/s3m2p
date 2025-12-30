# SLAM Context

## Quick Facts
- **Path**: /home/curious/S3M2P/LEARN/SLAM
- **Port**: 8106
- **Deploy**: `./SCRIPTS/deploy.sh slam --publish`
- **URL**: slam.too.foo
- **Type**: Static HTML + WASM demos

## Key Files
| File | Purpose |
|------|---------|
| index.html | Main tutorial page with all lessons |
| src/lib.rs | WASM demo logic |
| THEORY.md | Math explanations |
| demo-*.html | Individual demo pages |

## Interactive Demos (5)
1. Odometry simulation
2. Lidar scanning
3. EKF localization
4. Particle filter
5. Dark hallway navigation

## Validation
```bash
trunk build LEARN/SLAM/index.html
```

## Common Tasks
1. **Add math section**: Use KaTeX blocks in index.html
2. **Add demo**: Create demo-*.html, add WASM bindings in lib.rs
3. **Update theory**: Edit embedded content or THEORY.md

## Features
- KaTeX for math rendering
- Mermaid diagrams for algorithms
- Mobile-responsive pop-out demos
- Canvas-based visualizations
