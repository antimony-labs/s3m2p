//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: conduction.rs
//! PATH: DNA/src/physics/thermal/conduction.rs
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PURPOSE: Heat equation solver (thermal conduction)
//!
//! LAYER: DNA → PHYSICS → THERMAL
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ PHYSICS                                                                     │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Heat equation: ∂T/∂t = α∇²T                                                 │
//! │                                                                             │
//! │ Where:                                                                      │
//! │   T = temperature (K)                                                       │
//! │   α = thermal diffusivity (m²/s)                                            │
//! │   α = k/(ρ·c_p)                                                             │
//! │     k = thermal conductivity                                                │
//! │     ρ = density                                                             │
//! │     c_p = specific heat capacity                                            │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! REFERENCE: https://en.wikipedia.org/wiki/Heat_equation
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// TODO: Implement finite difference heat equation solver
// TODO: Support different boundary conditions (Dirichlet, Neumann, Robin)
// TODO: 1D, 2D, 3D versions
