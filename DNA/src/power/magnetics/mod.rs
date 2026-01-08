//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/power/magnetics/mod.rs
//! PURPOSE: Magnetic component design - cores, wire, inductors, transformers
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This module provides design automation for magnetic components:
//!
//! - **Core Materials**: Ferrite, iron powder, Sendust with Steinmetz loss models
//! - **Wire Properties**: AWG specs, skin effect, proximity effect calculations
//! - **Transformer Design**: Core selection, turns calculation, winding design
//!
//! # Example Usage
//!
//! ```rust
//! use dna::power::magnetics::{CoreMaterial, ferrite_database, WireSpec, awg_database};
//!
//! // Find a suitable core material
//! let n87 = ferrite_database().into_iter()
//!     .find(|m| m.name == "N87")
//!     .unwrap();
//!
//! // Calculate core loss
//! let pv = n87.core_loss_density(100e3, 0.1); // W/cm³ at 100kHz, 0.1T
//!
//! // Get wire specs
//! let awg22 = awg_database().into_iter()
//!     .find(|w| w.awg == 22)
//!     .unwrap();
//! ```

pub mod core_materials;
pub mod wire;
pub mod transformer;

pub use core_materials::*;
pub use wire::*;
pub use transformer::*;
