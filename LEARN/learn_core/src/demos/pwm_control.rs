//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: pwm_control.rs | LEARN/learn_core/src/demos/pwm_control.rs
//! PURPOSE: PWM waveform + average power simulation (ESP32 LEDC intuition)
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PWM (Pulse Width Modulation) is "digital" on the wire (HIGH/LOW),
//! but "analog-ish" after a load integrates it (LED persistence, motor inertia, RC filter).
//!
//! This demo models:
//! - A square wave with configurable duty cycle and frequency
//! - Quantization of duty based on a timer resolution (bits)
//! - A simple first-order low-pass that approximates averaged output
//!
//! It is intentionally conceptual (units are correct, but it's not a full electrical model).
//! Perfect for teaching ESP32 LEDC tradeoffs: duty resolution vs frequency.
 
use crate::{Demo, ParamMeta, Rng};
 
/// PWM demo showing raw waveform vs averaged output
#[derive(Clone)]
pub struct PwmControlDemo {
    /// Wall-clock simulation time (seconds)
    pub time: f32,
 
    /// Duty cycle (0.0..=1.0)
    pub duty: f32,
 
    /// PWM frequency in Hz (visualized; educational scale)
    pub frequency: f32,
 
    /// Timer resolution in bits (1..=15 typical for ESP32 LEDC)
    pub resolution_bits: u8,
 
    /// Duty after quantization to the current resolution (0.0..=1.0)
    pub quantized_duty: f32,
 
    /// Current digital output state (after quantization)
    pub output_high: bool,
 
    /// Low-pass filtered output (0.0..=1.0) ~ perceived brightness / average power
    pub avg: f32,
 
    /// Recent digital history (for waveform plot)
    pub raw_history: Vec<bool>,
 
    /// Recent averaged history
    pub avg_history: Vec<f32>,
 
    history_len: usize,
    sample_timer: f32,
    sample_time: f32,
    sample_rate: f32,
    filter_tau: f32,
 
    #[allow(dead_code)]
    rng: Rng,
}
 
impl Default for PwmControlDemo {
    fn default() -> Self {
        let mut demo = Self {
            time: 0.0,
            duty: 0.5,
            frequency: 500.0,
            resolution_bits: 8,
            quantized_duty: 0.5,
            output_high: false,
            avg: 0.0,
            raw_history: Vec::new(),
            avg_history: Vec::new(),
            history_len: 300,
            sample_timer: 0.0,
            sample_time: 0.0,
            sample_rate: 2000.0,
            filter_tau: 0.03, // ~ LED persistence / RC smoothing (seconds)
            rng: Rng::new(42),
        };
        demo.recompute();
        demo
    }
}
 
impl PwmControlDemo {
    fn recompute(&mut self) {
        self.quantized_duty = quantize_duty(self.duty, self.resolution_bits);
        self.sample_rate = compute_sample_rate(self.frequency);
    }
 
    fn pwm_at(&self, t: f32) -> bool {
        let freq = self.frequency.max(1.0);
        let period = 1.0 / freq;
        let phase = (t / period).fract(); // 0..1
        phase < self.quantized_duty
    }
 
    fn sample_once(&mut self, dt: f32) {
        let high = self.pwm_at(self.sample_time);
        self.output_high = high;
        let target = if high { 1.0 } else { 0.0 };
 
        // First-order low-pass: avg += (target - avg) * alpha
        let alpha = (dt / self.filter_tau.max(1e-3)).clamp(0.0, 1.0);
        self.avg += (target - self.avg) * alpha;
 
        self.raw_history.push(high);
        self.avg_history.push(self.avg);
 
        while self.raw_history.len() > self.history_len {
            self.raw_history.remove(0);
        }
        while self.avg_history.len() > self.history_len {
            self.avg_history.remove(0);
        }
    }
}
 
impl Demo for PwmControlDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.time = 0.0;
        self.sample_time = 0.0;
        self.sample_timer = 0.0;
        self.output_high = false;
        self.avg = 0.0;
 
        self.recompute();
 
        self.raw_history.clear();
        self.avg_history.clear();
        self.raw_history.reserve(self.history_len);
        self.avg_history.reserve(self.history_len);
 
        for _ in 0..self.history_len {
            self.raw_history.push(false);
            self.avg_history.push(0.0);
        }
    }
 
    fn step(&mut self, dt: f32) {
        self.time += dt.max(0.0);
 
        let sample_interval = 1.0 / self.sample_rate.max(1.0);
        self.sample_timer += dt;
 
        while self.sample_timer >= sample_interval {
            self.sample_timer -= sample_interval;
            self.sample_time += sample_interval;
            self.sample_once(sample_interval);
        }
    }
 
    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "duty" => {
                self.duty = value.clamp(0.0, 1.0);
                self.recompute();
                true
            }
            "frequency" => {
                self.frequency = value.clamp(10.0, 2000.0);
                self.recompute();
                true
            }
            "resolution_bits" => {
                self.resolution_bits = (value.round() as i32).clamp(1, 15) as u8;
                self.recompute();
                true
            }
            "filter_tau" => {
                self.filter_tau = value.clamp(0.005, 0.2);
                true
            }
            _ => false,
        }
    }
 
    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "duty",
                label: "Duty (0..1)",
                min: 0.0,
                max: 1.0,
                step: 0.01,
                default: 0.5,
            },
            ParamMeta {
                name: "frequency",
                label: "Frequency (Hz)",
                min: 10.0,
                max: 2000.0,
                step: 10.0,
                default: 500.0,
            },
            ParamMeta {
                name: "resolution_bits",
                label: "Resolution (bits)",
                min: 1.0,
                max: 15.0,
                step: 1.0,
                default: 8.0,
            },
            ParamMeta {
                name: "filter_tau",
                label: "Smoothing τ (s)",
                min: 0.005,
                max: 0.2,
                step: 0.005,
                default: 0.03,
            },
        ]
    }
}
 
fn quantize_duty(duty: f32, bits: u8) -> f32 {
    let duty = duty.clamp(0.0, 1.0);
    let bits = bits.clamp(1, 16);
    let steps = (1u32 << bits) as f32;
    let max_code = steps - 1.0;
    if max_code <= 0.0 {
        return duty;
    }
    (duty * max_code).round() / max_code
}
 
fn compute_sample_rate(freq_hz: f32) -> f32 {
    // Keep the waveform readable across frequencies:
    // aim for ~40 samples per period, but clamp to reasonable bounds.
    let freq = freq_hz.clamp(10.0, 2000.0);
    (freq * 40.0).clamp(200.0, 8000.0)
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_quantize_duty_rounds_to_steps() {
        // 2-bit resolution => 4 steps => codes 0..3 => duty in {0, 1/3, 2/3, 1}
        let q = quantize_duty(0.33, 2);
        assert!((q - (1.0 / 3.0)).abs() < 0.02, "q={}", q);
 
        let q2 = quantize_duty(0.9, 2);
        assert!((q2 - 1.0).abs() < 1e-6, "q2={}", q2);
    }
 
    #[test]
    fn test_history_is_bounded() {
        let mut demo = PwmControlDemo::default();
        demo.reset(42);
        for _ in 0..200 {
            demo.step(0.05);
        }
        assert_eq!(demo.raw_history.len(), 300);
        assert_eq!(demo.avg_history.len(), 300);
    }
}
