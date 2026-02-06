//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: flyback.rs | DNA/src/power/topologies/flyback.rs
//! PURPOSE: Flyback converter topology design
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Complete flyback converter design including:
//! - Operating mode selection (CCM, DCM, Boundary/CrM)
//! - Transformer design with core selection
//! - Primary switch (MOSFET) selection
//! - Output diode selection
//! - Output capacitor sizing
//! - Snubber/clamp circuit design
//! - Efficiency estimation

use serde::{Deserialize, Serialize};

use crate::power::components::diode::{
    find_suitable_diodes, DiodePreference, DiodeSpec, DiodeType,
};
use crate::power::components::mosfet::{find_suitable_mosfets, MOSFETPreference, MOSFETSpec};
use crate::power::magnetics::{
    auto_design_transformer, ferrite_database, CoreMaterial, CoreType, IsolationClass,
    TransformerDesign, TransformerRequirements, TransformerTopology,
};
use crate::power::types::VoltageRange;
use crate::power::{format_capacitance, format_current, format_inductance, format_voltage};

// ============================================================================
// OPERATING MODES
// ============================================================================

/// Flyback operating mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlybackMode {
    /// Continuous Conduction Mode - inductor current never reaches zero
    /// Pro: Lower peak currents, smaller caps
    /// Con: RHP zero limits bandwidth, requires slope compensation
    CCM,
    /// Discontinuous Conduction Mode - inductor current reaches zero each cycle
    /// Pro: No RHP zero, simpler control, natural valley switching
    /// Con: Higher peak currents, more capacitor ripple
    DCM,
    /// Critical Conduction Mode (Boundary/Transition)
    /// Pro: ZVS possible, good efficiency
    /// Con: Variable frequency or burst mode needed
    CrM,
}

impl FlybackMode {
    /// Description of the operating mode
    pub fn description(&self) -> &'static str {
        match self {
            FlybackMode::CCM => "Continuous - lower peaks, RHP zero present",
            FlybackMode::DCM => "Discontinuous - higher peaks, simpler control",
            FlybackMode::CrM => "Critical/Boundary - ZVS capable, variable frequency",
        }
    }
}

// ============================================================================
// DESIGN REQUIREMENTS
// ============================================================================

/// Output specification for multi-output flyback
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlybackOutput {
    /// Output voltage (V)
    pub voltage: f64,
    /// Maximum output current (A)
    pub current_max: f64,
    /// Minimum output current (A) - for load regulation
    pub current_min: f64,
    /// Maximum ripple voltage (peak-to-peak, V)
    pub ripple_pp: f64,
    /// Whether this is the regulated (feedback) output
    pub is_regulated: bool,
}

impl FlybackOutput {
    /// Create a new output specification
    pub fn new(voltage: f64, current: f64) -> Self {
        Self {
            voltage,
            current_max: current,
            current_min: current * 0.1,
            ripple_pp: voltage * 0.02, // 2% ripple default
            is_regulated: false,
        }
    }

    /// Create regulated output (main feedback)
    pub fn regulated(voltage: f64, current: f64) -> Self {
        Self {
            voltage,
            current_max: current,
            current_min: current * 0.1,
            ripple_pp: voltage * 0.01, // Tighter ripple for regulated
            is_regulated: true,
        }
    }

    /// Power at maximum load
    pub fn power_max(&self) -> f64 {
        self.voltage * self.current_max
    }
}

/// Flyback converter design requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlybackRequirements {
    /// Input voltage range
    pub vin: VoltageRange,
    /// Output specifications
    pub outputs: Vec<FlybackOutput>,
    /// Switching frequency (Hz)
    pub switching_freq: f64,
    /// Preferred operating mode (or None for automatic)
    pub mode: Option<FlybackMode>,
    /// Maximum duty cycle (typically 0.45-0.5 to allow reset time)
    pub duty_cycle_max: f64,
    /// Isolation requirement
    pub isolation: IsolationClass,
    /// Ambient temperature (°C)
    pub ambient_temp: f64,
    /// Maximum temperature rise (°C)
    pub max_temp_rise: f64,
    /// Target efficiency (0.0-1.0)
    pub efficiency_target: f64,
    /// Preferred core type (or None for automatic)
    pub preferred_core: Option<CoreType>,
}

impl Default for FlybackRequirements {
    fn default() -> Self {
        Self {
            vin: VoltageRange::range(36.0, 72.0), // Telecom range
            outputs: vec![FlybackOutput::regulated(5.0, 2.0)],
            switching_freq: 100e3,
            mode: None,
            duty_cycle_max: 0.45,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 50.0,
            efficiency_target: 0.85,
            preferred_core: None,
        }
    }
}

impl FlybackRequirements {
    /// Calculate total output power
    pub fn total_output_power(&self) -> f64 {
        self.outputs.iter().map(|o| o.power_max()).sum()
    }

    /// Calculate estimated input power
    pub fn estimated_input_power(&self) -> f64 {
        self.total_output_power() / self.efficiency_target
    }

    /// Get primary regulated output (or first output if none marked)
    pub fn primary_output(&self) -> &FlybackOutput {
        self.outputs
            .iter()
            .find(|o| o.is_regulated)
            .unwrap_or(&self.outputs[0])
    }
}

// ============================================================================
// DESIGN OUTPUT
// ============================================================================

/// Selected component with specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedMOSFET {
    pub spec: MOSFETSpec,
    /// Peak drain-source voltage
    pub vds_peak: f64,
    /// RMS drain current
    pub id_rms: f64,
    /// Peak drain current
    pub id_peak: f64,
    /// Conduction loss (W)
    pub loss_conduction: f64,
    /// Switching loss (W)
    pub loss_switching: f64,
    /// Total loss (W)
    pub loss_total: f64,
}

/// Selected diode with operating parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedDiode {
    pub spec: DiodeSpec,
    /// Reverse voltage stress
    pub vr_peak: f64,
    /// Average forward current
    pub if_avg: f64,
    /// Peak forward current
    pub if_peak: f64,
    /// Conduction loss (W)
    pub loss_conduction: f64,
    /// Reverse recovery loss (W)
    pub loss_recovery: f64,
    /// Total loss (W)
    pub loss_total: f64,
}

/// Output capacitor specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputCapacitor {
    /// Capacitance (F)
    pub capacitance: f64,
    /// Voltage rating (V)
    pub voltage_rating: f64,
    /// Required ESR for ripple (Ω)
    pub max_esr: f64,
    /// RMS ripple current (A)
    pub ripple_current_rms: f64,
    /// Recommended type
    pub cap_type: CapacitorType,
    /// Number of capacitors in parallel
    pub parallel_count: u32,
}

/// Capacitor type recommendation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacitorType {
    /// Aluminum electrolytic - high capacitance, moderate ESR
    Electrolytic,
    /// Ceramic MLCC - low ESR, limited capacitance
    Ceramic,
    /// Solid polymer - low ESR, moderate capacitance
    Polymer,
    /// Hybrid (electrolytic + ceramic in parallel)
    Hybrid,
}

/// Snubber/clamp circuit design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClampCircuit {
    /// Clamp type
    pub clamp_type: ClampType,
    /// Clamp voltage (V) - for RCD
    pub clamp_voltage: f64,
    /// Clamp resistor (Ω) - for RCD
    pub resistor: f64,
    /// Clamp capacitor (F) - for RCD
    pub capacitor: f64,
    /// Clamp diode - for RCD/Zener
    pub diode: Option<DiodeSpec>,
    /// Estimated power dissipation (W)
    pub power_dissipation: f64,
}

/// Clamp circuit type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClampType {
    /// RCD clamp (resistor-capacitor-diode)
    RCD,
    /// Zener clamp
    Zener,
    /// Active clamp (recycles leakage energy)
    Active,
    /// None (if leakage is very low)
    None,
}

/// Complete flyback converter design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlybackDesign {
    /// Design requirements
    pub requirements: FlybackRequirements,
    /// Selected operating mode
    pub mode: FlybackMode,
    /// Operating duty cycle at Vin_min (max D)
    pub duty_cycle_max: f64,
    /// Operating duty cycle at Vin_max (min D)
    pub duty_cycle_min: f64,
    /// Transformer design
    pub transformer: TransformerDesign,
    /// Primary switch
    pub primary_switch: SelectedMOSFET,
    /// Output diodes (one per output)
    pub output_diodes: Vec<SelectedDiode>,
    /// Output capacitors (one set per output)
    pub output_capacitors: Vec<OutputCapacitor>,
    /// Input capacitor
    pub input_capacitor: OutputCapacitor,
    /// Clamp/snubber circuit
    pub clamp: ClampCircuit,
    /// Peak primary current (A)
    pub i_pri_peak: f64,
    /// RMS primary current (A)
    pub i_pri_rms: f64,
    /// Magnetizing inductance (H)
    pub l_mag: f64,
    /// Estimated leakage inductance (H)
    pub l_leak: f64,
    /// Total efficiency estimate
    pub efficiency: f64,
    /// Loss breakdown
    pub losses: FlybackLosses,
}

/// Detailed loss breakdown
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FlybackLosses {
    /// Primary MOSFET conduction
    pub mosfet_conduction: f64,
    /// Primary MOSFET switching
    pub mosfet_switching: f64,
    /// Primary MOSFET gate drive
    pub mosfet_gate: f64,
    /// Output diode conduction
    pub diode_conduction: f64,
    /// Output diode recovery
    pub diode_recovery: f64,
    /// Transformer core loss
    pub transformer_core: f64,
    /// Transformer copper loss
    pub transformer_copper: f64,
    /// Clamp/snubber dissipation
    pub clamp: f64,
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

/// Design result
#[derive(Clone, Debug)]
pub enum FlybackDesignResult {
    Success(FlybackDesign),
    NoSuitableCore(String),
    NoSuitableMOSFET(String),
    NoSuitableDiode(String),
    DesignConstraintViolation(String),
}

/// Design a flyback converter from requirements
pub fn design_flyback(req: &FlybackRequirements) -> Result<FlybackDesign, String> {
    // Validate inputs
    if req.outputs.is_empty() {
        return Err("At least one output required".to_string());
    }
    if req.vin.nom_v <= 0.0 {
        return Err("Invalid input voltage range".to_string());
    }

    let p_out = req.total_output_power();
    let p_in = req.estimated_input_power();

    // Select operating mode
    let mode = req
        .mode
        .unwrap_or_else(|| select_operating_mode(p_out, req.switching_freq));

    // Calculate duty cycles at min and max input voltage
    let primary_output = req.primary_output();
    let (d_max, d_min) = calculate_duty_cycles(
        req.vin.min_v,
        req.vin.max_v,
        primary_output.voltage,
        req.duty_cycle_max,
    );

    // Calculate primary currents
    let (i_pri_peak, i_pri_rms) =
        calculate_primary_currents(p_in, req.vin.min_v, d_max, mode, req.switching_freq);

    // Design transformer
    let material = select_core_material(req.switching_freq);
    let transformer_req = TransformerRequirements {
        primary_voltage: req.vin.min_v,
        secondary_voltages: req.outputs.iter().map(|o| o.voltage).collect(),
        secondary_currents: req.outputs.iter().map(|o| o.current_max).collect(),
        frequency: req.switching_freq,
        duty_cycle_max: d_max,
        isolation: req.isolation,
        ambient_temp: req.ambient_temp,
        max_temp_rise: req.max_temp_rise,
        topology: TransformerTopology::Flyback,
    };

    let transformer = auto_design_transformer(&transformer_req, &material, req.preferred_core)
        .ok_or_else(|| "No suitable core found for transformer".to_string())?;

    // Calculate magnetizing inductance
    let l_mag = transformer.magnetizing_inductance;
    let l_leak = transformer.leakage_inductance;

    // Calculate switch voltage stress
    let v_reflected = primary_output.voltage * transformer.turns_ratio;
    let v_clamp = v_reflected * 1.5; // Clamp at 1.5× reflected voltage
    let vds_peak = req.vin.max_v + v_clamp + 50.0; // Add margin for leakage spike

    // Select primary MOSFET
    let mosfet = select_primary_mosfet(vds_peak, i_pri_peak, i_pri_rms, req.switching_freq, d_max)?;

    // Select output diodes
    let output_diodes: Result<Vec<_>, String> = req
        .outputs
        .iter()
        .map(|out| {
            let vr = req.vin.max_v / transformer.turns_ratio + out.voltage;
            select_output_diode(vr, out.current_max, req.switching_freq, 1.0 - d_max)
        })
        .collect();
    let output_diodes = output_diodes?;

    // Size output capacitors
    let output_capacitors: Vec<_> = req
        .outputs
        .iter()
        .map(|out| size_output_capacitor(out, req.switching_freq, d_max, mode))
        .collect();

    // Size input capacitor
    let input_capacitor = size_input_capacitor(req.vin.min_v, p_in, req.switching_freq, d_max);

    // Design clamp circuit
    let clamp = design_clamp(
        l_leak,
        i_pri_peak,
        v_reflected,
        req.vin.max_v,
        req.switching_freq,
    );

    // Calculate losses
    let losses = calculate_losses(
        &mosfet,
        &output_diodes,
        &transformer,
        &clamp,
        p_in,
        req.switching_freq,
    );

    // Calculate efficiency
    let efficiency = p_out / (p_out + losses.total);

    Ok(FlybackDesign {
        requirements: req.clone(),
        mode,
        duty_cycle_max: d_max,
        duty_cycle_min: d_min,
        transformer,
        primary_switch: mosfet,
        output_diodes,
        output_capacitors,
        input_capacitor,
        clamp,
        i_pri_peak,
        i_pri_rms,
        l_mag,
        l_leak,
        efficiency,
        losses,
    })
}

/// Select operating mode based on power level and frequency
fn select_operating_mode(power: f64, frequency: f64) -> FlybackMode {
    // General guidelines:
    // - DCM preferred for low power (<30W) or high frequency
    // - CCM preferred for higher power
    // - CrM for highest efficiency in mid-power range
    if power < 20.0 {
        FlybackMode::DCM
    } else if power < 75.0 && frequency >= 100e3 {
        FlybackMode::CrM
    } else {
        FlybackMode::CCM
    }
}

/// Calculate duty cycles at minimum and maximum input voltage
fn calculate_duty_cycles(vin_min: f64, vin_max: f64, _vout: f64, max_duty: f64) -> (f64, f64) {
    // For flyback: D = (Vout × N) / (Vin + Vout × N)
    // where N = Np/Ns (turns ratio)
    // At design, we often work with Vout_reflected = Vout × N
    // Typical design: Vout_reflected ≈ Vin_min × D / (1-D)

    // Target reflected voltage for ~45% duty at Vin_min
    let v_reflected = vin_min * max_duty / (1.0 - max_duty);

    // Duty cycle formula: D = V_reflected / (Vin + V_reflected)
    let d_at_vin_min = v_reflected / (vin_min + v_reflected);
    let d_at_vin_max = v_reflected / (vin_max + v_reflected);

    // Clamp to reasonable range
    let d_max = d_at_vin_min.min(max_duty);
    let d_min = d_at_vin_max.max(0.1);

    (d_max, d_min)
}

/// Calculate primary currents
fn calculate_primary_currents(
    p_in: f64,
    vin_min: f64,
    d_max: f64,
    mode: FlybackMode,
    fsw: f64,
) -> (f64, f64) {
    let i_avg = p_in / vin_min;

    match mode {
        FlybackMode::DCM => {
            // In DCM, current is triangular from 0 to peak
            // I_avg = 0.5 × I_peak × D
            let i_peak = 2.0 * i_avg / d_max;
            let i_rms = i_peak * (d_max / 3.0).sqrt();
            (i_peak, i_rms)
        }
        FlybackMode::CCM => {
            // In CCM, current is trapezoidal
            // Assume 30% ripple
            let ripple_factor = 0.3;
            let i_dc = i_avg / d_max;
            let delta_i = i_dc * ripple_factor;
            let i_peak = i_dc + delta_i / 2.0;
            let i_rms = ((i_dc * i_dc + delta_i * delta_i / 12.0) * d_max).sqrt();
            (i_peak, i_rms)
        }
        FlybackMode::CrM => {
            // Critical mode: triangular, just reaching zero
            let i_peak = 2.0 * i_avg / d_max;
            let i_rms = i_peak * (d_max / 3.0).sqrt();
            (i_peak, i_rms)
        }
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

/// Select primary MOSFET
fn select_primary_mosfet(
    vds_peak: f64,
    i_peak: f64,
    i_rms: f64,
    fsw: f64,
    _duty: f64,
) -> Result<SelectedMOSFET, String> {
    // Require at least 20% voltage margin
    let vds_required = vds_peak * 1.2;

    let candidates = find_suitable_mosfets(
        vds_required,
        i_rms,
        i_peak * 1.5,
        MOSFETPreference::LowLosses,
    );

    if candidates.is_empty() {
        return Err(format!(
            "No MOSFET found for Vds>{:.0}V, Id>{:.1}A",
            vds_required, i_peak
        ));
    }

    // Select best candidate (first one from sorted list)
    let spec = candidates[0].clone();

    // Calculate losses
    let rds_hot = spec.rds_on_at_temp(100.0);
    let loss_conduction = i_rms * i_rms * rds_hot;

    // Switching loss estimate: Psw ≈ 0.5 × Vds × Id × (tr + tf) × fsw
    let t_sw = 20e-9; // Assume 20ns total switching time
    let loss_switching = 0.5 * vds_peak * i_peak * t_sw * fsw;

    // Gate drive loss
    let loss_gate = spec.qg_total * spec.vgs_th.max(10.0) * fsw;

    let loss_total = loss_conduction + loss_switching + loss_gate;

    Ok(SelectedMOSFET {
        spec,
        vds_peak,
        id_rms: i_rms,
        id_peak: i_peak,
        loss_conduction,
        loss_switching,
        loss_total,
    })
}

/// Select output diode
fn select_output_diode(
    vr_peak: f64,
    i_avg: f64,
    fsw: f64,
    off_duty: f64,
) -> Result<SelectedDiode, String> {
    // Require at least 30% voltage margin for diodes
    let vr_required = vr_peak * 1.3;

    // Prefer Schottky for low voltage, fast recovery for higher voltage
    let pref = if vr_required < 100.0 {
        DiodePreference::LowVf
    } else {
        DiodePreference::HighSpeed
    };

    // if_rms ≈ if_avg × sqrt(D_off) for discontinuous conduction
    let i_rms = i_avg * off_duty.sqrt();
    let candidates = find_suitable_diodes(vr_required, i_avg * 2.0, i_rms, pref);

    if candidates.is_empty() {
        return Err(format!(
            "No diode found for Vr>{:.0}V, If>{:.1}A",
            vr_required, i_avg
        ));
    }

    let spec = candidates[0].clone();

    // Calculate losses
    // During off-time, secondary current flows through diode
    let i_peak = i_avg * 2.0 / off_duty; // Approximate peak
    let loss_conduction = spec.vf_typical * i_avg;

    // Recovery loss (zero for Schottky)
    let loss_recovery = if spec.diode_type == DiodeType::Schottky {
        0.0
    } else {
        spec.qrr * vr_peak * fsw
    };

    let loss_total = loss_conduction + loss_recovery;

    Ok(SelectedDiode {
        spec,
        vr_peak,
        if_avg: i_avg,
        if_peak: i_peak,
        loss_conduction,
        loss_recovery,
        loss_total,
    })
}

/// Size output capacitor for ripple and ESR requirements
fn size_output_capacitor(
    output: &FlybackOutput,
    fsw: f64,
    duty: f64,
    mode: FlybackMode,
) -> OutputCapacitor {
    let off_time = (1.0 - duty) / fsw;

    // Ripple current (RMS) through output cap
    let i_ripple_rms = match mode {
        FlybackMode::DCM | FlybackMode::CrM => {
            // Triangular current during off-time
            let i_peak = output.current_max * 2.0 / (1.0 - duty);
            i_peak * ((1.0 - duty) / 3.0).sqrt()
        }
        FlybackMode::CCM => {
            // More continuous, lower ripple
            output.current_max * 0.5
        }
    };

    // ESR requirement from ripple voltage
    // V_ripple_esr = I_peak × ESR
    let i_peak = output.current_max * 2.0 / (1.0 - duty);
    let max_esr = output.ripple_pp / (2.0 * i_peak);

    // Capacitance requirement
    // V_ripple_cap = I_avg × t_off / C
    // Plus some margin for load transients
    let c_for_ripple = output.current_max * off_time / (output.ripple_pp / 2.0);
    let c_for_transient = output.current_max * 10e-6 / (output.voltage * 0.05); // 10µs, 5% droop
    let capacitance = c_for_ripple.max(c_for_transient);

    // Recommend capacitor type
    let cap_type = if output.voltage < 25.0 && capacitance < 500e-6 {
        CapacitorType::Ceramic
    } else if capacitance > 1000e-6 {
        CapacitorType::Electrolytic
    } else {
        CapacitorType::Hybrid
    };

    // Parallel count for ESR
    let parallel_count = if cap_type == CapacitorType::Electrolytic {
        ((max_esr / 0.05).recip().ceil() as u32).max(1) // Typical electrolytic ESR ~50mΩ
    } else {
        1
    };

    OutputCapacitor {
        capacitance,
        voltage_rating: (output.voltage * 1.5).ceil(),
        max_esr,
        ripple_current_rms: i_ripple_rms,
        cap_type,
        parallel_count,
    }
}

/// Size input capacitor
fn size_input_capacitor(vin_min: f64, p_in: f64, fsw: f64, duty: f64) -> OutputCapacitor {
    let i_avg = p_in / vin_min;
    let i_ripple_rms = i_avg * (duty * (1.0 - duty)).sqrt();

    // Allow 2% ripple on input
    let ripple_pp = vin_min * 0.02;

    // C = I × D × T / ΔV
    let capacitance = i_avg * duty / (fsw * ripple_pp);

    OutputCapacitor {
        capacitance,
        voltage_rating: (vin_min * 1.5).ceil(),
        max_esr: ripple_pp / i_ripple_rms,
        ripple_current_rms: i_ripple_rms,
        cap_type: CapacitorType::Electrolytic,
        parallel_count: 1,
    }
}

/// Design RCD clamp circuit
fn design_clamp(
    l_leak: f64,
    i_peak: f64,
    v_reflected: f64,
    _vin_max: f64,
    fsw: f64,
) -> ClampCircuit {
    // Energy in leakage inductance
    let e_leak = 0.5 * l_leak * i_peak * i_peak;
    let p_clamp = e_leak * fsw;

    // If clamp power is very low (<100mW), no clamp needed
    if p_clamp < 0.1 {
        return ClampCircuit {
            clamp_type: ClampType::None,
            clamp_voltage: 0.0,
            resistor: 0.0,
            capacitor: 0.0,
            diode: None,
            power_dissipation: 0.0,
        };
    }

    // RCD clamp design
    // Clamp voltage = V_reflected + margin (50V typical)
    let v_clamp = v_reflected + 50.0;

    // Clamp resistor: R = Vclamp² / P_clamp
    let r_clamp = v_clamp * v_clamp / p_clamp;

    // Clamp capacitor: τ = R × C should be >> switching period
    // Use 10× switching period
    let c_clamp = 10.0 / (fsw * r_clamp);

    ClampCircuit {
        clamp_type: ClampType::RCD,
        clamp_voltage: v_clamp,
        resistor: r_clamp,
        capacitor: c_clamp,
        diode: None, // Would add specific diode spec here
        power_dissipation: p_clamp,
    }
}

/// Calculate all losses
fn calculate_losses(
    mosfet: &SelectedMOSFET,
    diodes: &[SelectedDiode],
    transformer: &TransformerDesign,
    clamp: &ClampCircuit,
    _p_in: f64,
    _fsw: f64,
) -> FlybackLosses {
    let diode_conduction: f64 = diodes.iter().map(|d| d.loss_conduction).sum();
    let diode_recovery: f64 = diodes.iter().map(|d| d.loss_recovery).sum();

    let total = mosfet.loss_total
        + diode_conduction
        + diode_recovery
        + transformer.core_loss
        + transformer.primary_copper_loss
        + transformer.secondary_copper_loss
        + clamp.power_dissipation;

    FlybackLosses {
        mosfet_conduction: mosfet.loss_conduction,
        mosfet_switching: mosfet.loss_switching,
        mosfet_gate: mosfet.loss_total - mosfet.loss_conduction - mosfet.loss_switching,
        diode_conduction,
        diode_recovery,
        transformer_core: transformer.core_loss,
        transformer_copper: transformer.primary_copper_loss + transformer.secondary_copper_loss,
        clamp: clamp.power_dissipation,
        input_cap_esr: 0.0,  // Would calculate from actual capacitor
        output_cap_esr: 0.0, // Would calculate from actual capacitor
        total,
    }
}

impl FlybackDesign {
    /// Generate design summary
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str("═══════════════════════════════════════════════════════════════\n");
        s.push_str("                    FLYBACK CONVERTER DESIGN\n");
        s.push_str("═══════════════════════════════════════════════════════════════\n\n");

        s.push_str(&format!(
            "Mode: {:?} - {}\n\n",
            self.mode,
            self.mode.description()
        ));

        s.push_str("OPERATING CONDITIONS\n");
        s.push_str("────────────────────\n");
        s.push_str(&format!(
            "Input: {} to {}\n",
            format_voltage(self.requirements.vin.min_v),
            format_voltage(self.requirements.vin.max_v)
        ));
        s.push_str(&format!(
            "Duty cycle: {:.1}% (Vin_max) to {:.1}% (Vin_min)\n",
            self.duty_cycle_min * 100.0,
            self.duty_cycle_max * 100.0
        ));
        s.push_str(&format!(
            "Frequency: {:.0} kHz\n",
            self.requirements.switching_freq / 1000.0
        ));
        s.push_str(&format!(
            "Primary current: {} peak, {} RMS\n\n",
            format_current(self.i_pri_peak),
            format_current(self.i_pri_rms)
        ));

        s.push_str("TRANSFORMER\n");
        s.push_str("───────────\n");
        s.push_str(&format!(
            "Core: {} ({})\n",
            self.transformer.core.part_number, self.transformer.material.name
        ));
        s.push_str(&format!(
            "Primary: {} turns\n",
            self.transformer.primary.turns
        ));
        s.push_str(&format!(
            "Magnetizing inductance: {}\n",
            format_inductance(self.l_mag)
        ));
        s.push_str(&format!(
            "Leakage inductance: {} ({:.1}%)\n",
            format_inductance(self.l_leak),
            self.l_leak / self.l_mag * 100.0
        ));
        s.push_str(&format!("Flux density: {:.3} T\n", self.transformer.b_peak));
        s.push_str(&format!(
            "Fill factor: {:.1}%\n\n",
            self.transformer.fill_factor * 100.0
        ));

        s.push_str("PRIMARY SWITCH\n");
        s.push_str("──────────────\n");
        s.push_str(&format!(
            "{} - {}V, {}Ω, {}\n",
            self.primary_switch.spec.part_number,
            self.primary_switch.spec.vds_max,
            self.primary_switch.spec.rds_on_25c,
            format!("{:?}", self.primary_switch.spec.package)
        ));
        s.push_str(&format!(
            "Vds_peak: {} ({:.0}% of rating)\n",
            format_voltage(self.primary_switch.vds_peak),
            self.primary_switch.vds_peak / self.primary_switch.spec.vds_max * 100.0
        ));
        s.push_str(&format!(
            "Loss: {:.2}W (cond: {:.2}W, sw: {:.2}W)\n\n",
            self.primary_switch.loss_total,
            self.primary_switch.loss_conduction,
            self.primary_switch.loss_switching
        ));

        s.push_str("OUTPUT CAPACITORS\n");
        s.push_str("─────────────────\n");
        for (i, (out, cap)) in self
            .requirements
            .outputs
            .iter()
            .zip(self.output_capacitors.iter())
            .enumerate()
        {
            s.push_str(&format!(
                "Output {}: {} @ {} → {} ({:?})\n",
                i + 1,
                format_voltage(out.voltage),
                format_current(out.current_max),
                format_capacitance(cap.capacitance),
                cap.cap_type
            ));
        }
        s.push_str("\n");

        s.push_str("EFFICIENCY & LOSSES\n");
        s.push_str("───────────────────\n");
        s.push_str(&format!(
            "Estimated efficiency: {:.1}%\n",
            self.efficiency * 100.0
        ));
        s.push_str(&format!("Total losses: {:.2}W\n", self.losses.total));
        s.push_str(&format!(
            "  MOSFET: {:.2}W ({:.2}W cond, {:.2}W sw)\n",
            self.losses.mosfet_conduction + self.losses.mosfet_switching,
            self.losses.mosfet_conduction,
            self.losses.mosfet_switching
        ));
        s.push_str(&format!(
            "  Diodes: {:.2}W ({:.2}W cond, {:.2}W rec)\n",
            self.losses.diode_conduction + self.losses.diode_recovery,
            self.losses.diode_conduction,
            self.losses.diode_recovery
        ));
        s.push_str(&format!(
            "  Transformer: {:.2}W ({:.2}W core, {:.2}W Cu)\n",
            self.losses.transformer_core + self.losses.transformer_copper,
            self.losses.transformer_core,
            self.losses.transformer_copper
        ));
        s.push_str(&format!("  Clamp: {:.2}W\n", self.losses.clamp));

        s
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyback_design_48v_to_5v() {
        let req = FlybackRequirements {
            vin: VoltageRange::range(36.0, 72.0),
            outputs: vec![FlybackOutput::regulated(5.0, 2.0)],
            switching_freq: 150e3,
            isolation: IsolationClass::Basic,
            ..Default::default()
        };

        let design = design_flyback(&req);
        assert!(design.is_ok(), "Design should succeed");

        let d = design.unwrap();
        assert!(d.efficiency > 0.75, "Efficiency should be >75%");
        assert!(
            d.transformer.b_peak < 0.35,
            "Flux density should be reasonable"
        );
        assert!(d.i_pri_peak > 0.0, "Peak current should be positive");

        println!("{}", d.summary());
    }

    #[test]
    fn test_flyback_design_multi_output() {
        let req = FlybackRequirements {
            vin: VoltageRange::range(36.0, 72.0),
            outputs: vec![
                FlybackOutput::regulated(5.0, 2.0),
                FlybackOutput::new(12.0, 0.5),
            ],
            switching_freq: 100e3,
            isolation: IsolationClass::Basic,
            ..Default::default()
        };

        let design = design_flyback(&req);
        assert!(design.is_ok(), "Multi-output design should succeed");

        let d = design.unwrap();
        assert_eq!(d.output_diodes.len(), 2, "Should have 2 output diodes");
        assert_eq!(d.output_capacitors.len(), 2, "Should have 2 output caps");
    }

    #[test]
    fn test_mode_selection() {
        assert_eq!(select_operating_mode(10.0, 100e3), FlybackMode::DCM);
        assert_eq!(select_operating_mode(50.0, 150e3), FlybackMode::CrM);
        assert_eq!(select_operating_mode(100.0, 100e3), FlybackMode::CCM);
    }

    #[test]
    fn test_duty_cycle_calculation() {
        let (d_max, d_min) = calculate_duty_cycles(36.0, 72.0, 5.0, 0.45);

        assert!(d_max <= 0.45, "Max duty should not exceed limit");
        assert!(d_min < d_max, "Min duty should be less than max");
        assert!(d_min > 0.1, "Min duty should be reasonable");
    }

    #[test]
    fn test_clamp_design() {
        let clamp = design_clamp(5e-6, 2.0, 30.0, 72.0, 100e3);

        assert_eq!(clamp.clamp_type, ClampType::RCD);
        assert!(
            clamp.clamp_voltage > 30.0,
            "Clamp voltage should exceed reflected"
        );
        assert!(clamp.resistor > 0.0);
        assert!(clamp.capacitor > 0.0);
    }
}
