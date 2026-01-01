# DNA Context

## Quick Facts
- **Path**: /home/curious/S3M2P/DNA
- **Port**: N/A (library crate)
- **Type**: Rust library (foundation layer)

## Key Modules
| Module | Purpose |
|--------|---------|
| physics/electromagnetics/lumped.rs | R, L, C, Diode, OpAmp, transistors |
| physics/solvers/rk45.rs | Runge-Kutta integrator |
| physics/solvers/filters.rs | EKF, trajectory smoothing |
| cad/ | B-Rep CAD kernel (geometry, topology, primitives) |
| pll/ | Phase-locked loop components |
| sim/ | BoidArena, SpatialGrid, state machine |
| export/ | PDF, Gerber X2 exporters |

## Validation
```bash
cargo check -p dna
cargo test -p dna
```

## Common Tasks
1. **Add physics component**: Add to `physics/electromagnetics/lumped.rs`
2. **Add CAD primitive**: Add to `cad/primitives.rs`
3. **Add PLL component**: Add to appropriate file in `pll/`

## Performance Rules
- No allocations in hot paths
- Use stack-allocated arrays: `let mut buf = [0u16; 64];`
- Use `#[inline]` for small per-entity functions
- Check vector length before normalize (guard NaN)
- Prefer f32 over f64 for simulation
