/**
 * Core data structures for research-grade heliosphere simulation
 * Stores time-series astronomical data with interpolation capabilities
 */

import * as THREE from 'three';

/**
 * Generic time-series data structure with interpolation
 */
export interface TimeSeriesData<T> {
  epochs: number[];  // Julian dates
  values: T[];
  interpolate(jd: number): T;
  getRange(): { start: number; end: number };
}

/**
 * Spacecraft telemetry data point
 */
export interface SpacecraftPosition {
  position: THREE.Vector3;  // Heliocentric coordinates (AU)
  velocity: THREE.Vector3;  // km/s
  distance: number;         // Distance from Sun (AU)
  earthDistance: number;    // Distance from Earth (AU)
  lightTime: number;        // One-way light time (minutes)
}

/**
 * Planetary encounter data
 */
export interface Encounter {
  planet: string;
  date: Date;
  distance: number;  // Closest approach distance (km)
  velocity: number;  // Relative velocity (km/s)
}

/**
 * Spacecraft trajectory data including Voyager 1/2
 */
export interface SpacecraftTrajectoryData {
  name: string;
  launch: Date;
  trajectory: {
    position: TimeSeriesData<THREE.Vector3>;
    velocity: TimeSeriesData<THREE.Vector3>;
    encounters: Encounter[];
  };
  milestones: {
    terminationShock?: { date: Date; distance: number; };
    heliopause?: { date: Date; distance: number; };
    bowShock?: { date: Date; distance: number; };  // If detected
  };
  instruments: {
    magnetometer?: TimeSeriesData<THREE.Vector3>;  // nT
    plasma?: TimeSeriesData<PlasmaData>;
    cosmic_rays?: TimeSeriesData<number>;
  };
  currentPosition?: SpacecraftPosition;  // Real-time position
}

/**
 * Plasma measurement data
 */
export interface PlasmaData {
  density: number;      // particles/cm³
  temperature: number;  // K
  velocity: THREE.Vector3;  // km/s
  pressure: number;     // nPa
}

/**
 * Heliospheric boundary model parameters
 */
export interface HeliosphericBoundary {
  terminationShock: (theta: number, phi: number) => number;  // Distance in AU
  heliopause: (theta: number, phi: number) => number;        // Distance in AU
  bowShock?: (theta: number, phi: number) => number;         // Optional, distance in AU
}

/**
 * Solar activity data
 */
export interface SolarActivityData {
  sunspotNumber: TimeSeriesData<number>;
  f107Flux: TimeSeriesData<number>;        // 10.7 cm radio flux
  solarWindSpeed: TimeSeriesData<number>;   // km/s at 1 AU
  solarWindDensity: TimeSeriesData<number>; // particles/cm³ at 1 AU
  magneticField: TimeSeriesData<number>;    // nT at 1 AU
  coronalHoles: TimeSeriesData<CoronalHoleData[]>;
}

/**
 * Coronal hole data
 */
export interface CoronalHoleData {
  latitude: number;    // Heliographic latitude
  longitude: number;   // Heliographic longitude
  area: number;        // Square degrees
  windSpeed: number;   // Associated wind speed (km/s)
}

/**
 * Interstellar medium properties
 */
export interface ISMProperties {
  density: number;              // Neutral H density (cm⁻³)
  temperature: number;          // K
  velocity: THREE.Vector3;      // km/s in heliocentric frame
  magneticField: THREE.Vector3; // nT
  ionizationFraction: number;   // 0-1
  
  // Hydrogen wall parameters
  hydrogenWallDensity: (position: THREE.Vector3) => number;
  hydrogenWallTemperature: (position: THREE.Vector3) => number;
}

/**
 * Planetary ephemeris data
 */
export interface PlanetaryEphemeris {
  name: string;
  epoch: number;  // Julian date
  position: THREE.Vector3;  // AU
  velocity: THREE.Vector3;  // km/s
  elements: OrbitalElements;
}

/**
 * Keplerian orbital elements
 */
export interface OrbitalElements {
  a: number;      // Semi-major axis (AU)
  e: number;      // Eccentricity
  i: number;      // Inclination (rad)
  omega: number;  // Argument of perihelion (rad)
  Omega: number;  // Longitude of ascending node (rad)
  M: number;      // Mean anomaly at epoch (rad)
  period: number; // Orbital period (years)
}

/**
 * Star catalog entry (Gaia DR3 format)
 */
export interface GaiaStarEntry {
  sourceId: bigint;           // Gaia source ID
  ra: number;                 // Right ascension (deg)
  dec: number;                // Declination (deg)
  parallax: number;           // mas
  parallaxError: number;      // mas
  pmra: number;               // Proper motion RA (mas/yr)
  pmdec: number;              // Proper motion Dec (mas/yr)
  radialVelocity?: number;    // km/s (if available)
  gMag: number;               // G-band magnitude
  bpRp: number;               // BP-RP color index
  temperature?: number;       // Effective temperature (K)
  radius?: number;            // Stellar radius (solar radii)
  luminosity?: number;        // Luminosity (solar luminosities)
}

/**
 * MHD model parameters for heliosphere shape
 */
export interface MHDModelParameters {
  solarWindRamPressure: TimeSeriesData<number>;     // nPa
  magneticFieldStrength: TimeSeriesData<THREE.Vector3>;  // nT
  plasmaBeta: TimeSeriesData<number>;               // Ratio of thermal to magnetic pressure
  machNumber: TimeSeriesData<number>;               // Solar wind Mach number
  
  // ISM parameters
  ismDensity: number;         // cm⁻³
  ismTemperature: number;     // K
  ismVelocity: THREE.Vector3; // km/s
  ismMagneticField: THREE.Vector3;  // nT
  
  // Calculated boundaries
  calculateBoundary(theta: number, phi: number, time: number): {
    terminationShock: number;
    heliopause: number;
    bowShock?: number;
  };
}

/**
 * Main astronomical data store
 */
export class AstronomicalDataStore {
  ephemeris: Map<string, TimeSeriesData<PlanetaryEphemeris>>;
  spacecraft: Map<string, SpacecraftTrajectoryData>;
  heliosphere!: any;  // Will be initialized by service (HeliosphereModel)
  starCatalog: GaiaStarEntry[];
  solarCycle!: SolarActivityData;  // Will be initialized by service
  interstellar: ISMProperties;
  
  constructor() {
    this.ephemeris = new Map();
    this.spacecraft = new Map();
    this.starCatalog = [];
    
    // Initialize with default values - will be populated with real data
    this.interstellar = {
      density: 0.1,  // cm⁻³
      temperature: 6300,  // K
      velocity: new THREE.Vector3(-26.3, 0, 0),  // km/s
      magneticField: new THREE.Vector3(0.2, 0.1, 0.1),  // nT
      ionizationFraction: 0.2,
      hydrogenWallDensity: (pos) => this.calculateHydrogenWallDensity(pos),
      hydrogenWallTemperature: (pos) => this.calculateHydrogenWallTemperature(pos)
    };
  }
  
  /**
   * Calculate hydrogen wall density enhancement
   */
  private calculateHydrogenWallDensity(position: THREE.Vector3): number {
    const r = position.length();
    const heliopauseDistance = 120; // AU
    
    // Enhanced density near heliopause due to charge exchange
    if (r > heliopauseDistance - 20 && r < heliopauseDistance + 10) {
      const enhancement = 1 + 2 * Math.exp(-Math.pow((r - heliopauseDistance) / 10, 2));
      return this.interstellar.density * enhancement;
    }
    
    return this.interstellar.density;
  }
  
  /**
   * Calculate hydrogen wall temperature
   */
  private calculateHydrogenWallTemperature(position: THREE.Vector3): number {
    const r = position.length();
    const heliopauseDistance = 120; // AU
    
    // Heated hydrogen near heliopause
    if (r > heliopauseDistance - 20 && r < heliopauseDistance + 10) {
      const heating = 1 + 0.5 * Math.exp(-Math.pow((r - heliopauseDistance) / 10, 2));
      return this.interstellar.temperature * heating;
    }
    
    return this.interstellar.temperature;
  }
  
  /**
   * Get interpolated ephemeris for a specific body at a given time
   */
  getEphemeris(bodyName: string, julianDate: number): PlanetaryEphemeris | null {
    const data = this.ephemeris.get(bodyName);
    if (!data) return null;
    
    return data.interpolate(julianDate);
  }
  
  /**
   * Get spacecraft position at a specific time
   */
  getSpacecraftPosition(spacecraftName: string, julianDate: number): SpacecraftPosition | null {
    const data = this.spacecraft.get(spacecraftName);
    if (!data) return null;
    
    const position = data.trajectory.position.interpolate(julianDate);
    const velocity = data.trajectory.velocity.interpolate(julianDate);
    const distance = position.length();
    
    // Calculate Earth distance (would need Earth ephemeris)
    const earthDistance = distance; // Simplified - should calculate actual distance
    const lightTime = distance * 8.317; // Minutes (AU to light-minutes)
    
    return {
      position,
      velocity,
      distance,
      earthDistance,
      lightTime
    };
  }
}

/**
 * Implementation of TimeSeriesData for Vector3
 */
export class Vector3TimeSeries implements TimeSeriesData<THREE.Vector3> {
  epochs: number[];
  values: THREE.Vector3[];
  
  constructor(epochs: number[] = [], values: THREE.Vector3[] = []) {
    this.epochs = epochs;
    this.values = values;
  }
  
  interpolate(jd: number): THREE.Vector3 {
    if (this.epochs.length === 0) {
      return new THREE.Vector3();
    }
    
    // Find surrounding epochs
    let i = 0;
    while (i < this.epochs.length - 1 && this.epochs[i + 1] < jd) {
      i++;
    }
    
    // Exact match or extrapolation
    if (i === this.epochs.length - 1 || jd <= this.epochs[0]) {
      return this.values[i].clone();
    }
    
    // Linear interpolation
    const t = (jd - this.epochs[i]) / (this.epochs[i + 1] - this.epochs[i]);
    return new THREE.Vector3().lerpVectors(this.values[i], this.values[i + 1], t);
  }
  
  getRange(): { start: number; end: number } {
    if (this.epochs.length === 0) {
      return { start: 0, end: 0 };
    }
    return {
      start: this.epochs[0],
      end: this.epochs[this.epochs.length - 1]
    };
  }
}

/**
 * Implementation of TimeSeriesData for numbers
 */
export class NumberTimeSeries implements TimeSeriesData<number> {
  epochs: number[];
  values: number[];
  
  constructor(epochs: number[] = [], values: number[] = []) {
    this.epochs = epochs;
    this.values = values;
  }
  
  interpolate(jd: number): number {
    if (this.epochs.length === 0) {
      return 0;
    }
    
    // Find surrounding epochs
    let i = 0;
    while (i < this.epochs.length - 1 && this.epochs[i + 1] < jd) {
      i++;
    }
    
    // Exact match or extrapolation
    if (i === this.epochs.length - 1 || jd <= this.epochs[0]) {
      return this.values[i];
    }
    
    // Linear interpolation
    const t = (jd - this.epochs[i]) / (this.epochs[i + 1] - this.epochs[i]);
    return this.values[i] + t * (this.values[i + 1] - this.values[i]);
  }
  
  getRange(): { start: number; end: number } {
    if (this.epochs.length === 0) {
      return { start: 0, end: 0 };
    }
    return {
      start: this.epochs[0],
      end: this.epochs[this.epochs.length - 1]
    };
  }
}

/**
 * Julian date conversion utilities
 */
export class JulianDate {
  /**
   * Convert Date to Julian Date
   */
  static fromDate(date: Date): number {
    const a = Math.floor((14 - (date.getMonth() + 1)) / 12);
    const y = date.getFullYear() + 4800 - a;
    const m = (date.getMonth() + 1) + 12 * a - 3;
    
    const jdn = date.getDate() + Math.floor((153 * m + 2) / 5) + 365 * y + 
                Math.floor(y / 4) - Math.floor(y / 100) + Math.floor(y / 400) - 32045;
    
    const jd = jdn + (date.getHours() - 12) / 24 + date.getMinutes() / 1440 + 
               date.getSeconds() / 86400 + date.getMilliseconds() / 86400000;
    
    return jd;
  }
  
  /**
   * Convert Julian Date to Date
   */
  static toDate(jd: number): Date {
    const jdn = Math.floor(jd + 0.5);
    const fraction = jd + 0.5 - jdn;
    
    const a = jdn + 32044;
    const b = Math.floor((4 * a + 3) / 146097);
    const c = a - Math.floor((146097 * b) / 4);
    
    const d = Math.floor((4 * c + 3) / 1461);
    const e = c - Math.floor((1461 * d) / 4);
    const m = Math.floor((5 * e + 2) / 153);
    
    const day = e - Math.floor((153 * m + 2) / 5) + 1;
    const month = m + 3 - 12 * Math.floor(m / 10);
    const year = 100 * b + d - 4800 + Math.floor(m / 10);
    
    const hours = Math.floor(fraction * 24);
    const minutes = Math.floor((fraction * 24 - hours) * 60);
    const seconds = Math.floor(((fraction * 24 - hours) * 60 - minutes) * 60);
    const milliseconds = Math.floor((((fraction * 24 - hours) * 60 - minutes) * 60 - seconds) * 1000);
    
    return new Date(year, month - 1, day, hours, minutes, seconds, milliseconds);
  }
}
