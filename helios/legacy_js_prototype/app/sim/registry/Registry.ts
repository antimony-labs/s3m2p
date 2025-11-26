/**
 * Central registry for heliosphere simulation state
 * Single source of truth for all simulation data
 */

import { CoordinateFrame, CoordinateTransforms } from '../frames/CoordinateFrame';
import { JulianDate, AU, Units } from '../types/units';
import { PositionAU, UnitVector } from '../types/vectors';
import { ParticleArrays, StarArrays, SpacecraftArrays } from '../data/StructureOfArrays';

/**
 * Global simulation configuration
 */
export interface SimConfig {
  // Primary coordinate frame (always HEE_J2000 internally)
  frame: CoordinateFrame;

  // Current simulation time
  julianDate: JulianDate;

  // Scale factor: AU to scene units (Three.js world space)
  // Example: auToScene = 1.0 means 1 AU = 1 scene unit
  auToScene: number;

  // Maximum render distance in AU
  maxRenderDistance: AU;

  // Performance settings
  targetFPS: number;
  enableGPUParticles: boolean;
  maxParticles: number;
  maxStars: number;
}

/**
 * Heliosphere surface configuration
 */
export interface HeliosphereSurfaceConfig {
  name: 'heliopause' | 'termination_shock' | 'bow_shock';
  
  // Parametric function: (theta, phi, params) => radius in AU
  radiusFunction: (theta: number, phi: number) => AU;
  
  // Visibility
  visible: boolean;
  
  // Visual properties (for renderer)
  color: number;
  opacity: number;
  wireframe: boolean;
}

/**
 * Celestial body (planet, spacecraft)
 */
export interface CelestialBody {
  id: string;
  name: string;
  position: PositionAU;    // Current position in HEE_J2000
  radius: AU;              // Physical radius (for scaling)
  visualScale: number;     // Scale multiplier for rendering
  color: number;
  visible: boolean;
  type: 'planet' | 'spacecraft' | 'dwarf_planet';
}

/**
 * Central simulation registry
 */
export class Registry {
  // Configuration
  public config: SimConfig;

  // Coordinate transforms
  public transforms: CoordinateTransforms;

  // Heliosphere surfaces
  public surfaces: Map<string, HeliosphereSurfaceConfig>;

  // Particle systems (SoA)
  public solarWindParticles: ParticleArrays;
  public ismParticles: ParticleArrays;

  // Stars (SoA)
  public nearbyStars: StarArrays;

  // Celestial bodies
  public bodies: Map<string, CelestialBody>;

  // Spacecraft trajectories
  public spacecraftTrajectories: Map<string, SpacecraftArrays>;

  // ISM parameters (from precomputed data or fixed)
  public ismInflowDirection: UnitVector;
  public ismSpeed: number; // km/s

  constructor(config?: Partial<SimConfig>) {
    // Default configuration
    this.config = {
      frame: CoordinateFrame.HEE_J2000,
      julianDate: Units.JulianDate(Date.now() / 86400000 + 2440587.5),
      auToScene: 1.0,
      maxRenderDistance: Units.AU(500),
      targetFPS: 60,
      enableGPUParticles: true,
      maxParticles: 100_000,
      maxStars: 20_000,
      ...config,
    };

    // Initialize coordinate transforms
    this.transforms = new CoordinateTransforms();
    this.ismInflowDirection = this.transforms.getIsmInflowDirection();
    this.ismSpeed = 26.3; // km/s (IBEX measurement)

    // Initialize collections
    this.surfaces = new Map();
    this.bodies = new Map();
    this.spacecraftTrajectories = new Map();

    // Initialize particle systems (will be populated later)
    this.solarWindParticles = { count: 0, capacity: 0 } as ParticleArrays;
    this.ismParticles = { count: 0, capacity: 0 } as ParticleArrays;
    this.nearbyStars = { count: 0, capacity: 0 } as StarArrays;
  }

  /**
   * Update simulation time
   */
  setTime(julianDate: JulianDate): void {
    this.config.julianDate = julianDate;
  }

  /**
   * Get current time
   */
  getTime(): JulianDate {
    return this.config.julianDate;
  }

  /**
   * Add or update a heliosphere surface
   */
  setSurface(surface: HeliosphereSurfaceConfig): void {
    this.surfaces.set(surface.name, surface);
  }

  /**
   * Get heliosphere surface by name
   */
  getSurface(name: string): HeliosphereSurfaceConfig | undefined {
    return this.surfaces.get(name);
  }

  /**
   * Add or update a celestial body
   */
  setBody(body: CelestialBody): void {
    this.bodies.set(body.id, body);
  }

  /**
   * Get celestial body by ID
   */
  getBody(id: string): CelestialBody | undefined {
    return this.bodies.get(id);
  }

  /**
   * Add spacecraft trajectory
   */
  setSpacecraftTrajectory(id: string, trajectory: SpacecraftArrays): void {
    this.spacecraftTrajectories.set(id, trajectory);
  }

  /**
   * Get spacecraft position at current time (interpolated)
   */
  getSpacecraftPosition(id: string): PositionAU | null {
    const trajectory = this.spacecraftTrajectories.get(id);
    if (!trajectory || trajectory.count === 0) return null;

    const currentJD = this.config.julianDate as number;

    // Binary search for time bracket
    let low = 0;
    let high = trajectory.count - 1;

    // Handle out of range
    if (currentJD <= trajectory.time[low]) {
      return {
        x: trajectory.posX[low] as AU,
        y: trajectory.posY[low] as AU,
        z: trajectory.posZ[low] as AU,
      };
    }
    if (currentJD >= trajectory.time[high]) {
      return {
        x: trajectory.posX[high] as AU,
        y: trajectory.posY[high] as AU,
        z: trajectory.posZ[high] as AU,
      };
    }

    // Binary search
    while (high - low > 1) {
      const mid = Math.floor((low + high) / 2);
      if (trajectory.time[mid] <= currentJD) {
        low = mid;
      } else {
        high = mid;
      }
    }

    // Linear interpolation
    const t0 = trajectory.time[low];
    const t1 = trajectory.time[high];
    const alpha = (currentJD - t0) / (t1 - t0);

    return {
      x: ((1 - alpha) * trajectory.posX[low] + alpha * trajectory.posX[high]) as AU,
      y: ((1 - alpha) * trajectory.posY[low] + alpha * trajectory.posY[high]) as AU,
      z: ((1 - alpha) * trajectory.posZ[low] + alpha * trajectory.posZ[high]) as AU,
    };
  }

  /**
   * Convert position from HEE_J2000 to scene coordinates (Three.js)
   */
  heeToScene(pos: PositionAU): { x: number; y: number; z: number } {
    return {
      x: (pos.x as number) * this.config.auToScene,
      y: (pos.y as number) * this.config.auToScene,
      z: (pos.z as number) * this.config.auToScene,
    };
  }

  /**
   * Convert distance from AU to scene units
   */
  auToSceneDistance(au: AU): number {
    return (au as number) * this.config.auToScene;
  }

  /**
   * Convert distance from scene units to AU
   */
  sceneToAU(distance: number): AU {
    return Units.AU(distance / this.config.auToScene);
  }

  /**
   * Dispose resources
   */
  dispose(): void {
    this.surfaces.clear();
    this.bodies.clear();
    this.spacecraftTrajectories.clear();
  }
}

/**
 * Global registry instance (singleton pattern)
 */
let globalRegistry: Registry | null = null;

/**
 * Get or create global registry
 */
export function getRegistry(config?: Partial<SimConfig>): Registry {
  if (!globalRegistry) {
    globalRegistry = new Registry(config);
  }
  return globalRegistry;
}

/**
 * Reset global registry (useful for testing)
 */
export function resetRegistry(): void {
  if (globalRegistry) {
    globalRegistry.dispose();
  }
  globalRegistry = null;
}

