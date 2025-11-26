/**
 * Vector utilities tests
 */

import { describe, it, expect } from 'vitest';
import { Vec3Utils } from '@/app/sim/types/vectors';
import { Units } from '@/app/sim/types/units';

describe('Vec3Utils', () => {
  describe('normalize', () => {
    it('should normalize a vector to unit length', () => {
      const v = { x: 3, y: 4, z: 0 };
      const normalized = Vec3Utils.normalize(v);
      
      expect(normalized.x).toBeCloseTo(0.6);
      expect(normalized.y).toBeCloseTo(0.8);
      expect(normalized.z).toBeCloseTo(0);
      
      const magnitude = Vec3Utils.magnitude(normalized);
      expect(magnitude).toBeCloseTo(1.0);
    });
  });

  describe('dot product', () => {
    it('should compute correct dot product', () => {
      const a = { x: 1, y: 2, z: 3 };
      const b = { x: 4, y: 5, z: 6 };
      
      const dot = Vec3Utils.dot(a, b);
      expect(dot).toBe(32); // 1*4 + 2*5 + 3*6
    });
  });

  describe('cross product', () => {
    it('should compute correct cross product', () => {
      const a = { x: 1, y: 0, z: 0 };
      const b = { x: 0, y: 1, z: 0 };
      
      const cross = Vec3Utils.cross(a, b);
      
      expect(cross.x).toBeCloseTo(0);
      expect(cross.y).toBeCloseTo(0);
      expect(cross.z).toBeCloseTo(1);
    });
  });

  describe('interpolation', () => {
    it('should lerp between two vectors', () => {
      const a = { x: 0, y: 0, z: 0 };
      const b = { x: 10, y: 10, z: 10 };
      
      const mid = Vec3Utils.lerp(a, b, 0.5);
      
      expect(mid.x).toBeCloseTo(5);
      expect(mid.y).toBeCloseTo(5);
      expect(mid.z).toBeCloseTo(5);
    });
  });

  describe('spherical coordinates', () => {
    it('should convert Cartesian to spherical and back', () => {
      const pos = {
        x: Units.AU(100),
        y: Units.AU(0),
        z: Units.AU(0),
      };
      
      const spherical = Vec3Utils.toSpherical(pos);
      expect(spherical.r).toBeCloseTo(100);
      
      const cartesian = Vec3Utils.fromSpherical(spherical);
      expect(cartesian.x as number).toBeCloseTo(100);
      expect(cartesian.y as number).toBeCloseTo(0);
      expect(cartesian.z as number).toBeCloseTo(0);
    });
  });
});

