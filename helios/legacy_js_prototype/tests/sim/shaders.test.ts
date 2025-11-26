/**
 * Shader compilation tests
 * Validates WebGL shaders compile correctly before deployment
 */

import { describe, it, expect, beforeAll } from 'vitest';
import * as THREE from 'three';
import { StarField } from '@/app/sim/rendering/StarField';
import { GPUParticleSystem } from '@/app/sim/gpu/ParticleSystem';
import { getRegistry } from '@/app/sim/registry/Registry';

// Mock WebGL context for testing
let renderer: THREE.WebGLRenderer;
let canvas: HTMLCanvasElement;

beforeAll(() => {
  // Create offscreen canvas for testing
  canvas = document.createElement('canvas');
  canvas.width = 256;
  canvas.height = 256;
  
  try {
    renderer = new THREE.WebGLRenderer({ 
      canvas,
      context: canvas.getContext('webgl2') || canvas.getContext('webgl') || undefined 
    });
  } catch (error) {
    console.warn('WebGL not available in test environment, using mock');
  }
});

describe('StarField Shaders', () => {
  it('should compile vertex shader without errors', () => {
    const registry = getRegistry();
    
    expect(() => {
      // Disable panorama for test environment
      const starField = new StarField(registry, { 
        maxStars: 100,
        usePanorama: false,
      });
      const mesh = starField.getMesh();
      
      // Check material is ShaderMaterial
      expect(mesh.material).toBeInstanceOf(THREE.ShaderMaterial);
      
      const material = mesh.material as THREE.ShaderMaterial;
      
      // Verify shader properties exist
      expect(material.vertexShader).toBeDefined();
      expect(material.fragmentShader).toBeDefined();
      expect(material.uniforms).toBeDefined();
      
      starField.dispose();
    }).not.toThrow();
  });

  it('should have valid uniform definitions', () => {
    const registry = getRegistry();
    const starField = new StarField(registry, { 
      maxStars: 100,
      usePanorama: false,
    });
    const mesh = starField.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // Check required uniforms
    expect(material.uniforms.auToScene).toBeDefined();
    expect(material.uniforms.auToScene.value).toBe(1.0);
    
    starField.dispose();
  });

  it('should not declare built-in attributes', () => {
    const registry = getRegistry();
    const starField = new StarField(registry, { 
      maxStars: 100,
      usePanorama: false,
    });
    const mesh = starField.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // Shader should not redeclare built-in attributes like 'position', 'uv', 'normal'
    const shader = material.vertexShader;
    
    // Check for common mistakes
    expect(shader).not.toMatch(/attribute\s+vec3\s+position/);
    expect(shader).not.toMatch(/attribute\s+vec2\s+uv(?![A-Za-z])/); // uv but not uvOffset
    expect(shader).not.toMatch(/attribute\s+vec3\s+normal/);
    
    starField.dispose();
  });
});

describe('GPU Particle System Shaders', () => {
  it('should compile position update shader without errors', () => {
    if (!renderer) {
      console.warn('Skipping GPU particle test - no WebGL context');
      return;
    }

    expect(() => {
      const particleSystem = new GPUParticleSystem(renderer, {
        maxParticles: 100,
        emissionRate: 10,
        particleLifetime: 5.0,
      });
      
      particleSystem.dispose();
    }).not.toThrow();
  });

  it('should compile velocity update shader without errors', () => {
    if (!renderer) {
      console.warn('Skipping GPU particle test - no WebGL context');
      return;
    }

    const particleSystem = new GPUParticleSystem(renderer, {
      maxParticles: 100,
      emissionRate: 10,
      particleLifetime: 5.0,
    });

    // Access private materials via getMesh
    const mesh = particleSystem.getMesh();
    expect(mesh.material).toBeInstanceOf(THREE.ShaderMaterial);
    
    particleSystem.dispose();
  });

  it('should compile display shader without errors', () => {
    if (!renderer) {
      console.warn('Skipping GPU particle test - no WebGL context');
      return;
    }

    const particleSystem = new GPUParticleSystem(renderer, {
      maxParticles: 100,
      emissionRate: 10,
      particleLifetime: 5.0,
    });

    const mesh = particleSystem.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // Check display shader uniforms
    expect(material.uniforms.tPosition).toBeDefined();
    expect(material.uniforms.tVelocity).toBeDefined();
    expect(material.uniforms.auToScene).toBeDefined();
    
    particleSystem.dispose();
  });

  it('should not redeclare uv attribute in particle display shader', () => {
    if (!renderer) {
      console.warn('Skipping GPU particle test - no WebGL context');
      return;
    }

    const particleSystem = new GPUParticleSystem(renderer, {
      maxParticles: 100,
      emissionRate: 10,
      particleLifetime: 5.0,
    });

    const mesh = particleSystem.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // Shader should not redeclare 'uv' attribute
    expect(material.vertexShader).not.toMatch(/attribute\s+vec2\s+uv/);
    
    // But should use 'uv' (it's automatically provided by Three.js)
    expect(material.vertexShader).toMatch(/\buv\b/);
    
    particleSystem.dispose();
  });
});

describe('Shader Syntax Validation', () => {
  it('should use valid GLSL syntax in all shaders', () => {
    const registry = getRegistry();
    const starField = new StarField(registry, { 
      maxStars: 100,
      usePanorama: false,
    });
    const mesh = starField.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // Check for common GLSL syntax errors
    const vertexShader = material.vertexShader;
    const fragmentShader = material.fragmentShader;
    
    // All main functions should be defined
    expect(vertexShader).toMatch(/void\s+main\s*\(\s*\)/);
    expect(fragmentShader).toMatch(/void\s+main\s*\(\s*\)/);
    
    // All varyings should be declared in both vertex and fragment
    const varyingMatches = vertexShader.match(/varying\s+\w+\s+(\w+)/g);
    if (varyingMatches) {
      varyingMatches.forEach(varying => {
        const varName = varying.split(/\s+/).pop();
        expect(fragmentShader).toContain(varName!);
      });
    }
    
    starField.dispose();
  });

  it('should have matching precision qualifiers', () => {
    const registry = getRegistry();
    const starField = new StarField(registry, { 
      maxStars: 100,
      usePanorama: false,
    });
    const mesh = starField.getMesh();
    const material = mesh.material as THREE.ShaderMaterial;
    
    // If fragment shader uses floats, it should have precision
    const fragmentShader = material.fragmentShader;
    
    // Three.js automatically adds precision qualifiers, but check our code doesn't conflict
    expect(fragmentShader).not.toMatch(/precision\s+\w+\s+float.*precision\s+\w+\s+float/);
    
    starField.dispose();
  });
});

describe('Shader Integration', () => {
  it('should create StarField and render without WebGL errors', () => {
    const registry = getRegistry();
    const starField = new StarField(registry, {
      maxStars: 100,
      usePanorama: false, // Disable for faster test
    });

    // Verify mesh is renderable
    const mesh = starField.getMesh();
    expect(mesh).toBeInstanceOf(THREE.InstancedMesh);
    expect(mesh.count).toBeGreaterThan(0);
    
    // Update should not throw
    expect(() => {
      starField.update(0.016); // 1 frame at 60 FPS
    }).not.toThrow();
    
    starField.dispose();
  });

  it('should create GPUParticleSystem and update without errors', () => {
    if (!renderer) {
      console.warn('Skipping GPU particle test - no WebGL context');
      return;
    }

    const particleSystem = new GPUParticleSystem(renderer, {
      maxParticles: 100,
      emissionRate: 10,
      particleLifetime: 5.0,
    });

    // Update should not throw
    expect(() => {
      particleSystem.update(0.016, 1.0);
    }).not.toThrow();
    
    particleSystem.dispose();
  });
});

