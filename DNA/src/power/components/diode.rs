//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: diode.rs | DNA/src/power/components/diode.rs
//! PURPOSE: Diode database and loss models for power supply design
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides a database of power diodes with loss calculations for:
//! - Conduction losses (Vf × If)
//! - Reverse recovery losses (Qrr based)
//! - Reverse leakage (typically negligible)
//!
//! Diode types covered:
//! - Schottky: Low Vf, no reverse recovery, limited voltage (<200V)
//! - Fast Recovery: Moderate Vf, fast trr for high-frequency switching
//! - Ultrafast: Very fast trr, higher Vf
//! - SiC Schottky: High voltage (600V+), zero reverse recovery, expensive

use serde::{Deserialize, Serialize};

/// Diode type classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiodeType {
    /// Standard silicon rectifier - slow, cheap, high voltage
    Standard,
    /// Schottky barrier - low Vf, no trr, limited voltage (<200V typically)
    Schottky,
    /// Fast recovery silicon - moderate trr (50-200ns)
    FastRecovery,
    /// Ultrafast silicon - very fast trr (<50ns)
    Ultrafast,
    /// Silicon carbide Schottky - high voltage, zero trr, expensive
    SiCSchottky,
}

impl DiodeType {
    /// Typical forward voltage drop at rated current
    pub fn typical_vf(&self) -> f64 {
        match self {
            DiodeType::Standard => 1.0,
            DiodeType::Schottky => 0.4,
            DiodeType::FastRecovery => 0.9,
            DiodeType::Ultrafast => 1.1,
            DiodeType::SiCSchottky => 1.5,
        }
    }

    /// Does this diode type have significant reverse recovery?
    pub fn has_reverse_recovery(&self) -> bool {
        match self {
            DiodeType::Standard => true,
            DiodeType::Schottky => false,  // Majority carrier device
            DiodeType::FastRecovery => true,
            DiodeType::Ultrafast => true,
            DiodeType::SiCSchottky => false,  // Majority carrier device
        }
    }
}

/// Diode package types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiodePackage {
    /// Axial leaded (DO-41, DO-201)
    Axial,
    /// DO-214 series (SMA, SMB, SMC)
    DO214,
    /// TO-220 series
    TO220,
    /// TO-247 series
    TO247,
    /// D2PAK (TO-263)
    D2PAK,
    /// DPAK (TO-252)
    DPAK,
    /// PowerDI series (5x6, 3x3)
    PowerDI,
}

impl DiodePackage {
    /// Typical junction-to-case thermal resistance (°C/W)
    pub fn typical_rth_jc(&self) -> f64 {
        match self {
            DiodePackage::Axial => 15.0,
            DiodePackage::DO214 => 8.0,
            DiodePackage::TO220 => 1.0,
            DiodePackage::TO247 => 0.5,
            DiodePackage::D2PAK => 1.5,
            DiodePackage::DPAK => 3.0,
            DiodePackage::PowerDI => 2.0,
        }
    }

    /// Typical junction-to-ambient thermal resistance (°C/W)
    pub fn typical_rth_ja(&self) -> f64 {
        match self {
            DiodePackage::Axial => 50.0,
            DiodePackage::DO214 => 65.0,
            DiodePackage::TO220 => 50.0,
            DiodePackage::TO247 => 40.0,
            DiodePackage::D2PAK => 60.0,
            DiodePackage::DPAK => 90.0,
            DiodePackage::PowerDI => 45.0,
        }
    }
}

/// Full diode specification from datasheet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiodeSpec {
    /// Part number
    pub part_number: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Diode type
    pub diode_type: DiodeType,
    /// Repetitive peak reverse voltage (V)
    pub vrrm: f64,
    /// DC blocking voltage (V)
    pub vr: f64,
    /// Average forward current (A)
    pub if_avg: f64,
    /// RMS forward current (A)
    pub if_rms: f64,
    /// Surge current rating, 8.3ms half-sine (A)
    pub ifsm: f64,
    /// Forward voltage at If_avg, 25°C (V)
    pub vf_typical: f64,
    /// Maximum forward voltage at If_avg, 25°C (V)
    pub vf_max: f64,
    /// Forward voltage temperature coefficient (mV/°C)
    pub vf_temp_coeff: f64,
    /// Reverse recovery time (s) - 0 for Schottky/SiC
    pub trr: f64,
    /// Reverse recovery charge (C) - 0 for Schottky/SiC
    pub qrr: f64,
    /// Junction capacitance at Vr=4V (F)
    pub cj: f64,
    /// Reverse leakage current at Vr, 25°C (A)
    pub ir_25c: f64,
    /// Reverse leakage current at Vr, 125°C (A)
    pub ir_125c: f64,
    /// Package type
    pub package: DiodePackage,
    /// Junction-to-case thermal resistance (°C/W)
    pub rth_jc: f64,
    /// Junction-to-ambient thermal resistance (°C/W)
    pub rth_ja: f64,
    /// Maximum junction temperature (°C)
    pub tj_max: f64,
}

impl Default for DiodeSpec {
    fn default() -> Self {
        Self {
            part_number: String::new(),
            manufacturer: String::new(),
            diode_type: DiodeType::Schottky,
            vrrm: 100.0,
            vr: 100.0,
            if_avg: 5.0,
            if_rms: 8.0,
            ifsm: 100.0,
            vf_typical: 0.55,
            vf_max: 0.70,
            vf_temp_coeff: -1.5,
            trr: 0.0,
            qrr: 0.0,
            cj: 200e-12,
            ir_25c: 1e-6,
            ir_125c: 100e-6,
            package: DiodePackage::TO220,
            rth_jc: 1.0,
            rth_ja: 50.0,
            tj_max: 150.0,
        }
    }
}

/// Operating point for diode loss calculations
#[derive(Clone, Debug)]
pub struct DiodeOperatingPoint {
    /// Average forward current (A)
    pub if_avg: f64,
    /// RMS forward current (A)
    pub if_rms: f64,
    /// Peak forward current (A)
    pub if_peak: f64,
    /// Reverse voltage during off state (V)
    pub vr_off: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// di/dt during turn-off (A/s)
    pub di_dt: f64,
    /// Estimated junction temperature (°C)
    pub tj_estimate: f64,
}

impl Default for DiodeOperatingPoint {
    fn default() -> Self {
        Self {
            if_avg: 1.0,
            if_rms: 1.5,
            if_peak: 3.0,
            vr_off: 50.0,
            fsw: 100e3,
            di_dt: 100e6, // 100 A/µs
            tj_estimate: 75.0,
        }
    }
}

/// Detailed diode loss breakdown
#[derive(Clone, Debug, Default)]
pub struct DiodeLosses {
    /// Conduction loss (W)
    pub conduction: f64,
    /// Reverse recovery loss (W)
    pub reverse_recovery: f64,
    /// Capacitance switching loss (W)
    pub capacitive: f64,
    /// Reverse leakage loss (W) - usually negligible
    pub leakage: f64,
    /// Total losses (W)
    pub total: f64,
}

impl DiodeSpec {
    /// Calculate forward voltage at temperature
    pub fn vf_at_temp(&self, tj: f64) -> f64 {
        // Vf decreases with temperature (negative temp coeff)
        let delta_t = tj - 25.0;
        self.vf_typical + (delta_t * self.vf_temp_coeff / 1000.0)
    }

    /// Calculate all losses for a given operating point
    pub fn calculate_losses(&self, op: &DiodeOperatingPoint) -> DiodeLosses {
        let mut losses = DiodeLosses::default();

        // Conduction loss: P = Vf × If_avg + Rd × If_rms²
        // Approximate dynamic resistance from Vf variation
        let vf = self.vf_at_temp(op.tj_estimate);
        losses.conduction = vf * op.if_avg;

        // Reverse recovery loss (only for bipolar diodes)
        // P_rr = 0.5 × Qrr × Vr × fsw
        // More accurate: P_rr = 0.25 × Qrr × (Vr × fsw + Irr × di/dt / fsw)
        if self.diode_type.has_reverse_recovery() {
            losses.reverse_recovery = 0.5 * self.qrr * op.vr_off * op.fsw;
        }

        // Capacitive switching loss (for Schottky/SiC)
        // P_cap = 0.5 × Cj × Vr² × fsw
        if !self.diode_type.has_reverse_recovery() {
            losses.capacitive = 0.5 * self.cj * op.vr_off * op.vr_off * op.fsw;
        }

        // Reverse leakage - interpolate between 25°C and 125°C
        let ir = if op.tj_estimate <= 25.0 {
            self.ir_25c
        } else if op.tj_estimate >= 125.0 {
            self.ir_125c
        } else {
            let ratio = (op.tj_estimate - 25.0) / 100.0;
            self.ir_25c * (self.ir_125c / self.ir_25c).powf(ratio)
        };
        losses.leakage = ir * op.vr_off;

        losses.total = losses.conduction + losses.reverse_recovery + losses.capacitive + losses.leakage;
        losses
    }

    /// Calculate junction temperature given losses and thermal conditions
    pub fn junction_temperature(
        &self,
        total_losses: f64,
        ambient_temp: f64,
        heatsink_rth: Option<f64>,
    ) -> f64 {
        let rth = match heatsink_rth {
            Some(rth_hs) => self.rth_jc + 0.3 + rth_hs, // 0.3 for interface
            None => self.rth_ja,
        };
        ambient_temp + total_losses * rth
    }

    /// Check if diode is suitable for given voltage and current
    pub fn is_suitable(&self, vr_max: f64, if_avg: f64, if_rms: f64) -> bool {
        // Use 80% derating for voltage, 70% for current
        vr_max <= self.vrrm * 0.8
            && if_avg <= self.if_avg * 0.7
            && if_rms <= self.if_rms * 0.7
    }

    /// Get efficiency advantage over a reference diode (Vf based)
    pub fn efficiency_vs_standard(&self, if_avg: f64) -> f64 {
        // Compare against 1V drop of standard diode
        let standard_loss = 1.0 * if_avg;
        let this_loss = self.vf_typical * if_avg;
        (standard_loss - this_loss) / standard_loss * 100.0
    }
}

/// Selected diode with design context
#[derive(Clone, Debug)]
pub struct SelectedDiode {
    pub spec: DiodeSpec,
    pub operating_point: DiodeOperatingPoint,
    pub losses: DiodeLosses,
    pub junction_temp: f64,
    pub derating_voltage: f64,  // Actual Vr / Vrrm
    pub derating_current: f64,  // Actual If_avg / If_avg_rated
}

// ============================================================================
// DIODE DATABASE
// ============================================================================

/// Get the built-in diode database
pub fn diode_database() -> Vec<DiodeSpec> {
    vec![
        // ====================================================================
        // SCHOTTKY DIODES (Low voltage, low Vf, no trr)
        // ====================================================================
        DiodeSpec {
            part_number: "SS34".to_string(),
            manufacturer: "Taiwan Semi".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 40.0,
            vr: 40.0,
            if_avg: 3.0,
            if_rms: 4.5,
            ifsm: 80.0,
            vf_typical: 0.50,
            vf_max: 0.55,
            vf_temp_coeff: -1.8,
            trr: 0.0,
            qrr: 0.0,
            cj: 350e-12,
            ir_25c: 0.5e-3,
            ir_125c: 10e-3,
            package: DiodePackage::DO214,
            rth_jc: 5.0,
            rth_ja: 65.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "MBRS340".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 40.0,
            vr: 40.0,
            if_avg: 3.0,
            if_rms: 5.0,
            ifsm: 80.0,
            vf_typical: 0.50,
            vf_max: 0.60,
            vf_temp_coeff: -1.5,
            trr: 0.0,
            qrr: 0.0,
            cj: 300e-12,
            ir_25c: 0.5e-3,
            ir_125c: 15e-3,
            package: DiodePackage::DO214,
            rth_jc: 4.0,
            rth_ja: 60.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "SBR10U45".to_string(),
            manufacturer: "Diodes Inc".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 45.0,
            vr: 45.0,
            if_avg: 10.0,
            if_rms: 15.0,
            ifsm: 200.0,
            vf_typical: 0.45,
            vf_max: 0.55,
            vf_temp_coeff: -1.6,
            trr: 0.0,
            qrr: 0.0,
            cj: 800e-12,
            ir_25c: 2e-3,
            ir_125c: 50e-3,
            package: DiodePackage::TO220,
            rth_jc: 1.2,
            rth_ja: 50.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "SS54".to_string(),
            manufacturer: "Taiwan Semi".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 40.0,
            vr: 40.0,
            if_avg: 5.0,
            if_rms: 7.5,
            ifsm: 120.0,
            vf_typical: 0.50,
            vf_max: 0.55,
            vf_temp_coeff: -1.8,
            trr: 0.0,
            qrr: 0.0,
            cj: 500e-12,
            ir_25c: 1e-3,
            ir_125c: 20e-3,
            package: DiodePackage::DO214,
            rth_jc: 3.0,
            rth_ja: 55.0,
            tj_max: 150.0,
        },
        // High voltage Schottky (100-200V range)
        DiodeSpec {
            part_number: "MBR10100".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 100.0,
            vr: 100.0,
            if_avg: 10.0,
            if_rms: 15.0,
            ifsm: 200.0,
            vf_typical: 0.75,
            vf_max: 0.85,
            vf_temp_coeff: -2.0,
            trr: 0.0,
            qrr: 0.0,
            cj: 300e-12,
            ir_25c: 0.5e-3,
            ir_125c: 50e-3,
            package: DiodePackage::TO220,
            rth_jc: 1.0,
            rth_ja: 50.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "STPS20150CT".to_string(),
            manufacturer: "STMicroelectronics".to_string(),
            diode_type: DiodeType::Schottky,
            vrrm: 150.0,
            vr: 150.0,
            if_avg: 20.0,  // Per leg, dual diode
            if_rms: 30.0,
            ifsm: 400.0,
            vf_typical: 0.82,
            vf_max: 0.95,
            vf_temp_coeff: -2.0,
            trr: 0.0,
            qrr: 0.0,
            cj: 250e-12,
            ir_25c: 0.3e-3,
            ir_125c: 30e-3,
            package: DiodePackage::TO220,
            rth_jc: 0.9,
            rth_ja: 45.0,
            tj_max: 175.0,
        },
        // ====================================================================
        // FAST RECOVERY DIODES (Medium trr, 50-200ns)
        // ====================================================================
        DiodeSpec {
            part_number: "MUR460".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::FastRecovery,
            vrrm: 600.0,
            vr: 600.0,
            if_avg: 4.0,
            if_rms: 6.0,
            ifsm: 100.0,
            vf_typical: 0.95,
            vf_max: 1.2,
            vf_temp_coeff: -2.0,
            trr: 75e-9,
            qrr: 300e-9,
            cj: 20e-12,
            ir_25c: 5e-6,
            ir_125c: 500e-6,
            package: DiodePackage::TO220,
            rth_jc: 2.0,
            rth_ja: 60.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "UF4007".to_string(),
            manufacturer: "Various".to_string(),
            diode_type: DiodeType::FastRecovery,
            vrrm: 1000.0,
            vr: 1000.0,
            if_avg: 1.0,
            if_rms: 1.5,
            ifsm: 30.0,
            vf_typical: 1.0,
            vf_max: 1.7,
            vf_temp_coeff: -2.0,
            trr: 75e-9,
            qrr: 150e-9,
            cj: 15e-12,
            ir_25c: 5e-6,
            ir_125c: 500e-6,
            package: DiodePackage::Axial,
            rth_jc: 15.0,
            rth_ja: 50.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "BYV29-500".to_string(),
            manufacturer: "Nexperia".to_string(),
            diode_type: DiodeType::FastRecovery,
            vrrm: 500.0,
            vr: 500.0,
            if_avg: 9.0,
            if_rms: 14.0,
            ifsm: 175.0,
            vf_typical: 0.90,
            vf_max: 1.05,
            vf_temp_coeff: -2.0,
            trr: 60e-9,
            qrr: 250e-9,
            cj: 25e-12,
            ir_25c: 10e-6,
            ir_125c: 1e-3,
            package: DiodePackage::TO220,
            rth_jc: 1.4,
            rth_ja: 50.0,
            tj_max: 175.0,
        },
        // ====================================================================
        // ULTRAFAST DIODES (Very fast trr, <50ns)
        // ====================================================================
        DiodeSpec {
            part_number: "ES2D".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::Ultrafast,
            vrrm: 200.0,
            vr: 200.0,
            if_avg: 2.0,
            if_rms: 3.0,
            ifsm: 50.0,
            vf_typical: 1.25,
            vf_max: 1.4,
            vf_temp_coeff: -2.0,
            trr: 35e-9,
            qrr: 80e-9,
            cj: 20e-12,
            ir_25c: 5e-6,
            ir_125c: 200e-6,
            package: DiodePackage::DO214,
            rth_jc: 6.0,
            rth_ja: 70.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "MURS320".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::Ultrafast,
            vrrm: 200.0,
            vr: 200.0,
            if_avg: 3.0,
            if_rms: 4.5,
            ifsm: 75.0,
            vf_typical: 1.1,
            vf_max: 1.25,
            vf_temp_coeff: -2.0,
            trr: 35e-9,
            qrr: 50e-9,
            cj: 25e-12,
            ir_25c: 5e-6,
            ir_125c: 300e-6,
            package: DiodePackage::DO214,
            rth_jc: 4.0,
            rth_ja: 60.0,
            tj_max: 150.0,
        },
        DiodeSpec {
            part_number: "RHRP8120".to_string(),
            manufacturer: "ON Semi".to_string(),
            diode_type: DiodeType::Ultrafast,
            vrrm: 1200.0,
            vr: 1200.0,
            if_avg: 8.0,
            if_rms: 12.0,
            ifsm: 100.0,
            vf_typical: 2.1,
            vf_max: 2.5,
            vf_temp_coeff: -2.0,
            trr: 45e-9,
            qrr: 270e-9,
            cj: 15e-12,
            ir_25c: 10e-6,
            ir_125c: 1e-3,
            package: DiodePackage::TO220,
            rth_jc: 1.5,
            rth_ja: 50.0,
            tj_max: 150.0,
        },
        // ====================================================================
        // SiC SCHOTTKY DIODES (High voltage, zero trr, premium)
        // ====================================================================
        DiodeSpec {
            part_number: "C3D10060A".to_string(),
            manufacturer: "Wolfspeed".to_string(),
            diode_type: DiodeType::SiCSchottky,
            vrrm: 600.0,
            vr: 600.0,
            if_avg: 10.0,
            if_rms: 20.0,
            ifsm: 78.0,
            vf_typical: 1.5,
            vf_max: 1.8,
            vf_temp_coeff: 0.4,  // SiC has positive temp coeff!
            trr: 0.0,
            qrr: 0.0,
            cj: 35e-12,
            ir_25c: 10e-6,
            ir_125c: 100e-6,
            package: DiodePackage::TO220,
            rth_jc: 0.85,
            rth_ja: 40.0,
            tj_max: 175.0,
        },
        DiodeSpec {
            part_number: "C4D10120A".to_string(),
            manufacturer: "Wolfspeed".to_string(),
            diode_type: DiodeType::SiCSchottky,
            vrrm: 1200.0,
            vr: 1200.0,
            if_avg: 10.0,
            if_rms: 20.0,
            ifsm: 50.0,
            vf_typical: 1.6,
            vf_max: 1.9,
            vf_temp_coeff: 0.5,
            trr: 0.0,
            qrr: 0.0,
            cj: 25e-12,
            ir_25c: 5e-6,
            ir_125c: 50e-6,
            package: DiodePackage::TO220,
            rth_jc: 0.9,
            rth_ja: 40.0,
            tj_max: 175.0,
        },
        DiodeSpec {
            part_number: "STPSC6H12".to_string(),
            manufacturer: "STMicroelectronics".to_string(),
            diode_type: DiodeType::SiCSchottky,
            vrrm: 1200.0,
            vr: 1200.0,
            if_avg: 6.0,
            if_rms: 12.0,
            ifsm: 37.0,
            vf_typical: 1.55,
            vf_max: 1.8,
            vf_temp_coeff: 0.5,
            trr: 0.0,
            qrr: 0.0,
            cj: 20e-12,
            ir_25c: 5e-6,
            ir_125c: 40e-6,
            package: DiodePackage::TO220,
            rth_jc: 1.2,
            rth_ja: 45.0,
            tj_max: 175.0,
        },
        DiodeSpec {
            part_number: "IDH04G65C6".to_string(),
            manufacturer: "Infineon".to_string(),
            diode_type: DiodeType::SiCSchottky,
            vrrm: 650.0,
            vr: 650.0,
            if_avg: 4.0,
            if_rms: 8.0,
            ifsm: 15.0,
            vf_typical: 1.5,
            vf_max: 1.7,
            vf_temp_coeff: 0.4,
            trr: 0.0,
            qrr: 0.0,
            cj: 12e-12,
            ir_25c: 1e-6,
            ir_125c: 10e-6,
            package: DiodePackage::D2PAK,
            rth_jc: 1.5,
            rth_ja: 50.0,
            tj_max: 175.0,
        },
    ]
}

/// Find suitable diodes for given requirements
pub fn find_suitable_diodes(
    vr_required: f64,
    if_avg: f64,
    if_rms: f64,
    preference: DiodePreference,
) -> Vec<&'static DiodeSpec> {
    // Lazy static for database
    static DATABASE: std::sync::OnceLock<Vec<DiodeSpec>> = std::sync::OnceLock::new();
    let db = DATABASE.get_or_init(diode_database);

    let mut suitable: Vec<&DiodeSpec> = db
        .iter()
        .filter(|d| d.is_suitable(vr_required, if_avg, if_rms))
        .collect();

    // Sort by preference
    match preference {
        DiodePreference::LowVf => {
            suitable.sort_by(|a, b| a.vf_typical.partial_cmp(&b.vf_typical).unwrap());
        }
        DiodePreference::LowQrr => {
            suitable.sort_by(|a, b| a.qrr.partial_cmp(&b.qrr).unwrap());
        }
        DiodePreference::HighSpeed => {
            // Prefer SiC and Schottky first (zero trr), then ultrafast
            suitable.sort_by(|a, b| {
                let score_a = match a.diode_type {
                    DiodeType::SiCSchottky | DiodeType::Schottky => 0,
                    DiodeType::Ultrafast => 1,
                    DiodeType::FastRecovery => 2,
                    DiodeType::Standard => 3,
                };
                let score_b = match b.diode_type {
                    DiodeType::SiCSchottky | DiodeType::Schottky => 0,
                    DiodeType::Ultrafast => 1,
                    DiodeType::FastRecovery => 2,
                    DiodeType::Standard => 3,
                };
                score_a.cmp(&score_b)
            });
        }
        DiodePreference::LowCost => {
            // Standard > Schottky > Fast > Ultrafast > SiC
            suitable.sort_by_key(|d| match d.diode_type {
                DiodeType::Standard => 0,
                DiodeType::Schottky => 1,
                DiodeType::FastRecovery => 2,
                DiodeType::Ultrafast => 3,
                DiodeType::SiCSchottky => 4,
            });
        }
        DiodePreference::LowLosses => {
            // Sort by (Vf + equivalent Qrr loss) at typical conditions
            // Rough approximation: Qrr contributes ~fsw×Vr×Qrr/2
            let fsw = 100e3;
            let vr = vr_required;
            suitable.sort_by(|a, b| {
                let loss_a = a.vf_typical * if_avg + 0.5 * a.qrr * vr * fsw;
                let loss_b = b.vf_typical * if_avg + 0.5 * b.qrr * vr * fsw;
                loss_a.partial_cmp(&loss_b).unwrap()
            });
        }
    }

    suitable
}

/// Diode selection preference
#[derive(Clone, Copy, Debug)]
pub enum DiodePreference {
    /// Minimize forward voltage drop
    LowVf,
    /// Minimize reverse recovery charge
    LowQrr,
    /// Prefer fast diodes (Schottky, SiC, Ultrafast)
    HighSpeed,
    /// Prefer lower cost options
    LowCost,
    /// Minimize total losses
    LowLosses,
}

/// Recommend diode type for application
pub fn recommend_diode_type(
    vr_max: f64,
    fsw: f64,
    efficiency_priority: bool,
) -> DiodeType {
    if vr_max <= 40.0 {
        // Low voltage - Schottky is ideal
        DiodeType::Schottky
    } else if vr_max <= 200.0 {
        if fsw > 200e3 || efficiency_priority {
            // High frequency or efficiency critical - Schottky if available
            DiodeType::Schottky
        } else {
            DiodeType::FastRecovery
        }
    } else if vr_max <= 600.0 {
        if fsw > 100e3 || efficiency_priority {
            // High frequency - SiC eliminates reverse recovery losses
            DiodeType::SiCSchottky
        } else {
            DiodeType::Ultrafast
        }
    } else {
        // >600V - SiC or Ultrafast
        if efficiency_priority {
            DiodeType::SiCSchottky
        } else {
            DiodeType::Ultrafast
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
    fn test_vf_temperature() {
        let diode = DiodeSpec {
            vf_typical: 0.50,
            vf_temp_coeff: -2.0,  // mV/°C
            ..Default::default()
        };

        // At 25°C - should be typical
        assert!((diode.vf_at_temp(25.0) - 0.50).abs() < 0.001);

        // At 125°C - should be lower (negative temp coeff)
        // ΔVf = -2.0 mV/°C × 100°C = -200mV = -0.2V
        assert!((diode.vf_at_temp(125.0) - 0.30).abs() < 0.001);
    }

    #[test]
    fn test_loss_calculation_schottky() {
        let schottky = diode_database()
            .into_iter()
            .find(|d| d.diode_type == DiodeType::Schottky)
            .unwrap();

        let op = DiodeOperatingPoint {
            if_avg: 2.0,
            if_rms: 3.0,
            if_peak: 5.0,
            vr_off: 30.0,
            fsw: 100e3,
            di_dt: 100e6,
            tj_estimate: 75.0,
        };

        let losses = schottky.calculate_losses(&op);

        // Schottky should have zero reverse recovery
        assert!(losses.reverse_recovery < 0.001);
        // But should have capacitive loss
        assert!(losses.capacitive > 0.0);
        // Conduction loss should be dominant
        assert!(losses.conduction > losses.capacitive);
    }

    #[test]
    fn test_loss_calculation_fast_recovery() {
        let fast = diode_database()
            .into_iter()
            .find(|d| d.diode_type == DiodeType::FastRecovery)
            .unwrap();

        let op = DiodeOperatingPoint {
            if_avg: 2.0,
            if_rms: 3.0,
            if_peak: 5.0,
            vr_off: 400.0,
            fsw: 100e3,
            di_dt: 100e6,
            tj_estimate: 75.0,
        };

        let losses = fast.calculate_losses(&op);

        // Fast recovery should have reverse recovery loss
        assert!(losses.reverse_recovery > 0.0);
        // No capacitive loss for bipolar diodes
        assert!(losses.capacitive < 0.001);
    }

    #[test]
    fn test_find_suitable_diodes() {
        let suitable = find_suitable_diodes(50.0, 2.0, 3.0, DiodePreference::LowVf);
        assert!(!suitable.is_empty());

        // All should be rated above 50V × 0.8 = 62.5V
        for d in &suitable {
            assert!(d.vrrm >= 50.0);
        }
    }

    #[test]
    fn test_recommend_diode_type() {
        // Low voltage → Schottky
        assert_eq!(recommend_diode_type(30.0, 100e3, false), DiodeType::Schottky);

        // High voltage, high frequency → SiC
        assert_eq!(recommend_diode_type(400.0, 200e3, false), DiodeType::SiCSchottky);

        // High voltage, efficiency priority → SiC
        assert_eq!(recommend_diode_type(400.0, 50e3, true), DiodeType::SiCSchottky);

        // High voltage, low frequency, cost priority → Ultrafast
        assert_eq!(recommend_diode_type(400.0, 50e3, false), DiodeType::Ultrafast);
    }

    #[test]
    fn test_sic_positive_temp_coeff() {
        let sic = diode_database()
            .into_iter()
            .find(|d| d.diode_type == DiodeType::SiCSchottky)
            .unwrap();

        // SiC has positive temp coefficient (Vf increases with temperature)
        assert!(sic.vf_temp_coeff > 0.0);

        // Verify Vf increases at higher temperature
        assert!(sic.vf_at_temp(125.0) > sic.vf_at_temp(25.0));
    }

    #[test]
    fn test_junction_temperature() {
        let diode = DiodeSpec {
            rth_jc: 1.0,
            rth_ja: 50.0,
            ..Default::default()
        };

        // Without heatsink
        let tj = diode.junction_temperature(2.0, 25.0, None);
        assert!((tj - 125.0).abs() < 1.0); // 25 + 2×50

        // With 5°C/W heatsink
        let tj_hs = diode.junction_temperature(2.0, 25.0, Some(5.0));
        assert!((tj_hs - 37.6).abs() < 1.0); // 25 + 2×(1.0+0.3+5.0)
    }
}
