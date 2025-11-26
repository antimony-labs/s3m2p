/**
 * Structure-of-Arrays (SoA) data layout for efficient GPU access
 * Maximizes memory coherency and allows direct GPU buffer uploads
 */

import { AU, KmPerSec } from '../types/units';
import { PositionAU, VelocityKmS } from '../types/vectors';

/**
 * Particle system using SoA layout
 * Suitable for solar wind and ISM particles
 */
export interface ParticleArrays {
  // Positions (3 separate arrays for x, y, z in AU)
  posX: Float32Array;
  posY: Float32Array;
  posZ: Float32Array;

  // Velocities (3 separate arrays for vx, vy, vz in km/s)
  velX: Float32Array;
  velY: Float32Array;
  velZ: Float32Array;

  // Properties
  mass: Float32Array;      // Particle mass (normalized)
  age: Float32Array;       // Age in simulation time
  temperature: Float32Array; // Temperature in K

  // Metadata
  count: number;           // Current active particle count
  capacity: number;        // Maximum particles
}

/**
 * Star catalog using SoA layout
 * Optimized for instanced rendering
 */
export interface StarArrays {
  // Positions in Sun-centric HEE_J2000 (AU)
  posX: Float32Array;
  posY: Float32Array;
  posZ: Float32Array;

  // Visual properties
  magnitude: Float32Array;  // Apparent magnitude
  colorR: Uint8Array;       // RGB color (0-255)
  colorG: Uint8Array;
  colorB: Uint8Array;

  // Optional: proper motion (for nearby stars only)
  pmRA: Float32Array | null;  // mas/yr in RA
  pmDec: Float32Array | null; // mas/yr in Dec

  // Metadata
  count: number;
  capacity: number;
}

/**
 * Spacecraft trajectory using SoA layout
 */
export interface SpacecraftArrays {
  // Positions at each time step (AU)
  posX: Float32Array;
  posY: Float32Array;
  posZ: Float32Array;

  // Timestamps (Julian Date)
  time: Float64Array;

  // Status flags
  operational: Uint8Array; // 0 = inactive, 1 = active

  count: number;
  capacity: number;
}

/**
 * SoA utilities
 */
export const SoA = {
  /**
   * Create empty particle arrays
   */
  createParticles(capacity: number): ParticleArrays {
    return {
      posX: new Float32Array(capacity),
      posY: new Float32Array(capacity),
      posZ: new Float32Array(capacity),
      velX: new Float32Array(capacity),
      velY: new Float32Array(capacity),
      velZ: new Float32Array(capacity),
      mass: new Float32Array(capacity),
      age: new Float32Array(capacity),
      temperature: new Float32Array(capacity),
      count: 0,
      capacity,
    };
  },

  /**
   * Create empty star arrays
   */
  createStars(capacity: number, includeProperMotion: boolean = false): StarArrays {
    return {
      posX: new Float32Array(capacity),
      posY: new Float32Array(capacity),
      posZ: new Float32Array(capacity),
      magnitude: new Float32Array(capacity),
      colorR: new Uint8Array(capacity),
      colorG: new Uint8Array(capacity),
      colorB: new Uint8Array(capacity),
      pmRA: includeProperMotion ? new Float32Array(capacity) : null,
      pmDec: includeProperMotion ? new Float32Array(capacity) : null,
      count: 0,
      capacity,
    };
  },

  /**
   * Create empty spacecraft arrays
   */
  createSpacecraft(capacity: number): SpacecraftArrays {
    return {
      posX: new Float32Array(capacity),
      posY: new Float32Array(capacity),
      posZ: new Float32Array(capacity),
      time: new Float64Array(capacity),
      operational: new Uint8Array(capacity),
      count: 0,
      capacity,
    };
  },

  /**
   * Add particle to SoA
   */
  addParticle(
    arrays: ParticleArrays,
    pos: PositionAU,
    vel: VelocityKmS,
    mass: number,
    temperature: number
  ): boolean {
    if (arrays.count >= arrays.capacity) return false;

    const idx = arrays.count;
    arrays.posX[idx] = pos.x as number;
    arrays.posY[idx] = pos.y as number;
    arrays.posZ[idx] = pos.z as number;
    arrays.velX[idx] = vel.x as number;
    arrays.velY[idx] = vel.y as number;
    arrays.velZ[idx] = vel.z as number;
    arrays.mass[idx] = mass;
    arrays.age[idx] = 0;
    arrays.temperature[idx] = temperature;
    arrays.count++;

    return true;
  },

  /**
   * Add star to SoA
   */
  addStar(
    arrays: StarArrays,
    pos: PositionAU,
    magnitude: number,
    color: { r: number; g: number; b: number },
    properMotion?: { ra: number; dec: number }
  ): boolean {
    if (arrays.count >= arrays.capacity) return false;

    const idx = arrays.count;
    arrays.posX[idx] = pos.x as number;
    arrays.posY[idx] = pos.y as number;
    arrays.posZ[idx] = pos.z as number;
    arrays.magnitude[idx] = magnitude;
    arrays.colorR[idx] = color.r;
    arrays.colorG[idx] = color.g;
    arrays.colorB[idx] = color.b;

    if (properMotion && arrays.pmRA && arrays.pmDec) {
      arrays.pmRA[idx] = properMotion.ra;
      arrays.pmDec[idx] = properMotion.dec;
    }

    arrays.count++;
    return true;
  },

  /**
   * Get particle position at index
   */
  getParticlePosition(arrays: ParticleArrays, index: number): PositionAU {
    return {
      x: arrays.posX[index] as AU,
      y: arrays.posY[index] as AU,
      z: arrays.posZ[index] as AU,
    };
  },

  /**
   * Get particle velocity at index
   */
  getParticleVelocity(arrays: ParticleArrays, index: number): VelocityKmS {
    return {
      x: arrays.velX[index] as KmPerSec,
      y: arrays.velY[index] as KmPerSec,
      z: arrays.velZ[index] as KmPerSec,
    };
  },

  /**
   * Pack RGB8 color into RGBA32F for GPU texture
   * (useful for WebGL2 texture-based storage)
   */
  packColorToFloat(r: number, g: number, b: number, a: number = 255): number {
    return (
      (r & 0xff) |
      ((g & 0xff) << 8) |
      ((b & 0xff) << 16) |
      ((a & 0xff) << 24)
    );
  },

  /**
   * Create interleaved RGBA32F texture data for WebGL2 ping-pong
   * Packs position (xyz) and velocity (xyz) into two RGBA textures
   */
  createParticleTextures(
    arrays: ParticleArrays,
    textureSize: number
  ): {
    positionData: Float32Array;
    velocityData: Float32Array;
  } {
    const capacity = textureSize * textureSize;
    const positionData = new Float32Array(capacity * 4); // RGBA
    const velocityData = new Float32Array(capacity * 4);

    for (let i = 0; i < arrays.count; i++) {
      const idx = i * 4;
      
      // Position texture: RGB = xyz, A = mass
      positionData[idx + 0] = arrays.posX[i];
      positionData[idx + 1] = arrays.posY[i];
      positionData[idx + 2] = arrays.posZ[i];
      positionData[idx + 3] = arrays.mass[i];

      // Velocity texture: RGB = vxvyvz, A = age
      velocityData[idx + 0] = arrays.velX[i];
      velocityData[idx + 1] = arrays.velY[i];
      velocityData[idx + 2] = arrays.velZ[i];
      velocityData[idx + 3] = arrays.age[i];
    }

    return { positionData, velocityData };
  },
};

