// Chladni - Wave Pattern Visualization
// Rust/WASM port of realistic Chladni plate simulation
#![allow(unexpected_cfgs)]

use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext};

pub mod renderer;
pub mod wave;

pub use renderer::WaveRenderer;
pub use wave::{ChladniMode, WaveSimulation};

/// Chladni plate modes (m, n) - defines the vibration pattern
#[derive(Clone, Copy, Debug)]
pub struct PlateMode {
    pub m: u32, // Horizontal mode number
    pub n: u32, // Vertical mode number
}

impl PlateMode {
    pub fn new(m: u32, n: u32) -> Self {
        Self { m, n }
    }

    /// Calculate frequency for a square plate
    /// f_mn = C * (m^2 + n^2) where C depends on plate properties
    pub fn frequency(&self, plate_constant: f32) -> f32 {
        plate_constant * ((self.m * self.m + self.n * self.n) as f32)
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
    pub wave: WaveSimulation,
    pub particles: Vec<Particle>,
    pub current_mode: PlateMode,
    pub time: f32,
}

impl ChladniSimulation {
    pub fn new(config: SimConfig) -> Self {
        let wave = WaveSimulation::new(config.grid_size as usize);
        let particles = Self::spawn_particles(config.grid_size, 5000);

        Self {
            config,
            wave,
            particles,
            current_mode: PlateMode::new(3, 2), // Default (3,2) mode
            time: 0.0,
        }
    }

    fn spawn_particles(grid_size: u32, count: usize) -> Vec<Particle> {
        use js_sys::Math;
        let mut particles = Vec::with_capacity(count);
        let size = grid_size as f32;

        for _ in 0..count {
            let x = Math::random() as f32 * size;
            let y = Math::random() as f32 * size;
            particles.push(Particle::new(x, y));
        }

        particles
    }

    /// Update simulation by one timestep
    pub fn step(&mut self, dt: f32) {
        let dt_scaled = dt * self.config.time_scale;
        self.time += dt_scaled;

        // Update wave field
        self.wave
            .update(dt_scaled, self.current_mode, self.config.wave_speed);

        // Update particles based on wave gradient
        self.update_particles(dt_scaled);
    }

    fn update_particles(&mut self, dt: f32) {
        let grid_size = self.config.grid_size as f32;
        let damping = 0.98;

        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            // Get wave gradient at particle position
            let gradient = self.wave.gradient_at(particle.pos.x, particle.pos.y);

            // Particles move toward nodal lines (low amplitude)
            // Force is proportional to negative gradient of amplitude squared
            let force = -gradient * 50.0;

            particle.vel += force * dt;
            particle.vel *= damping;
            particle.pos += particle.vel * dt;

            // Boundary reflection
            if particle.pos.x < 0.0 {
                particle.pos.x = 0.0;
                particle.vel.x = -particle.vel.x * 0.5;
            }
            if particle.pos.x >= grid_size {
                particle.pos.x = grid_size - 1.0;
                particle.vel.x = -particle.vel.x * 0.5;
            }
            if particle.pos.y < 0.0 {
                particle.pos.y = 0.0;
                particle.vel.y = -particle.vel.y * 0.5;
            }
            if particle.pos.y >= grid_size {
                particle.pos.y = grid_size - 1.0;
                particle.vel.y = -particle.vel.y * 0.5;
            }
        }
    }

    /// Set vibration mode
    pub fn set_mode(&mut self, m: u32, n: u32) {
        self.current_mode = PlateMode::new(m, n);
        self.reset_particles();
    }

    /// Reset particle positions
    pub fn reset_particles(&mut self) {
        self.particles = Self::spawn_particles(self.config.grid_size, self.particles.len());
    }
}

// Thread-local storage for global simulation state
thread_local! {
    static APP: RefCell<Option<App>> = RefCell::new(None);
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
    set_mode_fn.forget(); // Prevent closure from being dropped

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

/// Start the requestAnimationFrame loop
fn start_animation_loop() -> Result<(), JsValue> {
    let window = window().ok_or("No window found")?;

    // Create self-referential closure for animation loop
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
                if container_width != app.canvas.width() || container_height != app.canvas.height() {
                    if container_width > 0 && container_height > 0 {
                        app.canvas.set_width(container_width);
                        app.canvas.set_height(container_height);
                    }
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
