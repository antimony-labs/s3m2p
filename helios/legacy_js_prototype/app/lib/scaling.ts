/**
 * Scaling utilities for heliosphere visualization
 * Provides consistent conversion between real units (km, AU) and scene units
 */

export const AU_IN_KM = 149597871; // 1 AU in kilometers
export const SCENE_SCALE = 0.03; // AU per scene unit (based on heliosphere scale)

// Physical constants
export const SUN_RADIUS_KM = 696000; // Solar radius in km

// Visibility scales - how much to exaggerate sizes for visibility
export const PLANET_VISIBILITY_SCALE = 20000; // Planets use large scale (tiny in reality)
export const SUN_VISIBILITY_SCALE = 200; // Sun uses smaller scale (already bright)

/**
 * Convert kilometers to Astronomical Units
 */
export function kmToAU(km: number): number {
  return km / AU_IN_KM;
}

/**
 * Convert AU to scene units
 */
export function auToSceneUnits(au: number): number {
  return au * SCENE_SCALE;
}

/**
 * Convert kilometers directly to scene units
 */
export function kmToSceneUnits(km: number): number {
  const au = kmToAU(km);
  return auToSceneUnits(au);
}

/**
 * Calculate visible planet size in scene units
 * Applies realistic proportional scaling with visibility enhancement
 */
export function planetVisibleSize(radiusKm: number): number {
  const sceneUnits = kmToSceneUnits(radiusKm);
  return sceneUnits * PLANET_VISIBILITY_SCALE;
}

/**
 * Calculate visible sun size in scene units
 * Uses smaller visibility scale than planets
 */
export function sunVisibleSize(radiusKm: number = 696000): number {
  const sceneUnits = kmToSceneUnits(radiusKm);
  return sceneUnits * SUN_VISIBILITY_SCALE;
}

/**
 * Calculate visible moon size in scene units
 * Uses same scale as planets for consistency
 */
export function moonVisibleSize(radiusKm: number = 1737.4): number {
  const sceneUnits = kmToSceneUnits(radiusKm);
  return sceneUnits * PLANET_VISIBILITY_SCALE;
}

/**
 * Calculate moon orbital radius in scene units
 */
export function moonOrbitRadius(orbitKm: number = 384400): number {
  return kmToSceneUnits(orbitKm);
}

/**
 * Convert orbital distance from AU to scene units
 */
export function orbitalRadiusToScene(orbitAU: number): number {
  return auToSceneUnits(orbitAU);
}

/**
 * Calculate planet angle from year and orbital period
 * Handles negative years correctly
 */
export function calculatePlanetAngle(year: number, periodYears: number): number {
  let normalizedYear = year % periodYears;
  if (normalizedYear < 0) {
    normalizedYear += periodYears;
  }
  return (normalizedYear / periodYears) * Math.PI * 2;
}

/**
 * Planet physical properties
 */
export const CELESTIAL_RADII_KM = {
  Sun: 696000,
  Mercury: 2439.7,
  Venus: 6051.8,
  Earth: 6371.0,
  Mars: 3389.5,
  Jupiter: 69911,
  Saturn: 58232,
  Uranus: 25362,
  Neptune: 24622,
  Pluto: 1188.3,
  Moon: 1737.4
} as const;

/**
 * Orbital distances in AU
 */
export const ORBITAL_DISTANCES_AU = {
  Mercury: 0.387,
  Venus: 0.723,
  Earth: 1.0,
  Mars: 1.524,
  Jupiter: 5.203,
  Saturn: 9.537,
  Uranus: 19.191,
  Neptune: 30.069,
  Pluto: 39.482
} as const;

/**
 * Orbital periods in Earth years
 */
export const ORBITAL_PERIODS_YEARS = {
  Mercury: 0.241,
  Venus: 0.615,
  Earth: 1.0,
  Mars: 1.881,
  Jupiter: 11.86,
  Saturn: 29.46,
  Uranus: 84.01,
  Neptune: 164.8,
  Pluto: 247.92
} as const;

/**
 * Moon constants
 */
export const MOON_ORBIT_KM = 384400;
export const MOON_PERIOD_DAYS = 27.32;
export const MOON_PERIOD_YEARS = MOON_PERIOD_DAYS / 365.25;

