//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: fdm.rs | DNA/src/physics/solvers/pde/fdm.rs
//! PURPOSE: Finite Difference Method solvers for PDEs
//! MODIFIED: 2025-12-29
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! ALGORITHM: Explicit Leapfrog Time-Stepping for 2D Wave Equation
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ PDE: u_tt = c² * (u_xx + u_yy) + source - damping * u_t                     │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Discretization (leapfrog):                                                  │
//! │   u[n+1] = (2 - d·dt) * u[n] - (1 - d·dt) * u[n-1]                          │
//! │            + (c·dt/dx)² * Lap(u[n]) + dt² * source[n]                        │
//! │                                                                             │
//! │ 5-point Laplacian stencil:                                                  │
//! │   Lap(u) = u[i-1,j] + u[i+1,j] + u[i,j-1] + u[i,j+1] - 4*u[i,j]            │
//! │                                                                             │
//! │ Stability (CFL): c * dt / dx <= 1/√2 ≈ 0.707 for 2D                         │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DATA DEFINED:
//!   DrivenWaveSolver2D - Time-stepped 2D wave solver with point excitation
//!
//! USED BY:
//!   • SIMULATION/CHLADNI → Driven Chladni plate simulation
//!
//! ═══════════════════════════════════════════════════════════════════════════════

/// 2D driven wave equation solver using explicit finite differences.
///
/// Simulates a vibrating plate driven by a point source (e.g., speaker contact).
/// Uses leapfrog time integration with damping for stability.
pub struct DrivenWaveSolver2D {
    /// Grid width
    pub width: usize,
    /// Grid height
    pub height: usize,

    // Wave state (double-buffered for time-stepping)
    u_curr: Vec<f32>, // u at time n
    u_prev: Vec<f32>, // u at time n-1
    u_next: Vec<f32>, // u at time n+1 (scratch buffer)

    // Derived quantities
    velocity: Vec<f32>, // du/dt = (u_curr - u_prev) / dt

    // Physical parameters
    wave_speed: f32, // c in wave equation
    damping: f32,    // Energy loss coefficient
    dx: f32,         // Grid spacing

    // Simulation time tracking
    time: f32,
}

impl DrivenWaveSolver2D {
    /// Create a new solver for a grid of given dimensions.
    ///
    /// # Arguments
    /// * `width` - Grid width in cells
    /// * `height` - Grid height in cells
    /// * `wave_speed` - Wave propagation speed (affects resonant frequencies)
    /// * `damping` - Energy loss rate (0.0 = no damping, 0.1 = moderate)
    pub fn new(width: usize, height: usize, wave_speed: f32, damping: f32) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u_curr: vec![0.0; size],
            u_prev: vec![0.0; size],
            u_next: vec![0.0; size],
            velocity: vec![0.0; size],
            wave_speed,
            damping,
            dx: 1.0,
            time: 0.0,
        }
    }

    /// Take one or more substeps to advance simulation by dt seconds.
    ///
    /// Automatically computes the number of substeps needed for CFL stability.
    ///
    /// # Arguments
    /// * `dt` - Time to advance (seconds)
    /// * `source_x` - Excitation point X (normalized 0-1)
    /// * `source_y` - Excitation point Y (normalized 0-1)
    /// * `source_amplitude` - Driving amplitude this frame
    pub fn step(&mut self, dt: f32, source_x: f32, source_y: f32, source_amplitude: f32) {
        if dt <= 0.0 {
            return;
        }

        // CFL stability: c * dt / dx <= 1/sqrt(2)
        const CFL_LIMIT: f32 = 0.707;
        let cfl_actual = self.wave_speed * dt / self.dx;
        let num_substeps = ((cfl_actual / CFL_LIMIT).ceil() as usize).max(1);
        let sub_dt = dt / num_substeps as f32;

        // Precompute coefficients
        let c2_dt2_dx2 = (self.wave_speed * sub_dt / self.dx).powi(2);
        let damping_coeff = self.damping * sub_dt;

        // Source location in grid coordinates
        let src_x = (source_x * self.width as f32).clamp(2.0, self.width as f32 - 3.0) as usize;
        let src_y = (source_y * self.height as f32).clamp(2.0, self.height as f32 - 3.0) as usize;

        for _ in 0..num_substeps {
            self.substep(c2_dt2_dx2, damping_coeff, src_x, src_y, source_amplitude, sub_dt);
        }

        // Compute velocity field for motion metric
        let inv_dt = 1.0 / dt;
        for i in 0..self.velocity.len() {
            self.velocity[i] = (self.u_curr[i] - self.u_prev[i]) * inv_dt;
        }

        self.time += dt;
    }

    /// Execute a single substep of the wave solver.
    fn substep(
        &mut self,
        c2_dt2_dx2: f32,
        damping_coeff: f32,
        src_x: usize,
        src_y: usize,
        source_amp: f32,
        sub_dt: f32,
    ) {
        let w = self.width;
        let h = self.height;

        // 5-point Laplacian stencil (interior points only)
        for j in 1..h - 1 {
            for i in 1..w - 1 {
                let idx = j * w + i;

                let laplacian = self.u_curr[idx - 1]     // left
                    + self.u_curr[idx + 1]               // right
                    + self.u_curr[idx - w]               // up
                    + self.u_curr[idx + w]               // down
                    - 4.0 * self.u_curr[idx]; // center

                // Leapfrog update with damping
                self.u_next[idx] = (2.0 - damping_coeff) * self.u_curr[idx]
                    - (1.0 - damping_coeff) * self.u_prev[idx]
                    + c2_dt2_dx2 * laplacian;
            }
        }

        // Apply point source with Gaussian spread (3x3 kernel)
        let source_value = source_amp * sub_dt * sub_dt;
        if source_value.abs() > 1e-10 {
            let idx = src_y * w + src_x;

            // Center gets 50% of energy
            self.u_next[idx] += source_value * 0.5;

            // Adjacent cells get 12.5% each (total 50%)
            self.u_next[idx - 1] += source_value * 0.125;
            self.u_next[idx + 1] += source_value * 0.125;
            self.u_next[idx - w] += source_value * 0.125;
            self.u_next[idx + w] += source_value * 0.125;
        }

        // Clamped boundary conditions (edges stay at zero - already handled by interior-only iteration)

        // Swap buffers: prev <- curr <- next
        std::mem::swap(&mut self.u_prev, &mut self.u_curr);
        std::mem::swap(&mut self.u_curr, &mut self.u_next);
    }

    /// Get amplitude at position using bilinear interpolation.
    pub fn amplitude_at(&self, x: f32, y: f32) -> f32 {
        self.sample_bilinear(&self.u_curr, x, y)
    }

    /// Get velocity magnitude at position (for motion metric).
    pub fn velocity_at(&self, x: f32, y: f32) -> f32 {
        self.sample_bilinear(&self.velocity, x, y)
    }

    /// Get raw velocity array for motion field computation.
    pub fn get_velocity_data(&self) -> &[f32] {
        &self.velocity
    }

    /// Get raw amplitude array for visualization.
    pub fn get_amplitude_data(&self) -> &[f32] {
        &self.u_curr
    }

    /// Reset the solver to zero state.
    pub fn clear(&mut self) {
        self.u_curr.fill(0.0);
        self.u_prev.fill(0.0);
        self.u_next.fill(0.0);
        self.velocity.fill(0.0);
        self.time = 0.0;
    }

    /// Get current simulation time.
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Set wave speed (affects resonant frequencies).
    pub fn set_wave_speed(&mut self, speed: f32) {
        self.wave_speed = speed.max(1.0);
    }

    /// Set damping coefficient.
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.0, 1.0);
    }

    /// Compute total energy in the system (for diagnostics).
    pub fn total_energy(&self) -> f32 {
        // Approximate conserved energy for the undamped wave equation:
        // E = 1/2 ∫ (u_t^2 + c^2 |∇u|^2) dA
        //
        // Discrete approximation (dx = dy):
        // kinetic ~ Σ u_t^2
        // potential ~ c^2 Σ ((du/dx)^2 + (du/dy)^2)
        let mut kinetic = 0.0f32;
        for v in &self.velocity {
            kinetic += v * v;
        }

        let mut potential = 0.0f32;
        let w = self.width;
        let h = self.height;
        let inv_dx = 1.0 / self.dx.max(1e-6);

        for j in 0..(h - 1) {
            for i in 0..(w - 1) {
                let idx = j * w + i;
                let u = self.u_curr[idx];
                let dudx = (self.u_curr[idx + 1] - u) * inv_dx;
                let dudy = (self.u_curr[idx + w] - u) * inv_dx;
                potential += dudx * dudx + dudy * dudy;
            }
        }

        0.5 * (kinetic + (self.wave_speed * self.wave_speed) * potential)
    }

    /// Bilinear interpolation helper.
    fn sample_bilinear(&self, data: &[f32], x: f32, y: f32) -> f32 {
        let x = x.clamp(0.0, (self.width - 1) as f32);
        let y = y.clamp(0.0, (self.height - 1) as f32);

        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let w = self.width;
        let v00 = data[y0 * w + x0];
        let v10 = data[y0 * w + x1];
        let v01 = data[y1 * w + x0];
        let v11 = data[y1 * w + x1];

        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;

        v0 * (1.0 - fy) + v1 * fy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = DrivenWaveSolver2D::new(64, 64, 100.0, 0.05);
        assert_eq!(solver.width, 64);
        assert_eq!(solver.height, 64);
        assert_eq!(solver.get_amplitude_data().len(), 64 * 64);
    }

    #[test]
    fn test_energy_with_no_damping() {
        let mut solver = DrivenWaveSolver2D::new(32, 32, 50.0, 0.0);

        // Inject energy at center
        solver.step(0.01, 0.5, 0.5, 100.0);
        let energy_after_inject = solver.total_energy();
        assert!(energy_after_inject > 0.0);

        // Step without injection - energy should be conserved (no damping)
        for _ in 0..10 {
            solver.step(0.01, 0.5, 0.5, 0.0);
        }
        let energy_later = solver.total_energy();

        // Should be approximately conserved (small numerical drift allowed)
        let ratio = energy_later / energy_after_inject;
        assert!(
            // Explicit FDM + clamped boundaries will exhibit some numerical energy drift.
            // We only require it to stay in the same ballpark when damping=0.
            ratio > 0.7 && ratio < 1.3,
            "Energy should be conserved: {} vs {}",
            energy_later,
            energy_after_inject
        );
    }

    #[test]
    fn test_energy_with_damping() {
        let mut solver = DrivenWaveSolver2D::new(32, 32, 50.0, 0.1);

        // Inject energy
        solver.step(0.01, 0.5, 0.5, 100.0);
        let initial_energy = solver.total_energy();

        // Step without injection - energy should decay
        for _ in 0..50 {
            solver.step(0.01, 0.5, 0.5, 0.0);
        }
        let final_energy = solver.total_energy();

        assert!(
            final_energy < initial_energy,
            "Energy should decay with damping"
        );
    }

    #[test]
    fn test_clear() {
        let mut solver = DrivenWaveSolver2D::new(16, 16, 50.0, 0.05);
        solver.step(0.01, 0.5, 0.5, 100.0);
        assert!(solver.total_energy() > 0.0);

        solver.clear();
        assert!(solver.total_energy() < 1e-10);
        assert_eq!(solver.time(), 0.0);
    }

    #[test]
    fn test_clamped_boundaries() {
        let mut solver = DrivenWaveSolver2D::new(16, 16, 50.0, 0.0);

        // Excite near edge
        solver.step(0.01, 0.1, 0.5, 100.0);

        // Check edges stay at zero
        let data = solver.get_amplitude_data();
        for i in 0..16 {
            assert_eq!(data[i], 0.0, "Top edge should be zero");
            assert_eq!(data[15 * 16 + i], 0.0, "Bottom edge should be zero");
            assert_eq!(data[i * 16], 0.0, "Left edge should be zero");
            assert_eq!(data[i * 16 + 15], 0.0, "Right edge should be zero");
        }
    }

    #[test]
    fn test_no_nan_or_inf() {
        let mut solver = DrivenWaveSolver2D::new(64, 64, 100.0, 0.05);

        // Run many steps with varying input
        for i in 0..100 {
            let amp = (i as f32 * 0.1).sin() * 50.0;
            solver.step(0.016, 0.5, 0.5, amp);
        }

        for &v in solver.get_amplitude_data() {
            assert!(v.is_finite(), "Amplitude should be finite");
        }
        for &v in solver.get_velocity_data() {
            assert!(v.is_finite(), "Velocity should be finite");
        }
    }
}
