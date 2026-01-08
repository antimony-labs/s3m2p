//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/power/topologies/mod.rs
//! PURPOSE: Power supply topology implementations - isolated and non-isolated
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This module provides complete topology implementations for SMPS design:
//!
//! ## Non-Isolated Topologies
//! - Buck (step-down) - existing enhanced implementation
//! - Boost (step-up) - existing enhanced implementation
//! - Buck-boost (inverting)
//! - SEPIC
//! - Ćuk
//!
//! ## Isolated Topologies
//! - Flyback - single/multi-output, CCM/DCM/CrM
//! - Forward - single-switch with reset winding
//! - Push-pull
//! - Half-bridge (PWM and LLC resonant)
//! - Full-bridge (phase-shifted)
//!
//! # Example Usage
//!
//! ```rust
//! use dna::power::topologies::flyback::{FlybackRequirements, FlybackOutput, design_flyback};
//! use dna::power::VoltageRange;
//! use dna::power::magnetics::IsolationClass;
//!
//! let requirements = FlybackRequirements {
//!     vin: VoltageRange::range(36.0, 72.0), // 48V telecom range
//!     outputs: vec![
//!         FlybackOutput::regulated(5.0, 2.0),   // 5V @ 2A (main output)
//!         FlybackOutput::new(12.0, 0.5),        // 12V @ 0.5A (auxiliary)
//!     ],
//!     switching_freq: 100e3,
//!     isolation: IsolationClass::Basic,
//!     ..Default::default()
//! };
//!
//! let design = design_flyback(&requirements).expect("Design failed");
//! ```

pub mod buck_boost;
pub mod flyback;
pub mod forward;

// Re-export main types
pub use buck_boost::*;
pub use flyback::*;
pub use forward::*;
