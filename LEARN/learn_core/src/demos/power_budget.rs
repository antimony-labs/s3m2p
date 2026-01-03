//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: power_budget.rs | LEARN/learn_core/src/demos/power_budget.rs
//! PURPOSE: Power budget calculator (battery lifetime estimation)
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Demo, ParamMeta, Rng};

/// Power Budget demo
#[derive(Clone)]
pub struct PowerBudgetDemo {
    /// Active current (mA)
    pub active_current: f32,
    /// Active time per cycle (s)
    pub active_time: f32,
    /// Sleep current (µA)
    pub sleep_current: f32,
    /// Sleep time per cycle (s)
    pub sleep_time: f32,
    /// Battery capacity (mAh)
    pub battery_capacity: f32,
    /// Energy per cycle (mAs)
    pub energy_per_cycle: f32,
    /// Number of cycles
    pub cycles: f32,
    /// Battery lifetime (days)
    pub lifetime_days: f32,
    /// Cycle time (s)
    pub cycle_time: f32,
    /// History for visualization
    pub energy_history: Vec<f32>,
    pub cycle_history: Vec<f32>,
    history_len: usize,
    sample_timer: f32,
    sample_rate: f32,
    #[allow(dead_code)]
    rng: Rng,
}

impl Default for PowerBudgetDemo {
    fn default() -> Self {
        let mut demo = Self {
            active_current: 80.0,
            active_time: 3.0,
            sleep_current: 10.0,
            sleep_time: 297.0,
            battery_capacity: 2000.0,
            energy_per_cycle: 0.0,
            cycles: 0.0,
            lifetime_days: 0.0,
            cycle_time: 300.0,
            energy_history: Vec::new(),
            cycle_history: Vec::new(),
            history_len: 300,
            sample_timer: 0.0,
            sample_rate: 1.0,
            rng: Rng::new(42),
        };
        demo.recompute();
        demo
    }
}

impl PowerBudgetDemo {
    fn recompute(&mut self) {
        // Energy per cycle = (I_active × t_active) + (I_sleep × t_sleep)
        // Convert sleep current from µA to mA
        let sleep_current_ma = self.sleep_current / 1000.0;
        self.energy_per_cycle = (self.active_current * self.active_time) + (sleep_current_ma * self.sleep_time);
        
        // Cycle time
        self.cycle_time = self.active_time + self.sleep_time;
        
        // Battery capacity in mAs
        let capacity_mas = self.battery_capacity * 3600.0;
        
        // Number of cycles
        if self.energy_per_cycle > 0.0 {
            self.cycles = capacity_mas / self.energy_per_cycle;
        } else {
            self.cycles = 0.0;
        }
        
        // Lifetime in days
        if self.cycle_time > 0.0 {
            let lifetime_seconds = self.cycles * self.cycle_time;
            self.lifetime_days = lifetime_seconds / 86400.0;
        } else {
            self.lifetime_days = 0.0;
        }
    }

    fn sample_once(&mut self) {
        self.energy_history.push(self.energy_per_cycle);
        self.cycle_history.push(self.cycles);

        // Keep history bounded
        if self.energy_history.len() > self.history_len {
            self.energy_history.remove(0);
            self.cycle_history.remove(0);
        }
    }
}

impl Demo for PowerBudgetDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.energy_history.clear();
        self.cycle_history.clear();
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
            "active_current" => {
                self.active_current = value.clamp(10.0, 200.0);
                self.recompute();
                true
            }
            "active_time" => {
                self.active_time = value.clamp(0.5, 10.0);
                self.recompute();
                true
            }
            "sleep_current" => {
                self.sleep_current = value.clamp(1.0, 100.0);
                self.recompute();
                true
            }
            "sleep_time" => {
                self.sleep_time = value.clamp(10.0, 600.0);
                self.recompute();
                true
            }
            "battery_capacity" => {
                self.battery_capacity = value.clamp(100.0, 10000.0);
                self.recompute();
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "active_current",
                label: "Active Current (mA)",
                min: 10.0,
                max: 200.0,
                step: 5.0,
                default: 80.0,
            },
            ParamMeta {
                name: "active_time",
                label: "Active Time (s)",
                min: 0.5,
                max: 10.0,
                step: 0.5,
                default: 3.0,
            },
            ParamMeta {
                name: "sleep_current",
                label: "Sleep Current (µA)",
                min: 1.0,
                max: 100.0,
                step: 1.0,
                default: 10.0,
            },
            ParamMeta {
                name: "sleep_time",
                label: "Sleep Time (s)",
                min: 10.0,
                max: 600.0,
                step: 1.0,
                default: 297.0,
            },
        ]
    }
}

