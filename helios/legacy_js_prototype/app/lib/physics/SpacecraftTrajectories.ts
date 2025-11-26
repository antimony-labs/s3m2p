/**
 * Spacecraft trajectory data and calculations
 * Includes Voyager 1, Voyager 2, Pioneer 10, Pioneer 11, and New Horizons
 * Based on JPL Horizons ephemeris data
 */

import * as THREE from 'three';
import { 
  SpacecraftTrajectoryData, 
  SpacecraftPosition,
  Encounter,
  Vector3TimeSeries,
  NumberTimeSeries,
  JulianDate,
  PlasmaData
} from '../data/AstronomicalDataStore';

/**
 * Voyager mission milestones and trajectory data
 */
export class VoyagerTrajectories {
  /**
   * Voyager 1 launch and milestones
   */
  static readonly VOYAGER_1 = {
    launch: new Date('1977-09-05T12:56:00Z'),
    
    encounters: [
      { planet: 'Jupiter', date: new Date('1979-03-05'), distance: 349000, velocity: 48 },
      { planet: 'Saturn', date: new Date('1980-11-12'), distance: 124000, velocity: 36 }
    ] as Encounter[],
    
    milestones: {
      terminationShock: { 
        date: new Date('2004-12-16'), 
        distance: 94.01,
        heliocentricLongitude: 34.4,  // degrees
        heliocentricLatitude: 34.9    // degrees
      },
      heliopause: { 
        date: new Date('2012-08-25'), 
        distance: 121.6,
        heliocentricLongitude: 35.0,
        heliocentricLatitude: 34.9
      }
    },
    
    // Current trajectory parameters (as of 2024)
    current: {
      distance: 163.0,  // AU
      velocity: 16.99,  // km/s
      heliocentricLongitude: 35.2,  // degrees
      heliocentricLatitude: 34.9,   // degrees
      annualMotion: 3.523  // AU/year
    }
  };
  
  /**
   * Voyager 2 launch and milestones
   */
  static readonly VOYAGER_2 = {
    launch: new Date('1977-08-20T14:29:00Z'),
    
    encounters: [
      { planet: 'Jupiter', date: new Date('1979-07-09'), distance: 570000, velocity: 45 },
      { planet: 'Saturn', date: new Date('1981-08-25'), distance: 101000, velocity: 35 },
      { planet: 'Uranus', date: new Date('1986-01-24'), distance: 81500, velocity: 28 },
      { planet: 'Neptune', date: new Date('1989-08-25'), distance: 4950, velocity: 27 }
    ] as Encounter[],
    
    milestones: {
      terminationShock: { 
        date: new Date('2007-08-30'), 
        distance: 83.7,
        heliocentricLongitude: 310.4,  // degrees
        heliocentricLatitude: -31.2    // degrees
      },
      heliopause: { 
        date: new Date('2018-11-05'), 
        distance: 119.0,
        heliocentricLongitude: 310.8,
        heliocentricLatitude: -32.2
      }
    },
    
    // Current trajectory parameters (as of 2024)
    current: {
      distance: 136.0,  // AU
      velocity: 15.37,  // km/s
      heliocentricLongitude: 311.0,  // degrees
      heliocentricLatitude: -32.5,   // degrees
      annualMotion: 3.239  // AU/year
    }
  };
  
  /**
   * Generate trajectory data for Voyager 1
   */
  static generateVoyager1Trajectory(): SpacecraftTrajectoryData {
    const data = this.VOYAGER_1;
    const launchJD = JulianDate.fromDate(data.launch);
    const currentJD = JulianDate.fromDate(new Date());
    
    // Key trajectory points (simplified - real data would come from JPL)
    const keyPoints = [
      { jd: launchJD, r: 1.0, lon: 0, lat: 0 },
      { jd: JulianDate.fromDate(data.encounters[0].date), r: 5.2, lon: 15, lat: 2 },
      { jd: JulianDate.fromDate(data.encounters[1].date), r: 9.5, lon: 25, lat: 10 },
      { jd: JulianDate.fromDate(data.milestones.terminationShock.date), r: 94.01, lon: 34.4, lat: 34.9 },
      { jd: JulianDate.fromDate(data.milestones.heliopause.date), r: 121.6, lon: 35.0, lat: 34.9 },
      { jd: currentJD, r: data.current.distance, lon: data.current.heliocentricLongitude, lat: data.current.heliocentricLatitude }
    ];
    
    // Build time series
    const epochs: number[] = [];
    const positions: THREE.Vector3[] = [];
    const velocities: THREE.Vector3[] = [];
    
    // Interpolate trajectory
    const numPoints = 1000;
    for (let i = 0; i < numPoints; i++) {
      const t = i / (numPoints - 1);
      const jd = launchJD + t * (currentJD - launchJD);
      
      // Find surrounding key points
      let idx = 0;
      while (idx < keyPoints.length - 1 && keyPoints[idx + 1].jd < jd) {
        idx++;
      }
      
      let r: number, lon: number, lat: number;
      
      if (idx === keyPoints.length - 1) {
        // Extrapolate beyond last point
        const point = keyPoints[idx];
        const yearsSince = (jd - point.jd) / 365.25;
        r = point.r + data.current.annualMotion * yearsSince;
        lon = point.lon;
        lat = point.lat;
      } else {
        // Interpolate between points
        const p1 = keyPoints[idx];
        const p2 = keyPoints[idx + 1];
        const frac = (jd - p1.jd) / (p2.jd - p1.jd);
        
        r = p1.r + (p2.r - p1.r) * frac;
        lon = p1.lon + (p2.lon - p1.lon) * frac;
        lat = p1.lat + (p2.lat - p1.lat) * frac;
      }
      
      // Convert to Cartesian
      const lonRad = lon * Math.PI / 180;
      const latRad = lat * Math.PI / 180;
      
      const x = r * Math.cos(latRad) * Math.cos(lonRad);
      const y = r * Math.cos(latRad) * Math.sin(lonRad);
      const z = r * Math.sin(latRad);
      
      epochs.push(jd);
      positions.push(new THREE.Vector3(x, y, z));
      
      // Simplified velocity (radial outward)
      const v = new THREE.Vector3(x, y, z).normalize().multiplyScalar(data.current.velocity);
      velocities.push(v);
    }
    
    // Create plasma data time series (simplified)
    const plasmaEpochs: number[] = [];
    const plasmaValues: PlasmaData[] = [];
    
    // Before termination shock: normal solar wind
    const tsJD = JulianDate.fromDate(data.milestones.terminationShock.date);
    const hpJD = JulianDate.fromDate(data.milestones.heliopause.date);
    
    for (let i = 0; i < 100; i++) {
      const jd = launchJD + i * (currentJD - launchJD) / 100;
      
      let density: number, temperature: number, velocity: THREE.Vector3;
      
      if (jd < tsJD) {
        // Supersonic solar wind
        const r = this.getDistanceAtJD(jd, keyPoints);
        density = 5 / (r * r);  // n ∝ r⁻²
        temperature = 1e5;
        velocity = new THREE.Vector3(400, 0, 0);  // Radial outflow
      } else if (jd < hpJD) {
        // Heliosheath: compressed, turbulent
        density = 0.002;  // Compressed
        temperature = 2e5;  // Heated
        velocity = new THREE.Vector3(100, 20, 10);  // Subsonic, turbulent
      } else {
        // Interstellar medium
        density = 0.0001;
        temperature = 6300;
        velocity = new THREE.Vector3(-26, 0, 0);  // ISM flow
      }
      
      plasmaEpochs.push(jd);
      plasmaValues.push({
        density,
        temperature,
        velocity,
        pressure: density * 1.38e-23 * temperature * 2 * 1e15  // nPa
      });
    }
    
    return {
      name: 'Voyager 1',
      launch: data.launch,
      trajectory: {
        position: new Vector3TimeSeries(epochs, positions),
        velocity: new Vector3TimeSeries(epochs, velocities),
        encounters: data.encounters
      },
      milestones: {
        terminationShock: data.milestones.terminationShock,
        heliopause: data.milestones.heliopause
      },
      instruments: {
        plasma: {
          epochs: plasmaEpochs,
          values: plasmaValues,
          interpolate: function(jd: number) {
            // Simple linear interpolation
            let idx = 0;
            while (idx < this.epochs.length - 1 && this.epochs[idx + 1] < jd) {
              idx++;
            }
            
            if (idx === this.epochs.length - 1) {
              return this.values[idx];
            }
            
            const t = (jd - this.epochs[idx]) / (this.epochs[idx + 1] - this.epochs[idx]);
            const v1 = this.values[idx];
            const v2 = this.values[idx + 1];
            
            return {
              density: v1.density + t * (v2.density - v1.density),
              temperature: v1.temperature + t * (v2.temperature - v1.temperature),
              velocity: v1.velocity.clone().lerp(v2.velocity, t),
              pressure: v1.pressure + t * (v2.pressure - v1.pressure)
            };
          },
          getRange: function() {
            return {
              start: this.epochs[0],
              end: this.epochs[this.epochs.length - 1]
            };
          }
        } as any
      }
    };
  }
  
  /**
   * Generate trajectory data for Voyager 2
   */
  static generateVoyager2Trajectory(): SpacecraftTrajectoryData {
    const data = this.VOYAGER_2;
    const launchJD = JulianDate.fromDate(data.launch);
    const currentJD = JulianDate.fromDate(new Date());
    
    // Key trajectory points
    const keyPoints = [
      { jd: launchJD, r: 1.0, lon: 0, lat: 0 },
      { jd: JulianDate.fromDate(data.encounters[0].date), r: 5.2, lon: 180, lat: -2 },
      { jd: JulianDate.fromDate(data.encounters[1].date), r: 9.5, lon: 220, lat: -8 },
      { jd: JulianDate.fromDate(data.encounters[2].date), r: 19.2, lon: 280, lat: -20 },
      { jd: JulianDate.fromDate(data.encounters[3].date), r: 30.1, lon: 300, lat: -25 },
      { jd: JulianDate.fromDate(data.milestones.terminationShock.date), r: 83.7, lon: 310.4, lat: -31.2 },
      { jd: JulianDate.fromDate(data.milestones.heliopause.date), r: 119.0, lon: 310.8, lat: -32.2 },
      { jd: currentJD, r: data.current.distance, lon: data.current.heliocentricLongitude, lat: data.current.heliocentricLatitude }
    ];
    
    // Build time series (similar to Voyager 1)
    const epochs: number[] = [];
    const positions: THREE.Vector3[] = [];
    const velocities: THREE.Vector3[] = [];
    
    const numPoints = 1000;
    for (let i = 0; i < numPoints; i++) {
      const t = i / (numPoints - 1);
      const jd = launchJD + t * (currentJD - launchJD);
      
      // Find surrounding key points and interpolate
      let idx = 0;
      while (idx < keyPoints.length - 1 && keyPoints[idx + 1].jd < jd) {
        idx++;
      }
      
      let r: number, lon: number, lat: number;
      
      if (idx === keyPoints.length - 1) {
        const point = keyPoints[idx];
        const yearsSince = (jd - point.jd) / 365.25;
        r = point.r + data.current.annualMotion * yearsSince;
        lon = point.lon;
        lat = point.lat;
      } else {
        const p1 = keyPoints[idx];
        const p2 = keyPoints[idx + 1];
        const frac = (jd - p1.jd) / (p2.jd - p1.jd);
        
        r = p1.r + (p2.r - p1.r) * frac;
        lon = p1.lon + (p2.lon - p1.lon) * frac;
        lat = p1.lat + (p2.lat - p1.lat) * frac;
      }
      
      // Convert to Cartesian
      const lonRad = lon * Math.PI / 180;
      const latRad = lat * Math.PI / 180;
      
      const x = r * Math.cos(latRad) * Math.cos(lonRad);
      const y = r * Math.cos(latRad) * Math.sin(lonRad);
      const z = r * Math.sin(latRad);
      
      epochs.push(jd);
      positions.push(new THREE.Vector3(x, y, z));
      
      const v = new THREE.Vector3(x, y, z).normalize().multiplyScalar(data.current.velocity);
      velocities.push(v);
    }
    
    return {
      name: 'Voyager 2',
      launch: data.launch,
      trajectory: {
        position: new Vector3TimeSeries(epochs, positions),
        velocity: new Vector3TimeSeries(epochs, velocities),
        encounters: data.encounters
      },
      milestones: {
        terminationShock: data.milestones.terminationShock,
        heliopause: data.milestones.heliopause
      },
      instruments: {}  // Simplified - no instrument data for brevity
    };
  }
  
  /**
   * Helper to get distance at a given Julian date
   */
  private static getDistanceAtJD(jd: number, keyPoints: any[]): number {
    let idx = 0;
    while (idx < keyPoints.length - 1 && keyPoints[idx + 1].jd < jd) {
      idx++;
    }
    
    if (idx === keyPoints.length - 1) {
      return keyPoints[idx].r;
    }
    
    const p1 = keyPoints[idx];
    const p2 = keyPoints[idx + 1];
    const frac = (jd - p1.jd) / (p2.jd - p1.jd);
    
    return p1.r + (p2.r - p1.r) * frac;
  }
  
  /**
   * Get current real-time position of Voyager 1
   * This would normally fetch from NASA API
   */
  static getCurrentVoyager1Position(): SpacecraftPosition {
    const now = new Date();
    const yearsSinceLaunch = (now.getTime() - this.VOYAGER_1.launch.getTime()) / (365.25 * 24 * 60 * 60 * 1000);
    
    // Estimate current position
    const distance = this.VOYAGER_1.current.distance + 
                    (now.getFullYear() - 2024) * this.VOYAGER_1.current.annualMotion;
    
    const lonRad = this.VOYAGER_1.current.heliocentricLongitude * Math.PI / 180;
    const latRad = this.VOYAGER_1.current.heliocentricLatitude * Math.PI / 180;
    
    const position = new THREE.Vector3(
      distance * Math.cos(latRad) * Math.cos(lonRad),
      distance * Math.cos(latRad) * Math.sin(lonRad),
      distance * Math.sin(latRad)
    );
    
    const velocity = position.clone().normalize().multiplyScalar(this.VOYAGER_1.current.velocity);
    
    // Earth distance approximation (would need Earth's position)
    const earthDistance = distance - 1;  // Simplified
    const lightTime = distance * 8.317;  // AU to light-minutes
    
    return {
      position,
      velocity,
      distance,
      earthDistance,
      lightTime
    };
  }
  
  /**
   * Get current real-time position of Voyager 2
   */
  static getCurrentVoyager2Position(): SpacecraftPosition {
    const now = new Date();
    const distance = this.VOYAGER_2.current.distance + 
                    (now.getFullYear() - 2024) * this.VOYAGER_2.current.annualMotion;
    
    const lonRad = this.VOYAGER_2.current.heliocentricLongitude * Math.PI / 180;
    const latRad = this.VOYAGER_2.current.heliocentricLatitude * Math.PI / 180;
    
    const position = new THREE.Vector3(
      distance * Math.cos(latRad) * Math.cos(lonRad),
      distance * Math.cos(latRad) * Math.sin(lonRad),
      distance * Math.sin(latRad)
    );
    
    const velocity = position.clone().normalize().multiplyScalar(this.VOYAGER_2.current.velocity);
    const earthDistance = distance - 1;
    const lightTime = distance * 8.317;
    
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
 * Other notable spacecraft trajectories
 */
export class OtherSpacecraft {
  /**
   * New Horizons data
   */
  static readonly NEW_HORIZONS = {
    launch: new Date('2006-01-19T19:00:00Z'),
    encounters: [
      { planet: 'Jupiter', date: new Date('2007-02-28'), distance: 2300000, velocity: 23 },
      { planet: 'Pluto', date: new Date('2015-07-14'), distance: 12500, velocity: 14 }
    ],
    current: {
      distance: 59.0,  // AU (as of 2024)
      velocity: 14.0,  // km/s
      heliocentricLongitude: 75,
      heliocentricLatitude: -2.5,
      annualMotion: 2.97
    }
  };
  
  /**
   * Pioneer 10 data (signal lost 2003)
   */
  static readonly PIONEER_10 = {
    launch: new Date('1972-03-02T22:49:00Z'),
    encounters: [
      { planet: 'Jupiter', date: new Date('1973-12-03'), distance: 130000, velocity: 32 }
    ],
    lastContact: new Date('2003-01-23'),
    lastKnown: {
      distance: 80.0,  // AU at last contact
      velocity: 12.0,  // km/s
      heliocentricLongitude: 75,
      heliocentricLatitude: 3
    }
  };
  
  /**
   * Pioneer 11 data (signal lost 1995)
   */
  static readonly PIONEER_11 = {
    launch: new Date('1973-04-06T02:11:00Z'),
    encounters: [
      { planet: 'Jupiter', date: new Date('1974-12-02'), distance: 43000, velocity: 48 },
      { planet: 'Saturn', date: new Date('1979-09-01'), distance: 21000, velocity: 35 }
    ],
    lastContact: new Date('1995-09-30'),
    lastKnown: {
      distance: 43.4,  // AU at last contact
      velocity: 11.4,  // km/s
      heliocentricLongitude: 345,
      heliocentricLatitude: 15
    }
  };
}
