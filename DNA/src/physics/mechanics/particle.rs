//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: particle.rs | DNA/src/physics/mechanics/particle.rs
//! PURPOSE: Point mass dynamics using Newton's laws
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: Point mass dynamics using Newton's laws
//!
//! LAYER: DNA → PHYSICS → MECHANICS
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA DEFINED                                                                │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Particle          Point mass with position, velocity, forces                │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA FLOW                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ CONSUMES:  Vec3 (forces), f64 (dt), f64 (mass)                              │
//! │ PRODUCES:  Vec3 (position), Vec3 (velocity), f64 (kinetic energy)           │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! PHYSICS:
//!   F = ma           Newton's second law
//!   a = F/m          Acceleration from force
//!   v += a·dt        Velocity integration
//!   x += v·dt        Position integration
//!   KE = ½mv²        Kinetic energy
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::Vec3;

/// Point mass particle
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
    force_accumulator: Vec3,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            force_accumulator: Vec3::ZERO,
        }
    }

    /// Apply force to particle
    pub fn apply_force(&mut self, force: Vec3) {
        self.force_accumulator += force;
    }

    /// Update particle physics (Euler integration)
    pub fn update(&mut self, dt: f64) {
        let acceleration = self.force_accumulator / self.mass as f32;
        self.velocity += acceleration * dt as f32;
        self.position += self.velocity * dt as f32;
        self.force_accumulator = Vec3::ZERO;
    }

    /// Compute kinetic energy
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * (self.velocity.length_squared() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_fall() {
        let mut p = Particle::new(Vec3::new(0.0, 10.0, 0.0), Vec3::ZERO, 1.0);
        let gravity = Vec3::new(0.0, -9.81, 0.0);

        for _ in 0..10 {
            p.apply_force(gravity);
            p.update(0.1);
        }

        assert!(p.position.y < 10.0);
        assert!(p.velocity.y < 0.0);
    }
}
