import { describe, expect, it } from 'vitest';
import {
  SCENE_SCALE,
  ORBITAL_DISTANCES_AU,
  ORBITAL_PERIODS_YEARS,
  MOON_ORBIT_KM,
  MOON_PERIOD_DAYS,
  MOON_PERIOD_YEARS,
  auToSceneUnits,
  kmToSceneUnits,
  orbitalRadiusToScene,
  calculatePlanetAngle
} from '@/app/lib/scaling';

/**
 * Test-Driven Development: Orbital Mechanics
 * These tests define the CORRECT behavior for orbital calculations
 */

describe('Orbital Mechanics', () => {
  describe('Orbital distances', () => {
    it('converts Mercury orbit (0.387 AU) to scene units', () => {
      const mercuryScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Mercury);
      expect(mercuryScene).toBeCloseTo(0.01161, 5);
    });

    it('converts Earth orbit (1.0 AU) to scene units', () => {
      const earthScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Earth);
      expect(earthScene).toBe(0.03);
    });

    it('converts Pluto orbit (39.482 AU) to scene units', () => {
      const plutoScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Pluto);
      expect(plutoScene).toBeCloseTo(1.18446, 5);
    });

    it('maintains correct relative orbital distances', () => {
      const earthScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Earth);
      const marsScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Mars);
      const jupiterScene = orbitalRadiusToScene(ORBITAL_DISTANCES_AU.Jupiter);
      
      // Mars should be 1.524x Earth distance
      expect(marsScene / earthScene).toBeCloseTo(1.524, 3);
      
      // Jupiter should be 5.203x Earth distance
      expect(jupiterScene / earthScene).toBeCloseTo(5.203, 3);
    });
  });

  describe('Orbital periods', () => {
    it('defines correct orbital periods', () => {
      expect(ORBITAL_PERIODS_YEARS.Earth).toBe(1.0);
      expect(ORBITAL_PERIODS_YEARS.Jupiter).toBeCloseTo(11.86, 2);
      expect(ORBITAL_PERIODS_YEARS.Neptune).toBeCloseTo(164.8, 1);
    });

    it('calculates planet angle from year correctly', () => {
      // Earth at year 0.5 should be at π (opposite side of orbit)
      const earthAngle = calculatePlanetAngle(0.5, ORBITAL_PERIODS_YEARS.Earth);
      expect(earthAngle).toBeCloseTo(Math.PI, 5);

      // Earth at year 1.0 should be back at 0 (full orbit)
      const earthAngleFull = calculatePlanetAngle(1.0, ORBITAL_PERIODS_YEARS.Earth);
      expect(earthAngleFull).toBeCloseTo(0, 5);
    });

    it('handles negative years correctly', () => {
      // -0.5 years should be same as +0.5 years
      const negativeAngle = calculatePlanetAngle(-0.5, ORBITAL_PERIODS_YEARS.Earth);
      const positiveAngle = calculatePlanetAngle(0.5, ORBITAL_PERIODS_YEARS.Earth);
      expect(negativeAngle).toBeCloseTo(positiveAngle, 5);
    });
  });

  describe('Moon orbit', () => {
    it('converts moon orbital radius from km to scene units', () => {
      const moonOrbitScene = kmToSceneUnits(MOON_ORBIT_KM);
      expect(moonOrbitScene).toBeCloseTo(0.0000771, 7);
    });

    it('calculates moon period in years correctly', () => {
      expect(MOON_PERIOD_YEARS).toBeCloseTo(0.0748, 4);
    });
  });

  describe('Planet position calculations', () => {
    it('calculates planet position from angle and radius', () => {
      const radius = orbitalRadiusToScene(1.0); // Earth orbit
      const angle = Math.PI / 2; // 90 degrees
      
      const x = Math.cos(angle) * radius;
      const z = Math.sin(angle) * radius;
      
      expect(x).toBeCloseTo(0, 5);
      expect(z).toBeCloseTo(0.03, 5);
    });
  });
});

