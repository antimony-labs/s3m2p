//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: adc_reading.rs | LEARN/learn_core/src/demos/adc_reading.rs
//! PURPOSE: ADC sampling + quantization + noise/averaging intuition (ESP32 ADC)
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! The real world is analog (continuous voltage), but computers are digital.
//! An ADC converts voltage into an integer code:
//!
//!   code = round( (v / v_full_scale) * (2^bits - 1) )
//!
//! On ESP32 (ESP-WROOM-32), ADC behavior is nuanced:
//! - ADC resolution is typically 12-bit
//! - Input range depends on attenuation (0dB..11dB)
//! - Noise + non-linearity often motivate averaging / filtering
//!
//! This demo models:
//! - A smooth analog waveform (0..Vfs)
//! - Measurement noise
//! - Quantization to N bits
//! - A simple moving average filter
 
use crate::{Demo, ParamMeta, Rng};
 
/// ADC attenuation setting (conceptual, mapped to a full-scale voltage)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcAttenuation {
    Db0,
    Db2p5,
    Db6,
    Db11,
}
 
impl AdcAttenuation {
    pub fn from_index(idx: u8) -> Self {
        match idx {
            0 => Self::Db0,
            1 => Self::Db2p5,
            2 => Self::Db6,
            _ => Self::Db11,
        }
    }
 
    pub fn index(self) -> u8 {
        match self {
            Self::Db0 => 0,
            Self::Db2p5 => 1,
            Self::Db6 => 2,
            Self::Db11 => 3,
        }
    }
 
    /// Approximate full-scale voltage for teaching (not calibration-grade)
    pub fn full_scale_volts(self) -> f32 {
        match self {
            Self::Db0 => 1.1,
            Self::Db2p5 => 1.5,
            Self::Db6 => 2.2,
            Self::Db11 => 3.3,
        }
    }
}
 
/// ADC demo: analog signal -> samples -> quantized codes -> filtered output
#[derive(Clone)]
pub struct AdcReadingDemo {
    pub time: f32,
 
    /// Current true analog voltage (V)
    pub analog_v: f32,
 
    /// Current noisy sampled voltage (V)
    pub sampled_v: f32,
 
    /// Current ADC code (0..(2^bits-1))
    pub code: u16,
 
    /// Current quantized voltage (V)
    pub quantized_v: f32,
 
    /// Moving-average filtered voltage (V)
    pub filtered_v: f32,
 
    /// Parameters
    pub bits: u8,
    pub sample_rate: f32,
    pub noise_std_v: f32,
    pub avg_window: usize,
    pub attenuation: AdcAttenuation,
 
    /// History (for plotting)
    pub analog_history: Vec<f32>,
    pub sampled_history: Vec<f32>,
    pub quantized_history: Vec<f32>,
    pub filtered_history: Vec<f32>,
 
    history_len: usize,
    sample_timer: f32,
 
    // Moving average buffer
    avg_buf: Vec<f32>,
    avg_sum: f32,
 
    // Signal parameters
    signal_freq_hz: f32,
 
    rng: Rng,
}
 
impl Default for AdcReadingDemo {
    fn default() -> Self {
        Self {
            time: 0.0,
            analog_v: 0.0,
            sampled_v: 0.0,
            code: 0,
            quantized_v: 0.0,
            filtered_v: 0.0,
            bits: 12,
            sample_rate: 120.0,
            noise_std_v: 0.03,
            avg_window: 8,
            attenuation: AdcAttenuation::Db11,
            analog_history: Vec::new(),
            sampled_history: Vec::new(),
            quantized_history: Vec::new(),
            filtered_history: Vec::new(),
            history_len: 220,
            sample_timer: 0.0,
            avg_buf: Vec::new(),
            avg_sum: 0.0,
            signal_freq_hz: 0.35,
            rng: Rng::new(42),
        }
    }
}
 
impl AdcReadingDemo {
    pub fn v_full_scale(&self) -> f32 {
        self.attenuation.full_scale_volts()
    }
 
    fn levels(&self) -> u16 {
        let bits = self.bits.clamp(6, 12) as u32;
        ((1u32 << bits) - 1) as u16
    }
 
    fn true_signal(&self, t: f32) -> f32 {
        // Smooth signal in [0.05..0.95] of full-scale
        let vfs = self.v_full_scale();
        let s = 0.5
            + 0.42 * (std::f32::consts::TAU * self.signal_freq_hz * t).sin()
            + 0.06 * (std::f32::consts::TAU * (self.signal_freq_hz * 2.7) * t).cos();
        (s.clamp(0.05, 0.95)) * vfs
    }
 
    fn sample_once(&mut self, t: f32) {
        let vfs = self.v_full_scale();
        let true_v = self.true_signal(t);
        let noisy = (true_v + self.rng.normal_with(0.0, self.noise_std_v)).clamp(0.0, vfs);
 
        let levels = self.levels() as f32;
        let code_f = (noisy / vfs * levels).round().clamp(0.0, levels);
        let code = code_f as u16;
        let quant_v = (code as f32 / levels) * vfs;
 
        // Moving average
        self.avg_buf.push(quant_v);
        self.avg_sum += quant_v;
        while self.avg_buf.len() > self.avg_window.max(1) {
            if let Some(v) = self.avg_buf.first().copied() {
                self.avg_sum -= v;
            }
            self.avg_buf.remove(0);
        }
        let denom = self.avg_buf.len().max(1) as f32;
        let filtered = self.avg_sum / denom;
 
        // Update public fields
        self.analog_v = true_v;
        self.sampled_v = noisy;
        self.code = code;
        self.quantized_v = quant_v;
        self.filtered_v = filtered;
 
        // Record history
        self.analog_history.push(true_v);
        self.sampled_history.push(noisy);
        self.quantized_history.push(quant_v);
        self.filtered_history.push(filtered);
 
        while self.analog_history.len() > self.history_len {
            self.analog_history.remove(0);
            self.sampled_history.remove(0);
            self.quantized_history.remove(0);
            self.filtered_history.remove(0);
        }
    }
 
    fn reset_history(&mut self) {
        self.analog_history.clear();
        self.sampled_history.clear();
        self.quantized_history.clear();
        self.filtered_history.clear();
 
        self.analog_history.reserve(self.history_len);
        self.sampled_history.reserve(self.history_len);
        self.quantized_history.reserve(self.history_len);
        self.filtered_history.reserve(self.history_len);
 
        // Seed the graph with a flat line
        for _ in 0..self.history_len {
            self.analog_history.push(0.0);
            self.sampled_history.push(0.0);
            self.quantized_history.push(0.0);
            self.filtered_history.push(0.0);
        }
    }
}
 
impl Demo for AdcReadingDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.time = 0.0;
        self.sample_timer = 0.0;
 
        self.avg_buf.clear();
        self.avg_sum = 0.0;
 
        self.bits = self.bits.clamp(6, 12);
        self.sample_rate = self.sample_rate.clamp(5.0, 500.0);
        self.noise_std_v = self.noise_std_v.clamp(0.0, 0.2);
        self.avg_window = self.avg_window.clamp(1, 64);
 
        self.reset_history();
 
        // Prime one sample so fields look sensible immediately
        self.sample_once(0.0);
    }
 
    fn step(&mut self, dt: f32) {
        let dt = dt.max(0.0);
        self.time += dt;
 
        let interval = 1.0 / self.sample_rate.max(1.0);
        self.sample_timer += dt;
 
        while self.sample_timer >= interval {
            self.sample_timer -= interval;
            self.sample_once(self.time - self.sample_timer);
        }
    }
 
    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "bits" => {
                self.bits = (value.round() as i32).clamp(6, 12) as u8;
                true
            }
            "sample_rate" => {
                self.sample_rate = value.clamp(5.0, 500.0);
                true
            }
            "noise" => {
                self.noise_std_v = value.clamp(0.0, 0.2);
                true
            }
            "avg_window" => {
                self.avg_window = (value.round() as i32).clamp(1, 64) as usize;
                self.avg_buf.clear();
                self.avg_sum = 0.0;
                true
            }
            "attenuation" => {
                self.attenuation = AdcAttenuation::from_index((value.round() as i32).clamp(0, 3) as u8);
                true
            }
            _ => false,
        }
    }
 
    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "bits",
                label: "Resolution (bits)",
                min: 6.0,
                max: 12.0,
                step: 1.0,
                default: 12.0,
            },
            ParamMeta {
                name: "sample_rate",
                label: "Sample Rate (Hz)",
                min: 5.0,
                max: 500.0,
                step: 5.0,
                default: 120.0,
            },
            ParamMeta {
                name: "noise",
                label: "Noise (V)",
                min: 0.0,
                max: 0.2,
                step: 0.01,
                default: 0.03,
            },
            ParamMeta {
                name: "avg_window",
                label: "Avg Window (samples)",
                min: 1.0,
                max: 64.0,
                step: 1.0,
                default: 8.0,
            },
            ParamMeta {
                name: "attenuation",
                label: "Attenuation (0..3)",
                min: 0.0,
                max: 3.0,
                step: 1.0,
                default: 3.0,
            },
        ]
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_deterministic_for_same_seed() {
        let mut a = AdcReadingDemo::default();
        let mut b = AdcReadingDemo::default();
        a.reset(123);
        b.reset(123);
 
        for _ in 0..200 {
            a.step(0.01);
            b.step(0.01);
        }
 
        assert_eq!(a.code, b.code);
        assert!((a.quantized_v - b.quantized_v).abs() < 1e-6);
    }
 
    #[test]
    fn test_code_within_range() {
        let mut d = AdcReadingDemo::default();
        d.reset(42);
        for _ in 0..200 {
            d.step(0.02);
            assert!(d.code <= d.levels());
        }
    }
}
