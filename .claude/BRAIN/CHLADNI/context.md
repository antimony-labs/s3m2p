# CHLADNI Context

## Quick Facts
- **Path**: /home/curious/S3M2P/SIMULATION/CHLADNI
- **Port**: 8082
- **Deploy**: `./SCRIPTS/deploy.sh chladni --publish`
- **URL**: chladni.too.foo
- **Type**: WASM (Rust/WebGL2)

## Key Files
| File | Purpose |
|------|---------|
| src/lib.rs | Simulation state, particle physics |
| src/wave.rs | Wave equation solver, eigenmodes |
| src/renderer.rs | WebGL2 rendering, particle points |
| index.html | Entry point with mode controls |

## Validation
```bash
cargo check -p chladni
trunk build SIMULATION/CHLADNI/index.html
```

## Common Tasks
1. **Change particle size**: Modify `point_size` calculation in `renderer.rs`
2. **Add wave mode**: Add to PlateMode enum, update eigenmode calc
3. **Audio integration**: Live mode uses microphone pitch detection

## Common Issues
- Mobile WebGL needs `precision highp float` in shaders
- Point size scaling: use sqrt() for gentler scaling
- Grid size affects particle density
- Recent fix: particle sizes were too big (fixed with sqrt scaling)
