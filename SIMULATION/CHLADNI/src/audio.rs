//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: audio.rs | SIMULATION/CHLADNI/src/audio.rs
//! PURPOSE: Audio input processing and frequency analysis for Chladni simulation
//! MODIFIED: 2025-12-14
//! LAYER: SIMULATION → CHLADNI
//! ═══════════════════════════════════════════════════════════════════════════════

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AnalyserNode, AudioContext, MediaStreamConstraints};

/// Audio analyzer for microphone input
pub struct AudioAnalyzer {
    audio_context: AudioContext,
    analyser: AnalyserNode,
    source: Option<web_sys::MediaStreamAudioSourceNode>,
    fft_buffer: Vec<f32>,
    /// Time-domain buffer for RMS calculation
    time_buffer: Vec<f32>,
    sample_rate: f32,
    is_active: bool,
}

impl AudioAnalyzer {
    /// Create a new audio analyzer
    pub fn new() -> Result<Self, JsValue> {
        let audio_context = AudioContext::new()?;

        // Create analyser node using create_analyser
        let analyser = audio_context.create_analyser()?;

        // Configure analyser for frequency analysis
        analyser.set_fft_size(2048);
        analyser.set_smoothing_time_constant(0.8);

        let sample_rate = audio_context.sample_rate();

        Ok(Self {
            audio_context,
            analyser,
            source: None,
            fft_buffer: vec![0.0; 1024],  // Half of FFT size
            time_buffer: vec![0.0; 2048], // Full FFT size for time domain
            sample_rate,
            is_active: false,
        })
    }

    /// Start capturing audio from microphone
    pub async fn start_microphone(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
        let navigator = window.navigator();
        let media_devices = navigator
            .media_devices()
            .map_err(|_| JsValue::from_str("No media devices"))?;

        // Request microphone access
        let constraints = MediaStreamConstraints::new();
        constraints.set_audio(&JsValue::TRUE);

        let promise = media_devices.get_user_media_with_constraints(&constraints)?;
        let stream = wasm_bindgen_futures::JsFuture::from(promise).await?;
        let stream = stream.dyn_into::<web_sys::MediaStream>()?;

        // Create audio source from stream using JsCast
        // Note: create_media_stream_source returns a MediaStreamAudioSourceNode
        let source_obj = self
            .audio_context
            .create_media_stream_source(&stream)
            .map_err(|e| JsValue::from_str(&format!("Failed to create audio source: {:?}", e)))?;
        let source: web_sys::MediaStreamAudioSourceNode = source_obj
            .dyn_into()
            .map_err(|_| JsValue::from_str("Failed to cast to MediaStreamAudioSourceNode"))?;
        source
            .connect_with_audio_node(&self.analyser)
            .map_err(|e| JsValue::from_str(&format!("Failed to connect audio: {:?}", e)))?;

        self.source = Some(source);
        self.is_active = true;
        Ok(())
    }

    /// Stop audio capture
    pub fn stop(&mut self) {
        if let Some(source) = &self.source {
            let _ = source.disconnect();
        }
        self.source = None;
        self.is_active = false;
    }

    /// Check if audio is active
    pub fn is_active(&self) -> bool {
        self.is_active && self.source.is_some()
    }

    /// Get the current dominant frequency from audio input
    pub fn get_dominant_frequency(&mut self) -> Option<f32> {
        if !self.is_active() {
            return None;
        }

        // Get frequency data - ensure buffer is the right size
        if self.fft_buffer.len() != 1024 {
            self.fft_buffer.resize(1024, 0.0);
        }
        self.analyser.get_float_frequency_data(&mut self.fft_buffer);

        // Find peak frequency
        let mut max_magnitude = f32::NEG_INFINITY;
        let mut peak_bin = 0;

        for (i, &magnitude) in self.fft_buffer.iter().enumerate() {
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
                peak_bin = i;
            }
        }

        // Convert bin to frequency
        // Frequency = (bin_index * sample_rate) / fft_size
        let frequency = (peak_bin as f32 * self.sample_rate) / 2048.0;

        // Only return if magnitude is significant (above noise floor)
        if max_magnitude > -60.0 {
            Some(frequency)
        } else {
            None
        }
    }

    /// Get frequency spectrum for visualization
    pub fn get_frequency_spectrum(&mut self) -> Vec<f32> {
        if !self.is_active() {
            return vec![0.0; 1024];
        }

        // Ensure buffer is the right size
        if self.fft_buffer.len() != 1024 {
            self.fft_buffer.resize(1024, 0.0);
        }
        self.analyser.get_float_frequency_data(&mut self.fft_buffer);
        self.fft_buffer.clone()
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Calculate RMS (root mean square) of the audio signal.
    /// Returns a normalized value in 0.0-1.0 range.
    pub fn get_rms(&mut self) -> f32 {
        if !self.is_active() {
            return 0.0;
        }

        // Get time-domain data
        self.analyser
            .get_float_time_domain_data(&mut self.time_buffer);

        // Calculate RMS
        let sum_squares: f32 = self.time_buffer.iter().map(|&x| x * x).sum();
        let rms = (sum_squares / self.time_buffer.len() as f32).sqrt();

        // Normalize to 0-1 range (time domain data is typically in -1 to 1)
        // RMS of a full-scale sine is ~0.707, so we scale accordingly
        (rms * 1.414).min(1.0)
    }

    /// Get energy in multiple frequency bands for driven simulation.
    ///
    /// Returns [sub_bass, bass, mid, high] energies, each normalized 0-1.
    ///
    /// Band definitions:
    /// - Sub-bass: 20-80 Hz
    /// - Bass: 80-250 Hz
    /// - Mid: 250-2000 Hz
    /// - High: 2000-8000 Hz
    pub fn get_band_energies(&mut self) -> [f32; 4] {
        if !self.is_active() {
            return [0.0; 4];
        }

        // Get frequency data
        self.analyser.get_float_frequency_data(&mut self.fft_buffer);

        // Frequency resolution: sample_rate / fft_size
        // With 48kHz and 2048 FFT: each bin = ~23.4 Hz
        let bin_hz = self.sample_rate / 2048.0;

        // Convert frequency to bin index
        let freq_to_bin =
            |freq: f32| -> usize { ((freq / bin_hz) as usize).min(self.fft_buffer.len() - 1) };

        // Band boundaries in bins
        let sub_bass_start = freq_to_bin(20.0);
        let sub_bass_end = freq_to_bin(80.0);
        let bass_end = freq_to_bin(250.0);
        let mid_end = freq_to_bin(2000.0);
        let high_end = freq_to_bin(8000.0);

        // Calculate average energy in each band (dB values)
        let band_energy = |start: usize, end: usize| -> f32 {
            if end <= start {
                return 0.0;
            }
            let sum: f32 = self.fft_buffer[start..end]
                .iter()
                .map(|&db| {
                    // Convert dB to linear, clamp to noise floor
                    let db_clamped = db.max(-100.0);
                    10.0f32.powf(db_clamped / 20.0)
                })
                .sum();
            sum / (end - start) as f32
        };

        let sub_bass = band_energy(sub_bass_start, sub_bass_end);
        let bass = band_energy(sub_bass_end, bass_end);
        let mid = band_energy(bass_end, mid_end);
        let high = band_energy(mid_end, high_end);

        // Normalize (typical speech/music values, can tune)
        let normalize = |v: f32, scale: f32| -> f32 { (v * scale).min(1.0) };

        [
            normalize(sub_bass, 50.0), // Sub-bass often weaker
            normalize(bass, 30.0),     // Bass
            normalize(mid, 20.0),      // Mid is usually stronger
            normalize(high, 40.0),     // High often weaker
        ]
    }

    /// Get all driver features for Live mode in one call.
    /// Returns (rms, [sub_bass, bass, mid, high]).
    pub fn get_driver_features(&mut self) -> (f32, [f32; 4]) {
        if !self.is_active() {
            return (0.0, [0.0; 4]);
        }

        let rms = self.get_rms();
        let bands = self.get_band_energies();
        (rms, bands)
    }
}

/// Map audio frequency to Chladni plate mode
///
/// For a square plate: f_mn = C * (m² + n²)
/// We need to find the closest (m, n) mode for a given frequency
///
/// Plate constant C depends on:
/// - Plate material properties (Young's modulus, density)
/// - Plate dimensions
/// - Boundary conditions
///
/// Typical values: C ≈ 100-1000 Hz for a 30cm square plate
pub fn frequency_to_mode(frequency: f32, plate_constant: f32) -> (u32, u32) {
    // Target value: m² + n² = frequency / plate_constant
    let target = frequency / plate_constant;

    // Find closest (m, n) such that m² + n² ≈ target
    let mut best_m = 1;
    let mut best_n = 1;
    let mut best_diff = f32::INFINITY;

    // Search reasonable range (m, n from 1 to 20)
    for m in 1..=20 {
        for n in 1..=20 {
            let mode_value = (m * m + n * n) as f32;
            let diff = (mode_value - target).abs();

            if diff < best_diff {
                best_diff = diff;
                best_m = m;
                best_n = n;
            }
        }
    }

    (best_m as u32, best_n as u32)
}

/// Calculate plate constant from physical properties
///
/// For a square plate with fixed edges:
/// C = (π² / L²) * sqrt(D / ρh)
///
/// Where:
/// - L = plate side length (m)
/// - D = flexural rigidity = Eh³ / (12(1-ν²))
/// - E = Young's modulus (Pa)
/// - h = plate thickness (m)
/// - ν = Poisson's ratio
/// - ρ = density (kg/m³)
///
/// Simplified: C ≈ 1000 Hz for typical 30cm aluminum plate
pub fn calculate_plate_constant(
    plate_size: f32,
    youngs_modulus: f32,
    thickness: f32,
    density: f32,
    poisson_ratio: f32,
) -> f32 {
    let l = plate_size;
    let h = thickness;
    let e = youngs_modulus;
    let rho = density;
    let nu = poisson_ratio;

    // Flexural rigidity
    let d = (e * h * h * h) / (12.0 * (1.0 - nu * nu));

    // Plate constant
    let pi = std::f32::consts::PI;
    (pi * pi / (l * l)) * (d / (rho * h)).sqrt()
}
