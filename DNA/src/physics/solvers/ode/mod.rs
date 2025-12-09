//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/solvers/ode/mod.rs
//! PURPOSE: Ordinary Differential Equation solvers
//! LAYER: DNA → PHYSICS → SOLVERS → ODE
//! ═══════════════════════════════════════════════════════════════════════════════

/// Forward Euler (1st order)
pub mod euler;
pub use euler::euler_step;

/// Runge-Kutta 4 (4th order)
pub mod rk4;
pub use rk4::rk4_step;

/// Velocity Verlet (symplectic)
pub mod verlet;
pub use verlet::verlet_step;

// pub mod leapfrog;  // TODO: Leapfrog (symplectic)
// pub mod adaptive;  // TODO: RK45, Dormand-Prince
