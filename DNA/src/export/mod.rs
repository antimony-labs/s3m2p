//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/export/mod.rs
//! PURPOSE: Module exports: pdf, gerber, step
//! MODIFIED: 2026-01-01
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//! Export module for generating PDF, Gerber X2, and STEP files
//!
//! This module implements PDF and Gerber generation from scratch,
//! following the CLAUDE.md philosophy of minimizing external dependencies.
//! STEP export provides ISO 10303-242 (AP242) compliance with PMI/GD&T.

pub mod gerber;
pub mod pdf;
pub mod step;

pub use gerber::*;
pub use pdf::*;
pub use step::*;
