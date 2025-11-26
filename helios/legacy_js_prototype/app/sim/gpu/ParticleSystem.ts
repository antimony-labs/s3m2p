/**
 * GPU-accelerated particle system
 * Uses WebGL2 ping-pong rendering for particle advection
 */

import * as THREE from 'three';
import { ParticleArrays, SoA } from '../data/StructureOfArrays';

/**
 * Particle system configuration
 */
export interface ParticleSystemConfig {
  maxParticles: number;
  emissionRate: number;  // particles per second
  particleLifetime: number;  // seconds
  initialVelocity: THREE.Vector3;
  velocitySpread: number;
  colorStart: THREE.Color;
  colorEnd: THREE.Color;
  sizeStart: number;
  sizeEnd: number;
}

/**
 * GPU particle system using ping-pong FBO technique
 */
export class GPUParticleSystem {
  private renderer: THREE.WebGLRenderer;
  private config: ParticleSystemConfig;
  
  // Ping-pong render targets
  private positionTarget0: THREE.WebGLRenderTarget;
  private positionTarget1: THREE.WebGLRenderTarget;
  private velocityTarget0: THREE.WebGLRenderTarget;
  private velocityTarget1: THREE.WebGLRenderTarget;
  
  // Compute scenes (for GPGPU)
  private positionScene: THREE.Scene;
  private velocityScene: THREE.Scene;
  private positionCamera: THREE.OrthographicCamera;
  private velocityCamera: THREE.OrthographicCamera;
  
  // Update materials
  private positionMaterial: THREE.ShaderMaterial;
  private velocityMaterial: THREE.ShaderMaterial;
  
  // Display mesh (instanced points)
  private displayMesh: THREE.Points;
  private displayGeometry: THREE.BufferGeometry;
  private displayMaterial: THREE.ShaderMaterial;
  
  // State
  private currentTarget: 0 | 1 = 0;
  private time: number = 0;
  private textureSize: number;
  
  constructor(renderer: THREE.WebGLRenderer, config: Partial<ParticleSystemConfig> = {}) {
    this.renderer = renderer;
    
    // Default config
    this.config = {
      maxParticles: 100_000,
      emissionRate: 1000,
      particleLifetime: 10.0,
      initialVelocity: new THREE.Vector3(0, 0, 0),
      velocitySpread: 1.0,
      colorStart: new THREE.Color(1, 1, 1),
      colorEnd: new THREE.Color(0.5, 0.5, 1),
      sizeStart: 1.0,
      sizeEnd: 0.1,
      ...config,
    };
    
    // Compute texture size (square texture)
    this.textureSize = Math.ceil(Math.sqrt(this.config.maxParticles));
    const actualParticles = this.textureSize ** 2;
    
    console.log(`GPU Particle System: ${actualParticles} particles (${this.textureSize}x${this.textureSize})`);
    
    // Initialize render targets
    this.positionTarget0 = this.createRenderTarget();
    this.positionTarget1 = this.createRenderTarget();
    this.velocityTarget0 = this.createRenderTarget();
    this.velocityTarget1 = this.createRenderTarget();
    
    // Initialize compute materials
    this.positionMaterial = this.createPositionUpdateMaterial();
    this.velocityMaterial = this.createVelocityUpdateMaterial();
    
    // Setup compute scenes
    this.positionCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);
    this.velocityCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);
    
    this.positionScene = new THREE.Scene();
    this.velocityScene = new THREE.Scene();
    
    const quad = new THREE.PlaneGeometry(2, 2);
    this.positionScene.add(new THREE.Mesh(quad, this.positionMaterial));
    this.velocityScene.add(new THREE.Mesh(quad, this.velocityMaterial));
    
    // Initialize display mesh
    this.displayGeometry = this.createDisplayGeometry();
    this.displayMaterial = this.createDisplayMaterial();
    this.displayMesh = new THREE.Points(this.displayGeometry, this.displayMaterial);
    
    // Initialize particle data
    this.initializeParticles();
  }
  
  /**
   * Create RGBA32F render target for particle data
   */
  private createRenderTarget(): THREE.WebGLRenderTarget {
    return new THREE.WebGLRenderTarget(this.textureSize, this.textureSize, {
      minFilter: THREE.NearestFilter,
      magFilter: THREE.NearestFilter,
      format: THREE.RGBAFormat,
      type: THREE.FloatType,
      depthBuffer: false,
      stencilBuffer: false,
    });
  }
  
  /**
   * Position update shader (advects particles)
   */
  private createPositionUpdateMaterial(): THREE.ShaderMaterial {
    return new THREE.ShaderMaterial({
      uniforms: {
        tPosition: { value: null },
        tVelocity: { value: null },
        dt: { value: 0.0 },
        time: { value: 0.0 },
      },
      vertexShader: `
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        uniform sampler2D tPosition;
        uniform sampler2D tVelocity;
        uniform float dt;
        uniform float time;
        
        varying vec2 vUv;
        
        void main() {
          vec4 posData = texture2D(tPosition, vUv);
          vec4 velData = texture2D(tVelocity, vUv);
          
          vec3 pos = posData.xyz;
          vec3 vel = velData.xyz;
          float age = velData.w;
          
          // Update position: p' = p + v * dt
          pos += vel * dt;
          
          // Update age
          age += dt;
          
          // Store mass in w component (unchanged)
          float mass = posData.w;
          
          gl_FragColor = vec4(pos, mass);
        }
      `,
    });
  }
  
  /**
   * Velocity update shader (applies forces)
   */
  private createVelocityUpdateMaterial(): THREE.ShaderMaterial {
    return new THREE.ShaderMaterial({
      uniforms: {
        tPosition: { value: null },
        tVelocity: { value: null },
        dt: { value: 0.0 },
        time: { value: 0.0 },
        // Force fields (e.g., solar wind acceleration)
        solarWindForce: { value: new THREE.Vector3(1, 0, 0) },
      },
      vertexShader: `
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        uniform sampler2D tPosition;
        uniform sampler2D tVelocity;
        uniform float dt;
        uniform float time;
        uniform vec3 solarWindForce;
        
        varying vec2 vUv;
        
        void main() {
          vec4 posData = texture2D(tPosition, vUv);
          vec4 velData = texture2D(tVelocity, vUv);
          
          vec3 pos = posData.xyz;
          vec3 vel = velData.xyz;
          float age = velData.w;
          
          // Apply force (simple solar wind radial acceleration)
          vec3 dir = normalize(pos);
          vec3 force = solarWindForce * 0.001; // Scaled
          
          // Update velocity: v' = v + F * dt
          vel += force * dt;
          
          // Damping (optional)
          vel *= 0.999;
          
          // Update age
          age += dt;
          
          gl_FragColor = vec4(vel, age);
        }
      `,
    });
  }
  
  /**
   * Create display geometry (instanced points with UV for texture lookup)
   */
  private createDisplayGeometry(): THREE.BufferGeometry {
    const geometry = new THREE.BufferGeometry();
    
    const numParticles = this.textureSize ** 2;
    const uvs = new Float32Array(numParticles * 2);
    
    // Generate UV coordinates for texture lookup
    for (let i = 0; i < this.textureSize; i++) {
      for (let j = 0; j < this.textureSize; j++) {
        const idx = (i * this.textureSize + j) * 2;
        uvs[idx + 0] = (j + 0.5) / this.textureSize;
        uvs[idx + 1] = (i + 0.5) / this.textureSize;
      }
    }
    
    geometry.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));
    geometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(numParticles * 3), 3));
    
    return geometry;
  }
  
  /**
   * Display material (samples particle data from textures)
   */
  private createDisplayMaterial(): THREE.ShaderMaterial {
    return new THREE.ShaderMaterial({
      uniforms: {
        tPosition: { value: null },
        tVelocity: { value: null },
        auToScene: { value: 1.0 },
        colorStart: { value: this.config.colorStart },
        colorEnd: { value: this.config.colorEnd },
        sizeStart: { value: this.config.sizeStart },
        sizeEnd: { value: this.config.sizeEnd },
        lifetime: { value: this.config.particleLifetime },
      },
      vertexShader: `
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
          
          // Age-based alpha and size
          float t = clamp(age / lifetime, 0.0, 1.0);
          vAlpha = 1.0 - t;
          
          float size = mix(sizeStart, sizeEnd, t);
          gl_PointSize = size;
          
          // Color (would be passed in, but using simple fade for now)
          vColor = vec4(1.0, 1.0, 1.0, vAlpha);
          
          gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
        }
      `,
      fragmentShader: `
        varying vec4 vColor;
        varying float vAlpha;
        
        void main() {
          // Circular point
          vec2 center = gl_PointCoord - vec2(0.5);
          float dist = length(center);
          if (dist > 0.5) discard;
          
          float alpha = (1.0 - dist * 2.0) * vAlpha;
          gl_FragColor = vec4(vColor.rgb, alpha);
        }
      `,
      transparent: true,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
    });
  }
  
  /**
   * Initialize particle data
   */
  private initializeParticles(): void {
    // Create initial random particle distribution
    const size = this.textureSize ** 2;
    const posData = new Float32Array(size * 4);
    const velData = new Float32Array(size * 4);
    
    for (let i = 0; i < size; i++) {
      const idx = i * 4;
      
      // Random position (sphere)
      const theta = Math.random() * Math.PI * 2;
      const phi = Math.acos(2 * Math.random() - 1);
      const r = 50 + Math.random() * 50;
      
      posData[idx + 0] = r * Math.sin(phi) * Math.cos(theta);
      posData[idx + 1] = r * Math.sin(phi) * Math.sin(theta);
      posData[idx + 2] = r * Math.cos(phi);
      posData[idx + 3] = 1.0; // mass
      
      // Random velocity
      velData[idx + 0] = (Math.random() - 0.5) * 2.0;
      velData[idx + 1] = (Math.random() - 0.5) * 2.0;
      velData[idx + 2] = (Math.random() - 0.5) * 2.0;
      velData[idx + 3] = Math.random() * this.config.particleLifetime; // age
    }
    
    // Upload to GPU
    const posTexture = new THREE.DataTexture(posData, this.textureSize, this.textureSize, THREE.RGBAFormat, THREE.FloatType);
    posTexture.needsUpdate = true;
    
    const velTexture = new THREE.DataTexture(velData, this.textureSize, this.textureSize, THREE.RGBAFormat, THREE.FloatType);
    velTexture.needsUpdate = true;
    
    // Copy to both targets (ping-pong init)
    this.renderer.setRenderTarget(this.positionTarget0);
    this.renderer.clear();
    // (Would render posTexture here using a copy material)
    
    this.renderer.setRenderTarget(this.velocityTarget0);
    this.renderer.clear();
    
    this.renderer.setRenderTarget(null);
  }
  
  /**
   * Update particles (called each frame)
   */
  update(dt: number, auToScene: number): void {
    this.time += dt;
    
    // Update uniforms
    this.positionMaterial.uniforms.dt.value = dt;
    this.positionMaterial.uniforms.time.value = this.time;
    this.velocityMaterial.uniforms.dt.value = dt;
    this.velocityMaterial.uniforms.time.value = this.time;
    
    // Ping-pong: read from current, write to next
    const readPos = this.currentTarget === 0 ? this.positionTarget0 : this.positionTarget1;
    const readVel = this.currentTarget === 0 ? this.velocityTarget0 : this.velocityTarget1;
    const writePos = this.currentTarget === 0 ? this.positionTarget1 : this.positionTarget0;
    const writeVel = this.currentTarget === 0 ? this.velocityTarget1 : this.velocityTarget0;
    
    // Update velocity
    this.velocityMaterial.uniforms.tPosition.value = readPos.texture;
    this.velocityMaterial.uniforms.tVelocity.value = readVel.texture;
    this.renderer.setRenderTarget(writeVel);
    this.renderer.render(this.velocityScene, this.velocityCamera);
    
    // Update position
    this.positionMaterial.uniforms.tPosition.value = readPos.texture;
    this.positionMaterial.uniforms.tVelocity.value = writeVel.texture;
    this.renderer.setRenderTarget(writePos);
    this.renderer.render(this.positionScene, this.positionCamera);
    
    // Restore render target
    this.renderer.setRenderTarget(null);
    
    // Swap targets
    this.currentTarget = this.currentTarget === 0 ? 1 : 0;
    
    // Update display material uniforms
    this.displayMaterial.uniforms.tPosition.value = writePos.texture;
    this.displayMaterial.uniforms.tVelocity.value = writeVel.texture;
    this.displayMaterial.uniforms.auToScene.value = auToScene;
  }
  
  /**
   * Get display mesh for rendering
   */
  getMesh(): THREE.Points {
    return this.displayMesh;
  }
  
  /**
   * Dispose resources
   */
  dispose(): void {
    this.positionTarget0.dispose();
    this.positionTarget1.dispose();
    this.velocityTarget0.dispose();
    this.velocityTarget1.dispose();
    this.positionMaterial.dispose();
    this.velocityMaterial.dispose();
    this.displayMaterial.dispose();
    this.displayGeometry.dispose();
  }
}

