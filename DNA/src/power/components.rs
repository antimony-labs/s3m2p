//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: components.rs | DNA/src/power/components.rs
//! PURPOSE: E-series component selection and formatting for power supplies
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

// ============================================================================
// E-SERIES STANDARD VALUES
// ============================================================================

/// E6 series (20% tolerance) - 6 values per decade
pub const E6: [f64; 6] = [1.0, 1.5, 2.2, 3.3, 4.7, 6.8];

/// E12 series (10% tolerance) - 12 values per decade
pub const E12: [f64; 12] = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];

/// E24 series (5% tolerance) - 24 values per decade
pub const E24: [f64; 24] = [
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6,
    6.2, 6.8, 7.5, 8.2, 9.1,
];

/// E96 series (1% tolerance) - 96 values per decade
pub const E96: [f64; 96] = [
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43,
    1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10,
    2.15, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80, 2.87, 2.94, 3.01, 3.09,
    3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53,
    4.64, 4.75, 4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65,
    6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];

/// Standard power inductor values (uH) - commonly available
pub const STANDARD_INDUCTORS_UH: [f64; 24] = [
    1.0, 1.5, 2.2, 3.3, 4.7, 6.8, 10.0, 15.0, 22.0, 33.0, 47.0, 68.0, 100.0, 150.0, 220.0, 330.0,
    470.0, 680.0, 1000.0, 1500.0, 2200.0, 3300.0, 4700.0, 6800.0,
];

/// Standard electrolytic capacitor values (uF) - commonly available
pub const STANDARD_ELECTROLYTICS_UF: [f64; 20] = [
    1.0, 2.2, 4.7, 10.0, 22.0, 33.0, 47.0, 100.0, 220.0, 330.0, 470.0, 680.0, 1000.0, 1500.0,
    2200.0, 3300.0, 4700.0, 6800.0, 10000.0, 22000.0,
];

/// Standard ceramic capacitor values (uF) - commonly available for MLCC
pub const STANDARD_CERAMICS_UF: [f64; 16] = [
    0.1, 0.22, 0.47, 1.0, 2.2, 4.7, 10.0, 22.0, 47.0, 100.0, 220.0, 470.0, 1000.0, 2200.0, 4700.0,
    10000.0,
];

// ============================================================================
// E-SERIES SELECTION FUNCTIONS
// ============================================================================

/// Find the nearest value in an E-series
fn nearest_in_series(target: f64, series: &[f64]) -> f64 {
    if target <= 0.0 {
        return series[0];
    }

    // Determine the decade multiplier
    let decade = 10.0_f64.powf(target.log10().floor());
    let normalized = target / decade;

    // Find nearest value in series
    let mut closest = series[0];
    let mut min_diff = (normalized - series[0]).abs();

    for &value in series {
        let diff = (normalized - value).abs();
        if diff < min_diff {
            min_diff = diff;
            closest = value;
        }
    }

    // Also check the first value of the next decade
    let next_decade_first = series[0] * 10.0;
    if (normalized - next_decade_first).abs() < min_diff {
        closest = next_decade_first;
    }

    closest * decade
}

/// Find the nearest E6 value (20% tolerance)
pub fn nearest_e6(target: f64) -> f64 {
    nearest_in_series(target, &E6)
}

/// Find the nearest E12 value (10% tolerance)
pub fn nearest_e12(target: f64) -> f64 {
    nearest_in_series(target, &E12)
}

/// Find the nearest E24 value (5% tolerance)
pub fn nearest_e24(target: f64) -> f64 {
    nearest_in_series(target, &E24)
}

/// Find the nearest E96 value (1% tolerance)
pub fn nearest_e96(target: f64) -> f64 {
    nearest_in_series(target, &E96)
}

/// Find the nearest standard power inductor value (in Henries)
/// Input: value in Henries
/// Output: value in Henries
pub fn nearest_inductor(target_h: f64) -> f64 {
    let target_uh = target_h * 1e6;
    if target_uh <= 0.0 {
        return STANDARD_INDUCTORS_UH[0] * 1e-6;
    }

    let mut closest = STANDARD_INDUCTORS_UH[0];
    let mut min_diff = (target_uh - STANDARD_INDUCTORS_UH[0]).abs();

    for &value in &STANDARD_INDUCTORS_UH {
        let diff = (target_uh - value).abs();
        if diff < min_diff {
            min_diff = diff;
            closest = value;
        }
    }

    closest * 1e-6
}

/// Find the nearest standard capacitor value
/// Input: value in Farads
/// Output: value in Farads
/// Prefers ceramic for small values, electrolytic for large
pub fn nearest_capacitor(target_f: f64) -> f64 {
    let target_uf = target_f * 1e6;

    // For small values (< 10uF), use ceramic
    if target_uf < 10.0 {
        let mut closest = STANDARD_CERAMICS_UF[0];
        let mut min_diff = (target_uf - STANDARD_CERAMICS_UF[0]).abs();

        for &value in &STANDARD_CERAMICS_UF {
            if value > 100.0 {
                break;
            } // Limit ceramic search
            let diff = (target_uf - value).abs();
            if diff < min_diff {
                min_diff = diff;
                closest = value;
            }
        }
        return closest * 1e-6;
    }

    // For larger values, use electrolytic
    let mut closest = STANDARD_ELECTROLYTICS_UF[0];
    let mut min_diff = (target_uf - STANDARD_ELECTROLYTICS_UF[0]).abs();

    for &value in &STANDARD_ELECTROLYTICS_UF {
        let diff = (target_uf - value).abs();
        if diff < min_diff {
            min_diff = diff;
            closest = value;
        }
    }

    closest * 1e-6
}

/// Select next higher standard value (conservative design)
pub fn next_higher_inductor(target_h: f64) -> f64 {
    let target_uh = target_h * 1e6;

    for &value in &STANDARD_INDUCTORS_UH {
        if value >= target_uh {
            return value * 1e-6;
        }
    }

    // Return the largest if nothing is bigger
    STANDARD_INDUCTORS_UH[STANDARD_INDUCTORS_UH.len() - 1] * 1e-6
}

/// Select next higher standard capacitor value (conservative design)
pub fn next_higher_capacitor(target_f: f64) -> f64 {
    let target_uf = target_f * 1e6;

    // Check ceramics first for small values
    if target_uf < 100.0 {
        for &value in &STANDARD_CERAMICS_UF {
            if value >= target_uf {
                return value * 1e-6;
            }
        }
    }

    // Then check electrolytics
    for &value in &STANDARD_ELECTROLYTICS_UF {
        if value >= target_uf {
            return value * 1e-6;
        }
    }

    STANDARD_ELECTROLYTICS_UF[STANDARD_ELECTROLYTICS_UF.len() - 1] * 1e-6
}

// ============================================================================
// FORMATTING FUNCTIONS
// ============================================================================

/// Format inductance value with appropriate unit prefix
pub fn format_inductance(henries: f64) -> String {
    if henries >= 1.0 {
        format!("{:.2} H", henries)
    } else if henries >= 1e-3 {
        format!("{:.2} mH", henries * 1e3)
    } else if henries >= 1e-6 {
        format!("{:.2} uH", henries * 1e6)
    } else {
        format!("{:.2} nH", henries * 1e9)
    }
}

/// Format capacitance value with appropriate unit prefix
pub fn format_capacitance(farads: f64) -> String {
    if farads >= 1e-3 {
        format!("{:.2} mF", farads * 1e3)
    } else if farads >= 1e-6 {
        format!("{:.2} uF", farads * 1e6)
    } else if farads >= 1e-9 {
        format!("{:.2} nF", farads * 1e9)
    } else if farads >= 1e-12 {
        format!("{:.2} pF", farads * 1e12)
    } else {
        format!("{:.2} fF", farads * 1e15)
    }
}

/// Format resistance value with appropriate unit prefix
pub fn format_resistance(ohms: f64) -> String {
    if ohms >= 1e6 {
        format!("{:.2} MOhm", ohms / 1e6)
    } else if ohms >= 1e3 {
        format!("{:.2} kOhm", ohms / 1e3)
    } else if ohms >= 1.0 {
        format!("{:.2} Ohm", ohms)
    } else {
        format!("{:.2} mOhm", ohms * 1e3)
    }
}

/// Format current value with appropriate unit prefix
pub fn format_current(amps: f64) -> String {
    if amps >= 1.0 {
        format!("{:.2} A", amps)
    } else if amps >= 1e-3 {
        format!("{:.2} mA", amps * 1e3)
    } else if amps >= 1e-6 {
        format!("{:.2} uA", amps * 1e6)
    } else {
        format!("{:.2} nA", amps * 1e9)
    }
}

/// Format voltage value with appropriate unit prefix
pub fn format_voltage(volts: f64) -> String {
    if volts >= 1.0 {
        format!("{:.2} V", volts)
    } else if volts >= 1e-3 {
        format!("{:.2} mV", volts * 1e3)
    } else {
        format!("{:.2} uV", volts * 1e6)
    }
}

/// Format frequency value with appropriate unit prefix
pub fn format_frequency(hz: f64) -> String {
    if hz >= 1e9 {
        format!("{:.2} GHz", hz / 1e9)
    } else if hz >= 1e6 {
        format!("{:.2} MHz", hz / 1e6)
    } else if hz >= 1e3 {
        format!("{:.2} kHz", hz / 1e3)
    } else {
        format!("{:.2} Hz", hz)
    }
}

/// Format power value with appropriate unit prefix
pub fn format_power(watts: f64) -> String {
    if watts >= 1.0 {
        format!("{:.2} W", watts)
    } else if watts >= 1e-3 {
        format!("{:.2} mW", watts * 1e3)
    } else {
        format!("{:.2} uW", watts * 1e6)
    }
}

/// Format percentage
pub fn format_percent(fraction: f64) -> String {
    format!("{:.1}%", fraction * 100.0)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_e24() {
        // Exact value
        assert_eq!(nearest_e24(1000.0), 1000.0);
        // Close to 1.1k
        assert!((nearest_e24(1050.0) - 1000.0).abs() < 1.0 || (nearest_e24(1050.0) - 1100.0).abs() < 1.0);
        // Verify decade scaling
        assert_eq!(nearest_e24(10000.0), 10000.0);
        assert_eq!(nearest_e24(100.0), 100.0);
    }

    #[test]
    fn test_nearest_e96() {
        assert_eq!(nearest_e96(1000.0), 1000.0);
        // 1.02k should be close to 1020
        assert!((nearest_e96(1020.0) - 1020.0).abs() < 10.0);
    }

    #[test]
    fn test_nearest_inductor() {
        // 21uH should select 22uH
        let result = nearest_inductor(21e-6);
        assert!((result - 22e-6).abs() < 1e-6);

        // 100uH exact
        let result = nearest_inductor(100e-6);
        assert!((result - 100e-6).abs() < 1e-6);
    }

    #[test]
    fn test_next_higher_inductor() {
        // 21uH should select 22uH
        let result = next_higher_inductor(21e-6);
        assert!((result - 22e-6).abs() < 1e-6);

        // 22uH exact should still be 22uH
        let result = next_higher_inductor(22e-6);
        assert!((result - 22e-6).abs() < 1e-6);
    }

    #[test]
    fn test_format_inductance() {
        assert_eq!(format_inductance(22e-6), "22.00 uH");
        assert_eq!(format_inductance(1e-3), "1.00 mH");
        assert_eq!(format_inductance(100e-9), "100.00 nH");
    }

    #[test]
    fn test_format_capacitance() {
        assert_eq!(format_capacitance(100e-6), "100.00 uF");
        assert_eq!(format_capacitance(1e-9), "1.00 nF");
        assert_eq!(format_capacitance(10e-12), "10.00 pF");
    }

    #[test]
    fn test_format_resistance() {
        assert_eq!(format_resistance(100.0), "100.00 Ohm");
        assert_eq!(format_resistance(10000.0), "10.00 kOhm");
        assert_eq!(format_resistance(1e6), "1.00 MOhm");
        assert_eq!(format_resistance(0.05), "50.00 mOhm");
    }

    #[test]
    fn test_format_frequency() {
        assert_eq!(format_frequency(500e3), "500.00 kHz");
        assert_eq!(format_frequency(1e6), "1.00 MHz");
        assert_eq!(format_frequency(2.4e9), "2.40 GHz");
    }
}
