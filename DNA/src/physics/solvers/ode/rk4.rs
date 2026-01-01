//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: rk4.rs | DNA/src/physics/solvers/ode/rk4.rs
//! PURPOSE: 4th-order Runge-Kutta ODE integrator
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: 4th-order Runge-Kutta ODE integrator
//!
//! LAYER: DNA → PHYSICS → SOLVERS → ODE
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ ALGORITHM                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Classic RK4 for dy/dt = f(t, y):                                            │
//! │                                                                             │
//! │   k1 = f(t,       y)                                                        │
//! │   k2 = f(t + h/2, y + h·k1/2)                                               │
//! │   k3 = f(t + h/2, y + h·k2/2)                                               │
//! │   k4 = f(t + h,   y + h·k3)                                                 │
//! │                                                                             │
//! │   y_{n+1} = y_n + h·(k1 + 2k2 + 2k3 + k4)/6                                 │
//! │                                                                             │
//! │ Error: O(h⁵) per step, O(h⁴) global                                         │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! REFERENCE: https://en.wikipedia.org/wiki/Runge-Kutta_methods
//!
//! ═══════════════════════════════════════════════════════════════════════════════

/// RK4 step: 4th-order Runge-Kutta
pub fn rk4_step<F>(y: f64, t: f64, h: f64, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let k1 = f(t, y);
    let k2 = f(t + h / 2.0, y + h * k1 / 2.0);
    let k3 = f(t + h / 2.0, y + h * k2 / 2.0);
    let k4 = f(t + h, y + h * k3);

    y + h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_rk4() {
        // dy/dt = y, solution y(t) = e^t
        let mut y = 1.0;
        let dt = 0.1;

        for _ in 0..10 {
            y = rk4_step(y, 0.0, dt, |_, y_val| y_val);
        }

        // After t=1.0, should be very close to e ≈ 2.718
        assert!((y - std::f64::consts::E).abs() < 0.001);
    }
}
