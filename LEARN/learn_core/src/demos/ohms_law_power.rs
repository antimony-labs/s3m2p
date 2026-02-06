//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: ohms_law_power.rs | LEARN/learn_core/src/demos/ohms_law_power.rs
//! PURPOSE: Ohm's Law + Power visualization (V = I × R, P = V × I)
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Demo, ParamMeta, Rng};

/// Ohm's Law + Power demo
#[derive(Clone)]
pub struct OhmsLawPowerDemo {
    /// Voltage (V)
    pub voltage: f32,
    /// Resistance (Ω)
    pub resistance: f32,
    /// Current (A) - calculated: I = V / R
    pub current: f32,
    /// Power (W) - calculated: P = V × I
    pub power: f32,
    /// Power limit warning threshold (W) - default 0.25W for common resistors
    pub power_limit: f32,
    /// Current limit warning threshold (A) - default 0.04A for ESP32 GPIO
    pub current_limit: f32,
    /// History for visualization
    pub voltage_history: Vec<f32>,
    pub current_history: Vec<f32>,
    pub power_history: Vec<f32>,
    history_len: usize,
    sample_timer: f32,
    sample_rate: f32,
    #[allow(dead_code)]
    rng: Rng,
}

impl Default for OhmsLawPowerDemo {
    fn default() -> Self {
        let mut demo = Self {
            voltage: 3.3,
            resistance: 1000.0,
            current: 0.0,
            power: 0.0,
            power_limit: 0.25,
            current_limit: 0.04,
            voltage_history: Vec::new(),
            current_history: Vec::new(),
            power_history: Vec::new(),
            history_len: 300,
            sample_timer: 0.0,
            sample_rate: 10.0, // 10 Hz for smooth visualization
            rng: Rng::new(42),
        };
        demo.recompute();
        demo
    }
}

impl OhmsLawPowerDemo {
    fn recompute(&mut self) {
        // Ohm's Law: I = V / R
        if self.resistance > 0.0 {
            self.current = self.voltage / self.resistance;
        } else {
            self.current = 0.0;
        }
        // Power: P = V × I
        self.power = self.voltage * self.current;
    }

    fn sample_once(&mut self) {
        self.voltage_history.push(self.voltage);
        self.current_history.push(self.current * 1000.0); // Convert to mA for display
        self.power_history.push(self.power * 1000.0); // Convert to mW for display

        // Keep history bounded
        if self.voltage_history.len() > self.history_len {
            self.voltage_history.remove(0);
            self.current_history.remove(0);
            self.power_history.remove(0);
        }
    }

    /// Check if current exceeds limit
    pub fn current_exceeds_limit(&self) -> bool {
        self.current > self.current_limit
    }

    /// Check if power exceeds limit
    pub fn power_exceeds_limit(&self) -> bool {
        self.power > self.power_limit
    }
}

impl Demo for OhmsLawPowerDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.voltage_history.clear();
        self.current_history.clear();
        self.power_history.clear();
        self.sample_timer = 0.0;
        self.recompute();
    }

    fn step(&mut self, dt: f32) {
        self.sample_timer += dt;
        let sample_interval = 1.0 / self.sample_rate;
        if self.sample_timer >= sample_interval {
            self.sample_timer -= sample_interval;
            self.sample_once();
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "voltage" => {
                self.voltage = value.clamp(0.0, 12.0);
                self.recompute();
                true
            }
            "resistance" => {
                self.resistance = value.clamp(10.0, 10000.0);
                self.recompute();
                true
            }
            "power_limit" => {
                self.power_limit = value.clamp(0.01, 1.0);
                true
            }
            "current_limit" => {
                self.current_limit = value.clamp(0.001, 0.2);
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "voltage",
                label: "Voltage (V)",
                min: 0.0,
                max: 12.0,
                step: 0.1,
                default: 3.3,
            },
            ParamMeta {
                name: "resistance",
                label: "Resistance (Ω)",
                min: 10.0,
                max: 10000.0,
                step: 10.0,
                default: 1000.0,
            },
        ]
    }
}
