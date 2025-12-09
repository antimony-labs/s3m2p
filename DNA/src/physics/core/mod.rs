//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/core/mod.rs
//! PURPOSE: Physical units, constants, and quantity types
//! LAYER: DNA → PHYSICS → CORE
//! ═══════════════════════════════════════════════════════════════════════════════

/// Physical constants (c, G, h, k_B, etc.)
pub mod constants;
pub use constants::*;

// pub mod units;       // TODO: Type-safe physical units (N, J, kg)
// pub mod quantities;  // TODO: Force, Energy, Momentum types
