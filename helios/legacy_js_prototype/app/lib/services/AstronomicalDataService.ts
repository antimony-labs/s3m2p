/**
 * Service for fetching and managing astronomical data
 * Integrates various data sources and provides a unified API
 */

import * as THREE from 'three';
import { 
  AstronomicalDataStore, 
  JulianDate,
  NumberTimeSeries,
  Vector3TimeSeries,
  SpacecraftTrajectoryData,
  SpacecraftPosition
} from '../data/AstronomicalDataStore';
import { VoyagerTrajectories } from '../physics/SpacecraftTrajectories';
import { PlanetaryEphemeris } from '../data/PlanetaryEphemeris';
import { HeliosphereModel } from '../physics/HeliosphereModel';

/**
 * Solar cycle data (simplified - would fetch from NOAA/NASA)
 */
const SOLAR_CYCLE_DATA = {
  // Solar cycle 24 (2008-2019) and 25 (2020-2031)
  sunspotNumbers: [
    { year: 2008, value: 2.9 },
    { year: 2009, value: 3.1 },
    { year: 2010, value: 16.5 },
    { year: 2011, value: 55.7 },
    { year: 2012, value: 57.7 },
    { year: 2013, value: 64.9 },
    { year: 2014, value: 79.3 },
    { year: 2015, value: 69.8 },
    { year: 2016, value: 39.8 },
    { year: 2017, value: 21.7 },
    { year: 2018, value: 7.0 },
    { year: 2019, value: 3.6 },
    { year: 2020, value: 8.8 },
    { year: 2021, value: 29.6 },
    { year: 2022, value: 73.4 },
    { year: 2023, value: 125.2 },
    { year: 2024, value: 142.0 },  // Predicted
    { year: 2025, value: 145.0 },  // Predicted peak
  ],
  
  // Solar wind parameters vs sunspot number (empirical relations)
  windSpeedRelation: (sunspots: number) => 350 + 2.5 * sunspots,  // km/s
  windDensityRelation: (sunspots: number) => 5 + 0.03 * sunspots,  // cm⁻³
  magneticFieldRelation: (sunspots: number) => 5 + 0.05 * sunspots  // nT
};

/**
 * Main service for astronomical data
 */
export class AstronomicalDataService {
  private dataStore: AstronomicalDataStore;
  private heliosphereModel: HeliosphereModel;
  private initialized: boolean = false;
  
  constructor() {
    this.dataStore = new AstronomicalDataStore();
    this.heliosphereModel = new HeliosphereModel();
  }
  
  /**
   * Initialize the service with data
   */
  async initialize(): Promise<void> {
    if (this.initialized) return;
    
    // Load spacecraft data
    this.loadSpacecraftData();
    
    // Load solar cycle data
    this.loadSolarCycleData();
    
    // Load planetary ephemerides
    this.loadPlanetaryData();
    
    // Initialize heliosphere model
    this.initializeHeliosphereModel();
    
    this.initialized = true;
  }
  
  /**
   * Load spacecraft trajectory data
   */
  private loadSpacecraftData(): void {
    // Voyager 1
    const voyager1 = VoyagerTrajectories.generateVoyager1Trajectory();
    this.dataStore.spacecraft.set('Voyager 1', voyager1);
    
    // Voyager 2
    const voyager2 = VoyagerTrajectories.generateVoyager2Trajectory();
    this.dataStore.spacecraft.set('Voyager 2', voyager2);
    
    // Add current real-time positions
    voyager1.currentPosition = VoyagerTrajectories.getCurrentVoyager1Position();
    voyager2.currentPosition = VoyagerTrajectories.getCurrentVoyager2Position();
  }
  
  /**
   * Load solar cycle data
   */
  private loadSolarCycleData(): void {
    const epochs: number[] = [];
    const sunspotNumbers: number[] = [];
    const windSpeeds: number[] = [];
    const windDensities: number[] = [];
    const magneticFields: number[] = [];
    
    // Convert yearly data to Julian dates
    SOLAR_CYCLE_DATA.sunspotNumbers.forEach(({ year, value }) => {
      const jd = JulianDate.fromDate(new Date(year, 0, 1));
      epochs.push(jd);
      sunspotNumbers.push(value);
      windSpeeds.push(SOLAR_CYCLE_DATA.windSpeedRelation(value));
      windDensities.push(SOLAR_CYCLE_DATA.windDensityRelation(value));
      magneticFields.push(SOLAR_CYCLE_DATA.magneticFieldRelation(value));
    });
    
    // Create time series
    this.dataStore.solarCycle = {
      sunspotNumber: new NumberTimeSeries(epochs, sunspotNumbers),
      f107Flux: new NumberTimeSeries(epochs, sunspotNumbers.map(n => 70 + 0.8 * n)),
      solarWindSpeed: new NumberTimeSeries(epochs, windSpeeds),
      solarWindDensity: new NumberTimeSeries(epochs, windDensities),
      magneticField: new NumberTimeSeries(epochs, magneticFields),
      coronalHoles: {  // Simplified implementation
        epochs: [],
        values: [],
        interpolate: () => [],
        getRange: () => ({ start: 0, end: 0 })
      }
    };
  }
  
  /**
   * Load planetary ephemeris data
   */
  private loadPlanetaryData(): void {
    const startDate = new Date(1900, 0, 1);
    const endDate = new Date(2100, 0, 1);
    const stepDays = 30;  // Monthly resolution
    
    const planets = ['Mercury', 'Venus', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune', 'Pluto'];
    
    planets.forEach(planet => {
      const epochs: number[] = [];
      const ephemerides: any[] = [];
      
      for (let date = new Date(startDate); date <= endDate; date.setDate(date.getDate() + stepDays)) {
        const jd = JulianDate.fromDate(date);
        const ephem = PlanetaryEphemeris.calculatePosition(planet, jd);
        
        epochs.push(jd);
        ephemerides.push(ephem);
      }
      
      // Create time series
      this.dataStore.ephemeris.set(planet, {
        epochs,
        values: ephemerides,
        interpolate: (jd: number) => {
          // For real-time, calculate directly
          return PlanetaryEphemeris.calculatePosition(planet, jd);
        },
        getRange: () => ({ start: epochs[0], end: epochs[epochs.length - 1] })
      });
    });
  }
  
  /**
   * Initialize heliosphere model with solar cycle data
   */
  private initializeHeliosphereModel(): void {
    // Connect solar cycle data to heliosphere model
    this.heliosphereModel.solarWindSpeed = this.dataStore.solarCycle.solarWindSpeed;
    this.heliosphereModel.solarWindDensity = this.dataStore.solarCycle.solarWindDensity;
    
    // Temperature varies with solar cycle
    const tempEpochs = this.dataStore.solarCycle.sunspotNumber.epochs;
    const tempValues = this.dataStore.solarCycle.sunspotNumber.values.map(
      n => 1e5 * (1 + 0.2 * n / 100)  // Temperature increases with activity
    );
    this.heliosphereModel.solarWindTemperature = new NumberTimeSeries(tempEpochs, tempValues);
    
    // Magnetic field (Parker spiral at 1 AU)
    const bEpochs = this.dataStore.solarCycle.magneticField.epochs;
    const bValues = this.dataStore.solarCycle.magneticField.values.map(b => {
      // Simplified Parker spiral: mostly azimuthal at 1 AU
      return new THREE.Vector3(b * 0.8, 0, b * 0.6);  // Bφ dominant
    });
    this.heliosphereModel.magneticFieldStrength = new Vector3TimeSeries(bEpochs, bValues);
    
    // Store reference
    this.dataStore.heliosphere = this.heliosphereModel;
  }
  
  /**
   * Get current spacecraft positions
   */
  getCurrentSpacecraftPositions(): Map<string, SpacecraftPosition> {
    const positions = new Map<string, SpacecraftPosition>();
    
    this.dataStore.spacecraft.forEach((spacecraft, name) => {
      if (spacecraft.currentPosition) {
        positions.set(name, spacecraft.currentPosition);
      }
    });
    
    return positions;
  }
  
  /**
   * Get planetary positions at a specific time
   */
  getPlanetaryPositions(date: Date): Map<string, THREE.Vector3> {
    const jd = JulianDate.fromDate(date);
    const positions = new Map<string, THREE.Vector3>();
    
    this.dataStore.ephemeris.forEach((_, planet) => {
      const ephem = this.dataStore.getEphemeris(planet, jd);
      if (ephem) {
        positions.set(planet, ephem.position);
      }
    });
    
    return positions;
  }
  
  /**
   * Get heliosphere boundaries at current time
   */
  getHeliosphereBoundaries(date: Date) {
    const jd = JulianDate.fromDate(date);
    
    return {
      terminationShock: (theta: number, phi: number) => 
        this.heliosphereModel.calculateBoundary(theta, phi, jd).terminationShock,
      heliopause: (theta: number, phi: number) => 
        this.heliosphereModel.calculateBoundary(theta, phi, jd).heliopause,
      bowShock: (theta: number, phi: number) => 
        this.heliosphereModel.calculateBoundary(theta, phi, jd).bowShock
    };
  }
  
  /**
   * Get solar wind conditions at a specific time and location
   */
  getSolarWindConditions(date: Date, distanceAU: number): {
    speed: number;
    density: number;
    temperature: number;
    magneticField: THREE.Vector3;
    pressure: number;
  } {
    const jd = JulianDate.fromDate(date);
    
    // Get values at 1 AU
    const speed1AU = this.dataStore.solarCycle.solarWindSpeed.interpolate(jd);
    const density1AU = this.dataStore.solarCycle.solarWindDensity.interpolate(jd);
    const temp1AU = this.heliosphereModel.solarWindTemperature.interpolate(jd);
    const b1AU = this.heliosphereModel.magneticFieldStrength.interpolate(jd);
    
    // Scale with distance
    const speed = speed1AU;  // Speed roughly constant
    const density = density1AU / (distanceAU * distanceAU);  // n ∝ r⁻²
    const temperature = temp1AU / Math.pow(distanceAU, 0.7);  // Polytropic
    
    // Parker spiral magnetic field
    const Br = b1AU.z / (distanceAU * distanceAU);  // Radial: B_r ∝ r⁻²
    const Bphi = b1AU.x / distanceAU;  // Azimuthal: B_φ ∝ r⁻¹
    const magneticField = new THREE.Vector3(Bphi, 0, Br);
    
    // Dynamic pressure
    const pressure = density * 1.67e-27 * speed * speed * 1e6 * 1e9;  // nPa
    
    return { speed, density, temperature, magneticField, pressure };
  }
  
  /**
   * Get spacecraft trajectory for visualization
   */
  getSpacecraftTrajectory(
    spacecraftName: string, 
    startDate: Date, 
    endDate: Date, 
    resolution: number = 100
  ): THREE.Vector3[] {
    const spacecraft = this.dataStore.spacecraft.get(spacecraftName);
    if (!spacecraft) return [];
    
    const trajectory: THREE.Vector3[] = [];
    const startJD = JulianDate.fromDate(startDate);
    const endJD = JulianDate.fromDate(endDate);
    
    for (let i = 0; i <= resolution; i++) {
      const t = i / resolution;
      const jd = startJD + t * (endJD - startJD);
      
      const position = spacecraft.trajectory.position.interpolate(jd);
      trajectory.push(position);
    }
    
    return trajectory;
  }
  
  /**
   * Validate Voyager crossing positions
   */
  validateVoyagerCrossings(): {
    voyager1: { ts: boolean; hp: boolean };
    voyager2: { ts: boolean; hp: boolean };
  } {
    const v1Data = this.dataStore.spacecraft.get('Voyager 1');
    const v2Data = this.dataStore.spacecraft.get('Voyager 2');
    
    const tolerance = 1.0;  // AU
    
    return {
      voyager1: {
        ts: v1Data ? Math.abs(v1Data.milestones.terminationShock!.distance - 94.01) < tolerance : false,
        hp: v1Data ? Math.abs(v1Data.milestones.heliopause!.distance - 121.6) < tolerance : false
      },
      voyager2: {
        ts: v2Data ? Math.abs(v2Data.milestones.terminationShock!.distance - 83.7) < tolerance : false,
        hp: v2Data ? Math.abs(v2Data.milestones.heliopause!.distance - 119.0) < tolerance : false
      }
    };
  }
  
  /**
   * Get data store for direct access
   */
  getDataStore(): AstronomicalDataStore {
    return this.dataStore;
  }
  
  /**
   * Get heliosphere model for direct access
   */
  getHeliosphereModel(): HeliosphereModel {
    return this.heliosphereModel;
  }
}

// Singleton instance
let serviceInstance: AstronomicalDataService | null = null;

export function getAstronomicalDataService(): AstronomicalDataService {
  if (!serviceInstance) {
    serviceInstance = new AstronomicalDataService();
  }
  return serviceInstance;
}
