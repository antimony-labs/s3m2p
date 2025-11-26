/**
 * Coordinate frames for heliosphere simulation
 * All positions are expressed relative to the Sun
 */

import { PositionAU, VelocityKmS, UnitVector, Vec3Utils } from '../types/vectors';
import { AU, KmPerSec, Radians, Units } from '../types/units';

/**
 * Supported coordinate frames
 */
export enum CoordinateFrame {
  /**
   * International Celestial Reference System (ICRS)
   * Inertial frame aligned with celestial coordinates
   * Origin: Solar System Barycenter (approximated as Sun)
   */
  ICRS = 'ICRS',

  /**
   * Heliocentric Earth Ecliptic J2000 (HEE/J2000)
   * Primary simulation frame
   * Origin: Sun center
   * +X: Towards vernal equinox at J2000.0
   * +Z: North ecliptic pole
   * +Y: Completes right-handed system
   */
  HEE_J2000 = 'HEE_J2000',

  /**
   * Apex-aligned frame (display basis)
   * Origin: Sun center
   * +X: Direction of interstellar medium inflow (nose)
   * +Z: Perpendicular to ecliptic plane
   * +Y: Completes right-handed system
   */
  APEX = 'APEX',
}

/**
 * 3×3 rotation matrix for coordinate transforms
 */
export interface Matrix3x3 {
  m00: number; m01: number; m02: number;
  m10: number; m11: number; m12: number;
  m20: number; m21: number; m22: number;
}

/**
 * Coordinate frame transformation service
 */
export class CoordinateTransforms {
  /**
   * Direction of ISM inflow in HEE_J2000 (from IBEX measurements)
   * Galactic longitude λ ≈ 255.4°, latitude β ≈ 5.2°
   * Converted to HEE_J2000 frame
   */
  private readonly ismInflowDirection: UnitVector;

  /**
   * Rotation matrix from HEE_J2000 to APEX
   */
  private readonly heeToApex: Matrix3x3;

  /**
   * Rotation matrix from APEX to HEE_J2000
   */
  private readonly apexToHee: Matrix3x3;

  constructor(
    ismInflowLongitude: Radians = Units.Radians(255.4 * Math.PI / 180),
    ismInflowLatitude: Radians = Units.Radians(5.2 * Math.PI / 180)
  ) {
    // Convert ISM inflow galactic coordinates to HEE_J2000
    this.ismInflowDirection = this.galacticToHEE(ismInflowLongitude, ismInflowLatitude);

    // Build rotation matrices
    this.heeToApex = this.buildApexRotation(this.ismInflowDirection);
    this.apexToHee = this.transposeMatrix(this.heeToApex);
  }

  /**
   * Convert galactic longitude/latitude to HEE_J2000 direction
   * Simplified conversion (full implementation would include proper precession)
   */
  private galacticToHEE(lon: Radians, lat: Radians): UnitVector {
    const lonRad = lon as number;
    const latRad = lat as number;

    // Simplified transformation (assumes small offset from ecliptic)
    // Full implementation would use proper ICRS → Galactic → HEE chain
    const cosLat = Math.cos(latRad);
    
    return Vec3Utils.unit(
      -cosLat * Math.cos(lonRad), // Inflow is towards Sun (negative)
      -cosLat * Math.sin(lonRad),
      -Math.sin(latRad)
    );
  }

  /**
   * Build rotation matrix that aligns +X with ISM inflow direction
   */
  private buildApexRotation(noseDirection: UnitVector): Matrix3x3 {
    // New +X axis: direction of inflow (nose)
    const xAxis = noseDirection;

    // Keep +Z roughly aligned with ecliptic north
    // (perpendicular to ecliptic plane)
    const eclipticNorth: UnitVector = { x: 0, y: 0, z: 1 };

    // +Y = +Z × +X (right-handed system)
    const yAxis = Vec3Utils.normalize(Vec3Utils.cross(eclipticNorth, xAxis));

    // Recompute +Z to ensure orthogonality: +Z = +X × +Y
    const zAxis = Vec3Utils.normalize(Vec3Utils.cross(xAxis, yAxis));

    // Matrix where columns are the new basis vectors
    return {
      m00: xAxis.x, m01: yAxis.x, m02: zAxis.x,
      m10: xAxis.y, m11: yAxis.y, m12: zAxis.y,
      m20: xAxis.z, m21: yAxis.z, m22: zAxis.z,
    };
  }

  /**
   * Transpose a 3×3 matrix (used for inverse rotation)
   */
  private transposeMatrix(m: Matrix3x3): Matrix3x3 {
    return {
      m00: m.m00, m01: m.m10, m02: m.m20,
      m10: m.m01, m11: m.m11, m12: m.m21,
      m20: m.m02, m21: m.m12, m22: m.m22,
    };
  }

  /**
   * Apply rotation matrix to a position vector
   */
  private rotatePosition(pos: PositionAU, matrix: Matrix3x3): PositionAU {
    const x = pos.x as number;
    const y = pos.y as number;
    const z = pos.z as number;

    return {
      x: (matrix.m00 * x + matrix.m01 * y + matrix.m02 * z) as AU,
      y: (matrix.m10 * x + matrix.m11 * y + matrix.m12 * z) as AU,
      z: (matrix.m20 * x + matrix.m21 * y + matrix.m22 * z) as AU,
    };
  }

  /**
   * Apply rotation matrix to a velocity vector
   */
  private rotateVelocity(vel: VelocityKmS, matrix: Matrix3x3): VelocityKmS {
    const x = vel.x as number;
    const y = vel.y as number;
    const z = vel.z as number;

    return {
      x: (matrix.m00 * x + matrix.m01 * y + matrix.m02 * z) as KmPerSec,
      y: (matrix.m10 * x + matrix.m11 * y + matrix.m12 * z) as KmPerSec,
      z: (matrix.m20 * x + matrix.m21 * y + matrix.m22 * z) as KmPerSec,
    };
  }

  /**
   * Transform position from HEE_J2000 to APEX frame
   */
  public heeToApexPosition(pos: PositionAU): PositionAU {
    return this.rotatePosition(pos, this.heeToApex);
  }

  /**
   * Transform position from APEX to HEE_J2000 frame
   */
  public apexToHeePosition(pos: PositionAU): PositionAU {
    return this.rotatePosition(pos, this.apexToHee);
  }

  /**
   * Transform velocity from HEE_J2000 to APEX frame
   */
  public heeToApexVelocity(vel: VelocityKmS): VelocityKmS {
    return this.rotateVelocity(vel, this.heeToApex);
  }

  /**
   * Transform velocity from APEX to HEE_J2000 frame
   */
  public apexToHeeVelocity(vel: VelocityKmS): VelocityKmS {
    return this.rotateVelocity(vel, this.apexToHee);
  }

  /**
   * Get ISM inflow direction in HEE_J2000
   */
  public getIsmInflowDirection(): UnitVector {
    return Vec3Utils.clone(this.ismInflowDirection);
  }

  /**
   * Get rotation matrix for HEE → APEX transformation
   * (Useful for bulk transforms in Three.js)
   */
  public getHeeToApexMatrix(): Matrix3x3 {
    return { ...this.heeToApex };
  }

  /**
   * Convert to Three.js Matrix4 format (column-major)
   * Returns flat array of 16 elements
   */
  public toThreeMatrix4(matrix: Matrix3x3): number[] {
    return [
      matrix.m00, matrix.m10, matrix.m20, 0,
      matrix.m01, matrix.m11, matrix.m21, 0,
      matrix.m02, matrix.m12, matrix.m22, 0,
      0,          0,          0,          1,
    ];
  }
}

