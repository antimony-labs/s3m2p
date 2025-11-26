/**
 * Integration test for Sun-centric scene
 * Tests the complete scene initialization and rendering
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { JSDOM } from 'jsdom';

describe('SunCentricScene Integration', () => {
  let dom: JSDOM;
  let canvas: HTMLCanvasElement;

  beforeAll(() => {
    // Create a basic DOM environment
    dom = new JSDOM('<!DOCTYPE html><html><body><canvas></canvas></body></html>', {
      url: 'http://localhost',
      pretendToBeVisual: true,
    });

    global.document = dom.window.document as any;
    global.window = dom.window as any;
    global.HTMLCanvasElement = dom.window.HTMLCanvasElement as any;
    
    canvas = dom.window.document.querySelector('canvas') as HTMLCanvasElement;
  });

  afterAll(() => {
    dom.window.close();
  });

  it('should import scene creation function', async () => {
    const module = await import('@/app/lib/SunCentricHeliosphereScene');
    
    expect(module.createSunCentricScene).toBeDefined();
    expect(typeof module.createSunCentricScene).toBe('function');
  });

  it('should import all required sim modules', async () => {
    const sim = await import('@/app/sim');
    
    // Check all exports
    expect(sim.Units).toBeDefined();
    expect(sim.getRegistry).toBeDefined();
    expect(sim.getDatasetLoader).toBeDefined();
    expect(sim.CoordinateTransforms).toBeDefined();
    expect(sim.Vec3Utils).toBeDefined();
    expect(sim.SoA).toBeDefined();
  });

  it('should export HeliosphereSurface from physics module', async () => {
    const { HeliosphereSurface } = await import('@/app/sim/physics/HeliosphereSurface');
    
    expect(HeliosphereSurface).toBeDefined();
    expect(typeof HeliosphereSurface).toBe('function');
  });

  it('should have valid scene API interface', async () => {
    const { isSunCentricScene } = await import('@/app/lib/SunCentricHeliosphereScene');
    
    const mockAPI = {
      canvas: canvas,
      update: () => {},
      resize: () => {},
      dispose: () => {},
      setTime: async () => {},
      toggleValidation: () => {},
    };

    expect(isSunCentricScene(mockAPI)).toBe(true);
    expect(isSunCentricScene({})).toBe(false);
  });
});

describe('Dataset Validation', () => {
  it('should have dataset files in public directory', async () => {
    const fs = await import('fs');
    const path = await import('path');
    
    const datasetPath = path.join(process.cwd(), 'public', 'dataset');
    
    // Check if dataset directory exists
    expect(fs.existsSync(datasetPath)).toBe(true);
    
    // Check for meta.json
    const metaPath = path.join(datasetPath, 'meta.json');
    expect(fs.existsSync(metaPath)).toBe(true);
    
    // Parse metadata
    const meta = JSON.parse(fs.readFileSync(metaPath, 'utf-8'));
    expect(meta.version).toBeDefined();
    expect(meta.time_axis).toBeDefined();
    expect(meta.time_axis.n_epochs).toBeGreaterThan(0);
  });

  it('should have valid epoch files', async () => {
    const fs = await import('fs');
    const path = await import('path');
    
    const epochPath = path.join(process.cwd(), 'public', 'dataset', 'heliosphere', 'epoch_000000.json');
    expect(fs.existsSync(epochPath)).toBe(true);
    
    const epoch = JSON.parse(fs.readFileSync(epochPath, 'utf-8'));
    
    // Validate structure
    expect(epoch.R_HP_nose).toBeDefined();
    expect(epoch.R_TS_over_HP).toBeDefined();
    expect(epoch.nose_vec).toBeDefined();
    expect(epoch.nose_vec).toHaveLength(3);
    expect(epoch.morphology).toBeDefined();
    expect(epoch.shape_params).toBeDefined();
  });
});

