/**
 * Interstellar objects data and visualization
 * Includes 'Oumuamua, 2I/Borisov, Andromeda Galaxy, and other notable objects
 */

import * as THREE from 'three';
import { JulianDate, Vector3TimeSeries } from './AstronomicalDataStore';
import { CoordinateTransforms } from '../physics/CoordinateTransforms';

/**
 * Interstellar object types
 */
export type InterstellarObjectType = 'asteroid' | 'comet' | 'galaxy' | 'star' | 'dwarf_planet';

/**
 * Interstellar object data
 */
export interface InterstellarObject {
  name: string;
  designation: string;
  type: InterstellarObjectType;
  discoveryDate: Date;
  currentPosition: THREE.Vector3; // AU
  velocity: THREE.Vector3; // km/s
  trajectory?: Vector3TimeSeries;
  distance: number; // Distance from Sun in AU (or light-years for galaxies)
  magnitude?: number; // Apparent magnitude
  description: string;
}

/**
 * 'Oumuamua (1I/2017 U1) - First interstellar visitor
 */
export const OUMUAMUA: InterstellarObject = {
  name: "'Oumuamua",
  designation: '1I/2017 U1',
  type: 'asteroid',
  discoveryDate: new Date('2017-10-19'),
  currentPosition: new THREE.Vector3(-50, 10, 5), // Approximate current position (AU)
  velocity: new THREE.Vector3(-26, 0, 0), // km/s (approximate)
  distance: 50, // AU (approximate, increasing)
  magnitude: 23.4,
  description: "First known interstellar object to pass through the Solar System. Discovered October 2017. Elongated shape (10:1 aspect ratio)."
};

/**
 * 2I/Borisov - Interstellar comet
 */
export const BORISOV: InterstellarObject = {
  name: '2I/Borisov',
  designation: '2I/2019 Q4',
  type: 'comet',
  discoveryDate: new Date('2019-08-30'),
  currentPosition: new THREE.Vector3(-30, -8, 3), // Approximate (AU)
  velocity: new THREE.Vector3(-32, 0, 0), // km/s
  distance: 30, // AU (approximate)
  magnitude: 15.0,
  description: "Second known interstellar object. Comet-like with visible coma and tail. Discovered August 2019."
};

/**
 * Andromeda Galaxy (M31)
 */
export const ANDROMEDA_GALAXY: InterstellarObject = {
  name: 'Andromeda Galaxy',
  designation: 'M31',
  type: 'galaxy',
  discoveryDate: new Date('964-01-01'), // Historical - first recorded observation
  currentPosition: new THREE.Vector3(0, 0, 0), // Will be calculated from RA/Dec
  velocity: new THREE.Vector3(0, 0, 0), // Relative motion negligible
  distance: 2.537e6, // 2.537 million light-years
  magnitude: 3.4,
  description: "Nearest major galaxy to Milky Way. 2.537 million light-years away. Visible to naked eye under dark skies."
};

/**
 * Proxima Centauri - Closest star
 */
export const PROXIMA_CENTAURI: InterstellarObject = {
  name: 'Proxima Centauri',
  designation: 'α Cen C',
  type: 'star',
  discoveryDate: new Date('1915-01-01'),
  currentPosition: new THREE.Vector3(0, 0, 0), // Will calculate from coordinates
  velocity: new THREE.Vector3(0, 0, 0),
  distance: 4.246, // light-years = 268,770 AU
  magnitude: 11.13,
  description: "Closest star to Sun. 4.246 light-years away. Red dwarf star. Part of Alpha Centauri system."
};

/**
 * Calculate Andromeda position from RA/Dec
 */
export function getAndromedaPosition(): THREE.Vector3 {
  // Andromeda: RA 00h 42m 44s, Dec +41° 16' 9"
  const ra = (0 + 42/60 + 44/3600) * 15; // Convert to degrees
  const dec = 41 + 16/60 + 9/3600;
  
  // Convert to ecliptic coordinates
  const ecliptic = CoordinateTransforms.icrsToEcliptic(ra, dec, 1);
  
  // Scale to visible distance (very far, but show at reasonable scale)
  // For visualization, we'll show it at a large but visible distance
  return ecliptic.multiplyScalar(1000); // Scaled for visibility
}

/**
 * Calculate Proxima Centauri position
 */
export function getProximaCentauriPosition(): THREE.Vector3 {
  // Proxima Centauri: RA 14h 29m 42.9s, Dec -62° 40' 46"
  const ra = (14 + 29/60 + 42.9/3600) * 15;
  const dec = -(62 + 40/60 + 46/3600);
  
  const ecliptic = CoordinateTransforms.icrsToEcliptic(ra, dec, 1);
  
  // Distance: 4.246 light-years = 268,770 AU
  // Scale for visibility
  return ecliptic.multiplyScalar(500); // Scaled for visualization
}

/**
 * Generate 'Oumuamua trajectory through solar system
 */
export function generateOumuamuaTrajectory(): Vector3TimeSeries {
  const epochs: number[] = [];
  const positions: THREE.Vector3[] = [];
  
  // Hyperbolic orbit through solar system
  // Perihelion: ~0.255 AU on Sept 9, 2017
  const perihelionDate = new Date('2017-09-09');
  const perihelionJD = JulianDate.fromDate(perihelionDate);
  
  // Trajectory points (simplified hyperbolic orbit)
  const startJD = perihelionJD - 30; // 30 days before perihelion
  const endJD = perihelionJD + 60; // 60 days after perihelion
  
  for (let jd = startJD; jd <= endJD; jd += 1) {
    const daysFromPerihelion = jd - perihelionJD;
    
    // Simplified hyperbolic trajectory
    const r = 0.255 + Math.abs(daysFromPerihelion) * 0.1; // AU
    const theta = Math.atan2(daysFromPerihelion, 10) + Math.PI / 2;
    
    const x = r * Math.cos(theta);
    const y = r * Math.sin(theta) * 0.3; // Inclined orbit
    const z = r * Math.sin(theta) * 0.1;
    
    epochs.push(jd);
    positions.push(new THREE.Vector3(x, y, z));
  }
  
  return new Vector3TimeSeries(epochs, positions);
}

/**
 * Generate 2I/Borisov trajectory
 */
export function generateBorisovTrajectory(): Vector3TimeSeries {
  const epochs: number[] = [];
  const positions: THREE.Vector3[] = [];
  
  // Perihelion: ~2.006 AU on Dec 8, 2019
  const perihelionDate = new Date('2019-12-08');
  const perihelionJD = JulianDate.fromDate(perihelionDate);
  
  const startJD = perihelionJD - 60;
  const endJD = perihelionJD + 90;
  
  for (let jd = startJD; jd <= endJD; jd += 1) {
    const daysFromPerihelion = jd - perihelionJD;
    const r = 2.006 + Math.abs(daysFromPerihelion) * 0.08;
    const theta = Math.atan2(daysFromPerihelion, 15) + Math.PI / 2;
    
    const x = r * Math.cos(theta);
    const y = r * Math.sin(theta) * 0.4;
    const z = r * Math.sin(theta) * 0.2;
    
    epochs.push(jd);
    positions.push(new THREE.Vector3(x, y, z));
  }
  
  return new Vector3TimeSeries(epochs, positions);
}

/**
 * Get all interstellar objects
 */
export function getAllInterstellarObjects(): InterstellarObject[] {
  const andromeda = {
    ...ANDROMEDA_GALAXY,
    currentPosition: getAndromedaPosition()
  };
  
  const proxima = {
    ...PROXIMA_CENTAURI,
    currentPosition: getProximaCentauriPosition()
  };
  
  return [
    OUMUAMUA,
    BORISOV,
    andromeda,
    proxima
  ];
}
