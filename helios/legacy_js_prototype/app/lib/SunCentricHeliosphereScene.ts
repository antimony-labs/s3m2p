/**
 * Example Sun-Centric Heliosphere Scene
 * Demonstrates integration of the new sim/ architecture
 */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import {
  getRegistry,
  getDatasetLoader,
  Units,
  StarField,
  GPUParticleSystem,
  ValidationOverlays,
  ValidationTests,
} from '@/app/sim';
import { HeliosphereSurface } from '@/app/sim/physics/HeliosphereSurface';
import type { Registry, DatasetLoader } from '@/app/sim';
import type { JulianDate, MyrSinceZAMS } from '@/app/sim/types/units';

export interface SunCentricSceneAPI {
  canvas: HTMLCanvasElement;
  update: (dt: number) => void;
  resize: (w: number, h: number) => void;
  dispose: () => void;
  setTime: (julianDate: number) => Promise<void>;
  toggleValidation: (show: boolean) => void;
}

/**
 * Create Sun-centric heliosphere scene
 */
export async function createSunCentricScene(
  canvas: HTMLCanvasElement
): Promise<SunCentricSceneAPI> {
  console.log('[SunCentricScene] Initializing...');

  // ===== Three.js Setup =====
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: true,
    powerPreference: 'high-performance',
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x000000);

  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 10000);
  camera.position.set(200, 150, 250);

  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;
  controls.minDistance = 10;
  controls.maxDistance = 2000;

  // ===== Initialize Registry =====
  const registry = getRegistry({
    auToScene: 1.0, // 1 AU = 1 Three.js unit
    maxRenderDistance: Units.AU(500),
    enableGPUParticles: true,
    maxParticles: 50_000,
    maxStars: 10_000,
  });

  console.log('[SunCentricScene] Registry initialized');

  // ===== Load Dataset =====
  const loader = getDatasetLoader('/dataset');

  try {
    await loader.initialize();
    console.log('[SunCentricScene] Dataset loaded');
    
    // Get metadata to check time units
    const metadata = loader.getMetadata();
    console.log('[SunCentricScene] Dataset time units:', metadata.units.time);
    
    // If dataset uses GyrSinceZAMS, we'll convert when loading
    // Registry stays in JulianDate for consistency
    console.log('[SunCentricScene] Dataset uses time units:', metadata.units.time);
  } catch (error) {
    console.warn('[SunCentricScene] Dataset not found, using fallback parameters');
    // Will use fallback parameters in loader
  }

  // ===== Load Initial Parameters =====
  // Convert registry time to dataset time format
  const registryTime = registry.getTime();
  const metadata = loader.getMetadata();
  
  let datasetTime: JulianDate | MyrSinceZAMS | number;
  if (metadata?.units?.time === 'GyrSinceZAMS') {
    // Use present day: 4.6 Gyr since ZAMS
    datasetTime = 4.6;
  } else {
    datasetTime = registryTime;
  }
  
  const params = await loader.loadParametersAt(datasetTime);

  console.log('[SunCentricScene] Heliosphere parameters:', {
    R_HP_nose: `${params.R_HP_nose} AU`,
    morphology: params.morphology,
    R_TS_over_HP: params.R_TS_over_HP,
  });

  // ===== Create Heliosphere Surfaces =====

  // Heliopause
  const hpSurface = new HeliosphereSurface(params);
  const hpMeshData = hpSurface.generateMesh(64, 128, 'heliopause');

  const hpGeometry = new THREE.BufferGeometry();
  hpGeometry.setAttribute('position', new THREE.BufferAttribute(hpMeshData.positions, 3));
  hpGeometry.setIndex(new THREE.BufferAttribute(hpMeshData.indices, 1));
  hpGeometry.setAttribute('normal', new THREE.BufferAttribute(hpMeshData.normals, 3));

  const hpMaterial = new THREE.MeshBasicMaterial({
    color: 0x4ecdc4,
    wireframe: false,
    transparent: true,
    opacity: 0.2,
    side: THREE.DoubleSide,
  });

  const hpMesh = new THREE.Mesh(hpGeometry, hpMaterial);
  hpMesh.name = 'Heliopause';
  scene.add(hpMesh);

  // Termination Shock
  const tsMeshData = hpSurface.generateMesh(48, 96, 'termination_shock');

  const tsGeometry = new THREE.BufferGeometry();
  tsGeometry.setAttribute('position', new THREE.BufferAttribute(tsMeshData.positions, 3));
  tsGeometry.setIndex(new THREE.BufferAttribute(tsMeshData.indices, 1));
  tsGeometry.setAttribute('normal', new THREE.BufferAttribute(tsMeshData.normals, 3));

  const tsMaterial = new THREE.MeshBasicMaterial({
    color: 0xff6b6b,
    wireframe: false,
    transparent: true,
    opacity: 0.15,
    side: THREE.DoubleSide,
  });

  const tsMesh = new THREE.Mesh(tsGeometry, tsMaterial);
  tsMesh.name = 'TerminationShock';
  scene.add(tsMesh);

  console.log('[SunCentricScene] Heliosphere surfaces created');

  // ===== Create Sun =====
  const sunGeometry = new THREE.SphereGeometry(5, 32, 32);
  const sunMaterial = new THREE.MeshBasicMaterial({
    color: 0xffff00,
  });
  const sun = new THREE.Mesh(sunGeometry, sunMaterial);
  sun.name = 'Sun';
  scene.add(sun);

  // Sun glow
  const glowGeometry = new THREE.SphereGeometry(8, 32, 32);
  const glowMaterial = new THREE.MeshBasicMaterial({
    color: 0xffaa00,
    transparent: true,
    opacity: 0.3,
  });
  const glow = new THREE.Mesh(glowGeometry, glowMaterial);
  sun.add(glow);

  // ===== Create Starfield =====
  const starField = new StarField(registry, {
    maxStars: 10_000,
    nearbyRadiusAU: 6.5e6, // ~100 parsecs
    usePanorama: true,
  });

  scene.add(starField.getMesh());

  const panorama = starField.getPanorama();
  if (panorama) {
    scene.add(panorama);
  }

  console.log('[SunCentricScene] Starfield created');

  // ===== Create Particle System =====
  const particleSystem = new GPUParticleSystem(renderer, {
    maxParticles: 50_000,
    emissionRate: 1000,
    particleLifetime: 10.0,
    initialVelocity: new THREE.Vector3(1, 0, 0),
    velocitySpread: 0.5,
  });

  scene.add(particleSystem.getMesh());

  console.log('[SunCentricScene] Particle system created');

  // ===== Validation Overlays =====
  const validation = new ValidationOverlays(registry, {
    showReferenceRings: true,
    showVoyagerTracks: true,
    showApexArrow: true,
    showIBEXArrow: true,
  });

  scene.add(validation.getGroup());

  // Run validation tests
  ValidationTests.runAll(registry, params.R_HP_nose, params.R_TS_over_HP);

  console.log('[SunCentricScene] Scene initialized successfully');

  // ===== Animation State =====
  let animationTime = 0;

  // ===== API =====

  const update = (dt: number) => {
    animationTime += dt;

    // Update controls
    controls.update();

    // Update systems
    particleSystem.update(dt, registry.config.auToScene);
    starField.update(dt);
    validation.update();

    // Render
    renderer.render(scene, camera);
  };

  const resize = (width: number, height: number) => {
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
  };

  const setTime = async (julianDate: number) => {
    // Update registry time
    registry.setTime(Units.JulianDate(julianDate));

    // Load new parameters
    const newParams = await loader.loadParametersAt(registry.getTime());

    // Update heliosphere surface
    hpSurface.updateParameters(newParams);

    // Regenerate meshes
    const newHpMesh = hpSurface.generateMesh(64, 128, 'heliopause');
    const newTsMesh = hpSurface.generateMesh(48, 96, 'termination_shock');

    // Update geometry
    hpGeometry.setAttribute('position', new THREE.BufferAttribute(newHpMesh.positions, 3));
    hpGeometry.setIndex(new THREE.BufferAttribute(newHpMesh.indices, 1));
    hpGeometry.setAttribute('normal', new THREE.BufferAttribute(newHpMesh.normals, 3));
    hpGeometry.computeBoundingSphere();

    tsGeometry.setAttribute('position', new THREE.BufferAttribute(newTsMesh.positions, 3));
    tsGeometry.setIndex(new THREE.BufferAttribute(newTsMesh.indices, 1));
    tsGeometry.setAttribute('normal', new THREE.BufferAttribute(newTsMesh.normals, 3));
    tsGeometry.computeBoundingSphere();

    console.log(`[SunCentricScene] Time updated to JD ${julianDate}`);
  };

  const toggleValidation = (show: boolean) => {
    validation.getGroup().visible = show;
  };

  const dispose = () => {
    console.log('[SunCentricScene] Disposing...');

    controls.dispose();
    renderer.dispose();

    hpGeometry.dispose();
    hpMaterial.dispose();
    tsGeometry.dispose();
    tsMaterial.dispose();
    sunGeometry.dispose();
    sunMaterial.dispose();
    glowGeometry.dispose();
    glowMaterial.dispose();

    particleSystem.dispose();
    starField.dispose();
    validation.dispose();

    loader.clearCache();

    scene.clear();

    console.log('[SunCentricScene] Disposed');
  };

  return {
    canvas,
    update,
    resize,
    dispose,
    setTime,
    toggleValidation,
  };
}

/**
 * Type guard to check if scene API is initialized
 */
export function isSunCentricScene(api: any): api is SunCentricSceneAPI {
  return (
    api &&
    typeof api.update === 'function' &&
    typeof api.resize === 'function' &&
    typeof api.dispose === 'function' &&
    typeof api.setTime === 'function'
  );
}

