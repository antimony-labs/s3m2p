//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: buck.rs | DNA/src/power/buck.rs
//! PURPOSE: Buck (step-down) converter design equations and calculations
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::components::{next_higher_capacitor, next_higher_inductor};

#[cfg(test)]
use super::components::format_inductance;
use super::types::*;

// ============================================================================
// BUCK CONVERTER EQUATIONS
// ============================================================================

/// Calculate duty cycle for buck converter
/// D = Vout / Vin (ideal)
/// D = Vout / (Vin * eta) (accounting for efficiency)
pub fn calculate_duty_cycle(vin: f64, vout: f64, efficiency: f64) -> f64 {
    let eta = efficiency.clamp(0.5, 1.0);
    (vout / (vin * eta)).clamp(0.0, 0.95) // Limit to 95% max duty
}

/// Calculate minimum inductance for CCM operation
/// L_min = (Vout * (1 - D)) / (2 * fsw * Iout * K_ripple)
/// where K_ripple is the inductor current ripple ratio (typically 0.2-0.4)
pub fn calculate_minimum_inductance(
    vin: f64,
    vout: f64,
    iout: f64,
    fsw: f64,
    ripple_ratio: f64,
) -> f64 {
    let d = calculate_duty_cycle(vin, vout, 1.0); // Use ideal duty for inductance calc
    let delta_il = iout * ripple_ratio;

    if delta_il.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 1e-6; // Return minimum 1uH if invalid
    }

    // L = (Vin - Vout) * D / (fsw * delta_IL)
    // or equivalently: L = Vout * (1 - D) / (fsw * delta_IL)
    let l = (vin - vout) * d / (fsw * delta_il);
    l.max(1e-6) // Minimum 1uH
}

/// Calculate inductor current ripple for a given inductance
/// delta_IL = (Vin - Vout) * D / (fsw * L)
pub fn calculate_inductor_ripple(vin: f64, vout: f64, fsw: f64, inductance: f64) -> f64 {
    let d = calculate_duty_cycle(vin, vout, 1.0);

    if inductance.abs() < 1e-12 || fsw.abs() < 1e-9 {
        return 0.0;
    }

    (vin - vout) * d / (fsw * inductance)
}

/// Calculate peak inductor current
/// IL_peak = Iout + delta_IL / 2
pub fn calculate_peak_current(iout: f64, delta_il: f64) -> f64 {
    iout + delta_il / 2.0
}

/// Calculate valley inductor current
/// IL_valley = Iout - delta_IL / 2
pub fn calculate_valley_current(iout: f64, delta_il: f64) -> f64 {
    iout - delta_il / 2.0
}

/// Calculate critical current for CCM/DCM boundary
/// I_crit = delta_IL / 2
/// When Iout < I_crit, converter enters DCM
pub fn calculate_critical_current(delta_il: f64) -> f64 {
    delta_il / 2.0
}

/// Calculate minimum output capacitance for voltage ripple specification
/// C_out = delta_IL / (8 * fsw * delta_Vout)
pub fn calculate_output_capacitance(delta_il: f64, fsw: f64, ripple_mv: f64) -> f64 {
    let delta_vout = ripple_mv / 1000.0; // Convert mV to V

    if delta_vout.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 100e-6; // Default 100uF
    }

    let c = delta_il / (8.0 * fsw * delta_vout);
    c.max(1e-6) // Minimum 1uF
}

/// Calculate actual output voltage ripple for given capacitance
/// delta_Vout = delta_IL / (8 * fsw * Cout)
pub fn calculate_output_ripple(delta_il: f64, fsw: f64, cout: f64) -> f64 {
    if cout.abs() < 1e-12 || fsw.abs() < 1e-9 {
        return 0.0;
    }

    let delta_vout = delta_il / (8.0 * fsw * cout);
    delta_vout * 1000.0 // Return in mV
}

/// Calculate input capacitor RMS current
/// I_cin_rms = Iout * sqrt(D * (1 - D))
pub fn calculate_input_capacitor_rms_current(iout: f64, duty: f64) -> f64 {
    iout * (duty * (1.0 - duty)).sqrt()
}

/// Calculate minimum input capacitance
/// C_in = I_cin_rms / (fsw * delta_Vin)
pub fn calculate_input_capacitance(iout: f64, duty: f64, fsw: f64, ripple_vin_mv: f64) -> f64 {
    let i_rms = calculate_input_capacitor_rms_current(iout, duty);
    let delta_vin = ripple_vin_mv / 1000.0;

    if delta_vin.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 10e-6; // Default 10uF
    }

    let c = i_rms / (fsw * delta_vin);
    c.max(1e-6) // Minimum 1uF
}

/// Determine operating mode based on load current
pub fn determine_operating_mode(iout: f64, critical_current: f64) -> OperatingMode {
    if iout < critical_current * 0.95 {
        OperatingMode::DCM
    } else if iout < critical_current * 1.05 {
        OperatingMode::BCM
    } else {
        OperatingMode::CCM
    }
}

// ============================================================================
// MAIN DESIGN FUNCTION
// ============================================================================

/// Design a buck converter from requirements
pub fn design_buck(requirements: &BuckRequirements) -> Result<BuckDesign, String> {
    // Validate requirements
    if requirements.vout >= requirements.vin.min_v {
        return Err(format!(
            "Output voltage ({:.2}V) must be less than minimum input voltage ({:.2}V)",
            requirements.vout, requirements.vin.min_v
        ));
    }

    if requirements.iout_max <= 0.0 {
        return Err("Maximum output current must be positive".to_string());
    }

    if requirements.switching_freq_hz < 10e3 || requirements.switching_freq_hz > 10e6 {
        return Err("Switching frequency should be between 10kHz and 10MHz".to_string());
    }

    let mut warnings = Vec::new();

    // Calculate duty cycles at different input voltages
    // Use 90% efficiency estimate for initial duty cycle calculation
    let efficiency_estimate = 0.90;
    let duty_max = calculate_duty_cycle(
        requirements.vin.min_v,
        requirements.vout,
        efficiency_estimate,
    );
    let duty_nom = calculate_duty_cycle(
        requirements.vin.nom_v,
        requirements.vout,
        efficiency_estimate,
    );
    let duty_min = calculate_duty_cycle(
        requirements.vin.max_v,
        requirements.vout,
        efficiency_estimate,
    );

    // Check duty cycle limits
    if duty_max > 0.90 {
        warnings.push(DesignWarning::HighDutyCycle {
            duty_percent: duty_max * 100.0,
        });
    }

    // Calculate inductance (design for nominal Vin)
    let l_ideal = calculate_minimum_inductance(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.iout_max,
        requirements.switching_freq_hz,
        requirements.ripple.current_ratio,
    );

    // Select next higher standard value for margin
    let l_selected = next_higher_inductor(l_ideal);

    // Calculate actual ripple with selected inductance
    let delta_il = calculate_inductor_ripple(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.switching_freq_hz,
        l_selected,
    );

    let il_peak = calculate_peak_current(requirements.iout_max, delta_il);
    let i_crit = calculate_critical_current(delta_il);

    // Check if minimum load stays in CCM
    let operating_mode = determine_operating_mode(requirements.iout_min, i_crit);
    if operating_mode == OperatingMode::DCM {
        let margin = (i_crit - requirements.iout_min) / i_crit * 100.0;
        warnings.push(DesignWarning::NearDCMBoundary {
            margin_percent: margin,
        });
    }

    // Calculate output capacitance
    let cout_ideal = calculate_output_capacitance(
        delta_il,
        requirements.switching_freq_hz,
        requirements.ripple.voltage_mv,
    );
    let cout_selected = next_higher_capacitor(cout_ideal);

    // Calculate actual output ripple
    let actual_ripple_mv =
        calculate_output_ripple(delta_il, requirements.switching_freq_hz, cout_selected);

    if actual_ripple_mv > requirements.ripple.voltage_mv * 1.1 {
        warnings.push(DesignWarning::HighRipple {
            actual_mv: actual_ripple_mv,
            spec_mv: requirements.ripple.voltage_mv,
        });
    }

    // Calculate input capacitance (allow 100mV input ripple)
    let cin_ideal = calculate_input_capacitance(
        requirements.iout_max,
        duty_nom,
        requirements.switching_freq_hz,
        100.0, // 100mV input ripple
    );
    let cin_selected = next_higher_capacitor(cin_ideal);

    // Create component selections
    let inductor = SelectedComponent::new("L1", l_ideal, l_selected, "H")
        .with_tolerance(20.0)
        .with_note(&format!("Isat > {:.2}A, DCR < 50mOhm", il_peak * 1.2));

    let output_capacitor = SelectedComponent::new("Cout", cout_ideal, cout_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Voltage rating > {:.1}V, Low ESR",
            requirements.vout * 1.5
        ));

    let input_capacitor = SelectedComponent::new("Cin", cin_ideal, cin_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Voltage rating > {:.1}V, Low ESR",
            requirements.vin.max_v * 1.25
        ));

    // Estimate efficiency (simplified model)
    let efficiency = calculate_efficiency_estimate(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.iout_max,
        duty_nom,
        requirements.switching_freq_hz,
        l_selected,
    );

    // Calculate thermal (simplified - assumes integrated solution)
    let p_loss = efficiency.total_losses();
    let thermal = ThermalAnalysis {
        power_dissipation_w: p_loss,
        junction_temp_c: requirements.ambient_temp_c + p_loss * 40.0, // Assume 40 C/W
        thermal_margin_c: 125.0 - (requirements.ambient_temp_c + p_loss * 40.0),
        max_ambient_c: 125.0 - p_loss * 40.0,
        requires_heatsink: p_loss > 2.0,
        heatsink_theta_ja: if p_loss > 2.0 {
            Some((125.0 - requirements.ambient_temp_c) / p_loss)
        } else {
            None
        },
    };

    if thermal.junction_temp_c > 100.0 {
        warnings.push(DesignWarning::ThermalConcern {
            junction_temp_c: thermal.junction_temp_c,
            max_temp_c: 125.0,
        });
    }

    if efficiency.total_efficiency < 0.80 {
        warnings.push(DesignWarning::LowEfficiency {
            efficiency_percent: efficiency.total_efficiency * 100.0,
        });
    }

    Ok(BuckDesign {
        requirements: requirements.clone(),
        duty_cycle_max: duty_max,
        duty_cycle_nom: duty_nom,
        duty_cycle_min: duty_min,
        inductor,
        output_capacitor,
        input_capacitor,
        inductor_current_ripple_a: delta_il,
        inductor_peak_current_a: il_peak,
        output_ripple_mv: actual_ripple_mv,
        operating_mode,
        critical_current_a: i_crit,
        efficiency,
        thermal,
        warnings,
    })
}

/// Simplified efficiency estimation
fn calculate_efficiency_estimate(
    _vin: f64,
    vout: f64,
    iout: f64,
    duty: f64,
    _fsw: f64,
    _inductance: f64,
) -> EfficiencyBreakdown {
    let p_out = vout * iout;

    // Simplified loss model
    // Assume: Rds_on = 10mOhm, Vf = 0.5V, DCR = 30mOhm
    let rds_on = 0.010; // 10 mOhm
    let vf = 0.5; // 0.5V diode forward drop
    let dcr = 0.030; // 30 mOhm inductor DCR

    // Conduction losses
    let p_cond_sw = iout * iout * rds_on * duty;
    let p_diode = vf * iout * (1.0 - duty);
    let p_dcr = iout * iout * dcr;

    // Switching losses (simplified)
    let p_sw = 0.05 * p_out; // Assume 5% switching loss

    // Quiescent
    let p_q = 0.010; // 10mW controller

    let total_loss = p_cond_sw + p_diode + p_dcr + p_sw + p_q;
    let p_in = p_out + total_loss;

    EfficiencyBreakdown {
        total_efficiency: p_out / p_in,
        conduction_loss_w: p_cond_sw,
        switching_loss_w: p_sw,
        diode_loss_w: p_diode,
        inductor_dcr_loss_w: p_dcr,
        capacitor_esr_loss_w: 0.001, // Negligible
        quiescent_loss_w: p_q,
        output_power_w: p_out,
        input_power_w: p_in,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duty_cycle() {
        // 12V to 5V should be ~42% duty (ideal)
        let d = calculate_duty_cycle(12.0, 5.0, 1.0);
        assert!((d - 0.417).abs() < 0.01);

        // With 90% efficiency, duty should be higher
        let d_eff = calculate_duty_cycle(12.0, 5.0, 0.9);
        assert!(d_eff > d);
    }

    #[test]
    fn test_inductance_calculation() {
        // 12V to 5V, 2A, 500kHz, 30% ripple
        // D = 5/12 = 0.417, delta_IL = 2A * 0.3 = 0.6A
        // L = (12-5) * 0.417 / (500e3 * 0.6) = 9.7uH
        let l = calculate_minimum_inductance(12.0, 5.0, 2.0, 500e3, 0.3);
        assert!(l > 5e-6 && l < 15e-6, "L = {:.2}uH", l * 1e6);
    }

    #[test]
    fn test_ripple_calculation() {
        // With 10uH inductor (closer to calculated value)
        // delta_IL = (12-5) * 0.417 / (500e3 * 10e-6) = 0.58A
        let delta_il = calculate_inductor_ripple(12.0, 5.0, 500e3, 10e-6);
        assert!(
            delta_il > 0.4 && delta_il < 0.8,
            "delta_IL = {:.2}A",
            delta_il
        );
    }

    #[test]
    fn test_output_capacitance() {
        // For 0.6A ripple, 50mV target, 500kHz
        // C = 0.6 / (8 * 500e3 * 0.05) = 3uF
        let cout = calculate_output_capacitance(0.6, 500e3, 50.0);
        assert!(cout > 1e-6 && cout < 10e-6, "Cout = {:.2}uF", cout * 1e6);
    }

    #[test]
    fn test_design_buck_basic() {
        let req = BuckRequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 5.0,
            iout_max: 2.0,
            iout_min: 0.2,
            ripple: RippleSpec {
                voltage_mv: 50.0,
                current_ratio: 0.3,
            },
            switching_freq_hz: 500e3,
            ambient_temp_c: 25.0,
        };

        let result = design_buck(&req);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert!(design.duty_cycle_nom > 0.3 && design.duty_cycle_nom < 0.6);
        assert!(
            design.inductor.selected_value > 1e-6,
            "L = {:.2}uH",
            design.inductor.selected_value * 1e6
        );
        assert!(design.output_capacitor.selected_value > 1e-6);
        assert!(design.efficiency.total_efficiency > 0.8);

        println!("Buck Design Summary:");
        println!("  Duty cycle: {:.1}%", design.duty_cycle_nom * 100.0);
        println!(
            "  Inductor: {}",
            format_inductance(design.inductor.selected_value)
        );
        println!("  IL ripple: {:.2}A", design.inductor_current_ripple_a);
        println!("  IL peak: {:.2}A", design.inductor_peak_current_a);
        println!(
            "  Efficiency: {:.1}%",
            design.efficiency.total_efficiency * 100.0
        );
    }

    #[test]
    fn test_design_buck_invalid_vout() {
        let req = BuckRequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 15.0, // Invalid: Vout > Vin_min
            ..Default::default()
        };

        let result = design_buck(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_operating_mode_detection() {
        assert_eq!(determine_operating_mode(2.0, 0.5), OperatingMode::CCM);
        assert_eq!(determine_operating_mode(0.2, 0.5), OperatingMode::DCM);
        assert_eq!(determine_operating_mode(0.5, 0.5), OperatingMode::BCM);
    }
}
