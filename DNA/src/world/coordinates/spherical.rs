//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: spherical.rs | DNA/src/world/coordinates/spherical.rs
//! PURPOSE: Spherical coordinate system (r, θ, φ)
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: Spherical coordinate system (r, θ, φ)
//!
//! LAYER: DNA → WORLD → COORDINATES
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA DEFINED                                                                │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Spherical         (radius, theta, phi) representation                       │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA FLOW                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ CONSUMES:  Vec3 (Cartesian) or (r, θ, φ)                                    │
//! │ PRODUCES:  Vec3 (Cartesian) or Spherical                                    │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! PHYSICS:
//!   x = r·sin(θ)·cos(φ)
//!   y = r·sin(θ)·sin(φ)
//!   z = r·cos(θ)
//!
//! Convention: Physics/ISO (θ from Z-axis, φ azimuthal)
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::Vec3;

/// Spherical coordinates (r, θ, φ)
#[derive(Clone, Copy, Debug)]
pub struct Spherical {
    pub r: f32,     // Radius
    pub theta: f32, // Polar angle from Z-axis [0, π]
    pub phi: f32,   // Azimuthal angle [0, 2π]
}

impl Spherical {
    pub fn new(r: f32, theta: f32, phi: f32) -> Self {
        Self { r, theta, phi }
    }

    /// Convert to Cartesian coordinates
    pub fn to_cartesian(&self) -> Vec3 {
        let sin_theta = self.theta.sin();
        Vec3::new(
            self.r * sin_theta * self.phi.cos(),
            self.r * sin_theta * self.phi.sin(),
            self.r * self.theta.cos(),
        )
    }

    /// Create from Cartesian coordinates
    pub fn from_cartesian(v: Vec3) -> Self {
        let r = v.length();
        let theta = (v.z / r).acos();
        let phi = v.y.atan2(v.x);

        Self { r, theta, phi }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spherical_roundtrip() {
        let cartesian = Vec3::new(1.0, 2.0, 3.0);
        let spherical = Spherical::from_cartesian(cartesian);
        let back = spherical.to_cartesian();

        assert!((cartesian - back).length() < 1e-5);
    }
}
