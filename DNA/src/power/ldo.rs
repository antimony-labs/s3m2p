//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: ldo.rs | DNA/src/power/ldo.rs
//! PURPOSE: LDO (Low Dropout Regulator) design equations and calculations
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::components::next_higher_capacitor;
use super::types::*;

// ============================================================================
// LDO EQUATIONS
// ============================================================================

/// Calculate power dissipation in LDO
/// P_diss = (Vin - Vout) * Iout
/// This is the fundamental limitation of linear regulators
pub fn calculate_power_dissipation(vin: f64, vout: f64, iout: f64) -> f64 {
    (vin - vout) * iout
}

/// Calculate efficiency (always Vout/Vin for LDO)
/// eta = Vout / Vin
pub fn calculate_efficiency(vin: f64, vout: f64) -> f64 {
    if vin.abs() < 1e-9 {
        return 0.0;
    }
    vout / vin
}

/// Calculate headroom (voltage across the pass element)
/// Headroom = Vin - Vout
pub fn calculate_headroom(vin: f64, vout: f64) -> f64 {
    vin - vout
}

/// Check if dropout requirement is met
/// Returns true if headroom >= dropout voltage
pub fn check_dropout(vin: f64, vout: f64, dropout: f64) -> bool {
    calculate_headroom(vin, vout) >= dropout
}

/// Calculate junction temperature
/// T_j = T_ambient + P_diss * Theta_JA
pub fn calculate_junction_temperature(p_diss: f64, theta_ja: f64, t_ambient: f64) -> f64 {
    t_ambient + p_diss * theta_ja
}

/// Calculate thermal margin to maximum junction temperature
pub fn calculate_thermal_margin(t_junction: f64, t_max: f64) -> f64 {
    t_max - t_junction
}

/// Calculate maximum ambient temperature for safe operation
/// T_ambient_max = T_max - P_diss * Theta_JA
pub fn calculate_max_ambient(p_diss: f64, theta_ja: f64, t_max: f64) -> f64 {
    t_max - p_diss * theta_ja
}

/// Calculate maximum current limited by thermal dissipation
/// I_max = (T_max - T_ambient) / (Theta_JA * (Vin - Vout))
pub fn calculate_max_current_thermal(
    vin: f64,
    vout: f64,
    theta_ja: f64,
    t_ambient: f64,
    t_max: f64,
) -> f64 {
    let headroom = vin - vout;
    if headroom.abs() < 1e-9 || theta_ja.abs() < 1e-9 {
        return 10.0; // Default high limit
    }

    (t_max - t_ambient) / (theta_ja * headroom)
}

/// Calculate recommended output capacitor for stability
/// LDOs require specific ESR range for stability
/// Typical requirement: 1-22uF ceramic or 10-100uF electrolytic
pub fn calculate_output_capacitance(
    iout_max: f64,
    transient_response_us: f64,
    voltage_tolerance_mv: f64,
) -> f64 {
    // C = I_step * delta_t / delta_V
    // For load step response
    let delta_v = voltage_tolerance_mv / 1000.0;
    let delta_t = transient_response_us * 1e-6;

    if delta_v.abs() < 1e-9 {
        return 10e-6; // Default 10uF
    }

    let c = iout_max * delta_t / delta_v;
    c.clamp(1e-6, 1000e-6) // 1uF to 1000uF range
}

/// Calculate minimum input capacitor
/// Typically 0.1-10uF for noise filtering
pub fn calculate_input_capacitance(iout_max: f64) -> f64 {
    // Rule of thumb: 1uF per 100mA
    let c = iout_max * 10e-6;
    c.clamp(0.1e-6, 100e-6)
}

/// Estimate PSRR (Power Supply Rejection Ratio) at DC
/// Typical LDO PSRR: 60-80 dB at DC, decreasing with frequency
pub fn estimate_psrr_dc(headroom: f64, dropout: f64) -> f64 {
    // PSRR improves with more headroom
    let headroom_factor = (headroom / dropout).clamp(1.0, 5.0);
    60.0 + 10.0 * (headroom_factor - 1.0).ln().max(0.0) // 60-80 dB range
}

// ============================================================================
// MAIN DESIGN FUNCTION
// ============================================================================

/// Design an LDO from requirements
pub fn design_ldo(requirements: &LDORequirements) -> Result<LDODesign, String> {
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

    let mut warnings = Vec::new();

    // Calculate headroom at different input voltages
    let headroom_min = calculate_headroom(requirements.vin.min_v, requirements.vout);
    let _headroom_nom = calculate_headroom(requirements.vin.nom_v, requirements.vout);
    let headroom_max = calculate_headroom(requirements.vin.max_v, requirements.vout);

    // Check dropout requirement
    let meets_dropout = headroom_min >= requirements.dropout_voltage;
    if !meets_dropout {
        warnings.push(DesignWarning::InsufficientHeadroom {
            headroom_v: headroom_min,
            minimum_v: requirements.dropout_voltage,
        });
    }

    // Calculate power dissipation at different input voltages
    let p_diss_min = calculate_power_dissipation(
        requirements.vin.min_v,
        requirements.vout,
        requirements.iout_max,
    );
    let p_diss_nom = calculate_power_dissipation(
        requirements.vin.nom_v,
        requirements.vout,
        requirements.iout_max,
    );
    let p_diss_max = calculate_power_dissipation(
        requirements.vin.max_v,
        requirements.vout,
        requirements.iout_max,
    );

    // Calculate efficiency at different input voltages
    let eff_max = calculate_efficiency(requirements.vin.min_v, requirements.vout) * 100.0;
    let eff_nom = calculate_efficiency(requirements.vin.nom_v, requirements.vout) * 100.0;
    let eff_min = calculate_efficiency(requirements.vin.max_v, requirements.vout) * 100.0;

    // Thermal analysis (at worst case = max Vin)
    let t_junction = calculate_junction_temperature(
        p_diss_max,
        requirements.package_theta_ja,
        requirements.ambient_temp_c,
    );
    let thermal_margin = calculate_thermal_margin(t_junction, requirements.max_junction_temp_c);
    let max_ambient = calculate_max_ambient(
        p_diss_max,
        requirements.package_theta_ja,
        requirements.max_junction_temp_c,
    );

    // Check thermal limits
    let requires_heatsink = t_junction > requirements.max_junction_temp_c - 10.0;
    let heatsink_theta = if requires_heatsink {
        // Calculate required heatsink thermal resistance
        let target_theta =
            (requirements.max_junction_temp_c - 10.0 - requirements.ambient_temp_c) / p_diss_max;
        Some(target_theta)
    } else {
        None
    };

    let thermal = ThermalAnalysis {
        power_dissipation_w: p_diss_max,
        junction_temp_c: t_junction,
        thermal_margin_c: thermal_margin,
        max_ambient_c: max_ambient,
        requires_heatsink,
        heatsink_theta_ja: heatsink_theta,
    };

    if t_junction > requirements.max_junction_temp_c - 25.0 {
        warnings.push(DesignWarning::ThermalConcern {
            junction_temp_c: t_junction,
            max_temp_c: requirements.max_junction_temp_c,
        });
    }

    // Maximum current limited by thermal
    let max_current_thermal = calculate_max_current_thermal(
        requirements.vin.max_v,
        requirements.vout,
        requirements.package_theta_ja,
        requirements.ambient_temp_c,
        requirements.max_junction_temp_c,
    );

    let safe_operating_area = requirements.iout_max <= max_current_thermal;
    if !safe_operating_area {
        warnings.push(DesignWarning::HighCurrent {
            actual_a: requirements.iout_max,
            recommended_a: max_current_thermal,
        });
    }

    // Low efficiency warning (LDOs are inherently inefficient at high Vin/Vout ratio)
    if eff_min < 50.0 {
        warnings.push(DesignWarning::LowEfficiency {
            efficiency_percent: eff_min,
        });
    }

    // Calculate capacitors
    // Output cap: 10uF typical for ceramic, need low ESR for stability
    let cout_ideal = calculate_output_capacitance(
        requirements.iout_max,
        50.0, // 50us transient response
        50.0, // 50mV tolerance
    );
    let cout_selected = next_higher_capacitor(cout_ideal).max(10e-6); // Minimum 10uF

    // Input cap: 0.1-10uF
    let cin_ideal = calculate_input_capacitance(requirements.iout_max);
    let cin_selected = next_higher_capacitor(cin_ideal).max(1e-6); // Minimum 1uF

    // Create component selections
    let output_capacitor = SelectedComponent::new("Cout", cout_ideal, cout_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "ESR: {:.3}-{:.3} Ohm for stability",
            requirements.output_cap_esr_range.0, requirements.output_cap_esr_range.1
        ))
        .with_note(&format!("Voltage rating > {:.1}V", requirements.vout * 1.5));

    let input_capacitor = SelectedComponent::new("Cin", cin_ideal, cin_selected, "F")
        .with_tolerance(20.0)
        .with_note(&format!(
            "Voltage rating > {:.1}V",
            requirements.vin.max_v * 1.25
        ));

    Ok(LDODesign {
        requirements: requirements.clone(),
        headroom_min_v: headroom_min,
        headroom_max_v: headroom_max,
        meets_dropout,
        input_capacitor,
        output_capacitor,
        power_dissipation_min_w: p_diss_min,
        power_dissipation_nom_w: p_diss_nom,
        power_dissipation_max_w: p_diss_max,
        efficiency_max_percent: eff_max,
        efficiency_nom_percent: eff_nom,
        efficiency_min_percent: eff_min,
        thermal,
        safe_operating_area,
        max_current_thermal_a: max_current_thermal,
        warnings,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_dissipation() {
        // 5V to 3.3V, 500mA = 0.85W
        let p = calculate_power_dissipation(5.0, 3.3, 0.5);
        assert!((p - 0.85).abs() < 0.01);

        // 12V to 3.3V, 500mA = 4.35W (much higher!)
        let p2 = calculate_power_dissipation(12.0, 3.3, 0.5);
        assert!((p2 - 4.35).abs() < 0.01);
    }

    #[test]
    fn test_efficiency() {
        // 5V to 3.3V = 66% efficiency
        let eff = calculate_efficiency(5.0, 3.3);
        assert!((eff - 0.66).abs() < 0.01);

        // 3.6V to 3.3V = 91.7% efficiency (near dropout = best case)
        let eff2 = calculate_efficiency(3.6, 3.3);
        assert!((eff2 - 0.917).abs() < 0.01);
    }

    #[test]
    fn test_junction_temperature() {
        // 1W dissipation, 50 C/W thermal resistance, 25C ambient
        // Tj = 25 + 1 * 50 = 75C
        let tj = calculate_junction_temperature(1.0, 50.0, 25.0);
        assert!((tj - 75.0).abs() < 0.1);
    }

    #[test]
    fn test_max_current_thermal() {
        // 5V to 3.3V, 50 C/W, 25C ambient, 125C max
        // I_max = (125 - 25) / (50 * 1.7) = 1.18A
        let i_max = calculate_max_current_thermal(5.0, 3.3, 50.0, 25.0, 125.0);
        assert!((i_max - 1.18).abs() < 0.05);
    }

    #[test]
    fn test_dropout_check() {
        // 3.6V in, 3.3V out, 0.2V dropout requirement
        assert!(check_dropout(3.6, 3.3, 0.2));

        // 3.4V in, 3.3V out, 0.2V dropout requirement - fails
        assert!(!check_dropout(3.4, 3.3, 0.2));
    }

    #[test]
    fn test_design_ldo_basic() {
        let req = LDORequirements {
            vin: VoltageRange::range(4.5, 5.5),
            vout: 3.3,
            iout_max: 0.5,
            dropout_voltage: 0.3,
            output_cap_esr_range: (0.01, 0.3),
            ambient_temp_c: 25.0,
            package_theta_ja: 50.0,
            max_junction_temp_c: 125.0,
        };

        let result = design_ldo(&req);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert!(design.meets_dropout);
        assert!(design.efficiency_nom_percent > 60.0);
        assert!(design.thermal.junction_temp_c < 125.0);

        println!("LDO Design Summary:");
        println!("  Headroom (min): {:.2}V", design.headroom_min_v);
        println!("  Efficiency (nom): {:.1}%", design.efficiency_nom_percent);
        println!(
            "  Power dissipation: {:.2}W",
            design.power_dissipation_max_w
        );
        println!("  Junction temp: {:.1}C", design.thermal.junction_temp_c);
        println!(
            "  Max thermal current: {:.2}A",
            design.max_current_thermal_a
        );
    }

    #[test]
    fn test_design_ldo_thermal_limit() {
        // High Vin, low Vout, high current = thermal challenge
        let req = LDORequirements {
            vin: VoltageRange::range(10.0, 14.0),
            vout: 3.3,
            iout_max: 1.0,
            dropout_voltage: 0.3,
            output_cap_esr_range: (0.01, 0.3),
            ambient_temp_c: 25.0,
            package_theta_ja: 50.0, // SOT-223
            max_junction_temp_c: 125.0,
        };

        let result = design_ldo(&req);
        assert!(result.is_ok());

        let design = result.unwrap();
        // Should have thermal warning
        assert!(design
            .warnings
            .iter()
            .any(|w| matches!(w, DesignWarning::ThermalConcern { .. })));
        // Should flag heatsink requirement
        assert!(design.thermal.requires_heatsink);
    }

    #[test]
    fn test_design_ldo_dropout_fail() {
        let req = LDORequirements {
            vin: VoltageRange::range(3.4, 3.6),
            vout: 3.3,
            iout_max: 0.5,
            dropout_voltage: 0.3, // Needs 0.3V but only has 0.1V at Vin_min
            ..Default::default()
        };

        let result = design_ldo(&req);
        assert!(result.is_ok());

        let design = result.unwrap();
        assert!(!design.meets_dropout);
        assert!(design
            .warnings
            .iter()
            .any(|w| matches!(w, DesignWarning::InsufficientHeadroom { .. })));
    }
}
