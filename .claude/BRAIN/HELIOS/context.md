# HELIOS Context

## Quick Facts
- **Path**: /home/curious/S3M2P/HELIOS
- **Port**: 8081
- **Deploy**: `./SCRIPTS/deploy.sh helios --publish`
- **URL**: helios.too.foo
- **Type**: WASM (Rust/Canvas)

## Key Files
| File | Purpose |
|------|---------|
| src/main.rs | WASM entry, event handlers, animation loop |
| src/render.rs | Canvas 2D rendering, star drawing |
| src/simulation.rs | Solar wind, heliosphere updates |
| src/star_data.rs | UniverseDataManager, spectral bands |
| src/cca_projection.rs | CelestialCamera, sun-centered projection |
| index.html | Entry point |

## Validation
```bash
cargo check -p helios
trunk build HELIOS/index.html
```

## Common Tasks
1. **Add spectral band**: Add to `Band` enum in `star_data.rs`
2. **Add constellation**: Add `ConstellationEdge` entries in `update_constellations()`
3. **Adjust LOD**: Modify `adjust_lod()` thresholds in `star_data.rs`
4. **Add HUD element**: Create render function in `render.rs`

## Common Issues
- LOD auto-adjusts mag_limit based on frame time (3-8ms target)
- Max 4000 stars/frame hard cap
- Use glam::DVec3 for astronomy (double precision)
