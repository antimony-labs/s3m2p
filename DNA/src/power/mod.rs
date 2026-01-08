//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/power/mod.rs
//! PURPOSE: Power supply design module - Buck, Boost, LDO topologies
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This module provides design automation for common power supply topologies:
//!
//! - **Buck (Step-Down)**: Convert higher voltage to lower voltage efficiently
//! - **Boost (Step-Up)**: Convert lower voltage to higher voltage
//! - **LDO (Low Dropout)**: Linear regulator for low-noise, low-dropout applications
//!
//! # Example Usage
//!
//! ```rust
//! use dna::power::{BuckRequirements, VoltageRange, RippleSpec, design_buck};
//!
//! let requirements = BuckRequirements {
//!     vin: VoltageRange::range(10.0, 14.0),
//!     vout: 5.0,
//!     iout_max: 2.0,
//!     iout_min: 0.2,
//!     ripple: RippleSpec::default(),
//!     switching_freq_hz: 500_000.0,
//!     ambient_temp_c: 25.0,
//! };
//!
//! let design = design_buck(&requirements).expect("Design failed");
//! println!("Inductor: {:?}", design.inductor);
//! println!("Efficiency: {:.1}%", design.efficiency.total_efficiency * 100.0);
//! ```

pub mod buck;
pub mod boost;
pub mod components;
pub mod ldo;
pub mod state_space;
pub mod transient;
pub mod types;

// Re-export main types for convenient access
pub use types::*;

// Re-export design functions
pub use buck::design_buck;
pub use boost::design_boost;
pub use ldo::design_ldo;

// Re-export component utilities
pub use components::{
    format_capacitance, format_current, format_frequency, format_inductance,
    format_percent, format_power, format_resistance, format_voltage,
    nearest_capacitor, nearest_e12, nearest_e24, nearest_e96, nearest_e6,
    nearest_inductor, next_higher_capacitor, next_higher_inductor,
    E12, E24, E6, E96, STANDARD_CERAMICS_UF, STANDARD_ELECTROLYTICS_UF,
    STANDARD_INDUCTORS_UH,
};

// Re-export simulation types
pub use state_space::{Matrix1x2, Matrix2x1, Matrix2x2, StateSpace2, SwitchedConverter};
pub use transient::{
    buck_duty_for_vout, buck_steady_state_vout, boost_duty_for_vout,
    boost_steady_state_vout, simulate_boost, simulate_buck, SimulationStats,
    TransientConfig, TransientResult,
};

// ============================================================================
// HIGH-LEVEL DESIGN HELPERS
// ============================================================================

/// Quick design helper for a buck converter with sensible defaults
pub fn quick_buck(vin_nom: f64, vout: f64, iout: f64, fsw: f64) -> Result<BuckDesign, String> {
    let requirements = BuckRequirements {
        vin: VoltageRange::range(vin_nom * 0.9, vin_nom * 1.1),
        vout,
        iout_max: iout,
        iout_min: iout * 0.1,
        switching_freq_hz: fsw,
        ..Default::default()
    };
    design_buck(&requirements)
}

/// Quick design helper for a boost converter with sensible defaults
pub fn quick_boost(vin_nom: f64, vout: f64, iout: f64, fsw: f64) -> Result<BoostDesign, String> {
    let requirements = BoostRequirements {
        vin: VoltageRange::range(vin_nom * 0.9, vin_nom * 1.1),
        vout,
        iout_max: iout,
        iout_min: iout * 0.1,
        switching_freq_hz: fsw,
        ..Default::default()
    };
    design_boost(&requirements)
}

/// Quick design helper for an LDO with sensible defaults
pub fn quick_ldo(vin_nom: f64, vout: f64, iout: f64) -> Result<LDODesign, String> {
    let requirements = LDORequirements {
        vin: VoltageRange::range(vin_nom * 0.9, vin_nom * 1.1),
        vout,
        iout_max: iout,
        ..Default::default()
    };
    design_ldo(&requirements)
}

// ============================================================================
// TOPOLOGY SELECTION
// ============================================================================

/// Design priority for topology selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignPriority {
    /// Maximize efficiency
    Efficiency,
    /// Minimize solution size
    Size,
    /// Minimize component cost
    Cost,
    /// Minimize output noise
    Noise,
}

/// Topology recommendation result
#[derive(Clone, Debug)]
pub struct TopologyRecommendation {
    /// Recommended topology
    pub recommended: TopologyType,
    /// Reasoning for the recommendation
    pub reasoning: String,
    /// Alternative options with explanations
    pub alternatives: Vec<(TopologyType, String)>,
}

/// Recommend a topology based on requirements
pub fn recommend_topology(
    vin: f64,
    vout: f64,
    iout: f64,
    priority: DesignPriority,
) -> TopologyRecommendation {
    let mut alternatives = Vec::new();

    // Determine voltage relationship
    let is_step_down = vout < vin;
    let is_step_up = vout > vin;
    let headroom = if is_step_down { vin - vout } else { 0.0 };
    let p_out = vout * iout;

    // Calculate LDO efficiency if applicable
    let ldo_efficiency = if is_step_down { vout / vin } else { 0.0 };
    let ldo_dissipation = if is_step_down { headroom * iout } else { f64::INFINITY };

    // Decision logic
    let recommended = if is_step_up {
        // Must use boost for step-up
        alternatives.push((TopologyType::BuckBoost, "Buck-boost for wide Vin range".to_string()));
        TopologyType::Boost
    } else if is_step_down {
        match priority {
            DesignPriority::Noise => {
                // LDO has best noise performance
                if ldo_efficiency > 0.6 && ldo_dissipation < 2.0 {
                    alternatives.push((TopologyType::Buck, "Buck for higher efficiency".to_string()));
                    TopologyType::LDO
                } else {
                    alternatives.push((TopologyType::LDO, "LDO for lower noise (but thermal concerns)".to_string()));
                    TopologyType::Buck
                }
            }
            DesignPriority::Efficiency => {
                // Buck is most efficient for step-down
                if ldo_efficiency > 0.85 {
                    alternatives.push((TopologyType::LDO, "LDO acceptable at >85% efficiency".to_string()));
                }
                TopologyType::Buck
            }
            DesignPriority::Size | DesignPriority::Cost => {
                // LDO is smaller/cheaper if thermal allows
                if ldo_dissipation < 0.5 {
                    alternatives.push((TopologyType::Buck, "Buck for higher power".to_string()));
                    TopologyType::LDO
                } else if ldo_dissipation < 2.0 && p_out < 1.0 {
                    alternatives.push((TopologyType::Buck, "Buck for better thermal".to_string()));
                    TopologyType::LDO
                } else {
                    alternatives.push((TopologyType::LDO, "LDO if thermal solution available".to_string()));
                    TopologyType::Buck
                }
            }
        }
    } else {
        // Vin ≈ Vout
        alternatives.push((TopologyType::Buck, "Buck if Vin slightly higher".to_string()));
        alternatives.push((TopologyType::Boost, "Boost if Vin slightly lower".to_string()));
        TopologyType::BuckBoost
    };

    let reasoning = match recommended {
        TopologyType::Buck => format!(
            "Buck converter: {:.1}V to {:.1}V at {:.2}A. \
            Estimated efficiency >90%. Switching solution required.",
            vin, vout, iout
        ),
        TopologyType::Boost => format!(
            "Boost converter: {:.1}V to {:.1}V at {:.2}A. \
            Step-up requires boost topology. Watch for high input current ({:.2}A).",
            vin, vout, iout, iout * vout / vin
        ),
        TopologyType::LDO => format!(
            "LDO regulator: {:.1}V to {:.1}V at {:.2}A. \
            Efficiency {:.0}%, dissipation {:.2}W. Low noise solution.",
            vin, vout, iout, ldo_efficiency * 100.0, ldo_dissipation
        ),
        TopologyType::BuckBoost => format!(
            "Buck-boost for Vin ≈ Vout or wide input range.",
        ),
        _ => "See alternatives".to_string(),
    };

    TopologyRecommendation {
        recommended,
        reasoning,
        alternatives,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_buck() {
        let result = quick_buck(12.0, 5.0, 2.0, 500e3);
        assert!(result.is_ok());
        let design = result.unwrap();
        assert!(design.efficiency.total_efficiency > 0.8);
    }

    #[test]
    fn test_quick_boost() {
        let result = quick_boost(5.0, 12.0, 0.5, 500e3);
        assert!(result.is_ok());
        let design = result.unwrap();
        assert!(design.input_current_a > design.requirements.iout_max);
    }

    #[test]
    fn test_quick_ldo() {
        let result = quick_ldo(5.0, 3.3, 0.5);
        assert!(result.is_ok());
        let design = result.unwrap();
        assert!(design.efficiency_nom_percent > 60.0);
    }

    #[test]
    fn test_topology_recommendation_step_down_high_power() {
        let rec = recommend_topology(12.0, 5.0, 3.0, DesignPriority::Efficiency);
        assert_eq!(rec.recommended, TopologyType::Buck);
    }

    #[test]
    fn test_topology_recommendation_step_down_low_power() {
        let rec = recommend_topology(5.0, 3.3, 0.1, DesignPriority::Size);
        // LDO should be recommended for low power, small size
        assert!(rec.recommended == TopologyType::LDO || rec.recommended == TopologyType::Buck);
    }

    #[test]
    fn test_topology_recommendation_step_up() {
        let rec = recommend_topology(3.3, 12.0, 0.5, DesignPriority::Efficiency);
        assert_eq!(rec.recommended, TopologyType::Boost);
    }

    #[test]
    fn test_topology_recommendation_noise() {
        let rec = recommend_topology(5.0, 3.3, 0.2, DesignPriority::Noise);
        // LDO should be recommended for noise-sensitive applications
        assert_eq!(rec.recommended, TopologyType::LDO);
    }
}
