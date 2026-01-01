//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/mechanics/mod.rs
//! PURPOSE: Module exports: particle
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// Point mass dynamics (F=ma)
pub mod particle;
pub use particle::Particle;

// pub mod rigid_body;  // TODO: 3D rotation, inertia tensor
// pub mod soft_body;   // TODO: Mass-spring, FEM deformable
// pub mod constraint;  // TODO: Joints, springs, distance constraints
// pub mod collision;   // TODO: GJK, SAT, contact generation
