# Sun-Centric Heliosphere Architecture

## Overview

This document describes the **Sun-centric, precomputed dataset-driven architecture** for heliosphere visualization. The system is designed for scientific accuracy, performance, and scalability across the Sun's entire lifetime (ZAMS → White Dwarf).

## Core Principles

### 1. Single Source of Truth

- **Coordinate Frame**: Heliocentric Ecliptic J2000 (HEE/J2000)
- **Origin**: Sun center (all positions relative to Sun)
- **Units**: AU for distance, km/s for velocity, radians for angles
- **Scale**: Single global `AU_TO_SCENE` factor for Three.js world space

### 2. Precomputed Data

- **Offline generation**: Python pipeline computes heliosphere evolution
- **Compact storage**: Zarr arrays for chunked HTTP streaming
- **Runtime interpolation**: Linear/Hermite between epochs (no heavy physics in browser)

### 3. GPU-First Rendering

- **Particles**: WebGL2 ping-pong or WebGPU compute (no per-frame CPU loops)
- **Stars**: Instanced rendering with SoA layout
- **Surfaces**: Parametric generation in AU, converted to scene units

### 4. Scientific Accuracy

- **Validation overlays**: Voyager crossings, TS/HP rings, IBEX/apex arrows
- **Automated tests**: Scale, orientation, and ratio checks
- **Units enforcement**: TypeScript branded types prevent mixing

---

## Architecture Components

### `sim/types/` — Type System

**Purpose**: Enforce unit safety with branded types.

**Files**:
- `units.ts` — AU, KmPerSec, Radians, JulianDate, etc.
- `vectors.ts` — PositionAU, VelocityKmS, Vec3 utilities

**Key Concept**: Use `Units.AU(value)` to create typed values. Prevents accidental unit mixing.

```typescript
import { Units, AU, Convert } from '@/app/sim/types/units';

const distance: AU = Units.AU(121.6); // Voyager 1 HP crossing
const km = Convert.auToKm(distance);  // Type-safe conversion
```

---

### `sim/frames/` — Coordinate Transforms

**Purpose**: Convert between ICRS, HEE_J2000, and APEX frames.

**Files**:
- `CoordinateFrame.ts` — CoordinateTransforms service

**Key Features**:
- Fixed rotation matrices (precomputed)
- ISM inflow direction from IBEX measurements
- Three.js Matrix4 export for GPU transforms

```typescript
import { CoordinateTransforms } from '@/app/sim/frames/CoordinateFrame';

const transforms = new CoordinateTransforms();

// Convert position from HEE to APEX (nose-aligned)
const heePos = { x: Units.AU(100), y: Units.AU(0), z: Units.AU(0) };
const apexPos = transforms.heeToApexPosition(heePos);
```

---

### `sim/registry/` — Central Data Registry

**Purpose**: Single source of truth for all simulation state.

**Files**:
- `Registry.ts` — Registry class and global accessor

**Contents**:
- `config`: SimConfig (frame, time, scale)
- `transforms`: CoordinateTransforms instance
- `surfaces`: Heliosphere surfaces (HP, TS, BS)
- `particles`: SoA particle systems
- `stars`: Star catalog
- `bodies`: Planets, spacecraft

**Usage**:

```typescript
import { getRegistry, Units } from '@/app/sim';

const registry = getRegistry({
  auToScene: 1.0,
  julianDate: Units.JulianDate(2451545.0), // J2000.0
});

// Add a celestial body
registry.setBody({
  id: 'voyager1',
  name: 'Voyager 1',
  position: { x: Units.AU(160), y: Units.AU(0), z: Units.AU(0) },
  radius: Units.AU(0.001),
  visualScale: 100,
  color: 0xffff00,
  visible: true,
  type: 'spacecraft',
});

// Convert to scene coordinates
const scenePos = registry.heeToScene(body.position);
```

---

### `sim/data/` — Data Structures & Loaders

**Purpose**: Structure-of-Arrays for GPU efficiency + dataset streaming.

**Files**:
- `StructureOfArrays.ts` — SoA layouts for particles/stars/spacecraft
- `DatasetLoader.ts` — Zarr/JSON loader with LRU cache

**Particle SoA Layout**:

```typescript
interface ParticleArrays {
  posX: Float32Array;  // x positions (AU)
  posY: Float32Array;
  posZ: Float32Array;
  velX: Float32Array;  // velocities (km/s)
  velY: Float32Array;
  velZ: Float32Array;
  mass: Float32Array;
  age: Float32Array;
  count: number;
}
```

**Dataset Loading**:

```typescript
import { getDatasetLoader } from '@/app/sim';

const loader = getDatasetLoader('/dataset');
await loader.initialize();

// Load parameters at specific time
const params = await loader.loadParametersAt(currentJD);

// Prefetch for smooth playback
await loader.prefetch(currentJD, 8);
```

---

### `sim/physics/` — Heliosphere Models

**Purpose**: Parametric surface generation from precomputed parameters.

**Files**:
- `HeliosphereSurface.ts` — HeliosphereSurface class, morphology models

**Morphology Types**:
- **Cometary**: Classic elongated tail (main sequence)
- **Croissant**: Flattened, bifurcated tail (RGB/AGB)
- **Bubble**: Nearly spherical (post-MS)

**Parameters** (per epoch):

```typescript
interface HeliosphereParameters {
  R_HP_nose: AU;           // Nose radius
  R_TS_over_HP: number;    // TS/HP ratio (0.75-0.85)
  nose_vec: [number, number, number];  // ISM inflow direction
  ISM_rho: number;         // ISM density (cm⁻³)
  ISM_T: number;           // ISM temperature (K)
  ISM_B: number;           // ISM B-field (nT)
  SW_Mdot: number;         // Solar mass loss rate
  SW_v: number;            // Solar wind speed (km/s)
  morphology: HeliosphereMorphology;
  shape_params: number[];  // Morphology-specific coefficients
}
```

**Usage**:

```typescript
import { HeliosphereSurface, interpolateParameters } from '@/app/sim/physics/HeliosphereSurface';

// Create surface from parameters
const surface = new HeliosphereSurface(params);

// Generate mesh (64 theta steps × 128 phi steps)
const { positions, indices, normals } = surface.generateMesh(64, 128, 'heliopause');

// Sample point on surface (for particle emission)
const theta = Units.Radians(Math.PI / 4);
const phi = Units.Radians(Math.PI / 2);
const point = surface.samplePoint(theta, phi, 'termination_shock');
```

---

### `sim/rendering/` — Rendering Systems

**Purpose**: GPU-optimized rendering for stars, particles, surfaces.

**Files**:
- `StarField.ts` — Instanced star rendering + panorama

**Starfield Features**:
- Instanced rendering (up to 20k stars)
- Magnitude-based sizing
- Color from temperature
- Optional panoramic background (KTX2 tiles)

```typescript
import { StarField } from '@/app/sim/rendering/StarField';

const starField = new StarField(registry, {
  maxStars: 20_000,
  nearbyRadiusAU: 6.5e6, // ~100 parsecs
  usePanorama: true,
});

scene.add(starField.getMesh());

if (starField.getPanorama()) {
  scene.add(starField.getPanorama());
}
```

---

### `sim/gpu/` — GPU Particle Systems

**Purpose**: WebGL2 ping-pong particle advection (no CPU loops).

**Files**:
- `ParticleSystem.ts` — GPUParticleSystem class

**How It Works**:
1. Store particle state in RGBA32F textures (position, velocity)
2. Ping-pong: read from texture A, write to texture B, swap
3. Display: sample textures in vertex shader, render as points

```typescript
import { GPUParticleSystem } from '@/app/sim/gpu/ParticleSystem';

const particles = new GPUParticleSystem(renderer, {
  maxParticles: 100_000,
  emissionRate: 1000,
  particleLifetime: 10.0,
});

// Each frame
particles.update(dt, registry.config.auToScene);
scene.add(particles.getMesh());
```

---

### `sim/validation/` — Validation & Testing

**Purpose**: Overlays and tests to verify scientific accuracy.

**Files**:
- `ValidationOverlays.ts` — Reference rings, Voyager tracks, arrows

**Overlays**:
- **Reference Rings**: TS/HP at Voyager crossing distances (84, 94, 119, 122 AU)
- **Voyager Tracks**: Trajectories with crossing markers
- **Apex Arrow**: Solar motion through ISM (orange)
- **IBEX Arrow**: ISM inflow direction (cyan)

**Validation Tests**:

```typescript
import { ValidationTests } from '@/app/sim/validation/ValidationOverlays';

// Run all tests
const pass = ValidationTests.runAll(
  registry,
  params.R_HP_nose,
  params.R_TS_over_HP
);

// Individual tests
ValidationTests.testHeliopauseScale(registry, params.R_HP_nose);
ValidationTests.testTSHPRatio(params.R_TS_over_HP);
ValidationTests.testISMDirection(registry.ismInflowDirection);
```

---

## Precompute Pipeline

### Python Script: `backend/precompute/generate_dataset.py`

**Purpose**: Generate heliosphere evolution dataset offline.

**Steps**:
1. Generate non-uniform time axis (0–13 Gyr)
2. Compute solar evolution (ZAMS → WD)
3. Calculate heliosphere parameters (R_HP, morphology, etc.)
4. Save as Zarr arrays + JSON fallback

**Run**:

```bash
python backend/precompute/generate_dataset.py
```

**Output**:

```
public/dataset/
  meta.json
  time/epochs.json
  heliosphere/epoch_000000.json
  heliosphere/epoch_000001.json
  ...
```

---

## Integration Example

### Complete Scene Setup

```typescript
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import {
  getRegistry,
  getDatasetLoader,
  Units,
  HeliosphereSurface,
  StarField,
  GPUParticleSystem,
  ValidationOverlays,
} from '@/app/sim';

async function createScene(canvas: HTMLCanvasElement) {
  // 1. Setup Three.js
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 5000);
  const controls = new OrbitControls(camera, canvas);

  camera.position.set(200, 150, 250);

  // 2. Initialize registry
  const registry = getRegistry({
    auToScene: 1.0,
    maxRenderDistance: Units.AU(500),
  });

  // 3. Load dataset
  const loader = getDatasetLoader('/dataset');
  await loader.initialize();

  // 4. Load heliosphere parameters
  const params = await loader.loadParametersAt(registry.getTime());

  // 5. Create heliosphere surface
  const hpSurface = new HeliosphereSurface(params);
  const hpMesh = hpSurface.generateMesh(64, 128, 'heliopause');

  const hpGeometry = new THREE.BufferGeometry();
  hpGeometry.setAttribute('position', new THREE.BufferAttribute(hpMesh.positions, 3));
  hpGeometry.setIndex(new THREE.BufferAttribute(hpMesh.indices, 1));
  hpGeometry.setAttribute('normal', new THREE.BufferAttribute(hpMesh.normals, 3));

  const hpMaterial = new THREE.MeshBasicMaterial({
    color: 0x4ecdc4,
    wireframe: true,
    transparent: true,
    opacity: 0.3,
  });

  const hpMeshObject = new THREE.Mesh(hpGeometry, hpMaterial);
  scene.add(hpMeshObject);

  // 6. Create starfield
  const starField = new StarField(registry);
  scene.add(starField.getMesh());
  if (starField.getPanorama()) {
    scene.add(starField.getPanorama());
  }

  // 7. Create particle system
  const particles = new GPUParticleSystem(renderer, {
    maxParticles: 50_000,
  });
  scene.add(particles.getMesh());

  // 8. Add validation overlays
  const validation = new ValidationOverlays(registry);
  scene.add(validation.getGroup());

  // Run validation tests
  ValidationTests.runAll(registry, params.R_HP_nose, params.R_TS_over_HP);

  // 9. Animation loop
  let lastTime = performance.now();

  function animate() {
    requestAnimationFrame(animate);

    const now = performance.now();
    const dt = (now - lastTime) / 1000;
    lastTime = now;

    controls.update();
    particles.update(dt, registry.config.auToScene);
    starField.update(dt);

    renderer.render(scene, camera);
  }

  animate();

  return { scene, camera, renderer, registry };
}
```

---

## Performance Guidelines

### Target Metrics

- **First paint**: < 2 seconds on 2019 laptops
- **Frame rate**: Steady 60 FPS with 20k stars + 50k particles
- **Dataset bandwidth**: < 10 MB typical session

### Optimizations

1. **SoA Layout**: Maximizes GPU fetch coherency
2. **Instanced Rendering**: Stars use single draw call
3. **Ping-Pong Particles**: No CPU-GPU transfer per frame
4. **LRU Cache**: Keep 8–16 epochs in memory
5. **HTTP Range Requests**: Load only needed chunks
6. **DPR Clamping**: Limit pixel ratio on mobile

---

## Scientific Accuracy Notes

### Present-Day Values (Validation)

- **Heliopause nose**: 121.6 AU (Voyager 1 crossing, 2012-08-25)
- **TS nose**: ~94 AU (Voyager 1 crossing, 2004-12-16)
- **TS/HP ratio**: 0.77 (nose), varies by direction
- **ISM inflow**: Galactic l≈255.4°, b≈5.2° (IBEX)
- **ISM speed**: 26.3 km/s

### Assumptions & Limits

- **Planets**: Illustrative only outside ±10 Myr of present
- **ISM**: Assumed constant (in reality, Sun moves through varying clouds)
- **Morphology**: Simplified models; real heliosphere has turbulence
- **Proper motion**: Only accurate within ±100 Myr of present

### References

- Opher et al. (2020) — Croissant heliosphere model
- Stone et al. (2013, 2019) — Voyager crossing papers
- McComas et al. (2012) — IBEX measurements
- JPL Horizons — Ephemeris data

---

## Future Enhancements

### WebGPU Compute Path

Replace WebGL2 ping-pong with WGSL compute shaders for better performance and advanced features (field line tracing, MHD simulation).

### Real Zarr Streaming

Implement proper Zarr HTTP range requests using `zarr.js` or similar.

### KTX2 Panorama Tiles

Replace simple gradient with HEALPix KTX2 tiles for high-quality Milky Way background.

### Dynamic Planet Orbits

Integrate ephemeris calculations for accurate planet positions across all epochs.

---

## Conclusion

This architecture provides a **scientifically accurate, performant, and scalable** foundation for heliosphere visualization. By separating concerns (data, physics, rendering) and using GPU-first techniques, we achieve real-time interaction with precomputed datasets spanning billions of years.

**Key Takeaway**: *Everything is Sun-centric in AU, with a single global scale to Three.js world space.*


