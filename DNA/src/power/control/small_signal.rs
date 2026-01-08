//! ═══════════════════════════════════════════════════════════════════════════════
//! Small-signal models for SMPS topologies
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides linearized transfer functions around the operating point for:
//! - Control-to-output (Gvd): How duty cycle changes affect output voltage
//! - Line-to-output (Gvg): How input voltage changes affect output voltage
//! - Output impedance (Zout): Dynamic output impedance
//!
//! All models assume CCM operation unless otherwise specified.

use std::f64::consts::PI;

// ============================================================================
// COMPLEX NUMBER SUPPORT
// ============================================================================

/// Complex number for frequency domain analysis
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(mag: f64, phase_rad: f64) -> Self {
        Self {
            re: mag * phase_rad.cos(),
            im: mag * phase_rad.sin(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase_rad(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn phase_deg(&self) -> f64 {
        self.phase_rad() * 180.0 / PI
    }

    pub fn magnitude_db(&self) -> f64 {
        20.0 * self.magnitude().log10()
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn inv(&self) -> Self {
        let denom = self.re * self.re + self.im * self.im;
        Self {
            re: self.re / denom,
            im: -self.im / denom,
        }
    }

    /// Create complex number for s = jω
    pub fn jw(freq_hz: f64) -> Self {
        Self {
            re: 0.0,
            im: 2.0 * PI * freq_hz,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

// ============================================================================
// TRANSFER FUNCTION
// ============================================================================

/// A general transfer function H(s) = K × Π(s - zeros) / Π(s - poles)
#[derive(Clone, Debug)]
pub struct TransferFunction {
    /// DC gain (or gain at reference frequency)
    pub dc_gain: f64,
    /// Zeros as complex frequencies (rad/s)
    pub zeros: Vec<Complex>,
    /// Poles as complex frequencies (rad/s)
    pub poles: Vec<Complex>,
    /// Description of the transfer function
    pub description: String,
}

impl TransferFunction {
    /// Create a new transfer function
    pub fn new(dc_gain: f64, zeros: Vec<Complex>, poles: Vec<Complex>) -> Self {
        Self {
            dc_gain,
            zeros,
            poles,
            description: String::new(),
        }
    }

    /// Create transfer function with description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Evaluate the transfer function at frequency f (Hz)
    pub fn evaluate(&self, freq_hz: f64) -> Complex {
        let s = Complex::jw(freq_hz);
        let mut result = Complex::new(self.dc_gain, 0.0);

        // Multiply by (s - z) for each zero
        for zero in &self.zeros {
            result = result * (s - *zero);
        }

        // Divide by (s - p) for each pole
        for pole in &self.poles {
            result = result / (s - *pole);
        }

        result
    }

    /// Get magnitude in dB at frequency
    pub fn magnitude_db(&self, freq_hz: f64) -> f64 {
        self.evaluate(freq_hz).magnitude_db()
    }

    /// Get phase in degrees at frequency
    pub fn phase_deg(&self, freq_hz: f64) -> f64 {
        self.evaluate(freq_hz).phase_deg()
    }

    /// Generate Bode plot data
    pub fn bode_data(&self, f_start: f64, f_end: f64, points: usize) -> BodeData {
        let log_start = f_start.log10();
        let log_end = f_end.log10();
        let log_step = (log_end - log_start) / (points - 1) as f64;

        let mut frequencies = Vec::with_capacity(points);
        let mut magnitudes = Vec::with_capacity(points);
        let mut phases = Vec::with_capacity(points);

        for i in 0..points {
            let freq = 10f64.powf(log_start + i as f64 * log_step);
            let h = self.evaluate(freq);
            frequencies.push(freq);
            magnitudes.push(h.magnitude_db());
            phases.push(h.phase_deg());
        }

        BodeData {
            frequencies,
            magnitudes_db: magnitudes,
            phases_deg: phases,
        }
    }

    /// Multiply two transfer functions (series connection)
    pub fn cascade(&self, other: &TransferFunction) -> TransferFunction {
        let mut zeros = self.zeros.clone();
        zeros.extend(other.zeros.iter().cloned());

        let mut poles = self.poles.clone();
        poles.extend(other.poles.iter().cloned());

        TransferFunction {
            dc_gain: self.dc_gain * other.dc_gain,
            zeros,
            poles,
            description: format!("{} × {}", self.description, other.description),
        }
    }

    /// Find crossover frequency (where magnitude = 0 dB)
    /// Uses bisection search
    pub fn find_crossover(&self, f_min: f64, f_max: f64) -> Option<f64> {
        let mag_min = self.magnitude_db(f_min);
        let mag_max = self.magnitude_db(f_max);

        // Check if crossover exists in range
        if mag_min * mag_max > 0.0 && mag_min.abs() > 0.1 && mag_max.abs() > 0.1 {
            return None;
        }

        // Bisection search
        let mut low = f_min;
        let mut high = f_max;

        for _ in 0..50 {
            let mid = (low * high).sqrt(); // Geometric mean for log scale
            let mag_mid = self.magnitude_db(mid);

            if mag_mid.abs() < 0.01 {
                return Some(mid);
            }

            if (mag_min > 0.0 && mag_mid > 0.0) || (mag_min < 0.0 && mag_mid < 0.0) {
                low = mid;
            } else {
                high = mid;
            }
        }

        Some((low * high).sqrt())
    }
}

/// Bode plot data
#[derive(Clone, Debug)]
pub struct BodeData {
    /// Frequency points (Hz)
    pub frequencies: Vec<f64>,
    /// Magnitude at each frequency (dB)
    pub magnitudes_db: Vec<f64>,
    /// Phase at each frequency (degrees)
    pub phases_deg: Vec<f64>,
}

// ============================================================================
// BUCK CONVERTER SMALL-SIGNAL MODEL
// ============================================================================

/// Small-signal model for buck converter in CCM
#[derive(Clone, Debug)]
pub struct BuckSmallSignal {
    /// Input voltage (V)
    pub vin: f64,
    /// Output voltage (V)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Inductor value (H)
    pub l: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Duty cycle
    pub duty: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl BuckSmallSignal {
    /// Create a new buck converter small-signal model
    pub fn new(vin: f64, vout: f64, iout: f64, l: f64, c: f64, esr: f64, fsw: f64) -> Self {
        let duty = vout / vin;
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };

        Self {
            vin,
            vout,
            iout,
            l,
            c,
            esr,
            fsw,
            duty,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s)
    ///
    /// Gvd(s) = Vin × (1 + s×Rc×C) / (1 + s×L/R + s²×L×C×(1 + Rc/R))
    ///
    /// Features:
    /// - ESR zero at f_esr = 1/(2π×Rc×C)
    /// - LC double pole at f_lc = 1/(2π×√(L×C))
    /// - Q factor depends on load
    pub fn control_to_output(&self) -> TransferFunction {
        let r = self.r_load;
        let rc = self.esr;

        // ESR zero: ωz = 1/(Rc×C)
        let wz_esr = 1.0 / (rc * self.c);

        // LC resonance: ω0 = 1/√(L×C×(1 + Rc/R))
        let w0 = 1.0 / (self.l * self.c * (1.0 + rc / r)).sqrt();

        // Q factor: Q = R×√(C×(1+Rc/R)/L) / (1 + R/Rc×(1+Rc/R))
        // Simplified: Q ≈ R × √(C/L) for small Rc
        let q = r * (self.c / self.l).sqrt() / (1.0 + rc / r);

        // Convert to complex poles
        let (p1, p2) = if q > 0.5 {
            // Underdamped - complex conjugate poles
            let sigma = w0 / (2.0 * q);
            let wd = w0 * (1.0 - 1.0 / (4.0 * q * q)).sqrt();
            (
                Complex::new(-sigma, wd),
                Complex::new(-sigma, -wd),
            )
        } else {
            // Overdamped - two real poles
            let factor = (1.0 - 4.0 * q * q).sqrt();
            (
                Complex::new(-w0 / (2.0 * q) * (1.0 + factor), 0.0),
                Complex::new(-w0 / (2.0 * q) * (1.0 - factor), 0.0),
            )
        };

        // DC gain = Vin
        TransferFunction {
            dc_gain: self.vin,
            zeros: vec![Complex::new(-wz_esr, 0.0)],
            poles: vec![p1, p2],
            description: "Buck Gvd(s)".to_string(),
        }
    }

    /// Line-to-output transfer function Gvg(s) (audio susceptibility)
    ///
    /// Gvg(s) = D × (1 + s×Rc×C) / (1 + s×L/R + s²×L×C×(1 + Rc/R))
    ///
    /// Same poles as control-to-output, but DC gain = D
    pub fn line_to_output(&self) -> TransferFunction {
        let mut tf = self.control_to_output();
        tf.dc_gain = self.duty;
        tf.description = "Buck Gvg(s)".to_string();
        tf
    }

    /// Output impedance Zout(s)
    ///
    /// Zout(s) = (s×L + R_dson) || (1/(s×C) + Rc) || R
    pub fn output_impedance(&self) -> TransferFunction {
        // Simplified: dominated by capacitor ESR at high frequency
        // Zout ≈ Rc + 1/(s×C) at high freq, limited by R at DC
        let wz = 1.0 / (self.esr * self.c);
        let wp = 1.0 / (self.r_load * self.c);

        TransferFunction {
            dc_gain: self.r_load,
            zeros: vec![Complex::new(-wz, 0.0)],
            poles: vec![Complex::new(-wp, 0.0)],
            description: "Buck Zout(s)".to_string(),
        }
    }

    /// Get characteristic frequencies
    pub fn characteristic_frequencies(&self) -> BuckFrequencies {
        let f_lc = 1.0 / (2.0 * PI * (self.l * self.c).sqrt());
        let f_esr = 1.0 / (2.0 * PI * self.esr * self.c);
        let q = self.r_load * (self.c / self.l).sqrt();

        BuckFrequencies {
            f_lc,
            f_esr,
            q_factor: q,
            fsw: self.fsw,
            max_crossover: self.fsw / 10.0, // Rule of thumb
        }
    }
}

/// Characteristic frequencies for buck converter
#[derive(Clone, Debug)]
pub struct BuckFrequencies {
    /// LC resonant frequency (Hz)
    pub f_lc: f64,
    /// ESR zero frequency (Hz)
    pub f_esr: f64,
    /// Q factor of LC filter
    pub q_factor: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Maximum recommended crossover frequency (Hz)
    pub max_crossover: f64,
}

// ============================================================================
// BOOST CONVERTER SMALL-SIGNAL MODEL
// ============================================================================

/// Small-signal model for boost converter in CCM
#[derive(Clone, Debug)]
pub struct BoostSmallSignal {
    /// Input voltage (V)
    pub vin: f64,
    /// Output voltage (V)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Inductor value (H)
    pub l: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Duty cycle
    pub duty: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl BoostSmallSignal {
    /// Create a new boost converter small-signal model
    pub fn new(vin: f64, vout: f64, iout: f64, l: f64, c: f64, esr: f64, fsw: f64) -> Self {
        // Boost: Vout = Vin / (1 - D), so D = 1 - Vin/Vout
        let duty = 1.0 - vin / vout;
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };

        Self {
            vin,
            vout,
            iout,
            l,
            c,
            esr,
            fsw,
            duty,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s)
    ///
    /// CRITICAL: Boost has a Right-Half-Plane (RHP) zero!
    ///
    /// Gvd(s) = (Vout/D') × (1 - s×L/(D'²×R)) × (1 + s×Rc×C) /
    ///          (1 + s×L/(D'²×R) + s²×L×C/D'²)
    ///
    /// where D' = 1 - D
    ///
    /// The RHP zero at f_rhp = D'²×R/(2π×L) limits achievable bandwidth!
    pub fn control_to_output(&self) -> TransferFunction {
        let d_prime = 1.0 - self.duty;
        let r = self.r_load;
        let rc = self.esr;

        // RHP zero (this is the problem!) - positive real part
        let w_rhp = d_prime * d_prime * r / self.l;

        // ESR zero (LHP - helpful)
        let w_esr = 1.0 / (rc * self.c);

        // Resonant frequency (shifted by D')
        let w0 = d_prime / (self.l * self.c).sqrt();

        // Q factor
        let q = d_prime * r * (self.c / self.l).sqrt();

        // DC gain
        let dc_gain = self.vout / d_prime;

        // Complex poles (usually underdamped)
        let (p1, p2) = if q > 0.5 {
            let sigma = w0 / (2.0 * q);
            let wd = w0 * (1.0 - 1.0 / (4.0 * q * q)).sqrt();
            (
                Complex::new(-sigma, wd),
                Complex::new(-sigma, -wd),
            )
        } else {
            let factor = (1.0 - 4.0 * q * q).sqrt();
            (
                Complex::new(-w0 / (2.0 * q) * (1.0 + factor), 0.0),
                Complex::new(-w0 / (2.0 * q) * (1.0 - factor), 0.0),
            )
        };

        TransferFunction {
            dc_gain,
            zeros: vec![
                Complex::new(w_rhp, 0.0),   // RHP zero (positive!)
                Complex::new(-w_esr, 0.0),  // ESR zero (negative)
            ],
            poles: vec![p1, p2],
            description: "Boost Gvd(s) - WARNING: RHP zero".to_string(),
        }
    }

    /// Get the RHP zero frequency - THIS LIMITS BANDWIDTH
    pub fn rhp_zero_freq(&self) -> f64 {
        let d_prime = 1.0 - self.duty;
        d_prime * d_prime * self.r_load / (2.0 * PI * self.l)
    }

    /// Maximum recommended crossover frequency
    /// Rule of thumb: fc < f_rhp / 5
    pub fn max_crossover(&self) -> f64 {
        self.rhp_zero_freq() / 5.0
    }

    /// Get characteristic frequencies
    pub fn characteristic_frequencies(&self) -> BoostFrequencies {
        let d_prime = 1.0 - self.duty;
        let f_lc = d_prime / (2.0 * PI * (self.l * self.c).sqrt());
        let f_esr = 1.0 / (2.0 * PI * self.esr * self.c);
        let f_rhp = self.rhp_zero_freq();
        let q = d_prime * self.r_load * (self.c / self.l).sqrt();

        BoostFrequencies {
            f_lc,
            f_esr,
            f_rhp,
            q_factor: q,
            fsw: self.fsw,
            max_crossover: f_rhp / 5.0,
        }
    }
}

/// Characteristic frequencies for boost converter
#[derive(Clone, Debug)]
pub struct BoostFrequencies {
    /// LC resonant frequency (Hz)
    pub f_lc: f64,
    /// ESR zero frequency (Hz)
    pub f_esr: f64,
    /// RHP zero frequency (Hz) - CRITICAL LIMITATION
    pub f_rhp: f64,
    /// Q factor of LC filter
    pub q_factor: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Maximum recommended crossover frequency (Hz)
    pub max_crossover: f64,
}

// ============================================================================
// BUCK-BOOST CONVERTER SMALL-SIGNAL MODEL
// ============================================================================

/// Small-signal model for inverting buck-boost converter in CCM
#[derive(Clone, Debug)]
pub struct BuckBoostSmallSignal {
    /// Input voltage (V)
    pub vin: f64,
    /// Output voltage magnitude (V, positive)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Inductor value (H)
    pub l: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Duty cycle
    pub duty: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl BuckBoostSmallSignal {
    /// Create a new buck-boost converter small-signal model
    pub fn new(vin: f64, vout: f64, iout: f64, l: f64, c: f64, esr: f64, fsw: f64) -> Self {
        // Buck-boost: Vout = Vin × D / (1 - D), so D = Vout / (Vin + Vout)
        let duty = vout / (vin + vout);
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };

        Self {
            vin,
            vout,
            iout,
            l,
            c,
            esr,
            fsw,
            duty,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s)
    ///
    /// Like boost, buck-boost has an RHP zero (even worse due to D factor)
    ///
    /// f_rhp = (1-D)² × R / (2π × L × D)
    pub fn control_to_output(&self) -> TransferFunction {
        let d = self.duty;
        let d_prime = 1.0 - d;
        let r = self.r_load;
        let rc = self.esr;

        // RHP zero
        let w_rhp = d_prime * d_prime * r / (self.l * d);

        // ESR zero
        let w_esr = 1.0 / (rc * self.c);

        // Resonant frequency
        let w0 = d_prime / (self.l * self.c).sqrt();

        // Q factor
        let q = d_prime * r * (self.c / self.l).sqrt();

        // DC gain = (Vin + Vout) / D' = Vout × (1 + Vin/Vout) / D'
        let dc_gain = (self.vin + self.vout) / d_prime;

        // Complex poles
        let (p1, p2) = if q > 0.5 {
            let sigma = w0 / (2.0 * q);
            let wd = w0 * (1.0 - 1.0 / (4.0 * q * q)).sqrt();
            (
                Complex::new(-sigma, wd),
                Complex::new(-sigma, -wd),
            )
        } else {
            let factor = (1.0 - 4.0 * q * q).sqrt();
            (
                Complex::new(-w0 / (2.0 * q) * (1.0 + factor), 0.0),
                Complex::new(-w0 / (2.0 * q) * (1.0 - factor), 0.0),
            )
        };

        TransferFunction {
            dc_gain,
            zeros: vec![
                Complex::new(w_rhp, 0.0),   // RHP zero
                Complex::new(-w_esr, 0.0),  // ESR zero
            ],
            poles: vec![p1, p2],
            description: "Buck-Boost Gvd(s) - WARNING: RHP zero".to_string(),
        }
    }

    /// Get the RHP zero frequency
    pub fn rhp_zero_freq(&self) -> f64 {
        let d_prime = 1.0 - self.duty;
        d_prime * d_prime * self.r_load / (2.0 * PI * self.l * self.duty)
    }

    /// Maximum recommended crossover frequency
    pub fn max_crossover(&self) -> f64 {
        self.rhp_zero_freq() / 5.0
    }
}

// ============================================================================
// FLYBACK CONVERTER SMALL-SIGNAL MODEL (CCM)
// ============================================================================

/// Small-signal model for flyback converter in CCM
#[derive(Clone, Debug)]
pub struct FlybackSmallSignal {
    /// Input voltage (V)
    pub vin: f64,
    /// Output voltage (V)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Magnetizing inductance (H)
    pub lm: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Turns ratio (Np/Ns)
    pub n: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Duty cycle
    pub duty: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl FlybackSmallSignal {
    /// Create a new flyback converter small-signal model
    pub fn new(
        vin: f64,
        vout: f64,
        iout: f64,
        lm: f64,
        c: f64,
        esr: f64,
        n: f64,
        fsw: f64,
    ) -> Self {
        // Flyback CCM: Vout = Vin × (D / (1-D)) × (1/n)
        // D = n × Vout / (Vin + n × Vout)
        let duty = n * vout / (vin + n * vout);
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };

        Self {
            vin,
            vout,
            iout,
            lm,
            c,
            esr,
            n,
            fsw,
            duty,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s) for CCM
    ///
    /// Similar to buck-boost, flyback in CCM has RHP zero
    /// f_rhp = (1-D)² × n² × R / (2π × Lm × D)
    pub fn control_to_output(&self) -> TransferFunction {
        let d = self.duty;
        let d_prime = 1.0 - d;
        let r = self.r_load;
        let rc = self.esr;

        // RHP zero (referred to secondary)
        let w_rhp = d_prime * d_prime * self.n * self.n * r / (self.lm * d);

        // ESR zero
        let w_esr = 1.0 / (rc * self.c);

        // Effective inductance referred to secondary
        let l_sec = self.lm / (self.n * self.n);

        // Resonant frequency
        let w0 = d_prime / (l_sec * self.c).sqrt();

        // Q factor
        let q = d_prime * r * (self.c / l_sec).sqrt();

        // DC gain
        let dc_gain = self.vout / d_prime;

        // Complex poles
        let (p1, p2) = if q > 0.5 {
            let sigma = w0 / (2.0 * q);
            let wd = w0 * (1.0 - 1.0 / (4.0 * q * q)).sqrt();
            (
                Complex::new(-sigma, wd),
                Complex::new(-sigma, -wd),
            )
        } else {
            let factor = (1.0 - 4.0 * q * q).sqrt();
            (
                Complex::new(-w0 / (2.0 * q) * (1.0 + factor), 0.0),
                Complex::new(-w0 / (2.0 * q) * (1.0 - factor), 0.0),
            )
        };

        TransferFunction {
            dc_gain,
            zeros: vec![
                Complex::new(w_rhp, 0.0),
                Complex::new(-w_esr, 0.0),
            ],
            poles: vec![p1, p2],
            description: "Flyback CCM Gvd(s) - WARNING: RHP zero".to_string(),
        }
    }

    /// Get the RHP zero frequency
    pub fn rhp_zero_freq(&self) -> f64 {
        let d_prime = 1.0 - self.duty;
        d_prime * d_prime * self.n * self.n * self.r_load
            / (2.0 * PI * self.lm * self.duty)
    }

    /// Maximum recommended crossover frequency
    pub fn max_crossover(&self) -> f64 {
        (self.rhp_zero_freq() / 5.0).min(self.fsw / 10.0)
    }
}

/// Small-signal model for flyback converter in DCM
///
/// DCM flyback does NOT have an RHP zero - much easier to compensate!
#[derive(Clone, Debug)]
pub struct FlybackDCMSmallSignal {
    /// Output voltage (V)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl FlybackDCMSmallSignal {
    pub fn new(vout: f64, iout: f64, c: f64, esr: f64, fsw: f64) -> Self {
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };
        Self {
            vout,
            iout,
            c,
            esr,
            fsw,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s) for DCM
    ///
    /// DCM flyback is a single-pole system with ESR zero
    /// Much simpler than CCM - no RHP zero!
    pub fn control_to_output(&self) -> TransferFunction {
        let r = self.r_load;
        let rc = self.esr;

        // Single pole from R-C
        let wp = 1.0 / (r * self.c);

        // ESR zero
        let wz = 1.0 / (rc * self.c);

        // DC gain depends on operating point (simplified)
        let dc_gain = self.vout * 2.0; // Approximate

        TransferFunction {
            dc_gain,
            zeros: vec![Complex::new(-wz, 0.0)],
            poles: vec![Complex::new(-wp, 0.0)],
            description: "Flyback DCM Gvd(s) - No RHP zero".to_string(),
        }
    }
}

// ============================================================================
// FORWARD CONVERTER SMALL-SIGNAL MODEL
// ============================================================================

/// Small-signal model for forward converter
///
/// Forward converter is like a buck with transformer isolation
/// Same pole-zero structure as buck (no RHP zero!)
#[derive(Clone, Debug)]
pub struct ForwardSmallSignal {
    /// Input voltage (V)
    pub vin: f64,
    /// Output voltage (V)
    pub vout: f64,
    /// Output current (A)
    pub iout: f64,
    /// Output inductor value (H)
    pub l: f64,
    /// Output capacitor (F)
    pub c: f64,
    /// Capacitor ESR (Ω)
    pub esr: f64,
    /// Turns ratio (Ns/Np)
    pub n: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Duty cycle
    pub duty: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
}

impl ForwardSmallSignal {
    /// Create a new forward converter small-signal model
    pub fn new(
        vin: f64,
        vout: f64,
        iout: f64,
        l: f64,
        c: f64,
        esr: f64,
        n: f64,
        fsw: f64,
    ) -> Self {
        // Forward: Vout = Vin × D × n
        let duty = vout / (vin * n);
        let r_load = if iout > 0.0 { vout / iout } else { 1e6 };

        Self {
            vin,
            vout,
            iout,
            l,
            c,
            esr,
            n,
            fsw,
            duty,
            r_load,
        }
    }

    /// Control-to-output transfer function Gvd(s)
    ///
    /// Same as buck - LC double pole and ESR zero
    /// No RHP zero (energy transfers directly, not stored)
    pub fn control_to_output(&self) -> TransferFunction {
        let r = self.r_load;
        let rc = self.esr;

        // ESR zero
        let wz_esr = 1.0 / (rc * self.c);

        // LC resonance
        let w0 = 1.0 / (self.l * self.c).sqrt();

        // Q factor
        let q = r * (self.c / self.l).sqrt();

        // DC gain = Vin × n
        let dc_gain = self.vin * self.n;

        // Complex poles
        let (p1, p2) = if q > 0.5 {
            let sigma = w0 / (2.0 * q);
            let wd = w0 * (1.0 - 1.0 / (4.0 * q * q)).sqrt();
            (
                Complex::new(-sigma, wd),
                Complex::new(-sigma, -wd),
            )
        } else {
            let factor = (1.0 - 4.0 * q * q).sqrt();
            (
                Complex::new(-w0 / (2.0 * q) * (1.0 + factor), 0.0),
                Complex::new(-w0 / (2.0 * q) * (1.0 - factor), 0.0),
            )
        };

        TransferFunction {
            dc_gain,
            zeros: vec![Complex::new(-wz_esr, 0.0)],
            poles: vec![p1, p2],
            description: "Forward Gvd(s)".to_string(),
        }
    }

    /// Get characteristic frequencies
    pub fn characteristic_frequencies(&self) -> BuckFrequencies {
        let f_lc = 1.0 / (2.0 * PI * (self.l * self.c).sqrt());
        let f_esr = 1.0 / (2.0 * PI * self.esr * self.c);
        let q = self.r_load * (self.c / self.l).sqrt();

        BuckFrequencies {
            f_lc,
            f_esr,
            q_factor: q,
            fsw: self.fsw,
            max_crossover: self.fsw / 10.0,
        }
    }
}

// ============================================================================
// CURRENT MODE CONTROL
// ============================================================================

/// Current mode control modifies the small-signal model
/// by adding a current feedback loop
#[derive(Clone, Debug)]
pub struct CurrentModeModel {
    /// Original voltage-mode plant
    pub plant: TransferFunction,
    /// Current sense gain (V/A)
    pub ri: f64,
    /// Slope compensation (V/s)
    pub slope_comp: f64,
    /// Duty cycle
    pub duty: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Inductor value (H)
    pub l: f64,
    /// Input voltage (V)
    pub vin: f64,
}

impl CurrentModeModel {
    /// Create current-mode model from voltage-mode buck
    pub fn from_buck(buck: &BuckSmallSignal, ri: f64, slope_comp: f64) -> Self {
        Self {
            plant: buck.control_to_output(),
            ri,
            slope_comp,
            duty: buck.duty,
            fsw: buck.fsw,
            l: buck.l,
            vin: buck.vin,
        }
    }

    /// Get the current loop gain
    ///
    /// Current mode control converts the LC double-pole to a single pole
    /// at much lower frequency, making compensation much easier
    pub fn control_to_output(&self) -> TransferFunction {
        // Current mode effectively removes one pole
        // The control-to-output becomes approximately:
        // Gvc(s) = Ri × Vout / (Vin × (1 + s × R × C))

        // Subharmonic oscillation pole at fsw/2
        let w_half = PI * self.fsw;

        // The plant is dominated by the output capacitor
        // Simplified model for Type II compensator design
        TransferFunction {
            dc_gain: self.plant.dc_gain * self.ri / self.vin,
            zeros: self.plant.zeros.clone(),
            poles: vec![
                self.plant.poles[0],
                Complex::new(-w_half, 0.0), // Sampling effect pole
            ],
            description: "Current-Mode Gvc(s)".to_string(),
        }
    }

    /// Calculate required slope compensation to prevent subharmonic oscillation
    ///
    /// Slope compensation required when D > 0.5
    /// Se >= Sn/2 where Sn = (Vin - Vout) / L
    pub fn required_slope_compensation(&self) -> f64 {
        if self.duty <= 0.5 {
            return 0.0;
        }

        // Natural downslope of inductor current
        let vout = self.vin * self.duty; // Approximate for buck
        let sn = (self.vin - vout) / self.l;

        // Se >= Sn/2 for stability, use Sn for margin
        sn * self.ri // Convert to voltage slope
    }
}

// ============================================================================
// PWM MODULATOR
// ============================================================================

/// PWM modulator gain
#[derive(Clone, Debug)]
pub struct PWMModulator {
    /// Peak-to-peak ramp voltage (V)
    pub vramp: f64,
    /// Modulator gain = 1/Vramp
    pub gain: f64,
}

impl PWMModulator {
    pub fn new(vramp: f64) -> Self {
        Self {
            vramp,
            gain: 1.0 / vramp,
        }
    }

    /// Get modulator transfer function (just a gain)
    pub fn transfer_function(&self) -> TransferFunction {
        TransferFunction {
            dc_gain: self.gain,
            zeros: vec![],
            poles: vec![],
            description: "PWM Modulator".to_string(),
        }
    }
}

// ============================================================================
// OPTOCOUPLER MODEL (FOR ISOLATED FEEDBACK)
// ============================================================================

/// Optocoupler transfer function for isolated converters
#[derive(Clone, Debug)]
pub struct OptocouplerModel {
    /// Current transfer ratio (CTR)
    pub ctr: f64,
    /// Dominant pole frequency (Hz)
    pub f_pole: f64,
    /// LED series resistance (Ω)
    pub r_led: f64,
}

impl OptocouplerModel {
    /// Create optocoupler model
    /// Typical values: CTR=1.0, f_pole=20kHz for PC817
    pub fn new(ctr: f64, f_pole: f64) -> Self {
        Self {
            ctr,
            f_pole,
            r_led: 1.0, // Typical LED dynamic resistance
        }
    }

    /// Common optocoupler: PC817
    pub fn pc817() -> Self {
        Self::new(1.0, 20e3)
    }

    /// Get transfer function
    pub fn transfer_function(&self) -> TransferFunction {
        let wp = 2.0 * PI * self.f_pole;

        TransferFunction {
            dc_gain: self.ctr,
            zeros: vec![],
            poles: vec![Complex::new(-wp, 0.0)],
            description: "Optocoupler".to_string(),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_operations() {
        let a = Complex::new(3.0, 4.0);
        assert!((a.magnitude() - 5.0).abs() < 1e-10);

        let b = Complex::new(1.0, 2.0);
        let c = a * b;
        assert!((c.re - (-5.0)).abs() < 1e-10);
        assert!((c.im - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_buck_small_signal() {
        let buck = BuckSmallSignal::new(
            12.0,   // Vin
            5.0,    // Vout
            2.0,    // Iout
            10e-6,  // L = 10µH
            100e-6, // C = 100µF
            0.02,   // ESR = 20mΩ
            500e3,  // fsw = 500kHz
        );

        let tf = buck.control_to_output();

        // DC gain should be Vin
        assert!((tf.dc_gain - 12.0).abs() < 0.1);

        // Should have 2 poles and 1 zero
        assert_eq!(tf.poles.len(), 2);
        assert_eq!(tf.zeros.len(), 1);

        let freqs = buck.characteristic_frequencies();
        // f_lc should be around 5kHz for 10µH, 100µF
        assert!(freqs.f_lc > 4e3 && freqs.f_lc < 6e3);
    }

    #[test]
    fn test_boost_rhp_zero() {
        let boost = BoostSmallSignal::new(
            5.0,    // Vin
            12.0,   // Vout
            0.5,    // Iout
            22e-6,  // L = 22µH
            100e-6, // C = 100µF
            0.05,   // ESR = 50mΩ
            300e3,  // fsw = 300kHz
        );

        let f_rhp = boost.rhp_zero_freq();

        // RHP zero should exist and be positive
        assert!(f_rhp > 0.0);

        // For this configuration, RHP zero should be in kHz range
        println!("Boost RHP zero: {:.1} kHz", f_rhp / 1e3);

        let tf = boost.control_to_output();
        // First zero should be RHP (positive real part)
        assert!(tf.zeros[0].re > 0.0);
    }

    #[test]
    fn test_flyback_ccm_vs_dcm() {
        // CCM flyback has RHP zero
        let flyback_ccm = FlybackSmallSignal::new(
            48.0,   // Vin
            5.0,    // Vout
            2.0,    // Iout
            200e-6, // Lm = 200µH
            470e-6, // C = 470µF
            0.02,   // ESR
            6.0,    // n = 6:1
            100e3,  // fsw
        );

        let tf_ccm = flyback_ccm.control_to_output();
        // CCM has RHP zero
        assert!(tf_ccm.zeros.iter().any(|z| z.re > 0.0));

        // DCM flyback has no RHP zero
        let flyback_dcm = FlybackDCMSmallSignal::new(
            5.0,    // Vout
            2.0,    // Iout
            470e-6, // C
            0.02,   // ESR
            100e3,  // fsw
        );

        let tf_dcm = flyback_dcm.control_to_output();
        // DCM has no RHP zero (all zeros should have negative real part)
        assert!(tf_dcm.zeros.iter().all(|z| z.re <= 0.0));
    }

    #[test]
    fn test_forward_no_rhp_zero() {
        let forward = ForwardSmallSignal::new(
            48.0,   // Vin
            5.0,    // Vout
            5.0,    // Iout
            10e-6,  // L = 10µH
            470e-6, // C = 470µF
            0.01,   // ESR
            0.2,    // n = 1:5 (step down)
            200e3,  // fsw
        );

        let tf = forward.control_to_output();

        // Forward converter (like buck) has no RHP zero
        assert!(tf.zeros.iter().all(|z| z.re <= 0.0));
    }

    #[test]
    fn test_bode_data() {
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let tf = buck.control_to_output();

        let bode = tf.bode_data(100.0, 1e6, 100);

        assert_eq!(bode.frequencies.len(), 100);
        assert_eq!(bode.magnitudes_db.len(), 100);
        assert_eq!(bode.phases_deg.len(), 100);

        // At low frequency, phase should be near 0
        assert!(bode.phases_deg[0].abs() < 45.0);

        // At high frequency (past LC pole), phase should roll off
        assert!(bode.phases_deg[99] < -90.0);
    }

    #[test]
    fn test_transfer_function_cascade() {
        let tf1 = TransferFunction::new(
            10.0,
            vec![Complex::new(-1000.0, 0.0)],
            vec![Complex::new(-100.0, 0.0)],
        );

        let tf2 = TransferFunction::new(
            2.0,
            vec![],
            vec![Complex::new(-10000.0, 0.0)],
        );

        let cascade = tf1.cascade(&tf2);

        assert!((cascade.dc_gain - 20.0).abs() < 0.01);
        assert_eq!(cascade.zeros.len(), 1);
        assert_eq!(cascade.poles.len(), 2);
    }

    #[test]
    fn test_crossover_finding() {
        let buck = BuckSmallSignal::new(12.0, 5.0, 2.0, 10e-6, 100e-6, 0.02, 500e3);
        let tf = buck.control_to_output();

        // The buck converter with these values should have a crossover
        if let Some(fc) = tf.find_crossover(100.0, 100e3) {
            println!("Crossover frequency: {:.1} kHz", fc / 1e3);

            // Verify magnitude is near 0 dB at crossover
            let mag_at_fc = tf.magnitude_db(fc);
            assert!(mag_at_fc.abs() < 1.0);
        }
    }

    #[test]
    fn test_current_mode_slope_compensation() {
        let buck = BuckSmallSignal::new(
            12.0, 3.0, // High duty cycle (D = 0.25, ok)
            2.0, 10e-6, 100e-6, 0.02, 500e3,
        );

        let cm = CurrentModeModel::from_buck(&buck, 0.1, 0.0);
        let slope = cm.required_slope_compensation();

        // D = 0.25 < 0.5, no slope comp needed
        assert_eq!(slope, 0.0);

        // Test with high duty cycle (D > 0.5)
        let buck_high_d = BuckSmallSignal::new(
            12.0, 8.0, // D = 0.67
            2.0, 10e-6, 100e-6, 0.02, 500e3,
        );

        let cm_high_d = CurrentModeModel::from_buck(&buck_high_d, 0.1, 0.0);
        let slope_high = cm_high_d.required_slope_compensation();

        // Should need slope compensation
        assert!(slope_high > 0.0);
    }
}
