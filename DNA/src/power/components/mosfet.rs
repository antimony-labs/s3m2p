//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mosfet.rs | DNA/src/power/components/mosfet.rs
//! PURPOSE: MOSFET database and loss models for power supply design
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides a database of common power MOSFETs with loss calculations for:
//! - Conduction losses (I²R with temperature derating)
//! - Switching losses (turn-on, turn-off, Miller charge)
//! - Gate drive losses
//! - Body diode reverse recovery losses

use serde::{Deserialize, Serialize};

/// MOSFET package type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MOSFETPackage {
    TO220,
    TO220F, // Isolated tab
    TO247,
    D2PAK,
    DPAK,
    SO8,
    PowerPAK, // 5x6mm or 3x3mm
    QFN,      // Various sizes
    LFPAK,    // NXP/Nexperia
}

impl MOSFETPackage {
    /// Typical junction-to-case thermal resistance (°C/W)
    pub fn typical_rth_jc(&self) -> f64 {
        match self {
            MOSFETPackage::TO247 => 0.3,
            MOSFETPackage::TO220 | MOSFETPackage::TO220F => 0.5,
            MOSFETPackage::D2PAK => 0.8,
            MOSFETPackage::DPAK => 1.5,
            MOSFETPackage::SO8 => 10.0,
            MOSFETPackage::PowerPAK => 3.0,
            MOSFETPackage::QFN => 5.0,
            MOSFETPackage::LFPAK => 2.0,
        }
    }

    /// Typical junction-to-ambient thermal resistance without heatsink (°C/W)
    pub fn typical_rth_ja(&self) -> f64 {
        match self {
            MOSFETPackage::TO247 => 40.0,
            MOSFETPackage::TO220 | MOSFETPackage::TO220F => 62.0,
            MOSFETPackage::D2PAK => 50.0,
            MOSFETPackage::DPAK => 100.0,
            MOSFETPackage::SO8 => 120.0,
            MOSFETPackage::PowerPAK => 80.0,
            MOSFETPackage::QFN => 100.0,
            MOSFETPackage::LFPAK => 70.0,
        }
    }
}

/// Full MOSFET specification from datasheet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MOSFETSpec {
    /// Part number
    pub part_number: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Maximum drain-source voltage (V)
    pub vds_max: f64,
    /// Continuous drain current at 25°C (A)
    pub id_continuous_25c: f64,
    /// Continuous drain current at 100°C (A)
    pub id_continuous_100c: f64,
    /// Pulsed drain current (A)
    pub id_pulsed: f64,
    /// On-resistance at 25°C, Vgs=10V (Ω)
    pub rds_on_25c: f64,
    /// On-resistance at 100°C (Ω) - typically 1.5-2x of 25°C value
    pub rds_on_100c: f64,
    /// Total gate charge at Vgs=10V (C)
    pub qg_total: f64,
    /// Gate-drain (Miller) charge (C)
    pub qgd: f64,
    /// Gate-source charge (C)
    pub qgs: f64,
    /// Body diode reverse recovery charge (C)
    pub qrr: f64,
    /// Output capacitance at Vds=25V (F)
    pub coss: f64,
    /// Threshold voltage (V)
    pub vgs_th: f64,
    /// Maximum gate-source voltage (V)
    pub vgs_max: f64,
    /// Package type
    pub package: MOSFETPackage,
    /// Junction-to-case thermal resistance (°C/W)
    pub rth_jc: f64,
    /// Junction-to-ambient thermal resistance (°C/W)
    pub rth_ja: f64,
    /// Maximum junction temperature (°C)
    pub tj_max: f64,
    /// Rise time at specified conditions (s)
    pub tr: f64,
    /// Fall time at specified conditions (s)
    pub tf: f64,
}

impl Default for MOSFETSpec {
    fn default() -> Self {
        Self {
            part_number: String::new(),
            manufacturer: String::new(),
            vds_max: 100.0,
            id_continuous_25c: 10.0,
            id_continuous_100c: 7.0,
            id_pulsed: 40.0,
            rds_on_25c: 0.050,
            rds_on_100c: 0.085,
            qg_total: 20e-9,
            qgd: 8e-9,
            qgs: 5e-9,
            qrr: 50e-9,
            coss: 100e-12,
            vgs_th: 2.5,
            vgs_max: 20.0,
            package: MOSFETPackage::TO220,
            rth_jc: 0.5,
            rth_ja: 62.0,
            tj_max: 150.0,
            tr: 20e-9,
            tf: 15e-9,
        }
    }
}

/// Operating point for loss calculations
#[derive(Clone, Debug)]
pub struct MOSFETOperatingPoint {
    /// RMS drain current (A)
    pub id_rms: f64,
    /// Peak drain current (A)
    pub id_peak: f64,
    /// Drain-source voltage during switching (V)
    pub vds_switch: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Gate drive voltage (V)
    pub vgs_drive: f64,
    /// Duty cycle (0-1)
    pub duty_cycle: f64,
    /// Estimated junction temperature (°C)
    pub tj_estimate: f64,
    /// Gate driver source/sink current (A)
    pub gate_driver_current: f64,
}

impl Default for MOSFETOperatingPoint {
    fn default() -> Self {
        Self {
            id_rms: 1.0,
            id_peak: 2.0,
            vds_switch: 50.0,
            fsw: 100e3,
            vgs_drive: 10.0,
            duty_cycle: 0.5,
            tj_estimate: 75.0,
            gate_driver_current: 1.0,
        }
    }
}

/// Detailed MOSFET loss breakdown
#[derive(Clone, Debug, Default)]
pub struct MOSFETLosses {
    /// Conduction loss (W)
    pub conduction: f64,
    /// Turn-on switching loss (W)
    pub switching_on: f64,
    /// Turn-off switching loss (W)
    pub switching_off: f64,
    /// Gate drive loss (W)
    pub gate_drive: f64,
    /// Body diode reverse recovery loss (W)
    pub reverse_recovery: f64,
    /// Output capacitance loss (W)
    pub coss_loss: f64,
    /// Total losses (W)
    pub total: f64,
}

impl MOSFETSpec {
    /// Calculate Rds_on at a given junction temperature
    pub fn rds_on_at_temp(&self, tj: f64) -> f64 {
        // Linear interpolation between 25°C and 100°C values
        // Rds_on increases with temperature
        if tj <= 25.0 {
            self.rds_on_25c
        } else if tj >= 100.0 {
            self.rds_on_100c
        } else {
            let ratio = (tj - 25.0) / 75.0;
            self.rds_on_25c + ratio * (self.rds_on_100c - self.rds_on_25c)
        }
    }

    /// Calculate all losses for a given operating point
    pub fn calculate_losses(&self, op: &MOSFETOperatingPoint) -> MOSFETLosses {
        let mut losses = MOSFETLosses::default();

        // Conduction loss: P = I_rms² × Rds_on(Tj)
        let rds_on = self.rds_on_at_temp(op.tj_estimate);
        losses.conduction = op.id_rms * op.id_rms * rds_on;

        // Switching losses - simplified linear model
        // P_sw = 0.5 × Vds × Id × (tr + tf) × fsw
        let t_switch = self.tr + self.tf;
        let p_sw_ideal = 0.5 * op.vds_switch * op.id_peak * t_switch * op.fsw;

        // Miller effect adds to switching time - gate charge based model
        // t_miller ≈ Qgd / Ig
        let t_miller = self.qgd / op.gate_driver_current.max(0.1);
        let p_miller = 0.5 * op.vds_switch * op.id_peak * t_miller * op.fsw;

        losses.switching_on = p_sw_ideal * 0.5 + p_miller * 0.5;
        losses.switching_off = p_sw_ideal * 0.5 + p_miller * 0.5;

        // Gate drive loss: P = Qg × Vgs × fsw
        losses.gate_drive = self.qg_total * op.vgs_drive * op.fsw;

        // Body diode reverse recovery loss (for hard-switched applications)
        // P_qrr = 0.5 × Qrr × Vds × fsw
        losses.reverse_recovery = 0.5 * self.qrr * op.vds_switch * op.fsw;

        // Output capacitance loss: P = 0.5 × Coss × Vds² × fsw
        losses.coss_loss = 0.5 * self.coss * op.vds_switch * op.vds_switch * op.fsw;

        // Total
        losses.total = losses.conduction
            + losses.switching_on
            + losses.switching_off
            + losses.gate_drive
            + losses.reverse_recovery
            + losses.coss_loss;

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
            Some(rth_hs) => self.rth_jc + 0.5 + rth_hs, // 0.5 for interface
            None => self.rth_ja,
        };
        ambient_temp + total_losses * rth
    }

    /// Check if MOSFET is suitable for given voltage and current
    pub fn is_suitable(&self, vds_max: f64, id_rms: f64, id_peak: f64) -> bool {
        // Use 80% derating for voltage, 70% for current
        vds_max <= self.vds_max * 0.8
            && id_rms <= self.id_continuous_100c * 0.7
            && id_peak <= self.id_pulsed * 0.8
    }
}

/// Selected MOSFET with design context
#[derive(Clone, Debug)]
pub struct SelectedMOSFET {
    pub spec: MOSFETSpec,
    pub operating_point: MOSFETOperatingPoint,
    pub losses: MOSFETLosses,
    pub junction_temp: f64,
    pub derating_voltage: f64, // Actual Vds / Vds_max
    pub derating_current: f64, // Actual Id_rms / Id_cont
}

// ============================================================================
// MOSFET DATABASE
// ============================================================================

/// Get the built-in MOSFET database
pub fn mosfet_database() -> Vec<MOSFETSpec> {
    vec![
        // Low voltage (30-60V) - for POL, battery applications
        MOSFETSpec {
            part_number: "IRLR7843".to_string(),
            manufacturer: "Infineon".to_string(),
            vds_max: 30.0,
            id_continuous_25c: 161.0,
            id_continuous_100c: 114.0,
            id_pulsed: 640.0,
            rds_on_25c: 0.0029,
            rds_on_100c: 0.0049,
            qg_total: 38e-9,
            qgd: 7.5e-9,
            qgs: 11e-9,
            qrr: 33e-9,
            coss: 850e-12,
            vgs_th: 2.35,
            vgs_max: 20.0,
            package: MOSFETPackage::DPAK,
            rth_jc: 1.5,
            rth_ja: 100.0,
            tj_max: 175.0,
            tr: 22e-9,
            tf: 13e-9,
        },
        MOSFETSpec {
            part_number: "CSD17570Q5B".to_string(),
            manufacturer: "Texas Instruments".to_string(),
            vds_max: 30.0,
            id_continuous_25c: 57.0,
            id_continuous_100c: 40.0,
            id_pulsed: 200.0,
            rds_on_25c: 0.0021,
            rds_on_100c: 0.0036,
            qg_total: 17e-9,
            qgd: 2.3e-9,
            qgs: 5.8e-9,
            qrr: 12e-9,
            coss: 320e-12,
            vgs_th: 1.7,
            vgs_max: 20.0,
            package: MOSFETPackage::QFN,
            rth_jc: 0.6,
            rth_ja: 40.0,
            tj_max: 150.0,
            tr: 5e-9,
            tf: 4e-9,
        },
        MOSFETSpec {
            part_number: "Si7336ADP".to_string(),
            manufacturer: "Vishay".to_string(),
            vds_max: 30.0,
            id_continuous_25c: 50.0,
            id_continuous_100c: 35.0,
            id_pulsed: 200.0,
            rds_on_25c: 0.0065,
            rds_on_100c: 0.011,
            qg_total: 12e-9,
            qgd: 3.5e-9,
            qgs: 3.2e-9,
            qrr: 8e-9,
            coss: 200e-12,
            vgs_th: 1.5,
            vgs_max: 20.0,
            package: MOSFETPackage::PowerPAK,
            rth_jc: 1.0,
            rth_ja: 50.0,
            tj_max: 150.0,
            tr: 8e-9,
            tf: 6e-9,
        },
        // Medium voltage (60-150V) - for 12V/24V/48V systems
        MOSFETSpec {
            part_number: "IPB072N15N3".to_string(),
            manufacturer: "Infineon".to_string(),
            vds_max: 150.0,
            id_continuous_25c: 63.0,
            id_continuous_100c: 45.0,
            id_pulsed: 252.0,
            rds_on_25c: 0.0072,
            rds_on_100c: 0.013,
            qg_total: 62e-9,
            qgd: 16e-9,
            qgs: 18e-9,
            qrr: 100e-9,
            coss: 180e-12,
            vgs_th: 3.0,
            vgs_max: 20.0,
            package: MOSFETPackage::D2PAK,
            rth_jc: 0.8,
            rth_ja: 50.0,
            tj_max: 175.0,
            tr: 18e-9,
            tf: 10e-9,
        },
        MOSFETSpec {
            part_number: "IRFB4110".to_string(),
            manufacturer: "Infineon".to_string(),
            vds_max: 100.0,
            id_continuous_25c: 180.0,
            id_continuous_100c: 127.0,
            id_pulsed: 720.0,
            rds_on_25c: 0.0037,
            rds_on_100c: 0.0063,
            qg_total: 150e-9,
            qgd: 36e-9,
            qgs: 35e-9,
            qrr: 200e-9,
            coss: 680e-12,
            vgs_th: 3.0,
            vgs_max: 20.0,
            package: MOSFETPackage::TO220,
            rth_jc: 0.45,
            rth_ja: 62.0,
            tj_max: 175.0,
            tr: 69e-9,
            tf: 45e-9,
        },
        // High voltage (400-650V) - for offline/AC-DC applications
        MOSFETSpec {
            part_number: "IPP60R099C6".to_string(),
            manufacturer: "Infineon".to_string(),
            vds_max: 600.0,
            id_continuous_25c: 24.0,
            id_continuous_100c: 15.0,
            id_pulsed: 72.0,
            rds_on_25c: 0.099,
            rds_on_100c: 0.19,
            qg_total: 56e-9,
            qgd: 19e-9,
            qgs: 14e-9,
            qrr: 500e-9,
            coss: 65e-12,
            vgs_th: 3.0,
            vgs_max: 20.0,
            package: MOSFETPackage::TO220,
            rth_jc: 0.5,
            rth_ja: 40.0,
            tj_max: 150.0,
            tr: 15e-9,
            tf: 8e-9,
        },
        MOSFETSpec {
            part_number: "STF13NM60N".to_string(),
            manufacturer: "STMicroelectronics".to_string(),
            vds_max: 600.0,
            id_continuous_25c: 11.0,
            id_continuous_100c: 7.0,
            id_pulsed: 44.0,
            rds_on_25c: 0.35,
            rds_on_100c: 0.68,
            qg_total: 28e-9,
            qgd: 13e-9,
            qgs: 5.5e-9,
            qrr: 280e-9,
            coss: 45e-12,
            vgs_th: 3.5,
            vgs_max: 20.0,
            package: MOSFETPackage::TO220F,
            rth_jc: 0.93,
            rth_ja: 62.5,
            tj_max: 150.0,
            tr: 12e-9,
            tf: 7e-9,
        },
        MOSFETSpec {
            part_number: "SPP20N60C3".to_string(),
            manufacturer: "Infineon".to_string(),
            vds_max: 600.0,
            id_continuous_25c: 20.7,
            id_continuous_100c: 13.0,
            id_pulsed: 62.0,
            rds_on_25c: 0.19,
            rds_on_100c: 0.38,
            qg_total: 87e-9,
            qgd: 32e-9,
            qgs: 18e-9,
            qrr: 1100e-9,
            coss: 125e-12,
            vgs_th: 3.0,
            vgs_max: 20.0,
            package: MOSFETPackage::TO220,
            rth_jc: 0.5,
            rth_ja: 62.0,
            tj_max: 150.0,
            tr: 22e-9,
            tf: 14e-9,
        },
        // SiC MOSFET for high-efficiency applications
        MOSFETSpec {
            part_number: "C3M0065090D".to_string(),
            manufacturer: "Wolfspeed".to_string(),
            vds_max: 900.0,
            id_continuous_25c: 36.0,
            id_continuous_100c: 23.0,
            id_pulsed: 63.0,
            rds_on_25c: 0.065,
            rds_on_100c: 0.080, // SiC has lower temp coefficient
            qg_total: 37e-9,
            qgd: 6.4e-9,
            qgs: 11e-9,
            qrr: 50e-9, // Very low for SiC
            coss: 54e-12,
            vgs_th: 2.5,
            vgs_max: 15.0, // Lower for SiC
            package: MOSFETPackage::TO247,
            rth_jc: 0.24,
            rth_ja: 40.0,
            tj_max: 175.0,
            tr: 12e-9,
            tf: 8e-9,
        },
    ]
}

/// Find suitable MOSFETs for given requirements
pub fn find_suitable_mosfets(
    vds_required: f64,
    id_rms: f64,
    id_peak: f64,
    preference: MOSFETPreference,
) -> Vec<&'static MOSFETSpec> {
    // Lazy static for database
    static DATABASE: std::sync::OnceLock<Vec<MOSFETSpec>> = std::sync::OnceLock::new();
    let db = DATABASE.get_or_init(mosfet_database);

    let mut suitable: Vec<&MOSFETSpec> = db
        .iter()
        .filter(|m| m.is_suitable(vds_required, id_rms, id_peak))
        .collect();

    // Sort by preference
    match preference {
        MOSFETPreference::LowRdsOn => {
            suitable.sort_by(|a, b| a.rds_on_25c.partial_cmp(&b.rds_on_25c).unwrap());
        }
        MOSFETPreference::LowQg => {
            suitable.sort_by(|a, b| a.qg_total.partial_cmp(&b.qg_total).unwrap());
        }
        MOSFETPreference::LowCost => {
            // Sort by package (TO220 cheaper than D2PAK cheaper than QFN)
            suitable.sort_by_key(|m| match m.package {
                MOSFETPackage::TO220 | MOSFETPackage::TO220F => 0,
                MOSFETPackage::DPAK | MOSFETPackage::D2PAK => 1,
                _ => 2,
            });
        }
        MOSFETPreference::LowLosses => {
            // FOM = Rds_on × Qg (lower is better)
            suitable.sort_by(|a, b| {
                let fom_a = a.rds_on_25c * a.qg_total;
                let fom_b = b.rds_on_25c * b.qg_total;
                fom_a.partial_cmp(&fom_b).unwrap()
            });
        }
    }

    suitable
}

/// MOSFET selection preference
#[derive(Clone, Copy, Debug)]
pub enum MOSFETPreference {
    /// Minimize Rds_on for low conduction loss
    LowRdsOn,
    /// Minimize gate charge for high-frequency switching
    LowQg,
    /// Prefer common/cheap packages
    LowCost,
    /// Minimize total losses (Rds_on × Qg figure of merit)
    LowLosses,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rds_on_temperature() {
        let mosfet = MOSFETSpec {
            rds_on_25c: 0.010,
            rds_on_100c: 0.018,
            ..Default::default()
        };

        assert!((mosfet.rds_on_at_temp(25.0) - 0.010).abs() < 0.001);
        assert!((mosfet.rds_on_at_temp(100.0) - 0.018).abs() < 0.001);
        assert!((mosfet.rds_on_at_temp(62.5) - 0.014).abs() < 0.001); // Midpoint
    }

    #[test]
    fn test_loss_calculation() {
        let mosfet = mosfet_database().into_iter().next().unwrap();
        let op = MOSFETOperatingPoint {
            id_rms: 5.0,
            id_peak: 10.0,
            vds_switch: 24.0,
            fsw: 100e3,
            vgs_drive: 10.0,
            duty_cycle: 0.5,
            tj_estimate: 75.0,
            gate_driver_current: 1.0,
        };

        let losses = mosfet.calculate_losses(&op);
        assert!(losses.conduction > 0.0);
        assert!(losses.switching_on > 0.0);
        assert!(losses.total > losses.conduction);
    }

    #[test]
    fn test_find_suitable_mosfets() {
        let suitable = find_suitable_mosfets(50.0, 5.0, 10.0, MOSFETPreference::LowLosses);
        assert!(!suitable.is_empty());
        // All should be rated above 50V × 0.8 = 62.5V
        for m in &suitable {
            assert!(m.vds_max >= 50.0);
        }
    }

    #[test]
    fn test_junction_temperature() {
        let mosfet = MOSFETSpec {
            rth_jc: 0.5,
            rth_ja: 62.0,
            ..Default::default()
        };

        // Without heatsink
        let tj = mosfet.junction_temperature(2.0, 25.0, None);
        assert!((tj - 149.0).abs() < 1.0); // 25 + 2×62

        // With 5°C/W heatsink
        let tj_hs = mosfet.junction_temperature(2.0, 25.0, Some(5.0));
        assert!((tj_hs - 37.0).abs() < 1.0); // 25 + 2×(0.5+0.5+5)
    }
}
