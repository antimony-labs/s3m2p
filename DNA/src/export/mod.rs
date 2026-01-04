//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/export/mod.rs
//! PURPOSE: Module exports: pdf, gerber, step
//! MODIFIED: 2026-01-01
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//! Export module for generating PDF, Gerber X2, STEP, and STL files
//!
//! This module implements file format generation from scratch,
//! following the CLAUDE.md philosophy of minimizing external dependencies.
//! - STEP export provides ISO 10303-242 (AP242) compliance with PMI/GD&T
//! - STL export for 3D printing (binary format)

pub mod gerber;
pub mod pdf;
pub mod step;
pub mod stl;

pub use gerber::*;
pub use pdf::*;
pub use step::*;
pub use stl::*;
