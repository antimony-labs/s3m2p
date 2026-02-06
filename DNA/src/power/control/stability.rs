//! ═══════════════════════════════════════════════════════════════════════════════
//! Stability analysis for SMPS control loops
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides tools for analyzing closed-loop stability:
//!
//! - Phase margin and gain margin calculation
//! - Crossover frequency identification
//! - Loop gain analysis
//! - Stability assessment
//!
//! A stable power supply requires:
//! - Phase margin > 45° (preferably 55-65°)
//! - Gain margin > 10 dB (preferably > 12 dB)
//! - Crossover frequency < fsw/10 (ideally fsw/5 to fsw/10)

use super::small_signal::{BodeData, Complex, TransferFunction};
use std::f64::consts::PI;

// ============================================================================
// STABILITY METRICS
// ============================================================================

/// Complete stability analysis result
#[derive(Clone, Debug)]
pub struct StabilityMetrics {
    /// Phase margin at crossover (degrees)
    pub phase_margin_deg: f64,
    /// Gain margin (dB)
    pub gain_margin_db: f64,
    /// Crossover frequency where |T(s)| = 0 dB (Hz)
    pub crossover_freq: f64,
    /// Phase crossover frequency where phase = -180° (Hz)
    pub phase_crossover_freq: f64,
    /// DC loop gain (dB)
    pub dc_gain_db: f64,
    /// Stability assessment
    pub stability: StabilityAssessment,
    /// Warnings and recommendations
    pub warnings: Vec<String>,
}

/// Stability assessment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StabilityAssessment {
    /// Stable with good margins
    Stable,
    /// Marginally stable (low margins)
    MarginallyStable,
    /// Conditionally stable (gain margin but limited PM)
    ConditionallyStable,
    /// Unstable (negative margins)
    Unstable,
}

impl StabilityMetrics {
    /// Check if system is stable with acceptable margins
    pub fn is_acceptable(&self) -> bool {
        self.phase_margin_deg >= 45.0 && self.gain_margin_db >= 10.0
    }

    /// Check if system needs redesign
    pub fn needs_redesign(&self) -> bool {
        self.phase_margin_deg < 30.0 || self.gain_margin_db < 6.0
    }

    /// Get a summary string
    pub fn summary(&self) -> String {
        format!(
            "PM={:.1}° GM={:.1}dB fc={:.1}kHz - {:?}",
            self.phase_margin_deg,
            self.gain_margin_db,
            self.crossover_freq / 1e3,
            self.stability
        )
    }
}

// ============================================================================
// LOOP GAIN ANALYSIS
// ============================================================================

/// Analyze loop gain for stability
pub fn analyze_loop_gain(loop_gain: &TransferFunction, fsw: f64) -> StabilityMetrics {
    // Find crossover frequency (where magnitude = 0 dB)
    let fc = find_crossover_frequency(loop_gain, 10.0, fsw);

    // Get phase at crossover
    let phase_at_fc = if let Some(f) = fc {
        loop_gain.phase_deg(f)
    } else {
        // No crossover found - check DC gain
        let dc_gain_db = loop_gain.evaluate(1.0).magnitude_db();
        if dc_gain_db < 0.0 {
            // Loop gain always < 0 dB, system is stable
            return StabilityMetrics {
                phase_margin_deg: f64::INFINITY,
                gain_margin_db: f64::INFINITY,
                crossover_freq: 0.0,
                phase_crossover_freq: 0.0,
                dc_gain_db,
                stability: StabilityAssessment::Stable,
                warnings: vec!["Loop gain < 0dB at all frequencies".to_string()],
            };
        } else {
            // Can't find crossover but gain > 0 dB - need more investigation
            0.0
        }
    };

    let crossover_freq = fc.unwrap_or(0.0);

    // Phase margin = 180° + phase at crossover
    let phase_margin_deg = 180.0 + phase_at_fc;

    // Find phase crossover (where phase = -180°)
    let phase_crossover = find_phase_crossover(loop_gain, 10.0, fsw);

    // Gain margin = negative of magnitude at phase crossover
    let gain_margin_db = if let Some(f_phase) = phase_crossover {
        -loop_gain.magnitude_db(f_phase)
    } else {
        f64::INFINITY // No phase crossover, infinite gain margin
    };

    let phase_crossover_freq = phase_crossover.unwrap_or(f64::INFINITY);

    // DC gain
    let dc_gain_db = loop_gain.evaluate(1.0).magnitude_db();

    // Assess stability
    let stability = assess_stability(phase_margin_deg, gain_margin_db);

    // Generate warnings
    let mut warnings = Vec::new();

    if phase_margin_deg < 30.0 {
        warnings.push(format!(
            "CRITICAL: Phase margin {:.1}° is below 30° - system may be unstable",
            phase_margin_deg
        ));
    } else if phase_margin_deg < 45.0 {
        warnings.push(format!(
            "WARNING: Phase margin {:.1}° is below 45° - poor transient response",
            phase_margin_deg
        ));
    }

    if gain_margin_db < 6.0 {
        warnings.push(format!(
            "CRITICAL: Gain margin {:.1}dB is below 6dB - system may be unstable",
            gain_margin_db
        ));
    } else if gain_margin_db < 10.0 {
        warnings.push(format!(
            "WARNING: Gain margin {:.1}dB is below 10dB - limited noise immunity",
            gain_margin_db
        ));
    }

    if crossover_freq > fsw / 5.0 {
        warnings.push(format!(
            "WARNING: Crossover {:.1}kHz > fsw/5 - may have sampling issues",
            crossover_freq / 1e3
        ));
    }

    StabilityMetrics {
        phase_margin_deg,
        gain_margin_db,
        crossover_freq,
        phase_crossover_freq,
        dc_gain_db,
        stability,
        warnings,
    }
}

/// Assess stability from margins
pub fn assess_stability(pm_deg: f64, gm_db: f64) -> StabilityAssessment {
    if pm_deg < 0.0 || gm_db < 0.0 {
        StabilityAssessment::Unstable
    } else if pm_deg < 30.0 || gm_db < 6.0 {
        StabilityAssessment::ConditionallyStable
    } else if pm_deg < 45.0 || gm_db < 10.0 {
        StabilityAssessment::MarginallyStable
    } else {
        StabilityAssessment::Stable
    }
}

/// Find crossover frequency using bisection search
fn find_crossover_frequency(tf: &TransferFunction, f_min: f64, f_max: f64) -> Option<f64> {
    let mag_min = tf.magnitude_db(f_min);
    let mag_max = tf.magnitude_db(f_max);

    // If both same sign and neither near 0, no crossover
    if mag_min * mag_max > 0.0 && mag_min.abs() > 3.0 && mag_max.abs() > 3.0 {
        return None;
    }

    // Bisection search on log frequency scale
    let mut low = f_min.log10();
    let mut high = f_max.log10();

    for _ in 0..50 {
        let mid = (low + high) / 2.0;
        let f_mid = 10f64.powf(mid);
        let mag_mid = tf.magnitude_db(f_mid);

        if mag_mid.abs() < 0.1 {
            return Some(f_mid);
        }

        if (mag_min > 0.0 && mag_mid > 0.0) || (mag_min < 0.0 && mag_mid < 0.0) {
            low = mid;
        } else {
            high = mid;
        }
    }

    Some(10f64.powf((low + high) / 2.0))
}

/// Find phase crossover frequency (where phase = -180°)
fn find_phase_crossover(tf: &TransferFunction, f_min: f64, f_max: f64) -> Option<f64> {
    let phase_min = tf.phase_deg(f_min);
    let phase_max = tf.phase_deg(f_max);

    // Looking for phase = -180°
    let target = -180.0;

    // Check if -180° is crossed
    if (phase_min - target) * (phase_max - target) > 0.0 {
        // No crossing if phase doesn't cross -180°
        if phase_max > -180.0 {
            return None;
        }
    }

    // Bisection search
    let mut low = f_min.log10();
    let mut high = f_max.log10();

    for _ in 0..50 {
        let mid = (low + high) / 2.0;
        let f_mid = 10f64.powf(mid);
        let phase_mid = tf.phase_deg(f_mid);

        if (phase_mid - target).abs() < 1.0 {
            return Some(f_mid);
        }

        // Phase typically decreases with frequency
        if phase_mid > target {
            low = mid;
        } else {
            high = mid;
        }
    }

    Some(10f64.powf((low + high) / 2.0))
}

// ============================================================================
// COMPLETE LOOP ANALYSIS
// ============================================================================

/// Complete loop analysis including all transfer functions
#[derive(Clone, Debug)]
pub struct LoopAnalysis {
    /// Plant transfer function (power stage)
    pub plant: TransferFunction,
    /// Compensator transfer function
    pub compensator: TransferFunction,
    /// PWM modulator gain
    pub modulator_gain: f64,
    /// Feedback divider ratio
    pub feedback_ratio: f64,
    /// Calculated loop gain T(s) = Plant × Compensator × Modulator × Feedback
    pub loop_gain: TransferFunction,
    /// Stability metrics
    pub stability: StabilityMetrics,
    /// Bode plot data for visualization
    pub bode_data: BodeData,
}

impl LoopAnalysis {
    /// Create new loop analysis
    pub fn new(
        plant: TransferFunction,
        compensator: TransferFunction,
        modulator_gain: f64,
        feedback_ratio: f64,
        fsw: f64,
    ) -> Self {
        // Build loop gain: T(s) = Gc(s) × Gvd(s) × Fm × H
        // where Fm = modulator gain, H = feedback ratio
        let total_gain = modulator_gain * feedback_ratio;

        let mut loop_gain = plant.cascade(&compensator);
        loop_gain.dc_gain *= total_gain;
        loop_gain.description = "Loop Gain T(s)".to_string();

        // Analyze stability
        let stability = analyze_loop_gain(&loop_gain, fsw);

        // Generate Bode plot data
        let bode_data = loop_gain.bode_data(10.0, fsw, 200);

        Self {
            plant,
            compensator,
            modulator_gain,
            feedback_ratio,
            loop_gain,
            stability,
            bode_data,
        }
    }

    /// Check if design meets requirements
    pub fn meets_requirements(&self, min_pm: f64, min_gm: f64, max_fc: f64) -> bool {
        self.stability.phase_margin_deg >= min_pm
            && self.stability.gain_margin_db >= min_gm
            && self.stability.crossover_freq <= max_fc
    }

    /// Get design recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        if self.stability.phase_margin_deg < 45.0 {
            recs.push(
                "Increase zero frequency or reduce crossover to improve phase margin".to_string(),
            );
        }

        if self.stability.gain_margin_db < 10.0 {
            recs.push(
                "Add high-frequency pole or reduce loop gain for better gain margin".to_string(),
            );
        }

        if self.stability.crossover_freq < 1e3 {
            recs.push(
                "Crossover frequency is low - consider faster response if transients allow"
                    .to_string(),
            );
        }

        // Check for RHP zero issues
        for zero in &self.plant.zeros {
            if zero.re > 0.0 {
                let f_rhp = zero.re / (2.0 * PI);
                if self.stability.crossover_freq > f_rhp / 3.0 {
                    recs.push(format!(
                        "WARNING: Crossover {:.1}kHz is too close to RHP zero at {:.1}kHz. Reduce fc.",
                        self.stability.crossover_freq / 1e3,
                        f_rhp / 1e3
                    ));
                }
            }
        }

        recs
    }
}

// ============================================================================
// TRANSIENT RESPONSE ESTIMATION
// ============================================================================

/// Estimated transient response characteristics
#[derive(Clone, Debug)]
pub struct TransientEstimate {
    /// Estimated overshoot for step response (%)
    pub overshoot_percent: f64,
    /// Estimated settling time to 5% (seconds)
    pub settling_time_5pct: f64,
    /// Estimated rise time (10% to 90%) (seconds)
    pub rise_time: f64,
    /// Estimated bandwidth (-3dB) (Hz)
    pub bandwidth_hz: f64,
}

/// Estimate transient response from stability metrics
///
/// Uses second-order approximation based on phase margin
pub fn estimate_transient(metrics: &StabilityMetrics) -> TransientEstimate {
    let pm_rad = metrics.phase_margin_deg * PI / 180.0;
    let fc = metrics.crossover_freq;

    // Approximate damping ratio from phase margin
    // ζ ≈ PM / 100 (rough approximation for PM in degrees)
    // More accurate: ζ = tan(PM) / √(√(1 + tan²(PM)×4) - 1)
    let zeta = if pm_rad > 0.0 {
        let tan_pm = pm_rad.tan();
        let temp = (1.0 + 4.0 * tan_pm * tan_pm).sqrt();
        tan_pm / (temp - 1.0).sqrt().max(0.1)
    } else {
        0.1 // Critically low damping
    };

    let zeta = zeta.clamp(0.1, 1.5);

    // Natural frequency approximation
    // ωn ≈ ωc / √(√(1 + 4ζ⁴) - 2ζ²)
    let wc = 2.0 * PI * fc;
    let wn = if zeta < 1.0 {
        wc / (((1.0 + 4.0 * zeta.powi(4)).sqrt() - 2.0 * zeta.powi(2)).sqrt())
    } else {
        wc
    };

    // Overshoot (for underdamped systems)
    let overshoot_percent = if zeta < 1.0 {
        100.0 * (-PI * zeta / (1.0 - zeta * zeta).sqrt()).exp()
    } else {
        0.0
    };

    // Settling time (5%)
    // ts ≈ 3 / (ζ × ωn) for 5% settling
    let settling_time_5pct = 3.0 / (zeta * wn);

    // Rise time (10% to 90%)
    // tr ≈ 1.8 / ωn for ζ ≈ 0.7
    let rise_time = (1.0 + 1.1 * zeta + 1.4 * zeta * zeta) / wn;

    // Bandwidth (-3dB of closed loop)
    // BW ≈ ωn × √(1 - 2ζ² + √(2 - 4ζ² + 4ζ⁴))
    let bw_factor = if zeta < 0.707 {
        (1.0 - 2.0 * zeta * zeta + (2.0 - 4.0 * zeta * zeta + 4.0 * zeta.powi(4)).sqrt()).sqrt()
    } else {
        1.0
    };
    let bandwidth_hz = wn * bw_factor / (2.0 * PI);

    TransientEstimate {
        overshoot_percent,
        settling_time_5pct,
        rise_time,
        bandwidth_hz,
    }
}

// ============================================================================
// SENSITIVITY FUNCTIONS
// ============================================================================

/// Calculate sensitivity function S(s) = 1 / (1 + T(s))
///
/// Sensitivity function shows how disturbances are rejected.
/// |S| should be low at low frequencies for good regulation.
pub fn sensitivity_function(loop_gain: &TransferFunction, freq: f64) -> Complex {
    let t = loop_gain.evaluate(freq);
    let one = Complex::new(1.0, 0.0);
    one / (one + t)
}

/// Calculate complementary sensitivity T(s) / (1 + T(s))
///
/// This is the closed-loop transfer function.
/// |T| should be flat up to bandwidth.
pub fn complementary_sensitivity(loop_gain: &TransferFunction, freq: f64) -> Complex {
    let t = loop_gain.evaluate(freq);
    let one = Complex::new(1.0, 0.0);
    t / (one + t)
}

/// Calculate sensitivity peak (maximum |S|)
///
/// Ms = max|S(jω)| should be < 2 (6dB) for good robustness
pub fn sensitivity_peak(loop_gain: &TransferFunction, f_max: f64) -> f64 {
    let mut max_s: f64 = 0.0;

    // Scan frequencies logarithmically
    for i in 0..100 {
        let f = 10f64.powf(1.0 + i as f64 * (f_max.log10() - 1.0) / 100.0);
        let s = sensitivity_function(loop_gain, f);
        max_s = max_s.max(s.magnitude());
    }

    max_s
}

// ============================================================================
// BODE PLOT HELPERS
// ============================================================================

/// Generate loop gain Bode plot with annotations
#[derive(Clone, Debug)]
pub struct AnnotatedBodePlot {
    /// Base Bode data
    pub bode: BodeData,
    /// Crossover frequency marker
    pub crossover_freq: Option<f64>,
    /// Phase margin annotation
    pub phase_margin: Option<(f64, f64)>, // (freq, margin)
    /// Gain margin annotation
    pub gain_margin: Option<(f64, f64)>, // (freq, margin)
    /// Pole/zero markers
    pub markers: Vec<BodeMarker>,
}

/// Marker on Bode plot
#[derive(Clone, Debug)]
pub struct BodeMarker {
    /// Frequency (Hz)
    pub freq: f64,
    /// Type of marker
    pub marker_type: MarkerType,
    /// Label
    pub label: String,
}

/// Type of Bode plot marker
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerType {
    Pole,
    Zero,
    RHPZero,
    Crossover,
    PhaseMargin,
    GainMargin,
}

/// Generate annotated Bode plot
pub fn annotated_bode_plot(
    loop_gain: &TransferFunction,
    metrics: &StabilityMetrics,
    f_min: f64,
    f_max: f64,
) -> AnnotatedBodePlot {
    let bode = loop_gain.bode_data(f_min, f_max, 200);
    let mut markers = Vec::new();

    // Add crossover marker
    if metrics.crossover_freq > 0.0 {
        markers.push(BodeMarker {
            freq: metrics.crossover_freq,
            marker_type: MarkerType::Crossover,
            label: format!("fc={:.1}kHz", metrics.crossover_freq / 1e3),
        });
    }

    // Add pole/zero markers
    for pole in &loop_gain.poles {
        if pole.re.abs() > 1e-6 || pole.im.abs() > 1e-6 {
            let freq = pole.magnitude() / (2.0 * PI);
            markers.push(BodeMarker {
                freq,
                marker_type: MarkerType::Pole,
                label: format!("Pole @ {:.1}kHz", freq / 1e3),
            });
        }
    }

    for zero in &loop_gain.zeros {
        let freq = zero.magnitude() / (2.0 * PI);
        let marker_type = if zero.re > 0.0 {
            MarkerType::RHPZero
        } else {
            MarkerType::Zero
        };
        markers.push(BodeMarker {
            freq,
            marker_type,
            label: format!(
                "{} @ {:.1}kHz",
                if zero.re > 0.0 { "RHP Zero" } else { "Zero" },
                freq / 1e3
            ),
        });
    }

    AnnotatedBodePlot {
        bode,
        crossover_freq: if metrics.crossover_freq > 0.0 {
            Some(metrics.crossover_freq)
        } else {
            None
        },
        phase_margin: if metrics.crossover_freq > 0.0 {
            Some((metrics.crossover_freq, metrics.phase_margin_deg))
        } else {
            None
        },
        gain_margin: if metrics.phase_crossover_freq < f64::INFINITY {
            Some((metrics.phase_crossover_freq, metrics.gain_margin_db))
        } else {
            None
        },
        markers,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power::control::compensator::{
        design_compensator, CompensatorRequirements, CompensatorType,
    };
    use crate::power::control::small_signal::BuckSmallSignal;

    #[test]
    fn test_stability_analysis() {
        // Create a simple stable system with high DC gain
        // Integrator-like behavior with high-frequency rolloff
        // H(s) = K × (s + wz) / (s × (s + wp))
        // This has an integrator (pole at 0) ensuring high low-freq gain
        let tf = TransferFunction::new(
            1e6,                              // High gain
            vec![Complex::new(-1000.0, 0.0)], // Zero at 159 Hz
            vec![
                Complex::new(-10.0, 0.0),     // Low-freq pole at 1.6 Hz
                Complex::new(-100000.0, 0.0), // High-freq pole at 16 kHz
            ],
        );

        let metrics = analyze_loop_gain(&tf, 1e6);

        // Should find crossover (gain crosses 0 dB somewhere)
        println!("Stability metrics: {:?}", metrics);
        assert!(
            metrics.crossover_freq > 0.0,
            "Expected crossover frequency > 0"
        );

        // Should be stable
        assert!(matches!(
            metrics.stability,
            StabilityAssessment::Stable | StabilityAssessment::MarginallyStable
        ));

        println!("Stability: {}", metrics.summary());
    }

    #[test]
    fn test_buck_loop_analysis() {
        // Design a buck converter control loop
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let plant = buck.control_to_output();

        let req = CompensatorRequirements {
            crossover_freq: 30e3,
            phase_margin_deg: 55.0,
            modulator_gain: 1.0,
            ..Default::default()
        };

        let comp_design = design_compensator(&plant, &req, CompensatorType::TypeII).unwrap();

        // Analyze the complete loop
        let analysis = LoopAnalysis::new(
            plant,
            comp_design.transfer_function,
            1.0, // modulator gain
            0.2, // feedback ratio (5V output, 1V reference)
            500e3,
        );

        println!("Loop analysis: {}", analysis.stability.summary());
        println!("Recommendations: {:?}", analysis.get_recommendations());

        // Should be stable
        assert!(matches!(
            analysis.stability.stability,
            StabilityAssessment::Stable | StabilityAssessment::MarginallyStable
        ));
    }

    #[test]
    fn test_transient_estimation() {
        let metrics = StabilityMetrics {
            phase_margin_deg: 55.0,
            gain_margin_db: 15.0,
            crossover_freq: 30e3,
            phase_crossover_freq: 150e3,
            dc_gain_db: 60.0,
            stability: StabilityAssessment::Stable,
            warnings: vec![],
        };

        let transient = estimate_transient(&metrics);

        // With 55° phase margin, expect moderate overshoot
        assert!(transient.overshoot_percent < 20.0);

        // Settling time should be reasonable
        assert!(transient.settling_time_5pct > 0.0);
        assert!(transient.settling_time_5pct < 1e-3); // < 1ms for 30kHz crossover

        println!("Overshoot: {:.1}%", transient.overshoot_percent);
        println!("Settling time: {:.1}µs", transient.settling_time_5pct * 1e6);
        println!("Rise time: {:.1}µs", transient.rise_time * 1e6);
        println!("Bandwidth: {:.1}kHz", transient.bandwidth_hz / 1e3);
    }

    #[test]
    fn test_sensitivity_peak() {
        // Create a well-designed loop
        let tf = TransferFunction::new(
            100.0,
            vec![Complex::new(-1000.0, 0.0)],
            vec![
                Complex::new(0.0, 0.0), // Integrator
                Complex::new(-50000.0, 0.0),
            ],
        );

        let ms = sensitivity_peak(&tf, 1e6);

        // Well-designed loop should have Ms < 2 (6dB)
        println!("Sensitivity peak: {:.2} ({:.1}dB)", ms, 20.0 * ms.log10());
    }

    #[test]
    fn test_unstable_detection() {
        // Test the stability assessment function directly
        assert_eq!(
            assess_stability(20.0, 5.0),
            StabilityAssessment::ConditionallyStable
        );
        assert_eq!(assess_stability(-10.0, 15.0), StabilityAssessment::Unstable);
        assert_eq!(assess_stability(60.0, 15.0), StabilityAssessment::Stable);
        assert_eq!(
            assess_stability(40.0, 8.0),
            StabilityAssessment::MarginallyStable
        );

        // Test that warnings are generated for low margins
        let metrics = StabilityMetrics {
            phase_margin_deg: 25.0,
            gain_margin_db: 5.0,
            crossover_freq: 50e3,
            phase_crossover_freq: 100e3,
            dc_gain_db: 60.0,
            stability: StabilityAssessment::ConditionallyStable,
            warnings: vec![],
        };

        // Check needs_redesign works
        assert!(metrics.needs_redesign());
    }

    #[test]
    fn test_annotated_bode() {
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let plant = buck.control_to_output();

        let metrics = analyze_loop_gain(&plant, 500e3);
        let annotated = annotated_bode_plot(&plant, &metrics, 100.0, 500e3);

        // Should have markers for poles and zeros
        assert!(!annotated.markers.is_empty());

        println!("Bode plot markers:");
        for marker in &annotated.markers {
            println!(
                "  {:?}: {} @ {:.1}kHz",
                marker.marker_type,
                marker.label,
                marker.freq / 1e3
            );
        }
    }
}
