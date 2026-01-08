//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: types.rs | DNA/src/power/types.rs
//! PURPOSE: Core types for power supply design (Buck, Boost, LDO)
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

// ============================================================================
// COMMON TYPES
// ============================================================================

/// Voltage range specification (for input voltage tolerance)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoltageRange {
    pub min_v: f64,
    pub nom_v: f64,
    pub max_v: f64,
}

impl VoltageRange {
    /// Create a fixed voltage (no range)
    pub fn fixed(v: f64) -> Self {
        Self {
            min_v: v,
            nom_v: v,
            max_v: v,
        }
    }

    /// Create a voltage range from min to max
    pub fn range(min: f64, max: f64) -> Self {
        Self {
            min_v: min,
            nom_v: (min + max) / 2.0,
            max_v: max,
        }
    }

    /// Create a voltage range with explicit nominal
    pub fn with_nominal(min: f64, nom: f64, max: f64) -> Self {
        Self {
            min_v: min,
            nom_v: nom,
            max_v: max,
        }
    }
}

/// Ripple specification for switching converters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RippleSpec {
    /// Peak-to-peak output voltage ripple (mV)
    pub voltage_mv: f64,
    /// Inductor current ripple ratio (typically 0.2-0.4)
    pub current_ratio: f64,
}

impl Default for RippleSpec {
    fn default() -> Self {
        Self {
            voltage_mv: 50.0,
            current_ratio: 0.3,
        }
    }
}

/// Operating mode for switching converters
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatingMode {
    /// Continuous Conduction Mode - inductor current never reaches zero
    CCM,
    /// Discontinuous Conduction Mode - inductor current reaches zero each cycle
    DCM,
    /// Boundary Conduction Mode - at the edge of CCM/DCM
    BCM,
}

/// Power supply topology type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyType {
    Buck,
    Boost,
    BuckBoost,
    LDO,
    Flyback,
    Forward,
}

// ============================================================================
// COMPONENT SELECTION
// ============================================================================

/// A selected component with ideal and actual E-series values
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedComponent {
    /// Designator (e.g., "L1", "Cout", "R1")
    pub designator: String,
    /// Calculated ideal value
    pub ideal_value: f64,
    /// Selected E-series value
    pub selected_value: f64,
    /// Unit string (e.g., "uH", "uF", "Ohm")
    pub unit: String,
    /// Tolerance percentage (1, 5, 10, 20)
    pub tolerance_pct: f64,
    /// Additional notes (voltage rating, current rating, etc.)
    pub notes: Vec<String>,
}

impl SelectedComponent {
    pub fn new(designator: &str, ideal: f64, selected: f64, unit: &str) -> Self {
        Self {
            designator: designator.to_string(),
            ideal_value: ideal,
            selected_value: selected,
            unit: unit.to_string(),
            tolerance_pct: 10.0,
            notes: Vec::new(),
        }
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance_pct = tolerance;
        self
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Calculate error between ideal and selected value
    pub fn error_percent(&self) -> f64 {
        if self.ideal_value.abs() < 1e-15 {
            return 0.0;
        }
        ((self.selected_value - self.ideal_value) / self.ideal_value * 100.0).abs()
    }
}

// ============================================================================
// EFFICIENCY AND THERMAL
// ============================================================================

/// Breakdown of power losses in a converter
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EfficiencyBreakdown {
    /// Total efficiency (0.0 - 1.0)
    pub total_efficiency: f64,
    /// MOSFET/switch conduction loss (W)
    pub conduction_loss_w: f64,
    /// Switching transition loss (W)
    pub switching_loss_w: f64,
    /// Diode conduction loss (W)
    pub diode_loss_w: f64,
    /// Inductor DCR loss (W)
    pub inductor_dcr_loss_w: f64,
    /// Capacitor ESR loss (W)
    pub capacitor_esr_loss_w: f64,
    /// Quiescent/controller loss (W)
    pub quiescent_loss_w: f64,
    /// Output power (W)
    pub output_power_w: f64,
    /// Input power (W)
    pub input_power_w: f64,
}

impl EfficiencyBreakdown {
    /// Calculate total losses
    pub fn total_losses(&self) -> f64 {
        self.conduction_loss_w
            + self.switching_loss_w
            + self.diode_loss_w
            + self.inductor_dcr_loss_w
            + self.capacitor_esr_loss_w
            + self.quiescent_loss_w
    }
}

/// Thermal analysis results
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThermalAnalysis {
    /// Total power dissipation (W)
    pub power_dissipation_w: f64,
    /// Estimated junction temperature (C)
    pub junction_temp_c: f64,
    /// Thermal margin to max junction temp (C)
    pub thermal_margin_c: f64,
    /// Maximum ambient temperature for safe operation (C)
    pub max_ambient_c: f64,
    /// Whether a heatsink is required
    pub requires_heatsink: bool,
    /// Required heatsink thermal resistance if needed (C/W)
    pub heatsink_theta_ja: Option<f64>,
}

impl Default for ThermalAnalysis {
    fn default() -> Self {
        Self {
            power_dissipation_w: 0.0,
            junction_temp_c: 25.0,
            thermal_margin_c: 100.0,
            max_ambient_c: 85.0,
            requires_heatsink: false,
            heatsink_theta_ja: None,
        }
    }
}

// ============================================================================
// DESIGN WARNINGS
// ============================================================================

/// Warnings generated during design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DesignWarning {
    /// Near CCM/DCM boundary - may enter DCM at light loads
    NearDCMBoundary { margin_percent: f64 },
    /// Output ripple exceeds specification
    HighRipple { actual_mv: f64, spec_mv: f64 },
    /// Thermal concern - junction temp approaching limit
    ThermalConcern {
        junction_temp_c: f64,
        max_temp_c: f64,
    },
    /// Low efficiency warning
    LowEfficiency { efficiency_percent: f64 },
    /// High duty cycle - near limits
    HighDutyCycle { duty_percent: f64 },
    /// Component value differs significantly from E-series
    NonStandardValue {
        component: String,
        ideal: f64,
        selected: f64,
    },
    /// Dropout voltage too low for LDO
    InsufficientHeadroom { headroom_v: f64, minimum_v: f64 },
    /// Current exceeds recommended limit
    HighCurrent { actual_a: f64, recommended_a: f64 },
}

// ============================================================================
// BUCK CONVERTER TYPES
// ============================================================================

/// Buck converter input requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuckRequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output voltage (V)
    pub vout: f64,
    /// Maximum output current (A)
    pub iout_max: f64,
    /// Minimum output current for CCM (A)
    pub iout_min: f64,
    /// Ripple specifications
    pub ripple: RippleSpec,
    /// Switching frequency (Hz)
    pub switching_freq_hz: f64,
    /// Ambient temperature (C)
    pub ambient_temp_c: f64,
}

impl Default for BuckRequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 5.0,
            iout_max: 2.0,
            iout_min: 0.2,
            ripple: RippleSpec::default(),
            switching_freq_hz: 500_000.0,
            ambient_temp_c: 25.0,
        }
    }
}

/// Buck converter design result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuckDesign {
    /// Original requirements
    pub requirements: BuckRequirements,

    // Duty cycles at different input voltages
    /// Duty cycle at minimum Vin (highest duty)
    pub duty_cycle_max: f64,
    /// Duty cycle at nominal Vin
    pub duty_cycle_nom: f64,
    /// Duty cycle at maximum Vin (lowest duty)
    pub duty_cycle_min: f64,

    // Selected components
    /// Main inductor
    pub inductor: SelectedComponent,
    /// Output capacitor
    pub output_capacitor: SelectedComponent,
    /// Input capacitor
    pub input_capacitor: SelectedComponent,

    // Operating characteristics
    /// Peak-to-peak inductor current ripple (A)
    pub inductor_current_ripple_a: f64,
    /// Peak inductor current (A)
    pub inductor_peak_current_a: f64,
    /// Actual output voltage ripple (mV)
    pub output_ripple_mv: f64,
    /// Operating mode at minimum load
    pub operating_mode: OperatingMode,
    /// Critical current for CCM/DCM boundary (A)
    pub critical_current_a: f64,

    // Efficiency analysis
    pub efficiency: EfficiencyBreakdown,

    // Thermal analysis
    pub thermal: ThermalAnalysis,

    // Design warnings
    pub warnings: Vec<DesignWarning>,
}

// ============================================================================
// BOOST CONVERTER TYPES
// ============================================================================

/// Boost converter input requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoostRequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output voltage (V)
    pub vout: f64,
    /// Maximum output current (A)
    pub iout_max: f64,
    /// Minimum output current for CCM (A)
    pub iout_min: f64,
    /// Ripple specifications
    pub ripple: RippleSpec,
    /// Switching frequency (Hz)
    pub switching_freq_hz: f64,
    /// Ambient temperature (C)
    pub ambient_temp_c: f64,
}

impl Default for BoostRequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(3.0, 5.0),
            vout: 12.0,
            iout_max: 0.5,
            iout_min: 0.05,
            ripple: RippleSpec::default(),
            switching_freq_hz: 500_000.0,
            ambient_temp_c: 25.0,
        }
    }
}

/// Boost converter design result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoostDesign {
    /// Original requirements
    pub requirements: BoostRequirements,

    // Duty cycles
    /// Duty cycle at minimum Vin (highest duty)
    pub duty_cycle_max: f64,
    /// Duty cycle at nominal Vin
    pub duty_cycle_nom: f64,
    /// Duty cycle at maximum Vin (lowest duty)
    pub duty_cycle_min: f64,

    // Selected components
    /// Main inductor
    pub inductor: SelectedComponent,
    /// Output capacitor
    pub output_capacitor: SelectedComponent,
    /// Input capacitor
    pub input_capacitor: SelectedComponent,

    // Operating characteristics
    /// Peak-to-peak inductor current ripple (A)
    pub inductor_current_ripple_a: f64,
    /// Peak inductor current (A)
    pub inductor_peak_current_a: f64,
    /// Average input current at max load (A)
    pub input_current_a: f64,
    /// Actual output voltage ripple (mV)
    pub output_ripple_mv: f64,
    /// Operating mode at minimum load
    pub operating_mode: OperatingMode,
    /// Critical current for CCM/DCM boundary (A)
    pub critical_current_a: f64,

    // Switch stress (important for boost)
    /// Voltage stress on switch (V)
    pub switch_voltage_stress_v: f64,
    /// Current stress on switch (A)
    pub switch_current_stress_a: f64,

    // Efficiency analysis
    pub efficiency: EfficiencyBreakdown,

    // Thermal analysis
    pub thermal: ThermalAnalysis,

    // Design warnings
    pub warnings: Vec<DesignWarning>,
}

// ============================================================================
// LDO TYPES
// ============================================================================

/// LDO regulator input requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LDORequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output voltage (V)
    pub vout: f64,
    /// Maximum output current (A)
    pub iout_max: f64,
    /// Dropout voltage specification (V)
    pub dropout_voltage: f64,
    /// Output capacitor ESR range for stability (min, max in Ohms)
    pub output_cap_esr_range: (f64, f64),
    /// Ambient temperature (C)
    pub ambient_temp_c: f64,
    /// Package thermal resistance junction-to-ambient (C/W)
    pub package_theta_ja: f64,
    /// Maximum junction temperature (C)
    pub max_junction_temp_c: f64,
}

impl Default for LDORequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(4.5, 5.5),
            vout: 3.3,
            iout_max: 0.5,
            dropout_voltage: 0.3,
            output_cap_esr_range: (0.01, 0.3),
            ambient_temp_c: 25.0,
            package_theta_ja: 50.0, // SOT-223 typical
            max_junction_temp_c: 125.0,
        }
    }
}

/// LDO design result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LDODesign {
    /// Original requirements
    pub requirements: LDORequirements,

    // Headroom analysis
    /// Minimum headroom at Vin_min (V)
    pub headroom_min_v: f64,
    /// Maximum headroom at Vin_max (V)
    pub headroom_max_v: f64,
    /// Whether dropout requirement is met
    pub meets_dropout: bool,

    // Selected components
    /// Input capacitor
    pub input_capacitor: SelectedComponent,
    /// Output capacitor
    pub output_capacitor: SelectedComponent,

    // Power dissipation
    /// Power dissipation at Vin_min (W)
    pub power_dissipation_min_w: f64,
    /// Power dissipation at Vin_nom (W)
    pub power_dissipation_nom_w: f64,
    /// Power dissipation at Vin_max (W)
    pub power_dissipation_max_w: f64,

    // Efficiency (always Vout/Vin for LDO)
    /// Efficiency at Vin_min (best case)
    pub efficiency_max_percent: f64,
    /// Efficiency at Vin_nom
    pub efficiency_nom_percent: f64,
    /// Efficiency at Vin_max (worst case)
    pub efficiency_min_percent: f64,

    // Thermal analysis
    pub thermal: ThermalAnalysis,

    // Current limits
    /// Whether operating within safe area
    pub safe_operating_area: bool,
    /// Maximum current limited by thermal (A)
    pub max_current_thermal_a: f64,

    // Design warnings
    pub warnings: Vec<DesignWarning>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_range() {
        let fixed = VoltageRange::fixed(5.0);
        assert_eq!(fixed.min_v, 5.0);
        assert_eq!(fixed.nom_v, 5.0);
        assert_eq!(fixed.max_v, 5.0);

        let range = VoltageRange::range(10.0, 14.0);
        assert_eq!(range.min_v, 10.0);
        assert_eq!(range.nom_v, 12.0);
        assert_eq!(range.max_v, 14.0);
    }

    #[test]
    fn test_selected_component() {
        let comp = SelectedComponent::new("L1", 21.5e-6, 22e-6, "H")
            .with_tolerance(10.0)
            .with_note("Use shielded inductor");

        assert_eq!(comp.designator, "L1");
        assert!((comp.error_percent() - 2.33).abs() < 0.1);
        assert_eq!(comp.notes.len(), 1);
    }

    #[test]
    fn test_buck_requirements_default() {
        let req = BuckRequirements::default();
        assert_eq!(req.vout, 5.0);
        assert_eq!(req.iout_max, 2.0);
        assert_eq!(req.switching_freq_hz, 500_000.0);
    }

    #[test]
    fn test_ldo_requirements_default() {
        let req = LDORequirements::default();
        assert_eq!(req.vout, 3.3);
        assert_eq!(req.dropout_voltage, 0.3);
    }
}
