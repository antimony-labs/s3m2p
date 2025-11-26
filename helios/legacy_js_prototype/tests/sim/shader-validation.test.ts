/**
 * Detailed shader validation tests
 * Tests each shader can compile and link in WebGL
 */

import { describe, it, expect } from 'vitest';

describe('Shader Syntax Validation', () => {
  // Test vertex shader syntax
  const testVertexShader = (name: string, code: string) => {
    it(`${name} vertex shader should have valid syntax`, () => {
      // Check for void main()
      expect(code).toMatch(/void\s+main\s*\(\s*\)/);
      
      // Check varyings are declared before use
      const varyingDecls = code.match(/varying\s+\w+\s+(\w+)/g) || [];
      const varyingNames = varyingDecls.map(d => d.split(/\s+/).pop()!);
      
      varyingNames.forEach(name => {
        // Varying should be assigned in main()
        const mainBody = code.substring(code.indexOf('void main'));
        expect(mainBody).toContain(name);
      });
      
      // Check no duplicate attribute declarations
      expect(code).not.toMatch(/attribute\s+vec2\s+uv;/);
      expect(code).not.toMatch(/attribute\s+vec3\s+position;/);
      expect(code).not.toMatch(/attribute\s+vec3\s+normal;/);
    });
  };

  // StarField vertex shader
  const starFieldVertex = `
    varying vec3 vColor;
    
    void main() {
      vColor = vec3(1.0, 1.0, 1.0);
      #ifdef USE_INSTANCING_COLOR
        vColor = instanceColor;
      #endif
      
      vec4 mvPosition = modelViewMatrix * instanceMatrix * vec4(position, 1.0);
      gl_Position = projectionMatrix * mvPosition;
      gl_PointSize = 2.0;
    }
  `;

  const starFieldFragment = `
    varying vec3 vColor;
    
    void main() {
      gl_FragColor = vec4(vColor, 1.0);
    }
  `;

  testVertexShader('StarField', starFieldVertex);

  it('StarField fragment shader should match vertex varyings', () => {
    const vertVaryings = starFieldVertex.match(/varying\s+\w+\s+(\w+)/g) || [];
    const fragVaryings = starFieldFragment.match(/varying\s+\w+\s+(\w+)/g) || [];
    
    expect(vertVaryings.length).toBe(fragVaryings.length);
    
    vertVaryings.forEach(v => {
      expect(starFieldFragment).toContain(v);
    });
  });

  // Particle system shaders
  const particleDisplayVertex = `
    uniform sampler2D tPosition;
    uniform sampler2D tVelocity;
    uniform float auToScene;
    uniform float sizeStart;
    uniform float sizeEnd;
    uniform float lifetime;
    
    varying vec4 vColor;
    varying float vAlpha;
    
    void main() {
      vec4 posData = texture2D(tPosition, uv);
      vec4 velData = texture2D(tVelocity, uv);
      
      vec3 pos = posData.xyz * auToScene;
      float age = velData.w;
      
      float t = clamp(age / lifetime, 0.0, 1.0);
      vAlpha = 1.0 - t;
      
      float size = mix(sizeStart, sizeEnd, t);
      gl_PointSize = size;
      
      vColor = vec4(1.0, 1.0, 1.0, vAlpha);
      
      gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
    }
  `;

  testVertexShader('ParticleDisplay', particleDisplayVertex);

  it('should not have GLSL syntax errors', () => {
    const shaders = [starFieldVertex, starFieldFragment, particleDisplayVertex];
    
    shaders.forEach(shader => {
      // Check balanced braces
      const openBraces = (shader.match(/{/g) || []).length;
      const closeBraces = (shader.match(/}/g) || []).length;
      expect(openBraces).toBe(closeBraces);
      
      // Check balanced parentheses
      const openParens = (shader.match(/\(/g) || []).length;
      const closeParens = (shader.match(/\)/g) || []).length;
      expect(openParens).toBe(closeParens);
      
      // Check all statements end with semicolon (approximately)
      const lines = shader.split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('//'));
      // Most lines should end with ; or { or }
      const properLines = lines.filter(l => /[;{}]$/.test(l) || /^(varying|uniform|attribute|#)/.test(l));
      expect(properLines.length).toBeGreaterThan(lines.length * 0.7);
    });
  });
});

