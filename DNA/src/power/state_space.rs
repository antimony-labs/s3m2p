//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: state_space.rs | DNA/src/power/state_space.rs
//! PURPOSE: State-space representation for power converter simulation
//! MODIFIED: 2026-01-07
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Power converters are switched systems that alternate between different
//! circuit configurations. Each configuration has its own state-space model:
//!
//! dx/dt = A·x + B·u
//! y = C·x + D·u
//!
//! Where:
//! - x = state vector [inductor current, capacitor voltage, ...]
//! - u = input vector [Vin, ...]
//! - y = output vector [Vout, Iin, ...]
//!
//! For small converters (2-3 state variables), we use simple fixed-size
//! matrices rather than dynamic allocation.

use serde::{Deserialize, Serialize};

// ============================================================================
// FIXED-SIZE MATRICES (avoiding heap allocation for small systems)
// ============================================================================

/// 2x2 matrix for second-order systems (most common converters)
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Matrix2x2 {
    pub m: [[f64; 2]; 2],
}

impl Matrix2x2 {
    pub const ZERO: Self = Self { m: [[0.0; 2]; 2] };
    pub const IDENTITY: Self = Self {
        m: [[1.0, 0.0], [0.0, 1.0]],
    };

    pub fn new(a11: f64, a12: f64, a21: f64, a22: f64) -> Self {
        Self {
            m: [[a11, a12], [a21, a22]],
        }
    }

    /// Matrix-vector multiplication: result = M * v
    pub fn mul_vec(&self, v: [f64; 2]) -> [f64; 2] {
        [
            self.m[0][0] * v[0] + self.m[0][1] * v[1],
            self.m[1][0] * v[0] + self.m[1][1] * v[1],
        ]
    }

    /// Matrix addition
    pub fn add(&self, other: &Self) -> Self {
        Self {
            m: [
                [self.m[0][0] + other.m[0][0], self.m[0][1] + other.m[0][1]],
                [self.m[1][0] + other.m[1][0], self.m[1][1] + other.m[1][1]],
            ],
        }
    }

    /// Scalar multiplication
    pub fn scale(&self, s: f64) -> Self {
        Self {
            m: [
                [self.m[0][0] * s, self.m[0][1] * s],
                [self.m[1][0] * s, self.m[1][1] * s],
            ],
        }
    }

    /// Matrix multiplication
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            m: [
                [
                    self.m[0][0] * other.m[0][0] + self.m[0][1] * other.m[1][0],
                    self.m[0][0] * other.m[0][1] + self.m[0][1] * other.m[1][1],
                ],
                [
                    self.m[1][0] * other.m[0][0] + self.m[1][1] * other.m[1][0],
                    self.m[1][0] * other.m[0][1] + self.m[1][1] * other.m[1][1],
                ],
            ],
        }
    }

    /// 2x2 matrix inverse using direct formula
    pub fn inverse(&self) -> Option<Self> {
        let det = self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0];
        if det.abs() < 1e-15 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self {
            m: [
                [self.m[1][1] * inv_det, -self.m[0][1] * inv_det],
                [-self.m[1][0] * inv_det, self.m[0][0] * inv_det],
            ],
        })
    }
}

/// 2x1 matrix (column vector or B matrix for single input)
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Matrix2x1 {
    pub m: [f64; 2],
}

impl Matrix2x1 {
    pub const ZERO: Self = Self { m: [0.0; 2] };

    pub fn new(a1: f64, a2: f64) -> Self {
        Self { m: [a1, a2] }
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            m: [self.m[0] * s, self.m[1] * s],
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            m: [self.m[0] + other.m[0], self.m[1] + other.m[1]],
        }
    }
}

/// 1x2 matrix (row vector for C matrix - single output)
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Matrix1x2 {
    pub m: [f64; 2],
}

impl Matrix1x2 {
    pub const ZERO: Self = Self { m: [0.0; 2] };

    pub fn new(a1: f64, a2: f64) -> Self {
        Self { m: [a1, a2] }
    }

    /// Dot product with state vector
    pub fn dot(&self, x: [f64; 2]) -> f64 {
        self.m[0] * x[0] + self.m[1] * x[1]
    }
}

// ============================================================================
// STATE-SPACE MODELS
// ============================================================================

/// State-space model for a second-order converter
///
/// dx/dt = A·x + B·u
/// y = C·x + D·u
///
/// States: x = [inductor current (A), capacitor voltage (V)]
/// Input:  u = input voltage (V)
/// Output: y = output voltage (V)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StateSpace2 {
    /// State matrix (2x2)
    pub a: Matrix2x2,
    /// Input matrix (2x1)
    pub b: Matrix2x1,
    /// Output matrix (1x2)
    pub c: Matrix1x2,
    /// Feedthrough (scalar for single-input single-output)
    pub d: f64,
}

impl Default for StateSpace2 {
    fn default() -> Self {
        Self {
            a: Matrix2x2::ZERO,
            b: Matrix2x1::ZERO,
            c: Matrix1x2::new(0.0, 1.0), // Output = capacitor voltage
            d: 0.0,
        }
    }
}

impl StateSpace2 {
    /// Create state-space model for buck converter in ON state (switch closed)
    ///
    /// Circuit: Vin --[SW]--[L]--+--[R_load]
    ///                           |
    ///                         [C]
    ///                           |
    ///                          GND
    ///
    /// States: x = [iL, vC]
    /// Equations (ideal, ignoring parasitics):
    ///   L·diL/dt = Vin - vC  => diL/dt = -vC/L + Vin/L
    ///   C·dvC/dt = iL - vC/R => dvC/dt = iL/C - vC/(RC)
    pub fn buck_on(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            a: Matrix2x2::new(0.0, -1.0 / l, 1.0 / c, -1.0 / (r_load * c)),
            b: Matrix2x1::new(1.0 / l, 0.0),
            c: Matrix1x2::new(0.0, 1.0), // Output = vC
            d: 0.0,
        }
    }

    /// Create state-space model for buck converter in OFF state (switch open)
    ///
    /// Circuit: [D]--[L]--+--[R_load]
    ///                    |
    ///                  [C]
    ///                    |
    ///                   GND
    ///
    /// Diode freewheeling, input disconnected
    /// Equations (ideal diode, Vd=0):
    ///   L·diL/dt = -vC        => diL/dt = -vC/L
    ///   C·dvC/dt = iL - vC/R  => dvC/dt = iL/C - vC/(RC)
    pub fn buck_off(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            a: Matrix2x2::new(0.0, -1.0 / l, 1.0 / c, -1.0 / (r_load * c)),
            b: Matrix2x1::new(0.0, 0.0), // Input disconnected
            c: Matrix1x2::new(0.0, 1.0),
            d: 0.0,
        }
    }

    /// Create state-space model for boost converter in ON state (switch closed)
    ///
    /// Circuit: Vin --[L]--+
    ///                     |
    ///                    [SW]
    ///                     |
    ///                    GND
    ///
    /// Output capacitor isolated (diode reverse biased)
    /// Equations:
    ///   L·diL/dt = Vin        => diL/dt = Vin/L
    ///   C·dvC/dt = -vC/R      => dvC/dt = -vC/(RC)  (load only)
    pub fn boost_on(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            a: Matrix2x2::new(0.0, 0.0, 0.0, -1.0 / (r_load * c)),
            b: Matrix2x1::new(1.0 / l, 0.0),
            c: Matrix1x2::new(0.0, 1.0),
            d: 0.0,
        }
    }

    /// Create state-space model for boost converter in OFF state (switch open)
    ///
    /// Circuit: Vin --[L]--[D]--+--[R_load]
    ///                          |
    ///                        [C]
    ///                          |
    ///                         GND
    ///
    /// Equations:
    ///   L·diL/dt = Vin - vC   => diL/dt = -vC/L + Vin/L
    ///   C·dvC/dt = iL - vC/R  => dvC/dt = iL/C - vC/(RC)
    pub fn boost_off(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            a: Matrix2x2::new(0.0, -1.0 / l, 1.0 / c, -1.0 / (r_load * c)),
            b: Matrix2x1::new(1.0 / l, 0.0),
            c: Matrix1x2::new(0.0, 1.0),
            d: 0.0,
        }
    }

    /// Calculate state derivative: dx/dt = A·x + B·u
    pub fn derivative(&self, x: [f64; 2], u: f64) -> [f64; 2] {
        let ax = self.a.mul_vec(x);
        let bu = self.b.scale(u);
        [ax[0] + bu.m[0], ax[1] + bu.m[1]]
    }

    /// Calculate output: y = C·x + D·u
    pub fn output(&self, x: [f64; 2], u: f64) -> f64 {
        self.c.dot(x) + self.d * u
    }
}

// ============================================================================
// SWITCHED CONVERTER MODEL
// ============================================================================

/// Switched converter model with multiple configurations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchedConverter {
    /// State-space model when switch is ON
    pub model_on: StateSpace2,
    /// State-space model when switch is OFF
    pub model_off: StateSpace2,
    /// Inductor value (H)
    pub inductance: f64,
    /// Output capacitor (F)
    pub capacitance: f64,
    /// Load resistance (Ohm)
    pub load_resistance: f64,
}

impl SwitchedConverter {
    /// Create a buck converter model
    pub fn buck(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            model_on: StateSpace2::buck_on(l, c, r_load),
            model_off: StateSpace2::buck_off(l, c, r_load),
            inductance: l,
            capacitance: c,
            load_resistance: r_load,
        }
    }

    /// Create a boost converter model
    pub fn boost(l: f64, c: f64, r_load: f64) -> Self {
        Self {
            model_on: StateSpace2::boost_on(l, c, r_load),
            model_off: StateSpace2::boost_off(l, c, r_load),
            inductance: l,
            capacitance: c,
            load_resistance: r_load,
        }
    }

    /// Get the active model based on switch state
    pub fn model(&self, switch_on: bool) -> &StateSpace2 {
        if switch_on {
            &self.model_on
        } else {
            &self.model_off
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
    fn test_matrix_inverse() {
        let m = Matrix2x2::new(4.0, 7.0, 2.0, 6.0);
        let inv = m.inverse().unwrap();

        // M * M^-1 should be identity
        let product = m.mul(&inv);
        assert!((product.m[0][0] - 1.0).abs() < 1e-10);
        assert!((product.m[1][1] - 1.0).abs() < 1e-10);
        assert!(product.m[0][1].abs() < 1e-10);
        assert!(product.m[1][0].abs() < 1e-10);
    }

    #[test]
    fn test_buck_equilibrium() {
        // At steady state with D=0.5, Vin=12V, we expect Vout=6V
        // For equilibrium: dx/dt = 0
        // With average model: D*Aon + (1-D)*Aoff, D*Bon + (1-D)*Boff
        let l = 22e-6;
        let c = 100e-6;
        let r = 3.0; // 6V / 2A
        let vin = 12.0;

        let buck = SwitchedConverter::buck(l, c, r);

        // Test derivative calculation
        let x = [2.0, 6.0]; // iL=2A, vC=6V (expected equilibrium)
        let dxdt_on = buck.model_on.derivative(x, vin);

        // During ON: diL/dt = (Vin - Vout)/L = (12-6)/(22e-6) ≈ 272727 A/s
        let expected_dil_on = (vin - x[1]) / l;
        assert!((dxdt_on[0] - expected_dil_on).abs() / expected_dil_on.abs() < 0.01);
    }

    #[test]
    fn test_boost_model() {
        let l = 47e-6;
        let c = 100e-6;
        let r = 12.0; // 12V / 1A

        let boost = SwitchedConverter::boost(l, c, r);

        // During ON state, inductor charges from input
        let x = [1.0, 12.0];
        let dxdt_on = boost.model_on.derivative(x, 5.0);

        // diL/dt = Vin/L during ON
        let expected_dil_on = 5.0 / l;
        assert!((dxdt_on[0] - expected_dil_on).abs() / expected_dil_on.abs() < 0.01);
    }

    #[test]
    fn test_output_calculation() {
        let ss = StateSpace2::buck_on(22e-6, 100e-6, 2.5);
        let x = [2.0, 5.0]; // iL=2A, vC=5V

        let y = ss.output(x, 12.0);
        assert!((y - 5.0).abs() < 1e-10); // Output should be capacitor voltage
    }
}
