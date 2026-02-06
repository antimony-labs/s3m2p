//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: transformer.rs | DNA/src/power/magnetics/transformer.rs
//! PURPOSE: Transformer design algorithms for SMPS applications
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides transformer design automation for switched-mode power supplies:
//!
//! - **Core Selection**: Based on power throughput and frequency
//! - **Turns Calculation**: From volt-seconds and flux density limits
//! - **Wire Selection**: Based on current density and skin effect
//! - **Loss Calculation**: Core loss + copper loss (AC and DC)
//! - **Thermal Estimation**: Temperature rise from losses

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use super::core_materials::{CoreGeometry, CoreMaterial, CoreType};
use super::wire::{
    awg_spec, copper_skin_depth, total_ac_resistance_factor, CurrentDensity, InsulationClass,
    LitzWireSpec, WireSpec,
};

/// Transformer winding type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindingType {
    /// Primary winding
    Primary,
    /// Secondary winding (main output)
    Secondary,
    /// Auxiliary winding (bias, feedback)
    Auxiliary,
    /// Reset/clamp winding
    Reset,
}

/// Winding configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindingSpec {
    /// Winding type
    pub winding_type: WindingType,
    /// Number of turns
    pub turns: u32,
    /// Voltage across winding (peak or average depending on topology)
    pub voltage: f64,
    /// RMS current through winding (A)
    pub current_rms: f64,
    /// Peak current through winding (A)
    pub current_peak: f64,
    /// Wire specification
    pub wire: WireSpec,
    /// Number of parallel strands
    pub parallel_strands: u32,
    /// Number of winding layers
    pub layers: u32,
    /// Insulation class
    pub insulation: InsulationClass,
}

impl WindingSpec {
    /// Calculate DC resistance of winding
    pub fn dc_resistance(&self, mlt_mm: f64, temp_c: f64) -> f64 {
        let length_m = (self.turns as f64 * mlt_mm) / 1000.0;
        let r_dc = self.wire.resistance_at_temp(temp_c) * length_m;
        r_dc / self.parallel_strands as f64
    }

    /// Calculate AC resistance of winding
    pub fn ac_resistance(&self, mlt_mm: f64, frequency: f64, temp_c: f64) -> f64 {
        let length_m = (self.turns as f64 * mlt_mm) / 1000.0;
        let r_dc = self.wire.resistance_at_temp(temp_c) * length_m;

        // Account for parallel strands
        let r_dc_parallel = r_dc / self.parallel_strands as f64;

        // AC resistance factor
        let fr = total_ac_resistance_factor(
            self.wire.diameter_mm,
            frequency,
            self.layers,
            0.7, // Typical fill factor
        );

        r_dc_parallel * fr
    }

    /// Calculate copper loss (I²R)
    pub fn copper_loss(&self, mlt_mm: f64, frequency: f64, temp_c: f64) -> f64 {
        let r_ac = self.ac_resistance(mlt_mm, frequency, temp_c);
        self.current_rms * self.current_rms * r_ac
    }

    /// Calculate wire window area required (mm²)
    pub fn window_area_required(&self) -> f64 {
        let wire_od = self.wire.outer_diameter(self.insulation);
        let wire_area = PI * (wire_od / 2.0).powi(2);
        wire_area * self.turns as f64 * self.parallel_strands as f64
    }

    /// Calculate fill factor for given window area
    pub fn fill_factor(&self, window_area_mm2: f64) -> f64 {
        self.window_area_required() / window_area_mm2
    }
}

/// Transformer isolation requirements
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationClass {
    /// No isolation required
    None,
    /// Basic isolation (single fault protection)
    Basic,
    /// Supplementary isolation
    Supplementary,
    /// Reinforced isolation (double insulation, safety critical)
    Reinforced,
}

impl IsolationClass {
    /// Minimum creepage/clearance (mm) for 240VAC input
    pub fn min_clearance_mm(&self) -> f64 {
        match self {
            IsolationClass::None => 0.0,
            IsolationClass::Basic => 3.0,
            IsolationClass::Supplementary => 3.0,
            IsolationClass::Reinforced => 6.0,
        }
    }

    /// Number of insulation layers required
    pub fn insulation_layers(&self) -> u32 {
        match self {
            IsolationClass::None => 0,
            IsolationClass::Basic => 1,
            IsolationClass::Supplementary => 1,
            IsolationClass::Reinforced => 3,
        }
    }
}

/// Complete transformer design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransformerDesign {
    /// Core geometry
    pub core: CoreGeometry,
    /// Core material
    pub material: CoreMaterial,
    /// Primary winding
    pub primary: WindingSpec,
    /// Secondary windings
    pub secondaries: Vec<WindingSpec>,
    /// Auxiliary windings (bias, etc.)
    pub auxiliaries: Vec<WindingSpec>,
    /// Magnetizing inductance (H)
    pub magnetizing_inductance: f64,
    /// Estimated leakage inductance (H)
    pub leakage_inductance: f64,
    /// Turns ratio (Np:Ns for first secondary)
    pub turns_ratio: f64,
    /// Peak flux density (T)
    pub b_peak: f64,
    /// Core loss (W)
    pub core_loss: f64,
    /// Primary copper loss (W)
    pub primary_copper_loss: f64,
    /// Secondary copper loss (W)
    pub secondary_copper_loss: f64,
    /// Total losses (W)
    pub total_loss: f64,
    /// Estimated temperature rise (°C)
    pub temp_rise: f64,
    /// Total window fill factor
    pub fill_factor: f64,
}

/// Transformer requirements for design
#[derive(Clone, Debug)]
pub struct TransformerRequirements {
    /// Primary voltage (V) - typically Vin for flyback
    pub primary_voltage: f64,
    /// Secondary voltages (V)
    pub secondary_voltages: Vec<f64>,
    /// Secondary currents (A rms)
    pub secondary_currents: Vec<f64>,
    /// Switching frequency (Hz)
    pub frequency: f64,
    /// Maximum duty cycle
    pub duty_cycle_max: f64,
    /// Isolation requirement
    pub isolation: IsolationClass,
    /// Operating temperature (°C)
    pub ambient_temp: f64,
    /// Maximum temperature rise (°C)
    pub max_temp_rise: f64,
    /// Topology type (affects design equations)
    pub topology: TransformerTopology,
}

/// Transformer topology type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransformerTopology {
    /// Flyback (energy storing transformer)
    Flyback,
    /// Forward (true transformer)
    Forward,
    /// Push-pull
    PushPull,
    /// Half-bridge
    HalfBridge,
    /// Full-bridge
    FullBridge,
}

impl TransformerTopology {
    /// Get effective volt-second multiplier
    pub fn volt_second_factor(&self) -> f64 {
        match self {
            TransformerTopology::Flyback => 1.0,
            TransformerTopology::Forward => 1.0,
            TransformerTopology::PushPull => 0.5,
            TransformerTopology::HalfBridge => 0.5,
            TransformerTopology::FullBridge => 0.5,
        }
    }
}

/// Transformer design result
#[derive(Clone, Debug)]
pub enum TransformerDesignResult {
    /// Successful design
    Success(TransformerDesign),
    /// Core too small (area product insufficient)
    CoreTooSmall { required_ap: f64, available_ap: f64 },
    /// Window too small (can't fit windings)
    WindowTooSmall {
        required_area: f64,
        available_area: f64,
    },
    /// Flux density too high
    FluxDensityTooHigh { b_peak: f64, b_max: f64 },
    /// Temperature rise too high
    TemperatureExceeded { rise: f64, max_rise: f64 },
}

// ============================================================================
// TRANSFORMER DESIGN ALGORITHMS
// ============================================================================

/// Design a transformer for given requirements and core
pub fn design_transformer(
    req: &TransformerRequirements,
    core: &CoreGeometry,
    material: &CoreMaterial,
) -> TransformerDesignResult {
    let operating_temp = req.ambient_temp + req.max_temp_rise / 2.0;

    // Calculate power throughput
    let p_out: f64 = req
        .secondary_voltages
        .iter()
        .zip(req.secondary_currents.iter())
        .map(|(v, i)| v * i)
        .sum();

    // For flyback, power throughput is different
    let p_throughput = match req.topology {
        TransformerTopology::Flyback => p_out * 2.0, // Energy stored and released
        _ => p_out,
    };

    // Check area product
    let required_ap = estimate_required_area_product(p_throughput, req.frequency);
    let available_ap = core.area_product();

    if available_ap < required_ap * 0.7 {
        return TransformerDesignResult::CoreTooSmall {
            required_ap,
            available_ap,
        };
    }

    // Calculate maximum flux density with margin
    let b_max = material.max_flux_with_margin(operating_temp, 0.3); // 30% margin

    // Calculate primary turns from volt-seconds
    // V × t = N × Ae × ΔB
    let volt_seconds = req.primary_voltage * req.duty_cycle_max / req.frequency;
    let volt_second_factor = req.topology.volt_second_factor();

    // For flyback: unipolar flux swing (0 to Bpeak), use full b_max
    // For forward/bridge: bipolar swing (-Bpeak to +Bpeak), use b_max * 2
    let flux_swing_factor = match req.topology {
        TransformerTopology::Flyback => 1.0,
        _ => 2.0,
    };

    let n_calculated = ((volt_seconds * volt_second_factor)
        / (core.ae * 1e-6 * b_max * flux_swing_factor))
        .ceil() as u32;

    // Minimum turns for practical transformer (coupling, leakage control)
    // Also helps reduce flux density and losses for oversized cores
    let min_turns = 6u32;
    let n_primary = n_calculated.max(min_turns);

    // Calculate actual B peak (will be lower than b_max if we used more turns)
    let b_peak =
        volt_seconds * volt_second_factor / (n_primary as f64 * core.ae * 1e-6 * flux_swing_factor);

    if b_peak > b_max * 1.1 {
        return TransformerDesignResult::FluxDensityTooHigh { b_peak, b_max };
    }

    // Calculate secondary turns from turns ratio
    let mut secondary_specs = Vec::new();
    for (i, (vout, iout)) in req
        .secondary_voltages
        .iter()
        .zip(req.secondary_currents.iter())
        .enumerate()
    {
        // For flyback: Ns/Np = Vout × (1-D) / (Vin × D)
        // For forward: Ns/Np = Vout / (Vin × D)
        let n_ratio = match req.topology {
            TransformerTopology::Flyback => {
                (*vout + 0.5) * (1.0 - req.duty_cycle_max)
                    / (req.primary_voltage * req.duty_cycle_max)
            }
            _ => (*vout + 0.5) / (req.primary_voltage * req.duty_cycle_max),
        };

        let n_secondary = ((n_primary as f64 * n_ratio).ceil() as u32).max(1);

        // Calculate secondary current (RMS)
        let i_rms = *iout * (1.0 - req.duty_cycle_max).sqrt(); // Approximate for flyback
        let i_peak = *iout * 2.0; // Approximate

        // Select wire for secondary
        let wire = select_wire_for_current(i_rms, req.frequency, CurrentDensity::Ventilated);

        secondary_specs.push(WindingSpec {
            winding_type: if i == 0 {
                WindingType::Secondary
            } else {
                WindingType::Auxiliary
            },
            turns: n_secondary,
            voltage: *vout,
            current_rms: i_rms,
            current_peak: i_peak,
            wire,
            parallel_strands: 1,
            layers: 1,
            insulation: InsulationClass::Heavy,
        });
    }

    // Calculate primary current
    let p_in = p_out / 0.9; // Assume 90% efficiency
    let i_pri_avg = p_in / req.primary_voltage;
    let i_pri_rms = i_pri_avg * (req.duty_cycle_max).sqrt();
    let i_pri_peak = i_pri_avg * 2.0 / req.duty_cycle_max;

    // Select wire for primary
    let pri_wire = select_wire_for_current(i_pri_rms, req.frequency, CurrentDensity::Ventilated);

    let primary_spec = WindingSpec {
        winding_type: WindingType::Primary,
        turns: n_primary,
        voltage: req.primary_voltage,
        current_rms: i_pri_rms,
        current_peak: i_pri_peak,
        wire: pri_wire,
        parallel_strands: 1,
        layers: 1,
        insulation: InsulationClass::Heavy,
    };

    // Check window fill
    let mut total_window_area = primary_spec.window_area_required();
    for sec in &secondary_specs {
        total_window_area += sec.window_area_required();
    }

    // Add isolation margin between primary and secondary windings
    // Isolation area ≈ clearance_height × winding_width
    // Estimate winding width as sqrt(bobbin_window)
    let winding_width = core.bobbin_window.sqrt();
    let isolation_area =
        req.isolation.min_clearance_mm() * winding_width * req.isolation.insulation_layers() as f64;
    total_window_area += isolation_area;

    let fill_factor = total_window_area / core.bobbin_window;

    if fill_factor > 0.45 {
        return TransformerDesignResult::WindowTooSmall {
            required_area: total_window_area,
            available_area: core.bobbin_window,
        };
    }

    // Calculate losses
    let pv = material.core_loss_density_igse(req.frequency, b_peak, req.duty_cycle_max);
    let core_loss = pv * core.volume_cm3();

    let primary_copper_loss = primary_spec.copper_loss(core.mlt, req.frequency, operating_temp);

    let secondary_copper_loss: f64 = secondary_specs
        .iter()
        .map(|s| s.copper_loss(core.mlt, req.frequency, operating_temp))
        .sum();

    let total_loss = core_loss + primary_copper_loss + secondary_copper_loss;

    // Estimate temperature rise
    // ΔT ≈ P_loss / (h × A) where h ≈ 10 W/(m²·K) for natural convection
    let h = 10.0; // W/(m²·K)
    let temp_rise = total_loss / (h * core.surface_area * 1e-4); // surface_area in cm²

    if temp_rise > req.max_temp_rise {
        return TransformerDesignResult::TemperatureExceeded {
            rise: temp_rise,
            max_rise: req.max_temp_rise,
        };
    }

    // Calculate magnetizing inductance
    // Lm = AL × Np² (AL in nH/turn²)
    let magnetizing_inductance = core.al * 1e-9 * (n_primary as f64).powi(2);

    // Estimate leakage inductance (typically 1-5% of Lm)
    let leakage_inductance = magnetizing_inductance * 0.02;

    // Calculate turns ratio
    let turns_ratio = if !secondary_specs.is_empty() {
        n_primary as f64 / secondary_specs[0].turns as f64
    } else {
        1.0
    };

    TransformerDesignResult::Success(TransformerDesign {
        core: core.clone(),
        material: material.clone(),
        primary: primary_spec,
        secondaries: secondary_specs
            .iter()
            .filter(|s| s.winding_type == WindingType::Secondary)
            .cloned()
            .collect(),
        auxiliaries: secondary_specs
            .iter()
            .filter(|s| s.winding_type == WindingType::Auxiliary)
            .cloned()
            .collect(),
        magnetizing_inductance,
        leakage_inductance,
        turns_ratio,
        b_peak,
        core_loss,
        primary_copper_loss,
        secondary_copper_loss,
        total_loss,
        temp_rise,
        fill_factor,
    })
}

/// Estimate required area product (Ae × Aw) for given power
/// Based on empirical formula: AP = (Pt / (Kf × Ku × Bmax × J × f))^(1/k)
fn estimate_required_area_product(power_throughput: f64, frequency: f64) -> f64 {
    // Empirical formula based on typical transformer designs:
    // AP (mm⁴) ≈ 50 × Pt^1.14 × (100kHz/f)^0.5
    // At 100kHz, 50W → ~4200 mm⁴, which corresponds to ETD29-ETD34 range
    let f_factor = (100e3 / frequency.max(10e3)).sqrt();
    50.0 * power_throughput.powf(1.14) * f_factor
}

/// Select wire gauge for given current and frequency
fn select_wire_for_current(current_rms: f64, frequency: f64, density: CurrentDensity) -> WireSpec {
    let required_area = current_rms / density.value();
    let skin_depth = copper_skin_depth(frequency);

    // Find suitable AWG - iterate from thick (low AWG) to thin (high AWG)
    // AWG 10 is thicker than AWG 40
    for awg in 10..=40 {
        let spec = awg_spec(awg);

        // Check if wire is thick enough for current
        if spec.area_mm2 < required_area {
            // Wire too thin, use previous (thicker) gauge
            if awg > 10 {
                return awg_spec(awg - 1);
            }
            return spec; // AWG 10 is the thickest we have
        }

        // Check skin effect - wire diameter should be < 4× skin depth
        let radius_m = spec.diameter_mm / 2000.0;
        if radius_m > skin_depth * 4.0 {
            // Wire too thick for frequency at low frequency
            // but at high frequency this is less of a concern for smaller wires
            continue;
        }

        // If this wire is thick enough and not too thick, return it
        if spec.area_mm2 >= required_area {
            return spec;
        }
    }

    // Default to AWG 22 if nothing suitable found
    awg_spec(22)
}

/// Select Litz wire for high-frequency applications
pub fn select_litz_wire(
    current_rms: f64,
    frequency: f64,
    density: CurrentDensity,
) -> Option<LitzWireSpec> {
    super::wire::find_litz_wire(current_rms, frequency, density)
}

// ============================================================================
// DESIGN ITERATION
// ============================================================================

/// Find optimal core for transformer requirements
pub fn find_optimal_core(
    req: &TransformerRequirements,
    available_cores: &[CoreGeometry],
    material: &CoreMaterial,
) -> Option<TransformerDesign> {
    let mut best_design: Option<TransformerDesign> = None;
    let mut best_score = f64::MAX;

    for core in available_cores {
        if let TransformerDesignResult::Success(design) = design_transformer(req, core, material) {
            // Score based on:
            // - Lower total loss is better
            // - Lower fill factor (more margin) is better
            // - Smaller core (lower weight/cost) is better
            let loss_score = design.total_loss * 10.0;
            let fill_score = design.fill_factor * 5.0;
            let size_score = core.ve / 1000.0;

            let score = loss_score + fill_score + size_score;

            if score < best_score {
                best_score = score;
                best_design = Some(design);
            }
        }
    }

    best_design
}

/// Design transformer with automatic core selection
pub fn auto_design_transformer(
    req: &TransformerRequirements,
    material: &CoreMaterial,
    core_type: Option<CoreType>,
) -> Option<TransformerDesign> {
    let all_cores = super::core_materials::core_geometry_database();

    let filtered_cores: Vec<CoreGeometry> = if let Some(ct) = core_type {
        all_cores
            .into_iter()
            .filter(|c| c.core_type == ct)
            .collect()
    } else {
        all_cores
    };

    find_optimal_core(req, &filtered_cores, material)
}

// ============================================================================
// FLYBACK-SPECIFIC DESIGN
// ============================================================================

/// Flyback transformer design parameters
#[derive(Clone, Debug)]
pub struct FlybackTransformerSpec {
    /// Input voltage range (min, max)
    pub vin_range: (f64, f64),
    /// Output voltage
    pub vout: f64,
    /// Output current
    pub iout: f64,
    /// Switching frequency
    pub frequency: f64,
    /// Maximum duty cycle
    pub d_max: f64,
    /// Continuous conduction mode?
    pub ccm: bool,
    /// Reflected voltage on primary (determines clamp level)
    pub reflected_voltage: f64,
    /// Efficiency target
    pub efficiency: f64,
}

/// Calculate flyback magnetizing inductance
pub fn flyback_magnetizing_inductance(spec: &FlybackTransformerSpec) -> f64 {
    let pout = spec.vout * spec.iout;
    let pin = pout / spec.efficiency;

    if spec.ccm {
        // CCM: Lm = Vin_min × D_max / (2 × ΔI × f)
        // Where ΔI is ripple current (typically 20-40% of average)
        let i_avg = pin / (spec.vin_range.0 * spec.d_max);
        let delta_i = i_avg * 0.3; // 30% ripple
        spec.vin_range.0 * spec.d_max / (2.0 * delta_i * spec.frequency)
    } else {
        // DCM: Lm = (Vin_min × D_max)² / (2 × Pin × f)
        (spec.vin_range.0 * spec.d_max).powi(2) / (2.0 * pin * spec.frequency)
    }
}

/// Calculate flyback turns ratio
pub fn flyback_turns_ratio(spec: &FlybackTransformerSpec) -> f64 {
    // Ns/Np = (Vout + Vf) × (1 - D) / (Vin_min × D)
    // Or equivalently: Np/Ns = Vin_min × D / ((Vout + 0.5) × (1 - D))
    let vf = 0.5; // Diode forward drop
    (spec.vin_range.0 * spec.d_max) / ((spec.vout + vf) * (1.0 - spec.d_max))
}

/// Calculate peak primary current
pub fn flyback_peak_primary_current(spec: &FlybackTransformerSpec) -> f64 {
    let pout = spec.vout * spec.iout;
    let pin = pout / spec.efficiency;

    if spec.ccm {
        // CCM: Ip_pk = Iavg + ΔI/2
        let i_avg = pin / (spec.vin_range.0 * spec.d_max);
        let delta_i = i_avg * 0.3;
        i_avg + delta_i / 2.0
    } else {
        // DCM: Ip_pk = 2 × Pin / (Vin_min × D_max)
        2.0 * pin / (spec.vin_range.0 * spec.d_max)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power::magnetics::core_materials::{core_geometry_database, ferrite_database};

    #[test]
    fn test_area_product_estimate() {
        // 50W at 100kHz: AP = 50 × 50^1.14 ≈ 4200 mm⁴
        let ap = estimate_required_area_product(50.0, 100e3);
        assert!(
            ap > 2000.0 && ap < 10000.0,
            "Expected AP 2000-10000 mm⁴, got {}",
            ap
        );
    }

    #[test]
    fn test_flyback_inductance() {
        let spec = FlybackTransformerSpec {
            vin_range: (90.0, 265.0), // Rectified AC
            vout: 5.0,
            iout: 2.0,
            frequency: 100e3,
            d_max: 0.45,
            ccm: true,
            reflected_voltage: 100.0,
            efficiency: 0.85,
        };

        let lm = flyback_magnetizing_inductance(&spec);
        // For 10W output with CCM, Lm is typically in mH range
        assert!(
            lm > 100e-6 && lm < 10e-3,
            "Expected Lm 100µH-10mH, got {} H",
            lm
        );
    }

    #[test]
    fn test_flyback_turns_ratio() {
        let spec = FlybackTransformerSpec {
            vin_range: (100.0, 265.0),
            vout: 12.0,
            iout: 1.0,
            frequency: 100e3,
            d_max: 0.45,
            ccm: true,
            reflected_voltage: 100.0,
            efficiency: 0.85,
        };

        let np_ns = flyback_turns_ratio(&spec);
        // Should be in reasonable range (3-10 for typical flyback)
        assert!(np_ns > 1.0 && np_ns < 20.0);
    }

    #[test]
    fn test_winding_resistance() {
        let wire = awg_spec(22);
        let winding = WindingSpec {
            winding_type: WindingType::Primary,
            turns: 50,
            voltage: 100.0,
            current_rms: 0.5,
            current_peak: 1.0,
            wire,
            parallel_strands: 1,
            layers: 2,
            insulation: InsulationClass::Heavy,
        };

        // With 50mm MLT, DC resistance should be reasonable
        let r_dc = winding.dc_resistance(50.0, 25.0);
        assert!(r_dc > 0.1 && r_dc < 5.0); // Ohms

        // AC resistance should be higher
        let r_ac = winding.ac_resistance(50.0, 100e3, 75.0);
        assert!(r_ac >= r_dc);
    }

    #[test]
    fn test_transformer_design() {
        let material = ferrite_database()
            .into_iter()
            .find(|m| m.name == "N87")
            .unwrap();

        // Use ETD49 which has larger window and MORE surface area (72 cm²)
        let core = core_geometry_database()
            .into_iter()
            .find(|c| c.part_number == "ETD49")
            .unwrap();

        println!(
            "Core: {} - Ae={} mm², Ve={} mm³, Surface={} cm²",
            core.part_number, core.ae, core.ve, core.surface_area
        );
        println!(
            "Material: {} - k={}, α={}, β={}",
            material.name, material.steinmetz_k, material.steinmetz_alpha, material.steinmetz_beta
        );

        // Test simple sinusoidal core loss first
        let pv_100k_0p1t = material.core_loss_density(100e3, 0.1);
        println!(
            "Core loss density at 100kHz, 0.1T: {:.4} W/cm³",
            pv_100k_0p1t
        );

        // Use 48V DC input (POE/telecom application) which has lower volt-seconds
        let req = TransformerRequirements {
            primary_voltage: 48.0, // 48V DC (POE/telecom)
            secondary_voltages: vec![5.0],
            secondary_currents: vec![1.0], // 5W output (reduced for test)
            frequency: 200e3,              // Higher frequency for smaller core
            duty_cycle_max: 0.45,
            isolation: IsolationClass::None, // No isolation overhead for basic test
            ambient_temp: 25.0,
            max_temp_rise: 60.0, // Realistic thermal limit
            topology: TransformerTopology::Flyback,
        };

        match design_transformer(&req, &core, &material) {
            TransformerDesignResult::Success(design) => {
                println!(
                    "Design: Np={}, B={:.3}T, Fill={:.1}%, CoreLoss={:.3}W, CuLoss={:.3}W, TotalLoss={:.3}W, TempRise={:.1}°C",
                    design.primary.turns,
                    design.b_peak,
                    design.fill_factor * 100.0,
                    design.core_loss,
                    design.primary_copper_loss + design.secondary_copper_loss,
                    design.total_loss,
                    design.temp_rise
                );
                assert!(
                    design.primary.turns > 1,
                    "Expected >1 turns, got {}",
                    design.primary.turns
                );
                assert!(
                    design.b_peak < 0.35,
                    "Expected B_peak < 0.35T, got {}",
                    design.b_peak
                );
                assert!(design.total_loss > 0.0);
            }
            other => panic!("Expected success, got {:?}", other),
        }
    }

    #[test]
    fn test_auto_design() {
        let material = ferrite_database()
            .into_iter()
            .find(|m| m.name == "N87")
            .unwrap();

        // Use 48V DC input for auto-select test
        let req = TransformerRequirements {
            primary_voltage: 48.0, // 48V DC input
            secondary_voltages: vec![12.0],
            secondary_currents: vec![1.0], // 12W
            frequency: 150e3,
            duty_cycle_max: 0.45,
            isolation: IsolationClass::Basic,
            ambient_temp: 25.0,
            max_temp_rise: 60.0, // Realistic thermal limit
            topology: TransformerTopology::Flyback,
        };

        let design = auto_design_transformer(&req, &material, Some(CoreType::ETD));
        assert!(
            design.is_some(),
            "Expected auto_design to find a valid core"
        );

        let d = design.unwrap();
        println!(
            "Auto-selected core: {}, Np={}, Fill={:.1}%, Loss={:.2}W, TempRise={:.1}°C",
            d.core.part_number,
            d.primary.turns,
            d.fill_factor * 100.0,
            d.total_loss,
            d.temp_rise
        );
    }
}
