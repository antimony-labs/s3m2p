import * as THREE from 'three';

/**
 * ASTRONOMICAL REFERENCE DATA (2024)
 * 
 * The heliosphere's shape and orientation are determined by the Sun's motion
 * through the Local Interstellar Cloud (LIC). Based on NASA IBEX and Voyager data:
 * 
 * INTERSTELLAR WIND DIRECTION (where the heliosphere nose points):
 * - Ecliptic longitude: λ ≈ 255.4° ± 0.5°
 * - Ecliptic latitude: β ≈ 5.2° ± 0.2°
 * - Velocity: ~26.3 km/s (relative to Sun)
 * - Source: Scorpius-Centaurus Association
 * 
 * SOLAR APEX (Sun's motion relative to nearby stars):
 * - Right Ascension: 18h 28m (≈ 277°)
 * - Declination: +30°
 * - Constellation: Hercules (near λ Herculis)
 * - Velocity: ~19.5 km/s relative to Local Standard of Rest
 * 
 * GALACTIC MOTION:
 * - Sun orbits Milky Way center at ~230 km/s
 * - Direction: toward Cygnus (l=90°, b=0° in galactic coords)
 * - Period: ~220-250 million years
 */

// Interstellar wind direction (IBEX measurements, J2000 ecliptic coordinates)
export const INTERSTELLAR_WIND_LON = (255.4 * Math.PI) / 180; // λ in radians
export const INTERSTELLAR_WIND_LAT = (5.2 * Math.PI) / 180;   // β in radians

// Solar apex in equatorial coordinates (for reference)
export const APEX_RA = (18.466667 * Math.PI) / 12; // 18h 28m → radians
export const APEX_DEC = (30 * Math.PI) / 180;      // +30° → radians

// Ecliptic obliquity (J2000)
export const ECLIPTIC_TILT = (23.43928 * Math.PI) / 180;

/**
 * Convert ecliptic coordinates to Cartesian unit vector
 * @param lon - Ecliptic longitude in radians
 * @param lat - Ecliptic latitude in radians
 * @returns Unit vector in ecliptic coordinate system
 */
export function eclipticToVec3(lon: number, lat: number): THREE.Vector3 {
  const x = Math.cos(lat) * Math.cos(lon);
  const y = Math.cos(lat) * Math.sin(lon);
  const z = Math.sin(lat);
  return new THREE.Vector3(x, y, z).normalize();
}

/**
 * Convert RA/Dec to Cartesian unit vector (equatorial coords)
 * @param ra - Right ascension in radians
 * @param dec - Declination in radians
 * @returns Unit vector in equatorial coordinate system
 */
export function radecToVec3(ra: number, dec: number): THREE.Vector3 {
  const x = Math.cos(dec) * Math.cos(ra);
  const y = Math.cos(dec) * Math.sin(ra);
  const z = Math.sin(dec);
  return new THREE.Vector3(x, y, z).normalize();
}

// The heliosphere nose points into the interstellar wind
export const HELIOSPHERE_NOSE = eclipticToVec3(INTERSTELLAR_WIND_LON, INTERSTELLAR_WIND_LAT);

// Solar apex direction (for reference - Sun's motion relative to local stars)
export const APEX_DIR = radecToVec3(APEX_RA, APEX_DEC);

/**
 * Create a basis matrix oriented with the heliosphere nose
 * X-axis: Points toward the interstellar wind (heliosphere nose/upwind)
 * Y-axis: Approximately "up" relative to ecliptic
 * Z-axis: Completes right-handed system
 */
export function basisFromApex(): THREE.Matrix4 {
  // X-axis aligns with heliosphere nose (upwind direction)
  const x = HELIOSPHERE_NOSE.clone().normalize();
  
  // Use ecliptic north as reference for "up"
  const eclipticNorth = new THREE.Vector3(0, 0, 1);
  
  // If nose is nearly aligned with ecliptic pole, use different reference
  let upHelper = eclipticNorth;
  if (Math.abs(x.dot(eclipticNorth)) > 0.95) {
    upHelper = new THREE.Vector3(1, 0, 0); // Use x-axis as helper
  }
  
  // Create orthonormal basis
  const z = new THREE.Vector3().crossVectors(x, upHelper).normalize();
  const y = new THREE.Vector3().crossVectors(z, x).normalize();
  
  return new THREE.Matrix4().makeBasis(x, y, z);
}

/**
 * Physical scales and velocities for accurate simulation
 */
export const PHYSICAL_SCALES = {
  // Heliosphere dimensions (in AU)
  TERMINATION_SHOCK: 90,      // Where solar wind slows down
  HELIOPAUSE: 120,             // Edge of heliosphere
  BOW_SHOCK: 230,              // If it exists (debated)
  
  // Velocities (km/s)
  SOLAR_WIND: 400,             // Typical solar wind speed
  INTERSTELLAR_MEDIUM: 26.3,   // ISM relative to Sun
  GALACTIC_ORBIT: 230,         // Sun around Milky Way
  PECULIAR_VELOCITY: 19.5,     // Sun relative to LSR
  
  // Time scales
  GALACTIC_YEAR_MY: 225,       // Million years for one orbit
  
  // Directions (for reference frame indicators)
  GALACTIC_CENTER: { l: 0, b: 0 },        // Sagittarius A*
  GALACTIC_NORTH: { l: 0, b: 90 },        // North galactic pole
  GALACTIC_MOTION: { l: 90, b: 0 },       // Direction of solar orbit
};