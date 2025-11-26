import { describe, expect, it } from 'vitest';
import {
  AU_IN_KM,
  SCENE_SCALE,
  kmToAU,
  auToSceneUnits,
  kmToSceneUnits,
  planetVisibleSize,
  sunVisibleSize,
  moonVisibleSize,
  PLANET_VISIBILITY_SCALE,
  SUN_VISIBILITY_SCALE,
  CELESTIAL_RADII_KM
} from '@/app/lib/scaling';

/**
 * Test-Driven Development: Planet Scaling
 * These tests define the CORRECT behavior for planet and sun scaling
 * Implementation must pass these tests
 */

describe('Planet Scaling Calculations', () => {
  describe('Unit conversions', () => {
    it('converts km to AU correctly', () => {
      const earthRadiusKm = 6371;
      const earthRadiusAU = kmToAU(earthRadiusKm);
      expect(earthRadiusAU).toBeCloseTo(0.0000426, 7);
    });

    it('converts AU to scene units correctly', () => {
      const earthOrbitAU = 1.0;
      const earthOrbitScene = auToSceneUnits(earthOrbitAU);
      expect(earthOrbitScene).toBe(0.03);
    });

    it('converts planet radius from km to scene units', () => {
      const earthRadiusKm = 6371;
      const earthRadiusScene = kmToSceneUnits(earthRadiusKm);
      // Earth radius in scene units should be extremely tiny
      expect(earthRadiusScene).toBeCloseTo(0.00000128, 8);
    });
  });

  describe('Visibility scaling', () => {
    it('defines consistent visibility scale for planets', () => {
      expect(PLANET_VISIBILITY_SCALE).toBe(20000);
    });

    it('defines much smaller visibility scale for sun', () => {
      expect(SUN_VISIBILITY_SCALE).toBe(200);
    });

    it('sun scale is 100x smaller than planet scale', () => {
      expect(PLANET_VISIBILITY_SCALE / SUN_VISIBILITY_SCALE).toBe(100);
    });
  });

  describe('Relative planet sizes', () => {
    it('Jupiter is approximately 11x larger than Earth', () => {
      const ratio = CELESTIAL_RADII_KM.Jupiter / CELESTIAL_RADII_KM.Earth;
      expect(ratio).toBeCloseTo(10.97, 1);
    });

    it('Saturn is approximately 9x larger than Earth', () => {
      const ratio = CELESTIAL_RADII_KM.Saturn / CELESTIAL_RADII_KM.Earth;
      expect(ratio).toBeCloseTo(9.14, 1);
    });

    it('Mercury is approximately 0.38x size of Earth', () => {
      const ratio = CELESTIAL_RADII_KM.Mercury / CELESTIAL_RADII_KM.Earth;
      expect(ratio).toBeCloseTo(0.383, 2);
    });

    it('maintains proportional sizing with visibility scale', () => {
      const jupiterScaled = planetVisibleSize(CELESTIAL_RADII_KM.Jupiter);
      const earthScaled = planetVisibleSize(CELESTIAL_RADII_KM.Earth);
      
      // Ratio should be preserved
      expect(jupiterScaled / earthScaled).toBeCloseTo(10.97, 1);
    });
  });

  describe('Sun scaling', () => {
    it('converts sun radius to scene units', () => {
      const sunRadiusScene = kmToSceneUnits(CELESTIAL_RADII_KM.Sun);
      expect(sunRadiusScene).toBeCloseTo(0.0001395, 7);
    });

    it('applies 200x visibility scale to sun', () => {
      const sunSize = sunVisibleSize();
      expect(sunSize).toBeCloseTo(0.0279, 4);
    });

    it('sun is much smaller than Jupiter with correct scaling', () => {
      const sunSize = sunVisibleSize();
      const jupiterSize = planetVisibleSize(CELESTIAL_RADII_KM.Jupiter);
      
      // Jupiter should be larger than sun in the visualization
      expect(jupiterSize).toBeGreaterThan(sunSize);
    });
  });

  describe('Moon scaling', () => {
    it('converts moon radius to scene units', () => {
      const moonRadiusScene = kmToSceneUnits(CELESTIAL_RADII_KM.Moon);
      expect(moonRadiusScene).toBeCloseTo(0.000000348, 9);
    });

    it('applies same visibility scale as planets', () => {
      const moonSize = moonVisibleSize();
      expect(moonSize).toBeCloseTo(0.00696, 5);
    });
  });
});

