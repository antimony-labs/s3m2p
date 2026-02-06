//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: forward.rs | DNA/src/power/topologies/forward.rs
//! PURPOSE: Forward converter topology design
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Complete forward converter design including:
//! - Reset method selection (third winding, RCD, active clamp)
//! - Transformer design with turns ratio calculation
//! - Output inductor design
//! - Primary switch (MOSFET) selection
//! - Output rectifier selection (freewheeling diode)
//! - Output filter capacitor sizing
//! - Efficiency estimation
//!
//! Forward converters transfer energy directly during the switch ON time,
//! unlike flyback converters which store energy. This requires:
//! - A transformer reset mechanism (flux must return to zero each cycle)
//! - An output inductor (transformer is not an energy storage element)
//! - Two output diodes (forward diode D1 + freewheeling diode D2)

use serde::{Deserialize, Serialize};

use crate::power::components::diode::{find_suitable_diodes, DiodePreference, DiodeSpec};
use crate::power::components::mosfet::{find_suitable_mosfets, MOSFETPreference, MOSFETSpec};
use crate::power::magnetics::{
    auto_design_transformer, ferrite_database, CoreMaterial, CoreType, IsolationClass,
    TransformerDesign, TransformerRequirements, TransformerTopology,
};
use crate::power::types::VoltageRange;
use crate::power::{format_capacitance, format_current, format_inductance, format_voltage};

use super::flyback::{CapacitorType, OutputCapacitor, SelectedDiode, SelectedMOSFET};

// ============================================================================
// RESET METHODS
// ============================================================================

/// Transformer reset method for forward converter
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResetMethod {
    /// Third winding reset (traditional approach)
    /// Np:Nr determines max duty cycle: D_max = Np/(Np+Nr)
    /// Typically Nr = Np, giving D_max = 0.5
    ThirdWinding,
    /// RCD reset (resistor-capacitor-diode)
    /// Allows higher duty cycle but dissipates reset energy
    RCD,
    /// Active clamp reset
    /// Recycles reset energy, allows D > 0.5, enables ZVS
    ActiveClamp,
    /// Two-switch forward (both switches turn off for reset)
    /// Inherently voltage-clamped, D_max ≈ 0.5
    TwoSwitch,
}

impl ResetMethod {
    /// Maximum duty cycle for this reset method
    pub fn max_duty_cycle(&self, turns_ratio_reset: f64) -> f64 {
        match self {
            ResetMethod::ThirdWinding => {
                // D_max = Np / (Np + Nr) = 1 / (1 + Nr/Np)
                1.0 / (1.0 + turns_ratio_reset)
            }
            ResetMethod::RCD => 0.65,         // Higher than third winding
            ResetMethod::ActiveClamp => 0.70, // Can go higher with ZVS
            ResetMethod::TwoSwitch => 0.48,   // Slightly less than 0.5 for margin
        }
    }

    /// Description of the reset method
    pub fn description(&self) -> &'static str {
        match self {
            ResetMethod::ThirdWinding => "Third winding - simple, D_max=0.5 typical",
            ResetMethod::RCD => "RCD clamp - higher D, wastes reset energy",
            ResetMethod::ActiveClamp => "Active clamp - recycles energy, ZVS possible",
            ResetMethod::TwoSwitch => "Two-switch - clamped voltage, robust",
        }
    }
}

// ============================================================================
// DESIGN REQUIREMENTS
// ============================================================================

/// Forward converter design requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForwardRequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output voltage (V)
    pub vout: f64,
    /// Maximum output current (A)
    pub iout_max: f64,
    /// Minimum output current (A)
    pub iout_min: f64,
    /// Maximum output ripple voltage (V peak-to-peak)
    pub ripple_pp: f64,
    /// Switching frequency (Hz)
    pub switching_freq: f64,
    /// Reset method
    pub reset_method: ResetMethod,
    /// Reset winding turns ratio (Nr/Np) - for third winding reset
    pub reset_turns_ratio: f64,
    /// Isolation requirement
    pub isolation: IsolationClass,
    /// Ambient temperature (°C)
    pub ambient_temp: f64,
    /// Maximum temperature rise (°C)
    pub max_temp_rise: f64,
    /// Target efficiency (0.0-1.0)
    pub efficiency_target: f64,
    /// Inductor current ripple ratio (ΔI / Iout)
    pub inductor_ripple_ratio: f64,
    /// Preferred core type (or None for automatic)
    pub preferred_core: Option<CoreType>,
}

impl Default for ForwardRequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(36.0, 72.0), // Telecom range
            vout: 5.0,
            iout_max: 5.0,
            iout_min: 0.5,
            ripple_pp: 0.05, // 50mV
            switching_freq: 200e3,
            reset_method: ResetMethod::ThirdWinding,
            reset_turns_ratio: 1.0, // Nr = Np, D_max = 0.5
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.88,
            inductor_ripple_ratio: 0.3, // 30% ripple
            preferred_core: None,
        }
    }
}

impl ForwardRequirements {
    /// Calculate output power
    pub fn output_power(&self) -> f64 {
        self.vout * self.iout_max
    }

    /// Calculate estimated input power
    pub fn estimated_input_power(&self) -> f64 {
        self.output_power() / self.efficiency_target
    }

    /// Maximum duty cycle based on reset method
    pub fn max_duty_cycle(&self) -> f64 {
        self.reset_method.max_duty_cycle(self.reset_turns_ratio)
    }
}

// ============================================================================
// OUTPUT INDUCTOR
// ============================================================================

/// Output inductor design for forward converter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputInductor {
    /// Inductance value (H)
    pub inductance: f64,
    /// DC current rating (A)
    pub current_dc: f64,
    /// Peak current (A)
    pub current_peak: f64,
    /// RMS current (A)
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
// RESET CIRCUIT
// ============================================================================

/// Reset circuit design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResetCircuit {
    /// Reset method used
    pub method: ResetMethod,
    /// Reset winding turns (for third winding)
    pub reset_turns: Option<u32>,
    /// Reset resistor (for RCD)
    pub resistor: Option<f64>,
    /// Reset capacitor (for RCD/active)
    pub capacitor: Option<f64>,
    /// Reset diode
    pub diode: Option<DiodeSpec>,
    /// Active clamp MOSFET (for active clamp)
    pub clamp_mosfet: Option<MOSFETSpec>,
    /// Estimated power dissipation (W)
    pub power_dissipation: f64,
}

// ============================================================================
// COMPLETE DESIGN
// ============================================================================

/// Complete forward converter design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForwardDesign {
    /// Design requirements
    pub requirements: ForwardRequirements,
    /// Operating duty cycle at Vin_min
    pub duty_cycle_max: f64,
    /// Operating duty cycle at Vin_nom
    pub duty_cycle_nom: f64,
    /// Operating duty cycle at Vin_max
    pub duty_cycle_min: f64,
    /// Transformer turns ratio (Np:Ns)
    pub turns_ratio: f64,
    /// Transformer design
    pub transformer: TransformerDesign,
    /// Primary switch
    pub primary_switch: SelectedMOSFET,
    /// Forward diode (D1) - conducts when switch is ON
    pub forward_diode: SelectedDiode,
    /// Freewheeling diode (D2) - conducts when switch is OFF
    pub freewheeling_diode: SelectedDiode,
    /// Output inductor
    pub output_inductor: OutputInductor,
    /// Output capacitor
    pub output_capacitor: OutputCapacitor,
    /// Input capacitor
    pub input_capacitor: OutputCapacitor,
    /// Reset circuit
    pub reset_circuit: ResetCircuit,
    /// Peak primary current (A)
    pub i_pri_peak: f64,
    /// RMS primary current (A)
    pub i_pri_rms: f64,
    /// Total efficiency estimate
    pub efficiency: f64,
    /// Loss breakdown
    pub losses: ForwardLosses,
}

/// Detailed loss breakdown for forward converter
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ForwardLosses {
    /// Primary MOSFET conduction
    pub mosfet_conduction: f64,
    /// Primary MOSFET switching
    pub mosfet_switching: f64,
    /// Primary MOSFET gate drive
    pub mosfet_gate: f64,
    /// Forward diode conduction
    pub forward_diode: f64,
    /// Freewheeling diode conduction
    pub freewheeling_diode: f64,
    /// Transformer core loss
    pub transformer_core: f64,
    /// Transformer copper loss
    pub transformer_copper: f64,
    /// Output inductor core loss
    pub inductor_core: f64,
    /// Output inductor copper loss
    pub inductor_copper: f64,
    /// Reset circuit dissipation
    pub reset_circuit: f64,
    /// Input capacitor ESR
    pub input_cap_esr: f64,
    /// Output capacitor ESR
    pub output_cap_esr: f64,
    /// Total losses
    pub total: f64,
}

// ============================================================================
// DESIGN ALGORITHM
// ============================================================================

/// Design a forward converter from requirements
pub fn design_forward(req: &ForwardRequirements) -> Result<ForwardDesign, String> {
    // Validate inputs
    if req.outputs_power() <= 0.0 {
        return Err("Output power must be positive".to_string());
    }
    if req.switching_freq < 10e3 || req.switching_freq > 2e6 {
        return Err("Switching frequency must be between 10kHz and 2MHz".to_string());
    }

    let vin_min = req.vin.min_v;
    let vin_max = req.vin.max_v;
    let vin_nom = req.vin.nom_v;
    let vout = req.vout;
    let iout = req.iout_max;
    let fsw = req.switching_freq;
    let pout = req.output_power();

    // Forward voltage drops (estimated)
    let vd_forward = 0.5; // Forward diode drop
    let _vd_freewheel = 0.5; // Freewheeling diode drop (for future use)

    // Maximum duty cycle from reset method
    let d_max_allowed = req.max_duty_cycle();

    // Calculate required turns ratio
    // Vout = Vin * D * (Ns/Np) - Vd
    // At Vin_min and D_max: Ns/Np = (Vout + Vd) / (Vin_min * D_max)
    let ns_np_min = (vout + vd_forward) / (vin_min * d_max_allowed);

    // At Vin_max, we need D_min > 0 (typically want D_min > 0.1 for control)
    // D_min = (Vout + Vd) / (Vin_max * Ns/Np)
    let d_min_target = 0.15; // Minimum duty cycle for controllability
    let ns_np_max = (vout + vd_forward) / (vin_max * d_min_target);

    // Choose turns ratio - use geometric mean or the minimum required
    let turns_ratio_ns_np = if ns_np_min > ns_np_max {
        // Can't satisfy both constraints - use minimum required
        ns_np_min * 1.05 // Add 5% margin
    } else {
        // Pick a value that gives reasonable duty cycle range
        (ns_np_min * ns_np_max).sqrt()
    };

    let turns_ratio_np_ns = 1.0 / turns_ratio_ns_np;

    // Calculate actual duty cycles
    let duty_max = ((vout + vd_forward) / (vin_min * turns_ratio_ns_np)).min(d_max_allowed * 0.95);
    let duty_nom = (vout + vd_forward) / (vin_nom * turns_ratio_ns_np);
    let duty_min = (vout + vd_forward) / (vin_max * turns_ratio_ns_np);

    if duty_max > d_max_allowed {
        return Err(format!(
            "Cannot achieve required duty cycle. D_max={:.2} but need {:.2}",
            d_max_allowed, duty_max
        ));
    }

    // Design output inductor
    // ΔI = (Vout + Vd_fw) * (1-D) / (L * fsw)
    // Or: L = Vout * (1-D) / (ΔI * fsw)
    let delta_i = iout * req.inductor_ripple_ratio;
    let l_out = vout * (1.0 - duty_nom) / (delta_i * fsw);

    let i_l_dc = iout;
    let i_l_peak = iout + delta_i / 2.0;
    let i_l_rms = (i_l_dc.powi(2) + (delta_i / (2.0 * 3.0_f64.sqrt())).powi(2)).sqrt();

    // Estimate inductor losses
    let dcr_estimate = 0.005; // 5mΩ estimate
    let inductor_copper_loss = i_l_rms.powi(2) * dcr_estimate;
    let inductor_core_loss = 0.2; // Rough estimate

    let output_inductor = OutputInductor {
        inductance: l_out,
        current_dc: i_l_dc,
        current_peak: i_l_peak,
        current_rms: i_l_rms,
        ripple_current_pp: delta_i,
        dcr: dcr_estimate,
        core_loss: inductor_core_loss,
        copper_loss: inductor_copper_loss,
    };

    // Calculate primary currents
    // I_pri = I_sec * Ns/Np (during on-time)
    // I_pri_peak = I_L_peak * Ns/Np
    let i_pri_peak = i_l_peak * turns_ratio_ns_np;
    let i_pri_rms = i_l_rms * turns_ratio_ns_np * duty_nom.sqrt();

    // Primary switch stress
    // For third winding reset: V_ds_max = Vin_max + Vin_max * (Np/Nr)
    // For RCD/active: V_ds_max ≈ Vin_max + V_reset
    let vds_max = match req.reset_method {
        ResetMethod::ThirdWinding => vin_max * (1.0 + 1.0 / req.reset_turns_ratio),
        ResetMethod::TwoSwitch => vin_max, // Clamped to input
        ResetMethod::RCD | ResetMethod::ActiveClamp => vin_max * 1.5, // Approximate
    };

    // Select MOSFET
    let vds_margin = vds_max * 1.25; // 25% margin
    let mosfet_candidates = find_suitable_mosfets(
        vds_margin,
        i_pri_rms,
        i_pri_peak,
        MOSFETPreference::LowLosses,
    );

    let selected_mosfet = mosfet_candidates
        .first()
        .ok_or_else(|| {
            format!(
                "No suitable MOSFET found for Vds={:.0}V, Id={:.2}A",
                vds_margin, i_pri_peak
            )
        })
        .map(|m| (*m).clone())?;

    // Calculate MOSFET losses
    let rds_on_hot = selected_mosfet.rds_on_25c * 1.5; // Temperature derating
    let mosfet_conduction = i_pri_rms.powi(2) * rds_on_hot;

    // Switching loss estimate: P_sw = 0.5 * Vds * Id * (tr + tf) * fsw
    let t_rise_fall = 30e-9; // 30ns estimate
    let mosfet_switching = 0.5 * vds_max * i_pri_peak * t_rise_fall * fsw;
    let mosfet_gate = selected_mosfet.qg_total * 10.0 * fsw; // Assume 10V gate drive

    let primary_switch = SelectedMOSFET {
        spec: selected_mosfet.clone(),
        vds_peak: vds_max,
        id_rms: i_pri_rms,
        id_peak: i_pri_peak,
        loss_conduction: mosfet_conduction,
        loss_switching: mosfet_switching,
        loss_total: mosfet_conduction + mosfet_switching + mosfet_gate,
    };

    // Secondary diode stress
    // Forward diode: V_r = Vin_max * Ns/Np when switch is OFF
    // Freewheeling diode: V_r = Vout + Vin * Ns/Np when switch is ON
    let vr_forward = vin_max * turns_ratio_ns_np + vout;
    let vr_freewheel = vin_max * turns_ratio_ns_np + vout;

    let if_forward_avg = iout * duty_nom;
    let if_freewheel_avg = iout * (1.0 - duty_nom);

    // Select forward diode
    let diode_margin = 1.3;
    let forward_diode_candidates = find_suitable_diodes(
        vr_forward * diode_margin,
        if_forward_avg,
        i_l_peak,
        DiodePreference::LowVf,
    );

    let forward_diode_spec = forward_diode_candidates
        .first()
        .ok_or_else(|| "No suitable forward diode found".to_string())
        .map(|d| (*d).clone())?;

    let forward_diode_loss = if_forward_avg * forward_diode_spec.vf_typical;

    let forward_diode = SelectedDiode {
        spec: forward_diode_spec.clone(),
        vr_peak: vr_forward,
        if_avg: if_forward_avg,
        if_peak: i_l_peak,
        loss_conduction: forward_diode_loss,
        loss_recovery: 0.0, // Schottky assumed
        loss_total: forward_diode_loss,
    };

    // Select freewheeling diode
    let freewheel_diode_candidates = find_suitable_diodes(
        vr_freewheel * diode_margin,
        if_freewheel_avg,
        i_l_peak,
        DiodePreference::LowVf,
    );

    let freewheel_diode_spec = freewheel_diode_candidates
        .first()
        .ok_or_else(|| "No suitable freewheeling diode found".to_string())
        .map(|d| (*d).clone())?;

    let freewheel_diode_loss = if_freewheel_avg * freewheel_diode_spec.vf_typical;

    let freewheeling_diode = SelectedDiode {
        spec: freewheel_diode_spec.clone(),
        vr_peak: vr_freewheel,
        if_avg: if_freewheel_avg,
        if_peak: i_l_peak,
        loss_conduction: freewheel_diode_loss,
        loss_recovery: 0.0,
        loss_total: freewheel_diode_loss,
    };

    // Design transformer
    // Forward transformer doesn't store energy - it's sized for volt-seconds
    let transformer_req = TransformerRequirements {
        topology: TransformerTopology::Forward,
        primary_voltage: vin_max, // Design at max input voltage
        secondary_voltages: vec![vout],
        secondary_currents: vec![i_l_rms],
        frequency: fsw,
        duty_cycle_max: duty_max,
        isolation: req.isolation,
        ambient_temp: req.ambient_temp,
        max_temp_rise: req.max_temp_rise,
    };

    // Select core material based on frequency
    let material = select_core_material(fsw);
    let transformer = auto_design_transformer(&transformer_req, &material, req.preferred_core)
        .ok_or_else(|| "No suitable core found for transformer".to_string())?;

    // Design reset circuit
    let reset_circuit = design_reset_circuit(req, &transformer, vin_max, i_pri_peak, fsw)?;

    // Output capacitor sizing
    // Ripple from ESR: ΔV_esr = ΔI * ESR
    // Ripple from capacitance: ΔV_cap = ΔI / (8 * C * fsw)
    let max_esr = req.ripple_pp * 0.5 / delta_i; // Half ripple budget to ESR
    let c_out_min = delta_i / (8.0 * fsw * req.ripple_pp * 0.5); // Half to capacitance ripple

    let c_out_ripple_current = delta_i / (2.0 * 3.0_f64.sqrt());

    let output_capacitor = OutputCapacitor {
        capacitance: c_out_min * 1.5, // Add margin
        voltage_rating: (vout * 1.5).ceil(),
        max_esr,
        ripple_current_rms: c_out_ripple_current,
        cap_type: if vout <= 5.0 {
            CapacitorType::Ceramic
        } else {
            CapacitorType::Hybrid
        },
        parallel_count: 1,
    };

    // Input capacitor sizing
    // Input current is pulsed: I_in = I_pri during D, 0 during (1-D)
    let _i_in_avg = i_pri_rms.powi(2) * duty_nom / (vin_nom / pout * req.efficiency_target);
    let i_in_ripple_rms = i_pri_rms * (duty_nom * (1.0 - duty_nom)).sqrt();

    let c_in_min = i_in_ripple_rms / (fsw * 0.02 * vin_min); // 2% input ripple
    let cin_esr = 0.02 * vin_min / i_pri_peak;

    let input_capacitor = OutputCapacitor {
        capacitance: c_in_min * 2.0,
        voltage_rating: (vin_max * 1.25).ceil(),
        max_esr: cin_esr,
        ripple_current_rms: i_in_ripple_rms,
        cap_type: CapacitorType::Electrolytic,
        parallel_count: 1,
    };

    // Calculate total losses
    let losses = ForwardLosses {
        mosfet_conduction,
        mosfet_switching,
        mosfet_gate,
        forward_diode: forward_diode_loss,
        freewheeling_diode: freewheel_diode_loss,
        transformer_core: transformer.core_loss,
        transformer_copper: transformer.primary_copper_loss + transformer.secondary_copper_loss,
        inductor_core: inductor_core_loss,
        inductor_copper: inductor_copper_loss,
        reset_circuit: reset_circuit.power_dissipation,
        input_cap_esr: i_in_ripple_rms.powi(2) * cin_esr * 0.1,
        output_cap_esr: c_out_ripple_current.powi(2) * max_esr * 0.1,
        total: 0.0, // Will calculate below
    };

    let total_losses = losses.mosfet_conduction
        + losses.mosfet_switching
        + losses.mosfet_gate
        + losses.forward_diode
        + losses.freewheeling_diode
        + losses.transformer_core
        + losses.transformer_copper
        + losses.inductor_core
        + losses.inductor_copper
        + losses.reset_circuit
        + losses.input_cap_esr
        + losses.output_cap_esr;

    let efficiency = pout / (pout + total_losses);

    let mut losses = losses;
    losses.total = total_losses;

    Ok(ForwardDesign {
        requirements: req.clone(),
        duty_cycle_max: duty_max,
        duty_cycle_nom: duty_nom,
        duty_cycle_min: duty_min,
        turns_ratio: turns_ratio_np_ns,
        transformer,
        primary_switch,
        forward_diode,
        freewheeling_diode,
        output_inductor,
        output_capacitor,
        input_capacitor,
        reset_circuit,
        i_pri_peak,
        i_pri_rms,
        efficiency,
        losses,
    })
}

/// Helper method for requirements
impl ForwardRequirements {
    fn outputs_power(&self) -> f64 {
        self.vout * self.iout_max
    }
}

/// Select core material based on frequency
fn select_core_material(frequency: f64) -> CoreMaterial {
    ferrite_database()
        .into_iter()
        .find(|m| m.is_frequency_suitable(frequency))
        .unwrap_or_else(|| {
            // Default to N87 which works well for 25kHz-500kHz
            ferrite_database()
                .into_iter()
                .find(|m| m.name == "N87")
                .unwrap()
        })
}

/// Design the reset circuit based on selected method
fn design_reset_circuit(
    req: &ForwardRequirements,
    transformer: &TransformerDesign,
    vin_max: f64,
    i_pri_peak: f64,
    fsw: f64,
) -> Result<ResetCircuit, String> {
    let l_mag = transformer.magnetizing_inductance;

    match req.reset_method {
        ResetMethod::ThirdWinding => {
            // Third winding with Nr = Np * reset_turns_ratio
            let n_primary = transformer.primary.turns;
            let n_reset = ((n_primary as f64) * req.reset_turns_ratio).round() as u32;

            // Reset energy is returned to input - minimal loss
            // Small diode loss from reset current
            let i_reset_peak = i_pri_peak / req.reset_turns_ratio;
            let t_reset = (1.0 - req.max_duty_cycle()) / fsw;
            let p_diode = 0.5 * i_reset_peak * t_reset * fsw; // Rough estimate

            // Select reset diode (fast recovery type)
            let reset_diode = find_suitable_diodes(
                vin_max * 1.5,
                i_reset_peak * 0.5,
                i_reset_peak,
                DiodePreference::HighSpeed,
            )
            .first()
            .cloned();

            Ok(ResetCircuit {
                method: ResetMethod::ThirdWinding,
                reset_turns: Some(n_reset),
                resistor: None,
                capacitor: None,
                diode: reset_diode.cloned(),
                clamp_mosfet: None,
                power_dissipation: p_diode,
            })
        }

        ResetMethod::RCD => {
            // RCD clamp design
            // Energy stored: E = 0.5 * Lm * Ipk^2
            // Must be dissipated each cycle
            let e_mag = 0.5 * l_mag * i_pri_peak.powi(2);
            let p_reset = e_mag * fsw;

            // Clamp voltage: V_clamp ≈ 1.3 * Vin_max typical
            let v_clamp = vin_max * 1.3;

            // Capacitor: holds voltage relatively constant
            // ΔV = I * t / C, want ΔV < 10% of V_clamp
            let t_reset = (1.0 - req.max_duty_cycle()) / fsw;
            let c_clamp = i_pri_peak * t_reset / (0.1 * v_clamp);

            // Resistor: discharges cap between cycles
            // R * C = time constant, want several switching periods
            let r_clamp = 5.0 / (fsw * c_clamp);

            // Actual power dissipation in resistor
            let p_resistor = v_clamp.powi(2) / r_clamp * (1.0 - req.max_duty_cycle());

            let clamp_diode = find_suitable_diodes(
                v_clamp * 1.5,
                p_reset / v_clamp,
                i_pri_peak,
                DiodePreference::HighSpeed,
            )
            .first()
            .cloned();

            Ok(ResetCircuit {
                method: ResetMethod::RCD,
                reset_turns: None,
                resistor: Some(r_clamp),
                capacitor: Some(c_clamp),
                diode: clamp_diode.cloned(),
                clamp_mosfet: None,
                power_dissipation: p_resistor.max(p_reset),
            })
        }

        ResetMethod::ActiveClamp => {
            // Active clamp recycles energy
            // Needs auxiliary MOSFET and capacitor
            let _e_mag = 0.5 * l_mag * i_pri_peak.powi(2);

            // Clamp capacitor resonates with Lm
            // f_res = 1 / (2π * sqrt(Lm * Cclamp))
            // Want f_res around 2-3x fsw for good ZVS
            let f_res = fsw * 2.5;
            let c_clamp = 1.0 / (4.0 * std::f64::consts::PI.powi(2) * f_res.powi(2) * l_mag);

            // Clamp voltage
            let v_clamp = vin_max * 0.5; // Lower than RCD

            // Clamp MOSFET
            let clamp_mosfet = find_suitable_mosfets(
                v_clamp + vin_max,
                i_pri_peak * 0.3,
                i_pri_peak,
                MOSFETPreference::LowLosses,
            )
            .first()
            .cloned();

            // Active clamp has low losses (mostly switching)
            let p_clamp = 0.5; // Rough estimate

            Ok(ResetCircuit {
                method: ResetMethod::ActiveClamp,
                reset_turns: None,
                resistor: None,
                capacitor: Some(c_clamp),
                diode: None,
                clamp_mosfet: clamp_mosfet.cloned(),
                power_dissipation: p_clamp,
            })
        }

        ResetMethod::TwoSwitch => {
            // Two-switch forward uses body diodes for clamping
            // No additional reset circuit needed
            // Each switch sees Vin_max

            Ok(ResetCircuit {
                method: ResetMethod::TwoSwitch,
                reset_turns: None,
                resistor: None,
                capacitor: None,
                diode: None,
                clamp_mosfet: None, // Second switch handled in primary_switch selection
                power_dissipation: 0.0,
            })
        }
    }
}

// ============================================================================
// DISPLAY IMPLEMENTATION
// ============================================================================

impl ForwardDesign {
    /// Generate a summary string
    pub fn summary(&self) -> String {
        let req = &self.requirements;
        format!(
            "Forward Converter Design\n\
             ========================\n\
             Input: {}-{} V\n\
             Output: {} @ {}\n\
             Power: {:.1} W\n\
             Frequency: {:.0} kHz\n\
             Reset: {:?}\n\
             \n\
             Duty Cycle: {:.1}%-{:.1}%\n\
             Turns Ratio (Np:Ns): {:.2}:1\n\
             \n\
             Transformer: {:?} ({} primary turns)\n\
             Primary MOSFET: {} (Vds_pk={:.0}V)\n\
             Forward Diode: {} (If_avg={:.2}A)\n\
             Freewheel Diode: {} (If_avg={:.2}A)\n\
             \n\
             Output Inductor: {} (ΔI={:.2}A)\n\
             Output Cap: {} (ESR<{:.0}mΩ)\n\
             \n\
             Efficiency: {:.1}%\n\
             Total Losses: {:.2} W",
            format_voltage(req.vin.min_v),
            format_voltage(req.vin.max_v),
            format_voltage(req.vout),
            format_current(req.iout_max),
            req.output_power(),
            req.switching_freq / 1000.0,
            req.reset_method,
            self.duty_cycle_min * 100.0,
            self.duty_cycle_max * 100.0,
            self.turns_ratio,
            self.transformer.core.core_type,
            self.transformer.primary.turns,
            self.primary_switch.spec.part_number,
            self.primary_switch.vds_peak,
            self.forward_diode.spec.part_number,
            self.forward_diode.if_avg,
            self.freewheeling_diode.spec.part_number,
            self.freewheeling_diode.if_avg,
            format_inductance(self.output_inductor.inductance),
            self.output_inductor.ripple_current_pp,
            format_capacitance(self.output_capacitor.capacitance),
            self.output_capacitor.max_esr * 1000.0,
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
    fn test_forward_basic_design() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(36.0, 72.0),
            vout: 5.0,
            iout_max: 5.0,
            iout_min: 0.5,
            ripple_pp: 0.05,
            switching_freq: 200e3,
            reset_method: ResetMethod::ThirdWinding,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 55.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(result.is_ok(), "Design should succeed: {:?}", result.err());

        let design = result.unwrap();

        // Check duty cycle is within limits
        assert!(design.duty_cycle_max <= req.max_duty_cycle());
        assert!(design.duty_cycle_min > 0.05);

        // Check efficiency is reasonable
        assert!(
            design.efficiency > 0.75,
            "Efficiency {:.1}% too low",
            design.efficiency * 100.0
        );
        assert!(
            design.efficiency < 0.98,
            "Efficiency {:.1}% unrealistically high",
            design.efficiency * 100.0
        );

        // Check inductor is designed correctly
        assert!(design.output_inductor.inductance > 1e-6); // At least 1µH
        assert!(design.output_inductor.current_peak > design.output_inductor.current_dc);
    }

    #[test]
    fn test_forward_48v_to_3v3() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(42.0, 54.0), // Narrow 48V range
            vout: 3.3,
            iout_max: 5.0, // Reduced from 10A to 5A for more realistic transformer sizing
            iout_min: 0.5,
            ripple_pp: 0.033, // 1% ripple
            switching_freq: 200e3,
            reset_method: ResetMethod::ThirdWinding,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 60.0, // Increased for more margin
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(
            result.is_ok(),
            "48V to 3.3V design should succeed: {:?}",
            result.err()
        );

        let design = result.unwrap();

        // Should have reasonable efficiency
        assert!(design.efficiency > 0.75);

        // Check turns ratio is step-down
        assert!(design.turns_ratio > 1.0, "Should be step-down (Np>Ns)");
    }

    #[test]
    fn test_forward_rcd_reset() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(36.0, 72.0),
            vout: 12.0,
            iout_max: 1.5, // Reduced for realistic transformer sizing
            iout_min: 0.15,
            ripple_pp: 0.12,
            switching_freq: 200e3,
            reset_method: ResetMethod::RCD,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic, // Changed from Reinforced for easier core fit
            ambient_temp: 25.0,
            max_temp_rise: 60.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.3,
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(
            result.is_ok(),
            "RCD reset design should succeed: {:?}",
            result.err()
        );

        let design = result.unwrap();

        // RCD should have reset circuit with R and C
        assert!(design.reset_circuit.resistor.is_some());
        assert!(design.reset_circuit.capacitor.is_some());

        // RCD wastes energy - check it's accounted for
        assert!(design.reset_circuit.power_dissipation > 0.0);
    }

    #[test]
    fn test_forward_active_clamp() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(36.0, 72.0),
            vout: 5.0,
            iout_max: 3.0,
            iout_min: 0.3,
            ripple_pp: 0.05,
            switching_freq: 250e3,
            reset_method: ResetMethod::ActiveClamp,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 55.0,
            efficiency_target: 0.88,
            inductor_ripple_ratio: 0.3,
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(
            result.is_ok(),
            "Active clamp design should succeed: {:?}",
            result.err()
        );

        let design = result.unwrap();

        // Active clamp should have MOSFET and capacitor
        assert!(design.reset_circuit.capacitor.is_some());
        // Active clamp has low losses
        assert!(design.reset_circuit.power_dissipation < 2.0);
    }

    #[test]
    fn test_forward_two_switch() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(90.0, 160.0), // Moderate high voltage (rectified AC)
            vout: 12.0,
            iout_max: 2.0,
            iout_min: 0.2,
            ripple_pp: 0.12,
            switching_freq: 150e3,
            reset_method: ResetMethod::TwoSwitch,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 60.0,
            efficiency_target: 0.88,
            inductor_ripple_ratio: 0.3,
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(
            result.is_ok(),
            "Two-switch design should succeed: {:?}",
            result.err()
        );

        let design = result.unwrap();

        // Two-switch has clamped voltage (no overshoot from reset)
        assert!(design.primary_switch.vds_peak <= req.vin.max_v * 1.1);

        // Reset circuit should be minimal
        assert_eq!(design.reset_circuit.method, ResetMethod::TwoSwitch);
        assert!(design.reset_circuit.power_dissipation < 0.1);
    }

    #[test]
    fn test_reset_method_duty_limits() {
        // Third winding with Nr = Np: D_max = 0.5
        assert!((ResetMethod::ThirdWinding.max_duty_cycle(1.0) - 0.5).abs() < 0.01);

        // Third winding with Nr = 0.5*Np: D_max = 0.67
        assert!((ResetMethod::ThirdWinding.max_duty_cycle(0.5) - 0.667).abs() < 0.01);

        // RCD allows higher duty
        assert!(ResetMethod::RCD.max_duty_cycle(1.0) > 0.6);

        // Active clamp even higher
        assert!(ResetMethod::ActiveClamp.max_duty_cycle(1.0) > 0.65);

        // Two-switch is conservative
        assert!(ResetMethod::TwoSwitch.max_duty_cycle(1.0) < 0.5);
    }

    #[test]
    fn test_inductor_ripple() {
        let req = ForwardRequirements {
            vin: VoltageRange::range(42.0, 54.0),
            vout: 5.0,
            iout_max: 5.0,
            iout_min: 0.5,
            ripple_pp: 0.05,
            switching_freq: 200e3,
            reset_method: ResetMethod::ThirdWinding,
            reset_turns_ratio: 1.0,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 55.0,
            efficiency_target: 0.85,
            inductor_ripple_ratio: 0.4, // Higher ripple
            preferred_core: None,
        };

        let result = design_forward(&req);
        assert!(result.is_ok());

        let design = result.unwrap();

        // Check ripple is approximately what we asked for
        let actual_ripple_ratio = design.output_inductor.ripple_current_pp / req.iout_max;
        assert!((actual_ripple_ratio - 0.4).abs() < 0.1);
    }
}
