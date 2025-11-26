import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { basisFromApex, ECLIPTIC_TILT, PHYSICAL_SCALES, HELIOSPHERE_NOSE } from "./apex";
import { FAMOUS_STARS, raDecToCartesian } from "./starCatalog";
import { 
  HELIOSPHERIC_DISTANCES, 
  HELIOSPHERIC_VELOCITIES,
  solarWindDensity,
  solarWindVelocity,
  heliopauseDistance,
  terminationShockDistance
} from "./heliosphericPhysics";
import { HeliosphereModel } from "./physics/HeliosphereModel";
import { VoyagerTrajectories } from "./physics/SpacecraftTrajectories";
import { JulianDate } from "./data/AstronomicalDataStore";
import { PlanetaryEphemeris, PLANET_PROPERTIES } from "./data/PlanetaryEphemeris";
import { LabelManager } from "./LabelSystem";
import { getAllInterstellarObjects, generateOumuamuaTrajectory, generateBorisovTrajectory } from "./data/InterstellarObjects";
import { COMPLETE_CONSTELLATIONS, CONSTELLATION_STARS, generateConstellationLines } from "./data/Constellations";

type Direction = 1 | -1;

export type ComponentVisibility = {
  heliosphere: boolean;
  helioglow: boolean;
  terminationShock: boolean;
  bowShock: boolean;
  solarWind: boolean;
  interstellarWind: boolean;
  planets: boolean;
  orbits: boolean;
  moon: boolean;
  stars: boolean;
  famousStars: boolean;
  voyagers: boolean;
  distanceMarkers: boolean;
  solarApex: boolean;
  labels: boolean;
  interstellarObjects: boolean;
  constellations: boolean;
};

export type SceneAPI = {
  canvas: HTMLCanvasElement;
  update: (year: number, direction: Direction, motionEnabled: boolean) => void;
  resize: (w: number, h: number) => void;
  dispose: () => void;
  toggleComponent: (component: keyof ComponentVisibility, visible: boolean) => void;
  getVisibility: () => ComponentVisibility;
};

export type SceneOptions = {
  initialViewport?: {
    width: number;
    height: number;
    devicePixelRatio: number;
  };
};

export function createScene(canvas: HTMLCanvasElement, options?: SceneOptions): SceneAPI {
  const initialViewport = options?.initialViewport;
  const fallbackWidth = Math.max(
    initialViewport?.width ??
      (typeof window !== 'undefined' ? window.innerWidth : canvas.clientWidth) ??
      1,
    1
  );
  const fallbackHeight = Math.max(
    initialViewport?.height ??
      (typeof window !== 'undefined' ? window.innerHeight : canvas.clientHeight) ??
      1,
    1
  );
  const initialDevicePixelRatio =
    initialViewport?.devicePixelRatio ??
    (typeof window !== 'undefined' ? window.devicePixelRatio ?? 1 : 1);
  const initialAspect = fallbackWidth / fallbackHeight;
  const isMobileViewport = initialViewport
    ? initialViewport.width <= 768
    : (typeof window !== 'undefined' ? window.innerWidth <= 768 : false);

  // Renderer / Scene / Camera
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: !isMobileViewport,
    powerPreference: "high-performance",
  });
  const pixelCap = isMobileViewport ? 1.6 : 2.2;
  renderer.setPixelRatio(Math.min(initialDevicePixelRatio, pixelCap));
  renderer.setSize(fallbackWidth, fallbackHeight, false);
  renderer.domElement.style.transform = 'translateZ(0)';
  renderer.domElement.style.willChange = 'transform';
  const scene = new THREE.Scene();
  // Deep space black - no atmospheric scattering in interstellar space
  scene.background = new THREE.Color(0x0a0a15); // Dark blue instead of pure black

  const camera = new THREE.PerspectiveCamera(55, 1, 0.1, 3000);
  const defaultCameraPosition = new THREE.Vector3(0, 6, 25); // Pull back 2.5x to show full heliosphere
  camera.position.copy(defaultCameraPosition);
  camera.lookAt(0, 0, 0);
  
  // Interactive camera controls
  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true; // Smooth camera movement
  controls.dampingFactor = isMobileViewport ? 0.08 : 0.05; // More damping on mobile for stability
  controls.enableZoom = true;
  controls.enablePan = true;
  
  // Mobile-optimized touch controls
  if (isMobileViewport) {
    // Configure touch controls for mobile
    if ('touches' in controls) {
      (controls as any).touches = {
        ONE: 0, // ROTATE
        TWO: 2, // DOLLY_PAN
      };
    }
    controls.enableRotate = true;
    controls.rotateSpeed = 0.5; // Slower rotation on mobile
    controls.panSpeed = 0.5; // Slower panning on mobile
    controls.zoomSpeed = 0.6; // Slower zoom on mobile
  }
  
  controls.minDistance = 8; // Don't zoom too close
  controls.maxDistance = 100; // Allow more zoom out to see full structure
  controls.target.set(0, 0, 0); // Look at heliosphere center
  controls.update();
  adjustCameraForViewport(initialAspect, isMobileViewport);
  
  // Prevent OrbitControls from capturing clicks on UI elements
  // Check if click target is a UI element before allowing controls to handle it
  canvas.addEventListener('pointerdown', (e: PointerEvent) => {
    const target = e.target as HTMLElement;
    // Check if clicking on a UI element (button, input, or element with data-ui attribute)
    if (target.closest('button') || 
        target.closest('input') || 
        target.closest('[data-ui]') ||
        target.closest('.z-\\[9999\\]') ||
        window.getComputedStyle(target).zIndex === '9999') {
      e.stopPropagation();
      return;
    }
  }, true); // Use capture phase to intercept before OrbitControls
  
  // Camera panning state (optional auto-panning)
  let cameraTime = 0;
  const cameraRadius = 12;
  const cameraSpeed = 0.0001; // Slow panning
  let autoPanning = false; // Disabled by default, user controls camera

  // ===== Fixed heliosphere basis (screen +X = upwind/nose direction) =====
  // The heliosphere nose points into the interstellar wind at ecliptic λ≈255.4°, β≈5.2°
  const apexBasis = basisFromApex(); // X-axis points toward interstellar upwind
  
  // Initialize heliosphere model for accurate boundaries
  const heliosphereModel = new HeliosphereModel();
  const currentJD = JulianDate.fromDate(new Date());

  // ===== Starfield (galactic background streaming past) =====
  // Stars represent the Milky Way galaxy streaming past as the heliosphere moves through it
  // We're viewing from a distance: heliosphere is fixed, stars stream past
  // Distribution based on Gaia catalog statistics for solar neighborhood
  const starMat = new THREE.PointsMaterial({ 
    size: 0.028,  // Larger for better visibility from farther camera distance
    transparent: true, 
    opacity: 1.0,
    sizeAttenuation: true,  // Stars get smaller with distance
    vertexColors: true
  });
  const starGeo = new THREE.BufferGeometry();
  {
    const N = 8000; // Increased for richer field
    const pos = new Float32Array(N * 3);
    const colors = new Float32Array(N * 3);
    
    for (let i = 0; i < N; i++) {
      // Realistic distance distribution (parsecs converted to scene units)
      // Most stars within 100-500 pc, following inverse square law
      const r = 120 + Math.pow(Math.random(), 0.6) * 380;
      
      // Galactic plane concentration (stars cluster near galactic equator)
      const galacticLat = (Math.random() - 0.5) * Math.PI * 0.3; // ±27° concentration
      const galacticLon = Math.random() * 2 * Math.PI;
      
      // Convert galactic to equatorial-ish for display
      const theta = galacticLon;
      const phi = Math.PI/2 + galacticLat;
      
      const p = new THREE.Vector3(
        r * Math.sin(phi) * Math.cos(theta),
        r * Math.sin(phi) * Math.sin(theta),
        r * Math.cos(phi)
      ).applyMatrix4(apexBasis);
      
      pos[i * 3 + 0] = p.x; 
      pos[i * 3 + 1] = p.y; 
      pos[i * 3 + 2] = p.z;
      
      // Realistic star colors (based on spectral types)
      // Most stars are red dwarfs, some yellow like Sun, few blue giants
      const spectralRand = Math.random();
      if (spectralRand < 0.76) {
        // M-type red dwarfs (76% of stars)
        colors[i * 3 + 0] = 1.0;
        colors[i * 3 + 1] = 0.8;
        colors[i * 3 + 2] = 0.7;
      } else if (spectralRand < 0.88) {
        // K-type orange dwarfs (12%)
        colors[i * 3 + 0] = 1.0;
        colors[i * 3 + 1] = 0.9;
        colors[i * 3 + 2] = 0.8;
      } else if (spectralRand < 0.96) {
        // G-type yellow dwarfs like Sun (8%)
        colors[i * 3 + 0] = 1.0;
        colors[i * 3 + 1] = 1.0;
        colors[i * 3 + 2] = 0.9;
      } else {
        // F, A, B, O-type blue/white stars (4%)
        colors[i * 3 + 0] = 0.9;
        colors[i * 3 + 1] = 0.95;
        colors[i * 3 + 2] = 1.0;
      }
    }
    starGeo.setAttribute("position", new THREE.BufferAttribute(pos, 3));
    starGeo.setAttribute("color", new THREE.BufferAttribute(colors, 3));
  }
  const stars = new THREE.Points(starGeo, starMat);
  stars.name = 'stars';
  scene.add(stars);
  
  // ===== Famous Stars (labeled landmarks) =====
  const famousStarsGroup = new THREE.Group();
  famousStarsGroup.name = 'famousStars';
  FAMOUS_STARS.forEach((star) => {
    const [x, y, z] = raDecToCartesian(star.ra, star.dec, star.distance * 0.1); // Scale distance
    const pos = new THREE.Vector3(x, y, z).applyMatrix4(apexBasis);
    
    // Create star point
    const starGeo = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(0, 0, 0)]);
    const starMat = new THREE.PointsMaterial({
      size: 0.05 * (1 + (2.5 - star.magnitude) * 0.15), // Larger for visibility from distance
      color: star.color,
      transparent: true,
      opacity: 1.0,  // Fully opaque
      sizeAttenuation: true
    });
    const starPoint = new THREE.Points(starGeo, starMat);
    starPoint.position.copy(pos);
    famousStarsGroup.add(starPoint);
    
    // Add label sprite (optional - can be toggled)
    // For now, we'll just mark them with slightly larger/bright stars
  });
  scene.add(famousStarsGroup);

  // ===== Heliosphere (MHD-based asymmetric shape) =====
  // Use accurate MHD model for heliosphere boundaries
  const helio = (() => {
    // Create heliopause geometry using MHD model
    const hpGeometry = heliosphereModel.generateParametricSurface(
      'heliopause',
      currentJD,
      64
    );
    hpGeometry.scale(0.03, 0.03, 0.03); // Scale AU to scene units
    
    // Realistic heliosphere material - nearly transparent surface
    const m = new THREE.MeshPhysicalMaterial({
      color: new THREE.Color(0x1a2a4e),
      emissive: new THREE.Color(0x0a1a2a),
      transmission: 0.95,  // Higher transparency
      thickness: 0.4,
      roughness: 0.95,
      metalness: 0.0,
      transparent: true,
      opacity: 0.08,  // Much more transparent, almost invisible
      side: THREE.FrontSide  // Only render front to reduce visual density
    });
    
    const mesh = new THREE.Mesh(hpGeometry, m);
    mesh.setRotationFromMatrix(apexBasis);
    mesh.name = 'heliosphere';
    scene.add(mesh);
    
    // Add visible UV glow effect with proper shading
    const glowGeometry = hpGeometry.clone();
    glowGeometry.scale(1.15, 1.15, 1.15);  // Slightly larger glow
    const glowMaterial = new THREE.MeshPhongMaterial({
      color: new THREE.Color(0x3a5a8e),  // Brighter cyan-blue UV glow
      emissive: new THREE.Color(0x2a4a7e),  // Emissive glow
      emissiveIntensity: 0.8,
      transparent: true,
      opacity: 0.25,  // Much more visible
      shininess: 50,  // Glossy UV appearance
      side: THREE.DoubleSide
    });
    const glow = new THREE.Mesh(glowGeometry, glowMaterial);
    glow.setRotationFromMatrix(apexBasis);
    glow.name = 'helioglow';
    scene.add(glow);
    
    return { mesh, glow, geometry: hpGeometry, heliosphereModel };
  })();
  
  // ===== Interstellar Wind Particles =====
  // Interstellar medium flows into the heliosphere from the upwind direction (+X)
  // Flow direction: λ=255.4°, β=5.2° (ecliptic coordinates)
  // Speed: 26.3 km/s relative to Sun
  const ismWindCount = 1500;
  const ismWindGeo = new THREE.BufferGeometry();
  const ismWindPositions = new Float32Array(ismWindCount * 3);
  const ismWindVelocities = new Float32Array(ismWindCount * 3);
  const ismWindColors = new Float32Array(ismWindCount * 3);
  
  // ISM flow direction in ecliptic coordinates
  const ismFlowLon = 255.4 * Math.PI / 180;
  const ismFlowLat = 5.2 * Math.PI / 180;
  const ismFlowDirection = new THREE.Vector3(
    -Math.cos(ismFlowLat) * Math.cos(ismFlowLon),
    -Math.cos(ismFlowLat) * Math.sin(ismFlowLon),
    -Math.sin(ismFlowLat)
  ).normalize();
  
  for (let i = 0; i < ismWindCount; i++) {
    // Start from upwind direction (positive X in heliosphere frame, spread out uniformly)
    const spread = 8 + Math.random() * 4; // Start 8-12 units away
    const offsetY = (Math.random() - 0.5) * 6;
    const offsetZ = (Math.random() - 0.5) * 6;
    
    // Position particles upstream in heliosphere frame (X-axis points toward ISM wind)
    const startPosLocal = new THREE.Vector3(spread, offsetY, offsetZ);
    // Transform to world coordinates using apexBasis (heliosphere is rotated)
    const startPos = startPosLocal.applyMatrix4(apexBasis);
    ismWindPositions[i * 3 + 0] = startPos.x;
    ismWindPositions[i * 3 + 1] = startPos.y;
    ismWindPositions[i * 3 + 2] = startPos.z;
    
    // Flow velocity: uniform parallel flow toward heliosphere (in heliosphere frame, flows along -X)
    const speed = 0.015 + Math.random() * 0.01;
    const velocityLocal = new THREE.Vector3(-speed, 0, 0); // Flow along -X in heliosphere frame
    // Transform velocity to world coordinates
    const velocity = velocityLocal.applyMatrix4(apexBasis);
    ismWindVelocities[i * 3 + 0] = velocity.x;
    ismWindVelocities[i * 3 + 1] = velocity.y;
    ismWindVelocities[i * 3 + 2] = velocity.z;
    
    // Interstellar medium color (blue-purple, cooler than solar wind)
    ismWindColors[i * 3 + 0] = 0.6;
    ismWindColors[i * 3 + 1] = 0.7;
    ismWindColors[i * 3 + 2] = 1.0;
  }
  
  ismWindGeo.setAttribute('position', new THREE.BufferAttribute(ismWindPositions, 3));
  ismWindGeo.setAttribute('color', new THREE.BufferAttribute(ismWindColors, 3));
  
  const ismWindMat = new THREE.PointsMaterial({
    size: 0.015,  // Smaller, more numerous feel
    transparent: true,
    opacity: 0.4,  // More subtle
    vertexColors: true,
    sizeAttenuation: true,
    blending: THREE.AdditiveBlending
  });
  
  const ismWind = new THREE.Points(ismWindGeo, ismWindMat);
  ismWind.name = 'interstellarWind';
  scene.add(ismWind);

  // ===== Solar system (MOVES sideways through heliosphere) =====
  const sol = new THREE.Group(); // this group will translate along +X
  scene.add(sol);

  // Sun - smaller for better proportion to heliosphere
  const sunGeometry = new THREE.SphereGeometry(0.25, 32, 32);
  const sunMaterial = new THREE.MeshStandardMaterial({ 
    color: 0xffffaa,  // Brighter yellow-white
    emissive: 0xffaa44,  // Add emissive glow
    emissiveIntensity: 1.5
  });
  const sun = new THREE.Mesh(sunGeometry, sunMaterial);
  sol.add(sun);
  
  // Sun glow/halo - also smaller
  const sunGlowGeometry = new THREE.SphereGeometry(0.3, 32, 32);
  const sunGlowMaterial = new THREE.MeshBasicMaterial({
    color: 0xffaa44,
    transparent: true,
    opacity: 0.3,
    side: THREE.DoubleSide
  });
  const sunGlow = new THREE.Mesh(sunGlowGeometry, sunGlowMaterial);
  sol.add(sunGlow);
  
  // ===== Solar Wind Streams (fine threads with accurate physics) =====
  // Solar wind streams outward from the Sun and curves when hitting interstellar medium
  const solarWindGroup = new THREE.Group();
  solarWindGroup.name = 'solarWind';
  sol.add(solarWindGroup);
  
  const SOLAR_WIND_STREAMS = 150; // More streams for better visualization
  
  interface SolarWindStream {
    direction: THREE.Vector3;
    points: THREE.Vector3[];
    line: THREE.Line;
    age: number;
    heliopauseDist: number;
    terminationShockDist: number;
  }
  
  const solarWindStreams: SolarWindStream[] = [];
  const noseDirection = new THREE.Vector3(1, 0, 0); // Upwind direction (+X)
  
  // Create fine thread-like streams with accurate physics
  for (let i = 0; i < SOLAR_WIND_STREAMS; i++) {
    // Random direction from sun
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    const direction = new THREE.Vector3(
      Math.sin(phi) * Math.cos(theta),
      Math.sin(phi) * Math.sin(theta),
      Math.cos(phi)
    ).normalize();
    
    // Calculate accurate distances for this direction
    const heliopauseDist = heliopauseDistance(direction, noseDirection) * 0.03; // Scale to scene units
    const terminationShockDist = terminationShockDistance(direction, noseDirection) * 0.03;
    
    // Create stream with multiple points for smooth curves
    const streamPoints: THREE.Vector3[] = [];
    const segments = 60; // More segments for smoother curves
    
    for (let j = 0; j <= segments; j++) {
      const t = j / segments;
      const maxDist = heliopauseDist * 1.5; // Extend beyond heliopause
      const distance = t * maxDist;
      
      // Start from sun surface
      const basePos = direction.clone().multiplyScalar(0.5 + distance);
      
      // Physics: Different behavior in different regions
      if (distance < terminationShockDist) {
        // Inner heliosphere: supersonic, straight radial flow
        // Velocity decreases slightly with distance
        const velocity = solarWindVelocity(distance / 0.03) / HELIOSPHERIC_VELOCITIES.SOLAR_WIND_OUTER;
        basePos.multiplyScalar(1.0 + (1.0 - velocity) * 0.1);
      } else if (distance < heliopauseDist) {
        // Heliosheath: compressed, turbulent, subsonic
        const excessDist = distance - terminationShockDist;
        const sheathWidth = heliopauseDist - terminationShockDist;
        const compression = 1.0 + (excessDist / sheathWidth) * 0.3; // Compressed
        
        // Turbulent deflection in heliosheath
        const turbulence = Math.sin(excessDist * 5) * 0.05 * (excessDist / sheathWidth);
        const perp = new THREE.Vector3().crossVectors(direction, noseDirection).normalize();
        if (perp.length() > 0.1) {
          basePos.add(perp.multiplyScalar(turbulence));
        }
        
        basePos.multiplyScalar(compression);
      } else {
        // Beyond heliopause: curve due to ISM pressure and flow
        const excessDist = distance - heliopauseDist;
        const curveAmount = Math.min(excessDist * 0.25, 2.0);
        
        // ISM flows from +X, deflects solar wind
        const ismFlowDir = new THREE.Vector3(-1, 0, 0);
        
        // Deflect perpendicular to both directions
        const perp1 = new THREE.Vector3().crossVectors(direction, ismFlowDir).normalize();
        if (perp1.length() < 0.1) {
          perp1.set(-direction.y, direction.x, 0).normalize();
        }
        const perp2 = new THREE.Vector3().crossVectors(direction, perp1).normalize();
        
        // Curved deflection with exponential decay
        const deflectionStrength = curveAmount * Math.exp(-excessDist * 0.4);
        const phase = excessDist * 2.0;
        const deflection = perp1.clone().multiplyScalar(
          Math.sin(phase) * deflectionStrength
        ).add(
          perp2.clone().multiplyScalar(
            Math.cos(phase) * deflectionStrength * 0.7
          )
        );
        
        basePos.add(deflection);
      }
      
      streamPoints.push(basePos.clone());
    }
    
    // Density-based opacity (denser near Sun, thinner far out)
    const density = solarWindDensity(streamPoints[streamPoints.length - 1].length() / 0.03);
    const opacity = Math.min(0.7, Math.max(0.2, density / 10));
    
    // Create line geometry for fine threads (barely visible guide lines)
    const geometry = new THREE.BufferGeometry().setFromPoints(streamPoints);
    const material = new THREE.LineBasicMaterial({
      color: 0xffffaa,
      transparent: true,
      opacity: opacity * 0.15,  // Much more subtle, almost invisible
      linewidth: 0.5,
      blending: THREE.AdditiveBlending  // Soft glow effect
    });
    
    const line = new THREE.Line(geometry, material);
    solarWindGroup.add(line);
    
    solarWindStreams.push({
      direction: direction,
      points: streamPoints,
      line: line,
      age: Math.random(),
      heliopauseDist: heliopauseDist,
      terminationShockDist: terminationShockDist
    });
  }
  
  // ===== Realistic Particle-Based Solar Wind =====
  // Add thousands of small particles for realistic plasma flow appearance
  const SOLAR_WIND_PARTICLES = 4000;
  const solarWindParticleGeo = new THREE.BufferGeometry();
  const solarWindParticlePositions = new Float32Array(SOLAR_WIND_PARTICLES * 3);
  const solarWindParticleColors = new Float32Array(SOLAR_WIND_PARTICLES * 3);
  const solarWindParticleVelocities: THREE.Vector3[] = [];
  
  // Initialize solar wind particles
  for (let i = 0; i < SOLAR_WIND_PARTICLES; i++) {
    // Random direction from sun
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    const direction = new THREE.Vector3(
      Math.sin(phi) * Math.cos(theta),
      Math.sin(phi) * Math.sin(theta),
      Math.cos(phi)
    ).normalize();
    
    // Start from sun surface with some randomness
    const startDist = 0.5 + Math.random() * 0.3;
    const pos = direction.clone().multiplyScalar(startDist);
    
    solarWindParticlePositions[i * 3 + 0] = pos.x;
    solarWindParticlePositions[i * 3 + 1] = pos.y;
    solarWindParticlePositions[i * 3 + 2] = pos.z;
    
    // Velocity: radial outward with slight turbulence
    const baseSpeed = 0.008 + Math.random() * 0.004;
    const velocity = direction.clone().multiplyScalar(baseSpeed);
    // Add subtle turbulence (Perlin-like noise approximation)
    velocity.x += (Math.random() - 0.5) * 0.001;
    velocity.y += (Math.random() - 0.5) * 0.001;
    velocity.z += (Math.random() - 0.5) * 0.001;
    solarWindParticleVelocities.push(velocity);
    
    // Color gradient: yellow-white near sun to pale blue farther out
    const distFromSun = startDist;
    const maxDist = 4.0; // Heliosphere extent in scene units
    const t = Math.min(distFromSun / maxDist, 1.0);
    
    // Interpolate from yellow-white (1.0, 1.0, 0.8) to pale blue (0.8, 0.9, 1.0)
    solarWindParticleColors[i * 3 + 0] = 1.0 - t * 0.2;  // R
    solarWindParticleColors[i * 3 + 1] = 1.0 - t * 0.1;  // G
    solarWindParticleColors[i * 3 + 2] = 0.8 + t * 0.2;  // B
  }
  
  solarWindParticleGeo.setAttribute('position', new THREE.BufferAttribute(solarWindParticlePositions, 3));
  solarWindParticleGeo.setAttribute('color', new THREE.BufferAttribute(solarWindParticleColors, 3));
  
  const solarWindParticleMat = new THREE.PointsMaterial({
    size: 0.010,  // Small particles
    transparent: true,
    opacity: 0.25,  // Subtle, not bright
    vertexColors: true,
    sizeAttenuation: true,
    blending: THREE.AdditiveBlending,  // Soft glow
    depthWrite: false
  });
  
  const solarWindParticles = new THREE.Points(solarWindParticleGeo, solarWindParticleMat);
  solarWindParticles.name = 'solarWindParticles';
  sol.add(solarWindParticles);
  
  // ===== Termination Shock Visualization =====
  // Show the boundary where solar wind slows from supersonic to subsonic
  // Using smooth volumetric glow with multiple gradient layers for natural blending
  const terminationShockGroup = new THREE.Group();
  terminationShockGroup.name = 'terminationShock';
  scene.add(terminationShockGroup);
  
  // Store references for time-based updates
  let tsCoreMesh: THREE.Mesh;
  let tsInnerMesh: THREE.Mesh;
  let tsMidMesh: THREE.Mesh;
  let tsOuterMesh: THREE.Mesh;
  let tsHaloMesh: THREE.Mesh;
  let tsBaseGeometry: THREE.BufferGeometry | undefined;
  let lastTSUpdateYear = -Infinity;
  let animationTime = 0; // Continuous animation time for pulsing effects
  
  // Create asymmetric termination shock using MHD model
  const createTerminationShock = (year: number) => {
    // Clear existing meshes
    terminationShockGroup.clear();
    
    // Calculate solar cycle phase (0-1 over 11 years)
    const solarCyclePhase = (year % 11) / 11;
    const solarActivity = 0.7 + Math.sin(solarCyclePhase * Math.PI * 2) * 0.3; // 0.4 to 1.0
    
    // Subtle opacity for aurora-like glow - visible but doesn't overpower UV glow
    // Base opacity range 0.15 to 0.25 with additive blending
    const baseOpacity = 0.15 + solarActivity * 0.10; // 0.15 to 0.25 (15% to 25%)
    
    // Create thin layers with small spacing for smooth aurora-like effect
    // Aurora effect: overlapping layers with moderate opacity and additive blending
    const layerCount = 8;
    // Tight spacing between layers for smoother transitions
    const layerScales = [1.0, 1.01, 1.02, 1.03, 1.04, 1.05, 1.06, 1.07];
    // Smooth opacity falloff - no sharp transitions
    const layerOpacities = [
      baseOpacity * 1.0,   // Core
      baseOpacity * 0.9, 
      baseOpacity * 0.75,
      baseOpacity * 0.6,
      baseOpacity * 0.45,
      baseOpacity * 0.3,
      baseOpacity * 0.18,
      baseOpacity * 0.1  // Outer edge
    ];
    
    // Use lower resolution for memory efficiency (was 128, now 80 to prevent allocation errors)
    const resolution = 80;
    const jd = JulianDate.fromDate(new Date(Math.floor(year), 0, 1));
    const generatedGeometry = heliosphereModel.generateParametricSurface(
      'terminationShock',
      jd,
      resolution
    );
    
    // Check if geometry was generated successfully
    if (!generatedGeometry) {
      console.warn('Failed to generate termination shock geometry');
      return;
    }
    
    tsBaseGeometry = generatedGeometry;
    tsBaseGeometry.scale(0.03, 0.03, 0.03); // Scale AU to scene units
    
    for (let i = 0; i < layerCount; i++) {
      // Ensure tsBaseGeometry exists before cloning
      if (!tsBaseGeometry) {
        console.error('tsBaseGeometry is undefined, cannot create layers');
        return;
      }
      
      // For first layer, use original geometry; for others, clone only when needed
      let layerGeometry: THREE.BufferGeometry;
      if (i === 0) {
        layerGeometry = tsBaseGeometry;
      } else {
        // Clone geometry for this layer
        try {
          layerGeometry = tsBaseGeometry.clone();
          layerGeometry.scale(layerScales[i], layerScales[i], layerScales[i]);
        } catch (error) {
          console.error(`Failed to clone geometry for layer ${i}:`, error);
          // Fallback: reuse base geometry if clone fails
          layerGeometry = tsBaseGeometry;
        }
      }
      
      // Use MeshBasicMaterial with additive blending for aurora-like glow
      const layerMaterial = new THREE.MeshBasicMaterial({
        color: 0xff8844,
        transparent: true,
        opacity: layerOpacities[i],
        side: THREE.DoubleSide,
        blending: THREE.AdditiveBlending, // Aurora-like additive glow
        depthWrite: false, // Important for proper blending
        fog: false // Don't let fog affect the glow
      });
      
      const layerMesh = new THREE.Mesh(layerGeometry, layerMaterial);
      layerMesh.setRotationFromMatrix(apexBasis);
      layerMesh.renderOrder = -1; // Render before other objects
      terminationShockGroup.add(layerMesh);
      
      // Store references for first few layers
      if (i === 0) tsCoreMesh = layerMesh;
      if (i === 1) tsInnerMesh = layerMesh;
      if (i === 2) tsMidMesh = layerMesh;
      if (i === 3) tsOuterMesh = layerMesh;
      if (i === 4) tsHaloMesh = layerMesh;
    }
    
    lastTSUpdateYear = year; // Store exact year, not just integer
  };
  
  // Initialize termination shock (use current year from date)
  const initialYear = new Date().getFullYear();
  createTerminationShock(initialYear);
  
  // ===== Bow Shock Visualization (Optional/Controversial) =====
  const bowShockGroup = new THREE.Group();
  bowShockGroup.name = 'bowShock';
  bowShockGroup.visible = false; // Hidden by default (controversial feature)
  scene.add(bowShockGroup);
  
  const bowShockGeometry = heliosphereModel.generateParametricSurface(
    'bowShock',
    currentJD,
    32
  );
  
  const bowShockPositions = bowShockGeometry.getAttribute('position');
  if (bowShockPositions && bowShockPositions.count > 0) {
    bowShockGeometry.scale(0.03, 0.03, 0.03);
    const bowShockMaterial = new THREE.MeshBasicMaterial({
      color: 0xff44ff,
      transparent: true,
      opacity: 0.1,
      wireframe: true,
      side: THREE.DoubleSide
    });
    const bowShock = new THREE.Mesh(bowShockGeometry, bowShockMaterial);
    bowShock.setRotationFromMatrix(apexBasis);
    bowShockGroup.add(bowShock);
  }
  
  // ===== AU Distance Markers =====
  const distanceMarkersGroup = new THREE.Group();
  distanceMarkersGroup.name = 'distanceMarkers';
  scene.add(distanceMarkersGroup);
  
  const markerDistances = [1, 5, 10, 20, 30, 50, 100, 150]; // AU
  markerDistances.forEach(au => {
    const radius = au * 0.03; // Scale to scene units
    const geometry = new THREE.RingGeometry(radius - 0.05, radius + 0.05, 64);
    const material = new THREE.MeshBasicMaterial({
      color: 0x333333,
      transparent: true,
      opacity: 0.2,
      side: THREE.DoubleSide
    });
    const ring = new THREE.Mesh(geometry, material);
    ring.rotation.x = Math.PI / 2; // Lay flat in ecliptic plane
    ring.name = `${au}AU`;
    distanceMarkersGroup.add(ring);
  });
  
  // ===== Solar Apex Direction Indicator =====
  const solarApexGroup = new THREE.Group();
  solarApexGroup.name = 'solarApex';
  scene.add(solarApexGroup);
  
  // Solar apex: RA 18h 28m (277°), Dec +30°
  // Convert to ecliptic coordinates (simplified)
  const apexRA = 277 * Math.PI / 180;
  const apexDec = 30 * Math.PI / 180;
  const apexDirection = new THREE.Vector3(
    Math.cos(apexDec) * Math.cos(apexRA),
    Math.cos(apexDec) * Math.sin(apexRA),
    Math.sin(apexDec)
  ).normalize();
  
  const apexArrow = new THREE.ArrowHelper(
    apexDirection,
    new THREE.Vector3(0, 0, 0),
    8,
    0xff8800,
    1.5,
    0.8
  );
  apexArrow.line.material = new THREE.LineBasicMaterial({
    color: 0xff8800,
    transparent: true,
    opacity: 0.6
  });
  apexArrow.cone.material = new THREE.MeshBasicMaterial({
    color: 0xff8800,
    transparent: true,
    opacity: 0.6
  });
  solarApexGroup.add(apexArrow);
  
  // Label for solar apex
  const apexLabel = new THREE.Sprite(
    new THREE.SpriteMaterial({
      map: null, // Would need texture for label
      transparent: true,
      opacity: 0.8
    })
  );
  apexLabel.position.copy(apexDirection.multiplyScalar(9));
  solarApexGroup.add(apexLabel);
  
  // ===== Voyager Spacecraft =====
  const voyagerGroup = new THREE.Group();
  voyagerGroup.name = 'voyagers';
  scene.add(voyagerGroup);
  
  // Voyager 1 - Billboard sprite marker with ring
  const v1Group = new THREE.Group();
  v1Group.name = 'Voyager 1';
  
  // Core sprite (small dot)
  const v1SpriteMap = new THREE.Texture();
  const v1SpriteCanvas = document.createElement('canvas');
  v1SpriteCanvas.width = 64;
  v1SpriteCanvas.height = 64;
  const v1Ctx = v1SpriteCanvas.getContext('2d')!;
  v1Ctx.fillStyle = '#00ff00';
  v1Ctx.beginPath();
  v1Ctx.arc(32, 32, 16, 0, Math.PI * 2);
  v1Ctx.fill();
  v1SpriteMap.image = v1SpriteCanvas;
  v1SpriteMap.needsUpdate = true;
  
  const v1SpriteMat = new THREE.SpriteMaterial({ map: v1SpriteMap, color: 0x00ff00 });
  const v1Sprite = new THREE.Sprite(v1SpriteMat);
  v1Sprite.scale.set(0.15, 0.15, 1);
  v1Group.add(v1Sprite);
  
  // Ring around sprite
  const v1RingGeo = new THREE.RingGeometry(0.08, 0.10, 32);
  const v1RingMat = new THREE.MeshBasicMaterial({
    color: 0x00ff00,
    transparent: true,
    opacity: 0.4,
    side: THREE.DoubleSide,
    blending: THREE.AdditiveBlending
  });
  const v1Ring = new THREE.Mesh(v1RingGeo, v1RingMat);
  v1Group.add(v1Ring);
  const voyager1 = v1Group;
  
  // Voyager 2 - Billboard sprite marker with ring
  const v2Group = new THREE.Group();
  v2Group.name = 'Voyager 2';
  
  // Core sprite (small dot)
  const v2SpriteMap = new THREE.Texture();
  const v2SpriteCanvas = document.createElement('canvas');
  v2SpriteCanvas.width = 64;
  v2SpriteCanvas.height = 64;
  const v2Ctx = v2SpriteCanvas.getContext('2d')!;
  v2Ctx.fillStyle = '#00ffff';
  v2Ctx.beginPath();
  v2Ctx.arc(32, 32, 16, 0, Math.PI * 2);
  v2Ctx.fill();
  v2SpriteMap.image = v2SpriteCanvas;
  v2SpriteMap.needsUpdate = true;
  
  const v2SpriteMat = new THREE.SpriteMaterial({ map: v2SpriteMap, color: 0x00ffff });
  const v2Sprite = new THREE.Sprite(v2SpriteMat);
  v2Sprite.scale.set(0.15, 0.15, 1);
  v2Group.add(v2Sprite);
  
  // Ring around sprite
  const v2RingGeo = new THREE.RingGeometry(0.08, 0.10, 32);
  const v2RingMat = new THREE.MeshBasicMaterial({
    color: 0x00ffff,
    transparent: true,
    opacity: 0.4,
    side: THREE.DoubleSide,
    blending: THREE.AdditiveBlending
  });
  const v2Ring = new THREE.Mesh(v2RingGeo, v2RingMat);
  v2Group.add(v2Ring);
  const voyager2 = v2Group;
  
  voyagerGroup.add(voyager1);
  voyagerGroup.add(voyager2);
  
  // Voyager trajectories
  const v1TrajGeometry = new THREE.BufferGeometry();
  const v1TrajMaterial = new THREE.LineBasicMaterial({
    color: 0x00ff00,
    transparent: true,
    opacity: 0.5
  });
  const v1Trajectory = new THREE.Line(v1TrajGeometry, v1TrajMaterial);
  v1Trajectory.name = 'Voyager 1 Trajectory';
  voyagerGroup.add(v1Trajectory);
  
  const v2TrajGeometry = new THREE.BufferGeometry();
  const v2TrajMaterial = new THREE.LineBasicMaterial({
    color: 0x00ffff,
    transparent: true,
    opacity: 0.5
  });
  const v2Trajectory = new THREE.Line(v2TrajGeometry, v2TrajMaterial);
  v2Trajectory.name = 'Voyager 2 Trajectory';
  voyagerGroup.add(v2Trajectory);
  
  // ===== Label System =====
  const labelManager = new LabelManager(canvas, camera);
  
  // ===== Interstellar Objects =====
  const interstellarObjectsGroup = new THREE.Group();
  interstellarObjectsGroup.name = 'interstellarObjects';
  scene.add(interstellarObjectsGroup);
  
  // 'Oumuamua
  const oumuamuaGeometry = new THREE.BoxGeometry(0.2, 0.2, 2.0); // Elongated shape
  const oumuamuaMaterial = new THREE.MeshBasicMaterial({ color: 0xff6600 });
  const oumuamua = new THREE.Mesh(oumuamuaGeometry, oumuamuaMaterial);
  oumuamua.name = "Oumuamua";
  interstellarObjectsGroup.add(oumuamua);
  
  // Oumuamua trajectory
  const oumuamuaTraj = generateOumuamuaTrajectory();
  const oumuamuaTrajPoints: THREE.Vector3[] = [];
  for (let i = 0; i < oumuamuaTraj.epochs.length; i++) {
    const pos = oumuamuaTraj.values[i];
    oumuamuaTrajPoints.push(pos.multiplyScalar(0.03));
  }
  const oumuamuaTrajGeometry = new THREE.BufferGeometry().setFromPoints(oumuamuaTrajPoints);
  const oumuamuaTrajLine = new THREE.Line(
    oumuamuaTrajGeometry,
    new THREE.LineBasicMaterial({ color: 0xff6600, transparent: true, opacity: 0.3 })
  );
  interstellarObjectsGroup.add(oumuamuaTrajLine);
  
  // 2I/Borisov
  const borisovGeometry = new THREE.SphereGeometry(0.15, 16, 16);
  const borisovMaterial = new THREE.MeshBasicMaterial({ color: 0x00ffff });
  const borisov = new THREE.Mesh(borisovGeometry, borisovMaterial);
  borisov.name = "2I/Borisov";
  interstellarObjectsGroup.add(borisov);
  
  // Borisov trajectory
  const borisovTraj = generateBorisovTrajectory();
  const borisovTrajPoints: THREE.Vector3[] = [];
  for (let i = 0; i < borisovTraj.epochs.length; i++) {
    const pos = borisovTraj.values[i];
    borisovTrajPoints.push(pos.multiplyScalar(0.03));
  }
  const borisovTrajGeometry = new THREE.BufferGeometry().setFromPoints(borisovTrajPoints);
  const borisovTrajLine = new THREE.Line(
    borisovTrajGeometry,
    new THREE.LineBasicMaterial({ color: 0x00ffff, transparent: true, opacity: 0.3 })
  );
  interstellarObjectsGroup.add(borisovTrajLine);
  
  // Andromeda Galaxy (very distant, shown as large fuzzy patch)
  const andromedaGeometry = new THREE.SphereGeometry(2.0, 32, 32);
  const andromedaMaterial = new THREE.MeshBasicMaterial({
    color: 0x8888ff,
    transparent: true,
    opacity: 0.2
  });
  const andromeda = new THREE.Mesh(andromedaGeometry, andromedaMaterial);
  andromeda.name = "Andromeda Galaxy";
  interstellarObjectsGroup.add(andromeda);
  
  // ===== Constellations =====
  const constellationsGroup = new THREE.Group();
  constellationsGroup.name = 'constellations';
  scene.add(constellationsGroup);
  
  // Add constellation stars to star catalog
  const allStars = [...FAMOUS_STARS, ...CONSTELLATION_STARS];
  
  // Create constellation lines
  COMPLETE_CONSTELLATIONS.forEach(constellation => {
    const lineGeometry = generateConstellationLines(constellation, allStars);
    if (lineGeometry) {
      const lineMaterial = new THREE.LineBasicMaterial({
        color: 0x4444ff,
        transparent: true,
        opacity: 0.3
      });
      const constellationLine = new THREE.LineSegments(lineGeometry, lineMaterial);
      constellationLine.name = constellation.name;
      constellationsGroup.add(constellationLine);
    }
  });

  // Brighter lighting for better visibility
  scene.add(new THREE.AmbientLight(0xffffff, 0.6));  // Double ambient for overall brightness
  const sunLight = new THREE.DirectionalLight(0xffffaa, 1.2);  // Increased sunlight intensity
  sunLight.position.set(2, 1, 2);
  scene.add(sunLight);
  
  // Add brighter fill light for heliosphere visibility
  const fillLight = new THREE.DirectionalLight(0x6a7a9a, 0.5);  // Increased fill light
  fillLight.position.set(-2, -1, -2);
  scene.add(fillLight);
  
  // ===== Reference Frame Indicators (optional scientific markers) =====
  // These show the various reference frames for educational purposes
  const createReferenceArrow = (direction: THREE.Vector3, color: number, length: number = 5) => {
    const origin = new THREE.Vector3(0, 0, 0);
    const arrowHelper = new THREE.ArrowHelper(direction, origin, length, color, length * 0.3, length * 0.15);
    arrowHelper.line.material = new THREE.LineBasicMaterial({ 
      color, 
      transparent: true, 
      opacity: 0.3,
      linewidth: 2 
    });
    arrowHelper.cone.material = new THREE.MeshBasicMaterial({ 
      color, 
      transparent: true, 
      opacity: 0.3 
    });
    return arrowHelper;
  };
  
  // Add reference indicators (can be toggled in a production version)
  const referenceGroup = new THREE.Group();
  referenceGroup.visible = false; // Hidden by default - could be toggled via UI
  
  // Interstellar wind direction (heliosphere nose) - already aligned with +X
  const windArrow = createReferenceArrow(new THREE.Vector3(1, 0, 0), 0x00ffff, 6);
  referenceGroup.add(windArrow);
  
  // Galactic center direction (approximate)
  const galacticDir = new THREE.Vector3(-0.05, -0.87, -0.48).normalize();
  const galacticArrow = createReferenceArrow(galacticDir, 0xff00ff, 4);
  referenceGroup.add(galacticArrow);
  
  scene.add(referenceGroup);

  // Orbits group (thin), tilted by ecliptic
  const orbitsGroup = new THREE.Group();
  orbitsGroup.name = 'orbits';
  sol.add(orbitsGroup);
  
  function makeOrbit(radius: number) {
    const pts = 256;
    const geom = new THREE.BufferGeometry().setFromPoints(
      Array.from({ length: pts }, (_, i) => {
        const t = (i / pts) * Math.PI * 2;
        return new THREE.Vector3(Math.cos(t) * radius, 0, Math.sin(t) * radius);
      })
    );
    const line = new THREE.LineLoop(
      geom,
      new THREE.LineBasicMaterial({ color: 0x80d8ff, transparent: true, opacity: 0.07 })
    );
    line.rotation.z = ECLIPTIC_TILT;
    orbitsGroup.add(line);
    return line;
  }

  // Realistic orbital distances in AU (semi-major axes)
  const PLANET_RADII = {
    Mercury: 0.387,
    Venus: 0.723,
    Earth: 1.0,
    Mars: 1.524,
    Jupiter: 5.203,
    Saturn: 9.537,
    Uranus: 19.191,
    Neptune: 30.069,
    Pluto: 39.482
  } as const;

  // Orbital periods in Earth years (for relative angular speeds)
  const PERIOD_Y = {
    Mercury: 0.241,
    Venus: 0.615,
    Earth: 1.0,
    Mars: 1.881,
    Jupiter: 11.86,
    Saturn: 29.46,
    Uranus: 84.01,
    Neptune: 164.8,
    Pluto: 247.92
  } as const;

  // Create planets + orbits with more realistic materials
  const planetsGroup = new THREE.Group();
  planetsGroup.name = 'planets';
  sol.add(planetsGroup);
  
  const planetMeshes: Record<string, THREE.Mesh> = {};
  const earthMeshRef: { current: THREE.Mesh | null } = { current: null };
  
  Object.entries(PLANET_RADII).forEach(([name, R]) => {
    // Scale AU to scene units (0.03 AU per scene unit based on heliosphere scale)
    const sceneRadius = R * 0.03;
    makeOrbit(sceneRadius);
    
    // More realistic planet colors and materials
    let color, emissive, metalness, roughness;
    // Proportional sizes based on real planet radii (Earth = baseline)
    const earthRadius = PLANET_PROPERTIES.Earth.radius;
    const planetRadius = PLANET_PROPERTIES[name as keyof typeof PLANET_PROPERTIES]?.radius || earthRadius;
    const relativeSize = planetRadius / earthRadius;
    // Tiny dots for planets - symbolic representation, not to scale
    const size = name === "Jupiter" ? 0.04 : 
                 name === "Saturn" ? 0.04 : 
                 name === "Uranus" || name === "Neptune" ? 0.03 :
                 name === "Pluto" ? 0.02 :
                 0.02; // Terrestrial planets as small dots
    
    switch(name) {
      case "Mercury":
        color = 0x8c7853;
        emissive = 0x000000;
        metalness = 0.1;
        roughness = 0.9;
        break;
      case "Venus":
        color = 0xffc649;
        emissive = 0x332200;
        metalness = 0.0;
        roughness = 0.95;
        break;
      case "Earth":
        color = 0x4a90e2;
        emissive = 0x001122;
        metalness = 0.0;
        roughness = 0.7;
        break;
      case "Mars":
        color = 0xcd5c5c;
        emissive = 0x000000;
        metalness = 0.1;
        roughness = 0.9;
        break;
      case "Jupiter":
        color = 0xd8ca9d;
        emissive = 0x221100;
        metalness = 0.0;
        roughness = 0.8;
        break;
      case "Saturn":
        color = 0xfad5a5;
        emissive = 0x221100;
        metalness = 0.0;
        roughness = 0.85;
        break;
      case "Uranus":
        color = 0x4fd0e7;
        emissive = 0x001122;
        metalness = 0.0;
        roughness = 0.9;
        break;
      case "Neptune":
        color = 0x4166f5;
        emissive = 0x000011;
        metalness = 0.0;
        roughness = 0.9;
        break;
      case "Pluto":
        color = 0xdaa520;
        emissive = 0x000000;
        metalness = 0.0;
        roughness = 0.95;
        break;
      default:
        color = 0x888888;
        emissive = 0x000000;
        metalness = 0.0;
        roughness = 0.95;
    }
    
    const mesh = new THREE.Mesh(
      new THREE.SphereGeometry(size, 32, 32), // More segments for smoother appearance
      new THREE.MeshStandardMaterial({ 
        color, 
        emissive,
        metalness, 
        roughness 
      })
    );
    mesh.userData.radius = sceneRadius; // Store scene-scaled radius
    mesh.userData.period = PERIOD_Y[name as keyof typeof PERIOD_Y];
    mesh.userData.auDistance = R; // Store actual AU distance
    mesh.rotation.z = ECLIPTIC_TILT;
    planetsGroup.add(mesh);
    planetMeshes[name] = mesh;
    
    // Store Earth reference for Moon
    if (name === "Earth") {
      earthMeshRef.current = mesh;
    }
  });
  
  // ===== Moon orbiting Earth =====
  const moonGroup = new THREE.Group();
  moonGroup.name = 'moon';
  sol.add(moonGroup);
  
  const moonGeometry = new THREE.SphereGeometry(0.02, 16, 16);
  const moonMaterial = new THREE.MeshStandardMaterial({
    color: 0xaaaaaa,
    emissive: 0x000000,
    metalness: 0.0,
    roughness: 0.9
  });
  const moon = new THREE.Mesh(moonGeometry, moonMaterial);
  moonGroup.add(moon);
  
  // Moon orbit radius (relative to Earth)
  const MOON_ORBIT_RADIUS = 0.15;
  const MOON_PERIOD_DAYS = 27.32; // Sidereal month in days
  const MOON_PERIOD_YEARS = MOON_PERIOD_DAYS / 365.25;

  // ==== Component visibility state (astronomer controls) ====
  const visibility: ComponentVisibility = {
    heliosphere: true,
    helioglow: true,  // Visible by default - shows UV glow boundary
    terminationShock: true,
    bowShock: false,
    solarWind: true,
    interstellarWind: true,
    planets: true,
    orbits: true,
    moon: true,
    stars: true,
    famousStars: true,
    voyagers: true,
    distanceMarkers: false,
    solarApex: false,
    labels: true,
    interstellarObjects: false,
    constellations: false,
  };
  helio.mesh.visible = visibility.heliosphere;
  helio.glow.visible = visibility.helioglow;
  
  // ==== Animation state ====
  let currentYear = 2024.0;   // Start at current year (can be adjusted)
  let driftX = 0;              // solar-system sideways drift inside fixed heliosphere
  let direction: Direction = 1;
  let motionEnabled = true;
  
  // Realistic velocity ratios based on astronomical data
  // We're showing the heliosphere moving through the galaxy
  // Stars represent the galactic background streaming past us
  const VELOCITY_SCALE = 0.0003; // Scaling factor for screen units/frame
  const GALACTIC_MOTION = 230;   // km/s - Sun's orbital speed around Milky Way
  
  // Star drift: stars stream past as heliosphere moves through galaxy
  // Heliosphere moves at ~26.3 km/s relative to local ISM
  // This creates parallax effect - stars appear to stream past
  // Speed: ~0.00083 AU/year (26.3 km/s ≈ 0.00083 AU/year)
  // In scene units (1 AU = 0.03 units), this is ~0.000025 units/year
  const STAR_STREAM_SPEED = 0.000025; // Units per year (scaled)
  const SOLAR_DRIFT_SPEED = 0; // Disabled - solar system stays centered

  // Helpers
  const Z_AXIS = new THREE.Vector3(0, 0, 1);

  function placePlanets(year: number) {
    // Calculate planet positions based on actual year
    // Each planet's angle = (year / period) * 2π
    let earthPosition = new THREE.Vector3();
    
    Object.entries(planetMeshes).forEach(([name, mesh]) => {
      const R = mesh.userData.radius as number;
      const period = mesh.userData.period as number;
      // Handle negative years and wrap properly
      let normalizedYear = year % period;
      if (normalizedYear < 0) normalizedYear += period;
      const theta = (normalizedYear / period) * Math.PI * 2;
      // base ecliptic (x,0,z), then tilt
      mesh.position.set(Math.cos(theta) * R, 0, Math.sin(theta) * R);
      mesh.position.applyAxisAngle(Z_AXIS, ECLIPTIC_TILT);
      
      // Store Earth position for Moon
      if (name === "Earth") {
        earthPosition.copy(mesh.position);
      }
    });
    
    // Position Moon orbiting Earth
    if (earthMeshRef.current) {
      // Moon's orbital phase (27.32 day period)
      let normalizedMoonYear = (year % MOON_PERIOD_YEARS);
      if (normalizedMoonYear < 0) normalizedMoonYear += MOON_PERIOD_YEARS;
      const moonTheta = (normalizedMoonYear / MOON_PERIOD_YEARS) * Math.PI * 2;
      
      // Moon orbits in Earth's orbital plane (ecliptic)
      const moonOffset = new THREE.Vector3(
        Math.cos(moonTheta) * MOON_ORBIT_RADIUS,
        0,
        Math.sin(moonTheta) * MOON_ORBIT_RADIUS
      );
      moonOffset.applyAxisAngle(Z_AXIS, ECLIPTIC_TILT);
      
      // Position moon group at Earth + Moon offset
      moonGroup.position.copy(earthPosition).add(moonOffset);
    }
  }

  function update(year: number, dir: Direction, enableMotion: boolean) {
    direction = dir;
    motionEnabled = enableMotion;

    // Smooth interpolation to target year (keeps scrub smooth)
    // When motion is enabled (auto-advancing), use year directly for responsive updates
    // When scrubbing/seeking, use interpolation for smooth transitions
    if (enableMotion) {
      currentYear = year; // Direct update for real-time animation
    } else {
      const alpha = 0.15;
      currentYear = currentYear + (year - currentYear) * alpha;
    }
    
    // Calculate star drift based on time progression
    // Stars stream past as heliosphere moves through galaxy
    // Use year difference from initial year (2024) to calculate drift
    const initialYear = 2024.0;
    const yearDelta = currentYear - initialYear;
    const starDriftX = yearDelta * STAR_STREAM_SPEED * direction;

    // Convert year to date for Voyager positioning
    const currentDate = new Date(Math.floor(currentYear), 0, 1);
    const jd = JulianDate.fromDate(currentDate);

    // Planet placement (planets orbit within heliosphere)
    placePlanets(currentYear);
    
    // Update Voyager positions
    try {
      const v1Data = VoyagerTrajectories.generateVoyager1Trajectory();
      const v2Data = VoyagerTrajectories.generateVoyager2Trajectory();
      
      // Voyager 1 position
      const v1Pos = v1Data.trajectory.position.interpolate(jd);
      voyager1.position.copy(v1Pos.multiplyScalar(0.03)); // Scale AU to scene units
      
      // Voyager 2 position
      const v2Pos = v2Data.trajectory.position.interpolate(jd);
      voyager2.position.copy(v2Pos.multiplyScalar(0.03));
      
      // Update trajectories
      const v1TrajPoints: THREE.Vector3[] = [];
      const v2TrajPoints: THREE.Vector3[] = [];
      
      // Generate trajectory points from launch to current date
      const launchJD = JulianDate.fromDate(v1Data.launch);
      const steps = 200;
      
      for (let i = 0; i <= steps; i++) {
        const t = i / steps;
        const trajJD = launchJD + t * (jd - launchJD);
        
        if (trajJD <= jd) {
          const v1p = v1Data.trajectory.position.interpolate(trajJD);
          v1TrajPoints.push(v1p.multiplyScalar(0.03));
          
          const v2p = v2Data.trajectory.position.interpolate(trajJD);
          v2TrajPoints.push(v2p.multiplyScalar(0.03));
        }
      }
      
      v1TrajGeometry.setFromPoints(v1TrajPoints);
      v2TrajGeometry.setFromPoints(v2TrajPoints);
    } catch (error) {
      console.warn('Could not update Voyager positions:', error);
    }
    
    // Update interstellar object positions
    const interstellarObjects = getAllInterstellarObjects();
    // Positions are already in AU, scale to scene units
    oumuamua.position.copy(interstellarObjects[0].currentPosition.clone().multiplyScalar(0.03));
    borisov.position.copy(interstellarObjects[1].currentPosition.clone().multiplyScalar(0.03));
    andromeda.position.copy(interstellarObjects[2].currentPosition.clone().multiplyScalar(0.03));
    
    // Update labels
    if (visibility.labels) {
      // Planet labels
      Object.entries(planetMeshes).forEach(([name, mesh]) => {
        const worldPos = new THREE.Vector3();
        mesh.getWorldPosition(worldPos);
        labelManager.createLabel(`planet-${name}`, {
          text: name,
          position: worldPos,
          offset: new THREE.Vector3(0, 0.3, 0),
          showDistance: true,
          fontSize: 11
        });
      });
      
      // Voyager labels
      const v1WorldPos = new THREE.Vector3();
      voyager1.getWorldPosition(v1WorldPos);
      labelManager.createLabel('voyager1', {
        text: 'Voyager 1',
        position: v1WorldPos,
        offset: new THREE.Vector3(0, 0.3, 0),
        color: '#00ff00',
        fontSize: 10
      });
      
      const v2WorldPos = new THREE.Vector3();
      voyager2.getWorldPosition(v2WorldPos);
      labelManager.createLabel('voyager2', {
        text: 'Voyager 2',
        position: v2WorldPos,
        offset: new THREE.Vector3(0, 0.3, 0),
        color: '#00ffff',
        fontSize: 10
      });
      
      // Boundary labels
      labelManager.createLabel('termination-shock', {
        text: 'Termination Shock',
        position: new THREE.Vector3(90 * 0.03, 0, 0),
        color: '#ffaa44',
        fontSize: 10
      });
      
      labelManager.createLabel('heliopause', {
        text: 'Heliopause',
        position: new THREE.Vector3(120 * 0.03, 0, 0),
        color: '#1a2a4e',
        fontSize: 10
      });
    } else {
      // Remove all labels
      Object.keys(planetMeshes).forEach(name => {
        labelManager.removeLabel(`planet-${name}`);
      });
      labelManager.removeLabel('voyager1');
      labelManager.removeLabel('voyager2');
      labelManager.removeLabel('termination-shock');
      labelManager.removeLabel('heliopause');
    }
    
    // Update label manager
    labelManager.update(camera);
    labelManager.updateVisibility(camera, 0.5, 50);

    // Continuous animation time (increments every frame for smooth pulsing)
    animationTime += 0.016; // ~60fps, increment by frame time
    
    // Update termination shock geometry more frequently (every 0.1 years for visible changes)
    // This creates visible fluctuations based on the 11-year solar cycle
    const updateThreshold = 0.1; // Update every 0.1 years
    if (Math.abs(currentYear - lastTSUpdateYear) >= updateThreshold) {
      createTerminationShock(currentYear);
    }
    
    // Update material properties EVERY FRAME for continuous animation
    // Keep opacity VERY LOW for aurora-like faint glow
    if (tsCoreMesh && tsCoreMesh.material) {
      // Solar cycle phase (11-year cycle)
      const solarCyclePhase = (currentYear % 11) / 11;
      const solarActivity = 0.7 + Math.sin(solarCyclePhase * Math.PI * 2) * 0.3;
      
      // Add continuous pulsing animation (independent of year, happens every frame)
      const pulsePhase = animationTime * 0.5; // Slow pulse
      const pulse = 1.0 + Math.sin(pulsePhase) * 0.2; // ±20% variation for visibility
      
      // Add faster sub-cycle variations
      const fastPulse = Math.sin(animationTime * 2.0) * 0.15; // Faster variation
      
      // VERY LOW base opacity - aurora-like faint glow (0.002 to 0.006)
      const baseOpacity = (0.002 + solarActivity * 0.004) * pulse;
      
      // Update all layers in the group with smooth opacity gradients
      terminationShockGroup.children.forEach((child, index) => {
        if (child instanceof THREE.Mesh && child.material) {
          const mat = child.material as THREE.MeshBasicMaterial;
          // Smooth opacity falloff - no sharp jumps
          const smoothOpacities = [
            baseOpacity * 1.0,   // Core
            baseOpacity * 0.95, 
            baseOpacity * 0.85,
            baseOpacity * 0.75,
            baseOpacity * 0.65,
            baseOpacity * 0.55,
            baseOpacity * 0.45,
            baseOpacity * 0.35,
            baseOpacity * 0.25,
            baseOpacity * 0.18,
            baseOpacity * 0.12,
            baseOpacity * 0.08  // Outer edge
          ];
          if (index < smoothOpacities.length) {
            mat.opacity = smoothOpacities[index] * (1.0 + fastPulse);
          }
        }
      });
      
      // Subtle scale pulsing for breathing effect
      const scalePulse = 1.0 + Math.sin(pulsePhase * 0.7) * 0.01; // ±1% scale variation (subtle)
      const layerScales = [1.0, 1.008, 1.016, 1.024, 1.032, 1.04, 1.048, 1.056, 1.064, 1.072, 1.08, 1.088];
      terminationShockGroup.children.forEach((child, index) => {
        if (child instanceof THREE.Mesh && index < layerScales.length) {
          child.scale.set(scalePulse * layerScales[index], scalePulse * layerScales[index], scalePulse * layerScales[index]);
        }
      });
    }

    // Animate solar wind streams with accurate physics
    if (motionEnabled) {
      // Much faster animation for realistic solar wind flow
      // Solar wind flows at ~350 km/s, so we need faster animation
      const timeScale = 0.05; // Animation speed multiplier
      let streamTime = (currentYear % 1) * timeScale * 10; // Faster flow
      
      // Calculate solar wind pressure variation (11-year solar cycle + shorter variations)
      // Solar wind pressure varies with solar activity
      const solarCyclePhase = (currentYear % 11) / 11; // 11-year solar cycle
      const shortTermVariation = Math.sin(currentYear * 0.5) * 0.3; // Shorter term variations
      const windPressure = 0.8 + solarCyclePhase * 0.4 + shortTermVariation; // Varies between 0.8 and 1.2
      
      // Update heliosphere shape based on solar wind pressure (regenerate MHD geometry periodically)
      // Only regenerate occasionally to avoid performance issues
      if (Math.random() < 0.01) { // 1% chance per frame
        const newJD = JulianDate.fromDate(new Date(Math.floor(currentYear), 0, 1));
        const newHPGeometry = heliosphereModel.generateParametricSurface(
          'heliopause',
          newJD,
          64
        );
        newHPGeometry.scale(0.03, 0.03, 0.03);
        
        helio.mesh.geometry.dispose();
        helio.mesh.geometry = newHPGeometry;
        
        const newGlowGeometry = newHPGeometry.clone();
        newGlowGeometry.scale(1.1, 1.1, 1.1);
        helio.glow.geometry.dispose();
        helio.glow.geometry = newGlowGeometry;
      }
      
      // Update heliosphere material opacity slightly based on pressure
      const helioMaterial = helio.mesh.material as THREE.MeshPhysicalMaterial;
      helioMaterial.opacity = 0.2 + windPressure * 0.1; // Slightly brighter with higher pressure
      
      solarWindStreams.forEach((stream) => {
        // Update stream points with accurate physics
        const segments = 60;
        const newPoints: THREE.Vector3[] = [];
        const maxDist = stream.heliopauseDist * 1.5;
        
        for (let j = 0; j <= segments; j++) {
          const t = j / segments;
          const distance = t * maxDist;
          
          // Add time-based flow offset - much faster for realistic solar wind speed
          const flowOffset = streamTime * 2.0; // Faster flow animation
          const effectiveDist = distance + flowOffset;
          
          // Add dynamic pressure variation to stream behavior
          const pressureMultiplier = 0.9 + windPressure * 0.2; // Affects stream density/visibility
          
          // Start from sun surface
          const basePos = stream.direction.clone().multiplyScalar(0.5 + effectiveDist);
          
          // Accurate physics: Different behavior in different regions
          // Apply pressure multiplier to distances (higher pressure = larger heliosphere)
          const pressureAdjustedTerminationDist = stream.terminationShockDist * pressureMultiplier;
          const pressureAdjustedHeliopauseDist = stream.heliopauseDist * pressureMultiplier;
          
          if (effectiveDist < pressureAdjustedTerminationDist) {
            // Inner heliosphere: supersonic, straight radial flow
            const velocity = solarWindVelocity(effectiveDist / 0.03) / HELIOSPHERIC_VELOCITIES.SOLAR_WIND_OUTER;
            basePos.multiplyScalar(1.0 + (1.0 - velocity) * 0.1);
            
            // Add subtle wave-like motion to show flow
            const wavePhase = streamTime * 3.0 + stream.age * Math.PI * 2;
            const waveAmplitude = Math.sin(wavePhase) * 0.02;
            const waveDir = new THREE.Vector3().crossVectors(stream.direction, noseDirection).normalize();
            if (waveDir.length() > 0.1) {
              basePos.add(waveDir.multiplyScalar(waveAmplitude));
            }
          } else if (effectiveDist < pressureAdjustedHeliopauseDist) {
            // Heliosheath: compressed, turbulent, subsonic
            const excessDist = effectiveDist - pressureAdjustedTerminationDist;
            const sheathWidth = pressureAdjustedHeliopauseDist - pressureAdjustedTerminationDist;
            const compression = 1.0 + (excessDist / sheathWidth) * 0.3;
            
            // More dynamic turbulent deflection in heliosheath
            const turbulencePhase = excessDist * 8.0 + streamTime * 5.0 + stream.age * Math.PI * 2;
            const turbulence = Math.sin(turbulencePhase) * 0.08 * (excessDist / sheathWidth) * windPressure;
            const perp = new THREE.Vector3().crossVectors(stream.direction, noseDirection).normalize();
            if (perp.length() > 0.1) {
              basePos.add(perp.multiplyScalar(turbulence));
            }
            
            basePos.multiplyScalar(compression);
          } else {
            // Beyond heliopause: curve due to ISM pressure and flow
            const excessDist = effectiveDist - pressureAdjustedHeliopauseDist;
            const curveAmount = Math.min(excessDist * 0.25, 2.0);
            
            // ISM flows from +X, deflects solar wind
            const ismFlowDir = new THREE.Vector3(-1, 0, 0);
            
            // Deflect perpendicular to both directions
            const perp1 = new THREE.Vector3().crossVectors(stream.direction, ismFlowDir).normalize();
            if (perp1.length() < 0.1) {
              perp1.set(-stream.direction.y, stream.direction.x, 0).normalize();
            }
            const perp2 = new THREE.Vector3().crossVectors(stream.direction, perp1).normalize();
            
            // Curved deflection with exponential decay and dynamic motion
            const deflectionStrength = curveAmount * Math.exp(-excessDist * 0.4);
            const phase = excessDist * 2.0 + streamTime * 2.0 + stream.age * Math.PI * 2;
            const deflection = perp1.clone().multiplyScalar(
              Math.sin(phase) * deflectionStrength
            ).add(
              perp2.clone().multiplyScalar(
                Math.cos(phase) * deflectionStrength * 0.7
              )
            );
            
            basePos.add(deflection);
          }
          
          newPoints.push(basePos);
        }
        
        // Update line geometry
        stream.line.geometry.setFromPoints(newPoints);
        stream.line.geometry.attributes.position.needsUpdate = true;
        
        // Update opacity based on density and wind pressure
        const density = solarWindDensity(newPoints[newPoints.length - 1].length() / 0.03);
        const baseOpacity = Math.min(0.7, Math.max(0.2, density / 10));
        const opacity = baseOpacity * (0.7 + windPressure * 0.3); // Brighter with higher pressure
        (stream.line.material as THREE.LineBasicMaterial).opacity = opacity;
      });
      
      // Animate interstellar wind particles - with proper deflection around heliosphere
      const ismWindPos = ismWindGeo.attributes.position.array as Float32Array;
      const ismSpeedMultiplier = 1.0 + windPressure * 0.2; // ISM flow affected by solar wind pressure
      
      // Heliosphere boundary distance (scaled)
      const heliopauseDist = 120 * 0.03; // ~120 AU scaled
      
      for (let i = 0; i < ismWindCount; i++) {
        const idx = i * 3;
        const pos = new THREE.Vector3(
          ismWindPos[idx],
          ismWindPos[idx + 1],
          ismWindPos[idx + 2]
        );
        
        // Transform position to heliosphere local frame for boundary checking
        const posLocal = pos.clone().applyMatrix4(apexBasis.clone().invert());
        
        // Calculate distance from heliosphere center
        const dist = pos.length();
        const distLocal = posLocal.length();
        
        // Update position with uniform flow (velocity is already in world space)
        const vel = new THREE.Vector3(
          ismWindVelocities[idx],
          ismWindVelocities[idx + 1],
          ismWindVelocities[idx + 2]
        );
        pos.add(vel.clone().multiplyScalar(ismSpeedMultiplier));
        
        // Transform updated position to local frame for boundary check
        const posLocalUpdated = pos.clone().applyMatrix4(apexBasis.clone().invert());
        const distLocalUpdated = posLocalUpdated.length();
        
        // Deflect around heliosphere if close (in local frame, deflect around +X axis)
        if (distLocalUpdated < heliopauseDist * 1.5) {
          // In heliosphere frame, ISM flows along -X, deflect perpendicular to radial
          const radialDirLocal = posLocalUpdated.clone().normalize();
          const flowDirLocal = new THREE.Vector3(-1, 0, 0); // ISM flows along -X in heliosphere frame
          const dot = radialDirLocal.dot(flowDirLocal);
          
          // Deflection perpendicular to radial direction (in local frame)
          if (dot > -0.5) { // Only deflect if approaching from upwind side
            const deflectionLocal = radialDirLocal.clone()
              .sub(flowDirLocal.clone().multiplyScalar(dot))
              .normalize();
            
            const deflectionStrength = Math.exp(-(distLocalUpdated - heliopauseDist) / (heliopauseDist * 0.2));
            const deflectionWorld = deflectionLocal.applyMatrix4(apexBasis);
            pos.add(deflectionWorld.multiplyScalar(deflectionStrength * 0.1));
          }
        }
        
        // Reset particles that have passed through heliosphere (check in local frame)
        // Reset if they've passed through the heliosphere (negative X in local frame) or gone too far
        const posLocalFinal = pos.clone().applyMatrix4(apexBasis.clone().invert());
        if (posLocalFinal.x < -5 || dist > 15) {
          // Reset to upstream position in heliosphere frame
          const spread = 8 + Math.random() * 4;
          const resetPosLocal = new THREE.Vector3(spread, (Math.random() - 0.5) * 6, (Math.random() - 0.5) * 6);
          // Transform back to world coordinates
          const resetPos = resetPosLocal.applyMatrix4(apexBasis);
          pos.copy(resetPos);
        }
        
        ismWindPos[idx] = pos.x;
        ismWindPos[idx + 1] = pos.y;
        ismWindPos[idx + 2] = pos.z;
      }
      ismWindGeo.attributes.position.needsUpdate = true;
      
      // Star drift is calculated based on year delta (see above)
      // Solar system drift disabled - heliosphere is fixed
      
      // Optional auto-panning (disabled by default - user controls camera)
      if (autoPanning) {
        cameraTime += cameraSpeed;
        const camAngle = cameraTime;
        camera.position.x = Math.cos(camAngle) * cameraRadius;
        camera.position.y = 2.2 + Math.sin(camAngle * 0.5) * 1.5;
        camera.position.z = Math.sin(camAngle) * cameraRadius;
        camera.lookAt(0, 0, 0);
      }
    }
    
    // Update camera controls (for smooth damping)
    controls.update();
    
    // Solar system stays centered (heliosphere is fixed reference frame)
    sol.position.set(driftX, 0, 0);
    
    // Stars stream past the fixed heliosphere (showing galactic motion)
    // Negative drift because stars move opposite to heliosphere motion
    stars.position.set(-starDriftX, 0, 0);
    famousStarsGroup.position.set(-starDriftX, 0, 0);
    
    // Heliosphere stays at origin (0,0,0) - completely fixed

    renderer.render(scene, camera);
  }

  function adjustCameraForViewport(aspect: number, isMobile: boolean = false) {
    const portraitFactor = THREE.MathUtils.clamp(1 - aspect, 0, 1);
    
    // More aggressive FOV adjustment for mobile portrait mode
    if (isMobile && portraitFactor > 0) {
      // Mobile portrait: wider FOV (75-80°) for better overview
      // Apply mobile optimization to all portrait orientations on mobile
      const mobilePortraitFactor = Math.min(portraitFactor * 2, 1); // Scale to full range
      camera.fov = THREE.MathUtils.lerp(75, 80, mobilePortraitFactor);
      // Position camera further back for better overview
      camera.position.set(
        defaultCameraPosition.x,
        defaultCameraPosition.y + portraitFactor * 1.2,
        defaultCameraPosition.z + portraitFactor * 6.5
      );
    } else {
      // Desktop or landscape: standard adjustment
      camera.fov = THREE.MathUtils.lerp(55, 68, portraitFactor);
      camera.position.set(
        defaultCameraPosition.x,
        defaultCameraPosition.y + portraitFactor * 0.8,
        defaultCameraPosition.z + portraitFactor * 4.5
      );
    }
    
    camera.updateProjectionMatrix();
    controls.minDistance = 3 + portraitFactor * 0.5;
    controls.maxDistance = 50 + portraitFactor * 5;
  }

  function resolvePixelRatio(aspect: number) {
    const runtimeDevicePixelRatio =
      typeof window !== 'undefined' ? window.devicePixelRatio ?? initialDevicePixelRatio : initialDevicePixelRatio;
    const cap = aspect < 1 ? 1.6 : 2.2;
    return Math.min(runtimeDevicePixelRatio, cap);
  }

  function resize(w: number, h: number) {
    const safeWidth = Math.max(1, w);
    const safeHeight = Math.max(1, h);
    renderer.setSize(safeWidth, safeHeight, false);
    const aspect = safeWidth / safeHeight;
    camera.aspect = aspect;
    const currentIsMobile = safeWidth <= 768;
    adjustCameraForViewport(aspect, currentIsMobile);
    const targetPixelRatio = resolvePixelRatio(aspect);
    if (Math.abs(renderer.getPixelRatio() - targetPixelRatio) > 0.01) {
      renderer.setPixelRatio(targetPixelRatio);
    }
    labelManager.resize(safeWidth, safeHeight);
  }

  function dispose() {
    controls.dispose();
    renderer.dispose();
    starGeo.dispose();
    starMat.dispose();
    labelManager.dispose();
  }
  
  // Toggle component visibility (astronomer controls)
  function toggleComponent(component: keyof ComponentVisibility, visible: boolean) {
    visibility[component] = visible;
    
    switch(component) {
      case 'heliosphere':
        helio.mesh.visible = visible;
        break;
      case 'helioglow':
        helio.glow.visible = visible;
        break;
      case 'terminationShock':
        terminationShockGroup.visible = visible;
        break;
      case 'bowShock':
        bowShockGroup.visible = visible;
        break;
      case 'solarWind':
        solarWindGroup.visible = visible;
        break;
      case 'interstellarWind':
        ismWind.visible = visible;
        break;
      case 'planets':
        planetsGroup.visible = visible;
        break;
      case 'orbits':
        orbitsGroup.visible = visible;
        break;
      case 'moon':
        moonGroup.visible = visible;
        break;
      case 'stars':
        stars.visible = visible;
        break;
      case 'famousStars':
        famousStarsGroup.visible = visible;
        break;
      case 'voyagers':
        voyagerGroup.visible = visible;
        break;
      case 'distanceMarkers':
        distanceMarkersGroup.visible = visible;
        break;
      case 'solarApex':
        solarApexGroup.visible = visible;
        break;
      case 'labels':
        labelManager.setVisible(visible);
        break;
      case 'interstellarObjects':
        interstellarObjectsGroup.visible = visible;
        break;
      case 'constellations':
        constellationsGroup.visible = visible;
        break;
    }
  }
  
  function getVisibility(): ComponentVisibility {
    return { ...visibility };
  }

  return { canvas, update, resize, dispose, toggleComponent, getVisibility };
}
