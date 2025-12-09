//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/autocrate/mod.rs
//! PURPOSE: Module exports for autocrate
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod calculator;
pub mod constants;
pub mod geometry;
pub mod types;

pub use calculator::*;
pub use constants::*;
pub use geometry::*;
pub use types::*;
