import { describe, expect, it, vi, beforeEach } from 'vitest';
import { HeliosphereModel } from '../app/lib/physics/HeliosphereModel';

describe('HeliosphereModel.generateParametricSurface', () => {
  const model = new HeliosphereModel();
  const jd = 0;

  it('creates populated termination shock geometry', () => {
    const geometry = model.generateParametricSurface('terminationShock', jd, 8);
    const positions = geometry.getAttribute('position');
    expect(positions).toBeDefined();
    expect(positions.count).toBeGreaterThan(0);
  });

  it('maintains geometry when bow shock distances collapse', () => {
    const geometry = model.generateParametricSurface('bowShock', jd, 8);
    const positions = geometry.getAttribute('position');
    expect(positions).toBeDefined();
    expect(positions.count).toBeGreaterThan(0);
  });

  it('handles invalid surface types gracefully', () => {
    expect(() => {
      model.generateParametricSurface('invalid' as any, jd, 8);
    }).not.toThrow();
  });

  it('handles extreme Julian dates', () => {
    const extremeJd = 1000000;
    const geometry = model.generateParametricSurface('terminationShock', extremeJd, 8);
    const positions = geometry.getAttribute('position');
    expect(positions).toBeDefined();
    expect(positions.count).toBeGreaterThan(0);
  });

  it('handles negative Julian dates', () => {
    const negativeJd = -1000;
    const geometry = model.generateParametricSurface('heliopause', negativeJd, 8);
    const positions = geometry.getAttribute('position');
    expect(positions).toBeDefined();
    expect(positions.count).toBeGreaterThan(0);
  });

  it('handles different resolution values', () => {
    const resolutions = [4, 8, 16, 32];
    resolutions.forEach(res => {
      const geometry = model.generateParametricSurface('terminationShock', jd, res);
      const positions = geometry.getAttribute('position');
      expect(positions).toBeDefined();
      expect(positions.count).toBeGreaterThan(0);
    });
  });
});
