/**
 * HELIOSPHERIC PHYSICS CONSTANTS
 * Based on NASA Voyager 1 & 2, IBEX, and Ulysses mission data
 * 
 * References:
 * - Voyager 1 crossed termination shock at 94 AU (2004)
 * - Voyager 1 crossed heliopause at 121 AU (2012)
 * - Voyager 2 crossed termination shock at 84 AU (2007)
 * - Voyager 2 crossed heliopause at 119 AU (2018)
 */

import * as THREE from 'three';

// Distance scales (in Astronomical Units)
export const HELIOSPHERIC_DISTANCES = {
  SUN_RADIUS: 0.00465,           // ~700,000 km in AU
  TERMINATION_SHOCK_NOSE: 90,    // AU (upwind, compressed)
  TERMINATION_SHOCK_TAIL: 200,   // AU (downwind, extended)
  HELIOPAUSE_NOSE: 120,          // AU (upwind)
  HELIOPAUSE_TAIL: 300,          // AU (downwind, extended tail)
  HELIOSHEATH_WIDTH: 30,         // AU (typical width)
} as const;

// Velocities (km/s)
export const HELIOSPHERIC_VELOCITIES = {
  SOLAR_WIND_INNER: 400,         // km/s near Sun (fast solar wind)
  SOLAR_WIND_OUTER: 350,         // km/s at 1 AU
  SOLAR_WIND_TERMINATION: 100,   // km/s after termination shock (subsonic)
  ISM_RELATIVE: 26.3,            // km/s ISM relative to Sun
  ISM_TEMPERATURE: 6300,         // K (ISM temperature)
} as const;

// Density profiles
export const HELIOSPHERIC_DENSITY = {
  SOLAR_WIND_1AU: 5,             // particles/cm³ at 1 AU
  SOLAR_WIND_SCALING: 2,         // Power law: n ∝ r^-2
  ISM_DENSITY: 0.1,              // particles/cm³ (local ISM)
  HELIOSHEATH_COMPRESSION: 2.5,  // Density increase at termination shock
} as const;

// Pressure balance equation at heliopause:
// ρ_sw * v_sw² = ρ_ism * v_ism²
// This determines the heliopause location

// Magnetic field (nT)
export const HELIOSPHERIC_MAGNETIC = {
  SOLAR_WIND_1AU: 5,             // nT at 1 AU
  SOLAR_WIND_SCALING: 1,         // B ∝ r^-1 (radial)
  ISM_FIELD: 0.3,                // nT (local ISM)
} as const;

/**
 * Calculate solar wind density at distance r (in AU)
 * Density follows inverse square law: n(r) = n₀ * (r₀/r)²
 */
export function solarWindDensity(r: number): number {
  if (r < 0.1) return HELIOSPHERIC_DENSITY.SOLAR_WIND_1AU * 100; // Very close to Sun
  return HELIOSPHERIC_DENSITY.SOLAR_WIND_1AU * Math.pow(1.0 / r, HELIOSPHERIC_DENSITY.SOLAR_WIND_SCALING);
}

/**
 * Calculate solar wind velocity at distance r (in AU)
 * Velocity decreases slightly with distance due to expansion
 */
export function solarWindVelocity(r: number): number {
  if (r < HELIOSPHERIC_DISTANCES.TERMINATION_SHOCK_NOSE) {
    // Before termination shock: supersonic, slight decrease
    return HELIOSPHERIC_VELOCITIES.SOLAR_WIND_OUTER * (1 - r * 0.001);
  } else {
    // After termination shock: subsonic, compressed
    return HELIOSPHERIC_VELOCITIES.SOLAR_WIND_TERMINATION;
  }
}

/**
 * Calculate heliopause distance in a given direction
 * Accounts for compression on upwind side, extension on downwind
 */
export function heliopauseDistance(direction: THREE.Vector3, noseDirection: THREE.Vector3): number {
  const dot = direction.dot(noseDirection);
  // Upwind (nose): compressed, downwind (tail): extended
  const compression = 0.3; // Nose compressed by ~30%
  const extension = 2.5;   // Tail extended by ~2.5x
  
  if (dot > 0.5) {
    // Upwind side (nose)
    return HELIOSPHERIC_DISTANCES.HELIOPAUSE_NOSE * (1 - compression * dot);
  } else if (dot < -0.3) {
    // Downwind side (tail)
    return HELIOSPHERIC_DISTANCES.HELIOPAUSE_TAIL * (1 + extension * Math.abs(dot));
  } else {
    // Sides
    return HELIOSPHERIC_DISTANCES.HELIOPAUSE_NOSE;
  }
}

/**
 * Calculate termination shock distance
 */
export function terminationShockDistance(direction: THREE.Vector3, noseDirection: THREE.Vector3): number {
  const dot = direction.dot(noseDirection);
  const compression = 0.25;
  const extension = 2.0;
  
  if (dot > 0.5) {
    return HELIOSPHERIC_DISTANCES.TERMINATION_SHOCK_NOSE * (1 - compression * dot);
  } else if (dot < -0.3) {
    return HELIOSPHERIC_DISTANCES.TERMINATION_SHOCK_TAIL * (1 + extension * Math.abs(dot));
  } else {
    return HELIOSPHERIC_DISTANCES.TERMINATION_SHOCK_NOSE;
  }
}

/**
 * Pressure at heliopause (simplified)
 * P = ρ * v² (dynamic pressure)
 * Returns pressure values for verification
 */
export function pressureAtHeliopause(): { solarWind: number; ism: number; ratio: number } {
  const swDensity = solarWindDensity(HELIOSPHERIC_DISTANCES.HELIOPAUSE_NOSE);
  const swVelocity = solarWindVelocity(HELIOSPHERIC_DISTANCES.HELIOPAUSE_NOSE);
  const swPressure = swDensity * swVelocity * swVelocity;
  
  const ismDensity = HELIOSPHERIC_DENSITY.ISM_DENSITY;
  const ismVelocity = HELIOSPHERIC_VELOCITIES.ISM_RELATIVE;
  const ismPressure = ismDensity * ismVelocity * ismVelocity;
  
  // At heliopause, these should balance
  return { solarWind: swPressure, ism: ismPressure, ratio: swPressure / ismPressure };
}
