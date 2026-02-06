//! ═══════════════════════════════════════════════════════════════════════════════
//! Compensator design for SMPS control loops
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides compensator synthesis for stable, fast regulation:
//!
//! - **Type I**: Pure integrator (for current-mode control)
//! - **Type II**: One zero, one high-frequency pole (most common)
//! - **Type III**: Two zeros, two high-frequency poles (voltage-mode buck)
//!
//! Each compensator can be implemented with:
//! - Op-amp circuit with R/C components
//! - Transconductance amplifier (OTA)
//! - Digital (difference equations)

use super::small_signal::{Complex, TransferFunction};
use std::f64::consts::PI;

// ============================================================================
// COMPENSATOR TYPES
// ============================================================================

/// Compensator topology
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompensatorType {
    /// Type I: Integrator only
    /// Gc(s) = ωi / s
    /// Use: Current-mode control, simple applications
    TypeI,

    /// Type II: Integrator + one zero + one high-frequency pole
    /// Gc(s) = (ωi/s) × (1 + s/ωz) / (1 + s/ωp)
    /// Use: Current-mode control, single-pole plants
    TypeII,

    /// Type III: Integrator + two zeros + two high-frequency poles
    /// Gc(s) = (ωi/s) × (1 + s/ωz1)(1 + s/ωz2) / ((1 + s/ωp1)(1 + s/ωp2))
    /// Use: Voltage-mode control with LC double-pole
    TypeIII,
}

// ============================================================================
// COMPENSATOR DESIGN RESULT
// ============================================================================

/// Completed compensator design
#[derive(Clone, Debug)]
pub struct CompensatorDesign {
    /// Compensator type
    pub comp_type: CompensatorType,
    /// Transfer function
    pub transfer_function: TransferFunction,
    /// Component values for op-amp implementation
    pub components: CompensatorComponents,
    /// Achieved crossover frequency (Hz)
    pub crossover_freq: f64,
    /// Achieved phase margin (degrees)
    pub phase_margin_deg: f64,
    /// Achieved gain margin (dB)
    pub gain_margin_db: f64,
    /// Design notes
    pub notes: Vec<String>,
}

/// Component values for op-amp compensator implementation
#[derive(Clone, Debug)]
pub struct CompensatorComponents {
    /// Type I/II/III: Integrator resistor R1 (from Vout to inverting input)
    pub r1: f64,
    /// Type I: Integrator capacitor C1 (feedback)
    /// Type II/III: Feedback capacitor C1
    pub c1: f64,
    /// Type II/III: Feedback resistor R2 (sets zero)
    pub r2: Option<f64>,
    /// Type II/III: High-frequency pole capacitor C2
    pub c2: Option<f64>,
    /// Type III: Second zero resistor R3
    pub r3: Option<f64>,
    /// Type III: Second high-frequency pole capacitor C3
    pub c3: Option<f64>,
}

impl CompensatorComponents {
    /// Create Type I components (integrator)
    pub fn type_i(r1: f64, c1: f64) -> Self {
        Self {
            r1,
            c1,
            r2: None,
            c2: None,
            r3: None,
            c3: None,
        }
    }

    /// Create Type II components
    pub fn type_ii(r1: f64, c1: f64, r2: f64, c2: f64) -> Self {
        Self {
            r1,
            c1,
            r2: Some(r2),
            c2: Some(c2),
            r3: None,
            c3: None,
        }
    }

    /// Create Type III components
    pub fn type_iii(r1: f64, c1: f64, r2: f64, c2: f64, r3: f64, c3: f64) -> Self {
        Self {
            r1,
            c1,
            r2: Some(r2),
            c2: Some(c2),
            r3: Some(r3),
            c3: Some(c3),
        }
    }

    /// Get the integrator frequency fi = 1/(2π×R1×C1)
    pub fn integrator_freq(&self) -> f64 {
        1.0 / (2.0 * PI * self.r1 * self.c1)
    }

    /// Get the zero frequency fz = 1/(2π×R2×C1) for Type II/III
    pub fn zero_freq(&self) -> Option<f64> {
        self.r2.map(|r2| 1.0 / (2.0 * PI * r2 * self.c1))
    }

    /// Get the pole frequency fp = 1/(2π×R2×C2) for Type II/III
    pub fn pole_freq(&self) -> Option<f64> {
        match (self.r2, self.c2) {
            (Some(r2), Some(c2)) => Some(1.0 / (2.0 * PI * r2 * c2)),
            _ => None,
        }
    }
}

// ============================================================================
// COMPENSATOR DESIGN REQUIREMENTS
// ============================================================================

/// Requirements for compensator design
#[derive(Clone, Debug)]
pub struct CompensatorRequirements {
    /// Target crossover frequency (Hz)
    pub crossover_freq: f64,
    /// Target phase margin (degrees) - typically 45-60°
    pub phase_margin_deg: f64,
    /// Minimum gain margin (dB) - typically > 10 dB
    pub min_gain_margin_db: f64,
    /// Reference voltage (for gain calculation)
    pub vref: f64,
    /// Output voltage divider ratio (Vfb/Vout)
    pub divider_ratio: f64,
    /// PWM modulator gain (1/Vramp)
    pub modulator_gain: f64,
}

impl Default for CompensatorRequirements {
    fn default() -> Self {
        Self {
            crossover_freq: 10e3,     // 10 kHz
            phase_margin_deg: 55.0,   // 55° (good balance)
            min_gain_margin_db: 10.0, // 10 dB
            vref: 0.8,                // 0.8V reference (common)
            divider_ratio: 0.2,       // 5:1 divider for 5V output
            modulator_gain: 1.0,      // Unity modulator gain
        }
    }
}

// ============================================================================
// COMPENSATOR DESIGN FUNCTIONS
// ============================================================================

/// Design a compensator for the given plant and requirements
pub fn design_compensator(
    plant: &TransferFunction,
    requirements: &CompensatorRequirements,
    comp_type: CompensatorType,
) -> Result<CompensatorDesign, String> {
    let fc = requirements.crossover_freq;
    let target_pm = requirements.phase_margin_deg;

    // Get plant characteristics at crossover
    let plant_at_fc = plant.evaluate(fc);
    let plant_gain_at_fc = plant_at_fc.magnitude();
    let plant_phase_at_fc = plant_at_fc.phase_deg();

    // Required compensator gain at crossover (to make loop gain = 1)
    let total_gain = requirements.modulator_gain * requirements.divider_ratio;
    let required_comp_gain = 1.0 / (plant_gain_at_fc * total_gain);

    // Required compensator phase boost
    // Loop phase = Plant phase + Comp phase + 180° (inverting)
    // PM = 180° + Loop phase at fc
    // So Comp phase = PM - 180° - Plant phase = PM + Plant phase - 180°
    // Wait, that's wrong. Let me recalculate.
    // For PM = target_pm, we need:
    // Phase margin = 180° + (plant_phase + comp_phase)
    // target_pm = 180° + plant_phase + comp_phase
    // comp_phase = target_pm - 180° - plant_phase
    let required_phase_boost = target_pm - 180.0 - plant_phase_at_fc;

    match comp_type {
        CompensatorType::TypeI => design_type_i(fc, required_comp_gain, requirements),
        CompensatorType::TypeII => {
            design_type_ii(fc, required_comp_gain, required_phase_boost, requirements)
        }
        CompensatorType::TypeIII => design_type_iii(
            fc,
            required_comp_gain,
            required_phase_boost,
            plant,
            requirements,
        ),
    }
}

/// Design Type I compensator (pure integrator)
fn design_type_i(
    fc: f64,
    required_gain: f64,
    req: &CompensatorRequirements,
) -> Result<CompensatorDesign, String> {
    // Type I: Gc(s) = ωi / s = 1/(s×R1×C1)
    // |Gc(jω)| = ωi / ω
    // At fc: required_gain = fi / fc
    // So fi = required_gain × fc

    let fi = required_gain * fc;
    let wi = 2.0 * PI * fi;

    // Choose practical component values
    // Start with R1 = 10kΩ, calculate C1
    let r1 = 10e3;
    let c1 = 1.0 / (2.0 * PI * fi * r1);

    // Verify C1 is practical (1pF to 10µF)
    let (r1, c1) = if c1 < 1e-12 {
        // C too small, increase R1
        let c1_target = 100e-12; // 100pF
        let r1 = 1.0 / (2.0 * PI * fi * c1_target);
        (r1, c1_target)
    } else if c1 > 10e-6 {
        // C too large, decrease R1
        let c1_target = 1e-6; // 1µF
        let r1 = 1.0 / (2.0 * PI * fi * c1_target);
        (r1, c1_target)
    } else {
        (r1, c1)
    };

    let components = CompensatorComponents::type_i(r1, c1);

    // Type I phase is always -90°
    let comp_phase = -90.0;

    let tf = TransferFunction {
        dc_gain: wi,
        zeros: vec![],
        poles: vec![Complex::new(0.0, 0.0)], // Pole at origin
        description: "Type I Compensator".to_string(),
    };

    // Calculate achieved phase margin (approximate)
    // This requires knowing the full loop, but for Type I it's limited
    let achieved_pm: f64 = 90.0 + comp_phase; // Very rough

    Ok(CompensatorDesign {
        comp_type: CompensatorType::TypeI,
        transfer_function: tf,
        components,
        crossover_freq: fc,
        phase_margin_deg: achieved_pm.max(0.0),
        gain_margin_db: f64::INFINITY, // Type I has infinite GM
        notes: vec![
            "Type I provides -90° phase at all frequencies".to_string(),
            "Best for current-mode control or single-pole plants".to_string(),
            format!("Integrator frequency: {:.1} Hz", fi),
        ],
    })
}

/// Design Type II compensator
fn design_type_ii(
    fc: f64,
    required_gain: f64,
    required_phase_boost: f64,
    req: &CompensatorRequirements,
) -> Result<CompensatorDesign, String> {
    // Type II: Gc(s) = (1 + s/ωz) / (s/ωi × (1 + s/ωp))
    //
    // Phase at fc: φ = -90° + arctan(fc/fz) - arctan(fc/fp)
    // Gain at fc: |Gc| = (fi/fc) × √(1 + (fc/fz)²) / √(1 + (fc/fp)²)

    // Maximum phase boost from Type II is about 90°
    // (achieved when fz << fc << fp)
    let max_phase_boost = 90.0;
    if required_phase_boost > max_phase_boost {
        return Err(format!(
            "Type II cannot provide {:.1}° phase boost (max ~90°). Use Type III.",
            required_phase_boost
        ));
    }

    // For optimal phase boost at fc, place fz and fp symmetrically in log scale
    // fz × fp = fc² (geometric mean at fc)
    // Phase boost = 2 × arctan(fc/fz) - 90°
    // For phase boost φ: fc/fz = tan((φ + 90°) / 2)

    let phase_boost_target = required_phase_boost.max(20.0); // At least 20° boost
    let ratio = ((phase_boost_target + 90.0) * PI / 360.0).tan();

    let fz = fc / ratio;
    let fp = fc * ratio;

    // Ensure fp doesn't exceed fsw/2 (practical limit)
    // Assume fsw is about 10× fc (common rule)
    let fsw_estimate = fc * 10.0;
    let fp = fp.min(fsw_estimate / 2.0);

    // Calculate required integrator frequency for correct gain at fc
    // |Gc(fc)| = (fi/fc) × √(1 + (fc/fz)²) / √(1 + (fc/fp)²)
    let zero_factor = (1.0 + (fc / fz).powi(2)).sqrt();
    let pole_factor = (1.0 + (fc / fp).powi(2)).sqrt();
    let fi = required_gain * fc * pole_factor / zero_factor;

    // Component calculation
    // Standard Type II op-amp configuration:
    //
    //       C2
    //    ┌──┬──┐
    //    │ R2  │
    //    │  C1 │
    // ───┼──┤├─┼───┤-\
    //    R1     │    >───
    // ──────────┴───┤+/
    //
    // fi = 1/(2π×R1×C1)
    // fz = 1/(2π×R2×C1)
    // fp = 1/(2π×R2×C2)

    let r1 = 10e3; // Start with 10kΩ
    let c1 = 1.0 / (2.0 * PI * fi * r1);
    let r2 = 1.0 / (2.0 * PI * fz * c1);
    let c2 = 1.0 / (2.0 * PI * fp * r2);

    // Scale if needed for practical values
    let (r1, c1, r2, c2) = scale_type_ii_components(r1, c1, r2, c2, fi, fz, fp);

    let components = CompensatorComponents::type_ii(r1, c1, r2, c2);

    // Build transfer function
    let wi = 2.0 * PI * fi;
    let wz = 2.0 * PI * fz;
    let wp = 2.0 * PI * fp;

    let tf = TransferFunction {
        dc_gain: wi / wz, // Normalized to have DC gain = fi/fz × high-freq-factor
        zeros: vec![Complex::new(-wz, 0.0)],
        poles: vec![Complex::new(0.0, 0.0), Complex::new(-wp, 0.0)],
        description: "Type II Compensator".to_string(),
    };

    // Calculate actual phase boost at fc
    let actual_phase = -90.0 + (fc / fz).atan() * 180.0 / PI - (fc / fp).atan() * 180.0 / PI;

    Ok(CompensatorDesign {
        comp_type: CompensatorType::TypeII,
        transfer_function: tf,
        components,
        crossover_freq: fc,
        phase_margin_deg: 90.0 + actual_phase, // Rough estimate
        gain_margin_db: 20.0,                  // Typical
        notes: vec![
            format!("Zero at {:.1} Hz", fz),
            format!("Pole at {:.1} Hz", fp),
            format!("Integrator at {:.1} Hz", fi),
            format!("Phase boost at fc: {:.1}°", actual_phase + 90.0),
        ],
    })
}

/// Design Type III compensator
fn design_type_iii(
    fc: f64,
    required_gain: f64,
    required_phase_boost: f64,
    plant: &TransferFunction,
    req: &CompensatorRequirements,
) -> Result<CompensatorDesign, String> {
    // Type III: Two zeros, two high-frequency poles
    // Maximum phase boost ~180° (achieved with zeros << fc << poles)
    //
    // For voltage-mode buck with LC double-pole:
    // - Place both zeros at or below f_LC to cancel the -180° phase shift
    // - Place poles at ESR zero and fsw/2

    // Find plant's LC resonance (look for complex conjugate poles)
    let f_lc = find_lc_resonance(plant).unwrap_or(fc / 10.0);

    // Strategy: place zeros at f_LC/2 and f_LC to boost phase through resonance
    let fz1 = f_lc / 2.0;
    let fz2 = f_lc;

    // Place high-frequency poles
    // fp1 at ESR zero (if exists) or at fc × 5
    // fp2 at fsw/2 estimate
    let fsw_estimate = fc * 10.0;
    let fp1 = find_esr_zero(plant).unwrap_or(fc * 5.0);
    let fp2 = fsw_estimate / 2.0;

    // Calculate integrator frequency for required gain
    let zero_factor1 = (1.0 + (fc / fz1).powi(2)).sqrt();
    let zero_factor2 = (1.0 + (fc / fz2).powi(2)).sqrt();
    let pole_factor1 = (1.0 + (fc / fp1).powi(2)).sqrt();
    let pole_factor2 = (1.0 + (fc / fp2).powi(2)).sqrt();

    let fi = required_gain * fc * pole_factor1 * pole_factor2 / (zero_factor1 * zero_factor2);

    // Component calculation for Type III
    // More complex topology - simplified calculation
    let r1 = 10e3;
    let c1 = 1.0 / (2.0 * PI * fi * r1);
    let r2 = 1.0 / (2.0 * PI * fz1 * c1);
    let c2 = 1.0 / (2.0 * PI * fp1 * r2);
    let r3 = 1.0 / (2.0 * PI * fz2 * c1); // Simplified
    let c3 = 1.0 / (2.0 * PI * fp2 * r3);

    let components = CompensatorComponents::type_iii(r1, c1, r2, c2, r3, c3);

    // Build transfer function
    let wi = 2.0 * PI * fi;
    let wz1 = 2.0 * PI * fz1;
    let wz2 = 2.0 * PI * fz2;
    let wp1 = 2.0 * PI * fp1;
    let wp2 = 2.0 * PI * fp2;

    let tf = TransferFunction {
        dc_gain: wi / (wz1 * wz2) * wp1 * wp2, // Normalized
        zeros: vec![Complex::new(-wz1, 0.0), Complex::new(-wz2, 0.0)],
        poles: vec![
            Complex::new(0.0, 0.0), // Integrator
            Complex::new(-wp1, 0.0),
            Complex::new(-wp2, 0.0),
        ],
        description: "Type III Compensator".to_string(),
    };

    // Calculate actual phase at fc
    let phase_zeros = (fc / fz1).atan() + (fc / fz2).atan();
    let phase_poles = (fc / fp1).atan() + (fc / fp2).atan();
    let actual_phase = -90.0 + phase_zeros * 180.0 / PI - phase_poles * 180.0 / PI;

    Ok(CompensatorDesign {
        comp_type: CompensatorType::TypeIII,
        transfer_function: tf,
        components,
        crossover_freq: fc,
        phase_margin_deg: 90.0 + actual_phase,
        gain_margin_db: 15.0, // Typical
        notes: vec![
            format!("Zero 1 at {:.1} Hz (at f_LC/2)", fz1),
            format!("Zero 2 at {:.1} Hz (at f_LC)", fz2),
            format!("Pole 1 at {:.1} Hz", fp1),
            format!("Pole 2 at {:.1} Hz", fp2),
            format!("Integrator at {:.1} Hz", fi),
            format!("Phase boost at fc: {:.1}°", actual_phase + 90.0),
        ],
    })
}

/// Find LC resonance from plant poles
fn find_lc_resonance(plant: &TransferFunction) -> Option<f64> {
    for pole in &plant.poles {
        if pole.im.abs() > 1e-6 {
            // Complex pole - LC resonance
            let w0 = pole.magnitude();
            return Some(w0 / (2.0 * PI));
        }
    }
    None
}

/// Find ESR zero from plant zeros
fn find_esr_zero(plant: &TransferFunction) -> Option<f64> {
    for zero in &plant.zeros {
        if zero.re < 0.0 && zero.im.abs() < 1e-6 {
            // Real negative zero (LHP) - likely ESR
            return Some(-zero.re / (2.0 * PI));
        }
    }
    None
}

/// Scale Type II components to practical values
fn scale_type_ii_components(
    r1: f64,
    c1: f64,
    r2: f64,
    c2: f64,
    fi: f64,
    fz: f64,
    fp: f64,
) -> (f64, f64, f64, f64) {
    // Target ranges:
    // R: 1kΩ to 1MΩ
    // C: 10pF to 10µF

    let c1_min = 10e-12;
    let c1_max = 10e-6;

    if c1 < c1_min {
        // Scale up
        let scale = c1_min / c1;
        let new_c1 = c1 * scale;
        let new_r1 = r1 / scale;
        let new_r2 = r2 / scale;
        let new_c2 = c2 * scale;
        (new_r1, new_c1, new_r2, new_c2)
    } else if c1 > c1_max {
        // Scale down
        let scale = c1_max / c1;
        let new_c1 = c1 * scale;
        let new_r1 = r1 / scale;
        let new_r2 = r2 / scale;
        let new_c2 = c2 * scale;
        (new_r1, new_c1, new_r2, new_c2)
    } else {
        (r1, c1, r2, c2)
    }
}

// ============================================================================
// K-FACTOR METHOD FOR TYPE II
// ============================================================================

/// K-factor method for Type II compensator design
///
/// This is a systematic method that directly targets a specific
/// phase margin at a specific crossover frequency.
pub fn design_type_ii_k_factor(
    plant: &TransferFunction,
    fc: f64,
    target_pm_deg: f64,
    modulator_gain: f64,
) -> Result<CompensatorDesign, String> {
    // Get plant gain and phase at fc
    let plant_at_fc = plant.evaluate(fc);
    let plant_gain = plant_at_fc.magnitude();
    let plant_phase = plant_at_fc.phase_deg();

    // Required phase boost from compensator
    // PM = 180° + plant_phase + comp_phase
    // comp_phase = PM - 180° - plant_phase
    let required_boost = target_pm_deg - 180.0 - plant_phase;

    // K factor relates zero/pole placement to phase boost
    // Phase boost = arctan(K) - arctan(1/K)
    // Solving: K = tan((90° + boost) / 2)
    let boost_rad = required_boost * PI / 180.0;
    let k = ((PI / 2.0 + boost_rad) / 2.0).tan();

    if k <= 0.0 || k > 100.0 {
        return Err(format!(
            "K-factor {:.2} out of range for required boost {:.1}°",
            k, required_boost
        ));
    }

    // Place zero and pole symmetrically around fc
    let fz = fc / k;
    let fp = fc * k;

    // Required compensator gain at fc
    let required_gain = 1.0 / (plant_gain * modulator_gain);

    // Calculate integrator frequency
    let zero_factor = (1.0 + k * k).sqrt();
    let pole_factor = (1.0 + 1.0 / (k * k)).sqrt();
    let fi = required_gain * fc * pole_factor / zero_factor;

    // Component values
    let r1 = 10e3;
    let c1 = 1.0 / (2.0 * PI * fi * r1);
    let r2 = 1.0 / (2.0 * PI * fz * c1);
    let c2 = 1.0 / (2.0 * PI * fp * r2);

    let (r1, c1, r2, c2) = scale_type_ii_components(r1, c1, r2, c2, fi, fz, fp);
    let components = CompensatorComponents::type_ii(r1, c1, r2, c2);

    // Transfer function
    let wi = 2.0 * PI * fi;
    let wz = 2.0 * PI * fz;
    let wp = 2.0 * PI * fp;

    let tf = TransferFunction {
        dc_gain: wi / wz,
        zeros: vec![Complex::new(-wz, 0.0)],
        poles: vec![Complex::new(0.0, 0.0), Complex::new(-wp, 0.0)],
        description: "Type II (K-factor)".to_string(),
    };

    Ok(CompensatorDesign {
        comp_type: CompensatorType::TypeII,
        transfer_function: tf,
        components,
        crossover_freq: fc,
        phase_margin_deg: target_pm_deg,
        gain_margin_db: 20.0 * (fp / fc).log10(), // Approximate
        notes: vec![
            format!("K-factor: {:.2}", k),
            format!("Zero at {:.1} Hz", fz),
            format!("Pole at {:.1} Hz", fp),
            format!("Phase boost: {:.1}°", required_boost),
        ],
    })
}

// ============================================================================
// PRACTICAL COMPONENT SELECTION
// ============================================================================

/// Select nearest standard E24 resistor value
pub fn nearest_e24_resistor(target: f64) -> f64 {
    const E24: [f64; 24] = [
        1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1,
        5.6, 6.2, 6.8, 7.5, 8.2, 9.1,
    ];

    let decade = target.log10().floor();
    let mantissa = target / 10f64.powf(decade);

    let mut best = E24[0];
    let mut best_err = (mantissa - best).abs();

    // Check values in current decade
    for &val in &E24 {
        let err = (mantissa - val).abs();
        if err < best_err {
            best = val;
            best_err = err;
        }
    }

    // Also check 10.0 (first value of next decade) for values close to 10
    let err_10 = (mantissa - 10.0).abs();
    if err_10 < best_err {
        return 10f64.powf(decade + 1.0);
    }

    best * 10f64.powf(decade)
}

/// Select nearest standard capacitor value (E12 series)
pub fn nearest_e12_capacitor(target: f64) -> f64 {
    const E12: [f64; 12] = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];

    let decade = target.log10().floor();
    let mantissa = target / 10f64.powf(decade);

    let mut best = E12[0];
    let mut best_err = (mantissa - best).abs();

    for &val in &E12 {
        let err = (mantissa - val).abs();
        if err < best_err {
            best = val;
            best_err = err;
        }
    }

    // Also check 10.0 (first value of next decade) for values close to 10
    let err_10 = (mantissa - 10.0).abs();
    if err_10 < best_err {
        return 10f64.powf(decade + 1.0);
    }

    best * 10f64.powf(decade)
}

/// Snap compensator components to standard values
pub fn snap_to_standard_values(components: &CompensatorComponents) -> CompensatorComponents {
    CompensatorComponents {
        r1: nearest_e24_resistor(components.r1),
        c1: nearest_e12_capacitor(components.c1),
        r2: components.r2.map(nearest_e24_resistor),
        c2: components.c2.map(nearest_e12_capacitor),
        r3: components.r3.map(nearest_e24_resistor),
        c3: components.c3.map(nearest_e12_capacitor),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power::control::small_signal::BuckSmallSignal;

    #[test]
    fn test_type_i_design() {
        let req = CompensatorRequirements {
            crossover_freq: 10e3,
            phase_margin_deg: 45.0,
            ..Default::default()
        };

        let result = design_type_i(10e3, 0.1, &req);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert_eq!(design.comp_type, CompensatorType::TypeI);
        assert!(design.components.r1 > 0.0);
        assert!(design.components.c1 > 0.0);
    }

    #[test]
    fn test_type_ii_design() {
        // Create a buck converter plant
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let plant = buck.control_to_output();

        let req = CompensatorRequirements {
            crossover_freq: 30e3,
            phase_margin_deg: 55.0,
            modulator_gain: 1.0,
            ..Default::default()
        };

        let result = design_compensator(&plant, &req, CompensatorType::TypeII);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert_eq!(design.comp_type, CompensatorType::TypeII);

        // Should have zero and pole
        let fz = design.components.zero_freq();
        let fp = design.components.pole_freq();
        assert!(fz.is_some());
        assert!(fp.is_some());

        // Zero should be below crossover, pole above
        assert!(fz.unwrap() < req.crossover_freq);
        assert!(fp.unwrap() > req.crossover_freq);
    }

    #[test]
    fn test_type_iii_design() {
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let plant = buck.control_to_output();

        let req = CompensatorRequirements {
            crossover_freq: 50e3,
            phase_margin_deg: 60.0,
            modulator_gain: 1.0,
            ..Default::default()
        };

        let result = design_compensator(&plant, &req, CompensatorType::TypeIII);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert_eq!(design.comp_type, CompensatorType::TypeIII);

        // Type III should have 2 zeros and 3 poles (including integrator)
        assert_eq!(design.transfer_function.zeros.len(), 2);
        assert_eq!(design.transfer_function.poles.len(), 3);
    }

    #[test]
    fn test_k_factor_method() {
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let plant = buck.control_to_output();

        let result = design_type_ii_k_factor(&plant, 30e3, 55.0, 1.0);
        assert!(result.is_ok());

        let design = result.unwrap();
        println!("K-factor design notes: {:?}", design.notes);
    }

    #[test]
    fn test_standard_value_selection() {
        // Test resistor snapping
        assert!((nearest_e24_resistor(9800.0) - 10000.0).abs() < 100.0);
        assert!((nearest_e24_resistor(15500.0) - 15000.0).abs() < 1000.0);

        // Test capacitor snapping
        assert!((nearest_e12_capacitor(98e-9) - 100e-9).abs() < 10e-9);
    }

    #[test]
    fn test_component_frequencies() {
        let comp = CompensatorComponents::type_ii(10e3, 100e-9, 47e3, 22e-12);

        let fi = comp.integrator_freq();
        let fz = comp.zero_freq().unwrap();
        let fp = comp.pole_freq().unwrap();

        // fi = 1/(2π × 10kΩ × 100nF) ≈ 159 Hz
        assert!(fi > 100.0 && fi < 200.0);

        // fz = 1/(2π × 47kΩ × 100nF) ≈ 34 Hz
        assert!(fz > 20.0 && fz < 50.0);

        // fp = 1/(2π × 47kΩ × 22pF) ≈ 154 kHz
        assert!(fp > 100e3 && fp < 200e3);
    }
}
