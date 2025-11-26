/**
 * Sun-Centric Heliosphere Simulation Core
 * 
 * This module provides a complete framework for scientifically accurate
 * heliosphere visualization with precomputed datasets and GPU rendering.
 * 
 * @module sim
 */

// Types and Units
export * from './types/units';
export * from './types/vectors';

// Coordinate Frames
export * from './frames/CoordinateFrame';

// Registry (central data management)
export * from './registry/Registry';

// Data Structures
export * from './data/StructureOfArrays';
export * from './data/DatasetLoader';

// Physics Models
export * from './physics/HeliosphereSurface';

// Rendering
export * from './rendering/StarField';

// GPU Systems
export * from './gpu/ParticleSystem';

// Validation
export * from './validation/ValidationOverlays';

/**
 * Quick start guide:
 * 
 * ```typescript
 * import { getRegistry, getDatasetLoader, Units } from '@/app/sim';
 * 
 * // 1. Initialize registry with config
 * const registry = getRegistry({
 *   auToScene: 1.0,
 *   maxRenderDistance: Units.AU(500),
 * });
 * 
 * // 2. Load dataset
 * const loader = getDatasetLoader('/dataset');
 * await loader.initialize();
 * 
 * // 3. Load parameters for current time
 * const params = await loader.loadParametersAt(registry.getTime());
 * 
 * // 4. Create heliosphere surface
 * const surface = new HeliosphereSurface(params);
 * const mesh = surface.generateMesh(64, 128);
 * 
 * // 5. Add to scene and render!
 * ```
 */

