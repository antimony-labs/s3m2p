/**
 * Dataset loader for precomputed heliosphere data
 * Loads Zarr arrays via HTTP range requests
 */

import { JulianDate, MyrSinceZAMS, AU, Units } from '../types/units';
import { HeliosphereParameters, HeliosphereMorphology } from '../physics/HeliosphereSurface';

/**
 * Dataset metadata
 */
export interface DatasetMetadata {
  version: string;
  created: string;
  units: {
    distance: 'AU';
    velocity: 'km/s';
    time: 'JulianDate' | 'MyrSinceZAMS' | 'GyrSinceZAMS';
  };
  provenance: {
    solar_model: string;
    ism_model: string;
    generator_version: string;
  };
  time_axis: {
    n_epochs: number;
    t_min: number;
    t_max: number;
    epoch_file: string;
  };
}

/**
 * Epoch data loaded from dataset
 */
export interface EpochData {
  time: JulianDate | MyrSinceZAMS;
  parameters: HeliosphereParameters;
}

/**
 * Simple LRU cache for epoch data
 */
class EpochCache {
  private cache: Map<number, EpochData>;
  private maxSize: number;
  private accessOrder: number[];

  constructor(maxSize: number = 16) {
    this.cache = new Map();
    this.maxSize = maxSize;
    this.accessOrder = [];
  }

  get(index: number): EpochData | null {
    const data = this.cache.get(index);
    if (data) {
      // Update access order (move to end)
      const idx = this.accessOrder.indexOf(index);
      if (idx !== -1) {
        this.accessOrder.splice(idx, 1);
      }
      this.accessOrder.push(index);
    }
    return data ?? null;
  }

  set(index: number, data: EpochData): void {
    if (this.cache.size >= this.maxSize && !this.cache.has(index)) {
      // Evict least recently used
      const lru = this.accessOrder.shift();
      if (lru !== undefined) {
        this.cache.delete(lru);
      }
    }

    this.cache.set(index, data);
    this.accessOrder.push(index);
  }

  clear(): void {
    this.cache.clear();
    this.accessOrder = [];
  }
}

/**
 * Dataset loader with HTTP range-based streaming
 */
export class DatasetLoader {
  private baseUrl: string;
  private metadata: DatasetMetadata | null = null;
  private epochs: Float64Array | null = null;
  private cache: EpochCache;
  private loading: Map<number, Promise<EpochData>>;

  constructor(baseUrl: string = '/dataset') {
    this.baseUrl = baseUrl;
    this.cache = new EpochCache(16);
    this.loading = new Map();
  }

  /**
   * Initialize loader (fetch metadata and epoch list)
   */
  async initialize(): Promise<void> {
    // Load metadata
    const metaResponse = await fetch(`${this.baseUrl}/meta.json`);
    if (!metaResponse.ok) {
      throw new Error(`Failed to load metadata: ${metaResponse.statusText}`);
    }
    this.metadata = await metaResponse.json();

    // Load epoch array (time axis)
    // In real implementation, this would load from Zarr
    // For now, use a simple JSON fallback
    const epochPath = this.metadata?.time_axis?.epoch_file || 'time/epochs.json';
    const epochResponse = await fetch(`${this.baseUrl}/${epochPath}`);
    if (!epochResponse.ok) {
      throw new Error(`Failed to load epochs: ${epochResponse.statusText}`);
    }

    const epochData = await epochResponse.json();
    this.epochs = new Float64Array(epochData);
  }

  /**
   * Get metadata
   */
  getMetadata(): DatasetMetadata {
    if (!this.metadata) {
      throw new Error('Dataset not initialized. Call initialize() first.');
    }
    return this.metadata;
  }

  /**
   * Find closest epoch indices for interpolation
   * @returns [index0, index1, alpha] where alpha is interpolation factor
   */
  findEpochBracket(time: JulianDate | MyrSinceZAMS | number): [number, number, number] {
    if (!this.epochs) {
      throw new Error('Dataset not initialized');
    }

    // Handle GyrSinceZAMS (convert to Myr for comparison)
    let t: number;
    if (this.metadata?.units?.time === 'GyrSinceZAMS') {
      // If time is in Gyr, convert to same units as epochs array
      t = typeof time === 'number' ? time : (time as number);
    } else {
      t = time as number;
    }
    const n = this.epochs.length;

    // Handle edge cases
    if (t <= this.epochs[0]) return [0, 0, 0];
    if (t >= this.epochs[n - 1]) return [n - 1, n - 1, 0];

    // Binary search
    let low = 0;
    let high = n - 1;

    while (high - low > 1) {
      const mid = Math.floor((low + high) / 2);
      if (this.epochs[mid] <= t) {
        low = mid;
      } else {
        high = mid;
      }
    }

    // Linear interpolation factor
    const t0 = this.epochs[low];
    const t1 = this.epochs[high];
    const alpha = (t - t0) / (t1 - t0);

    return [low, high, alpha];
  }

  /**
   * Load parameters for a specific epoch index
   */
  async loadEpoch(index: number): Promise<EpochData> {
    // Check cache
    const cached = this.cache.get(index);
    if (cached) return cached;

    // Check if already loading
    const inProgress = this.loading.get(index);
    if (inProgress) return inProgress;

    // Start loading
    const promise = this._loadEpochData(index);
    this.loading.set(index, promise);

    try {
      const data = await promise;
      this.cache.set(index, data);
      return data;
    } finally {
      this.loading.delete(index);
    }
  }

  /**
   * Internal: actually load epoch data from storage
   */
  private async _loadEpochData(index: number): Promise<EpochData> {
    if (!this.epochs || !this.metadata) {
      throw new Error('Dataset not initialized');
    }

    // In a full implementation, this would:
    // 1. Use HTTP range requests to fetch specific Zarr chunks
    // 2. Decompress if needed
    // 3. Parse binary data
    //
    // For now, simulate with JSON files per epoch
    const epochFile = `${this.baseUrl}/heliosphere/epoch_${index.toString().padStart(6, '0')}.json`;
    
    const response = await fetch(epochFile);
    if (!response.ok) {
      // Fallback: generate synthetic parameters
      console.warn(`Epoch ${index} not found, using fallback`);
      return this._generateFallbackEpoch(index);
    }

    const raw = await response.json();

    // Parse parameters
    const parameters: HeliosphereParameters = {
      R_HP_nose: Units.AU(raw.R_HP_nose),
      R_TS_over_HP: raw.R_TS_over_HP,
      nose_vec: raw.nose_vec,
      ISM_rho: raw.ISM_rho,
      ISM_T: raw.ISM_T,
      ISM_B: raw.ISM_B,
      SW_Mdot: raw.SW_Mdot,
      SW_v: raw.SW_v,
      morphology: raw.morphology as HeliosphereMorphology,
      shape_params: raw.shape_params,
    };

    const time = this.metadata.units.time === 'JulianDate'
      ? Units.JulianDate(this.epochs[index])
      : Units.MyrSinceZAMS(this.epochs[index]);

    return { time, parameters };
  }

  /**
   * Generate fallback parameters (for development/testing)
   */
  private _generateFallbackEpoch(index: number): EpochData {
    if (!this.epochs || !this.metadata) {
      throw new Error('Dataset not initialized');
    }

    // Simple interpolation of present-day values
    const time = this.metadata.units.time === 'JulianDate'
      ? Units.JulianDate(this.epochs[index])
      : Units.MyrSinceZAMS(this.epochs[index]);

    const parameters: HeliosphereParameters = {
      R_HP_nose: Units.AU(121.0), // Voyager 1 crossing
      R_TS_over_HP: 0.77,
      nose_vec: [-0.93, -0.26, 0.26], // Approximate IBEX direction
      ISM_rho: 0.1,
      ISM_T: 6300,
      ISM_B: 0.3,
      SW_Mdot: 1.0,
      SW_v: 400,
      morphology: HeliosphereMorphology.COMETARY,
      shape_params: [1.0, 2.5, 0.5],
    };

    return { time, parameters };
  }

  /**
   * Load and interpolate parameters for a given time
   */
  async loadParametersAt(time: JulianDate | MyrSinceZAMS | number): Promise<HeliosphereParameters> {
    const [i0, i1, alpha] = this.findEpochBracket(time);

    // Load both epochs
    const [epoch0, epoch1] = await Promise.all([
      this.loadEpoch(i0),
      this.loadEpoch(i1),
    ]);

    // If same epoch, no interpolation needed
    if (i0 === i1) {
      return epoch0.parameters;
    }

    // Interpolate
    return this._interpolateParameters(epoch0.parameters, epoch1.parameters, alpha);
  }

  /**
   * Interpolate between two parameter sets
   */
  private _interpolateParameters(
    p0: HeliosphereParameters,
    p1: HeliosphereParameters,
    alpha: number
  ): HeliosphereParameters {
    // Simple linear interpolation
    const R_HP_nose = Units.AU(
      (p0.R_HP_nose as number) * (1 - alpha) + (p1.R_HP_nose as number) * alpha
    );

    const R_TS_over_HP = p0.R_TS_over_HP * (1 - alpha) + p1.R_TS_over_HP * alpha;

    // Interpolate and renormalize nose vector
    const nose_vec: [number, number, number] = [
      p0.nose_vec[0] * (1 - alpha) + p1.nose_vec[0] * alpha,
      p0.nose_vec[1] * (1 - alpha) + p1.nose_vec[1] * alpha,
      p0.nose_vec[2] * (1 - alpha) + p1.nose_vec[2] * alpha,
    ];
    const noseLen = Math.sqrt(nose_vec[0] ** 2 + nose_vec[1] ** 2 + nose_vec[2] ** 2);
    nose_vec[0] /= noseLen;
    nose_vec[1] /= noseLen;
    nose_vec[2] /= noseLen;

    const ISM_rho = p0.ISM_rho * (1 - alpha) + p1.ISM_rho * alpha;
    const ISM_T = p0.ISM_T * (1 - alpha) + p1.ISM_T * alpha;
    const ISM_B = p0.ISM_B * (1 - alpha) + p1.ISM_B * alpha;
    const SW_Mdot = p0.SW_Mdot * (1 - alpha) + p1.SW_Mdot * alpha;
    const SW_v = p0.SW_v * (1 - alpha) + p1.SW_v * alpha;

    // Morphology: snap at 0.5
    const morphology = alpha < 0.5 ? p0.morphology : p1.morphology;

    // Shape params
    const maxLen = Math.max(p0.shape_params.length, p1.shape_params.length);
    const shape_params: number[] = [];
    for (let i = 0; i < maxLen; i++) {
      const v0 = p0.shape_params[i] ?? 0;
      const v1 = p1.shape_params[i] ?? 0;
      shape_params.push(v0 * (1 - alpha) + v1 * alpha);
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

  /**
   * Prefetch epochs for smooth playback
   */
  async prefetch(time: JulianDate | MyrSinceZAMS, lookahead: number = 4): Promise<void> {
    const [i0] = this.findEpochBracket(time);
    
    const promises: Promise<EpochData>[] = [];
    for (let i = i0; i < Math.min(i0 + lookahead, this.epochs?.length ?? 0); i++) {
      promises.push(this.loadEpoch(i));
    }

    await Promise.all(promises);
  }

  /**
   * Clear cache
   */
  clearCache(): void {
    this.cache.clear();
  }
}

/**
 * Global dataset loader instance
 */
let globalLoader: DatasetLoader | null = null;

/**
 * Get or create global loader
 */
export function getDatasetLoader(baseUrl?: string): DatasetLoader {
  if (!globalLoader) {
    globalLoader = new DatasetLoader(baseUrl);
  }
  return globalLoader;
}

