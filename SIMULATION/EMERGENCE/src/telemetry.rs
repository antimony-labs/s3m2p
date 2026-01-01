//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: telemetry.rs | SIMULATION/EMERGENCE/src/telemetry.rs
//! PURPOSE: Performance metrics and sparkline rendering for boids simulation
//! MODIFIED: 2025-12-14
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Telemetry system with:
//! - Ring buffers for time-series data (10-second history at 1Hz sampling)
//! - Sparkline rendering (SVG polylines)
//! - Birth/death tracking
//! - Role distribution counters (Herbivore, Carnivore, Scavenger)
//! - Genetic diversity metrics

use simulation_engine::{BoidArena, BoidRole};
use std::fmt::Write;
use web_sys::Document;

const TELEMETRY_SAMPLES: usize = 10; // ~10 seconds at 1Hz

/// Ring buffer for time-series data with fixed capacity
#[derive(Clone, Debug)]
pub struct RingBuffer<T: Copy + Default, const N: usize> {
    data: [T; N],
    head: usize,
    count: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            head: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        self.data[self.head] = val;
        self.head = (self.head + 1) % N;
        if self.count < N {
            self.count += 1;
        }
    }

    pub fn iter_oldest_first(&self) -> impl Iterator<Item = T> + '_ {
        let start = if self.count < N { 0 } else { self.head };
        (0..self.count).map(move |i| self.data[(start + i) % N])
    }
}

/// Telemetry state tracking per-frame metrics
pub struct TelemetryState {
    // Accumulators (updated every frame from simulation_step returns)
    pub birth_acc: u32,
    pub death_acc: u32,

    // Ring buffers (pushed at 1Hz)
    births_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    deaths_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    herbivore_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    carnivore_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    scavenger_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    diversity_buf: RingBuffer<f32, TELEMETRY_SAMPLES>,

    // Timing
    last_sample_ms: f64,

    // Latest snapshot for display
    pub latest_births: u16,
    pub latest_deaths: u16,
    pub latest_h: u16,
    pub latest_c: u16,
    pub latest_s: u16,
    pub latest_div: f32,

    // Reusable buffer for polyline point strings
    points_buf: String,
}

impl TelemetryState {
    pub fn new(now_ms: f64) -> Self {
        Self {
            birth_acc: 0,
            death_acc: 0,
            births_buf: RingBuffer::new(),
            deaths_buf: RingBuffer::new(),
            herbivore_buf: RingBuffer::new(),
            carnivore_buf: RingBuffer::new(),
            scavenger_buf: RingBuffer::new(),
            diversity_buf: RingBuffer::new(),
            last_sample_ms: now_ms,
            latest_births: 0,
            latest_deaths: 0,
            latest_h: 0,
            latest_c: 0,
            latest_s: 0,
            latest_div: 1.0,
            points_buf: String::with_capacity(128),
        }
    }

    /// Sample metrics at 1Hz (call when 1 second has elapsed)
    pub fn sample<const CAP: usize>(&mut self, arena: &BoidArena<CAP>, diversity: f32, now_ms: f64) {
        self.last_sample_ms = now_ms;

        // Clamp accumulators to u16 range
        let births = (self.birth_acc.min(u16::MAX as u32)) as u16;
        let deaths = (self.death_acc.min(u16::MAX as u32)) as u16;

        // Count current role distribution
        let (h, c, s) = count_roles(arena);

        // Push to ring buffers
        self.births_buf.push(births);
        self.deaths_buf.push(deaths);
        self.herbivore_buf.push(h);
        self.carnivore_buf.push(c);
        self.scavenger_buf.push(s);
        self.diversity_buf.push(diversity);

        // Update latest snapshot
        self.latest_births = births;
        self.latest_deaths = deaths;
        self.latest_h = h;
        self.latest_c = c;
        self.latest_s = s;
        self.latest_div = diversity;

        // Reset per-second counters
        self.birth_acc = 0;
        self.death_acc = 0;
    }

    /// Check if it's time to sample (1 second elapsed)
    pub fn should_sample(&self, now_ms: f64) -> bool {
        (now_ms - self.last_sample_ms) >= 1000.0
    }
}

/// Count role distribution in the arena
fn count_roles<const CAP: usize>(arena: &BoidArena<CAP>) -> (u16, u16, u16) {
    let mut h: u32 = 0;
    let mut c: u32 = 0;
    let mut s: u32 = 0;

    for idx in arena.iter_alive() {
        match arena.roles[idx] {
            BoidRole::Herbivore => h += 1,
            BoidRole::Carnivore => c += 1,
            BoidRole::Scavenger => s += 1,
        }
    }

    (
        h.min(u16::MAX as u32) as u16,
        c.min(u16::MAX as u32) as u16,
        s.min(u16::MAX as u32) as u16,
    )
}

/// Set SVG polyline points attribute
fn set_polyline_points(document: &Document, id: &str, points: &str) {
    if let Some(el) = document.get_element_by_id(id) {
        let _ = el.set_attribute("points", points);
    }
}

/// Update all sparkline SVG elements with current telemetry data
pub fn update_sparklines(document: &Document, telemetry: &mut TelemetryState) {
    const SPARK_W: f32 = 80.0;
    const SPARK_H: f32 = 24.0;
    let x_step = SPARK_W / ((TELEMETRY_SAMPLES as f32) - 1.0);
    let y_span = SPARK_H - 1.0;

    // Reuse the same buffer for each polyline
    let points = &mut telemetry.points_buf;

    // Births/Deaths: normalize by max births/deaths in window
    let mut max_bd: u16 = 1;
    for v in telemetry.births_buf.iter_oldest_first() {
        max_bd = max_bd.max(v);
    }
    for v in telemetry.deaths_buf.iter_oldest_first() {
        max_bd = max_bd.max(v);
    }
    let max_bd_f = max_bd.max(1) as f32;

    // Births sparkline
    points.clear();
    for (i, v) in telemetry.births_buf.iter_oldest_first().enumerate() {
        let x = i as f32 * x_step;
        let y = SPARK_H - ((v as f32 / max_bd_f) * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-births", points);

    // Deaths sparkline
    points.clear();
    for (i, v) in telemetry.deaths_buf.iter_oldest_first().enumerate() {
        let x = i as f32 * x_step;
        let y = SPARK_H - ((v as f32 / max_bd_f) * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-deaths", points);

    // Diversity sparkline (0-1 clamped)
    points.clear();
    for (i, v) in telemetry.diversity_buf.iter_oldest_first().enumerate() {
        let v = v.clamp(0.0, 1.0);
        let x = i as f32 * x_step;
        let y = SPARK_H - (v * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-diversity", points);
}

/// Update DOM text elements with current metrics
pub fn update_telemetry_text(document: &Document, pop: usize, gen: u16, fps: u32, telemetry: &TelemetryState) {
    // Population
    if let Some(el) = document.get_element_by_id("pop-count") {
        el.set_text_content(Some(&pop.to_string()));
    }

    // Generation
    if let Some(el) = document.get_element_by_id("gen-count") {
        el.set_text_content(Some(&gen.to_string()));
    }

    // FPS
    if let Some(el) = document.get_element_by_id("fps-count") {
        el.set_text_content(Some(&fps.to_string()));
    }

    // Role counts (H/C/S)
    if let Some(el) = document.get_element_by_id("role-counts") {
        let text = format!("{}/{}/{}", telemetry.latest_h, telemetry.latest_c, telemetry.latest_s);
        el.set_text_content(Some(&text));
    }
}
