/**
 * Physically accurate plasma shader for heliosphere visualization
 * Based on actual solar wind plasma physics and emission mechanisms
 * 
 * Key Physics:
 * 1. Thermal Bremsstrahlung: Free-free emission from electron-ion interactions
 * 2. Cyclotron Radiation: Charged particles spiraling in magnetic fields
 * 3. Shock Heating: Enhanced emission at termination shock boundary
 * 4. Charge Exchange: Neutral atoms created by proton-neutral collisions
 */

import * as THREE from 'three';

export const PlasmaVertexShader = `
  varying vec3 vPosition;
  varying vec3 vNormal;
  varying float vDistance;
  varying vec3 vViewPosition;
  
  uniform float time;
  uniform float solarCyclePhase;
  
  void main() {
    vPosition = position;
    vNormal = normalize(normalMatrix * normal);
    vDistance = length(position);
    
    // Add subtle solar wind streaming effect
    vec3 streamOffset = vec3(0.0);
    float streamSpeed = 0.1 + 0.05 * sin(solarCyclePhase * 6.28318);
    streamOffset.x = sin(time * streamSpeed + position.y * 0.1) * 0.5;
    streamOffset.y = cos(time * streamSpeed * 1.3 + position.x * 0.1) * 0.3;
    
    vec4 mvPosition = modelViewMatrix * vec4(position + streamOffset, 1.0);
    vViewPosition = mvPosition.xyz;
    
    gl_Position = projectionMatrix * mvPosition;
  }
`;

export const PlasmaFragmentShader = `
  uniform vec3 baseColor;
  uniform float opacity;
  uniform float time;
  uniform float solarCyclePhase;
  uniform vec3 sunPosition;
  uniform float plasmaTemperature; // in eV
  uniform float plasmaDensity; // particles/cm³
  uniform float magneticFieldStrength; // nT
  uniform float shockCompression; // Compression ratio at shock
  
  varying vec3 vPosition;
  varying vec3 vNormal;
  varying float vDistance;
  varying vec3 vViewPosition;
  
  // Physical constants
  const float BOLTZMANN = 8.617e-5; // eV/K
  const float ELECTRON_MASS = 0.511e6; // eV/c²
  
  // Calculate thermal bremsstrahlung emission
  float bremsstrahlungEmission(float density, float temperature) {
    // Simplified bremsstrahlung emissivity ∝ n² * sqrt(T)
    // Using normalized units for visualization
    float sqrtTemp = sqrt(temperature);
    float emissivity = density * density * sqrtTemp * 0.0001;
    return clamp(emissivity, 0.0, 1.0);
  }
  
  // Calculate cyclotron/synchrotron emission
  float cyclotronEmission(float bField, float temperature) {
    // Cyclotron frequency ∝ B, emission ∝ B² * T
    // Normalized for visualization
    float cyclotronPower = bField * bField * temperature * 0.00001;
    return clamp(cyclotronPower, 0.0, 1.0);
  }
  
  // Shock heating enhancement
  float shockEnhancement(vec3 position, vec3 normal) {
    // Enhanced emission at shock front where compression occurs
    float compressionFactor = shockCompression;
    
    // Calculate gradient (shock front detection)
    float gradient = abs(dot(normalize(position), normal));
    float shockStrength = smoothstep(0.7, 0.95, gradient);
    
    // Temperature increases with compression (adiabatic heating)
    float temperatureBoost = pow(compressionFactor, 0.67); // γ = 5/3 for plasma
    
    return mix(1.0, temperatureBoost, shockStrength);
  }
  
  // Charge exchange glow (ENA emission)
  float chargeExchangeGlow(vec3 position, float density) {
    // Energetic Neutral Atom emission from charge exchange
    // Peaks where solar wind meets ISM
    float distance = length(position);
    float heliopauseRegion = smoothstep(100.0, 130.0, distance) * smoothstep(150.0, 120.0, distance);
    
    // ENA flux ∝ solar wind density * ISM density
    float enaEmission = density * 0.1 * heliopauseRegion; // ISM density ~0.1 cm⁻³
    
    return clamp(enaEmission * 0.01, 0.0, 1.0);
  }
  
  // Parker spiral magnetic field structure
  vec3 parkerSpiralField(vec3 position) {
    float r = length(position);
    float theta = atan(position.y, position.x);
    
    // Spiral angle increases with distance
    float spiralAngle = theta - r * 0.05;
    
    // Field strength decreases with distance
    float fieldStrength = magneticFieldStrength / (r * r + 1.0);
    
    vec3 field = vec3(
      cos(spiralAngle) * fieldStrength,
      sin(spiralAngle) * fieldStrength,
      position.z * 0.1 * fieldStrength / (r + 1.0)
    );
    
    return field;
  }
  
  // Main plasma emission calculation
  void main() {
    // Distance-based parameters
    float r = vDistance;
    float invR2 = 1.0 / (r * r + 1.0);
    
    // Local plasma parameters (vary with solar cycle and distance)
    float localDensity = plasmaDensity * invR2 * (1.0 + 0.3 * sin(solarCyclePhase * 6.28318));
    float localTemp = plasmaTemperature * pow(invR2, 0.4); // Temperature drops slower than density
    vec3 localBField = parkerSpiralField(vPosition);
    float localBStrength = length(localBField);
    
    // Calculate emission components
    float thermal = bremsstrahlungEmission(localDensity, localTemp);
    float cyclotron = cyclotronEmission(localBStrength, localTemp);
    float shock = shockEnhancement(vPosition, vNormal);
    float ena = chargeExchangeGlow(vPosition, localDensity);
    
    // Combine emission mechanisms
    float totalEmission = thermal + cyclotron * 0.5 + ena * 2.0;
    totalEmission *= shock;
    
    // Add subtle time variation (solar wind fluctuations)
    float fluctuation = 1.0 + 0.1 * sin(time * 0.5 + r * 0.1);
    totalEmission *= fluctuation;
    
    // Apply view-dependent effects (limb brightening for optically thin plasma)
    float viewAngle = abs(dot(normalize(vViewPosition), vNormal));
    float limbBrightening = 1.0 + 0.3 * (1.0 - viewAngle);
    totalEmission *= limbBrightening;
    
    // Color based on plasma temperature and emission mechanism
    vec3 color = baseColor;
    
    // Hot plasma appears bluer (Wien's law)
    float tempColor = clamp(localTemp / 100.0, 0.0, 1.0);
    color = mix(vec3(1.0, 0.6, 0.2), vec3(0.6, 0.8, 1.0), tempColor);
    
    // ENA emission has distinct color (greenish)
    color = mix(color, vec3(0.4, 1.0, 0.6), ena * 0.5);
    
    // Final color with emission
    vec3 finalColor = color * totalEmission;
    
    // Ultra-low opacity for subtle, realistic appearance
    float finalOpacity = opacity * totalEmission * 0.1;
    finalOpacity = clamp(finalOpacity, 0.0, 0.01); // Max 1% opacity
    
    gl_FragColor = vec4(finalColor, finalOpacity);
  }
`;

/**
 * Create plasma material with physical parameters
 */
export function createPlasmaMaterial(params: {
  baseColor?: THREE.Color;
  opacity?: number;
  plasmaTemperature?: number; // eV
  plasmaDensity?: number; // particles/cm³
  magneticFieldStrength?: number; // nT
  shockCompression?: number;
  transparent?: boolean;
  side?: THREE.Side;
}): THREE.ShaderMaterial {
  return new THREE.ShaderMaterial({
    uniforms: {
      baseColor: { value: params.baseColor || new THREE.Color(0xff8844) },
      opacity: { value: params.opacity || 0.002 },
      time: { value: 0 },
      solarCyclePhase: { value: 0 },
      sunPosition: { value: new THREE.Vector3(0, 0, 0) },
      plasmaTemperature: { value: params.plasmaTemperature || 10 }, // 10 eV typical at termination shock
      plasmaDensity: { value: params.plasmaDensity || 0.01 }, // 0.01 cm⁻³ at 100 AU
      magneticFieldStrength: { value: params.magneticFieldStrength || 0.1 }, // 0.1 nT at 100 AU
      shockCompression: { value: params.shockCompression || 2.5 }
    },
    vertexShader: PlasmaVertexShader,
    fragmentShader: PlasmaFragmentShader,
    transparent: params.transparent !== false,
    side: params.side || THREE.DoubleSide,
    depthWrite: false,
    blending: THREE.AdditiveBlending
  });
}

/**
 * Update plasma material uniforms
 */
export function updatePlasmaMaterial(
  material: THREE.ShaderMaterial,
  time: number,
  solarCyclePhase: number
): void {
  if (material.uniforms.time) {
    material.uniforms.time.value = time;
  }
  if (material.uniforms.solarCyclePhase) {
    material.uniforms.solarCyclePhase.value = solarCyclePhase;
  }
}
