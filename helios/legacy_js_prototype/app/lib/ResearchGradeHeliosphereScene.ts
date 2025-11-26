/**
 * Research-grade heliosphere visualization with real astronomical data
 * Integrates JPL ephemerides, Voyager trajectories, and MHD models
 */

import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { basisFromApex } from "./apex";
import { getAstronomicalDataService } from "./services/AstronomicalDataService";
import { CoordinateTransforms } from "./physics/CoordinateTransforms";
import { JulianDate } from "./data/AstronomicalDataStore";
import { PLANET_PROPERTIES } from "./data/PlanetaryEphemeris";
import { VoyagerTrajectories } from "./physics/SpacecraftTrajectories";

type Direction = 1 | -1;

export type ComponentVisibility = {
  heliosphere: boolean;
  terminationShock: boolean;
  heliopause: boolean;
  bowShock: boolean;
  solarWind: boolean;
  interstellarWind: boolean;
  planets: boolean;
  orbits: boolean;
  spacecraft: boolean;
  trajectories: boolean;
  stars: boolean;
  coordinateGrid: boolean;
  distanceMarkers: boolean;
  dataOverlay: boolean;
};

export type TimeMode = 'historical' | 'realtime' | 'prediction';

export type SceneAPI = {
  canvas: HTMLCanvasElement;
  update: (date: Date, timeSpeed: number, motionEnabled: boolean) => void;
  resize: (w: number, h: number) => void;
  dispose: () => void;
  toggleComponent: (component: keyof ComponentVisibility, visible: boolean) => void;
  getVisibility: () => ComponentVisibility;
  setTimeMode: (mode: TimeMode) => void;
  getCurrentDate: () => Date;
};

export async function createResearchGradeScene(canvas: HTMLCanvasElement): Promise<SceneAPI> {
  // Initialize data service
  const dataService = getAstronomicalDataService();
  await dataService.initialize();
  
  // Validate Voyager crossings
  const validation = dataService.validateVoyagerCrossings();
  console.log('Voyager validation:', validation);
  
  // Renderer setup
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, powerPreference: "high-performance" });
  renderer.setPixelRatio(Math.min(devicePixelRatio, 2));
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x000000);
  
  // Camera setup
  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 5000);
  camera.position.set(150, 100, 200);
  camera.lookAt(0, 0, 0);
  
  // Controls
  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;
  controls.minDistance = 10;
  controls.maxDistance = 1000;
  
  // Time management
  let currentDate = new Date();
  let timeMode: TimeMode = 'realtime';
  let timeSpeed = 1; // days per frame
  
  // Component visibility
  const visibility: ComponentVisibility = {
    heliosphere: true,
    terminationShock: true,
    heliopause: true,
    bowShock: false,
    solarWind: true,
    interstellarWind: true,
    planets: true,
    orbits: true,
    spacecraft: true,
    trajectories: true,
    stars: true,
    coordinateGrid: false,
    distanceMarkers: true,
    dataOverlay: true
  };
  
  // Fixed heliosphere basis
  const apexBasis = basisFromApex();
  
  // Scale factor: 1 AU = 1 unit in scene
  const AU_SCALE = 1;
  
  // ===== STAR FIELD (Gaia-based) =====
  const starGroup = new THREE.Group();
  starGroup.name = 'stars';
  
  // Simplified star field (full Gaia catalog would be loaded progressively)
  const starGeometry = new THREE.BufferGeometry();
  const starCount = 10000;
  const starPositions = new Float32Array(starCount * 3);
  const starColors = new Float32Array(starCount * 3);
  const starSizes = new Float32Array(starCount);
  
  for (let i = 0; i < starCount; i++) {
    // Random distribution in sphere
    const r = 500 + Math.random() * 1000;
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    
    starPositions[i * 3] = r * Math.sin(phi) * Math.cos(theta);
    starPositions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
    starPositions[i * 3 + 2] = r * Math.cos(phi);
    
    // Color based on temperature
    const temp = Math.random();
    if (temp < 0.7) {
      // Cool red stars
      starColors[i * 3] = 1.0;
      starColors[i * 3 + 1] = 0.7;
      starColors[i * 3 + 2] = 0.5;
    } else if (temp < 0.9) {
      // Sun-like stars
      starColors[i * 3] = 1.0;
      starColors[i * 3 + 1] = 1.0;
      starColors[i * 3 + 2] = 0.9;
    } else {
      // Hot blue stars
      starColors[i * 3] = 0.8;
      starColors[i * 3 + 1] = 0.9;
      starColors[i * 3 + 2] = 1.0;
    }
    
    starSizes[i] = 0.5 + Math.random() * 1.5;
  }
  
  starGeometry.setAttribute('position', new THREE.BufferAttribute(starPositions, 3));
  starGeometry.setAttribute('color', new THREE.BufferAttribute(starColors, 3));
  starGeometry.setAttribute('size', new THREE.BufferAttribute(starSizes, 1));
  
  const starMaterial = new THREE.PointsMaterial({
    size: 1,
    sizeAttenuation: true,
    vertexColors: true,
    transparent: true,
    opacity: 0.8
  });
  
  const stars = new THREE.Points(starGeometry, starMaterial);
  starGroup.add(stars);
  scene.add(starGroup);
  
  // ===== HELIOSPHERE BOUNDARIES =====
  const heliosphereGroup = new THREE.Group();
  heliosphereGroup.name = 'heliosphere';
  
  // Get heliosphere model
  const heliosphereModel = dataService.getHeliosphereModel();
  
  // Termination shock - using smooth volumetric glow with multiple gradient layers
  const terminationShockGeometry = heliosphereModel.generateParametricSurface(
    'terminationShock',
    JulianDate.fromDate(currentDate),
    64 // Higher resolution for smoother appearance
  );
  terminationShockGeometry.scale(AU_SCALE, AU_SCALE, AU_SCALE);
  
  // Create multiple gradient layers for smooth volumetric blending
  // Layer 1: Core glow (innermost, strongest)
  const tsCoreMaterial = new THREE.MeshPhysicalMaterial({
    color: 0xff8844,
    emissive: 0xff6600,
    emissiveIntensity: 1.2,
    transparent: true,
    opacity: 0.12,
    side: THREE.DoubleSide,
    roughness: 1.0,
    metalness: 0.0,
    transmission: 0.98,
    thickness: 0.15,
    ior: 1.05
  });
  const terminationShockCore = new THREE.Mesh(terminationShockGeometry, tsCoreMaterial);
  terminationShockCore.setRotationFromMatrix(apexBasis);
  terminationShockCore.name = 'terminationShockCore';
  heliosphereGroup.add(terminationShockCore);
  
  // Layer 2: Inner glow (smooth transition)
  const tsInnerGeometry = terminationShockGeometry.clone();
  tsInnerGeometry.scale(1.02, 1.02, 1.02);
  const tsInnerMaterial = new THREE.MeshPhysicalMaterial({
    color: 0xff8844,
    emissive: 0xff6600,
    emissiveIntensity: 0.9,
    transparent: true,
    opacity: 0.10,
    side: THREE.DoubleSide,
    roughness: 1.0,
    metalness: 0.0,
    transmission: 0.97,
    thickness: 0.18,
    ior: 1.05
  });
  const terminationShockInner = new THREE.Mesh(tsInnerGeometry, tsInnerMaterial);
  terminationShockInner.setRotationFromMatrix(apexBasis);
  terminationShockInner.name = 'terminationShockInner';
  heliosphereGroup.add(terminationShockInner);
  
  // Layer 3: Mid glow (smooth transition)
  const tsMidGeometry = terminationShockGeometry.clone();
  tsMidGeometry.scale(1.04, 1.04, 1.04);
  const tsMidMaterial = new THREE.MeshPhysicalMaterial({
    color: 0xff8844,
    emissive: 0xff6600,
    emissiveIntensity: 0.6,
    transparent: true,
    opacity: 0.08,
    side: THREE.DoubleSide,
    roughness: 1.0,
    metalness: 0.0,
    transmission: 0.96,
    thickness: 0.20,
    ior: 1.05
  });
  const terminationShockMid = new THREE.Mesh(tsMidGeometry, tsMidMaterial);
  terminationShockMid.setRotationFromMatrix(apexBasis);
  terminationShockMid.name = 'terminationShockMid';
  heliosphereGroup.add(terminationShockMid);
  
  // Layer 4: Outer glow (smooth fade-out)
  const tsOuterGeometry = terminationShockGeometry.clone();
  tsOuterGeometry.scale(1.06, 1.06, 1.06);
  const tsOuterMaterial = new THREE.MeshBasicMaterial({
    color: 0xff6600,
    transparent: true,
    opacity: 0.06,
    side: THREE.DoubleSide,
    blending: THREE.AdditiveBlending // Smooth additive blending
  });
  const terminationShockOuter = new THREE.Mesh(tsOuterGeometry, tsOuterMaterial);
  terminationShockOuter.setRotationFromMatrix(apexBasis);
  terminationShockOuter.name = 'terminationShockOuter';
  heliosphereGroup.add(terminationShockOuter);
  
  // Layer 5: Faint halo (very subtle outer edge)
  const tsHaloGeometry = terminationShockGeometry.clone();
  tsHaloGeometry.scale(1.08, 1.08, 1.08);
  const tsHaloMaterial = new THREE.MeshBasicMaterial({
    color: 0xff6600,
    transparent: true,
    opacity: 0.03,
    side: THREE.DoubleSide,
    blending: THREE.AdditiveBlending
  });
  const terminationShockHalo = new THREE.Mesh(tsHaloGeometry, tsHaloMaterial);
  terminationShockHalo.setRotationFromMatrix(apexBasis);
  terminationShockHalo.name = 'terminationShockHalo';
  heliosphereGroup.add(terminationShockHalo);
  
  // Heliopause
  const heliopauseGeometry = heliosphereModel.generateParametricSurface(
    'heliopause',
    JulianDate.fromDate(currentDate),
    48
  );
  heliopauseGeometry.scale(AU_SCALE, AU_SCALE, AU_SCALE);
  
  const heliopauseMaterial = new THREE.MeshPhysicalMaterial({
    color: 0x4488ff,
    transparent: true,
    opacity: 0.15,
    roughness: 0.8,
    metalness: 0.1,
    side: THREE.DoubleSide,
    transmission: 0.9,
    thickness: 1,
    ior: 1.1
  });
  
  const heliopauseMesh = new THREE.Mesh(heliopauseGeometry, heliopauseMaterial);
  heliopauseMesh.setRotationFromMatrix(apexBasis);
  heliopauseMesh.name = 'heliopause';
  heliosphereGroup.add(heliopauseMesh);
  
  // Bow shock (optional/controversial)
  const bowShockGeometry = heliosphereModel.generateParametricSurface(
    'bowShock',
    JulianDate.fromDate(currentDate),
    32
  );
  
  const bowShockPositions = bowShockGeometry.getAttribute('position');
  if (bowShockPositions && bowShockPositions.count > 0) {
    bowShockGeometry.scale(AU_SCALE, AU_SCALE, AU_SCALE);
    
    const bowShockMaterial = new THREE.MeshBasicMaterial({
      color: 0xff44ff,
      transparent: true,
      opacity: 0.1,
      wireframe: true,
      side: THREE.DoubleSide
    });
    
    const bowShockMesh = new THREE.Mesh(bowShockGeometry, bowShockMaterial);
    bowShockMesh.setRotationFromMatrix(apexBasis);
    bowShockMesh.name = 'bowShock';
    bowShockMesh.visible = visibility.bowShock;
    heliosphereGroup.add(bowShockMesh);
  }
  
  scene.add(heliosphereGroup);
  
  // ===== SOLAR SYSTEM =====
  const solarSystemGroup = new THREE.Group();
  solarSystemGroup.name = 'solarSystem';
  
  // Sun
  const sunGeometry = new THREE.SphereGeometry(0.00465 * 100, 32, 32); // Scaled up for visibility
  const sunMaterial = new THREE.MeshStandardMaterial({
    color: 0xffff00,
    emissive: 0xffff00,
    emissiveIntensity: 2
  });
  const sun = new THREE.Mesh(sunGeometry, sunMaterial);
  solarSystemGroup.add(sun);
  
  // Sun light
  const sunLight = new THREE.PointLight(0xffffff, 2, 500);
  sun.add(sunLight);
  
  // Planet groups
  const planetsGroup = new THREE.Group();
  planetsGroup.name = 'planets';
  const orbitsGroup = new THREE.Group();
  orbitsGroup.name = 'orbits';
  
  const planetMeshes: Map<string, THREE.Mesh> = new Map();
  
  // Create planets with accurate sizes (scaled for visibility)
  Object.entries(PLANET_PROPERTIES).forEach(([name, props]) => {
    // Planet mesh
    const scale = name === 'Jupiter' || name === 'Saturn' ? 0.0001 : 0.001;
    const geometry = new THREE.SphereGeometry(props.radius * scale, 32, 32);
    const material = new THREE.MeshPhongMaterial({
      color: props.color,
      shininess: 30
    });
    const planet = new THREE.Mesh(geometry, material);
    planet.name = name;
    planet.castShadow = true;
    planet.receiveShadow = true;
    planetMeshes.set(name, planet);
    planetsGroup.add(planet);
    
    // Orbit line (will be updated with real ephemeris)
    const orbitPoints: THREE.Vector3[] = [];
    const orbitGeometry = new THREE.BufferGeometry();
    const orbitMaterial = new THREE.LineBasicMaterial({
      color: props.color,
      transparent: true,
      opacity: 0.3
    });
    const orbit = new THREE.Line(orbitGeometry, orbitMaterial);
    orbit.name = name + 'Orbit';
    orbitsGroup.add(orbit);
  });
  
  solarSystemGroup.add(planetsGroup);
  solarSystemGroup.add(orbitsGroup);
  scene.add(solarSystemGroup);
  
  // ===== SPACECRAFT =====
  const spacecraftGroup = new THREE.Group();
  spacecraftGroup.name = 'spacecraft';
  
  // Voyager 1
  const voyager1Geometry = new THREE.ConeGeometry(0.5, 2, 8);
  const voyager1Material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const voyager1 = new THREE.Mesh(voyager1Geometry, voyager1Material);
  voyager1.name = 'Voyager 1';
  spacecraftGroup.add(voyager1);
  
  // Voyager 2
  const voyager2Geometry = new THREE.ConeGeometry(0.5, 2, 8);
  const voyager2Material = new THREE.MeshBasicMaterial({ color: 0x00ffff });
  const voyager2 = new THREE.Mesh(voyager2Geometry, voyager2Material);
  voyager2.name = 'Voyager 2';
  spacecraftGroup.add(voyager2);
  
  // Trajectory lines
  const trajectoriesGroup = new THREE.Group();
  trajectoriesGroup.name = 'trajectories';
  
  // Create trajectory lines (will be populated with data)
  const v1TrajGeometry = new THREE.BufferGeometry();
  const v1TrajMaterial = new THREE.LineBasicMaterial({ 
    color: 0x00ff00, 
    transparent: true, 
    opacity: 0.6 
  });
  const v1Trajectory = new THREE.Line(v1TrajGeometry, v1TrajMaterial);
  v1Trajectory.name = 'Voyager 1 Trajectory';
  trajectoriesGroup.add(v1Trajectory);
  
  const v2TrajGeometry = new THREE.BufferGeometry();
  const v2TrajMaterial = new THREE.LineBasicMaterial({ 
    color: 0x00ffff, 
    transparent: true, 
    opacity: 0.6 
  });
  const v2Trajectory = new THREE.Line(v2TrajGeometry, v2TrajMaterial);
  v2Trajectory.name = 'Voyager 2 Trajectory';
  trajectoriesGroup.add(v2Trajectory);
  
  spacecraftGroup.add(trajectoriesGroup);
  scene.add(spacecraftGroup);
  
  // ===== SOLAR WIND =====
  const solarWindGroup = new THREE.Group();
  solarWindGroup.name = 'solarWind';
  
  // Parker spiral field lines
  const spiralCount = 12;
  for (let i = 0; i < spiralCount; i++) {
    const angle = (i / spiralCount) * Math.PI * 2;
    const spiralPoints: THREE.Vector3[] = [];
    
    for (let r = 0.1; r < 150; r += 2) {
      // Parker spiral angle
      const spiralAngle = angle - (r / 10);
      const x = r * Math.cos(spiralAngle);
      const y = r * Math.sin(spiralAngle);
      const z = Math.sin(r * 0.05) * 5; // Slight waviness
      
      spiralPoints.push(new THREE.Vector3(x, z, y));
    }
    
    const spiralGeometry = new THREE.BufferGeometry().setFromPoints(spiralPoints);
    const spiralMaterial = new THREE.LineBasicMaterial({
      color: 0xffaa00,
      transparent: true,
      opacity: 0.3
    });
    const spiral = new THREE.Line(spiralGeometry, spiralMaterial);
    solarWindGroup.add(spiral);
  }
  
  scene.add(solarWindGroup);
  
  // ===== INTERSTELLAR WIND =====
  const ismWindGroup = new THREE.Group();
  ismWindGroup.name = 'interstellarWind';
  
  // Particle system for ISM flow
  const ismParticleCount = 2000;
  const ismGeometry = new THREE.BufferGeometry();
  const ismPositions = new Float32Array(ismParticleCount * 3);
  const ismVelocities = new Float32Array(ismParticleCount * 3);
  
  for (let i = 0; i < ismParticleCount; i++) {
    // Start from upstream
    ismPositions[i * 3] = 300 + Math.random() * 100;
    ismPositions[i * 3 + 1] = (Math.random() - 0.5) * 200;
    ismPositions[i * 3 + 2] = (Math.random() - 0.5) * 200;
    
    // Flow velocity
    ismVelocities[i * 3] = -2; // Flowing toward heliosphere
    ismVelocities[i * 3 + 1] = 0;
    ismVelocities[i * 3 + 2] = 0;
  }
  
  ismGeometry.setAttribute('position', new THREE.BufferAttribute(ismPositions, 3));
  
  const ismMaterial = new THREE.PointsMaterial({
    color: 0x6666ff,
    size: 0.5,
    transparent: true,
    opacity: 0.6
  });
  
  const ismParticles = new THREE.Points(ismGeometry, ismMaterial);
  ismWindGroup.add(ismParticles);
  scene.add(ismWindGroup);
  
  // ===== COORDINATE GRID =====
  const gridGroup = new THREE.Group();
  gridGroup.name = 'coordinateGrid';
  gridGroup.visible = visibility.coordinateGrid;
  
  // Ecliptic plane grid
  const gridHelper = new THREE.GridHelper(400, 40, 0x444444, 0x222222);
  gridHelper.rotateX(Math.PI / 2);
  gridGroup.add(gridHelper);
  
  // Axes
  const axesHelper = new THREE.AxesHelper(200);
  gridGroup.add(axesHelper);
  
  scene.add(gridGroup);
  
  // ===== DISTANCE MARKERS =====
  const markersGroup = new THREE.Group();
  markersGroup.name = 'distanceMarkers';
  
  // AU distance rings
  const distances = [10, 50, 100, 150, 200];
  distances.forEach(dist => {
    const geometry = new THREE.RingGeometry(dist - 0.5, dist + 0.5, 64);
    const material = new THREE.MeshBasicMaterial({
      color: 0x333333,
      side: THREE.DoubleSide,
      transparent: true,
      opacity: 0.3
    });
    const ring = new THREE.Mesh(geometry, material);
    ring.rotation.x = Math.PI / 2;
    markersGroup.add(ring);
    
    // Label
    // (In production, use CSS3D or sprite labels)
  });
  
  scene.add(markersGroup);
  
  // ===== LIGHTING =====
  const ambientLight = new THREE.AmbientLight(0x222222);
  scene.add(ambientLight);
  
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
  directionalLight.position.set(100, 100, 100);
  scene.add(directionalLight);
  
  // ===== UPDATE FUNCTION =====
  function updateScene(date: Date) {
    const jd = JulianDate.fromDate(date);
    
    // Update planetary positions
    const planetPositions = dataService.getPlanetaryPositions(date);
    planetPositions.forEach((position, name) => {
      const planet = planetMeshes.get(name);
      if (planet) {
        planet.position.copy(position.multiplyScalar(AU_SCALE));
      }
    });
    
    // Update spacecraft positions
    const spacecraftData = dataService.getDataStore().spacecraft;
    
    // Voyager 1
    const v1Data = spacecraftData.get('Voyager 1');
    if (v1Data) {
      const v1Pos = v1Data.trajectory.position.interpolate(jd);
      voyager1.position.copy(v1Pos.multiplyScalar(AU_SCALE));
      
      // Update trajectory
      const v1Traj = dataService.getSpacecraftTrajectory(
        'Voyager 1',
        v1Data.launch,
        date,
        200
      );
      v1TrajGeometry.setFromPoints(v1Traj.map(p => p.multiplyScalar(AU_SCALE)));
    }
    
    // Voyager 2
    const v2Data = spacecraftData.get('Voyager 2');
    if (v2Data) {
      const v2Pos = v2Data.trajectory.position.interpolate(jd);
      voyager2.position.copy(v2Pos.multiplyScalar(AU_SCALE));
      
      // Update trajectory
      const v2Traj = dataService.getSpacecraftTrajectory(
        'Voyager 2',
        v2Data.launch,
        date,
        200
      );
      v2TrajGeometry.setFromPoints(v2Traj.map(p => p.multiplyScalar(AU_SCALE)));
    }
    
    // Update heliosphere shape based on solar cycle
    const solarWind = dataService.getSolarWindConditions(date, 1);
    const pressure = solarWind.pressure / 2; // Normalize
    
    // Regenerate boundaries if needed (expensive, do sparingly)
    if (Math.random() < 0.01) { // 1% chance per frame
      // Update termination shock
      const newTSGeometry = heliosphereModel.generateParametricSurface(
        'terminationShock',
        jd,
        48
      );
      newTSGeometry.scale(AU_SCALE, AU_SCALE, AU_SCALE);
      
      // Update all termination shock layers
      const tsCore = heliosphereGroup.getObjectByName('terminationShockCore') as THREE.Mesh;
      const tsInner = heliosphereGroup.getObjectByName('terminationShockInner') as THREE.Mesh;
      const tsMid = heliosphereGroup.getObjectByName('terminationShockMid') as THREE.Mesh;
      const tsOuter = heliosphereGroup.getObjectByName('terminationShockOuter') as THREE.Mesh;
      const tsHalo = heliosphereGroup.getObjectByName('terminationShockHalo') as THREE.Mesh;
      
      if (tsCore) {
        tsCore.geometry.dispose();
        tsCore.geometry = newTSGeometry.clone();
      }
      if (tsInner) {
        tsInner.geometry.dispose();
        const tsInnerGeo = newTSGeometry.clone();
        tsInnerGeo.scale(1.008, 1.008, 1.008);
        tsInner.geometry = tsInnerGeo;
      }
      if (tsMid) {
        tsMid.geometry.dispose();
        const tsMidGeo = newTSGeometry.clone();
        tsMidGeo.scale(1.016, 1.016, 1.016);
        tsMid.geometry = tsMidGeo;
      }
      if (tsOuter) {
        tsOuter.geometry.dispose();
        const tsOuterGeo = newTSGeometry.clone();
        tsOuterGeo.scale(1.024, 1.024, 1.024);
        tsOuter.geometry = tsOuterGeo;
      }
      if (tsHalo) {
        tsHalo.geometry.dispose();
        const tsHaloGeo = newTSGeometry.clone();
        tsHaloGeo.scale(1.032, 1.032, 1.032);
        tsHalo.geometry = tsHaloGeo;
      }
      
      // Update heliopause
      const newHPGeometry = heliosphereModel.generateParametricSurface(
        'heliopause',
        jd,
        48
      );
      newHPGeometry.scale(AU_SCALE, AU_SCALE, AU_SCALE);
      heliopauseMesh.geometry.dispose();
      heliopauseMesh.geometry = newHPGeometry;
    }
    
    // Update ISM particles
    const ismPos = ismGeometry.attributes.position.array as Float32Array;
    const ismVel = ismVelocities;
    
    for (let i = 0; i < ismParticleCount; i++) {
      const idx = i * 3;
      
      // Update position
      ismPos[idx] += ismVel[idx];
      ismPos[idx + 1] += ismVel[idx + 1];
      ismPos[idx + 2] += ismVel[idx + 2];
      
      // Reset if too far
      if (ismPos[idx] < -300) {
        ismPos[idx] = 300 + Math.random() * 100;
        ismPos[idx + 1] = (Math.random() - 0.5) * 200;
        ismPos[idx + 2] = (Math.random() - 0.5) * 200;
      }
    }
    
    ismGeometry.attributes.position.needsUpdate = true;
  }
  
  // Main update function
  function update(date: Date, speed: number, motionEnabled: boolean) {
    // Update time
    if (motionEnabled) {
      currentDate = new Date(currentDate.getTime() + speed * 24 * 60 * 60 * 1000);
      
      if (timeMode === 'realtime') {
        currentDate = new Date(); // Snap to real time
      }
    }
    
    // Update scene
    updateScene(currentDate);
    
    // Update controls
    controls.update();
    
    // Render
    renderer.render(scene, camera);
  }
  
  // Resize handler
  function resize(w: number, h: number) {
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }
  
  // Cleanup
  function dispose() {
    controls.dispose();
    renderer.dispose();
    
    // Dispose geometries and materials
    scene.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose();
        if (child.material instanceof THREE.Material) {
          child.material.dispose();
        }
      }
    });
  }
  
  // Toggle visibility
  function toggleComponent(component: keyof ComponentVisibility, visible: boolean) {
    visibility[component] = visible;
    
    const componentMap: Record<string, string> = {
      'heliosphere': 'heliopause',
      'terminationShock': 'terminationShock',
      'bowShock': 'bowShock',
      'solarWind': 'solarWind',
      'interstellarWind': 'interstellarWind',
      'planets': 'planets',
      'orbits': 'orbits',
      'spacecraft': 'spacecraft',
      'trajectories': 'trajectories',
      'stars': 'stars',
      'coordinateGrid': 'coordinateGrid',
      'distanceMarkers': 'distanceMarkers'
    };
    
    const objectName = componentMap[component];
    if (objectName) {
      const object = scene.getObjectByName(objectName);
      if (object) {
        object.visible = visible;
      }
    }
  }
  
  function getVisibility(): ComponentVisibility {
    return { ...visibility };
  }
  
  function setTimeMode(mode: TimeMode) {
    timeMode = mode;
    
    switch (mode) {
      case 'historical':
        currentDate = VoyagerTrajectories.VOYAGER_1.launch;
        break;
      case 'realtime':
        currentDate = new Date();
        break;
      case 'prediction':
        currentDate = new Date(2030, 0, 1);
        break;
    }
  }
  
  function getCurrentDate(): Date {
    return new Date(currentDate);
  }
  
  // Initial update
  updateScene(currentDate);
  
  return { 
    canvas, 
    update, 
    resize, 
    dispose, 
    toggleComponent, 
    getVisibility,
    setTimeMode,
    getCurrentDate
  };
}
