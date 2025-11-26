# `sim/` — Sun-Centric Heliosphere Simulation Core

High-performance, scientifically accurate heliosphere visualization framework.

## Quick Start

```typescript
import { getRegistry, getDatasetLoader, Units, HeliosphereSurface } from '@/app/sim';

// 1. Initialize
const registry = getRegistry({ auToScene: 1.0 });
const loader = getDatasetLoader('/dataset');
await loader.initialize();

// 2. Load data
const params = await loader.loadParametersAt(registry.getTime());

// 3. Generate surface
const surface = new HeliosphereSurface(params);
const mesh = surface.generateMesh(64, 128, 'heliopause');

// 4. Render with Three.js!
```

## Directory Structure

```
sim/
├── index.ts                   # Main exports
├── README.md                  # This file
│
├── types/                     # Type system & units
│   ├── units.ts              # AU, KmPerSec, JulianDate, etc.
│   └── vectors.ts            # PositionAU, VelocityKmS, Vec3 utils
│
├── frames/                    # Coordinate transformations
│   └── CoordinateFrame.ts    # HEE_J2000 ↔ APEX ↔ ICRS
│
├── registry/                  # Central state management
│   └── Registry.ts           # Single source of truth
│
├── data/                      # Data structures & loaders
│   ├── StructureOfArrays.ts  # SoA for particles/stars
│   └── DatasetLoader.ts      # Zarr/JSON streaming
│
├── physics/                   # Physics models
│   └── HeliosphereSurface.ts # Parametric surface generation
│
├── rendering/                 # Rendering systems
│   └── StarField.ts          # Instanced stars + panorama
│
├── gpu/                       # GPU compute
│   └── ParticleSystem.ts     # Ping-pong particle advection
│
└── validation/                # Testing & overlays
    └── ValidationOverlays.ts  # Reference markers & tests
```

## Core Concepts

### 1. Units Are Typed

Use branded types to prevent unit mixing:

```typescript
import { Units, AU } from '@/app/sim/types/units';

const distance: AU = Units.AU(121.6);  // ✅ Type-safe
const wrong: AU = 121.6;               // ❌ Type error
```

### 2. Everything is Sun-Centric

All positions are in **HEE_J2000** frame (Sun at origin):

```typescript
const position: PositionAU = {
  x: Units.AU(100),  // X: towards vernal equinox
  y: Units.AU(0),
  z: Units.AU(0),    // Z: ecliptic north pole
};
```

### 3. Single Global Scale

Convert to Three.js world space only at render time:

```typescript
const scenePos = registry.heeToScene(position);
// scenePos = { x: 100, y: 0, z: 0 }  (if auToScene = 1.0)
```

### 4. GPU-First

No per-frame CPU loops. Use:
- **Instanced rendering** for stars
- **Ping-pong FBOs** for particles
- **SoA layout** for cache coherency

## Usage Examples

### Loading & Interpolating Data

```typescript
const loader = getDatasetLoader();
await loader.initialize();

// Find epochs bracketing current time
const [i0, i1, alpha] = loader.findEpochBracket(currentTime);

// Load and interpolate
const params = await loader.loadParametersAt(currentTime);
```

### Generating Heliosphere Surface

```typescript
const surface = new HeliosphereSurface(params);

// Get mesh data
const hp = surface.generateMesh(64, 128, 'heliopause');
const ts = surface.generateMesh(64, 128, 'termination_shock');

// Sample point on surface (for particle emission)
const point = surface.samplePoint(
  Units.Radians(Math.PI / 4),
  Units.Radians(Math.PI / 2),
  'heliopause'
);
```

### GPU Particle System

```typescript
const particles = new GPUParticleSystem(renderer, {
  maxParticles: 100_000,
  emissionRate: 1000,
  particleLifetime: 10.0,
});

// Update each frame
function animate() {
  particles.update(dt, registry.config.auToScene);
  renderer.render(scene, camera);
}
```

### Validation Overlays

```typescript
const validation = new ValidationOverlays(registry, {
  showReferenceRings: true,
  showVoyagerTracks: true,
  showApexArrow: true,
  showIBEXArrow: true,
});

scene.add(validation.getGroup());

// Run automated tests
ValidationTests.runAll(registry, params.R_HP_nose, params.R_TS_over_HP);
```

### Coordinate Transforms

```typescript
const transforms = new CoordinateTransforms();

// HEE → APEX (nose-aligned frame)
const apexPos = transforms.heeToApexPosition(heePos);

// Get ISM inflow direction
const inflowDir = transforms.getIsmInflowDirection();

// Export to Three.js
const matrix = transforms.toThreeMatrix4(transforms.getHeeToApexMatrix());
```

## Performance Tips

1. **Batch updates**: Update all particles/stars in single GPU pass
2. **Prefetch data**: Use `loader.prefetch()` for smooth playback
3. **LOD meshes**: Reduce theta/phi steps for distant surfaces
4. **Frustum culling**: Let Three.js cull invisible objects
5. **DPR clamping**: Limit `devicePixelRatio` on mobile

## Scientific Accuracy

This framework is designed for **visual scientific accuracy**:

- ✅ **Scale**: Distances in AU, ratios match observations
- ✅ **Orientation**: ISM inflow from IBEX measurements
- ✅ **Validation**: Automated tests verify Voyager crossings
- ⚠️ **Morphology**: Simplified models (cometary/croissant/bubble)
- ⚠️ **Planets**: Illustrative orbits outside ±10 Myr

## API Reference

See [SUN_CENTRIC_ARCHITECTURE.md](../../SUN_CENTRIC_ARCHITECTURE.md) for detailed documentation.

## Testing

```bash
npm test -- app/sim
```

Validation tests run automatically in browser console:

```
[Validation] Heliopause nose radius: 121.6 AU (expected 121.6 AU) - PASS
[Validation] TS/HP ratio: 0.77 (expected 0.75-0.85) - PASS
[Validation] ISM inflow direction: 2.3° from expected - PASS
[Validation] Overall: ALL TESTS PASSED
```

## Migration from Legacy Code

### Before (ad-hoc):

```typescript
// Mixed units, unclear frame
const voyager1 = new THREE.Vector3(121.6, 0, 0);
scene.add(createSphere(voyager1, 1));
```

### After (Sun-centric):

```typescript
// Type-safe, explicit frame
const voyager1: PositionAU = {
  x: Units.AU(121.6),
  y: Units.AU(0),
  z: Units.AU(0),
};
const scenePos = registry.heeToScene(voyager1);
scene.add(createSphere(new THREE.Vector3(scenePos.x, scenePos.y, scenePos.z), 1));
```

## Contributing

When adding features:
1. Use branded types for all physical quantities
2. Work in HEE_J2000 frame internally
3. Convert to scene units only at render time
4. Add validation tests for new physics models
5. Document assumptions and limitations

---

**Built for scientific visualization. Accurate. Performant. Scalable.**

