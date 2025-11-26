/**
 * Type-safe units for heliosphere simulation
 * All measurements use branded types to prevent accidental mixing
 */

// Branded type helper
type Brand<K, T> = K & { __brand: T };

/**
 * Distance in Astronomical Units (AU)
 * 1 AU ≈ 149,597,870.7 km (Earth-Sun distance)
 */
export type AU = Brand<number, 'AU'>;

/**
 * Velocity in kilometers per second
 */
export type KmPerSec = Brand<number, 'KmPerSec'>;

/**
 * Angle in radians
 */
export type Radians = Brand<number, 'Radians'>;

/**
 * Time in Julian Date
 * JD 0.0 = noon on January 1, 4713 BC
 */
export type JulianDate = Brand<number, 'JulianDate'>;

/**
 * Time in millions of years since Zero Age Main Sequence (ZAMS)
 */
export type MyrSinceZAMS = Brand<number, 'MyrSinceZAMS'>;

/**
 * Temperature in Kelvin
 */
export type Kelvin = Brand<number, 'Kelvin'>;

/**
 * Density in particles per cubic centimeter
 */
export type ParticlesPerCm3 = Brand<number, 'ParticlesPerCm3'>;

/**
 * Magnetic field strength in nanoTesla
 */
export type NanoTesla = Brand<number, 'NanoTesla'>;

/**
 * Mass loss rate in solar masses per year
 */
export type SolarMassPerYear = Brand<number, 'SolarMassPerYear'>;

/**
 * Unit constructors - use these to create typed values
 */
export const Units = {
  AU: (value: number): AU => value as AU,
  KmPerSec: (value: number): KmPerSec => value as KmPerSec,
  Radians: (value: number): Radians => value as Radians,
  JulianDate: (value: number): JulianDate => value as JulianDate,
  MyrSinceZAMS: (value: number): MyrSinceZAMS => value as MyrSinceZAMS,
  Kelvin: (value: number): Kelvin => value as Kelvin,
  ParticlesPerCm3: (value: number): ParticlesPerCm3 => value as ParticlesPerCm3,
  NanoTesla: (value: number): NanoTesla => value as NanoTesla,
  SolarMassPerYear: (value: number): SolarMassPerYear => value as SolarMassPerYear,
};

/**
 * Conversion constants
 */
export const CONVERSIONS = {
  // Distance
  AU_TO_KM: 149_597_870.7,
  AU_TO_M: 1.495_978_707e11,
  
  // Time
  SECONDS_PER_DAY: 86400,
  DAYS_PER_YEAR: 365.25,
  MYR_TO_YEARS: 1_000_000,
  
  // Angle
  DEG_TO_RAD: Math.PI / 180,
  RAD_TO_DEG: 180 / Math.PI,
  ARCSEC_TO_RAD: Math.PI / 648000,
  
  // Julian Date
  J2000_EPOCH: 2451545.0, // JD for J2000.0 epoch (2000-01-01 12:00 TT)
  UNIX_EPOCH_JD: 2440587.5, // JD for Unix epoch (1970-01-01 00:00 UTC)
  
  // Physical constants
  C_LIGHT_KM_S: 299_792.458, // Speed of light in km/s
  SOLAR_MASS_KG: 1.989e30,
  PROTON_MASS_KG: 1.672_621_9e-27,
};

/**
 * Conversion utilities
 */
export const Convert = {
  // Distance conversions
  auToKm: (au: AU): number => (au as number) * CONVERSIONS.AU_TO_KM,
  auToM: (au: AU): number => (au as number) * CONVERSIONS.AU_TO_M,
  kmToAU: (km: number): AU => Units.AU(km / CONVERSIONS.AU_TO_KM),
  
  // Time conversions
  julianDateToUnixMs: (jd: JulianDate): number => 
    ((jd as number) - CONVERSIONS.UNIX_EPOCH_JD) * CONVERSIONS.SECONDS_PER_DAY * 1000,
  
  unixMsToJulianDate: (ms: number): JulianDate =>
    Units.JulianDate(ms / 1000 / CONVERSIONS.SECONDS_PER_DAY + CONVERSIONS.UNIX_EPOCH_JD),
  
  dateToJulianDate: (date: Date): JulianDate =>
    Convert.unixMsToJulianDate(date.getTime()),
  
  julianDateToDate: (jd: JulianDate): Date =>
    new Date(Convert.julianDateToUnixMs(jd)),
  
  myrToJulianDate: (myr: MyrSinceZAMS, zamsJD: JulianDate): JulianDate =>
    Units.JulianDate((zamsJD as number) + (myr as number) * CONVERSIONS.MYR_TO_YEARS * CONVERSIONS.DAYS_PER_YEAR),
  
  // Angle conversions
  degToRad: (deg: number): Radians => Units.Radians(deg * CONVERSIONS.DEG_TO_RAD),
  radToDeg: (rad: Radians): number => (rad as number) * CONVERSIONS.RAD_TO_DEG,
};

/**
 * Extract raw numeric value from branded type (use sparingly!)
 */
export function unwrap<T>(branded: Brand<number, T>): number {
  return branded as number;
}

