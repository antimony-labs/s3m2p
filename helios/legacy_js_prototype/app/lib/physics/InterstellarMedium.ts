/**
 * Interstellar medium physics and visualization
 * Includes hydrogen wall, magnetic draping, and charge exchange processes
 */

import * as THREE from 'three';
import { HeliosphereModel } from './HeliosphereModel';

/**
 * ISM constants based on IBEX and Voyager measurements
 */
const ISM_CONSTANTS = {
  // Local Interstellar Cloud (LIC) properties
  NEUTRAL_H_DENSITY: 0.1,      // cm⁻³
  IONIZATION_FRACTION: 0.2,    // 20% ionized
  TEMPERATURE: 6300,           // K
  FLOW_SPEED: 26.3,            // km/s
  
  // Flow direction in J2000 ecliptic coordinates
  FLOW_LONGITUDE: 255.4,       // degrees
  FLOW_LATITUDE: 5.2,          // degrees
  
  // Magnetic field (from Voyager measurements)
  B_MAGNITUDE: 0.5,            // nT
  B_DIRECTION: new THREE.Vector3(0.2, 0.1, 0.1).normalize(),
  
  // Physical constants
  CHARGE_EXCHANGE_CROSS_SECTION: 2e-15,  // cm²
  MEAN_FREE_PATH: 100,         // AU for charge exchange
};

/**
 * Hydrogen wall model
 * Region of heated and compressed neutral hydrogen outside heliopause
 */
export class HydrogenWall {
  private heliopauseDistance: (theta: number, phi: number) => number;
  
  constructor(heliosphereModel: HeliosphereModel) {
    // Get heliopause shape function
    const currentJD = 2451545.0; // J2000 for now
    this.heliopauseDistance = (theta: number, phi: number) => 
      heliosphereModel.calculateBoundary(theta, phi, currentJD).heliopause;
  }
  
  /**
   * Calculate hydrogen density enhancement in the wall
   */
  getDensityEnhancement(position: THREE.Vector3): number {
    const r = position.length();
    const theta = Math.acos(position.z / r);
    const phi = Math.atan2(position.y, position.x);
    
    const hp = this.heliopauseDistance(theta, phi);
    
    // Wall peaks just outside heliopause
    const wallCenter = hp + 10; // AU
    const wallWidth = 30; // AU
    
    if (r < hp - 20 || r > hp + 60) {
      return 1.0; // No enhancement
    }
    
    // Gaussian profile with asymmetry
    const relDist = (r - wallCenter) / wallWidth;
    let enhancement = 1 + 1.5 * Math.exp(-relDist * relDist);
    
    // Stronger enhancement in upstream direction
    const flowDir = this.getISMFlowDirection();
    const dot = position.clone().normalize().dot(flowDir);
    if (dot > 0) {
      enhancement *= 1 + 0.5 * dot; // Up to 50% stronger upstream
    }
    
    return enhancement;
  }
  
  /**
   * Calculate temperature in hydrogen wall
   */
  getTemperature(position: THREE.Vector3): number {
    const densityEnhancement = this.getDensityEnhancement(position);
    
    // Temperature increases with compression (adiabatic heating)
    // T ∝ n^(γ-1) where γ = 5/3
    const tempEnhancement = Math.pow(densityEnhancement, 2/3);
    
    return ISM_CONSTANTS.TEMPERATURE * tempEnhancement;
  }
  
  /**
   * Get ISM flow direction vector
   */
  private getISMFlowDirection(): THREE.Vector3 {
    const lon = ISM_CONSTANTS.FLOW_LONGITUDE * Math.PI / 180;
    const lat = ISM_CONSTANTS.FLOW_LATITUDE * Math.PI / 180;
    
    return new THREE.Vector3(
      -Math.cos(lat) * Math.cos(lon),
      -Math.cos(lat) * Math.sin(lon),
      -Math.sin(lat)
    );
  }
  
  /**
   * Generate hydrogen wall visualization geometry
   */
  generateGeometry(resolution: number = 64): THREE.BufferGeometry {
    const geometry = new THREE.BufferGeometry();
    const positions: number[] = [];
    const colors: number[] = [];
    const sizes: number[] = [];
    
    // Generate point cloud representing hydrogen density
    const numPoints = resolution * resolution * 20;
    
    for (let i = 0; i < numPoints; i++) {
      // Random position in spherical shell around heliopause
      const theta = Math.acos(2 * Math.random() - 1);
      const phi = Math.random() * 2 * Math.PI;
      
      const hp = this.heliopauseDistance(theta, phi);
      const r = hp - 20 + Math.random() * 80; // Sample region around heliopause
      
      // Convert to Cartesian
      const x = r * Math.sin(theta) * Math.cos(phi);
      const y = r * Math.sin(theta) * Math.sin(phi);
      const z = r * Math.cos(theta);
      
      const pos = new THREE.Vector3(x, y, z);
      const enhancement = this.getDensityEnhancement(pos);
      
      // Only show enhanced regions
      if (enhancement > 1.1) {
        positions.push(x, y, z);
        
        // Color based on temperature
        const temp = this.getTemperature(pos);
        const colorTemp = Math.min(1, (temp - ISM_CONSTANTS.TEMPERATURE) / 5000);
        colors.push(1, 1 - colorTemp * 0.5, 1 - colorTemp);
        
        // Size based on density
        sizes.push(0.1 + (enhancement - 1) * 0.5);
      }
    }
    
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
    geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
    geometry.setAttribute('size', new THREE.Float32BufferAttribute(sizes, 1));
    
    return geometry;
  }
}

/**
 * Magnetic field draping around heliosphere
 */
export class MagneticDraping {
  private heliosphereModel: HeliosphereModel;
  
  constructor(heliosphereModel: HeliosphereModel) {
    this.heliosphereModel = heliosphereModel;
  }
  
  /**
   * Calculate draped magnetic field at a position
   */
  getDrapedField(position: THREE.Vector3, julianDate: number): THREE.Vector3 {
    const r = position.length();
    const theta = Math.acos(position.z / r);
    const phi = Math.atan2(position.y, position.x);
    
    const boundary = this.heliosphereModel.calculateBoundary(theta, phi, julianDate);
    const hp = boundary.heliopause;
    
    // Undisturbed ISM field far from heliosphere
    if (r > hp * 2) {
      return ISM_CONSTANTS.B_DIRECTION.clone().multiplyScalar(ISM_CONSTANTS.B_MAGNITUDE);
    }
    
    // Inside heliosphere - no ISM field
    if (r < hp) {
      return new THREE.Vector3(0, 0, 0);
    }
    
    // Draping region
    const relDist = (r - hp) / hp;
    
    // Field lines wrap around heliosphere
    const radialDir = position.clone().normalize();
    const flowDir = new THREE.Vector3(-1, 0, 0); // Simplified flow from +X
    
    // Perpendicular component to radial direction
    const perpComponent = ISM_CONSTANTS.B_DIRECTION.clone()
      .sub(radialDir.clone().multiplyScalar(ISM_CONSTANTS.B_DIRECTION.dot(radialDir)));
    
    // Draping increases field strength
    const compressionFactor = 1 + 2 * Math.exp(-relDist * 2);
    
    // Field tends to wrap around obstacle
    const tangentialDir = new THREE.Vector3()
      .crossVectors(radialDir, flowDir)
      .normalize();
    
    // Interpolate between original and tangential direction
    const drapingFactor = Math.exp(-relDist);
    const drapedDir = perpComponent.clone()
      .lerp(tangentialDir, drapingFactor)
      .normalize();
    
    return drapedDir.multiplyScalar(ISM_CONSTANTS.B_MAGNITUDE * compressionFactor);
  }
  
  /**
   * Generate magnetic field lines for visualization
   */
  generateFieldLines(
    julianDate: number,
    numLines: number = 20,
    stepsPerLine: number = 100
  ): THREE.BufferGeometry[] {
    const geometries: THREE.BufferGeometry[] = [];
    
    for (let i = 0; i < numLines; i++) {
      // Start positions upstream
      const y = (i / (numLines - 1) - 0.5) * 400;
      const z = (Math.random() - 0.5) * 200;
      const startPos = new THREE.Vector3(300, y, z);
      
      const points: THREE.Vector3[] = [startPos];
      let currentPos = startPos.clone();
      
      // Trace field line
      for (let step = 0; step < stepsPerLine; step++) {
        const field = this.getDrapedField(currentPos, julianDate);
        
        if (field.length() < 0.01) break; // Inside heliosphere
        
        // Step along field direction
        const stepSize = 5; // AU
        field.normalize();
        
        // Add some flow-aligned motion
        const flowComponent = new THREE.Vector3(-0.3, 0, 0);
        const nextPos = currentPos.clone()
          .add(field.multiplyScalar(stepSize))
          .add(flowComponent.multiplyScalar(stepSize));
        
        points.push(nextPos);
        currentPos = nextPos;
        
        // Stop if too far
        if (currentPos.length() > 400 || currentPos.x < -200) break;
      }
      
      if (points.length > 1) {
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        geometries.push(geometry);
      }
    }
    
    return geometries;
  }
}

/**
 * Charge exchange processes
 */
export class ChargeExchange {
  /**
   * Calculate charge exchange rate
   */
  static getChargeExchangeRate(
    neutralDensity: number,    // cm⁻³
    ionDensity: number,        // cm⁻³
    relativeVelocity: number   // km/s
  ): number {
    // Rate = n_n * n_i * σ * v
    const crossSection = ISM_CONSTANTS.CHARGE_EXCHANGE_CROSS_SECTION; // cm²
    const velocity = relativeVelocity * 1e5; // cm/s
    
    return neutralDensity * ionDensity * crossSection * velocity; // reactions/cm³/s
  }
  
  /**
   * Calculate mean free path for charge exchange
   */
  static getMeanFreePath(
    targetDensity: number,     // cm⁻³
    relativeVelocity: number   // km/s
  ): number {
    if (targetDensity === 0) return Infinity;
    
    const crossSection = ISM_CONSTANTS.CHARGE_EXCHANGE_CROSS_SECTION; // cm²
    const mfp_cm = 1 / (targetDensity * crossSection);
    const mfp_au = mfp_cm / 1.496e13; // Convert to AU
    
    return mfp_au;
  }
}

/**
 * ISM flow visualization with proper deflection
 */
export class InterstellarFlow {
  private heliosphereModel: HeliosphereModel;
  private hydrogenWall: HydrogenWall;
  
  constructor(heliosphereModel: HeliosphereModel) {
    this.heliosphereModel = heliosphereModel;
    this.hydrogenWall = new HydrogenWall(heliosphereModel);
  }
  
  /**
   * Calculate flow deflection around heliosphere
   */
  getDeflectedFlow(position: THREE.Vector3, julianDate: number): THREE.Vector3 {
    const r = position.length();
    const theta = Math.acos(position.z / r);
    const phi = Math.atan2(position.y, position.x);
    
    const boundary = this.heliosphereModel.calculateBoundary(theta, phi, julianDate);
    const hp = boundary.heliopause;
    const bs = boundary.bowShock;
    
    // Undeflected flow far upstream
    const baseFlow = new THREE.Vector3(-ISM_CONSTANTS.FLOW_SPEED, 0, 0);
    
    if (r > (bs || hp * 2)) {
      return baseFlow;
    }
    
    // No flow inside heliosphere
    if (r < hp) {
      return new THREE.Vector3(0, 0, 0);
    }
    
    // Deflection region
    const radialDir = position.clone().normalize();
    const flowDir = new THREE.Vector3(-1, 0, 0);
    
    // Flow deflects around obstacle
    const dot = radialDir.dot(flowDir);
    const deflectionStrength = Math.exp(-(r - hp) / 20); // Decays over ~20 AU
    
    // Perpendicular deflection
    const perpDir = radialDir.clone()
      .sub(flowDir.multiplyScalar(dot))
      .normalize();
    
    // Deflected flow
    const deflectedFlow = baseFlow.clone();
    if (perpDir.length() > 0.1) {
      deflectedFlow.add(perpDir.multiplyScalar(
        ISM_CONSTANTS.FLOW_SPEED * deflectionStrength * (1 - Math.abs(dot))
      ));
    }
    
    // Reduce flow speed near stagnation point
    if (dot > 0.8) {
      deflectedFlow.multiplyScalar(1 - deflectionStrength * dot);
    }
    
    return deflectedFlow;
  }
  
  /**
   * Generate streamlines for ISM flow
   */
  generateStreamlines(
    julianDate: number,
    numLines: number = 30,
    stepsPerLine: number = 200
  ): THREE.BufferGeometry[] {
    const geometries: THREE.BufferGeometry[] = [];
    
    // Start positions in grid upstream
    for (let i = 0; i < numLines; i++) {
      const y = (i / (numLines - 1) - 0.5) * 300;
      
      for (let j = 0; j < numLines / 3; j++) {
        const z = (j / (numLines / 3 - 1) - 0.5) * 150;
        const startPos = new THREE.Vector3(350, y, z);
        
        const points: THREE.Vector3[] = [];
        let currentPos = startPos.clone();
        
        // Trace streamline
        for (let step = 0; step < stepsPerLine; step++) {
          points.push(currentPos.clone());
          
          const flow = this.getDeflectedFlow(currentPos, julianDate);
          if (flow.length() < 1) break; // Stagnation region
          
          // Step along flow
          const dt = 0.5; // Time step
          currentPos.add(flow.clone().multiplyScalar(dt));
          
          // Stop conditions
          if (currentPos.x < -300 || currentPos.length() > 500) break;
          
          // Check if inside heliosphere
          const r = currentPos.length();
          const theta = Math.acos(currentPos.z / r);
          const phi = Math.atan2(currentPos.y, currentPos.x);
          const hp = this.heliosphereModel.calculateBoundary(theta, phi, julianDate).heliopause;
          
          if (r < hp - 5) break; // Don't enter heliosphere
        }
        
        if (points.length > 10) {
          const geometry = new THREE.BufferGeometry().setFromPoints(points);
          geometries.push(geometry);
        }
      }
    }
    
    return geometries;
  }
  
  /**
   * Generate particle system for ISM flow
   */
  generateParticles(numParticles: number = 5000): {
    geometry: THREE.BufferGeometry;
    velocities: Float32Array;
  } {
    const positions = new Float32Array(numParticles * 3);
    const velocities = new Float32Array(numParticles * 3);
    const colors = new Float32Array(numParticles * 3);
    
    for (let i = 0; i < numParticles; i++) {
      // Random starting position upstream
      const x = 250 + Math.random() * 150;
      const y = (Math.random() - 0.5) * 400;
      const z = (Math.random() - 0.5) * 300;
      
      positions[i * 3] = x;
      positions[i * 3 + 1] = y;
      positions[i * 3 + 2] = z;
      
      // Initial velocity
      velocities[i * 3] = -ISM_CONSTANTS.FLOW_SPEED * 0.001; // Scale for animation
      velocities[i * 3 + 1] = 0;
      velocities[i * 3 + 2] = 0;
      
      // Color (blue for cold neutral hydrogen)
      colors[i * 3] = 0.6;
      colors[i * 3 + 1] = 0.7;
      colors[i * 3 + 2] = 1.0;
    }
    
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));
    
    return { geometry, velocities };
  }
}
