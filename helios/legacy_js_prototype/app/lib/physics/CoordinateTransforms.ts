/**
 * Coordinate transformation systems for heliospheric physics
 * Implements transformations between various astronomical reference frames
 * 
 * Reference frames:
 * - HEE (Heliocentric Earth Ecliptic)
 * - HGI (Heliographic Inertial) 
 * - GAL (Galactic)
 * - ICRS (International Celestial Reference System)
 * - RTN (Radial-Tangential-Normal) for spacecraft
 */

import * as THREE from 'three';

/**
 * Astronomical constants
 */
export const ASTRONOMICAL_CONSTANTS = {
  AU_TO_KM: 149597870.7,          // Astronomical Unit in km
  ECLIPTIC_OBLIQUITY: 23.43928,   // Earth's axial tilt (degrees)
  J2000_EPOCH: 2451545.0,         // J2000.0 epoch (Julian Date)
  GALACTIC_NORTH_RA: 192.85948,   // Right ascension of galactic north pole (degrees)
  GALACTIC_NORTH_DEC: 27.12825,   // Declination of galactic north pole (degrees)
  GALACTIC_CENTER_L: 0.0,         // Galactic longitude of galactic center
  GALACTIC_CENTER_B: 0.0,         // Galactic latitude of galactic center
  SOLAR_APEX_RA: 277.0,           // Solar apex right ascension (degrees)
  SOLAR_APEX_DEC: 30.0,           // Solar apex declination (degrees)
};

/**
 * Convert degrees to radians
 */
function degToRad(degrees: number): number {
  return degrees * Math.PI / 180;
}

/**
 * Convert radians to degrees
 */
function radToDeg(radians: number): number {
  return radians * 180 / Math.PI;
}

/**
 * Base class for coordinate transformations
 */
export class CoordinateTransforms {
  private static eclipticToEquatorial: THREE.Matrix4;
  private static equatorialToEcliptic: THREE.Matrix4;
  private static equatorialToGalactic: THREE.Matrix4;
  private static galacticToEquatorial: THREE.Matrix4;
  
  static {
    // Initialize transformation matrices
    this.initializeMatrices();
  }
  
  /**
   * Initialize transformation matrices
   */
  private static initializeMatrices(): void {
    // Ecliptic to Equatorial transformation
    const obliquity = degToRad(ASTRONOMICAL_CONSTANTS.ECLIPTIC_OBLIQUITY);
    
    this.eclipticToEquatorial = new THREE.Matrix4().set(
      1, 0, 0, 0,
      0, Math.cos(obliquity), -Math.sin(obliquity), 0,
      0, Math.sin(obliquity), Math.cos(obliquity), 0,
      0, 0, 0, 1
    );
    
    this.equatorialToEcliptic = this.eclipticToEquatorial.clone().invert();
    
    // Galactic to Equatorial transformation (J2000.0)
    // Based on IAU galactic coordinate definition
    const l_cp = degToRad(122.93192);  // Galactic longitude of celestial pole
    const ra_gp = degToRad(ASTRONOMICAL_CONSTANTS.GALACTIC_NORTH_RA);
    const dec_gp = degToRad(ASTRONOMICAL_CONSTANTS.GALACTIC_NORTH_DEC);
    
    // Rotation matrices for galactic transformation
    const cosDecGP = Math.cos(dec_gp);
    const sinDecGP = Math.sin(dec_gp);
    const cosRAGP = Math.cos(ra_gp);
    const sinRAGP = Math.sin(ra_gp);
    const cosLCP = Math.cos(l_cp);
    const sinLCP = Math.sin(l_cp);
    
    this.galacticToEquatorial = new THREE.Matrix4().set(
      -sinLCP * cosDecGP * cosRAGP - cosLCP * sinRAGP,
      -sinLCP * cosDecGP * sinRAGP + cosLCP * cosRAGP,
      sinLCP * sinDecGP,
      0,
      
      cosLCP * cosDecGP * cosRAGP - sinLCP * sinRAGP,
      cosLCP * cosDecGP * sinRAGP + sinLCP * cosRAGP,
      -cosLCP * sinDecGP,
      0,
      
      sinDecGP * cosRAGP,
      sinDecGP * sinRAGP,
      cosDecGP,
      0,
      
      0, 0, 0, 1
    );
    
    this.equatorialToGalactic = this.galacticToEquatorial.clone().invert();
  }
  
  /**
   * Convert HEE (Heliocentric Earth Ecliptic) to HGI (Heliographic Inertial)
   */
  static heeToHgi(position: THREE.Vector3, julianDate: number): THREE.Vector3 {
    // Calculate Carrington rotation elements
    const t = (julianDate - ASTRONOMICAL_CONSTANTS.J2000_EPOCH) / 36525.0;
    
    // Solar rotation parameters
    const theta0 = 100.46 + 36000.77 * t + 0.04107 * t * t;  // degrees
    const i = 7.25;  // Solar inclination
    const omega = 74.37 + 0.0527 * t;  // Longitude of ascending node
    
    const theta0Rad = degToRad(theta0);
    const iRad = degToRad(i);
    const omegaRad = degToRad(omega);
    
    // Construct transformation matrix
    const cosI = Math.cos(iRad);
    const sinI = Math.sin(iRad);
    const cosOmega = Math.cos(omegaRad);
    const sinOmega = Math.sin(omegaRad);
    const cosTheta0 = Math.cos(theta0Rad);
    const sinTheta0 = Math.sin(theta0Rad);
    
    const transform = new THREE.Matrix4().set(
      cosOmega * cosTheta0 - sinOmega * sinTheta0 * cosI,
      -cosOmega * sinTheta0 - sinOmega * cosTheta0 * cosI,
      sinOmega * sinI,
      0,
      
      sinOmega * cosTheta0 + cosOmega * sinTheta0 * cosI,
      -sinOmega * sinTheta0 + cosOmega * cosTheta0 * cosI,
      -cosOmega * sinI,
      0,
      
      sinTheta0 * sinI,
      cosTheta0 * sinI,
      cosI,
      0,
      
      0, 0, 0, 1
    );
    
    return position.clone().applyMatrix4(transform);
  }
  
  /**
   * Convert HGI to HEE
   */
  static hgiToHee(position: THREE.Vector3, julianDate: number): THREE.Vector3 {
    // Inverse transformation of heeToHgi
    const t = (julianDate - ASTRONOMICAL_CONSTANTS.J2000_EPOCH) / 36525.0;
    
    const theta0 = 100.46 + 36000.77 * t + 0.04107 * t * t;
    const i = 7.25;
    const omega = 74.37 + 0.0527 * t;
    
    const theta0Rad = degToRad(theta0);
    const iRad = degToRad(i);
    const omegaRad = degToRad(omega);
    
    const cosI = Math.cos(iRad);
    const sinI = Math.sin(iRad);
    const cosOmega = Math.cos(omegaRad);
    const sinOmega = Math.sin(omegaRad);
    const cosTheta0 = Math.cos(theta0Rad);
    const sinTheta0 = Math.sin(theta0Rad);
    
    // Transpose of the HEE to HGI matrix
    const transform = new THREE.Matrix4().set(
      cosOmega * cosTheta0 - sinOmega * sinTheta0 * cosI,
      sinOmega * cosTheta0 + cosOmega * sinTheta0 * cosI,
      sinTheta0 * sinI,
      0,
      
      -cosOmega * sinTheta0 - sinOmega * cosTheta0 * cosI,
      -sinOmega * sinTheta0 + cosOmega * cosTheta0 * cosI,
      cosTheta0 * sinI,
      0,
      
      sinOmega * sinI,
      -cosOmega * sinI,
      cosI,
      0,
      
      0, 0, 0, 1
    );
    
    return position.clone().applyMatrix4(transform);
  }
  
  /**
   * Convert Ecliptic to Galactic coordinates
   */
  static eclipticToGalactic(position: THREE.Vector3): THREE.Vector3 {
    // First convert to equatorial
    const equatorial = position.clone().applyMatrix4(this.eclipticToEquatorial);
    // Then to galactic
    return equatorial.applyMatrix4(this.equatorialToGalactic);
  }
  
  /**
   * Convert Galactic to Ecliptic coordinates
   */
  static galacticToEcliptic(position: THREE.Vector3): THREE.Vector3 {
    // First convert to equatorial
    const equatorial = position.clone().applyMatrix4(this.galacticToEquatorial);
    // Then to ecliptic
    return equatorial.applyMatrix4(this.equatorialToEcliptic);
  }
  
  /**
   * Convert ICRS (RA/Dec) to Ecliptic coordinates
   */
  static icrsToEcliptic(ra: number, dec: number, distance: number = 1): THREE.Vector3 {
    const raRad = degToRad(ra);
    const decRad = degToRad(dec);
    
    // Convert to Cartesian
    const x = distance * Math.cos(decRad) * Math.cos(raRad);
    const y = distance * Math.cos(decRad) * Math.sin(raRad);
    const z = distance * Math.sin(decRad);
    
    const equatorial = new THREE.Vector3(x, y, z);
    return equatorial.applyMatrix4(this.equatorialToEcliptic);
  }
  
  /**
   * Convert Ecliptic to ICRS (RA/Dec)
   */
  static eclipticToIcrs(position: THREE.Vector3): { ra: number; dec: number; distance: number } {
    const equatorial = position.clone().applyMatrix4(this.eclipticToEquatorial);
    
    const distance = equatorial.length();
    const x = equatorial.x;
    const y = equatorial.y;
    const z = equatorial.z;
    
    const ra = radToDeg(Math.atan2(y, x));
    const dec = radToDeg(Math.asin(z / distance));
    
    return {
      ra: ra < 0 ? ra + 360 : ra,
      dec,
      distance
    };
  }
  
  /**
   * Create RTN (Radial-Tangential-Normal) coordinate system for spacecraft
   * R: Radial (Sun to spacecraft)
   * T: Tangential (velocity direction)
   * N: Normal (R × T)
   */
  static createRtnBasis(position: THREE.Vector3, velocity: THREE.Vector3): THREE.Matrix4 {
    // R axis: radial direction
    const r = position.clone().normalize();
    
    // T axis: tangential (approximately velocity direction)
    const v = velocity.clone();
    const t = v.sub(v.clone().projectOnVector(r)).normalize();
    
    // N axis: normal
    const n = new THREE.Vector3().crossVectors(r, t).normalize();
    
    // Recompute T to ensure orthogonality
    const tOrth = new THREE.Vector3().crossVectors(n, r).normalize();
    
    return new THREE.Matrix4().makeBasis(r, tOrth, n);
  }
  
  /**
   * Transform vector to RTN coordinates
   */
  static toRtn(vector: THREE.Vector3, position: THREE.Vector3, velocity: THREE.Vector3): THREE.Vector3 {
    const rtnBasis = this.createRtnBasis(position, velocity);
    const rtnInverse = rtnBasis.clone().invert();
    return vector.clone().applyMatrix4(rtnInverse);
  }
  
  /**
   * Transform vector from RTN to original coordinates
   */
  static fromRtn(vectorRtn: THREE.Vector3, position: THREE.Vector3, velocity: THREE.Vector3): THREE.Vector3 {
    const rtnBasis = this.createRtnBasis(position, velocity);
    return vectorRtn.clone().applyMatrix4(rtnBasis);
  }
  
  /**
   * Precess coordinates from one epoch to another
   * Uses simplified precession model
   */
  static precess(position: THREE.Vector3, fromEpoch: number, toEpoch: number): THREE.Vector3 {
    const dt = (toEpoch - fromEpoch) / 36525.0;  // Centuries
    
    // Precession angles (simplified)
    const zeta = degToRad(0.6406161 * dt + 0.0000839 * dt * dt);
    const theta = degToRad(0.5567530 * dt - 0.0001185 * dt * dt);
    const z = degToRad(0.6406161 * dt + 0.0003041 * dt * dt);
    
    // Precession matrix
    const cosZeta = Math.cos(zeta);
    const sinZeta = Math.sin(zeta);
    const cosTheta = Math.cos(theta);
    const sinTheta = Math.sin(theta);
    const cosZ = Math.cos(z);
    const sinZ = Math.sin(z);
    
    const p11 = cosZeta * cosZ - sinZeta * cosTheta * sinZ;
    const p12 = -sinZeta * cosZ - cosZeta * cosTheta * sinZ;
    const p13 = sinTheta * sinZ;
    
    const p21 = cosZeta * sinZ + sinZeta * cosTheta * cosZ;
    const p22 = -sinZeta * sinZ + cosZeta * cosTheta * cosZ;
    const p23 = -sinTheta * cosZ;
    
    const p31 = sinZeta * sinTheta;
    const p32 = cosZeta * sinTheta;
    const p33 = cosTheta;
    
    const precessionMatrix = new THREE.Matrix4().set(
      p11, p12, p13, 0,
      p21, p22, p23, 0,
      p31, p32, p33, 0,
      0, 0, 0, 1
    );
    
    return position.clone().applyMatrix4(precessionMatrix);
  }
  
  /**
   * Apply aberration correction for stellar positions
   * Due to Earth's orbital motion
   */
  static applyAberration(position: THREE.Vector3, earthVelocity: THREE.Vector3): THREE.Vector3 {
    const c = 299792.458;  // Speed of light in km/s
    const beta = earthVelocity.length() / c;
    
    if (beta < 1e-6) return position.clone();
    
    const n = position.clone().normalize();
    const v = earthVelocity.normalize();
    
    const gamma = 1 / Math.sqrt(1 - beta * beta);
    const dot = n.dot(v);
    
    // Relativistic aberration formula
    const factor = (gamma * (1 + beta * dot) - 1) / beta;
    
    return n.add(v.multiplyScalar(factor)).normalize().multiplyScalar(position.length());
  }
  
  /**
   * Convert between AU and kilometers
   */
  static auToKm(au: number): number {
    return au * ASTRONOMICAL_CONSTANTS.AU_TO_KM;
  }
  
  static kmToAu(km: number): number {
    return km / ASTRONOMICAL_CONSTANTS.AU_TO_KM;
  }
}

/**
 * Helper class for working with angles
 */
export class AngleUtils {
  /**
   * Normalize angle to [0, 360) degrees
   */
  static normalizeDegrees(degrees: number): number {
    let result = degrees % 360;
    if (result < 0) result += 360;
    return result;
  }
  
  /**
   * Normalize angle to [0, 2π) radians
   */
  static normalizeRadians(radians: number): number {
    let result = radians % (2 * Math.PI);
    if (result < 0) result += 2 * Math.PI;
    return result;
  }
  
  /**
   * Calculate angular separation between two points
   */
  static angularSeparation(ra1: number, dec1: number, ra2: number, dec2: number): number {
    const ra1Rad = degToRad(ra1);
    const dec1Rad = degToRad(dec1);
    const ra2Rad = degToRad(ra2);
    const dec2Rad = degToRad(dec2);
    
    // Haversine formula
    const dRa = ra2Rad - ra1Rad;
    const dDec = dec2Rad - dec1Rad;
    
    const a = Math.sin(dDec / 2) * Math.sin(dDec / 2) +
              Math.cos(dec1Rad) * Math.cos(dec2Rad) *
              Math.sin(dRa / 2) * Math.sin(dRa / 2);
    
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    
    return radToDeg(c);
  }
}
