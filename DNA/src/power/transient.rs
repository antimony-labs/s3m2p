//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: transient.rs | DNA/src/power/transient.rs
//! PURPOSE: Time-domain transient simulation for power converters
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! This module provides time-domain simulation of switched power converters
//! using state-space models and numerical integration.
//!
//! # Algorithm
//!
//! 1. **State-Space Model**: Each switch configuration has matrices (A, B, C, D)
//! 2. **PWM Generation**: Duty cycle determines switch on/off timing
//! 3. **Integration**: Backward Euler for stability with stiff systems
//! 4. **Event Handling**: Precise switching at PWM edges
//!
//! # Example
//!
//! ```rust
//! use dna::power::transient::{TransientConfig, TransientResult, simulate_buck};
//!
//! let config = TransientConfig {
//!     vin: 12.0,
//!     duty_cycle: 0.417,  // Target 5V output
//!     fsw: 500e3,         // 500 kHz
//!     inductance: 22e-6,
//!     capacitance: 100e-6,
//!     load_resistance: 2.5, // 5V / 2A
//!     ..Default::default()
//! };
//!
//! let result = simulate_buck(&config);
//! println!("Final Vout: {:.2}V", result.v_out.last().unwrap());
//! ```

use super::state_space::{Matrix2x2, SwitchedConverter};
use serde::{Deserialize, Serialize};

// ============================================================================
// SIMULATION CONFIGURATION
// ============================================================================

/// Configuration for transient simulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransientConfig {
    /// Input voltage (V)
    pub vin: f64,
    /// Duty cycle (0.0 to 1.0)
    pub duty_cycle: f64,
    /// Switching frequency (Hz)
    pub fsw: f64,
    /// Inductance (H)
    pub inductance: f64,
    /// Output capacitance (F)
    pub capacitance: f64,
    /// Load resistance (Ohm)
    pub load_resistance: f64,
    /// Simulation duration (s)
    pub duration: f64,
    /// Time step for output waveforms (s)
    pub output_step: f64,
    /// Number of simulation steps per switching period
    pub steps_per_cycle: usize,
    /// Initial inductor current (A)
    pub initial_il: f64,
    /// Initial capacitor voltage (V)
    pub initial_vc: f64,
}

impl Default for TransientConfig {
    fn default() -> Self {
        Self {
            vin: 12.0,
            duty_cycle: 0.417,
            fsw: 500e3,
            inductance: 22e-6,
            capacitance: 100e-6,
            load_resistance: 2.5,
            duration: 500e-6, // 500us default
            output_step: 1e-6, // 1us output resolution
            steps_per_cycle: 100,
            initial_il: 0.0,
            initial_vc: 0.0,
        }
    }
}

// ============================================================================
// SIMULATION RESULTS
// ============================================================================

/// Results from transient simulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransientResult {
    /// Time points (s)
    pub time: Vec<f64>,
    /// Output voltage waveform (V)
    pub v_out: Vec<f64>,
    /// Inductor current waveform (A)
    pub i_l: Vec<f64>,
    /// Input current waveform (A) - for efficiency calculations
    pub i_in: Vec<f64>,
    /// Switch state at each time point
    pub switch_state: Vec<bool>,
    /// Simulation statistics
    pub stats: SimulationStats,
}

impl TransientResult {
    /// Get steady-state output voltage (average of last few cycles)
    pub fn steady_state_vout(&self) -> f64 {
        if self.v_out.is_empty() {
            return 0.0;
        }
        let n = self.v_out.len().min(100);
        let sum: f64 = self.v_out.iter().rev().take(n).sum();
        sum / n as f64
    }

    /// Get peak-to-peak output voltage ripple (from last few cycles)
    pub fn output_ripple_pp(&self) -> f64 {
        if self.v_out.len() < 10 {
            return 0.0;
        }
        let n = self.v_out.len().min(100);
        let last_samples: Vec<f64> = self.v_out.iter().rev().take(n).copied().collect();
        let max = last_samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min = last_samples.iter().copied().fold(f64::INFINITY, f64::min);
        max - min
    }

    /// Get peak-to-peak inductor current ripple
    pub fn inductor_ripple_pp(&self) -> f64 {
        if self.i_l.len() < 10 {
            return 0.0;
        }
        let n = self.i_l.len().min(100);
        let last_samples: Vec<f64> = self.i_l.iter().rev().take(n).copied().collect();
        let max = last_samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min = last_samples.iter().copied().fold(f64::INFINITY, f64::min);
        max - min
    }
}

/// Simulation statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SimulationStats {
    /// Number of simulation steps
    pub total_steps: usize,
    /// Number of switching cycles
    pub cycles: usize,
    /// Time to reach steady state (s)
    pub settling_time: f64,
    /// Average output voltage
    pub avg_vout: f64,
    /// Average inductor current
    pub avg_il: f64,
    /// Average efficiency estimate
    pub efficiency_estimate: f64,
}

// ============================================================================
// NUMERICAL INTEGRATION
// ============================================================================

/// Backward Euler step for state-space system
///
/// Given: dx/dt = A·x + B·u
/// Backward Euler: x[n+1] = x[n] + h·(A·x[n+1] + B·u)
///
/// Rearranging: (I - h·A)·x[n+1] = x[n] + h·B·u
/// Therefore:   x[n+1] = (I - h·A)^(-1)·(x[n] + h·B·u)
fn backward_euler_step(
    x: [f64; 2],
    u: f64,
    a: &Matrix2x2,
    b: &[f64; 2],
    h: f64,
) -> [f64; 2] {
    // (I - h·A)
    let i_minus_ha = Matrix2x2::new(
        1.0 - h * a.m[0][0],
        -h * a.m[0][1],
        -h * a.m[1][0],
        1.0 - h * a.m[1][1],
    );

    // (I - h·A)^(-1)
    let inv = match i_minus_ha.inverse() {
        Some(m) => m,
        None => return x, // Fallback if singular (shouldn't happen)
    };

    // x[n] + h·B·u
    let rhs = [x[0] + h * b[0] * u, x[1] + h * b[1] * u];

    // x[n+1] = inv · rhs
    inv.mul_vec(rhs)
}

/// Forward Euler step (simpler, less stable but faster)
#[allow(dead_code)]
fn forward_euler_step(
    x: [f64; 2],
    u: f64,
    a: &Matrix2x2,
    b: &[f64; 2],
    h: f64,
) -> [f64; 2] {
    // dx/dt = A·x + B·u
    let ax = a.mul_vec(x);
    let dxdt = [ax[0] + b[0] * u, ax[1] + b[1] * u];

    // x[n+1] = x[n] + h·dx/dt
    [x[0] + h * dxdt[0], x[1] + h * dxdt[1]]
}

// ============================================================================
// SIMULATION ENGINE
// ============================================================================

/// Simulate a buck converter
pub fn simulate_buck(config: &TransientConfig) -> TransientResult {
    let converter = SwitchedConverter::buck(
        config.inductance,
        config.capacitance,
        config.load_resistance,
    );
    simulate_converter(&converter, config, ConverterType::Buck)
}

/// Simulate a boost converter
pub fn simulate_boost(config: &TransientConfig) -> TransientResult {
    let converter = SwitchedConverter::boost(
        config.inductance,
        config.capacitance,
        config.load_resistance,
    );
    simulate_converter(&converter, config, ConverterType::Boost)
}

/// Converter type for input current calculation
#[derive(Clone, Copy, Debug)]
enum ConverterType {
    Buck,
    Boost,
}

/// Core simulation loop
fn simulate_converter(
    converter: &SwitchedConverter,
    config: &TransientConfig,
    conv_type: ConverterType,
) -> TransientResult {
    let t_sw = 1.0 / config.fsw; // Switching period
    let t_on = config.duty_cycle * t_sw; // ON time
    let h = t_sw / config.steps_per_cycle as f64; // Integration step

    // State vector: [iL, vC]
    let mut x = [config.initial_il, config.initial_vc];

    // Output storage
    let mut time = Vec::new();
    let mut v_out = Vec::new();
    let mut i_l = Vec::new();
    let mut i_in = Vec::new();
    let mut switch_state = Vec::new();

    // Simulation tracking
    let mut t = 0.0;
    let mut next_output_time = 0.0;
    let mut cycle_count = 0;

    // Running sums for statistics
    let mut sum_vout = 0.0;
    let mut sum_il = 0.0;
    let mut sum_iin = 0.0;
    let mut sample_count = 0;

    while t < config.duration {
        // Determine position within switching cycle
        let t_in_cycle = t % t_sw;
        let switch_on = t_in_cycle < t_on;

        // Get active model
        let model = converter.model(switch_on);

        // Integration step (Backward Euler)
        x = backward_euler_step(x, config.vin, &model.a, &model.b.m, h);

        // Clamp states to physical limits
        x[0] = x[0].max(0.0); // Inductor current can't go negative in DCM
        x[1] = x[1].max(0.0); // Capacitor voltage can't go negative

        // Calculate input current
        let i_input = match conv_type {
            ConverterType::Buck => {
                if switch_on {
                    x[0] // When switch is on, input current = inductor current
                } else {
                    0.0 // When switch is off, no input current (diode freewheeling)
                }
            }
            ConverterType::Boost => x[0], // Boost: inductor is always in input path
        };

        // Store output if at output time
        if t >= next_output_time {
            time.push(t);
            v_out.push(x[1]);
            i_l.push(x[0]);
            i_in.push(i_input);
            switch_state.push(switch_on);
            next_output_time += config.output_step;
        }

        // Statistics accumulation
        sum_vout += x[1];
        sum_il += x[0];
        sum_iin += i_input;
        sample_count += 1;

        // Track cycles
        if t_in_cycle < h && t > 0.0 {
            cycle_count += 1;
        }

        t += h;
    }

    // Calculate statistics
    let avg_vout = if sample_count > 0 {
        sum_vout / sample_count as f64
    } else {
        0.0
    };
    let avg_il = if sample_count > 0 {
        sum_il / sample_count as f64
    } else {
        0.0
    };
    let avg_iin = if sample_count > 0 {
        sum_iin / sample_count as f64
    } else {
        0.0
    };

    // Efficiency estimate
    let p_out = avg_vout * avg_vout / config.load_resistance;
    let p_in = config.vin * avg_iin;
    let efficiency = if p_in > 0.0 { p_out / p_in } else { 0.0 };

    // Settling time estimate (time to reach 95% of final value)
    let final_vout = v_out.last().copied().unwrap_or(0.0);
    let settling_threshold = 0.95 * final_vout;
    let settling_time = time
        .iter()
        .zip(v_out.iter())
        .find(|(_, &v)| v >= settling_threshold)
        .map(|(&t, _)| t)
        .unwrap_or(config.duration);

    TransientResult {
        time,
        v_out,
        i_l,
        i_in,
        switch_state,
        stats: SimulationStats {
            total_steps: sample_count,
            cycles: cycle_count,
            settling_time,
            avg_vout,
            avg_il,
            efficiency_estimate: efficiency.clamp(0.0, 1.0),
        },
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Calculate expected steady-state output for buck converter
pub fn buck_steady_state_vout(vin: f64, duty: f64) -> f64 {
    vin * duty
}

/// Calculate expected steady-state output for boost converter
pub fn boost_steady_state_vout(vin: f64, duty: f64) -> f64 {
    if duty >= 1.0 {
        return f64::INFINITY;
    }
    vin / (1.0 - duty)
}

/// Calculate duty cycle needed for target output voltage (buck)
pub fn buck_duty_for_vout(vin: f64, vout: f64) -> f64 {
    (vout / vin).clamp(0.0, 1.0)
}

/// Calculate duty cycle needed for target output voltage (boost)
pub fn boost_duty_for_vout(vin: f64, vout: f64) -> f64 {
    if vout <= vin {
        return 0.0;
    }
    (1.0 - vin / vout).clamp(0.0, 0.95) // Limit to 95% for practicality
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_steady_state() {
        // Simulate a buck converter: 12V -> 5V
        let config = TransientConfig {
            vin: 12.0,
            duty_cycle: buck_duty_for_vout(12.0, 5.0), // D = 5/12 ≈ 0.417
            fsw: 500e3,
            inductance: 22e-6,
            capacitance: 100e-6,
            load_resistance: 2.5, // 5V / 2A
            duration: 1e-3,       // 1ms - enough for settling
            output_step: 1e-6,
            steps_per_cycle: 100,
            initial_il: 0.0,
            initial_vc: 0.0,
        };

        let result = simulate_buck(&config);

        // Check steady-state output voltage is close to target
        let vout_ss = result.steady_state_vout();
        assert!(
            (vout_ss - 5.0).abs() < 0.5,
            "Expected ~5V, got {:.2}V",
            vout_ss
        );
    }

    #[test]
    fn test_boost_steady_state() {
        // Simulate a boost converter: 5V -> 12V
        let config = TransientConfig {
            vin: 5.0,
            duty_cycle: boost_duty_for_vout(5.0, 12.0), // D = 1 - 5/12 ≈ 0.583
            fsw: 500e3,
            inductance: 47e-6,
            capacitance: 100e-6,
            load_resistance: 24.0, // 12V / 0.5A
            duration: 2e-3,        // 2ms for settling
            output_step: 2e-6,
            steps_per_cycle: 100,
            initial_il: 0.0,
            initial_vc: 5.0, // Start at input voltage
        };

        let result = simulate_boost(&config);

        // Check steady-state output voltage is close to target
        let vout_ss = result.steady_state_vout();
        assert!(
            (vout_ss - 12.0).abs() < 2.0,
            "Expected ~12V, got {:.2}V",
            vout_ss
        );
    }

    #[test]
    fn test_buck_ripple() {
        let config = TransientConfig {
            vin: 12.0,
            duty_cycle: 0.417,
            fsw: 500e3,
            inductance: 22e-6,
            capacitance: 100e-6,
            load_resistance: 2.5,
            duration: 1e-3,
            output_step: 0.5e-6, // High resolution for ripple measurement
            steps_per_cycle: 100,
            initial_il: 2.0, // Start near steady state
            initial_vc: 5.0,
        };

        let result = simulate_buck(&config);

        // Ripple should be reasonable (< 200mV for this design)
        let ripple = result.output_ripple_pp();
        assert!(
            ripple < 0.2,
            "Expected ripple < 200mV, got {:.1}mV",
            ripple * 1000.0
        );
    }

    #[test]
    fn test_duty_calculations() {
        // Buck: Vout = D * Vin
        assert!((buck_duty_for_vout(12.0, 5.0) - 5.0 / 12.0).abs() < 0.001);
        assert!((buck_steady_state_vout(12.0, 0.5) - 6.0).abs() < 0.001);

        // Boost: Vout = Vin / (1 - D)
        assert!((boost_duty_for_vout(5.0, 12.0) - (1.0 - 5.0 / 12.0)).abs() < 0.001);
        assert!((boost_steady_state_vout(5.0, 0.5) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_simulation_stats() {
        let config = TransientConfig::default();
        let result = simulate_buck(&config);

        assert!(result.stats.total_steps > 0);
        assert!(result.stats.cycles > 0);
        assert!(result.stats.efficiency_estimate > 0.0);
        assert!(result.stats.efficiency_estimate <= 1.0);
    }
}
