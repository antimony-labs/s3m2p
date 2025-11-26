/**
 * Solar wind physics implementation
 * Includes Parker spiral model, MHD shock physics, and stream interactions
 */

import * as THREE from 'three';
import { NumberTimeSeries, JulianDate } from '../data/AstronomicalDataStore';

/**
 * Solar wind constants
 */
const SOLAR_WIND_CONSTANTS = {
  // Typical values at 1 AU
  FAST_WIND_SPEED: 750,        // km/s (from coronal holes)
  SLOW_WIND_SPEED: 400,        // km/s (from streamer belt)
  TYPICAL_DENSITY: 5,          // protons/cm³ at 1 AU
  TYPICAL_TEMPERATURE: 1.2e5,  // K at 1 AU
  
  // Solar rotation
  CARRINGTON_PERIOD: 27.2753,  // days (synodic)
  SIDEREAL_PERIOD: 25.38,      // days at equator
  
  // Physical constants
  SOLAR_RADIUS: 6.96e5,        // km
  AU_KM: 1.496e8,             // km
  PROTON_MASS: 1.673e-27,     // kg
  BOLTZMANN: 1.38e-23,        // J/K
};

/**
 * Parker spiral magnetic field model
 */
export class ParkerSpiral {
  private solarRotationRate: number;  // rad/s
  private solarMagneticField: number; // Tesla at solar surface
  
  constructor() {
    // Omega = 2π / (rotation period in seconds)
    this.solarRotationRate = 2 * Math.PI / (SOLAR_WIND_CONSTANTS.SIDEREAL_PERIOD * 24 * 3600);
    this.solarMagneticField = 1e-4; // 1 Gauss = 10^-4 Tesla
  }
  
  /**
   * Calculate Parker spiral angle at given distance
   */
  getSpiralAngle(r: number, solarWindSpeed: number): number {
    // r in AU, speed in km/s
    const rMeters = r * SOLAR_WIND_CONSTANTS.AU_KM * 1000;
    const vMeters = solarWindSpeed * 1000;
    
    // Spiral angle: tan(ψ) = Ωr/v
    const tanPsi = (this.solarRotationRate * rMeters) / vMeters;
    return Math.atan(tanPsi);
  }
  
  /**
   * Calculate magnetic field components in heliocentric coordinates
   */
  getMagneticField(
    position: THREE.Vector3,  // Position in AU
    solarWindSpeed: number,   // km/s
    sourceLatitude: number = 0 // Heliographic latitude of source
  ): THREE.Vector3 {
    const r = position.length();
    const theta = Math.acos(position.z / r); // Co-latitude
    const phi = Math.atan2(position.y, position.x); // Azimuth
    
    // Spiral angle
    const spiralAngle = this.getSpiralAngle(r, solarWindSpeed);
    
    // Field strength scales as r^-2 for B_r, r^-1 for B_phi
    const B0 = this.solarMagneticField * Math.pow(SOLAR_WIND_CONSTANTS.SOLAR_RADIUS / (r * SOLAR_WIND_CONSTANTS.AU_KM), 2);
    
    // Parker spiral components (simplified - assumes radial solar wind)
    const Br = B0;
    const Btheta = 0; // Neglecting latitudinal component
    const Bphi = -B0 * r * Math.sin(spiralAngle) / Math.cos(spiralAngle);
    
    // Convert to Cartesian
    const sinTheta = Math.sin(theta);
    const cosTheta = Math.cos(theta);
    const sinPhi = Math.sin(phi);
    const cosPhi = Math.cos(phi);
    
    const Bx = Br * sinTheta * cosPhi + Bphi * (-sinPhi);
    const By = Br * sinTheta * sinPhi + Bphi * cosPhi;
    const Bz = Br * cosTheta;
    
    // Convert to nT
    return new THREE.Vector3(Bx, By, Bz).multiplyScalar(1e9);
  }
  
  /**
   * Get field line path from source point
   */
  getFieldLine(
    sourcePoint: THREE.Vector3,  // Starting point in AU
    solarWindSpeed: number,      // km/s
    maxDistance: number = 100,   // AU
    steps: number = 100
  ): THREE.Vector3[] {
    const points: THREE.Vector3[] = [];
    
    // Initial position
    let pos = sourcePoint.clone();
    points.push(pos.clone());
    
    // Trace field line
    const dr = (maxDistance - pos.length()) / steps;
    
    for (let i = 0; i < steps; i++) {
      const r = pos.length();
      if (r >= maxDistance) break;
      
      // Get local field direction
      const B = this.getMagneticField(pos, solarWindSpeed);
      B.normalize();
      
      // Step along field line (simplified - should use proper integration)
      const radialDir = pos.clone().normalize();
      
      // Move mostly radially, with azimuthal component from spiral
      const spiralAngle = this.getSpiralAngle(r, solarWindSpeed);
      const stepVector = radialDir.multiplyScalar(dr);
      
      // Add azimuthal motion
      const phi = Math.atan2(pos.y, pos.x);
      const dPhi = -spiralAngle * dr / r;
      const newPhi = phi + dPhi;
      
      // Update position
      const newR = r + dr;
      pos.x = newR * Math.cos(newPhi);
      pos.y = newR * Math.sin(newPhi);
      pos.z = pos.z * (newR / r); // Maintain latitude
      
      points.push(pos.clone());
    }
    
    return points;
  }
}

/**
 * Stream interaction regions (CIRs/SIRs)
 */
export class StreamInteractionRegion {
  /**
   * Calculate compression at stream interface
   */
  static calculateCompression(
    fastSpeed: number,   // km/s
    slowSpeed: number,   // km/s
    distance: number     // AU
  ): {
    compressionRatio: number;
    shockFormed: boolean;
    pressure: number;  // nPa
  } {
    const speedDiff = fastSpeed - slowSpeed;
    
    // Compression ratio from Rankine-Hugoniot relations (simplified)
    const soundSpeed = 50; // km/s typical in solar wind
    const machNumber = speedDiff / soundSpeed;
    
    let compressionRatio = 1;
    let shockFormed = false;
    
    if (machNumber > 1 && distance > 1) {
      // Shock formation
      shockFormed = true;
      // Strong shock limit: compression ratio = (γ+1)/(γ-1) = 4 for γ=5/3
      compressionRatio = Math.min(4, 1 + machNumber * machNumber / 2);
    } else {
      // Compression wave
      compressionRatio = 1 + 0.5 * machNumber * Math.exp(-distance / 5);
    }
    
    // Ram pressure
    const density = SOLAR_WIND_CONSTANTS.TYPICAL_DENSITY / (distance * distance); // n ∝ r⁻²
    const avgSpeed = (fastSpeed + slowSpeed) / 2;
    const pressure = density * SOLAR_WIND_CONSTANTS.PROTON_MASS * 
                    avgSpeed * avgSpeed * 1e6 * compressionRatio * 1e9; // nPa
    
    return { compressionRatio, shockFormed, pressure };
  }
}

/**
 * Coronal Mass Ejection (CME) model
 */
export class CoronalMassEjection {
  position: THREE.Vector3;     // Current position (AU)
  velocity: THREE.Vector3;     // km/s
  radius: number;              // AU
  density: number;             // particles/cm³
  magneticField: THREE.Vector3; // nT
  launchTime: number;          // Julian date
  
  constructor(params: {
    launchTime: number;
    initialSpeed: number;    // km/s
    direction: THREE.Vector3; // Unit vector
    mass: number;            // kg
    width: number;           // Angular width (degrees)
  }) {
    this.launchTime = params.launchTime;
    this.velocity = params.direction.clone().multiplyScalar(params.initialSpeed);
    this.position = params.direction.clone().multiplyScalar(0.01); // Start near Sun
    
    // Estimate initial radius from angular width
    this.radius = 0.01 * Math.tan((params.width / 2) * Math.PI / 180);
    
    // Estimate density from mass
    const volume = (4/3) * Math.PI * Math.pow(this.radius * SOLAR_WIND_CONSTANTS.AU_KM * 1e3, 3);
    const numberDensity = params.mass / (SOLAR_WIND_CONSTANTS.PROTON_MASS * volume);
    this.density = numberDensity * 1e-6; // Convert to cm⁻³
    
    // Flux rope magnetic field (simplified toroidal)
    this.magneticField = new THREE.Vector3(10, 0, 0); // nT
  }
  
  /**
   * Propagate CME to given time
   */
  propagate(currentJD: number): void {
    const dt = (currentJD - this.launchTime) * 24 * 3600; // seconds
    
    if (dt <= 0) return;
    
    // Simple ballistic propagation (real CMEs decelerate)
    const distance = this.velocity.length() * dt / (SOLAR_WIND_CONSTANTS.AU_KM * 1000);
    this.position = this.velocity.clone().normalize().multiplyScalar(distance);
    
    // Expansion (self-similar)
    this.radius = 0.01 + distance * 0.1; // Expands to ~10% of distance
    
    // Density decreases with expansion
    const expansionFactor = Math.pow(this.radius / 0.01, 3);
    this.density = this.density / expansionFactor;
    
    // Magnetic field decreases with distance
    this.magneticField.multiplyScalar(1 / (distance * distance));
  }
  
  /**
   * Check if position is inside CME
   */
  contains(position: THREE.Vector3): boolean {
    return position.distanceTo(this.position) < this.radius;
  }
}

/**
 * Solar wind stream structure
 */
export class SolarWindStream {
  private baseSpeed: NumberTimeSeries;      // Speed at source
  private sourceLatitude: number;           // Heliographic latitude
  private sourceType: 'coronalHole' | 'streamer';
  
  constructor(
    sourceType: 'coronalHole' | 'streamer',
    sourceLatitude: number,
    speedProfile?: NumberTimeSeries
  ) {
    this.sourceType = sourceType;
    this.sourceLatitude = sourceLatitude;
    
    // Default speed profiles
    if (!speedProfile) {
      const epochs = [0];
      const speeds = sourceType === 'coronalHole' ? 
        [SOLAR_WIND_CONSTANTS.FAST_WIND_SPEED] :
        [SOLAR_WIND_CONSTANTS.SLOW_WIND_SPEED];
      this.baseSpeed = new NumberTimeSeries(epochs, speeds);
    } else {
      this.baseSpeed = speedProfile;
    }
  }
  
  /**
   * Get stream properties at given position and time
   */
  getProperties(position: THREE.Vector3, julianDate: number): {
    speed: number;
    density: number;
    temperature: number;
    pressure: number;
  } {
    const r = position.length(); // AU
    const speed = this.baseSpeed.interpolate(julianDate);
    
    // Density falls as r⁻²
    const density = SOLAR_WIND_CONSTANTS.TYPICAL_DENSITY / (r * r);
    
    // Temperature evolution (polytropic)
    const T0 = this.sourceType === 'coronalHole' ? 1.5e6 : 1.0e6; // K at corona
    const temperature = T0 * Math.pow(r / 0.1, -0.7); // Polytropic index ~1.3
    
    // Dynamic pressure
    const pressure = density * SOLAR_WIND_CONSTANTS.PROTON_MASS * 
                    speed * speed * 1e6 * 1e9; // nPa
    
    return { speed, density, temperature, pressure };
  }
}

/**
 * Heliospheric current sheet
 */
export class HeliosphericCurrentSheet {
  private tilt: number; // Degrees
  private phase: number; // Carrington rotation phase
  
  constructor(tiltAngle: number = 15) {
    this.tilt = tiltAngle;
    this.phase = 0;
  }
  
  /**
   * Update tilt angle based on solar cycle
   */
  updateTilt(sunspotNumber: number): void {
    // Empirical relation: tilt increases with activity
    this.tilt = 5 + 0.5 * sunspotNumber;
    this.tilt = Math.min(75, Math.max(5, this.tilt));
  }
  
  /**
   * Get current sheet position (simplified wavy sheet)
   */
  isAboveSheet(position: THREE.Vector3, carringtonRotation: number): boolean {
    const r = position.length();
    const theta = Math.acos(position.z / r) * 180 / Math.PI; // Co-latitude in degrees
    const phi = Math.atan2(position.y, position.x);
    
    // Wavy current sheet (Parker model)
    const sheetLatitude = this.tilt * Math.sin(phi + this.phase);
    const sheetColatitude = 90 - sheetLatitude;
    
    return theta < sheetColatitude;
  }
  
  /**
   * Generate current sheet surface for visualization
   */
  generateSurface(
    innerRadius: number = 0.1,
    outerRadius: number = 100,
    resolution: number = 64
  ): THREE.BufferGeometry {
    const geometry = new THREE.BufferGeometry();
    const vertices: number[] = [];
    const normals: number[] = [];
    const uvs: number[] = [];
    const indices: number[] = [];
    
    // Generate vertices
    for (let i = 0; i <= resolution; i++) {
      const u = i / resolution;
      const r = innerRadius + u * (outerRadius - innerRadius);
      
      for (let j = 0; j <= resolution; j++) {
        const v = j / resolution;
        const phi = v * 2 * Math.PI;
        
        // Current sheet latitude
        const latitude = this.tilt * Math.sin(phi) * Math.PI / 180;
        const colatitude = Math.PI / 2 - latitude;
        
        // Spherical to Cartesian
        const x = r * Math.sin(colatitude) * Math.cos(phi);
        const y = r * Math.sin(colatitude) * Math.sin(phi);
        const z = r * Math.cos(colatitude);
        
        vertices.push(x, y, z);
        
        // Normal (pointing away from origin)
        const normal = new THREE.Vector3(x, y, z).normalize();
        normals.push(normal.x, normal.y, normal.z);
        
        // UV
        uvs.push(u, v);
      }
    }
    
    // Generate indices
    for (let i = 0; i < resolution; i++) {
      for (let j = 0; j < resolution; j++) {
        const a = i * (resolution + 1) + j;
        const b = a + 1;
        const c = a + resolution + 1;
        const d = c + 1;
        
        indices.push(a, b, c);
        indices.push(b, d, c);
      }
    }
    
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
    geometry.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
    geometry.setIndex(indices);
    
    return geometry;
  }
}
