/**
 * Parametric heliosphere surface generation
 * Based on MHD models and observational constraints
 */

import { AU, Units, Radians } from '../types/units';
import { PositionAU } from '../types/vectors';

/**
 * Heliosphere morphology types
 */
export enum HeliosphereMorphology {
  COMETARY = 'cometary',      // Classic comet-like tail
  CROISSANT = 'croissant',    // Flattened, croissant-shaped
  BUBBLE = 'bubble',          // Nearly spherical (rare)
}

/**
 * Parameters for heliosphere shape at a given epoch
 * These come from precomputed dataset
 */
export interface HeliosphereParameters {
  // Nose radius (upwind direction) in AU
  R_HP_nose: AU;

  // Termination shock to heliopause ratio (typically 0.75-0.85)
  R_TS_over_HP: number;

  // Direction of ISM inflow (unit vector in HEE_J2000)
  nose_vec: [number, number, number];

  // ISM conditions
  ISM_rho: number;  // density (particles/cm³)
  ISM_T: number;    // temperature (K)
  ISM_B: number;    // magnetic field strength (nT)

  // Solar wind conditions
  SW_Mdot: number;  // mass loss rate (proxy)
  SW_v: number;     // wind speed (km/s)

  // Morphology
  morphology: HeliosphereMorphology;

  // Shape coefficients (morphology-dependent)
  shape_params: number[];
}

/**
 * Heliosphere surface generator
 */
export class HeliosphereSurface {
  private params: HeliosphereParameters;

  constructor(params: HeliosphereParameters) {
    this.params = params;
  }

  /**
   * Calculate heliopause radius at given spherical angles
   * @param theta - Polar angle (0 at +Z, π at -Z)
   * @param phi - Azimuthal angle (0 at +X, π/2 at +Y)
   * @returns Radius in AU
   */
  heliopauseRadius(theta: Radians, phi: Radians): AU {
    const thetaRad = theta as number;
    const phiRad = phi as number;

    // Cosine of angle from nose direction
    // Nose is along -nose_vec (ISM flows toward Sun)
    const [nx, ny, nz] = this.params.nose_vec;
    const sinTheta = Math.sin(thetaRad);
    const cosTheta = Math.cos(thetaRad);
    const cosPhi = Math.cos(phiRad);
    const sinPhi = Math.sin(phiRad);

    // Direction vector in Cartesian
    const dx = sinTheta * cosPhi;
    const dy = sinTheta * sinPhi;
    const dz = cosTheta;

    // Angle from nose (0 = upwind, π = downwind)
    const cosAlpha = -(dx * nx + dy * ny + dz * nz);
    const alpha = Math.acos(Math.max(-1, Math.min(1, cosAlpha)));

    // Base radius (nose)
    const R_nose = this.params.R_HP_nose as number;

    // Apply morphology-specific shape
    let radius: number;

    switch (this.params.morphology) {
      case HeliosphereMorphology.COMETARY:
        radius = this.cometaryShape(alpha, R_nose);
        break;

      case HeliosphereMorphology.CROISSANT:
        radius = this.croissantShape(alpha, R_nose, thetaRad);
        break;

      case HeliosphereMorphology.BUBBLE:
        radius = this.bubbleShape(alpha, R_nose);
        break;

      default:
        radius = this.cometaryShape(alpha, R_nose);
    }

    return Units.AU(radius);
  }

  /**
   * Calculate termination shock radius at given angles
   */
  terminationShockRadius(theta: Radians, phi: Radians): AU {
    const hpRadius = this.heliopauseRadius(theta, phi) as number;
    return Units.AU(hpRadius * this.params.R_TS_over_HP);
  }

  /**
   * Cometary (classic) shape model
   * Elongated tail with smooth taper
   */
  private cometaryShape(alpha: number, R_nose: number): number {
    // Shape parameters
    const [a0, a1, a2] = this.params.shape_params.length >= 3 
      ? this.params.shape_params 
      : [1.0, 2.5, 0.5];

    // Polynomial model: R(α) = R_nose * (a0 + a1*cos(α) + a2*cos²(α))
    const cosAlpha = Math.cos(alpha);
    const factor = a0 + a1 * cosAlpha + a2 * cosAlpha * cosAlpha;

    return R_nose * Math.max(0.1, factor);
  }

  /**
   * Croissant shape (Opher et al. 2020 model)
   * Flattened with bifurcated tail
   */
  private croissantShape(alpha: number, R_nose: number, theta: number): number {
    // Shape parameters: [asymmetry, flattening, tail_spread]
    const [asymmetry, flattening, tailSpread] = this.params.shape_params.length >= 3
      ? this.params.shape_params
      : [1.5, 0.7, 0.3];

    // Base cometary shape
    const baseRadius = this.cometaryShape(alpha, R_nose);

    // Apply flattening perpendicular to ecliptic
    // theta = 0 (north) and theta = π (south) are compressed
    const latitudeFactor = 1.0 - flattening * Math.sin(theta) ** 2;

    // Tail spreading (creates bifurcation)
    const tailFactor = alpha > Math.PI / 2 
      ? 1.0 + tailSpread * Math.sin(2 * (alpha - Math.PI / 2))
      : 1.0;

    return baseRadius * asymmetry * latitudeFactor * tailFactor;
  }

  /**
   * Bubble shape (nearly spherical)
   * Used during solar minimum or future solar evolution
   */
  private bubbleShape(alpha: number, R_nose: number): number {
    // Shape parameter: asphericity (0 = perfect sphere)
    const asphericity = this.params.shape_params[0] ?? 0.1;

    // Slight asymmetry
    const factor = 1.0 + asphericity * Math.cos(alpha);

    return R_nose * factor;
  }

  /**
   * Generate mesh vertices for surface
   * @param thetaSteps - Number of steps in polar direction
   * @param phiSteps - Number of steps in azimuthal direction
   * @param surfaceType - 'heliopause' or 'termination_shock'
   */
  generateMesh(
    thetaSteps: number,
    phiSteps: number,
    surfaceType: 'heliopause' | 'termination_shock' = 'heliopause'
  ): {
    positions: Float32Array;
    indices: Uint32Array;
    normals: Float32Array;
  } {
    const radiusFunc = surfaceType === 'heliopause' 
      ? this.heliopauseRadius.bind(this)
      : this.terminationShockRadius.bind(this);

    const vertices: number[] = [];
    const normals: number[] = [];
    const indices: number[] = [];

    // Generate vertices
    for (let i = 0; i <= thetaSteps; i++) {
      const theta = Units.Radians((i / thetaSteps) * Math.PI);

      for (let j = 0; j <= phiSteps; j++) {
        const phi = Units.Radians((j / phiSteps) * 2 * Math.PI);

        const r = radiusFunc(theta, phi) as number;
        const thetaRad = theta as number;
        const phiRad = phi as number;

        // Spherical to Cartesian (in HEE_J2000)
        const sinTheta = Math.sin(thetaRad);
        const x = r * sinTheta * Math.cos(phiRad);
        const y = r * sinTheta * Math.sin(phiRad);
        const z = r * Math.cos(thetaRad);

        vertices.push(x, y, z);

        // Compute normal (approximate via small finite differences)
        const epsilon = 0.01;
        const r1 = radiusFunc(
          Units.Radians(thetaRad + epsilon),
          phi
        ) as number;
        const r2 = radiusFunc(
          theta,
          Units.Radians(phiRad + epsilon)
        ) as number;

        const dx_dtheta = (r1 - r) / epsilon;
        const dx_dphi = (r2 - r) / epsilon;

        // Simplified normal (could be more accurate)
        const nx = Math.cos(thetaRad) * Math.cos(phiRad);
        const ny = Math.cos(thetaRad) * Math.sin(phiRad);
        const nz = -Math.sin(thetaRad);

        const nLen = Math.sqrt(nx * nx + ny * ny + nz * nz);
        normals.push(nx / nLen, ny / nLen, nz / nLen);
      }
    }

    // Generate indices (triangle strips)
    for (let i = 0; i < thetaSteps; i++) {
      for (let j = 0; j < phiSteps; j++) {
        const a = i * (phiSteps + 1) + j;
        const b = a + phiSteps + 1;

        // Two triangles per quad
        indices.push(a, b, a + 1);
        indices.push(b, b + 1, a + 1);
      }
    }

    return {
      positions: new Float32Array(vertices),
      indices: new Uint32Array(indices),
      normals: new Float32Array(normals),
    };
  }

  /**
   * Sample point on surface (for particle emission)
   */
  samplePoint(
    theta: Radians,
    phi: Radians,
    surfaceType: 'heliopause' | 'termination_shock' = 'heliopause'
  ): PositionAU {
    const radiusFunc = surfaceType === 'heliopause'
      ? this.heliopauseRadius.bind(this)
      : this.terminationShockRadius.bind(this);

    const r = radiusFunc(theta, phi) as number;
    const thetaRad = theta as number;
    const phiRad = phi as number;

    const sinTheta = Math.sin(thetaRad);

    return {
      x: Units.AU(r * sinTheta * Math.cos(phiRad)),
      y: Units.AU(r * sinTheta * Math.sin(phiRad)),
      z: Units.AU(r * Math.cos(thetaRad)),
    };
  }

  /**
   * Update parameters (for interpolation between epochs)
   */
  updateParameters(params: HeliosphereParameters): void {
    this.params = params;
  }

  /**
   * Get current parameters
   */
  getParameters(): HeliosphereParameters {
    return { ...this.params };
  }
}

/**
 * Interpolate between two parameter sets
 */
export function interpolateParameters(
  params0: HeliosphereParameters,
  params1: HeliosphereParameters,
  t: number
): HeliosphereParameters {
  // Clamp t
  t = Math.max(0, Math.min(1, t));

  // Interpolate scalar values
  const R_HP_nose = Units.AU(
    (params0.R_HP_nose as number) * (1 - t) + (params1.R_HP_nose as number) * t
  );

  const R_TS_over_HP = params0.R_TS_over_HP * (1 - t) + params1.R_TS_over_HP * t;

  // Interpolate and renormalize nose vector
  const nose_vec: [number, number, number] = [
    params0.nose_vec[0] * (1 - t) + params1.nose_vec[0] * t,
    params0.nose_vec[1] * (1 - t) + params1.nose_vec[1] * t,
    params0.nose_vec[2] * (1 - t) + params1.nose_vec[2] * t,
  ];
  const noseLen = Math.sqrt(nose_vec[0] ** 2 + nose_vec[1] ** 2 + nose_vec[2] ** 2);
  nose_vec[0] /= noseLen;
  nose_vec[1] /= noseLen;
  nose_vec[2] /= noseLen;

  // Interpolate ISM and solar wind parameters
  const ISM_rho = params0.ISM_rho * (1 - t) + params1.ISM_rho * t;
  const ISM_T = params0.ISM_T * (1 - t) + params1.ISM_T * t;
  const ISM_B = params0.ISM_B * (1 - t) + params1.ISM_B * t;
  const SW_Mdot = params0.SW_Mdot * (1 - t) + params1.SW_Mdot * t;
  const SW_v = params0.SW_v * (1 - t) + params1.SW_v * t;

  // Morphology (snap at t=0.5)
  const morphology = t < 0.5 ? params0.morphology : params1.morphology;

  // Interpolate shape parameters
  const maxParams = Math.max(params0.shape_params.length, params1.shape_params.length);
  const shape_params: number[] = [];
  for (let i = 0; i < maxParams; i++) {
    const p0 = params0.shape_params[i] ?? 0;
    const p1 = params1.shape_params[i] ?? 0;
    shape_params.push(p0 * (1 - t) + p1 * t);
  }

  return {
    R_HP_nose,
    R_TS_over_HP,
    nose_vec,
    ISM_rho,
    ISM_T,
    ISM_B,
    SW_Mdot,
    SW_v,
    morphology,
    shape_params,
  };
}

