/**
 * Scientific validation tests for heliosphere simulation
 * Verifies accuracy against known measurements and observations
 */

import { getAstronomicalDataService } from '../services/AstronomicalDataService';
import { JulianDate } from '../data/AstronomicalDataStore';
import { VoyagerTrajectories } from '../physics/SpacecraftTrajectories';
import { PlanetaryEphemeris } from '../data/PlanetaryEphemeris';
import { CoordinateTransforms } from '../physics/CoordinateTransforms';

/**
 * Test result structure
 */
interface TestResult {
  name: string;
  passed: boolean;
  expected: any;
  actual: any;
  error?: number;
  tolerance: number;
  message?: string;
}

/**
 * Run all validation tests
 */
export async function runValidationTests(): Promise<TestResult[]> {
  const results: TestResult[] = [];
  const dataService = getAstronomicalDataService();
  await dataService.initialize();
  
  // 1. Voyager crossing validations
  results.push(...validateVoyagerCrossings());
  
  // 2. Planetary position validations
  results.push(...validatePlanetaryPositions());
  
  // 3. Heliosphere shape validations
  results.push(...validateHeliosphereShape());
  
  // 4. Solar wind physics validations
  results.push(...validateSolarWindPhysics());
  
  // 5. Coordinate transform validations
  results.push(...validateCoordinateTransforms());
  
  // 6. Time system validations
  results.push(...validateTimeSystems());
  
  return results;
}

/**
 * Validate Voyager crossing dates and distances
 */
function validateVoyagerCrossings(): TestResult[] {
  const results: TestResult[] = [];
  
  // Voyager 1 Termination Shock
  results.push({
    name: 'Voyager 1 Termination Shock Distance',
    expected: 94.01,
    actual: VoyagerTrajectories.VOYAGER_1.milestones.terminationShock.distance,
    passed: Math.abs(94.01 - VoyagerTrajectories.VOYAGER_1.milestones.terminationShock.distance) < 0.1,
    tolerance: 0.1,
    error: Math.abs(94.01 - VoyagerTrajectories.VOYAGER_1.milestones.terminationShock.distance)
  });
  
  results.push({
    name: 'Voyager 1 Termination Shock Date',
    expected: '2004-12-16',
    actual: VoyagerTrajectories.VOYAGER_1.milestones.terminationShock.date.toISOString().split('T')[0],
    passed: VoyagerTrajectories.VOYAGER_1.milestones.terminationShock.date.toISOString().split('T')[0] === '2004-12-16',
    tolerance: 0
  });
  
  // Voyager 1 Heliopause
  results.push({
    name: 'Voyager 1 Heliopause Distance',
    expected: 121.6,
    actual: VoyagerTrajectories.VOYAGER_1.milestones.heliopause.distance,
    passed: Math.abs(121.6 - VoyagerTrajectories.VOYAGER_1.milestones.heliopause.distance) < 0.1,
    tolerance: 0.1,
    error: Math.abs(121.6 - VoyagerTrajectories.VOYAGER_1.milestones.heliopause.distance)
  });
  
  results.push({
    name: 'Voyager 1 Heliopause Date',
    expected: '2012-08-25',
    actual: VoyagerTrajectories.VOYAGER_1.milestones.heliopause.date.toISOString().split('T')[0],
    passed: VoyagerTrajectories.VOYAGER_1.milestones.heliopause.date.toISOString().split('T')[0] === '2012-08-25',
    tolerance: 0
  });
  
  // Voyager 2 Termination Shock
  results.push({
    name: 'Voyager 2 Termination Shock Distance',
    expected: 83.7,
    actual: VoyagerTrajectories.VOYAGER_2.milestones.terminationShock.distance,
    passed: Math.abs(83.7 - VoyagerTrajectories.VOYAGER_2.milestones.terminationShock.distance) < 0.1,
    tolerance: 0.1,
    error: Math.abs(83.7 - VoyagerTrajectories.VOYAGER_2.milestones.terminationShock.distance)
  });
  
  results.push({
    name: 'Voyager 2 Heliopause Distance',
    expected: 119.0,
    actual: VoyagerTrajectories.VOYAGER_2.milestones.heliopause.distance,
    passed: Math.abs(119.0 - VoyagerTrajectories.VOYAGER_2.milestones.heliopause.distance) < 0.1,
    tolerance: 0.1,
    error: Math.abs(119.0 - VoyagerTrajectories.VOYAGER_2.milestones.heliopause.distance)
  });
  
  // Current positions (as of 2024)
  const v1Current = VoyagerTrajectories.getCurrentVoyager1Position();
  results.push({
    name: 'Voyager 1 Current Distance (2024)',
    expected: 163.0,
    actual: v1Current.distance,
    passed: Math.abs(163.0 - v1Current.distance) < 5,
    tolerance: 5,
    error: Math.abs(163.0 - v1Current.distance),
    message: 'Distance should be approximately 163 AU in 2024'
  });
  
  const v2Current = VoyagerTrajectories.getCurrentVoyager2Position();
  results.push({
    name: 'Voyager 2 Current Distance (2024)',
    expected: 136.0,
    actual: v2Current.distance,
    passed: Math.abs(136.0 - v2Current.distance) < 5,
    tolerance: 5,
    error: Math.abs(136.0 - v2Current.distance),
    message: 'Distance should be approximately 136 AU in 2024'
  });
  
  return results;
}

/**
 * Validate planetary positions against known ephemerides
 */
function validatePlanetaryPositions(): TestResult[] {
  const results: TestResult[] = [];
  
  // Test Earth position at J2000 epoch
  const j2000 = 2451545.0;
  const earth = PlanetaryEphemeris.calculatePosition('Earth', j2000);
  
  results.push({
    name: 'Earth Semi-major Axis at J2000',
    expected: 1.00000261,
    actual: earth.elements.a,
    passed: Math.abs(1.00000261 - earth.elements.a) < 0.00001,
    tolerance: 0.00001,
    error: Math.abs(1.00000261 - earth.elements.a)
  });
  
  // Test Jupiter orbital period
  results.push({
    name: 'Jupiter Orbital Period',
    expected: 11.862615,
    actual: PlanetaryEphemeris.getElements('Jupiter', j2000).period,
    passed: Math.abs(11.862615 - PlanetaryEphemeris.getElements('Jupiter', j2000).period) < 0.001,
    tolerance: 0.001,
    error: Math.abs(11.862615 - PlanetaryEphemeris.getElements('Jupiter', j2000).period)
  });
  
  // Test Pluto's eccentric orbit
  const pluto = PlanetaryEphemeris.getElements('Pluto', j2000);
  results.push({
    name: 'Pluto Eccentricity',
    expected: 0.2488273,
    actual: pluto.e,
    passed: Math.abs(0.2488273 - pluto.e) < 0.0001,
    tolerance: 0.0001,
    error: Math.abs(0.2488273 - pluto.e)
  });
  
  results.push({
    name: 'Pluto Inclination',
    expected: 17.14001206,
    actual: pluto.i * 180 / Math.PI,
    passed: Math.abs(17.14001206 - pluto.i * 180 / Math.PI) < 0.01,
    tolerance: 0.01,
    error: Math.abs(17.14001206 - pluto.i * 180 / Math.PI),
    message: 'Pluto has high inclination compared to other planets'
  });
  
  return results;
}

/**
 * Validate heliosphere shape parameters
 */
function validateHeliosphereShape(): TestResult[] {
  const results: TestResult[] = [];
  const dataService = getAstronomicalDataService();
  const heliosphereModel = dataService.getHeliosphereModel();
  
  // Test at J2000 epoch
  const j2000 = 2451545.0;
  
  // Nose direction (upstream)
  const noseBoundary = heliosphereModel.calculateBoundary(Math.PI/2, Math.PI, j2000);
  results.push({
    name: 'Termination Shock at Nose',
    expected: 94,
    actual: noseBoundary.terminationShock,
    passed: Math.abs(94 - noseBoundary.terminationShock) < 5,
    tolerance: 5,
    error: Math.abs(94 - noseBoundary.terminationShock),
    message: 'Should be compressed in nose direction'
  });
  
  results.push({
    name: 'Heliopause at Nose',
    expected: 121,
    actual: noseBoundary.heliopause,
    passed: Math.abs(121 - noseBoundary.heliopause) < 5,
    tolerance: 5,
    error: Math.abs(121 - noseBoundary.heliopause)
  });
  
  // Tail direction (downstream)
  const tailBoundary = heliosphereModel.calculateBoundary(Math.PI/2, 0, j2000);
  results.push({
    name: 'Termination Shock at Tail',
    expected: 200,
    actual: tailBoundary.terminationShock,
    passed: tailBoundary.terminationShock > 150 && tailBoundary.terminationShock < 250,
    tolerance: 50,
    error: Math.abs(200 - tailBoundary.terminationShock),
    message: 'Should be extended in tail direction'
  });
  
  results.push({
    name: 'Heliopause at Tail',
    expected: 300,
    actual: tailBoundary.heliopause,
    passed: tailBoundary.heliopause > 250 && tailBoundary.heliopause < 400,
    tolerance: 50,
    error: Math.abs(300 - tailBoundary.heliopause),
    message: 'Tail should extend to 300+ AU'
  });
  
  // Bow shock (controversial)
  results.push({
    name: 'Bow Shock Existence',
    expected: 'undefined or >200 AU',
    actual: noseBoundary.bowShock || 'undefined',
    passed: noseBoundary.bowShock === undefined || noseBoundary.bowShock > 200,
    tolerance: 0,
    message: 'McComas et al. 2012 suggests no bow shock'
  });
  
  return results;
}

/**
 * Validate solar wind physics
 */
function validateSolarWindPhysics(): TestResult[] {
  const results: TestResult[] = [];
  const dataService = getAstronomicalDataService();
  
  // Test solar wind at 1 AU
  const swAt1AU = dataService.getSolarWindConditions(new Date('2024-01-01'), 1);
  
  results.push({
    name: 'Solar Wind Speed at 1 AU',
    expected: 400,
    actual: swAt1AU.speed,
    passed: swAt1AU.speed > 250 && swAt1AU.speed < 800,
    tolerance: 200,
    error: Math.abs(400 - swAt1AU.speed),
    message: 'Typical range 250-800 km/s'
  });
  
  results.push({
    name: 'Solar Wind Density at 1 AU',
    expected: 5,
    actual: swAt1AU.density,
    passed: swAt1AU.density > 1 && swAt1AU.density < 20,
    tolerance: 15,
    error: Math.abs(5 - swAt1AU.density),
    message: 'Typical range 1-20 particles/cm³'
  });
  
  results.push({
    name: 'Solar Wind Temperature at 1 AU',
    expected: 1.2e5,
    actual: swAt1AU.temperature,
    passed: swAt1AU.temperature > 5e4 && swAt1AU.temperature < 2e5,
    tolerance: 1e5,
    error: Math.abs(1.2e5 - swAt1AU.temperature),
    message: 'Typical range 50,000-200,000 K'
  });
  
  results.push({
    name: 'Magnetic Field at 1 AU',
    expected: 5,
    actual: swAt1AU.magneticField.length(),
    passed: swAt1AU.magneticField.length() > 2 && swAt1AU.magneticField.length() < 10,
    tolerance: 5,
    error: Math.abs(5 - swAt1AU.magneticField.length()),
    message: 'Typical range 2-10 nT'
  });
  
  // Test density scaling with distance
  const swAt10AU = dataService.getSolarWindConditions(new Date('2024-01-01'), 10);
  const densityRatio = swAt10AU.density / swAt1AU.density;
  const expectedRatio = 1 / (10 * 10); // n ∝ r⁻²
  
  results.push({
    name: 'Solar Wind Density Scaling (r⁻²)',
    expected: expectedRatio,
    actual: densityRatio,
    passed: Math.abs(densityRatio - expectedRatio) < 0.01,
    tolerance: 0.01,
    error: Math.abs(densityRatio - expectedRatio),
    message: 'Density should scale as r⁻²'
  });
  
  return results;
}

/**
 * Validate coordinate transformations
 */
function validateCoordinateTransforms(): TestResult[] {
  const results: TestResult[] = [];
  
  // Test ecliptic to galactic transformation
  // Galactic center is at l=0°, b=0°, which corresponds to
  // RA=17h45m37s, Dec=-28°56'10" (J2000)
  const gcRA = 17.76028 * 15; // Convert hours to degrees
  const gcDec = -28.93611;
  
  const gcEcliptic = CoordinateTransforms.icrsToEcliptic(gcRA, gcDec);
  const gcGalactic = CoordinateTransforms.eclipticToGalactic(gcEcliptic);
  
  // Convert back to spherical
  const l = Math.atan2(gcGalactic.y, gcGalactic.x) * 180 / Math.PI;
  const b = Math.asin(gcGalactic.z / gcGalactic.length()) * 180 / Math.PI;
  
  results.push({
    name: 'Galactic Center Longitude',
    expected: 0,
    actual: l,
    passed: Math.abs(l) < 1,
    tolerance: 1,
    error: Math.abs(l),
    message: 'Galactic center should be at l=0°'
  });
  
  results.push({
    name: 'Galactic Center Latitude',
    expected: 0,
    actual: b,
    passed: Math.abs(b) < 1,
    tolerance: 1,
    error: Math.abs(b),
    message: 'Galactic center should be at b=0°'
  });
  
  return results;
}

/**
 * Validate time system conversions
 */
function validateTimeSystems(): TestResult[] {
  const results: TestResult[] = [];
  
  // Test J2000.0 epoch
  const j2000Date = new Date('2000-01-01T12:00:00Z');
  const j2000JD = JulianDate.fromDate(j2000Date);
  
  results.push({
    name: 'J2000.0 Julian Date',
    expected: 2451545.0,
    actual: j2000JD,
    passed: Math.abs(2451545.0 - j2000JD) < 0.001,
    tolerance: 0.001,
    error: Math.abs(2451545.0 - j2000JD)
  });
  
  // Test round-trip conversion
  const testDate = new Date('2024-11-07T15:30:00Z');
  const jd = JulianDate.fromDate(testDate);
  const backDate = JulianDate.toDate(jd);
  
  results.push({
    name: 'Julian Date Round-trip Conversion',
    expected: testDate.getTime(),
    actual: backDate.getTime(),
    passed: Math.abs(testDate.getTime() - backDate.getTime()) < 1000, // Within 1 second
    tolerance: 1000,
    error: Math.abs(testDate.getTime() - backDate.getTime()),
    message: 'Date conversion should be accurate to within 1 second'
  });
  
  return results;
}

/**
 * Format test results for display
 */
export function formatTestResults(results: TestResult[]): string {
  const passed = results.filter(r => r.passed).length;
  const total = results.length;
  const percentage = (passed / total * 100).toFixed(1);
  
  let output = `\n====================================\n`;
  output += `HELIOSPHERE SIMULATION VALIDATION\n`;
  output += `====================================\n\n`;
  output += `Overall: ${passed}/${total} tests passed (${percentage}%)\n\n`;
  
  // Group results by category
  const categories = {
    'Voyager': results.filter(r => r.name.includes('Voyager')),
    'Planetary': results.filter(r => r.name.includes('Planet') || r.name.includes('Pluto') || r.name.includes('Jupiter') || r.name.includes('Earth')),
    'Heliosphere': results.filter(r => r.name.includes('Heliosphere') || r.name.includes('Shock') || r.name.includes('Nose') || r.name.includes('Tail')),
    'Solar Wind': results.filter(r => r.name.includes('Solar Wind') || r.name.includes('Magnetic')),
    'Coordinates': results.filter(r => r.name.includes('Galactic') || r.name.includes('Coordinate')),
    'Time': results.filter(r => r.name.includes('Julian') || r.name.includes('Time'))
  };
  
  for (const [category, tests] of Object.entries(categories)) {
    if (tests.length === 0) continue;
    
    output += `\n${category} Tests:\n`;
    output += `${'─'.repeat(40)}\n`;
    
    for (const test of tests) {
      const status = test.passed ? '✓' : '✗';
      const color = test.passed ? '\x1b[32m' : '\x1b[31m';
      const reset = '\x1b[0m';
      
      output += `${color}${status}${reset} ${test.name}\n`;
      output += `  Expected: ${test.expected}`;
      if (test.tolerance > 0) output += ` ± ${test.tolerance}`;
      output += `\n`;
      output += `  Actual: ${test.actual}`;
      if (test.error !== undefined) output += ` (error: ${test.error.toFixed(6)})`;
      output += `\n`;
      if (test.message) output += `  Note: ${test.message}\n`;
      output += `\n`;
    }
  }
  
  // Summary of failures
  const failures = results.filter(r => !r.passed);
  if (failures.length > 0) {
    output += `\nFAILED TESTS:\n`;
    output += `${'─'.repeat(40)}\n`;
    for (const failure of failures) {
      output += `- ${failure.name}\n`;
    }
  }
  
  return output;
}

/**
 * Run validation and log results
 */
export async function runAndLogValidation(): Promise<boolean> {
  console.log('Running validation tests...');
  const results = await runValidationTests();
  console.log(formatTestResults(results));
  
  const allPassed = results.every(r => r.passed);
  return allPassed;
}
