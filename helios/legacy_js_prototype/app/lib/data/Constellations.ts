/**
 * Constellation data and visualization
 * Based on IAU 88 official constellations
 */

import * as THREE from 'three';
import { FAMOUS_STARS, StarData, raDecToCartesian } from '../starCatalog';

/**
 * Star connection for constellation lines
 */
export interface StarConnection {
  from: string; // Star name
  to: string;   // Star name
}

/**
 * Constellation definition
 */
export interface Constellation {
  name: string;
  abbreviation: string;
  stars: string[]; // Star names in this constellation
  connections: StarConnection[]; // Lines connecting stars
  center: { ra: number; dec: number }; // Center position
}

/**
 * Key constellations with their star patterns
 */
export const CONSTELLATIONS: Constellation[] = [
  {
    name: 'Orion',
    abbreviation: 'Ori',
    stars: ['Betelgeuse', 'Rigel'],
    connections: [
      { from: 'Betelgeuse', to: 'Rigel' }
    ],
    center: { ra: 5.5, dec: 0 }
  },
  {
    name: 'Ursa Major',
    abbreviation: 'UMa',
    stars: [],
    connections: [],
    center: { ra: 10.7, dec: 55 }
  },
  {
    name: 'Cassiopeia',
    abbreviation: 'Cas',
    stars: [],
    connections: [],
    center: { ra: 1.0, dec: 60 }
  },
  {
    name: 'Cygnus',
    abbreviation: 'Cyg',
    stars: [],
    connections: [],
    center: { ra: 20.5, dec: 45 }
  },
  {
    name: 'Scorpius',
    abbreviation: 'Sco',
    stars: ['Antares'],
    connections: [],
    center: { ra: 16.5, dec: -26 }
  }
];

/**
 * Extended star catalog for constellations
 * Adds more stars needed for complete constellation patterns
 */
export const CONSTELLATION_STARS: StarData[] = [
  // Orion stars
  {
    name: 'Bellatrix',
    ra: 5.4183,
    dec: 6.3497,
    distance: 77.0,
    magnitude: 1.64,
    color: 0xaaaaff,
    type: 'B2III'
  },
  {
    name: 'Mintaka',
    ra: 5.5334,
    dec: -0.2991,
    distance: 380.0,
    magnitude: 2.25,
    color: 0xaaaaff,
    type: 'O9.5II'
  },
  {
    name: 'Alnilam',
    ra: 5.6033,
    dec: -1.2019,
    distance: 600.0,
    magnitude: 1.70,
    color: 0xaaaaff,
    type: 'B0Ia'
  },
  {
    name: 'Alnitak',
    ra: 5.6794,
    dec: -1.9426,
    distance: 250.0,
    magnitude: 1.77,
    color: 0xaaaaff,
    type: 'O9.5Ib'
  },
  {
    name: 'Saiph',
    ra: 5.7958,
    dec: -9.6696,
    distance: 220.0,
    magnitude: 2.07,
    color: 0xaaaaff,
    type: 'B0.5Ia'
  },
  
  // Ursa Major (Big Dipper) stars
  {
    name: 'Dubhe',
    ra: 11.0621,
    dec: 61.7510,
    distance: 38.0,
    magnitude: 1.81,
    color: 0xffffaa,
    type: 'K1III'
  },
  {
    name: 'Merak',
    ra: 11.0307,
    dec: 56.3824,
    distance: 25.0,
    magnitude: 2.37,
    color: 0xffffaa,
    type: 'A1V'
  },
  {
    name: 'Phecda',
    ra: 11.8972,
    dec: 53.6948,
    distance: 25.0,
    magnitude: 2.44,
    color: 0xffffaa,
    type: 'A0V'
  },
  {
    name: 'Megrez',
    ra: 12.2571,
    dec: 57.0326,
    distance: 25.0,
    magnitude: 3.32,
    color: 0xffffaa,
    type: 'A3V'
  },
  {
    name: 'Alioth',
    ra: 12.9004,
    dec: 55.9598,
    distance: 25.0,
    magnitude: 1.76,
    color: 0xffffaa,
    type: 'A0V'
  },
  {
    name: 'Mizar',
    ra: 13.3988,
    dec: 54.9254,
    distance: 25.0,
    magnitude: 2.23,
    color: 0xffffaa,
    type: 'A2V'
  },
  {
    name: 'Alkaid',
    ra: 13.7923,
    dec: 49.3133,
    distance: 32.0,
    magnitude: 1.85,
    color: 0xffffaa,
    type: 'B3V'
  },
  
  // Cassiopeia stars
  {
    name: 'Schedar',
    ra: 0.6754,
    dec: 56.5373,
    distance: 70.0,
    magnitude: 2.24,
    color: 0xffffaa,
    type: 'K0III'
  },
  {
    name: 'Caph',
    ra: 0.1528,
    dec: 59.1498,
    distance: 19.0,
    magnitude: 2.28,
    color: 0xffffaa,
    type: 'F2III'
  },
  
  // Cygnus stars
  {
    name: 'Deneb',
    ra: 20.6905,
    dec: 45.2803,
    distance: 800.0,
    magnitude: 1.25,
    color: 0xaaaaff,
    type: 'A2Ia'
  },
  {
    name: 'Albireo',
    ra: 19.5121,
    dec: 27.9597,
    distance: 130.0,
    magnitude: 3.05,
    color: 0xffffaa,
    type: 'K3II'
  }
];

/**
 * Complete constellation definitions with connections
 */
export const COMPLETE_CONSTELLATIONS: Constellation[] = [
  // Orion - The Hunter
  {
    name: 'Orion',
    abbreviation: 'Ori',
    stars: ['Betelgeuse', 'Bellatrix', 'Mintaka', 'Alnilam', 'Alnitak', 'Rigel', 'Saiph'],
    connections: [
      { from: 'Betelgeuse', to: 'Bellatrix' },
      { from: 'Bellatrix', to: 'Mintaka' },
      { from: 'Mintaka', to: 'Alnilam' },
      { from: 'Alnilam', to: 'Alnitak' },
      { from: 'Alnitak', to: 'Rigel' },
      { from: 'Rigel', to: 'Saiph' },
      { from: 'Saiph', to: 'Alnitak' },
      { from: 'Alnitak', to: 'Mintaka' },
      { from: 'Mintaka', to: 'Bellatrix' },
      { from: 'Bellatrix', to: 'Betelgeuse' },
      { from: 'Betelgeuse', to: 'Mintaka' }, // Belt
      { from: 'Alnilam', to: 'Rigel' } // Center to Rigel
    ],
    center: { ra: 5.5, dec: 0 }
  },
  
  // Ursa Major - Big Dipper
  {
    name: 'Ursa Major',
    abbreviation: 'UMa',
    stars: ['Dubhe', 'Merak', 'Phecda', 'Megrez', 'Alioth', 'Mizar', 'Alkaid'],
    connections: [
      { from: 'Dubhe', to: 'Merak' },
      { from: 'Merak', to: 'Phecda' },
      { from: 'Phecda', to: 'Megrez' },
      { from: 'Megrez', to: 'Alioth' },
      { from: 'Alioth', to: 'Mizar' },
      { from: 'Mizar', to: 'Alkaid' },
      { from: 'Phecda', to: 'Megrez' },
      { from: 'Megrez', to: 'Dubhe' } // Complete the bowl
    ],
    center: { ra: 11.5, dec: 55 }
  },
  
  // Cassiopeia - The Queen
  {
    name: 'Cassiopeia',
    abbreviation: 'Cas',
    stars: ['Schedar', 'Caph'],
    connections: [
      { from: 'Schedar', to: 'Caph' }
    ],
    center: { ra: 1.0, dec: 60 }
  },
  
  // Cygnus - The Swan / Northern Cross
  {
    name: 'Cygnus',
    abbreviation: 'Cyg',
    stars: ['Deneb', 'Albireo'],
    connections: [
      { from: 'Deneb', to: 'Albireo' }
    ],
    center: { ra: 20.5, dec: 45 }
  },
  
  // Scorpius - The Scorpion
  {
    name: 'Scorpius',
    abbreviation: 'Sco',
    stars: ['Antares'],
    connections: [],
    center: { ra: 16.5, dec: -26 }
  }
];

/**
 * Get star position by name
 */
export function getStarPosition(starName: string, starCatalog: StarData[]): THREE.Vector3 | null {
  const star = starCatalog.find(s => s.name === starName);
  if (!star) return null;
  
  const [x, y, z] = raDecToCartesian(star.ra, star.dec, star.distance * 0.1);
  return new THREE.Vector3(x, y, z);
}

/**
 * Generate constellation line geometry
 */
export function generateConstellationLines(
  constellation: Constellation,
  starCatalog: StarData[]
): THREE.BufferGeometry | null {
  const points: THREE.Vector3[] = [];
  
  constellation.connections.forEach(conn => {
    const fromPos = getStarPosition(conn.from, starCatalog);
    const toPos = getStarPosition(conn.to, starCatalog);
    
    if (fromPos && toPos) {
      points.push(fromPos);
      points.push(toPos);
    }
  });

  if (points.length === 0) {
    return null;
  }

  return new THREE.BufferGeometry().setFromPoints(points);
}
