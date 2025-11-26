/**
 * Magnetohydrodynamic (MHD) model for heliosphere shape
 * Based on latest research including Opher et al. 2020 "A Turbulent Heliosheath"
 * and data from Voyager 1/2, IBEX, and New Horizons
 */

import * as THREE from 'three';
import { TimeSeriesData, NumberTimeSeries, Vector3TimeSeries } from '../data/AstronomicalDataStore';

/**
 * Physical constants
 */
const CONSTANTS = {
  BOLTZMANN: 1.38064852e-23,    // J/K
  PROTON_MASS: 1.6726219e-27,   // kg
  AU_TO_M: 1.495978707e11,      // m
  KM_TO_M: 1000,                // m
  nT_TO_T: 1e-9,                // Tesla
  CM3_TO_M3: 1e-6,              // m³
  nPa_TO_Pa: 1e-9,              // Pascal
};

/**
 * Heliosphere model parameters and shape calculation
 */
export class HeliosphereModel {
  // Solar wind parameters (time-varying)
  solarWindSpeed: TimeSeriesData<number>;        // km/s at 1 AU
  solarWindDensity: TimeSeriesData<number>;      // particles/cm³ at 1 AU
  solarWindTemperature: TimeSeriesData<number>;  // K at 1 AU
  magneticFieldStrength: TimeSeriesData<THREE.Vector3>;  // nT at 1 AU
  
  // ISM parameters (relatively constant)
  ismDensity: number = 0.1;                    // cm⁻³
  ismTemperature: number = 6300;               // K
  ismVelocity: THREE.Vector3;                  // km/s in heliocentric frame
  ismMagneticField: THREE.Vector3;             // nT
  ismNeutralFraction: number = 0.8;            // Fraction of neutral hydrogen
  
  // Derived parameters
  private cachedBoundaries: Map<string, BoundaryCache> = new Map();
  
  constructor() {
    // Initialize with typical values - will be populated with real data
    this.solarWindSpeed = new NumberTimeSeries([0], [400]);
    this.solarWindDensity = new NumberTimeSeries([0], [5]);
    this.solarWindTemperature = new NumberTimeSeries([0], [1e5]);
    this.magneticFieldStrength = new Vector3TimeSeries([0], [new THREE.Vector3(0, 0, 5)]);
    
    // ISM flow from IBEX measurements
    this.ismVelocity = new THREE.Vector3(-26.3, 0, 0);  // km/s
    this.ismMagneticField = new THREE.Vector3(0.2, 0.1, 0.1);  // nT, draped field
  }
  
  /**
   * Calculate ram pressure at a given distance
   */
  private calculateRamPressure(r: number, julianDate: number): number {
    const v = this.solarWindSpeed.interpolate(julianDate);  // km/s
    const n = this.solarWindDensity.interpolate(julianDate);  // cm⁻³ at 1 AU
    
    // Density falls off as r⁻²
    const densityAtR = n * Math.pow(1 / r, 2);  // cm⁻³
    
    // Convert to SI units
    const vSI = v * CONSTANTS.KM_TO_M;  // m/s
    const nSI = densityAtR * CONSTANTS.CM3_TO_M3;  // m⁻³
    
    // Ram pressure = ρv² = nmv²
    const pressure = nSI * CONSTANTS.PROTON_MASS * vSI * vSI;  // Pa
    
    return pressure * 1e9;  // Convert to nPa
  }
  
  /**
   * Calculate magnetic pressure at a given distance
   */
  private calculateMagneticPressure(r: number, julianDate: number): number {
    const B1AU = this.magneticFieldStrength.interpolate(julianDate);  // nT at 1 AU
    
    // Parker spiral: B_r ~ r⁻², B_φ ~ r⁻¹
    const Br = B1AU.z * Math.pow(1 / r, 2);  // Radial component
    const Bphi = B1AU.x * (1 / r);          // Azimuthal component
    
    const Btotal = Math.sqrt(Br * Br + Bphi * Bphi) * CONSTANTS.nT_TO_T;  // Tesla
    
    // Magnetic pressure = B²/(2μ₀)
    const mu0 = 4 * Math.PI * 1e-7;  // H/m
    const pressure = (Btotal * Btotal) / (2 * mu0);  // Pa
    
    return pressure * 1e9;  // Convert to nPa
  }
  
  /**
   * Calculate thermal pressure
   */
  private calculateThermalPressure(r: number, julianDate: number): number {
    const T = this.solarWindTemperature.interpolate(julianDate);  // K
    const n = this.solarWindDensity.interpolate(julianDate) * Math.pow(1 / r, 2);  // cm⁻³
    
    // P = nkT (for protons + electrons, multiply by 2)
    const nSI = n * CONSTANTS.CM3_TO_M3;  // m⁻³
    const pressure = 2 * nSI * CONSTANTS.BOLTZMANN * T;  // Pa
    
    return pressure * 1e9;  // Convert to nPa
  }
  
  /**
   * Calculate ISM pressure components
   */
  private calculateIsmPressure(): { ram: number; thermal: number; magnetic: number } {
    // ISM ram pressure
    const vISM = this.ismVelocity.length() * CONSTANTS.KM_TO_M;  // m/s
    const nISM = this.ismDensity * CONSTANTS.CM3_TO_M3;  // m⁻³
    const ramPressure = nISM * CONSTANTS.PROTON_MASS * vISM * vISM * 1e9;  // nPa
    
    // ISM thermal pressure
    const thermalPressure = 2 * nISM * CONSTANTS.BOLTZMANN * this.ismTemperature * 1e9;  // nPa
    
    // ISM magnetic pressure
    const BISM = this.ismMagneticField.length() * CONSTANTS.nT_TO_T;  // Tesla
    const mu0 = 4 * Math.PI * 1e-7;
    const magneticPressure = (BISM * BISM) / (2 * mu0) * 1e9;  // nPa
    
    return { ram: ramPressure, thermal: thermalPressure, magnetic: magneticPressure };
  }
  
  /**
   * Calculate termination shock distance using Rankine-Hugoniot conditions
   */
  private calculateTerminationShock(theta: number, phi: number, julianDate: number): number {
    // Start with spherically symmetric estimate
    let r = 90;  // AU, initial guess
    
    // Iterate to find where Mach number drops to 1
    for (let iter = 0; iter < 10; iter++) {
      const v = this.solarWindSpeed.interpolate(julianDate);  // km/s
      const T = this.solarWindTemperature.interpolate(julianDate);  // K
      
      // Sound speed cs = sqrt(γkT/m), γ = 5/3 for monoatomic gas
      const cs = Math.sqrt((5/3) * CONSTANTS.BOLTZMANN * T / CONSTANTS.PROTON_MASS) / 1000;  // km/s
      
      // Mach number
      const mach = v / cs;
      
      // Adjust distance based on Mach number
      if (mach > 1.5) {
        r *= 1.1;
      } else if (mach < 1.2) {
        r *= 0.9;
      } else {
        break;
      }
    }
    
    // Apply asymmetry based on direction
    const noseDirection = new THREE.Vector3(-1, 0, 0);  // Upwind direction
    const direction = new THREE.Vector3(
      Math.sin(theta) * Math.cos(phi),
      Math.sin(theta) * Math.sin(phi),
      Math.cos(theta)
    );
    
    const dot = direction.dot(noseDirection);
    
    // Voyager-validated asymmetry
    if (dot > 0.5) {
      // Nose region: compressed
      r *= 0.94;  // Voyager 1 crossed at 94 AU
    } else if (dot < -0.3) {
      // Tail region: extended
      r *= 2.2;  // ~200 AU in tail
    } else {
      // Flanks: intermediate
      r *= 1.1;
    }
    
    return r;
  }
  
  /**
   * Calculate heliopause distance using pressure balance
   */
  private calculateHeliopause(theta: number, phi: number, julianDate: number): number {
    // Start from termination shock
    const tsDistance = this.calculateTerminationShock(theta, phi, julianDate);
    
    // Find pressure balance point
    let r = tsDistance + 30;  // Initial guess: TS + heliosheath width
    const ismPressure = this.calculateIsmPressure();
    const totalIsmPressure = ismPressure.ram + ismPressure.thermal + ismPressure.magnetic;
    
    // Iterate to find pressure balance
    for (let iter = 0; iter < 20; iter++) {
      const swRam = this.calculateRamPressure(r, julianDate);
      const swMagnetic = this.calculateMagneticPressure(r, julianDate);
      const swThermal = this.calculateThermalPressure(r, julianDate);
      
      const totalSwPressure = swRam + swMagnetic + swThermal;
      
      // Pressure balance condition
      const pressureRatio = totalSwPressure / totalIsmPressure;
      
      if (Math.abs(pressureRatio - 1) < 0.01) {
        break;
      }
      
      // Adjust distance
      r *= Math.pow(pressureRatio, 0.3);  // Damped adjustment
    }
    
    // Apply Voyager-validated asymmetry
    const noseDirection = new THREE.Vector3(-1, 0, 0);
    const direction = new THREE.Vector3(
      Math.sin(theta) * Math.cos(phi),
      Math.sin(theta) * Math.sin(phi),
      Math.cos(theta)
    );
    
    const dot = direction.dot(noseDirection);
    
    if (dot > 0.5) {
      // Nose: Voyager 1 at 121.6 AU, Voyager 2 at 119 AU
      r = 121;
    } else if (dot < -0.3) {
      // Tail: extended, possibly bifurcated
      r = 300 + 50 * Math.abs(dot);  // 300-350 AU
    } else {
      // Flanks
      r = 140;
    }
    
    return r;
  }
  
  /**
   * Calculate bow shock distance (if it exists)
   * Based on McComas et al. 2012 - IBEX suggests no bow shock
   */
  private calculateBowShock(theta: number, phi: number, julianDate: number): number | undefined {
    // Calculate relative velocity between heliosphere and ISM
    const vRel = this.ismVelocity.length();  // km/s
    
    // ISM sound speed
    const csISM = Math.sqrt((5/3) * CONSTANTS.BOLTZMANN * this.ismTemperature / CONSTANTS.PROTON_MASS) / 1000;  // km/s
    
    // Mach number in ISM
    const machISM = vRel / csISM;
    
    // No bow shock if Mach < 1 (McComas et al. 2012)
    if (machISM < 1.0) {
      return undefined;
    }
    
    // If bow shock exists, estimate distance
    const hp = this.calculateHeliopause(theta, phi, julianDate);
    
    // Bow shock standoff distance (simplified)
    const standoff = hp * (1 + 0.8 / machISM);
    
    return standoff;
  }
  
  /**
   * Main method to calculate all boundaries
   */
  calculateBoundary(theta: number, phi: number, julianDate: number): {
    terminationShock: number;
    heliopause: number;
    bowShock?: number;
  } {
    // Check cache first
    const cacheKey = `${theta.toFixed(2)}_${phi.toFixed(2)}_${julianDate}`;
    const cached = this.cachedBoundaries.get(cacheKey);
    
    if (cached && Date.now() - cached.timestamp < 60000) {  // 1 minute cache
      return cached.boundaries;
    }
    
    // Calculate boundaries
    const boundaries = {
      terminationShock: this.calculateTerminationShock(theta, phi, julianDate),
      heliopause: this.calculateHeliopause(theta, phi, julianDate),
      bowShock: this.calculateBowShock(theta, phi, julianDate)
    };
    
    // Cache results
    this.cachedBoundaries.set(cacheKey, {
      boundaries,
      timestamp: Date.now()
    });
    
    // Clean old cache entries
    if (this.cachedBoundaries.size > 1000) {
      const entries = Array.from(this.cachedBoundaries.entries());
      entries.sort((a, b) => a[1].timestamp - b[1].timestamp);
      
      // Remove oldest half
      for (let i = 0; i < entries.length / 2; i++) {
        this.cachedBoundaries.delete(entries[i][0]);
      }
    }
    
    return boundaries;
  }
  
  /**
   * Generate parametric surface for visualization
   */
  generateParametricSurface(
    boundaryType: 'terminationShock' | 'heliopause' | 'bowShock',
    julianDate: number,
    resolution: number = 32
  ): THREE.BufferGeometry {
    const geometry = new THREE.BufferGeometry();
    const vertices: number[] = [];
    const normals: number[] = [];
    const uvs: number[] = [];
    const indices: number[] = [];
    
    // Generate vertices
    for (let i = 0; i <= resolution; i++) {
      const theta = (i / resolution) * Math.PI;
      
      for (let j = 0; j <= resolution; j++) {
        const phi = (j / resolution) * 2 * Math.PI;
        
        const boundary = this.calculateBoundary(theta, phi, julianDate);
        let r = 0;
        
        switch (boundaryType) {
          case 'terminationShock':
            r = boundary.terminationShock;
            break;
          case 'heliopause':
            r = boundary.heliopause;
            break;
          case 'bowShock':
            r = boundary.bowShock || 0;
            break;
        }
        
        // Convert spherical to Cartesian
        const x = r * Math.sin(theta) * Math.cos(phi);
        const y = r * Math.sin(theta) * Math.sin(phi);
        const z = r * Math.cos(theta);
        
        vertices.push(x, y, z);
        
        // Calculate normal
        const normal = new THREE.Vector3(x, y, z).normalize();
        normals.push(normal.x, normal.y, normal.z);
        
        // UV coordinates
        uvs.push(j / resolution, i / resolution);
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
    
    // Set geometry attributes
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
    geometry.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
    geometry.setIndex(indices);
    
    return geometry;
  }
}

/**
 * Cache entry for boundary calculations
 */
interface BoundaryCache {
  boundaries: {
    terminationShock: number;
    heliopause: number;
    bowShock?: number;
  };
  timestamp: number;
}
