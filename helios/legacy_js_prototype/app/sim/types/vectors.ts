/**
 * Type-safe 3D vectors in various coordinate frames
 */

import { AU, KmPerSec, Radians } from './units';

/**
 * Base 3D vector interface
 */
export interface Vec3<T = number> {
  x: T;
  y: T;
  z: T;
}

/**
 * Position vector in AU (Sun-centric)
 */
export interface PositionAU extends Vec3<AU> {}

/**
 * Velocity vector in km/s
 */
export interface VelocityKmS extends Vec3<KmPerSec> {}

/**
 * Unit direction vector (dimensionless)
 */
export interface UnitVector extends Vec3<number> {}

/**
 * Spherical coordinates
 */
export interface SphericalCoords {
  r: AU;          // Radial distance
  theta: Radians; // Polar angle (0 at +Z, π at -Z)
  phi: Radians;   // Azimuthal angle (0 at +X, π/2 at +Y)
}

/**
 * Vector utilities
 */
export const Vec3Utils = {
  /**
   * Create a position vector in AU
   */
  position(x: AU, y: AU, z: AU): PositionAU {
    return { x, y, z };
  },

  /**
   * Create a velocity vector in km/s
   */
  velocity(x: KmPerSec, y: KmPerSec, z: KmPerSec): VelocityKmS {
    return { x, y, z };
  },

  /**
   * Create a unit vector
   */
  unit(x: number, y: number, z: number): UnitVector {
    const mag = Math.sqrt(x * x + y * y + z * z);
    if (mag === 0) return { x: 0, y: 0, z: 1 };
    return { x: x / mag, y: y / mag, z: z / mag };
  },

  /**
   * Dot product
   */
  dot<T extends number>(a: Vec3<T>, b: Vec3<T>): number {
    return (a.x as number) * (b.x as number) + 
           (a.y as number) * (b.y as number) + 
           (a.z as number) * (b.z as number);
  },

  /**
   * Cross product (returns plain numbers, caller must re-type)
   */
  cross(a: Vec3, b: Vec3): Vec3<number> {
    return {
      x: (a.y as number) * (b.z as number) - (a.z as number) * (b.y as number),
      y: (a.z as number) * (b.x as number) - (a.x as number) * (b.z as number),
      z: (a.x as number) * (b.y as number) - (a.y as number) * (b.x as number),
    };
  },

  /**
   * Magnitude
   */
  magnitude<T extends number>(v: Vec3<T>): number {
    return Math.sqrt(
      (v.x as number) ** 2 + 
      (v.y as number) ** 2 + 
      (v.z as number) ** 2
    );
  },

  /**
   * Normalize to unit vector
   */
  normalize<T extends number>(v: Vec3<T>): UnitVector {
    const mag = Vec3Utils.magnitude(v);
    if (mag === 0) return { x: 0, y: 0, z: 1 };
    return {
      x: (v.x as number) / mag,
      y: (v.y as number) / mag,
      z: (v.z as number) / mag,
    };
  },

  /**
   * Scale vector by scalar
   */
  scale<T extends number>(v: Vec3<T>, s: number): Vec3<T> {
    return {
      x: ((v.x as number) * s) as T,
      y: ((v.y as number) * s) as T,
      z: ((v.z as number) * s) as T,
    };
  },

  /**
   * Add two vectors
   */
  add<T extends number>(a: Vec3<T>, b: Vec3<T>): Vec3<T> {
    return {
      x: ((a.x as number) + (b.x as number)) as T,
      y: ((a.y as number) + (b.y as number)) as T,
      z: ((a.z as number) + (b.z as number)) as T,
    };
  },

  /**
   * Subtract vectors (a - b)
   */
  subtract<T extends number>(a: Vec3<T>, b: Vec3<T>): Vec3<T> {
    return {
      x: ((a.x as number) - (b.x as number)) as T,
      y: ((a.y as number) - (b.y as number)) as T,
      z: ((a.z as number) - (b.z as number)) as T,
    };
  },

  /**
   * Convert Cartesian to spherical coordinates
   */
  toSpherical(v: PositionAU): SphericalCoords {
    const x = v.x as number;
    const y = v.y as number;
    const z = v.z as number;
    
    const r = Math.sqrt(x * x + y * y + z * z) as AU;
    const theta = Math.acos(z / (r as number)) as Radians;
    const phi = Math.atan2(y, x) as Radians;
    
    return { r, theta, phi };
  },

  /**
   * Convert spherical to Cartesian coordinates
   */
  fromSpherical(coords: SphericalCoords): PositionAU {
    const r = coords.r as number;
    const theta = coords.theta as number;
    const phi = coords.phi as number;
    
    const sinTheta = Math.sin(theta);
    
    return {
      x: (r * sinTheta * Math.cos(phi)) as AU,
      y: (r * sinTheta * Math.sin(phi)) as AU,
      z: (r * Math.cos(theta)) as AU,
    };
  },

  /**
   * Linear interpolation between two vectors
   */
  lerp<T extends number>(a: Vec3<T>, b: Vec3<T>, t: number): Vec3<T> {
    return {
      x: ((a.x as number) * (1 - t) + (b.x as number) * t) as T,
      y: ((a.y as number) * (1 - t) + (b.y as number) * t) as T,
      z: ((a.z as number) * (1 - t) + (b.z as number) * t) as T,
    };
  },

  /**
   * Clone a vector
   */
  clone<T extends number>(v: Vec3<T>): Vec3<T> {
    return { x: v.x, y: v.y, z: v.z };
  },
};

