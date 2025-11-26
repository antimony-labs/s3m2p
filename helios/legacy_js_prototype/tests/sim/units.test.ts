/**
 * Unit tests for type-safe units
 */

import { describe, it, expect } from 'vitest';
import { Units, Convert, CONVERSIONS } from '@/app/sim/types/units';

describe('Units', () => {
  describe('Type constructors', () => {
    it('should create AU units', () => {
      const distance = Units.AU(121.6);
      expect(distance).toBe(121.6);
    });

    it('should create KmPerSec units', () => {
      const velocity = Units.KmPerSec(400);
      expect(velocity).toBe(400);
    });

    it('should create Radians units', () => {
      const angle = Units.Radians(Math.PI);
      expect(angle).toBeCloseTo(Math.PI);
    });
  });

  describe('Conversions', () => {
    it('should convert AU to km', () => {
      const au = Units.AU(1);
      const km = Convert.auToKm(au);
      expect(km).toBeCloseTo(CONVERSIONS.AU_TO_KM);
    });

    it('should convert degrees to radians', () => {
      const rad = Convert.degToRad(180);
      expect(rad).toBeCloseTo(Math.PI);
    });

    it('should convert radians to degrees', () => {
      const rad = Units.Radians(Math.PI);
      const deg = Convert.radToDeg(rad);
      expect(deg).toBeCloseTo(180);
    });

    it('should convert Date to JulianDate', () => {
      const date = new Date('2000-01-01T12:00:00Z'); // J2000 epoch
      const jd = Convert.dateToJulianDate(date);
      expect(jd).toBeCloseTo(CONVERSIONS.J2000_EPOCH, 0.1);
    });
  });
});

