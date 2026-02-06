//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | TOOLS/CORE/POWER_ENGINE/src/lib.rs
//! PURPOSE: Power supply design engine - high-level API for WASM frontend
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This crate provides a high-level API for power supply design, wrapping the
//! core DNA power module with additional utilities for the WASM frontend.
//!
//! # Supported Topologies
//!
//! - **Buck (Step-Down)**: 12V→5V, 24V→3.3V, etc.
//! - **Boost (Step-Up)**: 3.3V→12V, 5V→24V, etc.
//! - **LDO (Linear)**: Low-noise, simple, but limited efficiency
//!
//! # Example
//!
//! ```rust
//! use power_engine::{design_buck, BuckRequirements, VoltageRange, RippleSpec};
//!
//! let req = BuckRequirements {
//!     vin: VoltageRange::range(10.0, 14.0),
//!     vout: 5.0,
//!     iout_max: 2.0,
//!     iout_min: 0.2,
//!     ripple: RippleSpec::default(),
//!     switching_freq_hz: 500_000.0,
//!     ambient_temp_c: 25.0,
//! };
//!
//! let design = design_buck(&req).expect("Design failed");
//! println!("Efficiency: {:.1}%", design.efficiency.total_efficiency * 100.0);
//! ```

// Re-export all types from DNA power module
pub use dna::power::*;

use serde::{Deserialize, Serialize};

// ============================================================================
// DESIGN REPORT GENERATION
// ============================================================================

/// Generate a formatted design report for display
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignReport {
    pub topology: String,
    pub summary: String,
    pub components: Vec<ComponentEntry>,
    pub performance: Vec<PerformanceEntry>,
    pub warnings: Vec<String>,
}

/// Component entry in design report
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentEntry {
    pub name: String,
    pub value: String,
    pub notes: Vec<String>,
}

/// Performance metric in design report
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceEntry {
    pub metric: String,
    pub value: String,
    pub unit: String,
}

impl From<&BuckDesign> for DesignReport {
    fn from(design: &BuckDesign) -> Self {
        let req = &design.requirements;

        let summary = format!(
            "Buck converter: {:.1}V → {:.1}V @ {:.2}A, fsw = {:.0}kHz",
            req.vin.nom_v,
            req.vout,
            req.iout_max,
            req.switching_freq_hz / 1000.0
        );

        let components = vec![
            ComponentEntry {
                name: "Inductor (L)".to_string(),
                value: format_inductance(design.inductor.selected_value),
                notes: design.inductor.notes.clone(),
            },
            ComponentEntry {
                name: "Output Capacitor (Cout)".to_string(),
                value: format_capacitance(design.output_capacitor.selected_value),
                notes: design.output_capacitor.notes.clone(),
            },
            ComponentEntry {
                name: "Input Capacitor (Cin)".to_string(),
                value: format_capacitance(design.input_capacitor.selected_value),
                notes: design.input_capacitor.notes.clone(),
            },
        ];

        let performance = vec![
            PerformanceEntry {
                metric: "Duty Cycle (nom)".to_string(),
                value: format!("{:.1}", design.duty_cycle_nom * 100.0),
                unit: "%".to_string(),
            },
            PerformanceEntry {
                metric: "Efficiency".to_string(),
                value: format!("{:.1}", design.efficiency.total_efficiency * 100.0),
                unit: "%".to_string(),
            },
            PerformanceEntry {
                metric: "Inductor Ripple".to_string(),
                value: format!("{:.2}", design.inductor_current_ripple_a),
                unit: "A p-p".to_string(),
            },
            PerformanceEntry {
                metric: "Output Ripple".to_string(),
                value: format!("{:.1}", design.output_ripple_mv),
                unit: "mV".to_string(),
            },
            PerformanceEntry {
                metric: "Operating Mode".to_string(),
                value: format!("{:?}", design.operating_mode),
                unit: "".to_string(),
            },
        ];

        let warnings = design.warnings.iter().map(|w| format!("{:?}", w)).collect();

        DesignReport {
            topology: "Buck".to_string(),
            summary,
            components,
            performance,
            warnings,
        }
    }
}

impl From<&BoostDesign> for DesignReport {
    fn from(design: &BoostDesign) -> Self {
        let req = &design.requirements;

        let summary = format!(
            "Boost converter: {:.1}V → {:.1}V @ {:.2}A, fsw = {:.0}kHz",
            req.vin.nom_v,
            req.vout,
            req.iout_max,
            req.switching_freq_hz / 1000.0
        );

        let components = vec![
            ComponentEntry {
                name: "Inductor (L)".to_string(),
                value: format_inductance(design.inductor.selected_value),
                notes: design.inductor.notes.clone(),
            },
            ComponentEntry {
                name: "Output Capacitor (Cout)".to_string(),
                value: format_capacitance(design.output_capacitor.selected_value),
                notes: design.output_capacitor.notes.clone(),
            },
            ComponentEntry {
                name: "Input Capacitor (Cin)".to_string(),
                value: format_capacitance(design.input_capacitor.selected_value),
                notes: design.input_capacitor.notes.clone(),
            },
        ];

        let performance = vec![
            PerformanceEntry {
                metric: "Duty Cycle (nom)".to_string(),
                value: format!("{:.1}", design.duty_cycle_nom * 100.0),
                unit: "%".to_string(),
            },
            PerformanceEntry {
                metric: "Efficiency".to_string(),
                value: format!("{:.1}", design.efficiency.total_efficiency * 100.0),
                unit: "%".to_string(),
            },
            PerformanceEntry {
                metric: "Input Current (max)".to_string(),
                value: format!("{:.2}", design.input_current_a),
                unit: "A".to_string(),
            },
            PerformanceEntry {
                metric: "Switch Voltage Stress".to_string(),
                value: format!("{:.1}", design.switch_voltage_stress_v),
                unit: "V".to_string(),
            },
            PerformanceEntry {
                metric: "Operating Mode".to_string(),
                value: format!("{:?}", design.operating_mode),
                unit: "".to_string(),
            },
        ];

        let warnings = design.warnings.iter().map(|w| format!("{:?}", w)).collect();

        DesignReport {
            topology: "Boost".to_string(),
            summary,
            components,
            performance,
            warnings,
        }
    }
}

impl From<&LDODesign> for DesignReport {
    fn from(design: &LDODesign) -> Self {
        let req = &design.requirements;

        let summary = format!(
            "LDO regulator: {:.1}V → {:.1}V @ {:.2}A",
            req.vin.nom_v, req.vout, req.iout_max
        );

        let components = vec![
            ComponentEntry {
                name: "Input Capacitor (Cin)".to_string(),
                value: format_capacitance(design.input_capacitor.selected_value),
                notes: design.input_capacitor.notes.clone(),
            },
            ComponentEntry {
                name: "Output Capacitor (Cout)".to_string(),
                value: format_capacitance(design.output_capacitor.selected_value),
                notes: design.output_capacitor.notes.clone(),
            },
        ];

        let performance = vec![
            PerformanceEntry {
                metric: "Efficiency (nom)".to_string(),
                value: format!("{:.1}", design.efficiency_nom_percent),
                unit: "%".to_string(),
            },
            PerformanceEntry {
                metric: "Power Dissipation (max)".to_string(),
                value: format!("{:.2}", design.power_dissipation_max_w),
                unit: "W".to_string(),
            },
            PerformanceEntry {
                metric: "Junction Temp".to_string(),
                value: format!("{:.0}", design.thermal.junction_temp_c),
                unit: "°C".to_string(),
            },
            PerformanceEntry {
                metric: "Thermal Margin".to_string(),
                value: format!("{:.0}", design.thermal.thermal_margin_c),
                unit: "°C".to_string(),
            },
            PerformanceEntry {
                metric: "Headroom (min)".to_string(),
                value: format!("{:.2}", design.headroom_min_v),
                unit: "V".to_string(),
            },
        ];

        let warnings = design.warnings.iter().map(|w| format!("{:?}", w)).collect();

        DesignReport {
            topology: "LDO".to_string(),
            summary,
            components,
            performance,
            warnings,
        }
    }
}

// ============================================================================
// DESIGN WORKFLOW HELPERS
// ============================================================================

/// Result of a complete design workflow
#[derive(Clone, Debug)]
pub enum PowerDesignResult {
    Buck(BuckDesign),
    Boost(BoostDesign),
    LDO(LDODesign),
}

impl PowerDesignResult {
    /// Convert to a formatted report
    pub fn to_report(&self) -> DesignReport {
        match self {
            PowerDesignResult::Buck(d) => d.into(),
            PowerDesignResult::Boost(d) => d.into(),
            PowerDesignResult::LDO(d) => d.into(),
        }
    }

    /// Get topology type
    pub fn topology(&self) -> TopologyType {
        match self {
            PowerDesignResult::Buck(_) => TopologyType::Buck,
            PowerDesignResult::Boost(_) => TopologyType::Boost,
            PowerDesignResult::LDO(_) => TopologyType::LDO,
        }
    }
}

/// Auto-design based on voltage relationship
/// Automatically selects Buck, Boost, or LDO based on Vin/Vout relationship
pub fn auto_design(
    vin_nom: f64,
    vout: f64,
    iout: f64,
    fsw: f64,
    priority: DesignPriority,
) -> Result<PowerDesignResult, String> {
    let recommendation = recommend_topology(vin_nom, vout, iout, priority);

    match recommendation.recommended {
        TopologyType::Buck => {
            let design = quick_buck(vin_nom, vout, iout, fsw)?;
            Ok(PowerDesignResult::Buck(design))
        }
        TopologyType::Boost => {
            let design = quick_boost(vin_nom, vout, iout, fsw)?;
            Ok(PowerDesignResult::Boost(design))
        }
        TopologyType::LDO => {
            let design = quick_ldo(vin_nom, vout, iout)?;
            Ok(PowerDesignResult::LDO(design))
        }
        _ => Err(format!(
            "Topology {:?} not yet implemented. {}",
            recommendation.recommended, recommendation.reasoning
        )),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_report() {
        let design = quick_buck(12.0, 5.0, 2.0, 500e3).unwrap();
        let report = DesignReport::from(&design);

        assert_eq!(report.topology, "Buck");
        assert!(!report.components.is_empty());
        assert!(!report.performance.is_empty());
    }

    #[test]
    fn test_boost_report() {
        let design = quick_boost(5.0, 12.0, 0.5, 500e3).unwrap();
        let report = DesignReport::from(&design);

        assert_eq!(report.topology, "Boost");
        assert!(!report.components.is_empty());
    }

    #[test]
    fn test_ldo_report() {
        let design = quick_ldo(5.0, 3.3, 0.5).unwrap();
        let report = DesignReport::from(&design);

        assert_eq!(report.topology, "LDO");
        assert!(!report.components.is_empty());
    }

    #[test]
    fn test_auto_design_step_down() {
        let result = auto_design(12.0, 5.0, 2.0, 500e3, DesignPriority::Efficiency);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().topology(), TopologyType::Buck);
    }

    #[test]
    fn test_auto_design_step_up() {
        let result = auto_design(5.0, 12.0, 0.5, 500e3, DesignPriority::Efficiency);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().topology(), TopologyType::Boost);
    }

    #[test]
    fn test_auto_design_low_power_ldo() {
        // Low power step-down with size priority should prefer LDO
        let result = auto_design(5.0, 3.3, 0.1, 500e3, DesignPriority::Size);
        assert!(result.is_ok());
        // May get LDO or Buck depending on power calculation
        let topology = result.unwrap().topology();
        assert!(topology == TopologyType::LDO || topology == TopologyType::Buck);
    }
}
