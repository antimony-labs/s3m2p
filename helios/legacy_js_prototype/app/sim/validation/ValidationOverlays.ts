/**
 * Validation overlays for heliosphere visualization
 * Shows reference distances, spacecraft crossings, and directional markers
 */

import * as THREE from 'three';
import { Registry } from '../registry/Registry';
import { AU, Units } from '../types/units';
import { PositionAU, Vec3Utils } from '../types/vectors';

/**
 * Validation overlay configuration
 */
export interface ValidationConfig {
  showReferenceRings: boolean;
  showVoyagerTracks: boolean;
  showApexArrow: boolean;
  showIBEXArrow: boolean;
  showDistanceLabels: boolean;
}

/**
 * Reference distances for validation (at present epoch)
 */
const REFERENCE_DISTANCES = {
  // Voyager crossings (AU, heliocentric distance)
  V1_TS: { distance: 94.01, label: 'V1 TS', date: '2004-12-16' },
  V2_TS: { distance: 83.7, label: 'V2 TS', date: '2007-08-30' },
  V1_HP: { distance: 121.6, label: 'V1 HP', date: '2012-08-25' },
  V2_HP: { distance: 119.0, label: 'V2 HP', date: '2018-11-05' },
};

/**
 * Validation overlay manager
 */
export class ValidationOverlays {
  private registry: Registry;
  private group: THREE.Group;
  private config: ValidationConfig;
  
  // Overlay objects
  private referenceRings: THREE.Group;
  private voyagerTracks: THREE.Group;
  private apexArrow: THREE.Group;
  private ibexArrow: THREE.Group;
  private labels: THREE.Group;
  
  constructor(registry: Registry, config: Partial<ValidationConfig> = {}) {
    this.registry = registry;
    this.group = new THREE.Group();
    
    this.config = {
      showReferenceRings: true,
      showVoyagerTracks: true,
      showApexArrow: true,
      showIBEXArrow: true,
      showDistanceLabels: true,
      ...config,
    };
    
    // Initialize overlay groups
    this.referenceRings = new THREE.Group();
    this.voyagerTracks = new THREE.Group();
    this.apexArrow = new THREE.Group();
    this.ibexArrow = new THREE.Group();
    this.labels = new THREE.Group();
    
    this.group.add(this.referenceRings);
    this.group.add(this.voyagerTracks);
    this.group.add(this.apexArrow);
    this.group.add(this.ibexArrow);
    this.group.add(this.labels);
    
    // Create overlays
    this.createReferenceRings();
    this.createVoyagerTracks();
    this.createApexArrow();
    this.createIBEXArrow();
    
    // Update visibility
    this.updateVisibility();
  }
  
  /**
   * Create reference distance rings
   */
  private createReferenceRings(): void {
    const distances = [
      REFERENCE_DISTANCES.V2_TS.distance,
      REFERENCE_DISTANCES.V1_TS.distance,
      REFERENCE_DISTANCES.V2_HP.distance,
      REFERENCE_DISTANCES.V1_HP.distance,
    ];
    
    const colors = [
      0xff6b6b, // V2 TS (red)
      0xff6b6b, // V1 TS (red)
      0x4ecdc4, // V2 HP (cyan)
      0x4ecdc4, // V1 HP (cyan)
    ];
    
    const labels = [
      'V2 TS (84 AU)',
      'V1 TS (94 AU)',
      'V2 HP (119 AU)',
      'V1 HP (122 AU)',
    ];
    
    distances.forEach((distAU, i) => {
      const radius = this.registry.auToSceneDistance(Units.AU(distAU));
      
      // Create ring (torus)
      const geometry = new THREE.TorusGeometry(radius, 0.2, 8, 64);
      const material = new THREE.MeshBasicMaterial({
        color: colors[i],
        transparent: true,
        opacity: 0.3,
        side: THREE.DoubleSide,
      });
      
      const ring = new THREE.Mesh(geometry, material);
      ring.rotation.x = Math.PI / 2; // Lie flat in ecliptic plane
      ring.name = labels[i];
      
      this.referenceRings.add(ring);
      
      // Add label
      // (In production, would use THREE.Sprite with canvas texture)
      // For now, just a small sphere marker
      const markerGeo = new THREE.SphereGeometry(1, 8, 8);
      const markerMat = new THREE.MeshBasicMaterial({ color: colors[i] });
      const marker = new THREE.Mesh(markerGeo, markerMat);
      marker.position.set(radius, 0, 0);
      
      this.referenceRings.add(marker);
    });
  }
  
  /**
   * Create Voyager trajectory lines
   */
  private createVoyagerTracks(): void {
    // Voyager 1 trajectory (simplified)
    // In production, would load actual ephemeris data
    const v1Points: THREE.Vector3[] = [];
    const v1StartPos = this.registry.heeToScene({
      x: Units.AU(0),
      y: Units.AU(0),
      z: Units.AU(0),
    });
    
    // Approximate trajectory (north-ish)
    const v1EndDist = 160; // AU (current distance)
    const v1EndPos = this.registry.heeToScene({
      x: Units.AU(v1EndDist * 0.4),
      y: Units.AU(v1EndDist * 0.3),
      z: Units.AU(v1EndDist * 0.8),
    });
    
    // Create line
    for (let t = 0; t <= 1; t += 0.01) {
      v1Points.push(
        new THREE.Vector3(
          v1StartPos.x * (1 - t) + v1EndPos.x * t,
          v1StartPos.y * (1 - t) + v1EndPos.y * t,
          v1StartPos.z * (1 - t) + v1EndPos.z * t
        )
      );
    }
    
    const v1Geometry = new THREE.BufferGeometry().setFromPoints(v1Points);
    const v1Material = new THREE.LineBasicMaterial({
      color: 0xffff00,
      transparent: true,
      opacity: 0.6,
    });
    const v1Line = new THREE.Line(v1Geometry, v1Material);
    v1Line.name = 'Voyager 1 Track';
    
    this.voyagerTracks.add(v1Line);
    
    // Add marker at crossing points
    const tsPos = this.registry.heeToScene({
      x: Units.AU(94.01 * 0.4),
      y: Units.AU(94.01 * 0.3),
      z: Units.AU(94.01 * 0.8),
    });
    
    const tsMarker = new THREE.Mesh(
      new THREE.SphereGeometry(2, 16, 16),
      new THREE.MeshBasicMaterial({ color: 0xff6b6b })
    );
    tsMarker.position.set(tsPos.x, tsPos.y, tsPos.z);
    tsMarker.name = 'V1 TS Crossing';
    
    this.voyagerTracks.add(tsMarker);
    
    // Voyager 2 trajectory (south-ish, simplified)
    // (Similar implementation)
  }
  
  /**
   * Create solar apex arrow (direction of Sun's motion through ISM)
   */
  private createApexArrow(): void {
    // Solar apex direction (rough, towards Hercules constellation)
    const apexDirection = new THREE.Vector3(0.6, 0.7, 0.3).normalize();
    
    // Arrow length in scene units
    const length = this.registry.auToSceneDistance(Units.AU(50));
    
    const arrowHelper = new THREE.ArrowHelper(
      apexDirection,
      new THREE.Vector3(0, 0, 0),
      length,
      0xffa500, // Orange
      length * 0.2,
      length * 0.1
    );
    
    arrowHelper.name = 'Solar Apex (Sun Motion)';
    this.apexArrow.add(arrowHelper);
  }
  
  /**
   * Create IBEX inflow arrow (ISM wind direction)
   */
  private createIBEXArrow(): void {
    // ISM inflow direction from registry
    const inflow = this.registry.ismInflowDirection;
    
    // Convert to THREE.Vector3
    const inflowVec = new THREE.Vector3(inflow.x, inflow.y, inflow.z);
    
    // Arrow length
    const length = this.registry.auToSceneDistance(Units.AU(60));
    
    const arrowHelper = new THREE.ArrowHelper(
      inflowVec,
      new THREE.Vector3(0, 0, 0),
      length,
      0x00ffff, // Cyan
      length * 0.2,
      length * 0.1
    );
    
    arrowHelper.name = 'ISM Inflow (IBEX)';
    this.ibexArrow.add(arrowHelper);
  }
  
  /**
   * Update overlay visibility
   */
  updateVisibility(): void {
    this.referenceRings.visible = this.config.showReferenceRings;
    this.voyagerTracks.visible = this.config.showVoyagerTracks;
    this.apexArrow.visible = this.config.showApexArrow;
    this.ibexArrow.visible = this.config.showIBEXArrow;
    this.labels.visible = this.config.showDistanceLabels;
  }
  
  /**
   * Toggle specific overlay
   */
  toggle(overlay: keyof ValidationConfig, visible?: boolean): void {
    const newValue = visible !== undefined ? visible : !this.config[overlay];
    this.config[overlay] = newValue;
    this.updateVisibility();
  }
  
  /**
   * Get overlay group for adding to scene
   */
  getGroup(): THREE.Group {
    return this.group;
  }
  
  /**
   * Update overlays (e.g., when registry config changes)
   */
  update(): void {
    // Recreate arrows if scale changed
    // For now, overlays are static relative to scene units
  }
  
  /**
   * Dispose resources
   */
  dispose(): void {
    this.group.traverse((child) => {
      if (child instanceof THREE.Mesh || child instanceof THREE.Line) {
        child.geometry.dispose();
        if (Array.isArray(child.material)) {
          child.material.forEach((m) => m.dispose());
        } else {
          child.material.dispose();
        }
      }
    });
  }
}

/**
 * Validation test utilities
 */
export const ValidationTests = {
  /**
   * Test: Verify heliosphere scale at present epoch
   * V1 HP crossing should be at ~121.6 AU
   */
  testHeliopauseScale(registry: Registry, hpNoseRadius: AU): boolean {
    const expectedAU = 121.6;
    const actualAU = hpNoseRadius as number;
    const tolerance = 5.0; // AU
    
    const pass = Math.abs(actualAU - expectedAU) < tolerance;
    
    console.log(`[Validation] Heliopause nose radius: ${actualAU.toFixed(1)} AU (expected ${expectedAU} AU) - ${pass ? 'PASS' : 'FAIL'}`);
    
    return pass;
  },
  
  /**
   * Test: Verify TS/HP ratio
   * Typical ratio is 0.75-0.85
   */
  testTSHPRatio(ratio: number): boolean {
    const pass = ratio >= 0.7 && ratio <= 0.9;
    
    console.log(`[Validation] TS/HP ratio: ${ratio.toFixed(2)} (expected 0.75-0.85) - ${pass ? 'PASS' : 'FAIL'}`);
    
    return pass;
  },
  
  /**
   * Test: Verify ISM inflow direction
   * Should be roughly towards Galactic coordinates l≈255°, b≈5°
   */
  testISMDirection(direction: { x: number; y: number; z: number }): boolean {
    // Expected approximate direction in HEE_J2000
    const expected = { x: -0.93, y: -0.26, z: 0.26 };
    
    const dot = direction.x * expected.x + direction.y * expected.y + direction.z * expected.z;
    const angle = Math.acos(Math.max(-1, Math.min(1, dot)));
    const angleDeg = (angle * 180) / Math.PI;
    
    const pass = angleDeg < 10; // Within 10 degrees
    
    console.log(`[Validation] ISM inflow direction: ${angleDeg.toFixed(1)}° from expected - ${pass ? 'PASS' : 'FAIL'}`);
    
    return pass;
  },
  
  /**
   * Run all validation tests
   */
  runAll(registry: Registry, hpNoseRadius: AU, tsHpRatio: number): boolean {
    console.log('[Validation] Running validation tests...');
    
    const test1 = ValidationTests.testHeliopauseScale(registry, hpNoseRadius);
    const test2 = ValidationTests.testTSHPRatio(tsHpRatio);
    const test3 = ValidationTests.testISMDirection(registry.ismInflowDirection);
    
    const allPass = test1 && test2 && test3;
    
    console.log(`[Validation] Overall: ${allPass ? 'ALL TESTS PASSED' : 'SOME TESTS FAILED'}`);
    
    return allPass;
  },
};

