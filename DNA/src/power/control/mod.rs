//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/power/control/mod.rs
//! PURPOSE: Control loop design for SMPS - compensators, stability analysis
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This module provides control loop design for switched-mode power supplies:
//!
//! ## Small-Signal Models
//! - Transfer functions for all topologies (buck, boost, flyback, forward)
//! - Control-to-output, line-to-output, impedance models
//! - CCM and DCM operation modes
//!
//! ## Compensator Design
//! - Type I (integrator)
//! - Type II (one zero, one high-frequency pole)
//! - Type III (two zeros, two high-frequency poles)
//! - Automatic compensator synthesis for target phase margin
//!
//! ## Stability Analysis
//! - Bode plot generation (magnitude and phase)
//! - Phase margin and gain margin calculation
//! - Crossover frequency analysis
//!
//! # Example Usage
//!
//! ```rust
//! use dna::power::control::{
//!     BuckSmallSignal, CompensatorDesign, CompensatorType,
//!     CompensatorRequirements, design_compensator,
//! };
//!
//! // Create small-signal model for buck converter
//! let plant = BuckSmallSignal::new(
//!     12.0,   // Vin
//!     5.0,    // Vout
//!     2.0,    // Iout
//!     10e-6,  // L
//!     100e-6, // C
//!     0.02,   // ESR
//!     500e3,  // fsw
//! );
//!
//! // Design Type II compensator for 50kHz crossover, 60° phase margin
//! let req = CompensatorRequirements {
//!     crossover_freq: 50e3,
//!     phase_margin_deg: 60.0,
//!     ..Default::default()
//! };
//!
//! let comp = design_compensator(
//!     &plant.control_to_output(),
//!     &req,
//!     CompensatorType::TypeII,
//! ).expect("Compensator design failed");
//!
//! println!("Crossover: {:.1} kHz", comp.crossover_freq / 1e3);
//! println!("Phase margin: {:.1}°", comp.phase_margin_deg);
//! ```

pub mod compensator;
pub mod small_signal;
pub mod stability;

// Re-export main types
pub use compensator::*;
pub use small_signal::*;
pub use stability::*;
