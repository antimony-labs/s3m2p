/**
 * Planetary ephemeris data and calculations
 * Based on JPL DE440/441 ephemerides and VSOP87 theory
 * Includes all planets plus Pluto
 */

import * as THREE from 'three';
import { 
  PlanetaryEphemeris as IPlanetaryEphemeris, 
  OrbitalElements,
  JulianDate
} from './AstronomicalDataStore';

/**
 * Mean orbital elements at J2000.0 epoch
 * Source: JPL planetary ephemerides
 */
const J2000_ELEMENTS: Record<string, OrbitalElements> = {
  Mercury: {
    a: 0.38709927,      // AU
    e: 0.20563593,
    i: 7.00497902,      // degrees (will convert to radians)
    omega: 29.12703035, // degrees
    Omega: 48.33076593, // degrees
    M: 174.79252722,    // degrees
    period: 0.2408467   // years
  },
  Venus: {
    a: 0.72333566,
    e: 0.00677672,
    i: 3.39467605,
    omega: 54.89101678,
    Omega: 76.67984255,
    M: 50.11532082,
    period: 0.61519726
  },
  Earth: {
    a: 1.00000261,
    e: 0.01671123,
    i: 0.00001531,
    omega: 102.93768193,
    Omega: 0.0,  // Reference plane
    M: 357.52911632,
    period: 1.0000174
  },
  Mars: {
    a: 1.52371034,
    e: 0.09339410,
    i: 1.84969142,
    omega: 286.53736850,
    Omega: 49.55953891,
    M: 19.37331303,
    period: 1.8808476
  },
  Jupiter: {
    a: 5.20288700,
    e: 0.04838624,
    i: 1.30439695,
    omega: 273.86734911,
    Omega: 100.47390909,
    M: 20.02080816,
    period: 11.862615
  },
  Saturn: {
    a: 9.53667594,
    e: 0.05386179,
    i: 2.48599187,
    omega: 339.39189718,
    Omega: 113.66242448,
    M: 316.96698813,
    period: 29.447498
  },
  Uranus: {
    a: 19.18916464,
    e: 0.04725744,
    i: 0.77263783,
    omega: 96.99892789,
    Omega: 74.01692503,
    M: 142.59864774,
    period: 84.016846
  },
  Neptune: {
    a: 30.06992276,
    e: 0.00859048,
    i: 1.77004347,
    omega: 276.33671733,
    Omega: 131.78422574,
    M: 259.91590562,
    period: 164.79132
  },
  Pluto: {
    a: 39.48211675,
    e: 0.24882730,
    i: 17.14001206,
    omega: 113.76242045,
    Omega: 110.30393684,
    M: 15.11038925,
    period: 247.92065
  }
};

/**
 * Secular rates of change per century
 * For high precision ephemerides
 */
const SECULAR_RATES: Record<string, Partial<OrbitalElements>> = {
  Mercury: {
    a: 0.00000037,
    e: 0.00001906,
    i: -0.00594749,
    omega: 0.16047689,
    Omega: -0.12534081,
    M: 149472.67411175
  },
  Venus: {
    a: 0.00000390,
    e: -0.00004107,
    i: -0.00078890,
    omega: 0.00268329,
    Omega: -0.27769418,
    M: 58517.81538729
  },
  Earth: {
    a: 0.00000562,
    e: -0.00004392,
    i: -0.01294668,
    omega: 0.32327364,
    Omega: 0.0,
    M: 35999.37244981
  },
  Mars: {
    a: 0.00001847,
    e: 0.00007882,
    i: -0.00813131,
    omega: 0.44441088,
    Omega: -0.29257343,
    M: 19140.30268499
  },
  Jupiter: {
    a: -0.00011607,
    e: -0.00013253,
    i: -0.00183714,
    omega: 0.20469106,
    Omega: 0.21252668,
    M: 3034.74612775
  },
  Saturn: {
    a: -0.00125060,
    e: -0.00050991,
    i: 0.00193609,
    omega: -0.28867794,
    Omega: -0.41897216,
    M: 1222.49362201
  },
  Uranus: {
    a: -0.00196176,
    e: -0.00004397,
    i: -0.00242939,
    omega: 0.04240589,
    Omega: 0.40805281,
    M: 428.48202785
  },
  Neptune: {
    a: 0.00026291,
    e: 0.00005105,
    i: 0.00035372,
    omega: -0.32241464,
    Omega: -0.00508664,
    M: 218.45945325
  },
  Pluto: {
    a: -0.00031596,
    e: 0.00005170,
    i: 0.00004818,
    omega: -0.04062942,
    Omega: -0.01183482,
    M: 145.20780515
  }
};

/**
 * Physical properties of planets
 */
export const PLANET_PROPERTIES = {
  Mercury: { radius: 2439.7, mass: 3.3011e23, color: 0x8c8c8c },
  Venus: { radius: 6051.8, mass: 4.8675e24, color: 0xffd700 },
  Earth: { radius: 6371.0, mass: 5.9724e24, color: 0x4169e1 },
  Mars: { radius: 3389.5, mass: 6.4171e23, color: 0xff4500 },
  Jupiter: { radius: 69911, mass: 1.8982e27, color: 0xffa500 },
  Saturn: { radius: 58232, mass: 5.6834e26, color: 0xf4a460 },
  Uranus: { radius: 25362, mass: 8.6810e25, color: 0x40e0d0 },
  Neptune: { radius: 24622, mass: 1.0241e26, color: 0x4169e1 },
  Pluto: { radius: 1188.3, mass: 1.3090e22, color: 0xdaa520 }
};

/**
 * Calculate planetary positions using Keplerian elements
 */
export class PlanetaryEphemeris {
  /**
   * Get orbital elements at a specific epoch
   */
  static getElements(planet: string, julianDate: number): OrbitalElements {
    const baseElements = J2000_ELEMENTS[planet];
    const rates = SECULAR_RATES[planet];
    
    if (!baseElements) {
      throw new Error(`Unknown planet: ${planet}`);
    }
    
    // Centuries since J2000.0
    const T = (julianDate - 2451545.0) / 36525.0;
    
    // Apply secular variations
    const elements: OrbitalElements = {
      a: baseElements.a + (rates.a || 0) * T,
      e: baseElements.e + (rates.e || 0) * T,
      i: (baseElements.i + (rates.i || 0) * T) * Math.PI / 180,  // Convert to radians
      omega: (baseElements.omega + (rates.omega || 0) * T) * Math.PI / 180,
      Omega: (baseElements.Omega + (rates.Omega || 0) * T) * Math.PI / 180,
      M: (baseElements.M + (rates.M || 0) * T) * Math.PI / 180,
      period: baseElements.period
    };
    
    // Normalize angles
    elements.M = this.normalizeAngle(elements.M);
    elements.omega = this.normalizeAngle(elements.omega);
    elements.Omega = this.normalizeAngle(elements.Omega);
    
    return elements;
  }
  
  /**
   * Calculate planetary position from orbital elements
   */
  static calculatePosition(planet: string, julianDate: number): IPlanetaryEphemeris {
    const elements = this.getElements(planet, julianDate);
    
    // Solve Kepler's equation for eccentric anomaly
    const E = this.solveKeplerEquation(elements.M, elements.e);
    
    // Calculate true anomaly
    const cosE = Math.cos(E);
    const sinE = Math.sin(E);
    const sqrtOneMinusE2 = Math.sqrt(1 - elements.e * elements.e);
    
    const cosNu = (cosE - elements.e) / (1 - elements.e * cosE);
    const sinNu = (sqrtOneMinusE2 * sinE) / (1 - elements.e * cosE);
    const nu = Math.atan2(sinNu, cosNu);
    
    // Calculate heliocentric distance
    const r = elements.a * (1 - elements.e * cosE);
    
    // Calculate position in orbital plane
    const x_orbital = r * Math.cos(nu);
    const y_orbital = r * Math.sin(nu);
    const z_orbital = 0;
    
    // Transform to ecliptic coordinates
    const cosOmega = Math.cos(elements.Omega);
    const sinOmega = Math.sin(elements.Omega);
    const cosI = Math.cos(elements.i);
    const sinI = Math.sin(elements.i);
    const cosOmegaPlusW = Math.cos(elements.Omega + elements.omega);
    const sinOmegaPlusW = Math.sin(elements.Omega + elements.omega);
    
    // Rotation matrix elements
    const P11 = cosOmegaPlusW * cosOmega - sinOmegaPlusW * cosI * sinOmega;
    const P12 = -sinOmegaPlusW * cosOmega - cosOmegaPlusW * cosI * sinOmega;
    const P21 = cosOmegaPlusW * sinOmega + sinOmegaPlusW * cosI * cosOmega;
    const P22 = -sinOmegaPlusW * sinOmega + cosOmegaPlusW * cosI * cosOmega;
    const P31 = sinOmegaPlusW * sinI;
    const P32 = cosOmegaPlusW * sinI;
    
    // Ecliptic coordinates
    const x = P11 * x_orbital + P12 * y_orbital;
    const y = P21 * x_orbital + P22 * y_orbital;
    const z = P31 * x_orbital + P32 * y_orbital;
    
    const position = new THREE.Vector3(x, y, z);
    
    // Calculate velocity (simplified - for accurate velocity, use numerical differentiation)
    const n = 2 * Math.PI / (elements.period * 365.25);  // Mean motion (rad/day)
    const v_orbital = n * elements.a / (1 - elements.e * cosE);  // AU/day
    const v_km_s = v_orbital * 1.731e3;  // Convert to km/s
    
    // Velocity direction (tangent to orbit)
    const velocity = new THREE.Vector3(
      -v_km_s * sinNu,
      v_km_s * (elements.e + cosNu),
      0
    );
    
    // Transform velocity to ecliptic frame
    const vx = P11 * velocity.x + P12 * velocity.y;
    const vy = P21 * velocity.x + P22 * velocity.y;
    const vz = P31 * velocity.x + P32 * velocity.y;
    
    velocity.set(vx, vy, vz);
    
    return {
      name: planet,
      epoch: julianDate,
      position,
      velocity,
      elements
    };
  }
  
  /**
   * Solve Kepler's equation using Newton-Raphson method
   */
  private static solveKeplerEquation(M: number, e: number, tolerance: number = 1e-8): number {
    let E = M;  // Initial guess
    
    for (let i = 0; i < 50; i++) {
      const delta = E - e * Math.sin(E) - M;
      
      if (Math.abs(delta) < tolerance) {
        break;
      }
      
      E = E - delta / (1 - e * Math.cos(E));
    }
    
    return E;
  }
  
  /**
   * Normalize angle to [0, 2π]
   */
  private static normalizeAngle(angle: number): number {
    while (angle < 0) angle += 2 * Math.PI;
    while (angle > 2 * Math.PI) angle -= 2 * Math.PI;
    return angle;
  }
  
  /**
   * Get all planet positions at a given time
   */
  static getAllPlanets(julianDate: number): Map<string, IPlanetaryEphemeris> {
    const planets = new Map<string, IPlanetaryEphemeris>();
    
    for (const planet of Object.keys(J2000_ELEMENTS)) {
      planets.set(planet, this.calculatePosition(planet, julianDate));
    }
    
    return planets;
  }
  
  /**
   * Calculate Earth-Moon barycenter position
   * (For more accuracy, the ephemerides should use EMB rather than Earth alone)
   */
  static getEarthMoonBarycenter(julianDate: number): IPlanetaryEphemeris {
    const earth = this.calculatePosition('Earth', julianDate);
    
    // Moon's position relative to Earth (simplified circular orbit)
    const moonPeriod = 27.321661;  // days
    const moonDistance = 0.00257;   // AU (average)
    const moonPhase = ((julianDate - 2451545.0) / moonPeriod) * 2 * Math.PI;
    
    const moonRelative = new THREE.Vector3(
      moonDistance * Math.cos(moonPhase),
      moonDistance * Math.sin(moonPhase) * Math.cos(5.145 * Math.PI / 180),  // Inclination
      moonDistance * Math.sin(moonPhase) * Math.sin(5.145 * Math.PI / 180)
    );
    
    // Barycenter is offset from Earth center
    const earthMoonMassRatio = 81.30059;
    const barycenterOffset = moonRelative.multiplyScalar(1 / (1 + earthMoonMassRatio));
    
    earth.position.sub(barycenterOffset);
    
    return earth;
  }
  
  /**
   * Special handling for Pluto's complex orbit
   * (Includes perturbations from Neptune)
   */
  static getPlutoPosition(julianDate: number): IPlanetaryEphemeris {
    const basePosition = this.calculatePosition('Pluto', julianDate);
    
    // Apply simplified perturbations from Neptune
    const neptune = this.calculatePosition('Neptune', julianDate);
    const plutoNeptune = basePosition.position.clone().sub(neptune.position);
    const distance = plutoNeptune.length();
    
    // Gravitational perturbation (very simplified)
    if (distance < 50) {  // AU
      const perturbation = plutoNeptune.normalize().multiplyScalar(0.01 / (distance * distance));
      basePosition.position.add(perturbation);
    }
    
    return basePosition;
  }
  
  /**
   * Get orbital period at current distance (Kepler's third law)
   */
  static getOrbitalPeriod(distanceAU: number): number {
    // P² = a³ (in years and AU)
    return Math.sqrt(distanceAU * distanceAU * distanceAU);
  }
}

/**
 * Moon ephemeris calculations
 */
export class MoonEphemeris {
  /**
   * Calculate Moon position relative to Earth
   * Using simplified Brown's lunar theory
   */
  static getMoonPosition(julianDate: number): THREE.Vector3 {
    const T = (julianDate - 2451545.0) / 36525.0;
    
    // Mean elements
    const L = 218.316 + 13.176396 * (julianDate - 2451545.0);  // Mean longitude
    const M = 134.963 + 13.064993 * (julianDate - 2451545.0);  // Mean anomaly
    const F = 93.272 + 13.229350 * (julianDate - 2451545.0);   // Mean distance
    
    const Lrad = L * Math.PI / 180;
    const Mrad = M * Math.PI / 180;
    const Frad = F * Math.PI / 180;
    
    // Perturbations
    const dL = 6.289 * Math.sin(Mrad) + 1.274 * Math.sin(2 * Frad - Mrad);
    const dB = 5.128 * Math.sin(Frad);
    const dR = -20.905 * Math.cos(Mrad);
    
    // Ecliptic coordinates
    const longitude = (L + dL) * Math.PI / 180;
    const latitude = dB * Math.PI / 180;
    const distance = (385001 + dR) / 149597870.7;  // Convert km to AU
    
    // Convert to Cartesian
    const x = distance * Math.cos(latitude) * Math.cos(longitude);
    const y = distance * Math.cos(latitude) * Math.sin(longitude);
    const z = distance * Math.sin(latitude);
    
    return new THREE.Vector3(x, y, z);
  }
}
