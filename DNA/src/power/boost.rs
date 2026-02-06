//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: boost.rs | DNA/src/power/boost.rs
//! PURPOSE: Boost (step-up) converter design equations and calculations
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::components::{next_higher_capacitor, next_higher_inductor};

#[cfg(test)]
use super::components::format_inductance;
use super::types::*;

// ============================================================================
// BOOST CONVERTER EQUATIONS
// ============================================================================

/// Calculate duty cycle for boost converter
/// D = 1 - (Vin / Vout) (ideal)
/// D = 1 - (Vin * eta / Vout) (accounting for efficiency)
pub fn calculate_duty_cycle(vin: f64, vout: f64, efficiency: f64) -> f64 {
    let eta = efficiency.clamp(0.5, 1.0);
    let d = 1.0 - (vin * eta / vout);
    d.clamp(0.05, 0.90) // Limit duty cycle range
}

/// Calculate input (inductor) current for boost converter
/// I_in = I_out / (1 - D)
/// This is the average inductor current
pub fn calculate_input_current(iout: f64, duty: f64) -> f64 {
    if duty >= 0.99 {
        return iout * 100.0; // Limit for very high duty
    }
    iout / (1.0 - duty)
}

/// Calculate minimum inductance for CCM operation
/// L = (Vin * D) / (fsw * delta_IL)
/// where delta_IL = K_ripple * I_in
pub fn calculate_minimum_inductance(
    vin: f64,
    vout: f64,
    iout: f64,
    fsw: f64,
    ripple_ratio: f64,
) -> f64 {
    let d = calculate_duty_cycle(vin, vout, 1.0);
    let i_in = calculate_input_current(iout, d);
    let delta_il = i_in * ripple_ratio;

    if delta_il.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 1e-6;
    }

    let l = (vin * d) / (fsw * delta_il);
    l.max(1e-6)
}

/// Calculate inductor current ripple for a given inductance
/// delta_IL = (Vin * D) / (fsw * L)
pub fn calculate_inductor_ripple(vin: f64, vout: f64, fsw: f64, inductance: f64) -> f64 {
    let d = calculate_duty_cycle(vin, vout, 1.0);

    if inductance.abs() < 1e-12 || fsw.abs() < 1e-9 {
        return 0.0;
    }

    (vin * d) / (fsw * inductance)
}

/// Calculate peak inductor current
/// IL_peak = I_in + delta_IL / 2 = I_out / (1-D) + delta_IL / 2
pub fn calculate_peak_current(iout: f64, duty: f64, delta_il: f64) -> f64 {
    let i_in = calculate_input_current(iout, duty);
    i_in + delta_il / 2.0
}

/// Calculate critical current for CCM/DCM boundary
/// I_crit = (Vin * D * (1-D)^2) / (2 * L * fsw)
pub fn calculate_critical_current(vin: f64, vout: f64, inductance: f64, fsw: f64) -> f64 {
    let d = calculate_duty_cycle(vin, vout, 1.0);

    if inductance.abs() < 1e-12 || fsw.abs() < 1e-9 {
        return 0.0;
    }

    let one_minus_d = 1.0 - d;
    (vin * d * one_minus_d * one_minus_d) / (2.0 * inductance * fsw)
}

/// Calculate minimum output capacitance for voltage ripple specification
/// C_out = (I_out * D) / (fsw * delta_Vout)
/// Note: Boost output capacitor sees higher stress than buck
pub fn calculate_output_capacitance(iout: f64, duty: f64, fsw: f64, ripple_mv: f64) -> f64 {
    let delta_vout = ripple_mv / 1000.0;

    if delta_vout.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 100e-6;
    }

    let c = (iout * duty) / (fsw * delta_vout);
    c.max(1e-6)
}

/// Calculate actual output voltage ripple for given capacitance
/// delta_Vout = (I_out * D) / (fsw * Cout)
pub fn calculate_output_ripple(iout: f64, duty: f64, fsw: f64, cout: f64) -> f64 {
    if cout.abs() < 1e-12 || fsw.abs() < 1e-9 {
        return 0.0;
    }

    let delta_vout = (iout * duty) / (fsw * cout);
    delta_vout * 1000.0 // Return in mV
}

/// Calculate minimum input capacitance
/// For boost, input cap sees triangular current, so ripple is less severe
/// C_in = delta_IL / (8 * fsw * delta_Vin)
pub fn calculate_input_capacitance(delta_il: f64, fsw: f64, ripple_vin_mv: f64) -> f64 {
    let delta_vin = ripple_vin_mv / 1000.0;

    if delta_vin.abs() < 1e-9 || fsw.abs() < 1e-9 {
        return 10e-6;
    }

    let c = delta_il / (8.0 * fsw * delta_vin);
    c.max(1e-6)
}

/// Calculate switch voltage stress (equals Vout for boost)
pub fn calculate_switch_voltage_stress(vout: f64) -> f64 {
    vout
}

/// Calculate switch current stress (equals peak inductor current)
pub fn calculate_switch_current_stress(iout: f64, duty: f64, delta_il: f64) -> f64 {
    calculate_peak_current(iout, duty, delta_il)
}

/// Determine operating mode based on output current
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

/// Design a boost converter from requirements
pub fn design_boost(requirements: &BoostRequirements) -> Result<BoostDesign, String> {
    // Validate requirements
    if requirements.vout <= requirements.vin.max_v {
        return Err(format!(
            "Output voltage ({:.2}V) must be greater than maximum input voltage ({:.2}V)",
            requirements.vout, requirements.vin.max_v
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
    let efficiency_estimate = 0.88; // Boost typically slightly less efficient than buck
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
    if duty_max > 0.85 {
        warnings.push(DesignWarning::HighDutyCycle {
            duty_percent: duty_max * 100.0,
        });
    }

    // Calculate inductance (design for minimum Vin = worst case)
    let l_ideal = calculate_minimum_inductance(
        requirements.vin.min_v,
        requirements.vout,
        requirements.iout_max,
        requirements.switching_freq_hz,
        requirements.ripple.current_ratio,
    );

    let l_selected = next_higher_inductor(l_ideal);

    // Calculate actual ripple with selected inductance (at nominal Vin)
    let delta_il = calculate_inductor_ripple(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.switching_freq_hz,
        l_selected,
    );

    // Calculate currents
    let i_in = calculate_input_current(requirements.iout_max, duty_nom);
    let il_peak = calculate_peak_current(requirements.iout_max, duty_nom, delta_il);

    // High inductor current is a key concern for boost
    if il_peak > 10.0 {
        warnings.push(DesignWarning::HighCurrent {
            actual_a: il_peak,
            recommended_a: 10.0,
        });
    }

    // Critical current for CCM/DCM boundary
    let i_crit = calculate_critical_current(
        requirements.vin.nom_v,
        requirements.vout,
        l_selected,
        requirements.switching_freq_hz,
    );

    let operating_mode = determine_operating_mode(requirements.iout_min, i_crit);
    if operating_mode == OperatingMode::DCM {
        let margin = (i_crit - requirements.iout_min) / i_crit * 100.0;
        warnings.push(DesignWarning::NearDCMBoundary {
            margin_percent: margin,
        });
    }

    // Calculate output capacitance (boost needs more than buck)
    let cout_ideal = calculate_output_capacitance(
        requirements.iout_max,
        duty_nom,
        requirements.switching_freq_hz,
        requirements.ripple.voltage_mv,
    );
    let cout_selected = next_higher_capacitor(cout_ideal);

    let actual_ripple_mv = calculate_output_ripple(
        requirements.iout_max,
        duty_nom,
        requirements.switching_freq_hz,
        cout_selected,
    );

    if actual_ripple_mv > requirements.ripple.voltage_mv * 1.1 {
        warnings.push(DesignWarning::HighRipple {
            actual_mv: actual_ripple_mv,
            spec_mv: requirements.ripple.voltage_mv,
        });
    }

    // Calculate input capacitance
    let cin_ideal = calculate_input_capacitance(
        delta_il,
        requirements.switching_freq_hz,
        100.0, // 100mV input ripple target
    );
    let cin_selected = next_higher_capacitor(cin_ideal);

    // Switch stress
    let switch_voltage = calculate_switch_voltage_stress(requirements.vout);
    let switch_current = calculate_switch_current_stress(requirements.iout_max, duty_nom, delta_il);

    // Create component selections
    let inductor = SelectedComponent::new("L1", l_ideal, l_selected, "H")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Isat > {:.2}A, DCR < 30mOhm recommended",
            il_peak * 1.3
        ));

    let output_capacitor = SelectedComponent::new("Cout", cout_ideal, cout_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Voltage rating > {:.1}V, Low ESR required",
            requirements.vout * 1.3
        ));

    let input_capacitor = SelectedComponent::new("Cin", cin_ideal, cin_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Voltage rating > {:.1}V",
            requirements.vin.max_v * 1.25
        ));

    // Efficiency estimate
    let efficiency = calculate_efficiency_estimate(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.iout_max,
        duty_nom,
        i_in,
    );

    // Thermal analysis
    let p_loss = efficiency.total_losses();
    let thermal = ThermalAnalysis {
        power_dissipation_w: p_loss,
        junction_temp_c: requirements.ambient_temp_c + p_loss * 45.0, // Boost runs hotter
        thermal_margin_c: 125.0 - (requirements.ambient_temp_c + p_loss * 45.0),
        max_ambient_c: 125.0 - p_loss * 45.0,
        requires_heatsink: p_loss > 1.5,
        heatsink_theta_ja: if p_loss > 1.5 {
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

    Ok(BoostDesign {
        requirements: requirements.clone(),
        duty_cycle_max: duty_max,
        duty_cycle_nom: duty_nom,
        duty_cycle_min: duty_min,
        inductor,
        output_capacitor,
        input_capacitor,
        inductor_current_ripple_a: delta_il,
        inductor_peak_current_a: il_peak,
        input_current_a: i_in,
        output_ripple_mv: actual_ripple_mv,
        operating_mode,
        critical_current_a: i_crit,
        switch_voltage_stress_v: switch_voltage,
        switch_current_stress_a: switch_current,
        efficiency,
        thermal,
        warnings,
    })
}

/// Simplified efficiency estimation for boost converter
fn calculate_efficiency_estimate(
    _vin: f64,
    vout: f64,
    iout: f64,
    duty: f64,
    i_in: f64,
) -> EfficiencyBreakdown {
    let p_out = vout * iout;

    // Boost has higher losses than buck due to higher currents
    let rds_on = 0.015; // 15 mOhm (slightly higher for boost)
    let vf = 0.5; // Diode forward drop
    let dcr = 0.040; // Inductor DCR (higher current = higher DCR loss)

    // Conduction losses - note inductor current is higher than output current
    let p_cond_sw = i_in * i_in * rds_on * duty;
    let p_diode = vf * iout; // Diode conducts full output current during off-time
    let p_dcr = i_in * i_in * dcr;

    // Switching losses (higher for boost due to hard switching at Vout)
    let p_sw = 0.07 * p_out; // ~7% switching loss for boost

    let p_q = 0.015; // 15mW controller (boost controllers often more complex)

    let total_loss = p_cond_sw + p_diode + p_dcr + p_sw + p_q;
    let p_in = p_out + total_loss;

    EfficiencyBreakdown {
        total_efficiency: p_out / p_in,
        conduction_loss_w: p_cond_sw,
        switching_loss_w: p_sw,
        diode_loss_w: p_diode,
        inductor_dcr_loss_w: p_dcr,
        capacitor_esr_loss_w: 0.002,
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
        // 5V to 12V should be ~58% duty (ideal)
        let d = calculate_duty_cycle(5.0, 12.0, 1.0);
        assert!((d - 0.583).abs() < 0.02);

        // 3V to 12V should be ~75% duty
        let d2 = calculate_duty_cycle(3.0, 12.0, 1.0);
        assert!((d2 - 0.75).abs() < 0.02);
    }

    #[test]
    fn test_input_current() {
        // At D=0.5, Iin = Iout * 2
        let i_in = calculate_input_current(1.0, 0.5);
        assert!((i_in - 2.0).abs() < 0.01);

        // At D=0.75, Iin = Iout * 4
        let i_in2 = calculate_input_current(1.0, 0.75);
        assert!((i_in2 - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_inductance_calculation() {
        // 3V to 12V, 0.5A, 500kHz, 30% ripple
        let l = calculate_minimum_inductance(3.0, 12.0, 0.5, 500e3, 0.3);
        // Input current is 2A, so ripple is 0.6A, L should be around 7-10uH
        assert!(l > 5e-6 && l < 15e-6);
    }

    #[test]
    fn test_switch_stress() {
        // Switch voltage should equal Vout
        assert_eq!(calculate_switch_voltage_stress(12.0), 12.0);

        // Switch current should equal peak inductor current
        let i_sw = calculate_switch_current_stress(0.5, 0.75, 0.5);
        let expected = 0.5 / 0.25 + 0.25; // I_in + delta_IL/2
        assert!((i_sw - expected).abs() < 0.1);
    }

    #[test]
    fn test_design_boost_basic() {
        let req = BoostRequirements {
            vin: VoltageRange::range(3.0, 5.0),
            vout: 12.0,
            iout_max: 0.5,
            iout_min: 0.05,
            ripple: RippleSpec {
                voltage_mv: 50.0,
                current_ratio: 0.3,
            },
            switching_freq_hz: 500e3,
            ambient_temp_c: 25.0,
        };

        let result = design_boost(&req);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert!(design.duty_cycle_nom > 0.5 && design.duty_cycle_nom < 0.85);
        assert!(design.inductor.selected_value > 5e-6);
        assert!(design.input_current_a > design.requirements.iout_max);
        assert!(design.switch_voltage_stress_v >= design.requirements.vout);

        println!("Boost Design Summary:");
        println!("  Duty cycle: {:.1}%", design.duty_cycle_nom * 100.0);
        println!(
            "  Inductor: {}",
            format_inductance(design.inductor.selected_value)
        );
        println!("  Input current: {:.2}A", design.input_current_a);
        println!("  IL peak: {:.2}A", design.inductor_peak_current_a);
        println!("  Switch V: {:.1}V", design.switch_voltage_stress_v);
        println!(
            "  Efficiency: {:.1}%",
            design.efficiency.total_efficiency * 100.0
        );
    }

    #[test]
    fn test_design_boost_invalid_vout() {
        let req = BoostRequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 5.0, // Invalid: Vout < Vin_max
            ..Default::default()
        };

        let result = design_boost(&req);
        assert!(result.is_err());
    }
}
