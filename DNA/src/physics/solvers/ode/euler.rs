//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: euler.rs
//! PATH: DNA/src/physics/solvers/ode/euler.rs
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PURPOSE: Forward Euler ODE integrator (1st order, explicit)
//!
//! LAYER: DNA → PHYSICS → SOLVERS → ODE
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ ALGORITHM                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Forward Euler for dy/dt = f(t, y):                                          │
//! │                                                                             │
//! │   y_{n+1} = y_n + h·f(t_n, y_n)                                             │
//! │                                                                             │
//! │ Error: O(h²) per step, O(h) global                                          │
//! │ Stability: Conditionally stable (small timestep required)                   │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! REFERENCE: https://en.wikipedia.org/wiki/Euler_method
//!
//! ═══════════════════════════════════════════════════════════════════════════════

/// Forward Euler step: y_{n+1} = y_n + h·f(t, y)
pub fn euler_step<F>(y: f64, t: f64, h: f64, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    y + h * f(t, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_growth() {
        // dy/dt = y, solution y(t) = e^t
        let mut y = 1.0;
        let dt = 0.01;

        for _ in 0..100 {
            y = euler_step(y, 0.0, dt, |_, y_val| y_val);
        }

        // After t=1.0, should be close to e ≈ 2.718
        assert!((y - std::f64::consts::E).abs() < 0.1);
    }
}
