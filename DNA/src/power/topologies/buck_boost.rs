//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: buck_boost.rs | DNA/src/power/topologies/buck_boost.rs
//! PURPOSE: Inverting Buck-Boost converter topology design
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Complete inverting buck-boost converter design including:
//! - Operating mode selection (CCM/DCM)
//! - Inductor design
//! - Switch and diode selection
//! - Capacitor sizing
//! - Efficiency estimation
//!
//! The inverting buck-boost can produce output voltage either higher or lower
//! than input, but the output is inverted (negative with respect to input ground).
//!
//! Key equations:
//! - Output voltage: Vout = -Vin × D / (1-D)
//! - Inductor current: IL_avg = Iout / (1-D)
//! - Input current: Iin = IL × D

use serde::{Deserialize, Serialize};

use crate::power::components::diode::{find_suitable_diodes, DiodePreference};
use crate::power::components::mosfet::{find_suitable_mosfets, MOSFETPreference};
use crate::power::types::VoltageRange;
use crate::power::{format_capacitance, format_current, format_inductance, format_voltage};

use super::flyback::{CapacitorType, OutputCapacitor, SelectedDiode, SelectedMOSFET};

// ============================================================================
// OPERATING MODES
// ============================================================================

/// Buck-boost operating mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuckBoostMode {
    /// Continuous Conduction Mode - inductor current never reaches zero
    /// Lower RMS currents but more complex control (RHP zero when stepping up)
    CCM,
    /// Discontinuous Conduction Mode - inductor current reaches zero each cycle
    /// Simpler control but higher peak currents
    DCM,
}

impl BuckBoostMode {
    /// Description of the operating mode
    pub fn description(&self) -> &'static str {
        match self {
            BuckBoostMode::CCM => "Continuous - lower peak currents, RHP zero in boost region",
            BuckBoostMode::DCM => "Discontinuous - higher peaks, simpler control",
        }
    }
}

// ============================================================================
// DESIGN REQUIREMENTS
// ============================================================================

/// Buck-boost converter design requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuckBoostRequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output voltage (magnitude, output is inverted)
    pub vout: f64,
    /// Maximum output current (A)
    pub iout_max: f64,
    /// Minimum output current (A)
    pub iout_min: f64,
    /// Maximum output ripple voltage (V peak-to-peak)
    pub ripple_pp: f64,
    /// Switching frequency (Hz)
    pub switching_freq: f64,
    /// Preferred operating mode (or None for automatic)
    pub mode: Option<BuckBoostMode>,
    /// Ambient temperature (°C)
    pub ambient_temp: f64,
    /// Maximum temperature rise (°C)
    pub max_temp_rise: f64,
    /// Target efficiency (0.0-1.0)
    pub efficiency_target: f64,
    /// Inductor current ripple ratio (ΔI / IL_avg)
    pub inductor_ripple_ratio: f64,
}

impl Default for BuckBoostRequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(9.0, 16.0), // Automotive 12V nominal
            vout: 12.0, // Inverted -12V output
            iout_max: 1.0,
            iout_min: 0.1,
            ripple_pp: 0.12, // 1% of output
            switching_freq: 300e3,
            mode: None,
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.88,
            inductor_ripple_ratio: 0.3,
        }
    }
}

impl BuckBoostRequirements {
    /// Calculate output power
    pub fn output_power(&self) -> f64 {
        self.vout * self.iout_max
    }

    /// Calculate estimated input power
    pub fn estimated_input_power(&self) -> f64 {
        self.output_power() / self.efficiency_target
    }

    /// Calculate duty cycle for given input voltage
    /// D = |Vout| / (Vin + |Vout|)
    pub fn duty_cycle_for_vin(&self, vin: f64) -> f64 {
        self.vout / (vin + self.vout)
    }
}

// ============================================================================
// OUTPUT INDUCTOR
// ============================================================================

/// Inductor design for buck-boost converter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuckBoostInductor {
    /// Inductance value (H)
    pub inductance: f64,
    /// Average inductor current (A)
    pub current_avg: f64,
    /// Peak inductor current (A)
    pub current_peak: f64,
    /// RMS inductor current (A)
    pub current_rms: f64,
    /// Ripple current peak-to-peak (A)
    pub ripple_current_pp: f64,
    /// Estimated DCR (Ω)
    pub dcr: f64,
    /// Estimated core loss (W)
    pub core_loss: f64,
    /// Estimated copper loss (W)
    pub copper_loss: f64,
}

// ============================================================================
// COMPLETE DESIGN
// ============================================================================

/// Complete buck-boost converter design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuckBoostDesign {
    /// Design requirements
    pub requirements: BuckBoostRequirements,
    /// Selected operating mode
    pub mode: BuckBoostMode,
    /// Duty cycle at Vin_min (maximum D)
    pub duty_cycle_max: f64,
    /// Duty cycle at Vin_nom
    pub duty_cycle_nom: f64,
    /// Duty cycle at Vin_max (minimum D)
    pub duty_cycle_min: f64,
    /// Main switch (MOSFET)
    pub main_switch: SelectedMOSFET,
    /// Freewheeling diode
    pub diode: SelectedDiode,
    /// Inductor
    pub inductor: BuckBoostInductor,
    /// Output capacitor
    pub output_capacitor: OutputCapacitor,
    /// Input capacitor
    pub input_capacitor: OutputCapacitor,
    /// Total efficiency estimate
    pub efficiency: f64,
    /// Loss breakdown
    pub losses: BuckBoostLosses,
}

/// Detailed loss breakdown for buck-boost converter
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BuckBoostLosses {
    /// MOSFET conduction loss
    pub mosfet_conduction: f64,
    /// MOSFET switching loss
    pub mosfet_switching: f64,
    /// MOSFET gate drive loss
    pub mosfet_gate: f64,
    /// Diode conduction loss
    pub diode_conduction: f64,
    /// Diode reverse recovery loss
    pub diode_recovery: f64,
    /// Inductor core loss
    pub inductor_core: f64,
    /// Inductor copper loss
    pub inductor_copper: f64,
    /// Input capacitor ESR loss
    pub input_cap_esr: f64,
    /// Output capacitor ESR loss
    pub output_cap_esr: f64,
    /// Total losses
    pub total: f64,
}

// ============================================================================
// DESIGN ALGORITHM
// ============================================================================

/// Design a buck-boost converter from requirements
pub fn design_buck_boost(req: &BuckBoostRequirements) -> Result<BuckBoostDesign, String> {
    // Validate inputs
    if req.output_power() <= 0.0 {
        return Err("Output power must be positive".to_string());
    }
    if req.switching_freq < 10e3 || req.switching_freq > 5e6 {
        return Err("Switching frequency must be between 10kHz and 5MHz".to_string());
    }

    let vin_min = req.vin.min_v;
    let vin_max = req.vin.max_v;
    let vin_nom = req.vin.nom_v;
    let vout = req.vout; // Magnitude of output voltage
    let iout = req.iout_max;
    let fsw = req.switching_freq;
    let pout = req.output_power();

    // Forward voltage drops
    let vd = 0.5; // Diode forward drop

    // Calculate duty cycles
    // D = Vout / (Vin + Vout) accounting for diode drop
    let duty_max = (vout + vd) / (vin_min + vout + vd);
    let duty_nom = (vout + vd) / (vin_nom + vout + vd);
    let duty_min = (vout + vd) / (vin_max + vout + vd);

    if duty_max > 0.85 {
        return Err(format!(
            "Duty cycle too high ({:.1}%). Consider higher input voltage or lower output.",
            duty_max * 100.0
        ));
    }

    // Average inductor current: IL = Iout / (1-D)
    let il_avg_max = iout / (1.0 - duty_max);
    let il_avg_nom = iout / (1.0 - duty_nom);

    // Select operating mode
    let mode = req.mode.unwrap_or_else(|| {
        // Default to CCM for lower ripple currents
        // DCM is simpler for light loads
        if iout < 0.5 && pout < 5.0 {
            BuckBoostMode::DCM
        } else {
            BuckBoostMode::CCM
        }
    });

    // Design inductor
    // ΔIL = Vin × D / (L × fsw) or equivalently Vout × (1-D) / (L × fsw)
    // L = Vin × D / (ΔIL × fsw)
    let delta_il = il_avg_nom * req.inductor_ripple_ratio;
    let l_inductor = vin_nom * duty_nom / (delta_il * fsw);

    // Ensure CCM operation if specified
    let l_min_ccm = if mode == BuckBoostMode::CCM {
        // Critical inductance for CCM: L_crit = Vin × D × (1-D)² / (2 × Iout × fsw)
        vin_min * duty_max * (1.0 - duty_max).powi(2) / (2.0 * iout * fsw)
    } else {
        0.0
    };

    let l_final = l_inductor.max(l_min_ccm);

    // Recalculate ripple with final inductance
    let actual_delta_il = vin_nom * duty_nom / (l_final * fsw);
    let il_peak = il_avg_max + actual_delta_il / 2.0;
    let il_rms = (il_avg_max.powi(2) + (actual_delta_il / (2.0 * 3.0_f64.sqrt())).powi(2)).sqrt();

    // Estimate inductor losses
    let dcr_estimate = 0.01; // 10mΩ estimate
    let inductor_copper_loss = il_rms.powi(2) * dcr_estimate;
    let inductor_core_loss = 0.15; // Rough estimate

    let inductor = BuckBoostInductor {
        inductance: l_final,
        current_avg: il_avg_max,
        current_peak: il_peak,
        current_rms: il_rms,
        ripple_current_pp: actual_delta_il,
        dcr: dcr_estimate,
        core_loss: inductor_core_loss,
        copper_loss: inductor_copper_loss,
    };

    // Switch stress
    // MOSFET sees Vin + Vout when off
    let vds_max = vin_max + vout + vd;
    let id_peak = il_peak;
    let id_rms = il_rms * duty_nom.sqrt();

    // Select MOSFET
    let vds_margin = vds_max * 1.25;
    let mosfet_candidates = find_suitable_mosfets(
        vds_margin,
        id_rms,
        id_peak,
        MOSFETPreference::LowLosses,
    );

    let selected_mosfet = mosfet_candidates
        .first()
        .ok_or_else(|| format!("No suitable MOSFET found for Vds={:.0}V, Id={:.2}A", vds_margin, id_peak))
        .map(|m| (*m).clone())?;

    // Calculate MOSFET losses
    let rds_on_hot = selected_mosfet.rds_on_25c * 1.5;
    let mosfet_conduction = id_rms.powi(2) * rds_on_hot;

    let t_rise_fall = 30e-9;
    let mosfet_switching = 0.5 * vds_max * id_peak * t_rise_fall * fsw;
    let mosfet_gate = selected_mosfet.qg_total * 10.0 * fsw;

    let main_switch = SelectedMOSFET {
        spec: selected_mosfet.clone(),
        vds_peak: vds_max,
        id_rms,
        id_peak,
        loss_conduction: mosfet_conduction,
        loss_switching: mosfet_switching,
        loss_total: mosfet_conduction + mosfet_switching + mosfet_gate,
    };

    // Diode stress
    // Diode also sees Vin + Vout
    let vr_diode = vds_max;
    let if_avg = iout;
    let _if_rms = il_rms * (1.0 - duty_nom).sqrt(); // For RMS rating validation

    let diode_candidates = find_suitable_diodes(
        vr_diode * 1.3,
        if_avg,
        il_peak,
        DiodePreference::LowVf,
    );

    let selected_diode = diode_candidates
        .first()
        .ok_or_else(|| "No suitable diode found".to_string())
        .map(|d| (*d).clone())?;

    let diode_conduction = if_avg * selected_diode.vf_typical;
    let diode_recovery = 0.0; // Schottky assumed

    let diode = SelectedDiode {
        spec: selected_diode.clone(),
        vr_peak: vr_diode,
        if_avg,
        if_peak: il_peak,
        loss_conduction: diode_conduction,
        loss_recovery: diode_recovery,
        loss_total: diode_conduction,
    };

    // Output capacitor sizing
    // Ripple from capacitance: ΔVout = Iout × D / (C × fsw)
    let c_out_min = iout * duty_nom / (req.ripple_pp * fsw);
    let c_out_ripple_rms = iout * duty_nom.sqrt(); // Output current is pulsating

    // ESR contribution: ΔV_esr = IL_peak × ESR
    let max_esr = req.ripple_pp * 0.5 / il_peak;

    let output_capacitor = OutputCapacitor {
        capacitance: c_out_min * 2.0, // Add margin
        voltage_rating: (vout * 1.5).ceil(),
        max_esr,
        ripple_current_rms: c_out_ripple_rms,
        cap_type: CapacitorType::Ceramic,
        parallel_count: 1,
    };

    // Input capacitor sizing
    // Input current is also pulsating: Iin = IL during D, 0 during (1-D)
    let iin_rms = il_rms * (duty_nom * (1.0 - duty_nom)).sqrt();
    let c_in_min = iin_rms / (fsw * 0.02 * vin_min);
    let cin_esr = 0.02 * vin_min / il_peak;

    let input_capacitor = OutputCapacitor {
        capacitance: c_in_min * 2.0,
        voltage_rating: (vin_max * 1.25).ceil(),
        max_esr: cin_esr,
        ripple_current_rms: iin_rms,
        cap_type: CapacitorType::Ceramic,
        parallel_count: 1,
    };

    // Calculate total losses
    let input_cap_esr = iin_rms.powi(2) * cin_esr * 0.1;
    let output_cap_esr = c_out_ripple_rms.powi(2) * max_esr * 0.1;

    let losses = BuckBoostLosses {
        mosfet_conduction,
        mosfet_switching,
        mosfet_gate,
        diode_conduction,
        diode_recovery,
        inductor_core: inductor_core_loss,
        inductor_copper: inductor_copper_loss,
        input_cap_esr,
        output_cap_esr,
        total: 0.0,
    };

    let total_losses = losses.mosfet_conduction
        + losses.mosfet_switching
        + losses.mosfet_gate
        + losses.diode_conduction
        + losses.diode_recovery
        + losses.inductor_core
        + losses.inductor_copper
        + losses.input_cap_esr
        + losses.output_cap_esr;

    let efficiency = pout / (pout + total_losses);

    let mut losses = losses;
    losses.total = total_losses;

    Ok(BuckBoostDesign {
        requirements: req.clone(),
        mode,
        duty_cycle_max: duty_max,
        duty_cycle_nom: duty_nom,
        duty_cycle_min: duty_min,
        main_switch,
        diode,
        inductor,
        output_capacitor,
        input_capacitor,
        efficiency,
        losses,
    })
}

// ============================================================================
// DISPLAY IMPLEMENTATION
// ============================================================================

impl BuckBoostDesign {
    /// Generate a summary string
    pub fn summary(&self) -> String {
        let req = &self.requirements;
        format!(
            "Inverting Buck-Boost Converter Design\n\
             =====================================\n\
             Input: {}-{} V\n\
             Output: -{} @ {}\n\
             Power: {:.1} W\n\
             Frequency: {:.0} kHz\n\
             Mode: {:?}\n\
             \n\
             Duty Cycle: {:.1}%-{:.1}%\n\
             \n\
             MOSFET: {} (Vds_pk={:.0}V)\n\
             Diode: {} (Vr={:.0}V)\n\
             \n\
             Inductor: {} (IL_avg={:.2}A, ΔI={:.2}A)\n\
             Output Cap: {} (ESR<{:.0}mΩ)\n\
             Input Cap: {}\n\
             \n\
             Efficiency: {:.1}%\n\
             Total Losses: {:.2} W",
            format_voltage(req.vin.min_v),
            format_voltage(req.vin.max_v),
            format_voltage(req.vout),
            format_current(req.iout_max),
            req.output_power(),
            req.switching_freq / 1000.0,
            self.mode,
            self.duty_cycle_min * 100.0,
            self.duty_cycle_max * 100.0,
            self.main_switch.spec.part_number,
            self.main_switch.vds_peak,
            self.diode.spec.part_number,
            self.diode.vr_peak,
            format_inductance(self.inductor.inductance),
            self.inductor.current_avg,
            self.inductor.ripple_current_pp,
            format_capacitance(self.output_capacitor.capacitance),
            self.output_capacitor.max_esr * 1000.0,
            format_capacitance(self.input_capacitor.capacitance),
            self.efficiency * 100.0,
            self.losses.total,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_boost_basic_design() {
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(9.0, 16.0), // 12V automotive
            vout: 12.0, // -12V output
            iout_max: 0.5,
            iout_min: 0.05,
            ripple_pp: 0.12,
            switching_freq: 300e3,
            mode: None,
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
        };

        let result = design_buck_boost(&req);
        assert!(result.is_ok(), "Design should succeed: {:?}", result.err());

        let design = result.unwrap();

        // Check duty cycle range is reasonable
        assert!(design.duty_cycle_min > 0.3);
        assert!(design.duty_cycle_max < 0.7);

        // Check efficiency is reasonable
        assert!(design.efficiency > 0.75, "Efficiency {:.1}% too low", design.efficiency * 100.0);
        assert!(design.efficiency < 0.98);

        // Inductor should have reasonable value
        assert!(design.inductor.inductance > 1e-6);
    }

    #[test]
    fn test_buck_boost_step_down() {
        // Stepping down: 24V to -12V
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(20.0, 28.0),
            vout: 12.0,
            iout_max: 1.0,
            iout_min: 0.1,
            ripple_pp: 0.12,
            switching_freq: 250e3,
            mode: Some(BuckBoostMode::CCM),
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.88,
            inductor_ripple_ratio: 0.25,
        };

        let result = design_buck_boost(&req);
        assert!(result.is_ok(), "Step-down design should succeed: {:?}", result.err());

        let design = result.unwrap();

        // For step-down, duty cycle should be less than 0.5 at nominal input
        assert!(design.duty_cycle_nom < 0.5, "Step-down should have D < 0.5");
    }

    #[test]
    fn test_buck_boost_step_up() {
        // Stepping up: 5V to -12V
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(4.5, 5.5),
            vout: 12.0,
            iout_max: 0.3,
            iout_min: 0.03,
            ripple_pp: 0.12,
            switching_freq: 400e3,
            mode: Some(BuckBoostMode::CCM),
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
        };

        let result = design_buck_boost(&req);
        assert!(result.is_ok(), "Step-up design should succeed: {:?}", result.err());

        let design = result.unwrap();

        // For step-up, duty cycle should be greater than 0.5
        assert!(design.duty_cycle_nom > 0.5, "Step-up should have D > 0.5");

        // Inductor current should be higher than output current
        assert!(design.inductor.current_avg > req.iout_max);
    }

    #[test]
    fn test_buck_boost_dcm_mode() {
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 5.0,
            iout_max: 0.2,
            iout_min: 0.02,
            ripple_pp: 0.05,
            switching_freq: 500e3,
            mode: Some(BuckBoostMode::DCM),
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.82,
            inductor_ripple_ratio: 0.5, // Higher ripple for DCM
        };

        let result = design_buck_boost(&req);
        assert!(result.is_ok(), "DCM design should succeed: {:?}", result.err());

        let design = result.unwrap();
        assert_eq!(design.mode, BuckBoostMode::DCM);
    }

    #[test]
    fn test_duty_cycle_calculation() {
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 12.0,
            ..Default::default()
        };

        // At Vin = 12V, Vout = 12V: D = 12 / (12 + 12) = 0.5
        let d = req.duty_cycle_for_vin(12.0);
        assert!((d - 0.5).abs() < 0.01, "D should be ~0.5 when Vin = Vout");

        // At Vin = 6V, Vout = 12V: D = 12 / (6 + 12) = 0.67
        let d = req.duty_cycle_for_vin(6.0);
        assert!((d - 0.667).abs() < 0.02);

        // At Vin = 24V, Vout = 12V: D = 12 / (24 + 12) = 0.33
        let d = req.duty_cycle_for_vin(24.0);
        assert!((d - 0.333).abs() < 0.02);
    }

    #[test]
    fn test_component_stress() {
        let req = BuckBoostRequirements {
            vin: VoltageRange::range(9.0, 16.0),
            vout: 15.0,
            iout_max: 0.5,
            iout_min: 0.05,
            ripple_pp: 0.15,
            switching_freq: 300e3,
            mode: Some(BuckBoostMode::CCM),
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
        };

        let result = design_buck_boost(&req);
        assert!(result.is_ok());

        let design = result.unwrap();

        // MOSFET and diode should see Vin_max + Vout
        let expected_voltage = req.vin.max_v + req.vout + 0.5; // Plus diode drop
        assert!(design.main_switch.vds_peak >= expected_voltage * 0.95);
        assert!(design.diode.vr_peak >= expected_voltage * 0.95);
    }
}
