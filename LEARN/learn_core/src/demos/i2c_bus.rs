//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: i2c_bus.rs | LEARN/learn_core/src/demos/i2c_bus.rs
//! PURPOSE: I2C bus transaction waveform simulation (SDA/SCL + ACK/NAK)
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! I2C is a two-wire bus:
//! - SDA: data (open-drain)
//! - SCL: clock (open-drain)
//!
//! Idle state: both lines HIGH (via pull-ups).
//! START: SDA falls while SCL is HIGH.
//! STOP:  SDA rises while SCL is HIGH.
//!
//! Bits are valid while SCL is HIGH; SDA changes only while SCL is LOW.
//! After every 8 bits, the receiver transmits an ACK bit:
//! - ACK: SDA LOW
//! - NACK: SDA HIGH
//!
//! This demo intentionally visualizes I2C at a slowed-down timescale.
 
use crate::{Demo, ParamMeta, Rng};
 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPhase {
    Idle,
    Start,
    Bits,
    Ack,
    Stop,
}
 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cStage {
    /// Address byte (7-bit address + R/W)
    Address,
    /// Data byte written by master
    WriteData,
    /// Data byte read from slave
    ReadData,
}
 
/// I2C bus demo for visualizing SDA/SCL + start/stop + ACK/NAK
#[derive(Clone)]
pub struct I2cBusDemo {
    pub time: f32,
 
    /// Live line states
    pub scl: bool,
    pub sda: bool,
 
    /// History for plotting
    pub scl_history: Vec<bool>,
    pub sda_history: Vec<bool>,
 
    pub phase: I2cPhase,
    pub stage: I2cStage,
 
    /// Parameters
    pub address: u8,       // 7-bit
    pub rw: bool,          // false=write, true=read
    pub clock_khz: f32,    // conceptual (visualized); treated as Hz internally
    pub nak_chance: f32,   // probability of NACK at ACK phases (address/data write)
    pub stretch_chance: f32, // probability of extra-long SCL low phase
 
    /// Current byte being transferred (address byte or data byte)
    pub current_byte: u8,
 
    /// Bit index within current byte (0..7, MSB first)
    pub bit_index: u8,
 
    /// Last ACK result (true=ACK, false=NACK)
    pub ack: bool,
 
    pub transactions: u32,
 
    history_len: usize,
    sample_rate: f32,
    sample_timer: f32,
 
    // Half-period timing (seconds)
    half_period: f32,
    phase_timer: f32,
 
    // Idle delay between transactions (seconds)
    idle_timer: f32,
 
    // Stop sequence step
    stop_step: u8,
 
    rng: Rng,
}
 
impl Default for I2cBusDemo {
    fn default() -> Self {
        let mut demo = Self {
            time: 0.0,
            scl: true,
            sda: true,
            scl_history: Vec::new(),
            sda_history: Vec::new(),
            phase: I2cPhase::Idle,
            stage: I2cStage::Address,
            address: 0x3C,
            rw: false,
            clock_khz: 100.0,
            nak_chance: 0.0,
            stretch_chance: 0.0,
            current_byte: 0,
            bit_index: 0,
            ack: true,
            transactions: 0,
            history_len: 320,
            sample_rate: 2000.0,
            sample_timer: 0.0,
            half_period: 0.005,
            phase_timer: 0.0,
            idle_timer: 0.25,
            stop_step: 0,
            rng: Rng::new(42),
        };
        demo.recompute_timing();
        demo
    }
}
 
impl I2cBusDemo {
    fn recompute_timing(&mut self) {
        // Visualized kHz -> internally treated as Hz (slowed down so you can see it)
        let hz = self.clock_khz.clamp(10.0, 400.0);
        self.half_period = 0.5 / hz;
        self.sample_rate = (hz * 40.0).clamp(300.0, 8000.0);
    }
 
    fn bit_of(byte: u8, bit_index: u8) -> bool {
        // MSB first
        let shift = 7u8.saturating_sub(bit_index);
        ((byte >> shift) & 1) != 0
    }
 
    fn begin_start(&mut self) {
        self.phase = I2cPhase::Start;
        self.stage = I2cStage::Address;
        self.scl = true;
        self.sda = false; // START (SDA falls while SCL high)
        self.phase_timer = self.half_period;
    }
 
    fn begin_bits(&mut self, stage: I2cStage) {
        self.phase = I2cPhase::Bits;
        self.stage = stage;
        self.bit_index = 0;
        self.ack = true;
 
        self.current_byte = match stage {
            I2cStage::Address => (self.address << 1) | (self.rw as u8),
            I2cStage::WriteData => 0xA5, // arbitrary data byte
            I2cStage::ReadData => (self.rng.next_u32() & 0xFF) as u8, // slave returns a byte
        };
 
        // After START, SCL goes low and first bit is placed on SDA
        self.scl = false;
        self.sda = Self::bit_of(self.current_byte, self.bit_index);
        self.phase_timer = self.next_low_duration();
    }
 
    fn begin_ack(&mut self) {
        self.phase = I2cPhase::Ack;
        self.bit_index = 0;
 
        // Who sends ACK depends on direction, but we just visualize the bit on SDA.
        let ack_bit_is_low = match self.stage {
            I2cStage::ReadData => {
                // Master ACK/NACK after reading a byte; NACK to end transfer
                false
            }
            _ => {
                // Receiver ACKs unless we inject NACK
                !self.rng.chance(self.nak_chance.clamp(0.0, 1.0))
            }
        };
 
        self.ack = ack_bit_is_low;
        self.sda = !ack_bit_is_low; // SDA low => ACK, high => NACK
        // SCL should currently be LOW (we enter ACK after a falling edge)
        self.scl = false;
        self.phase_timer = self.next_low_duration();
    }
 
    fn begin_stop(&mut self) {
        self.phase = I2cPhase::Stop;
        self.stop_step = 0;
        // Ensure we're low before generating STOP
        self.scl = false;
        self.sda = false;
        self.phase_timer = self.half_period;
    }
 
    fn next_low_duration(&mut self) -> f32 {
        let base = self.half_period.max(1e-4);
        if self.stretch_chance > 0.0 && self.rng.chance(self.stretch_chance.clamp(0.0, 1.0)) {
            base * 3.0
        } else {
            base
        }
    }
 
    fn on_half_period_end(&mut self) {
        match self.phase {
            I2cPhase::Start => {
                // Move into first byte bits (address)
                self.begin_bits(I2cStage::Address);
            }
            I2cPhase::Bits => {
                if !self.scl {
                    // Rising edge: data is sampled while SCL high
                    self.scl = true;
                    self.phase_timer = self.half_period;
                } else {
                    // Falling edge: advance bit index and prepare next bit (while SCL low)
                    self.scl = false;
                    self.bit_index += 1;
 
                    if self.bit_index < 8 {
                        self.sda = Self::bit_of(self.current_byte, self.bit_index);
                        self.phase_timer = self.next_low_duration();
                    } else {
                        // Completed 8 bits -> ACK phase
                        self.begin_ack();
                    }
                }
            }
            I2cPhase::Ack => {
                if !self.scl {
                    // Rising edge: ACK/NACK sampled
                    self.scl = true;
                    self.phase_timer = self.half_period;
                } else {
                    // Falling edge: decide next stage
                    self.scl = false;
 
                    match self.stage {
                        I2cStage::Address => {
                            if !self.ack {
                                self.begin_stop();
                            } else if self.rw {
                                self.begin_bits(I2cStage::ReadData);
                            } else {
                                self.begin_bits(I2cStage::WriteData);
                            }
                        }
                        I2cStage::WriteData => {
                            // After one data byte, stop
                            self.begin_stop();
                        }
                        I2cStage::ReadData => {
                            // We NACK to end read, then STOP
                            self.begin_stop();
                        }
                    }
                }
            }
            I2cPhase::Stop => {
                match self.stop_step {
                    0 => {
                        // Raise SCL while SDA low
                        self.scl = true;
                        self.sda = false;
                        self.stop_step = 1;
                        self.phase_timer = self.half_period;
                    }
                    _ => {
                        // Raise SDA while SCL high (STOP)
                        self.scl = true;
                        self.sda = true;
 
                        self.transactions += 1;
                        self.phase = I2cPhase::Idle;
                        self.idle_timer = 0.35;
                        self.phase_timer = 0.0;
                    }
                }
            }
            I2cPhase::Idle => { /* handled elsewhere */ }
        }
    }
 
    fn advance(&mut self, dt: f32) {
        let mut remaining = dt;
        while remaining > 0.0 {
            // Idle: wait a bit, then start another transaction
            if self.phase == I2cPhase::Idle {
                self.scl = true;
                self.sda = true;
                self.idle_timer -= remaining;
                if self.idle_timer <= 0.0 {
                    self.begin_start();
                }
                break;
            }
 
            // Prevent infinite loops if timing ever hits 0
            if self.phase_timer <= 0.0 {
                break;
            }
 
            if self.phase_timer > remaining {
                self.phase_timer -= remaining;
                remaining = 0.0;
            } else {
                remaining -= self.phase_timer;
                self.phase_timer = 0.0;
                self.on_half_period_end();
            }
        }
    }
 
    fn record_sample(&mut self) {
        self.scl_history.push(self.scl);
        self.sda_history.push(self.sda);
 
        while self.scl_history.len() > self.history_len {
            self.scl_history.remove(0);
        }
        while self.sda_history.len() > self.history_len {
            self.sda_history.remove(0);
        }
    }
}
 
impl Demo for I2cBusDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.time = 0.0;
        self.sample_timer = 0.0;
        self.phase_timer = 0.0;
        self.transactions = 0;
 
        self.address = self.address.clamp(0x08, 0x77);
        self.clock_khz = self.clock_khz.clamp(10.0, 400.0);
        self.nak_chance = self.nak_chance.clamp(0.0, 1.0);
        self.stretch_chance = self.stretch_chance.clamp(0.0, 1.0);
        self.recompute_timing();
 
        self.phase = I2cPhase::Idle;
        self.stage = I2cStage::Address;
        self.scl = true;
        self.sda = true;
        self.idle_timer = 0.2;
        self.stop_step = 0;
 
        self.scl_history.clear();
        self.sda_history.clear();
        self.scl_history.reserve(self.history_len);
        self.sda_history.reserve(self.history_len);
 
        for _ in 0..self.history_len {
            self.scl_history.push(true);
            self.sda_history.push(true);
        }
    }
 
    fn step(&mut self, dt: f32) {
        let dt = dt.max(0.0);
        self.time += dt;
 
        let interval = 1.0 / self.sample_rate.max(1.0);
        self.sample_timer += dt;
 
        while self.sample_timer >= interval {
            self.sample_timer -= interval;
            self.advance(interval);
            self.record_sample();
        }
    }
 
    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "address" => {
                self.address = (value.round() as i32).clamp(0x08, 0x77) as u8;
                true
            }
            "rw" => {
                self.rw = value >= 0.5;
                true
            }
            "clock_khz" => {
                self.clock_khz = value.clamp(10.0, 400.0);
                self.recompute_timing();
                true
            }
            "nak_chance" => {
                self.nak_chance = value.clamp(0.0, 1.0);
                true
            }
            "stretch_chance" => {
                self.stretch_chance = value.clamp(0.0, 1.0);
                true
            }
            _ => false,
        }
    }
 
    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "address",
                label: "Address (7-bit)",
                min: 8.0,
                max: 119.0,
                step: 1.0,
                default: 0x3C as f32,
            },
            ParamMeta {
                name: "rw",
                label: "R/W (0=write, 1=read)",
                min: 0.0,
                max: 1.0,
                step: 1.0,
                default: 0.0,
            },
            ParamMeta {
                name: "clock_khz",
                label: "Clock (kHz, slowed)",
                min: 10.0,
                max: 400.0,
                step: 10.0,
                default: 100.0,
            },
            ParamMeta {
                name: "nak_chance",
                label: "NAK chance",
                min: 0.0,
                max: 1.0,
                step: 0.05,
                default: 0.0,
            },
            ParamMeta {
                name: "stretch_chance",
                label: "Clock stretch chance",
                min: 0.0,
                max: 1.0,
                step: 0.05,
                default: 0.0,
            },
        ]
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_deterministic_for_same_seed() {
        let mut a = I2cBusDemo::default();
        let mut b = I2cBusDemo::default();
        a.nak_chance = 0.3;
        b.nak_chance = 0.3;
        a.stretch_chance = 0.2;
        b.stretch_chance = 0.2;
        a.reset(42);
        b.reset(42);
 
        for _ in 0..200 {
            a.step(0.02);
            b.step(0.02);
        }
 
        assert_eq!(a.scl_history, b.scl_history);
        assert_eq!(a.sda_history, b.sda_history);
        assert_eq!(a.transactions, b.transactions);
    }
 
    #[test]
    fn test_address_clamped() {
        let mut d = I2cBusDemo::default();
        d.set_param("address", 1.0);
        assert_eq!(d.address, 0x08);
        d.set_param("address", 200.0);
        assert_eq!(d.address, 0x77);
    }
}
