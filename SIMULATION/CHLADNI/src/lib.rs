//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | SIMULATION/CHLADNI/src/lib.rs
//! PURPOSE: Chladni wave pattern visualization with particle-based sand simulation
//! MODIFIED: 2025-12-09
//! LAYER: SIMULATION → CHLADNI
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]

use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext};

pub mod audio;
pub mod renderer;

pub use audio::{calculate_plate_constant, frequency_to_mode, AudioAnalyzer};
pub use renderer::WaveRenderer;
pub use wave_engine::{ChladniMode, DrivenWaveSolver2D, PlateMode, WaveSimulation};

/// Simulation mode: Demo uses static eigenmodes, Live uses driven physics
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SimulationMode {
    /// Classic eigenmode-based visualization (static patterns)
    #[default]
    Demo,
    /// Driven plate simulation with real-time audio input
    Live,
}

/// Audio driver parameters passed from JavaScript.
/// Contains multi-band energy data for driving the simulation.
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioDriverParams {
    /// Root mean square (loudness), 0-1
    pub rms: f32,
    /// Speaker excitation point X (normalized 0-1)
    pub speaker_x: f32,
    /// Speaker excitation point Y (normalized 0-1)
    pub speaker_y: f32,
    /// Multi-band energies (sub-bass, bass, mid, high), each 0-1
    pub band_energies: [f32; 4],
}

impl AudioDriverParams {
    pub fn new() -> Self {
        Self {
            rms: 0.0,
            speaker_x: 0.5,
            speaker_y: 0.5,
            band_energies: [0.0; 4],
        }
    }
}

/// Configuration for the wave simulation
#[derive(Clone, Debug)]
pub struct SimConfig {
    pub grid_size: u32,
    pub plate_size: f32, // Physical size in meters
    pub damping: f32,    // Wave damping factor
    pub wave_speed: f32, // Wave propagation speed
    pub time_scale: f32, // Simulation speed multiplier
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            grid_size: 256,
            plate_size: 0.3, // 30cm plate
            damping: 0.002,
            wave_speed: 100.0,
            time_scale: 1.0,
        }
    }
}

/// Particle for sand visualization
#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub active: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            active: true,
        }
    }
}

/// Main simulation state
pub struct ChladniSimulation {
    pub config: SimConfig,
    /// Eigenmode wave simulation (for Demo mode)
    pub wave: WaveSimulation,
    /// Driven wave solver (for Live mode)
    pub driven_solver: DrivenWaveSolver2D,
    /// Current simulation mode
    pub mode: SimulationMode,
    pub particles: Vec<Particle>,
    pub current_mode: PlateMode,
    pub time: f32,
    pub frequency_scale: f32, // Multiplier for mode frequencies
    pub amplitude: f32,       // Wave amplitude multiplier
    pub audio_analyzer: Option<AudioAnalyzer>, // Optional audio input
    pub plate_constant: f32,  // Plate physics constant for frequency-to-mode mapping
    /// Driver parameters from audio input (Live mode)
    pub driver_params: AudioDriverParams,
    /// Pre-allocated motion grid (|velocity| per cell) for particle dynamics
    motion_grid: Vec<f32>,
    /// Smoothed frequency for stable detection
    smoothed_freq: f32,
    /// Frequency history for smoothing
    freq_history: [f32; 8],
    freq_history_idx: usize,
}

impl ChladniSimulation {
    pub fn new(config: SimConfig) -> Self {
        let grid_size = config.grid_size as usize;
        let wave = WaveSimulation::new(grid_size);
        // Initialize driven solver with physics parameters
        let driven_solver =
            DrivenWaveSolver2D::new(grid_size, grid_size, config.wave_speed, config.damping);
        // Start with 50,000 particles for high-performance simulation
        let particles = Self::spawn_particles(config.grid_size, 50000);
        // Pre-allocate motion grid
        let motion_grid = vec![0.0; grid_size * grid_size];

        // Plate constant for frequency-to-mode mapping
        // Lower = more complex patterns for same frequency
        // 50 gives nice complex patterns for human voice range
        let plate_constant = 50.0;

        Self {
            config,
            wave,
            driven_solver,
            mode: SimulationMode::Demo,
            particles,
            current_mode: PlateMode::new(3, 2), // Default (3,2) mode
            time: 0.0,
            frequency_scale: 1.0,
            amplitude: 1.0,
            audio_analyzer: None,
            plate_constant,
            driver_params: AudioDriverParams::new(),
            motion_grid,
            smoothed_freq: 0.0,
            freq_history: [0.0; 8],
            freq_history_idx: 0,
        }
    }

    /// Create simulation with custom particle count
    pub fn with_particle_count(config: SimConfig, particle_count: usize) -> Self {
        let grid_size = config.grid_size as usize;
        let wave = WaveSimulation::new(grid_size);
        let driven_solver =
            DrivenWaveSolver2D::new(grid_size, grid_size, config.wave_speed, config.damping);
        let particles = Self::spawn_particles(config.grid_size, particle_count);
        let motion_grid = vec![0.0; grid_size * grid_size];

        let plate_constant = 50.0;

        Self {
            config,
            wave,
            driven_solver,
            mode: SimulationMode::Demo,
            particles,
            current_mode: PlateMode::new(3, 2),
            time: 0.0,
            frequency_scale: 1.0,
            amplitude: 1.0,
            audio_analyzer: None,
            plate_constant,
            driver_params: AudioDriverParams::new(),
            motion_grid,
            smoothed_freq: 0.0,
            freq_history: [0.0; 8],
            freq_history_idx: 0,
        }
    }

    /// Set frequency scale (affects pattern complexity)
    pub fn set_frequency_scale(&mut self, scale: f32) {
        self.frequency_scale = scale.clamp(0.1, 3.0);
    }

    /// Set amplitude (affects particle movement strength)
    pub fn set_amplitude(&mut self, amp: f32) {
        self.amplitude = amp.clamp(0.1, 2.0);
    }

    fn spawn_particles(grid_size: u32, count: usize) -> Vec<Particle> {
        use js_sys::Math;
        let mut particles = Vec::with_capacity(count);
        let size = grid_size as f32;
        let margin = 20.0; // Keep away from edges
        let inner_size = size - 2.0 * margin;

        for _ in 0..count {
            let x = margin + Math::random() as f32 * inner_size;
            let y = margin + Math::random() as f32 * inner_size;
            particles.push(Particle::new(x, y));
        }

        particles
    }

    /// Update simulation by one timestep
    pub fn step(&mut self, dt: f32) {
        let dt_scaled = dt * self.config.time_scale;
        self.time += dt_scaled;

        match self.mode {
            SimulationMode::Demo => {
                // Demo mode: eigenmode-based simulation
                // Check for audio input and update mode if available
                if let Some(ref mut analyzer) = self.audio_analyzer {
                    if let Some(frequency) = analyzer.get_dominant_frequency() {
                        // Map frequency to Chladni mode
                        let (m, n) = frequency_to_mode(frequency, self.plate_constant);
                        self.current_mode = PlateMode::new(m, n);
                        // Mark wave as dirty to force recomputation
                        self.wave.set_dirty();
                    }
                }

                // Update wave field with frequency scale and amplitude
                self.wave.update_with_params(
                    dt_scaled,
                    self.current_mode,
                    self.config.wave_speed,
                    self.frequency_scale,
                    self.amplitude,
                );

                // Update particles based on wave gradient (classic mode)
                self.update_particles_demo(dt_scaled);
            }
            SimulationMode::Live => {
                // Live mode: driven plate simulation with multi-band excitation
                self.step_live(dt_scaled);
            }
        }
    }

    /// Step the driven simulation (Live mode)
    /// Audio frequency controls pattern, resonance quality controls settling strength
    fn step_live(&mut self, dt: f32) {
        let mut resonance_quality = 0.0f32;
        let mut loudness = 0.0f32;

        // Get frequency from audio with smoothing
        if let Some(ref mut analyzer) = self.audio_analyzer {
            if analyzer.is_active() {
                // Get audio features
                let (rms, bands) = analyzer.get_driver_features();
                self.driver_params.rms = rms;
                self.driver_params.band_energies = bands;
                loudness = rms;

                // Only process frequency if there's actual sound
                if rms > 0.05 {
                    if let Some(raw_freq) = analyzer.get_dominant_frequency() {
                        // Add to history for smoothing
                        self.freq_history[self.freq_history_idx] = raw_freq;
                        self.freq_history_idx = (self.freq_history_idx + 1) % 8;

                        // Calculate median of recent frequencies (more robust than mean)
                        let mut sorted = self.freq_history;
                        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let median_freq = sorted[4]; // Middle value

                        // Only use if not too far from median (reject outliers)
                        let deviation = (raw_freq - median_freq).abs() / median_freq.max(1.0);
                        if deviation < 0.3 || self.smoothed_freq == 0.0 {
                            // Smooth the frequency (low-pass filter)
                            self.smoothed_freq = self.smoothed_freq * 0.7 + median_freq * 0.3;
                        }

                        let frequency = self.smoothed_freq;

                        // Map frequency to nearest Chladni mode
                        let (m, n) = frequency_to_mode(frequency, self.plate_constant);

                        // Calculate the exact eigenfrequency for this mode
                        let eigen_freq = self.plate_constant * (m * m + n * n) as f32;

                        // Resonance quality: how close are we to the eigenfrequency?
                        let freq_error = (frequency - eigen_freq).abs() / eigen_freq;
                        resonance_quality = (1.0 - freq_error * 5.0).clamp(0.0, 1.0);

                        // Update mode
                        if m != self.current_mode.m || n != self.current_mode.n {
                            self.current_mode = PlateMode::new(m, n);
                            self.wave.set_dirty();
                        }
                    }
                }
            }
        }

        // Store for UI display
        self.driver_params.speaker_x = resonance_quality;
        self.driver_params.speaker_y = self.smoothed_freq;

        // Update wave field
        self.wave.update_with_params(
            dt,
            self.current_mode,
            self.config.wave_speed,
            self.frequency_scale,
            self.amplitude,
        );

        // Update particles with resonance-based dynamics
        self.update_particles_resonance(dt, resonance_quality, self.driver_params.rms);
    }

    /// Particle update with resonance-based dynamics
    /// Like a game: pattern constantly wants to scatter, only sustained pitch keeps it stable
    fn update_particles_resonance(&mut self, dt: f32, resonance: f32, loudness: f32) {
        use js_sys::Math;

        let grid_size = self.config.grid_size as f32;
        let grid_size_minus_2 = grid_size - 2.0;

        // Normalize loudness: boost quiet signals, cap loud ones
        let normalized_loudness = (loudness * 3.0).clamp(0.0, 1.0);

        // Game-like parameters
        let base_chaos = 80.0; // Strong chaos when no sound
        let max_settling_force = 700.0 * self.amplitude;
        let damping = 0.82; // Lower = more movement
        let boundary_margin = 15.0;
        let boundary_force = 100.0;

        // Control = resonance × normalized loudness
        // Need BOTH correct pitch AND audible sound
        let control = (resonance * normalized_loudness).clamp(0.0, 1.0);

        // Settling force: exponential response for sharper "locking"
        let settling_strength = control * control * control * max_settling_force;

        // Chaos scales INVERSELY with control
        // Quiet/off-pitch = maximum chaos, good control = reduced chaos
        let noise_strength = base_chaos * (1.0 - control * 0.7) + 20.0;

        let grid_size_minus_margin = grid_size - boundary_margin;
        let inv_boundary_margin = 1.0 / boundary_margin;

        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            let mut force = glam::Vec2::ZERO;

            // 1. Settling force toward nodal lines (scaled by resonance)
            let gradient = self.wave.gradient_at(particle.pos.x, particle.pos.y);
            force -= gradient * settling_strength;

            // 2. Environmental noise (always present, reduced by resonance)
            let noise_x = (Math::random() as f32 - 0.5) * noise_strength;
            let noise_y = (Math::random() as f32 - 0.5) * noise_strength;
            force.x += noise_x;
            force.y += noise_y;

            // 3. Boundary repulsion
            let px = particle.pos.x;
            let py = particle.pos.y;

            if px < boundary_margin {
                force.x += boundary_force * (1.0 - px * inv_boundary_margin);
            } else if px > grid_size_minus_margin {
                force.x -= boundary_force * (1.0 - (grid_size - px) * inv_boundary_margin);
            }
            if py < boundary_margin {
                force.y += boundary_force * (1.0 - py * inv_boundary_margin);
            } else if py > grid_size_minus_margin {
                force.y -= boundary_force * (1.0 - (grid_size - py) * inv_boundary_margin);
            }

            // Clamp force
            let force_mag_sq = force.length_squared();
            if force_mag_sq > 250000.0 {
                force = force.normalize() * 500.0;
            }

            particle.vel += force * dt;
            particle.vel *= damping;

            // Clamp velocity
            let vel_mag_sq = particle.vel.length_squared();
            if vel_mag_sq > 40000.0 {
                particle.vel = particle.vel.normalize() * 200.0;
            }

            particle.pos += particle.vel * dt;
            particle.pos.x = particle.pos.x.clamp(1.0, grid_size_minus_2);
            particle.pos.y = particle.pos.y.clamp(1.0, grid_size_minus_2);
        }
    }

    /// Update driver parameters (called from JavaScript each frame)
    pub fn update_driver_params(&mut self, rms: f32, band_energies: [f32; 4]) {
        self.driver_params.rms = rms;
        self.driver_params.band_energies = band_energies;
    }

    /// Set speaker position (normalized 0-1)
    pub fn set_speaker_position(&mut self, x: f32, y: f32) {
        self.driver_params.speaker_x = x.clamp(0.0, 1.0);
        self.driver_params.speaker_y = y.clamp(0.0, 1.0);
    }

    /// Set simulation mode
    pub fn set_simulation_mode(&mut self, mode: SimulationMode) {
        if self.mode != mode {
            self.mode = mode;
            if mode == SimulationMode::Live {
                // Clear the driven solver when switching to Live mode
                self.driven_solver.clear();
            }
        }
    }

    /// Get current simulation mode
    pub fn get_simulation_mode(&self) -> SimulationMode {
        self.mode
    }

    /// Enable audio input
    pub async fn enable_audio(&mut self) -> Result<(), JsValue> {
        let mut analyzer = AudioAnalyzer::new()?;
        analyzer.start_microphone().await?;
        self.audio_analyzer = Some(analyzer);
        Ok(())
    }

    /// Set audio analyzer (for internal use)
    pub fn set_audio_analyzer(&mut self, analyzer: AudioAnalyzer) {
        self.audio_analyzer = Some(analyzer);
    }

    /// Disable audio input
    pub fn disable_audio(&mut self) {
        if let Some(mut analyzer) = self.audio_analyzer.take() {
            analyzer.stop();
        }
    }

    /// Check if audio is active
    pub fn is_audio_active(&self) -> bool {
        self.audio_analyzer.as_ref().is_some_and(|a| a.is_active())
    }

    /// Get current audio frequency (if available)
    pub fn get_audio_frequency(&mut self) -> Option<f32> {
        self.audio_analyzer.as_mut()?.get_dominant_frequency()
    }

    /// Update particles for Demo mode (gradient-based)
    fn update_particles_demo(&mut self, dt: f32) {
        use js_sys::Math;

        let grid_size = self.config.grid_size as f32;
        let damping = 0.85; // More damping for stability
        let force_scale = 300.0; // Balanced force
        let noise_strength = 20.0; // Random jitter to prevent getting stuck
        let boundary_margin = 15.0; // Keep particles away from edges
        let boundary_force = 100.0; // Force pushing away from edges

        // Optimized: Pre-calculate constants outside loop
        let grid_size_minus_2 = grid_size - 2.0;
        let grid_size_minus_margin = grid_size - boundary_margin;
        let inv_boundary_margin = 1.0 / boundary_margin;

        // Process particles in chunks for better cache locality
        // This helps with large particle counts (50k+)
        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            // Get wave gradient at particle position
            let gradient = self.wave.gradient_at(particle.pos.x, particle.pos.y);

            // Particles move toward nodal lines (low amplitude)
            // Force is proportional to negative gradient of amplitude squared
            let mut force = -gradient * force_scale;

            // Add random noise to prevent particles from getting stuck
            // Optimized: Only add noise occasionally for performance
            if Math::random() < 0.1 {
                let noise_x = (Math::random() as f32 - 0.5) * noise_strength;
                let noise_y = (Math::random() as f32 - 0.5) * noise_strength;
                force.x += noise_x;
                force.y += noise_y;
            }

            // Boundary repulsion - soft force pushing away from edges
            // Optimized: Use early exits
            let px = particle.pos.x;
            let py = particle.pos.y;

            if px < boundary_margin {
                force.x += boundary_force * (1.0 - px * inv_boundary_margin);
            } else if px > grid_size_minus_margin {
                force.x -= boundary_force * (1.0 - (grid_size - px) * inv_boundary_margin);
            }

            if py < boundary_margin {
                force.y += boundary_force * (1.0 - py * inv_boundary_margin);
            } else if py > grid_size_minus_margin {
                force.y -= boundary_force * (1.0 - (grid_size - py) * inv_boundary_margin);
            }

            // Clamp force magnitude to prevent particles from shooting off
            // Optimized: Use length_squared for comparison
            let force_mag_sq = force.length_squared();
            if force_mag_sq > 250000.0 {
                // 500^2
                force = force.normalize() * 500.0;
            }

            particle.vel += force * dt;
            particle.vel *= damping;

            // Clamp velocity for stability
            let vel_mag_sq = particle.vel.length_squared();
            if vel_mag_sq > 40000.0 {
                // 200^2
                particle.vel = particle.vel.normalize() * 200.0;
            }

            particle.pos += particle.vel * dt;

            // Hard boundary clamp (safety net) - optimized
            particle.pos.x = particle.pos.x.clamp(1.0, grid_size_minus_2);
            particle.pos.y = particle.pos.y.clamp(1.0, grid_size_minus_2);
        }
    }

    /// Update particles with hybrid approach: eigenmode guidance + audio excitation
    /// Particles follow eigenmode gradients but get kicked by audio energy
    fn update_particles_hybrid(&mut self, dt: f32, audio_energy: f32) {
        use js_sys::Math;
        use std::f32::consts::PI;

        let grid_size = self.config.grid_size as f32;
        let damping = 0.85;
        let force_scale = 300.0;
        let audio_kick_scale = 0.5; // How much audio affects particles
        let boundary_margin = 15.0;
        let boundary_force = 100.0;

        let grid_size_minus_2 = grid_size - 2.0;
        let grid_size_minus_margin = grid_size - boundary_margin;
        let inv_boundary_margin = 1.0 / boundary_margin;

        // Normalize audio energy for kick probability
        let kick_probability = (audio_energy / 100.0).clamp(0.0, 0.8);
        let kick_magnitude = audio_energy * audio_kick_scale;

        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            // Get eigenmode gradient (same as demo mode)
            let gradient = self.wave.gradient_at(particle.pos.x, particle.pos.y);

            // Base force: drift toward nodal lines (eigenmode gradient)
            let mut force = -gradient * force_scale;

            // Audio excitation: random kicks when audio is loud
            if Math::random() < kick_probability as f64 {
                let angle = Math::random() as f32 * 2.0 * PI;
                force.x += angle.cos() * kick_magnitude;
                force.y += angle.sin() * kick_magnitude;
            }

            // Boundary repulsion
            let px = particle.pos.x;
            let py = particle.pos.y;

            if px < boundary_margin {
                force.x += boundary_force * (1.0 - px * inv_boundary_margin);
            } else if px > grid_size_minus_margin {
                force.x -= boundary_force * (1.0 - (grid_size - px) * inv_boundary_margin);
            }

            if py < boundary_margin {
                force.y += boundary_force * (1.0 - py * inv_boundary_margin);
            } else if py > grid_size_minus_margin {
                force.y -= boundary_force * (1.0 - (grid_size - py) * inv_boundary_margin);
            }

            // Clamp force
            let force_mag_sq = force.length_squared();
            if force_mag_sq > 250000.0 {
                force = force.normalize() * 500.0;
            }

            particle.vel += force * dt;
            particle.vel *= damping;

            // Clamp velocity
            let vel_mag_sq = particle.vel.length_squared();
            if vel_mag_sq > 40000.0 {
                particle.vel = particle.vel.normalize() * 200.0;
            }

            particle.pos += particle.vel * dt;
            particle.pos.x = particle.pos.x.clamp(1.0, grid_size_minus_2);
            particle.pos.y = particle.pos.y.clamp(1.0, grid_size_minus_2);
        }
    }

    /// Update particles for Live mode (motion-based dynamics)
    /// Particles get kicked by high motion and drift toward low-motion nodes
    fn update_particles_live(&mut self, dt: f32) {
        use js_sys::Math;
        use std::f32::consts::PI;

        let grid_size = self.config.grid_size as usize;
        let grid_size_f = grid_size as f32;
        let damping = 0.92; // Slightly less damping for more dynamic feel
        let kick_threshold = 0.5; // Lower threshold for responsiveness
        let kick_strength = 100.0; // How hard particles get kicked
        let drift_strength = 200.0; // How strongly particles drift toward nodes
        let noise_strength = 15.0; // Baseline jitter to prevent freezing
        let boundary_margin = 15.0;
        let boundary_force = 100.0;

        let grid_size_minus_2 = grid_size_f - 2.0;
        let grid_size_minus_margin = grid_size_f - boundary_margin;
        let inv_boundary_margin = 1.0 / boundary_margin;

        // Borrow motion_grid immutably before the loop to avoid borrow conflicts
        let motion_grid = &self.motion_grid;

        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            let px = particle.pos.x;
            let py = particle.pos.y;

            // Sample local motion from grid (inline to avoid borrow issues)
            let gx = (px as usize).min(grid_size - 1);
            let gy = (py as usize).min(grid_size - 1);
            let local_motion = motion_grid[gy * grid_size + gx];

            let mut force = Vec2::ZERO;

            // Stochastic kick when motion is high (particle gets "thrown")
            if local_motion > kick_threshold {
                let excess_motion = local_motion - kick_threshold;
                let kick_magnitude = excess_motion * kick_strength;
                // Random direction for the kick
                let angle = Math::random() as f32 * 2.0 * PI;
                force.x += angle.cos() * kick_magnitude;
                force.y += angle.sin() * kick_magnitude;
            }

            // Drift toward low-motion regions (compute motion gradient)
            // Inline sampling to avoid borrow conflicts
            let eps = 1.0;
            let sample = |x: f32, y: f32| -> f32 {
                let sx = (x as usize).clamp(0, grid_size - 1);
                let sy = (y as usize).clamp(0, grid_size - 1);
                motion_grid[sy * grid_size + sx]
            };

            let motion_xp = sample(px + eps, py);
            let motion_xn = sample(px - eps, py);
            let motion_yp = sample(px, py + eps);
            let motion_yn = sample(px, py - eps);

            let grad_motion = Vec2::new(
                (motion_xp - motion_xn) / (2.0 * eps),
                (motion_yp - motion_yn) / (2.0 * eps),
            );

            // Move down the gradient (toward low motion / nodal lines)
            force -= grad_motion * drift_strength;

            // Add gentle noise to prevent freezing when motion is low
            if local_motion < kick_threshold && Math::random() < 0.15 {
                let noise_x = (Math::random() as f32 - 0.5) * noise_strength;
                let noise_y = (Math::random() as f32 - 0.5) * noise_strength;
                force.x += noise_x;
                force.y += noise_y;
            }

            // Boundary repulsion
            if px < boundary_margin {
                force.x += boundary_force * (1.0 - px * inv_boundary_margin);
            } else if px > grid_size_minus_margin {
                force.x -= boundary_force * (1.0 - (grid_size_f - px) * inv_boundary_margin);
            }

            if py < boundary_margin {
                force.y += boundary_force * (1.0 - py * inv_boundary_margin);
            } else if py > grid_size_minus_margin {
                force.y -= boundary_force * (1.0 - (grid_size_f - py) * inv_boundary_margin);
            }

            // Clamp force magnitude
            let force_mag_sq = force.length_squared();
            if force_mag_sq > 250000.0 {
                force = force.normalize() * 500.0;
            }

            particle.vel += force * dt;
            particle.vel *= damping;

            // Clamp velocity
            let vel_mag_sq = particle.vel.length_squared();
            if vel_mag_sq > 40000.0 {
                particle.vel = particle.vel.normalize() * 200.0;
            }

            particle.pos += particle.vel * dt;

            // Hard boundary clamp
            particle.pos.x = particle.pos.x.clamp(1.0, grid_size_minus_2);
            particle.pos.y = particle.pos.y.clamp(1.0, grid_size_minus_2);
        }
    }

    /// Set vibration mode (particles migrate to new nodal lines)
    pub fn set_mode(&mut self, m: u32, n: u32) {
        self.current_mode = PlateMode::new(m, n);
        // Don't reset particles - let them flow to new nodal lines
    }

    /// Reset particle positions
    pub fn reset_particles(&mut self) {
        self.particles = Self::spawn_particles(self.config.grid_size, self.particles.len());
    }
}

// Thread-local storage for global simulation state
thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

/// Application state holding simulation and renderer
struct App {
    simulation: ChladniSimulation,
    renderer: WaveRenderer,
    canvas: HtmlCanvasElement,
    last_time: f64,
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Chladni simulation starting...".into());

    let window = window().ok_or("No window found")?;
    let document = window.document().ok_or("No document found")?;

    // Get canvas element
    let canvas = document
        .get_element_by_id("simulation")
        .ok_or("Canvas #simulation not found")?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Element is not a canvas")?;

    // Set canvas size to match container
    let container = document
        .get_element_by_id("canvas-container")
        .ok_or("Canvas container not found")?;
    let width = container.client_width() as u32;
    let height = container.client_height() as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    web_sys::console::log_1(&format!("Canvas size: {}x{}", width, height).into());

    // Get WebGL2 context
    let gl = canvas
        .get_context("webgl2")
        .map_err(|e| format!("get_context failed: {:?}", e))?
        .ok_or("WebGL2 context is null")?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| "Failed to cast to WebGL2 context")?;

    // Initialize renderer
    let mut renderer = WaveRenderer::new(gl);
    renderer.init()?;

    // Initialize simulation
    let simulation = ChladniSimulation::new(SimConfig::default());

    // Store in thread-local
    let app = App {
        simulation,
        renderer,
        canvas,
        last_time: 0.0,
    };

    APP.with(|cell| {
        *cell.borrow_mut() = Some(app);
    });

    // Export setChladniMode to JavaScript
    let set_mode_fn = Closure::wrap(Box::new(|m: u32, n: u32| {
        set_chladni_mode(m, n);
    }) as Box<dyn Fn(u32, u32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniMode"),
        set_mode_fn.as_ref(),
    )?;
    set_mode_fn.forget();

    // Export setChladniFrequency to JavaScript
    let set_freq_fn = Closure::wrap(Box::new(|scale: f32| {
        set_chladni_frequency(scale);
    }) as Box<dyn Fn(f32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniFrequency"),
        set_freq_fn.as_ref(),
    )?;
    set_freq_fn.forget();

    // Export setChladniAmplitude to JavaScript
    let set_amp_fn = Closure::wrap(Box::new(|amp: f32| {
        set_chladni_amplitude(amp);
    }) as Box<dyn Fn(f32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniAmplitude"),
        set_amp_fn.as_ref(),
    )?;
    set_amp_fn.forget();

    // Export resetChladniParticles to JavaScript
    let reset_fn = Closure::wrap(Box::new(|| {
        reset_chladni_particles();
    }) as Box<dyn Fn()>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("resetChladniParticles"),
        reset_fn.as_ref(),
    )?;
    reset_fn.forget();

    // Export enableAudio to JavaScript (async - spawns future)
    let enable_audio_fn = Closure::wrap(Box::new(|| {
        enable_chladni_audio();
    }) as Box<dyn Fn()>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("enableChladniAudio"),
        enable_audio_fn.as_ref(),
    )?;
    enable_audio_fn.forget();

    // Export disableAudio to JavaScript
    let disable_audio_fn = Closure::wrap(Box::new(|| {
        disable_chladni_audio();
    }) as Box<dyn Fn()>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("disableChladniAudio"),
        disable_audio_fn.as_ref(),
    )?;
    disable_audio_fn.forget();

    // Export isAudioActive to JavaScript
    let is_audio_active_fn =
        Closure::wrap(Box::new(|| is_chladni_audio_active()) as Box<dyn Fn() -> bool>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("isChladniAudioActive"),
        is_audio_active_fn.as_ref(),
    )?;
    is_audio_active_fn.forget();

    // Export getAudioFrequency to JavaScript
    let get_freq_fn = Closure::wrap(
        Box::new(|| get_chladni_audio_frequency().unwrap_or(-1.0)) as Box<dyn Fn() -> f32>
    );

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("getChladniAudioFrequency"),
        get_freq_fn.as_ref(),
    )?;
    get_freq_fn.forget();

    // Export setSimulationMode to JavaScript (true = Live, false = Demo)
    let set_mode_fn = Closure::wrap(Box::new(|is_live: bool| {
        set_chladni_simulation_mode(is_live);
    }) as Box<dyn Fn(bool)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniSimulationMode"),
        set_mode_fn.as_ref(),
    )?;
    set_mode_fn.forget();

    // Export updateDriverParams to JavaScript
    let update_params_fn = Closure::wrap(Box::new(|rms: f32, b0: f32, b1: f32, b2: f32, b3: f32| {
        update_chladni_driver_params(rms, b0, b1, b2, b3);
    }) as Box<dyn Fn(f32, f32, f32, f32, f32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("updateChladniDriverParams"),
        update_params_fn.as_ref(),
    )?;
    update_params_fn.forget();

    // Export setSpeakerPosition to JavaScript
    let set_speaker_fn = Closure::wrap(Box::new(|x: f32, y: f32| {
        set_chladni_speaker_position(x, y);
    }) as Box<dyn Fn(f32, f32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniSpeakerPosition"),
        set_speaker_fn.as_ref(),
    )?;
    set_speaker_fn.forget();

    // Export getSimulationMode to JavaScript
    let get_mode_fn =
        Closure::wrap(Box::new(|| get_chladni_simulation_mode()) as Box<dyn Fn() -> bool>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("getChladniSimulationMode"),
        get_mode_fn.as_ref(),
    )?;
    get_mode_fn.forget();

    // Export getDriverParams to JavaScript (for visualization)
    let get_params_fn =
        Closure::wrap(Box::new(|| get_chladni_driver_params()) as Box<dyn Fn() -> Vec<f32>>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("getChladniDriverParams"),
        get_params_fn.as_ref(),
    )?;
    get_params_fn.forget();

    // Export setPlateConstant to JavaScript
    let set_plate_const_fn = Closure::wrap(Box::new(|val: f32| {
        set_chladni_plate_constant(val);
    }) as Box<dyn Fn(f32)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setChladniPlateConstant"),
        set_plate_const_fn.as_ref(),
    )?;
    set_plate_const_fn.forget();

    // Export getPlateConstant to JavaScript
    let get_plate_const_fn =
        Closure::wrap(Box::new(|| get_chladni_plate_constant()) as Box<dyn Fn() -> f32>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("getChladniPlateConstant"),
        get_plate_const_fn.as_ref(),
    )?;
    get_plate_const_fn.forget();

    // Export getCurrentMode to JavaScript
    let get_mode_fn2 =
        Closure::wrap(Box::new(|| get_chladni_current_mode()) as Box<dyn Fn() -> Vec<u32>>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("getChladniCurrentMode"),
        get_mode_fn2.as_ref(),
    )?;
    get_mode_fn2.forget();

    // Start animation loop
    start_animation_loop()?;

    web_sys::console::log_1(&"Chladni simulation initialized".into());
    Ok(())
}

/// Set vibration mode (called from JavaScript)
#[wasm_bindgen]
pub fn set_chladni_mode(m: u32, n: u32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.set_mode(m, n);
            web_sys::console::log_1(&format!("Mode set to ({}, {})", m, n).into());
        }
    });
}

/// Set frequency scale (called from JavaScript)
#[wasm_bindgen]
pub fn set_chladni_frequency(scale: f32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.set_frequency_scale(scale);
            web_sys::console::log_1(&format!("Frequency scale set to {}", scale).into());
        }
    });
}

/// Set amplitude (called from JavaScript)
#[wasm_bindgen]
pub fn set_chladni_amplitude(amp: f32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.set_amplitude(amp);
            web_sys::console::log_1(&format!("Amplitude set to {}", amp).into());
        }
    });
}

/// Reset particles (called from JavaScript)
#[wasm_bindgen]
pub fn reset_chladni_particles() {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.reset_particles();
            web_sys::console::log_1(&"Particles reset".into());
        }
    });
}

/// Enable audio input (called from JavaScript)
/// This spawns an async task since we can't return a Promise directly from a closure
#[wasm_bindgen]
pub fn enable_chladni_audio() {
    wasm_bindgen_futures::spawn_local(async {
        let result = async {
            let mut analyzer = AudioAnalyzer::new()?;
            analyzer.start_microphone().await?;
            Ok::<AudioAnalyzer, JsValue>(analyzer)
        }
        .await;

        match result {
            Ok(analyzer) => {
                APP.with(|cell| {
                    if let Some(ref mut app) = *cell.borrow_mut() {
                        app.simulation.set_audio_analyzer(analyzer);
                        web_sys::console::log_1(&"Audio enabled successfully".into());
                    }
                });
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to enable audio: {:?}", e).into());
            }
        }
    });
}

/// Disable audio input (called from JavaScript)
#[wasm_bindgen]
pub fn disable_chladni_audio() {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.disable_audio();
            web_sys::console::log_1(&"Audio disabled".into());
        }
    });
}

/// Check if audio is active (called from JavaScript)
#[wasm_bindgen]
pub fn is_chladni_audio_active() -> bool {
    APP.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|app| app.simulation.is_audio_active())
            .unwrap_or(false)
    })
}

/// Get current audio frequency (called from JavaScript)
#[wasm_bindgen]
pub fn get_chladni_audio_frequency() -> Option<f32> {
    APP.with(|cell| {
        cell.borrow_mut()
            .as_mut()
            .and_then(|app| app.simulation.get_audio_frequency())
    })
}

/// Set simulation mode (called from JavaScript)
/// true = Live mode (driven physics), false = Demo mode (eigenmodes)
#[wasm_bindgen]
pub fn set_chladni_simulation_mode(is_live: bool) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            let mode = if is_live {
                SimulationMode::Live
            } else {
                SimulationMode::Demo
            };
            app.simulation.set_simulation_mode(mode);
            web_sys::console::log_1(&format!("Simulation mode set to {:?}", mode).into());
        }
    });
}

/// Get current simulation mode (called from JavaScript)
/// Returns true for Live mode, false for Demo mode
#[wasm_bindgen]
pub fn get_chladni_simulation_mode() -> bool {
    APP.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|app| app.simulation.get_simulation_mode() == SimulationMode::Live)
            .unwrap_or(false)
    })
}

/// Update driver parameters for Live mode (called from JavaScript each frame)
#[wasm_bindgen]
pub fn update_chladni_driver_params(rms: f32, band0: f32, band1: f32, band2: f32, band3: f32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation
                .update_driver_params(rms, [band0, band1, band2, band3]);
        }
    });
}

/// Set speaker position for Live mode (called from JavaScript)
#[wasm_bindgen]
pub fn set_chladni_speaker_position(x: f32, y: f32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.set_speaker_position(x, y);
            web_sys::console::log_1(
                &format!("Speaker position set to ({:.2}, {:.2})", x, y).into(),
            );
        }
    });
}

/// Set plate constant (controls frequency-to-mode mapping)
/// Higher values = simpler patterns for same frequency
#[wasm_bindgen]
pub fn set_chladni_plate_constant(value: f32) {
    APP.with(|cell| {
        if let Some(ref mut app) = *cell.borrow_mut() {
            app.simulation.plate_constant = value.clamp(10.0, 2000.0);
        }
    });
}

/// Get current plate constant
#[wasm_bindgen]
pub fn get_chladni_plate_constant() -> f32 {
    APP.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|app| app.simulation.plate_constant)
            .unwrap_or(500.0)
    })
}

/// Get current detected mode (m, n)
#[wasm_bindgen]
pub fn get_chladni_current_mode() -> Vec<u32> {
    APP.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|app| vec![app.simulation.current_mode.m, app.simulation.current_mode.n])
            .unwrap_or_else(|| vec![3, 2])
    })
}

/// Get current driver params for visualization (called from JavaScript)
/// Returns [rms, band0, band1, band2, band3]
#[wasm_bindgen]
pub fn get_chladni_driver_params() -> Vec<f32> {
    APP.with(|cell| {
        if let Some(ref app) = *cell.borrow() {
            let p = &app.simulation.driver_params;
            vec![
                p.rms,
                p.band_energies[0],
                p.band_energies[1],
                p.band_energies[2],
                p.band_energies[3],
            ]
        } else {
            vec![0.0, 0.0, 0.0, 0.0, 0.0]
        }
    })
}

/// Start the requestAnimationFrame loop
fn start_animation_loop() -> Result<(), JsValue> {
    let window = window().ok_or("No window found")?;

    // Create self-referential closure for animation loop
    #[allow(clippy::type_complexity)]
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let window_clone = window.clone();
    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        APP.with(|cell| {
            if let Some(ref mut app) = *cell.borrow_mut() {
                // Calculate delta time (convert ms to seconds)
                let dt = if app.last_time > 0.0 {
                    ((timestamp - app.last_time) / 1000.0).min(0.1) as f32
                } else {
                    1.0 / 60.0 // First frame default
                };
                app.last_time = timestamp;

                // Handle canvas resize
                let container_width = app.canvas.client_width() as u32;
                let container_height = app.canvas.client_height() as u32;
                if (container_width != app.canvas.width()
                    || container_height != app.canvas.height())
                    && container_width > 0
                    && container_height > 0
                {
                    app.canvas.set_width(container_width);
                    app.canvas.set_height(container_height);
                }

                // Update simulation
                app.simulation.step(dt);

                // Render
                let width = app.canvas.width() as f32;
                let height = app.canvas.height() as f32;
                app.renderer.render(&app.simulation, width, height);
            }
        });

        // Request next frame
        window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("requestAnimationFrame failed");
    }));

    // Start animation
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
