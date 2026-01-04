//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: rc_time_constant.rs | LEARN/learn_core/src/demos/rc_time_constant.rs
//! PURPOSE: RC charging curve visualization (τ = R × C)
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Demo, ParamMeta, Rng};

/// RC Time Constant demo
#[derive(Clone)]
pub struct RcTimeConstantDemo {
    /// Resistance (Ω)
    pub resistance: f32,
    /// Capacitance (µF)
    pub capacitance: f32,
    /// Time constant τ = R × C (seconds)
    pub tau: f32,
    /// Final voltage (V)
    pub v_final: f32,
    /// Current voltage (V) - charging curve
    pub voltage: f32,
    /// Time since charge started (s)
    pub time: f32,
    /// History for visualization
    pub voltage_history: Vec<f32>,
    pub time_history: Vec<f32>,
    history_len: usize,
    charging: bool,
    #[allow(dead_code)]
    rng: Rng,
}

impl Default for RcTimeConstantDemo {
    fn default() -> Self {
        let mut demo = Self {
            resistance: 10000.0,
            capacitance: 100.0, // µF
            tau: 1.0,
            v_final: 3.3,
            voltage: 0.0,
            time: 0.0,
            voltage_history: Vec::new(),
            time_history: Vec::new(),
            history_len: 300,
            charging: true,
            rng: Rng::new(42),
        };
        demo.recompute();
        demo
    }
}

impl RcTimeConstantDemo {
    fn recompute(&mut self) {
        // τ = R × C (convert µF to F)
        self.tau = self.resistance * (self.capacitance / 1_000_000.0);
    }

    fn voltage_at_time(&self, t: f32) -> f32 {
        if self.tau <= 0.0 {
            return self.v_final;
        }
        // V(t) = V_final × (1 - e^(-t/τ))
        self.v_final * (1.0 - (-t / self.tau).exp())
    }
}

impl Demo for RcTimeConstantDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.voltage_history.clear();
        self.time_history.clear();
        self.time = 0.0;
        self.voltage = 0.0;
        self.charging = true;
        self.recompute();
    }

    fn step(&mut self, dt: f32) {
        if self.charging {
            self.time += dt;
            self.voltage = self.voltage_at_time(self.time);
            
            self.voltage_history.push(self.voltage);
            self.time_history.push(self.time);

            // Keep history bounded
            if self.voltage_history.len() > self.history_len {
                self.voltage_history.remove(0);
                self.time_history.remove(0);
            }

            // Reset when fully charged (99% of final)
            if self.voltage >= self.v_final * 0.99 {
                self.charging = false;
                // Auto-reset after a short delay
                if self.time > self.tau * 6.0 {
                    let seed = self.rng.next_u64();
                    self.reset(seed);
                }
            }
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "resistance" => {
                self.resistance = value.clamp(1000.0, 100000.0);
                self.recompute();
                true
            }
            "capacitance" => {
                self.capacitance = value.clamp(1.0, 1000.0);
                self.recompute();
                true
            }
            "v_final" => {
                self.v_final = value.clamp(1.0, 5.0);
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "resistance",
                label: "Resistance (Ω)",
                min: 1000.0,
                max: 100000.0,
                step: 1000.0,
                default: 10000.0,
            },
            ParamMeta {
                name: "capacitance",
                label: "Capacitance (µF)",
                min: 1.0,
                max: 1000.0,
                step: 1.0,
                default: 100.0,
            },
        ]
    }
}

