//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: main.rs | WELCOME/src/main.rs
//! PURPOSE: WASM entry point with constellation UI, boid simulation, and fungal network rendering
//! MODIFIED: 2025-12-09
//! LAYER: WELCOME (landing)
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]

use glam::Vec2;
use simulation_engine::{
    apply_predator_zones, compute_diversity, compute_flocking_forces, feed_from_sources,
    get_boid_color, simulation_step, trigger_mass_extinction, BoidArena, BoidRole, BoidState,
    FoodSource, Genome, Obstacle, PredatorZone, SeasonCycle, SimConfig, SpatialGrid,
};
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, Performance};

mod fungal;
use fungal::{FungalNetwork, InteractionResult};

mod shader;
use shader::BackgroundEffect;

mod bubbles;
mod routing;
use bubbles::{get_category, Bubble, BubbleAction, CategoryId, HOME_BUBBLES};
use routing::{get_current_route, navigate_home, navigate_to, Route};

mod arch_diagram;
use arch_diagram::render_architecture_diagram;

/// Type alias for the animation frame closure pattern
type AnimationCallback = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Add binding to update DOM from Rust for Center Animation
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = updateCenterText)]
    fn update_center_text(text: &str, opacity: f32, logo_opacity: f32, use_glitch: bool);
}

// Fixed capacity - no runtime allocations (increased for evolution)
const ARENA_CAPACITY: usize = 4096;
const CELL_CAPACITY: usize = 64;
const BOID_SIZE: f32 = 6.0;
const VISION_RADIUS: f32 = 60.0;

/// Simulation state tracking
struct SimulationStats {
    max_speed_record: f32,
    max_generation: u16,
    low_diversity_frames: u32,
}

// ============================================
// VIEWPORT MODE + DEVICE TUNING
// ============================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewportMode {
    Desktop,
    MobileLandscape,
    MobilePortrait,
}

impl ViewportMode {
    fn detect(width: f64, height: f64) -> Self {
        let min_dim = width.min(height);
        if min_dim < 768.0 {
            let aspect = width / height.max(1.0);
            if aspect < 0.85 {
                ViewportMode::MobilePortrait
            } else {
                ViewportMode::MobileLandscape
            }
        } else {
            ViewportMode::Desktop
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            ViewportMode::Desktop => "desktop",
            ViewportMode::MobileLandscape => "mobile-landscape",
            ViewportMode::MobilePortrait => "mobile-portrait",
        }
    }

    fn tuning(&self) -> ViewportTuning {
        match self {
            ViewportMode::Desktop => ViewportTuning {
                // Default: the full show
                spawn_rate_mult: 1.0,
                max_boids: 500,
            },
            ViewportMode::MobileLandscape => ViewportTuning {
                spawn_rate_mult: 0.7,
                max_boids: 350,
            },
            ViewportMode::MobilePortrait => ViewportTuning {
                spawn_rate_mult: 0.5,
                max_boids: 250,
            },
        }
    }
}

/// Device-specific simulation tuning (NOT `simulation_engine::SimConfig`)
#[derive(Clone, Copy, Debug)]
struct ViewportTuning {
    /// Spawn rate multiplier for the fountain.
    /// - 1.0 => baseline
    /// - <1.0 => spawn less often (slower)
    spawn_rate_mult: f32,
    /// Population cap guard (used for fountain + carrying capacity).
    max_boids: usize,
}

fn update_viewport_mode(document: &Document) -> ViewportMode {
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let mode = ViewportMode::detect(width, height);

    if let Some(html) = document.document_element() {
        html.set_attribute("data-viewport-mode", mode.as_str()).ok();
    }

    mode
}

fn compute_spawn_interval_frames(base_interval: u32, spawn_rate_mult: f32) -> u32 {
    if spawn_rate_mult <= 0.0 {
        return base_interval.saturating_mul(10).max(1);
    }
    // spawn_rate_mult < 1.0 => spawn less often (bigger interval)
    ((base_interval as f32 / spawn_rate_mult).round() as u32).max(1)
}

// ============================================
// TELEMETRY (1Hz sampling + micro-sparklines)
// ============================================

const TELEMETRY_SAMPLES: usize = 10; // ~10 seconds at 1Hz

struct RingBuffer<T: Copy + Default, const N: usize> {
    data: [T; N],
    head: usize,
    count: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    fn new() -> Self {
        Self {
            data: [T::default(); N],
            head: 0,
            count: 0,
        }
    }

    fn push(&mut self, val: T) {
        self.data[self.head] = val;
        self.head = (self.head + 1) % N;
        if self.count < N {
            self.count += 1;
        }
    }

    fn iter_oldest_first(&self) -> impl Iterator<Item = T> + '_ {
        let start = if self.count < N { 0 } else { self.head };
        (0..self.count).map(move |i| self.data[(start + i) % N])
    }
}

struct TelemetryState {
    // Accumulators (updated every frame from simulation_step returns)
    birth_acc: u32,
    death_acc: u32,

    // Ring buffers (pushed at 1Hz)
    births_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    deaths_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    herbivore_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    carnivore_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    scavenger_buf: RingBuffer<u16, TELEMETRY_SAMPLES>,
    diversity_buf: RingBuffer<f32, TELEMETRY_SAMPLES>,

    // Timing
    last_sample_ms: f64,

    // Latest snapshot for peek display
    latest_births: u16,
    latest_deaths: u16,
    latest_h: u16,
    latest_c: u16,
    latest_s: u16,
    latest_div: f32,

    // Scratch (avoid repeated allocations)
    points_buf: String,
}

impl TelemetryState {
    fn new(now_ms: f64) -> Self {
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
}

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

fn set_polyline_points(document: &Document, id: &str, points: &str) {
    if let Some(el) = document.get_element_by_id(id) {
        el.set_attribute("points", points).ok();
    }
}

fn update_sparklines(document: &Document, telemetry: &mut TelemetryState) {
    const SPARK_W: f32 = 40.0;
    const SPARK_H: f32 = 12.0;
    let x_step = SPARK_W / ((TELEMETRY_SAMPLES as f32) - 1.0);
    let y_span = SPARK_H - 1.0;

    // Reuse the same buffer for each polyline
    let points = &mut telemetry.points_buf;

    // BD: normalize by max births/deaths in window
    let mut max_bd: u16 = 1;
    for v in telemetry.births_buf.iter_oldest_first() {
        max_bd = max_bd.max(v);
    }
    for v in telemetry.deaths_buf.iter_oldest_first() {
        max_bd = max_bd.max(v);
    }
    let max_bd_f = max_bd.max(1) as f32;

    // Births (green)
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

    // Deaths (red)
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

    // H/C/S: normalize by max count in window
    let mut max_hcs: u16 = 1;
    for v in telemetry.herbivore_buf.iter_oldest_first() {
        max_hcs = max_hcs.max(v);
    }
    for v in telemetry.carnivore_buf.iter_oldest_first() {
        max_hcs = max_hcs.max(v);
    }
    for v in telemetry.scavenger_buf.iter_oldest_first() {
        max_hcs = max_hcs.max(v);
    }
    let max_hcs_f = max_hcs.max(1) as f32;

    // Herbivores
    points.clear();
    for (i, v) in telemetry.herbivore_buf.iter_oldest_first().enumerate() {
        let x = i as f32 * x_step;
        let y = SPARK_H - ((v as f32 / max_hcs_f) * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-h", points);

    // Carnivores
    points.clear();
    for (i, v) in telemetry.carnivore_buf.iter_oldest_first().enumerate() {
        let x = i as f32 * x_step;
        let y = SPARK_H - ((v as f32 / max_hcs_f) * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-c", points);

    // Scavengers
    points.clear();
    for (i, v) in telemetry.scavenger_buf.iter_oldest_first().enumerate() {
        let x = i as f32 * x_step;
        let y = SPARK_H - ((v as f32 / max_hcs_f) * y_span);
        if i > 0 {
            points.push(' ');
        }
        let _ = write!(points, "{:.1},{:.1}", x, y);
    }
    set_polyline_points(document, "spark-s", points);

    // DIV: 0–1 clamped
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
    set_polyline_points(document, "spark-div", points);
}

fn update_peek_attributes(document: &Document, telemetry: &TelemetryState) {
    if let Some(el) = document.get_element_by_id("stat-bd") {
        let peek = format!(
            "B/D {}/{}",
            telemetry.latest_births, telemetry.latest_deaths
        );
        el.set_attribute("data-peek", &peek).ok();
        el.set_attribute("aria-label", &peek).ok();
        el.set_attribute("title", &peek).ok();
    }
    if let Some(el) = document.get_element_by_id("stat-hcs") {
        let peek = format!(
            "H/C/S {}/{}/{}",
            telemetry.latest_h, telemetry.latest_c, telemetry.latest_s
        );
        el.set_attribute("data-peek", &peek).ok();
        el.set_attribute("aria-label", &peek).ok();
        el.set_attribute("title", &peek).ok();
    }
    if let Some(el) = document.get_element_by_id("stat-div") {
        let peek = format!("DIV {:.2}", telemetry.latest_div);
        el.set_attribute("data-peek", &peek).ok();
        el.set_attribute("aria-label", &peek).ok();
        el.set_attribute("title", &peek).ok();
    }
}

/// Chakravyu zone - the deadly center where boids can enter but not escape
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
struct ChakravyuZone {
    center: Vec2,
    radius: f32,
    _energy_drain: f32,
    inward_force: f32, // Used for rush mechanics
}

// ============================================
// BUBBLE LAYOUT CALCULATIONS (Issue #46)
// ============================================

/// All calculated layout values for the bubble constellation
/// Implements the spacing requirements:
/// - Text size = 10% of bubble diameter
/// - Text gap from bubble = 2% of bubble diameter
/// - Outer margin = 5% of (bubble + text) diameter
#[derive(Clone, Copy, Debug)]
struct BubbleLayout {
    /// Radius of the constellation circle
    big_circle_radius: f64,
    /// Radius of each bubble
    bubble_radius: f64,
    /// Font size for curved text (10% of diameter)
    text_size: f64,
    /// Gap between bubble edge and text (2% of diameter)
    text_gap: f64,
    /// Outer margin between bubbles (5% of effective diameter)
    #[allow(dead_code)]
    outer_margin: f64,
    /// Total radius including bubble + gap + text + margin
    effective_radius: f64,
    /// Distance from center to bubble center
    orbit_radius: f64,
}

impl BubbleLayout {
    /// Calculate layout values given viewport and bubble count
    ///
    /// Key insight: Each bubble's visual footprint = bubble + gap + text
    /// This "effective diameter" must be used for ALL spacing calculations.
    ///
    /// Constraints:
    /// 1. orbit_radius + effective_radius <= big_circle_radius (fit inside)
    /// 2. 2 * orbit_radius * sin(π/N) >= 2 * effective_radius (no overlap)
    ///
    /// We solve for bubble_radius that satisfies BOTH constraints.
    fn calculate(viewport_min: f64, bubble_count: usize, mode: ViewportMode) -> Self {
        // ViewportMode-driven sizing. This provides a single source of truth
        // (aspect ratio + width guard) for layout decisions.
        let size_ratio = match mode {
            ViewportMode::MobilePortrait => 0.85,
            ViewportMode::MobileLandscape => 0.65,
            ViewportMode::Desktop => 0.45,
        };
        let constellation_size = viewport_min * size_ratio;
        let big_circle_radius = constellation_size / 2.0;

        // Text sizing ratios (as fraction of bubble DIAMETER)
        let text_size_ratio = 0.10; // 10% of diameter
        let text_gap_ratio = 0.08; // 8% of diameter
        let edge_margin_ratio = 0.05; // 5% margin from constellation edge

        // effective_radius = bubble_radius + text_gap + text_size
        //                  = r + (2r * text_gap_ratio) + (2r * text_size_ratio)
        //                  = r * (1 + 2*0.08 + 2*0.10) = r * 1.36
        let effective_multiplier = 1.0 + 2.0 * text_gap_ratio + 2.0 * text_size_ratio;

        let bubble_radius = if bubble_count > 1 {
            let half_angle = std::f64::consts::PI / bubble_count as f64;
            let sin_half = half_angle.sin();

            // Constraint 1: orbit + effective_radius * (1 + edge_margin) <= big_circle_radius
            //   orbit <= big_circle_radius - effective_multiplier * r * 1.05
            //
            // Constraint 2: orbit >= effective_radius / sin(half_angle)
            //   orbit >= effective_multiplier * r / sin_half
            //
            // For both to be satisfiable:
            //   effective_multiplier * r / sin_half <= big_circle_radius - effective_multiplier * r * 1.05
            //   effective_multiplier * r * (1/sin_half + 1.05) <= big_circle_radius
            //   r <= big_circle_radius / (effective_multiplier * (1/sin_half + 1.05))

            let constraint_factor = 1.0 / sin_half + 1.0 + edge_margin_ratio;
            let max_radius = big_circle_radius / (effective_multiplier * constraint_factor);

            // Apply practical limits
            let min_radius = 15.0; // Minimum for usability
            let max_practical = 55.0; // Maximum to prevent huge bubbles

            max_radius.max(min_radius).min(max_practical)
        } else {
            // Single bubble: center it, make it reasonably sized
            (big_circle_radius * 0.35).clamp(15.0, 50.0)
        };
        let bubble_radius = match mode {
            ViewportMode::MobilePortrait | ViewportMode::MobileLandscape => bubble_radius.max(22.0),
            ViewportMode::Desktop => bubble_radius,
        };

        // Calculate all derived values from bubble_radius
        let diameter = bubble_radius * 2.0;
        let text_size = diameter * text_size_ratio;
        let text_gap = diameter * text_gap_ratio;
        let effective_radius = bubble_radius + text_gap + text_size;
        let outer_margin = effective_radius * edge_margin_ratio;

        // Calculate orbit radius: place bubbles as far out as possible
        // while keeping effective_radius inside big_circle
        let orbit_radius = if bubble_count > 1 {
            big_circle_radius - effective_radius - outer_margin
        } else {
            0.0 // Single bubble at center
        };

        BubbleLayout {
            big_circle_radius,
            bubble_radius,
            text_size,
            text_gap,
            outer_margin,
            effective_radius,
            orbit_radius,
        }
    }

    /// Verify no bubbles overlap (returns true if layout is valid)
    #[allow(dead_code)]
    fn validate(&self, bubble_count: usize) -> bool {
        if bubble_count <= 1 {
            return true;
        }

        let angle_step = std::f64::consts::TAU / bubble_count as f64;
        let min_distance = 2.0 * self.effective_radius + 0.5; // 0.5px epsilon

        // Check distance between adjacent bubbles
        // d = 2 * orbit * sin(angle_step / 2)
        let actual_distance = 2.0 * self.orbit_radius * (angle_step / 2.0).sin();

        actual_distance >= min_distance
    }
}

/// Update the single-line console log (replaces content)
fn log_event(document: &Document, msg: &str, event_class: &str) {
    if let Some(console_log) = document.get_element_by_id("console-log") {
        // Create a span with the message and class
        let styled_msg = format!("<span class=\"{}\">{}</span>", event_class, msg);
        console_log.set_inner_html(&styled_msg);
    }
}

/// Update commit info display with build-time git information
fn update_commit_info(document: &Document) {
    const COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
    const COMMIT_TIME: &str = env!("GIT_COMMIT_TIME");

    if let Some(commit_link) = document.get_element_by_id("commit-link") {
        // Parse timestamp and calculate time ago
        let commit_timestamp: i64 = COMMIT_TIME.parse().unwrap_or(0);
        let now = js_sys::Date::now() / 1000.0; // Convert ms to seconds
        let seconds_ago = (now as i64) - commit_timestamp;

        let time_ago = if seconds_ago < 60 {
            format!("{}s ago", seconds_ago)
        } else if seconds_ago < 3600 {
            format!("{}m ago", seconds_ago / 60)
        } else if seconds_ago < 86400 {
            format!("{}h ago", seconds_ago / 3600)
        } else {
            format!("{}d ago", seconds_ago / 86400)
        };

        // GitHub commit URL
        let commit_url = format!(
            "https://github.com/Shivam-Bhardwaj/S3M2P/commit/{}",
            COMMIT_HASH
        );

        // Update link
        commit_link.set_attribute("href", &commit_url).ok();
        commit_link.set_text_content(Some(&format!("{} • {}", COMMIT_HASH, time_ago)));
    }
}

/// Exclusion zone around UI elements where nothing should spawn/grow
#[derive(Clone, Copy, Debug)]
struct ExclusionZone {
    center: Vec2,
    radius: f32,
}

/// Scan DOM for monolith elements and create exclusion zones
/// Updated for circular layout: creates a large exclusion circle in the center
fn scan_exclusion_zones(document: &Document) -> (Vec<ExclusionZone>, Option<ChakravyuZone>) {
    let mut zones = Vec::new();
    let mut chakravyu = None;

    // Center constellation exclusion
    if let Some(constellation) = document.get_element_by_id("constellation") {
        let rect = constellation.get_bounding_client_rect();
        let center_x = rect.left() as f32 + rect.width() as f32 / 2.0;
        let center_y = rect.top() as f32 + rect.height() as f32 / 2.0;
        // Radius covers the whole ring + padding (for fungus exclusion)
        let outer_radius = (rect.width().max(rect.height()) as f32) / 2.0 + 20.0;

        // NOTE: We add the center to 'zones' vector to prevent FUNGAL GROWTH.
        // The boid repulsion logic handles the conflict (Rush force > Repulsion force).
        zones.push(ExclusionZone {
            center: Vec2::new(center_x, center_y),
            radius: outer_radius,
        });

        // Chakravyu zone - Deadly center
        // Use outer_radius for the trap threshold to match the visual ring
        chakravyu = Some(ChakravyuZone {
            center: Vec2::new(center_x, center_y),
            radius: outer_radius,
            _energy_drain: 0.5,
            inward_force: 2.0,
        });
    } else {
        // Fallback to scanning individual monoliths if constellation not found
        let elements = document.get_elements_by_class_name("monolith");
        for i in 0..elements.length() {
            if let Some(element) = elements.item(i) {
                let rect = element.get_bounding_client_rect();
                let center_x = rect.left() as f32 + rect.width() as f32 / 2.0;
                let center_y = rect.top() as f32 + rect.height() as f32 / 2.0;
                let radius = (rect.width().max(rect.height()) as f32) / 2.0 + 40.0;

                zones.push(ExclusionZone {
                    center: Vec2::new(center_x, center_y),
                    radius,
                });
            }
        }
    }

    (zones, chakravyu)
}

/// Check if a position is inside any exclusion zone
fn is_in_exclusion_zone(pos: Vec2, zones: &[ExclusionZone]) -> bool {
    for zone in zones {
        if pos.distance(zone.center) < zone.radius {
            return true;
        }
    }
    false
}

struct PopUp {
    text: String,
    pos: Vec2,
    life: f32, // 0.0 to 1.0
    color: String,
}

struct Miasma {
    pos: Vec2,
    vel: Vec2,
    life: f32, // 0.0 to 1.0
    radius: f32,
    color: String,
}

struct World {
    arena: BoidArena<ARENA_CAPACITY>,
    grid: SpatialGrid<CELL_CAPACITY>,
    obstacles: Vec<Obstacle>,
    exclusion_zones: Vec<ExclusionZone>,
    chakravyu: Option<ChakravyuZone>,
    food_sources: Vec<FoodSource>,
    fungal_network: FungalNetwork,
    background: BackgroundEffect,
    predators: Vec<PredatorZone>,
    season: SeasonCycle,
    config: SimConfig,
    width: f32,
    height: f32,
    popups: Vec<PopUp>,
    miasma: Vec<Miasma>,
}

fn scan_dom_obstacles(document: &Document) -> Vec<Obstacle> {
    let mut obstacles = Vec::new();
    let elements = document.get_elements_by_class_name("monolith");

    for i in 0..elements.length() {
        if let Some(element) = elements.item(i) {
            let rect = element.get_bounding_client_rect();
            let center_x = rect.left() as f32 + rect.width() as f32 / 2.0;
            let center_y = rect.top() as f32 + rect.height() as f32 / 2.0;
            let radius = (rect.width().max(rect.height()) as f32) / 2.0;

            obstacles.push(Obstacle {
                center: Vec2::new(center_x, center_y),
                radius,
            });
        }
    }
    obstacles
}

fn is_paused() -> bool {
    let window = window().unwrap();
    let location = window.location();
    if let Ok(search) = location.search() {
        search.contains("paused=true")
    } else {
        false
    }
}

// ============================================
// BUBBLE RENDERING
// ============================================

/// Create a straight text label element below the bubble
/// Shows both label (top line) and description (bottom line)
fn create_bubble_label(
    document: &Document,
    label: &str,
    description: &str,
) -> Option<web_sys::Element> {
    // Create container div
    let container = document.create_element("div").ok()?;
    container.set_attribute("class", "bubble-label-container").ok();

    // Create label span
    let label_el = document.create_element("span").ok()?;
    label_el.set_attribute("class", "bubble-label-text").ok();
    label_el.set_text_content(Some(label));
    container.append_child(&label_el).ok();

    // Create description span
    let desc_el = document.create_element("span").ok()?;
    desc_el.set_attribute("class", "bubble-desc-text").ok();
    desc_el.set_text_content(Some(description));
    container.append_child(&desc_el).ok();

    Some(container)
}

/// Clear existing bubbles and render new ones
fn render_bubbles(document: &Document, bubbles: &[Bubble], show_back: bool) {
    let constellation = match document.get_element_by_id("constellation") {
        Some(el) => el,
        None => return,
    };

    // Remove existing bubbles and labels
    let monoliths = document.get_elements_by_class_name("monolith");
    while monoliths.length() > 0 {
        if let Some(el) = monoliths.item(0) {
            el.remove();
        }
    }
    let text_arcs = document.get_elements_by_class_name("bubble-text-arc");
    while text_arcs.length() > 0 {
        if let Some(el) = text_arcs.item(0) {
            el.remove();
        }
    }
    let label_containers = document.get_elements_by_class_name("bubble-label-container");
    while label_containers.length() > 0 {
        if let Some(el) = label_containers.item(0) {
            el.remove();
        }
    }

    // Show/hide back button
    if let Some(back_btn) = document.get_element_by_id("back-button") {
        if show_back {
            back_btn.set_attribute("style", "display: flex;").ok();
        } else {
            back_btn.set_attribute("style", "display: none;").ok();
        }
    }

    // ============================================
    // CALCULATE LAYOUT USING NEW ALGORITHM (Issue #46)
    // ============================================

    let window = web_sys::window().unwrap();
    let viewport_width = window.inner_width().unwrap().as_f64().unwrap();
    let viewport_height = window.inner_height().unwrap().as_f64().unwrap();
    let mode = ViewportMode::detect(viewport_width, viewport_height);

    // Get telemetry bar height (if exists)
    let telemetry_height = document
        .get_element_by_id("telemetry-bar")
        .map(|el| el.get_bounding_client_rect().height())
        .unwrap_or(0.0);

    // Available vertical space = viewport - telemetry
    let available_height = viewport_height - telemetry_height;
    let available_min = viewport_width.min(available_height);

    // Calculate layout using new algorithm
    let bubble_count = bubbles.len();
    let layout = BubbleLayout::calculate(available_min, bubble_count, mode);

    // Derived values for positioning
    let constellation_size = layout.big_circle_radius * 2.0;
    let bubble_size = layout.bubble_radius * 2.0;
    let orbit_radius = layout.orbit_radius;

    let angle_step = std::f64::consts::TAU / bubble_count as f64;
    let start_angle = -std::f64::consts::FRAC_PI_2;

    // Calculate vertical offset to center the visual mass
    // Each bubble's visual center is shifted down by (text_gap + text_size)
    // So we shift the entire constellation UP by that full amount
    let vertical_offset = layout.text_gap + layout.text_size;

    // Set CSS variables dynamically, including the vertical offset
    constellation
        .set_attribute(
            "style",
            &format!(
                "--constellation-size: {:.1}px; --bubble-size: {:.1}px; --orbit-radius: {:.1}px; --text-size: {:.1}px; transform: translate(-50%, -50%) translateY(-{:.1}px);",
                constellation_size, bubble_size, orbit_radius, layout.text_size, vertical_offset
            ),
        )
        .ok();

    // Debug log to verify calculations
    web_sys::console::log_1(&format!(
        "Layout({}): viewport={}x{}, constellation={:.0}, bubble_r={:.1}, text={:.1}, orbit={:.1}, effective_r={:.1}",
        mode.as_str(),
        viewport_width as i32,
        viewport_height as i32,
        constellation_size,
        layout.bubble_radius,
        layout.text_size,
        orbit_radius,
        layout.effective_radius
    )
    .into());

    for (i, bubble) in bubbles.iter().enumerate() {
        let angle = start_angle + (i as f64 * angle_step);
        let angle_deg = angle.to_degrees();

        // Calculate bubble position in pixels (for SVG positioning)
        let bubble_x = constellation_size / 2.0 + orbit_radius * angle.cos();
        let bubble_y = constellation_size / 2.0 + orbit_radius * angle.sin();

        // Create the bubble element
        let link = document.create_element("a").unwrap();
        link.set_class_name("monolith");

        // Accessibility: title and aria-label
        let a11y_label = format!("{} — {}", bubble.label, bubble.description);
        link.set_attribute("title", &a11y_label).ok();
        link.set_attribute("aria-label", &a11y_label).ok();

        // Set position with inline transform
        let pos_style = format!(
            "transform: translate(-50%, -50%) rotate({:.1}deg) translate(var(--orbit-radius)) rotate({:.1}deg);",
            angle_deg, -angle_deg
        );
        link.set_attribute("style", &pos_style).ok();

        // Set href/click based on action
        match bubble.action {
            BubbleAction::External(url) => {
                link.set_attribute("href", url).ok();
                link.set_attribute("target", "_blank").ok();
                link.set_attribute("rel", "noopener noreferrer").ok();
            }
            BubbleAction::DirectProject(url) => {
                // Use relative protocol to support both http and https
                // If we are on localhost, we might want to use port-based URLs
                // But for now, let's assume the URL provided in bubbles.rs is correct
                // or we can make it relative if it's a subdomain.

                // Check if we are in dev mode (localhost/127.0.0.1)
                let window = web_sys::window().unwrap();
                let hostname = window.location().hostname().unwrap_or_default();

                let final_url = if hostname == "localhost" || hostname == "127.0.0.1" {
                    // In dev mode, we might need to map subdomains to ports if not using a proxy
                    // But if the user set up /etc/hosts, subdomains work.
                    // If they use ports, we need a mapping.
                    // For now, let's trust the URL but ensure it's protocol-relative
                    url.to_string()
                } else {
                    url.to_string()
                };

                link.set_attribute("href", &final_url).ok();
            }
            BubbleAction::Category(cat_id) => {
                let hash = cat_id.hash_route();
                link.set_attribute("href", hash).ok();
            }
            BubbleAction::Profile => {
                link.set_attribute("href", "#/profile").ok();
            }
        }

        // Add icon
        let img = document.create_element("img").unwrap();
        let icon_src = format!("{}?v=8", bubble.icon);
        img.set_attribute("src", &icon_src).ok();
        img.set_attribute("alt", bubble.label).ok();
        link.append_child(&img).ok();

        // Add to constellation
        constellation.append_child(&link).ok();

        // Create and position straight text label below bubble
        if let Some(label_el) = create_bubble_label(document, bubble.label, bubble.description) {
            // Position label below the bubble
            let label_top = bubble_y + layout.bubble_radius + 8.0; // 8px gap below bubble
            let label_style = format!(
                "left: {:.1}px; top: {:.1}px;",
                bubble_x,
                label_top
            );
            label_el.set_attribute("style", &label_style).ok();
            constellation.append_child(&label_el).ok();
        }
    }
}

/// Render the home page bubbles
fn render_home(document: &Document) {
    // Ensure center bubble is present
    render_center_bubble(document);
    render_bubbles(document, HOME_BUBBLES, false);
}

/// Render a category page
fn render_category(document: &Document, category_id: CategoryId) {
    let category = get_category(category_id);
    // Ensure center bubble is present
    render_center_bubble(document);
    render_bubbles(document, category.bubbles, true);
}

/// Handle route changes
fn handle_route_change(document: &Document) {
    let route = get_current_route();

    // Toggle containers
    let arch_container = document.get_element_by_id("arch-container");
    let about_container = document.get_element_by_id("about-container");
    let profile_container = document.get_element_by_id("profile-container");
    let back_button = document.get_element_by_id("back-button");

    // Handle arch-container visibility
    if let Some(container) = arch_container {
        if matches!(route, Route::Architecture) {
            container.set_attribute("style", "display: flex; position: fixed; inset: 0; background: rgba(5, 5, 8, 0.95); z-index: 5000; justify-content: center; align-items: center;").ok();
            if let Some(btn) = back_button.as_ref() {
                btn.set_attribute("style", "display: flex; z-index: 5001;").ok();
            }
        } else {
            container.set_attribute("style", "display: none;").ok();
        }
    }

    // Handle about-container visibility
    if let Some(container) = about_container {
        if matches!(route, Route::About) {
            container.set_attribute("style", "display: flex;").ok();
            if let Some(btn) = back_button.as_ref() {
                btn.set_attribute("style", "display: flex; z-index: 5001;").ok();
            }
        } else {
            container.set_attribute("style", "display: none;").ok();
        }
    }

    // Handle profile-container visibility
    if let Some(container) = profile_container {
        if matches!(route, Route::Profile) {
            container.set_attribute("style", "display: flex;").ok();
            if let Some(btn) = back_button.as_ref() {
                btn.set_attribute("style", "display: flex; z-index: 5001;").ok();
            }
        } else {
            container.set_attribute("style", "display: none;").ok();
        }
    }

    match route {
        Route::Home => render_home(document),
        Route::Category(cat_id) => render_category(document, cat_id),
        Route::Architecture => render_architecture_diagram(document),
        Route::About => render_about_page(document),
        Route::Profile => render_profile_page(document),
    }
}

/// Render the central Antimony bubble
fn render_center_bubble(document: &Document) {
    let center_core = match document.get_element_by_id("center-core") {
        Some(el) => el,
        None => return,
    };

    // Clear previous content (except text container which we want to keep/manage)
    // Actually, let's just append the image if it doesn't exist.
    if document.get_element_by_id("antimony-bubble").is_some() {
        return;
    }

    // Create the Antimony Bubble Image
    let img = document.create_element("img").unwrap();
    img.set_id("antimony-bubble");
    img.set_attribute("src", "assets/islands/antimony.svg").ok();
    img.set_attribute("alt", "Antimony Architecture").ok();

    // Click -> Navigate to About page
    let on_click = Closure::wrap(Box::new(move || {
        navigate_to(Route::About);
    }) as Box<dyn FnMut()>);
    img.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
        .ok();
    on_click.forget();

    // Insert before text container
    if let Some(text_container) = document.get_element_by_id("center-text-container") {
        center_core.insert_before(&img, Some(&text_container)).ok();
    } else {
        center_core.append_child(&img).ok();
    }
}

/// Render the About Antimony Labs intro page
fn render_about_page(document: &Document) {
    let container = match document.get_element_by_id("about-container") {
        Some(el) => el,
        None => return,
    };
    container.set_inner_html("");

    let panel = document.create_element("div").unwrap();
    panel.set_attribute("class", "about-panel").ok();
    panel.set_inner_html(
        r##"
        <button id="about-close">&times;</button>
        <div class="about-logo">
            <img src="assets/islands/antimony.svg" alt="Antimony Labs" />
        </div>
        <h1>Antimony Labs</h1>
        <p class="tagline">Let AI design, humans build.</p>

        <div class="about-sections">
            <div class="about-section">
                <h2>What We Build</h2>
                <p>Open-source engineering tools, simulations, and manufacturing compilers - built in Rust/WASM/WebGPU.</p>
            </div>

            <div class="about-section">
                <h2>The Vision</h2>
                <p>A compiler for physical products - one unified system that outputs manufacturing-ready artifacts. From CAD to G-code, Gerber files to BOMs.</p>
            </div>

            <div class="about-section">
                <h2>Explore</h2>
                <ul class="about-links">
                    <li><a href="#/tools">Tools</a> - Engineering applications</li>
                    <li><a href="#/sims">Simulations</a> - Interactive physics demos</li>
                    <li><a href="#/learn">Learn</a> - Tutorials on AI, robotics, embedded</li>
                </ul>
            </div>
        </div>

        <div class="about-cta">
            <a href="#" class="back-home-btn">Back to Home</a>
        </div>
    "##,
    );
    container.append_child(&panel).ok();

    // Close button handler
    if let Some(close_btn) = document.get_element_by_id("about-close") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            navigate_home();
        }) as Box<dyn FnMut(_)>);
        close_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // Back home button handler
    if let Some(back_btn) = document.query_selector(".back-home-btn").ok().flatten() {
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            e.prevent_default();
            navigate_home();
        }) as Box<dyn FnMut(_)>);
        back_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }
}

/// Render the personal profile page
fn render_profile_page(document: &Document) {
    let container = match document.get_element_by_id("profile-container") {
        Some(el) => el,
        None => return,
    };
    container.set_inner_html("");

    let panel = document.create_element("div").unwrap();
    panel.set_attribute("class", "profile-panel").ok();
    panel.set_inner_html(
        r##"
        <button id="profile-close">&times;</button>

        <div class="profile-header">
            <h1>SHIVAM BHARDWAJ</h1>
            <p class="title">Senior Robotics & Automation Engineer</p>
            <p class="location">San Jose, CA · H1B Transfer Ready</p>
        </div>

        <div class="profile-about">
            <p>I build embedded and mechatronic systems that ship. 6+ years delivering safety-critical robotics across self-driving vehicles, surgical robots, datacenter automation, and semiconductor equipment.</p>
            <p class="clients-line">Supported <span class="highlight">Meta</span>, <span class="highlight">Tesla</span>, <span class="highlight">Apple</span>, <span class="highlight">Amazon Robotics</span>, <span class="highlight">Applied Materials</span>, and <span class="highlight">Saildrone</span>.</p>
        </div>

        <div class="impact-metrics">
            <div class="metric-card">
                <span class="metric-context">AutoCrate</span>
                <span class="metric-value">5d→1hr</span>
                <span class="metric-label">Design Time</span>
            </div>
            <div class="metric-card">
                <span class="metric-context">Harness Production</span>
                <span class="metric-value">70→10%</span>
                <span class="metric-label">Defect Rate</span>
            </div>
            <div class="metric-card">
                <span class="metric-context">Surgical Robot</span>
                <span class="metric-value">0.1mm</span>
                <span class="metric-label">Registration</span>
            </div>
            <div class="metric-card">
                <span class="metric-context">Manufacturing</span>
                <span class="metric-value">2.8×</span>
                <span class="metric-label">Scale-up</span>
            </div>
        </div>

        <div class="featured-section">
            <div class="section-title">Featured Work · Click for Details</div>
            <div class="featured-grid">
                <div class="featured-card" data-project="autocrate">
                    <div class="featured-name">AutoCrate Design Engine →</div>
                    <div class="featured-desc">Reverse-engineered <a href="https://www.plm.automation.siemens.com/global/en/products/nx/" target="_blank">Siemens NX</a> Expressions to build a parametric crate design engine. Reduced design time from 5 days to 1 hour. Generates <a href="https://www.astm.org/d6251-13.html" target="_blank">ASTM D6251</a>-compliant shipping crate designs.</div>
                    <div class="featured-tags">
                        <span>Siemens NX API</span><span>React</span><span>Python</span><span>ASTM D6251</span>
                    </div>
                </div>
                <div class="featured-card" data-project="surgical">
                    <div class="featured-name">Surgical Robot Registration →</div>
                    <div class="featured-desc"><a href="https://en.wikipedia.org/wiki/Iterative_closest_point" target="_blank">ICP algorithm</a> for femur-to-drill alignment using <a href="https://vtk.org" target="_blank">VTK</a>/<a href="https://pointclouds.org" target="_blank">PCL</a>. Processed 10,000+ point clouds in 300ms with 0.1mm accuracy. <a href="https://www.fda.gov/medical-devices/premarket-submissions-selecting-and-preparing-correct-submission/premarket-notification-510k" target="_blank">FDA 510(k)</a> compliant.</div>
                    <div class="featured-tags">
                        <span>C++</span><span>VTK</span><span>PCL</span><span>FDA 510(k)</span>
                    </div>
                </div>
                <div class="featured-card" data-project="forensics">
                    <div class="featured-name">Robotic Forensics Workcell →</div>
                    <div class="featured-desc">Multi-modal sensing for server counterfeit detection at <a href="https://about.meta.com" target="_blank">Meta</a>. Combined RF spectrum analysis, <a href="https://www.flir.com" target="_blank">thermal imaging</a> (±0.5°C), capacitance probing, and <a href="https://opencv.org" target="_blank">machine vision</a>. <a href="https://www.ni.com/en/shop/labview.html" target="_blank">LabVIEW</a> UI for test automation.</div>
                    <div class="featured-tags">
                        <span>Sensor Fusion</span><span>Computer Vision</span><span>Meta</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="timeline-section">
            <div class="section-title">Experience</div>
            <div class="timeline">
                <div class="timeline-entry">
                    <div class="timeline-period">2023 — Present</div>
                    <div class="timeline-role">Mechatronics Engineer</div>
                    <div class="timeline-company"><a href="https://www.designvisionaries.com" target="_blank">Design Visionaries</a> <span class="timeline-location">· San Jose, CA</span></div>
                    <div class="timeline-desc">AutoCrate design engine, 10-layer AR glasses flex PCB, harness production scaling (25→70 units/day), MBD transition for semiconductor lab.</div>
                    <div class="timeline-clients"><a href="https://www.appliedmaterials.com" target="_blank">Applied Materials</a>, <a href="https://www.saildrone.com" target="_blank">Saildrone</a>, Industrial IoT</div>
                </div>
                <div class="timeline-entry">
                    <div class="timeline-period">2022 — 2023</div>
                    <div class="timeline-role">Engineering Manager</div>
                    <div class="timeline-company">Advanced Engineering Services <span class="timeline-location">· San Jose, CA</span></div>
                    <div class="timeline-desc">Robotic forensics workcell, Class-8 diesel-to-electric conversion, waveguide frame design for AR hardware demos.</div>
                    <div class="timeline-clients"><a href="https://about.meta.com/realitylabs/" target="_blank">Meta Reality Labs</a>, <a href="https://www.appliedmaterials.com" target="_blank">Applied Materials</a>, AAA</div>
                </div>
                <div class="timeline-entry">
                    <div class="timeline-period">2021 — 2022</div>
                    <div class="timeline-role">Senior Robotics Engineer</div>
                    <div class="timeline-company"><a href="https://velodynelidar.com" target="_blank">Velodyne Lidar</a> <span class="timeline-location">· Alameda, CA</span></div>
                    <div class="timeline-desc">Next-gen LiDAR validation at highway speeds, pioneered first SaaS deployment with <a href="https://www.ansible.com" target="_blank">Ansible</a> automation, <a href="https://en.wikipedia.org/wiki/Precision_Time_Protocol" target="_blank">PTP</a> sync and <a href="https://en.wikipedia.org/wiki/Real-time_kinematic_positioning" target="_blank">RTK GPS</a> integration.</div>
                    <div class="timeline-clients"><a href="https://www.ford.com" target="_blank">Ford</a>, <a href="https://www.aboutamazon.com/news/transportation/amazon-scout" target="_blank">Amazon Scout</a>, <a href="https://www.knightscope.com" target="_blank">Knightscope</a>, Bluecity</div>
                </div>
                <div class="timeline-entry">
                    <div class="timeline-period">2020</div>
                    <div class="timeline-role">Robotics Software Engineer</div>
                    <div class="timeline-company">ARI (stealth) <span class="timeline-location">· Sunnyvale, CA</span></div>
                    <div class="timeline-desc">Surgical registration algorithm (0.1mm accuracy), 6-DOF <a href="https://www.kuka.com" target="_blank">Kuka</a> robot control via <a href="https://www.beckhoff.com" target="_blank">Beckhoff</a> PLC, <a href="https://www.fda.gov/medical-devices/premarket-submissions-selecting-and-preparing-correct-submission/premarket-notification-510k" target="_blank">FDA 510(k)</a> compliance. Acquired by <a href="https://www.zimmerbiomet.com" target="_blank">Zimmer Biomet</a> via Monogram.</div>
                    <div class="timeline-clients"><a href="https://www.iso.org/standard/38421.html" target="_blank">IEC 62304</a>, Surgical Robotics</div>
                </div>
                <div class="timeline-entry">
                    <div class="timeline-period">2019</div>
                    <div class="timeline-role">Visiting Researcher</div>
                    <div class="timeline-company"><a href="https://ai4ce.github.io" target="_blank">NYU AI4CE Lab</a> <span class="timeline-location">· Brooklyn, NY</span></div>
                    <div class="timeline-desc">GPS-denied visual localization pipeline using <a href="https://colmap.github.io" target="_blank">COLMAP</a> 3D reconstruction. Fine-tuned neural network achieving ~10cm relocalization accuracy.</div>
                    <div class="timeline-clients">Prof. <a href="https://engineering.nyu.edu/faculty/chen-feng" target="_blank">Chen Feng</a></div>
                </div>
            </div>
        </div>

        <div class="education-section">
            <div class="section-title">Education</div>
            <div class="education-entry">
                <div class="edu-degree">M.S. Mechatronics & Robotics</div>
                <div class="edu-school"><a href="https://www.nyu.edu" target="_blank">New York University</a></div>
                <div class="edu-detail">Research: <a href="https://colmap.github.io" target="_blank">COLMAP</a>-based visual relocalization, swarm robotics. Founded Self-Driving VIP team — 1st Novel Design, 3rd overall at <a href="https://www.igvc.org" target="_blank">26th IGVC</a>.</div>
            </div>
            <div class="education-entry">
                <div class="edu-degree">B.Tech Electronics</div>
                <div class="edu-school"><a href="https://www.ipu.ac.in" target="_blank">I.P. University, Delhi</a></div>
                <div class="edu-detail">Top-3 Projects Award. Built <a href="https://ardupilot.org" target="_blank">ArduPilot</a> autonomous drone.</div>
            </div>
        </div>

        <div class="skills-section">
            <div class="section-title">Skills</div>
            <div class="skills-categories">
                <div class="skill-category">
                    <span class="skill-cat-name">Robotics</span>
                    <div class="skill-cat-tags">
                        <span>ROS2</span><span>MoveIt</span><span>Nav2</span><span>SLAM</span><span>Sensor Fusion</span>
                    </div>
                </div>
                <div class="skill-category">
                    <span class="skill-cat-name">Languages</span>
                    <div class="skill-cat-tags">
                        <span>C++</span><span>Rust</span><span>Python</span><span>Embedded C</span>
                    </div>
                </div>
                <div class="skill-category">
                    <span class="skill-cat-name">Hardware</span>
                    <div class="skill-cat-tags">
                        <span>PCB Design</span><span>Siemens NX</span><span>SolidWorks</span><span>GD&T</span>
                    </div>
                </div>
                <div class="skill-category">
                    <span class="skill-cat-name">Embedded</span>
                    <div class="skill-cat-tags">
                        <span>STM32</span><span>ESP32</span><span>I2C</span><span>CAN</span><span>EtherCAT</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="social-links">
            <a href="https://github.com/Shivam-Bhardwaj" target="_blank" rel="noopener noreferrer" class="social-link">
                <svg viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                GitHub
            </a>
            <a href="https://linkedin.com/in/shivambdj" target="_blank" rel="noopener noreferrer" class="social-link">
                <svg viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
                LinkedIn
            </a>
            <a href="https://x.com/LazyShivam" target="_blank" rel="noopener noreferrer" class="social-link">
                <svg viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                X
            </a>
        </div>
    "##,
    );
    container.append_child(&panel).ok();

    // Close button handler
    if let Some(close_btn) = document.get_element_by_id("profile-close") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            navigate_home();
        }) as Box<dyn FnMut(_)>);
        close_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // Featured work card click handlers
    let featured_cards = document.get_elements_by_class_name("featured-card");
    for i in 0..featured_cards.length() {
        if let Some(card) = featured_cards.item(i) {
            if let Ok(card_html) = card.dyn_into::<web_sys::HtmlElement>() {
                let project_id = card_html.get_attribute("data-project").unwrap_or_default();
                let project_id_clone = project_id.clone();

                let document_clone = document.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    render_case_study(&document_clone, &project_id_clone);
                }) as Box<dyn FnMut(_)>);

                card_html
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .ok();
                closure.forget();
            }
        }
    }
}

/// Render case study modal for a specific project
fn render_case_study(document: &Document, project_id: &str) {
    let container = match document.get_element_by_id("case-study-container") {
        Some(el) => el,
        None => return,
    };

    // Show the container
    container.set_attribute("style", "display: flex;").ok();
    container.set_inner_html("");

    let panel = document.create_element("div").unwrap();
    panel.set_attribute("class", "case-study-panel").ok();

    // Generate content based on project
    let content = match project_id {
        "autocrate" => r##"
        <button id="case-study-close">&times;</button>
        <div class="case-study-header">
            <h1>AutoCrate Design Engine</h1>
            <div class="case-study-meta">Design Visionaries · 2023-2025 · <a href="https://www.appliedmaterials.com" target="_blank">Applied Materials</a>, <a href="https://www.saildrone.com" target="_blank">Saildrone</a></div>
        </div>

        <div class="case-study-section">
            <h2>Problem</h2>
            <p><a href="https://www.appliedmaterials.com" target="_blank">Applied Materials</a> needed custom <a href="https://www.astm.org/d6251-13.html" target="_blank">ASTM D6251</a>-compliant shipping crates for semiconductor equipment. Traditional CAD workflow took 5 days per design using <a href="https://www.plm.automation.siemens.com/global/en/products/nx/" target="_blank">Siemens NX</a>'s slow Open API. Custom crate adoption was only 20% due to long lead times.</p>
        </div>

        <div class="case-study-section">
            <h2>Solution</h2>
            <p>Reverse-engineered <a href="https://www.plm.automation.siemens.com/global/en/products/nx/" target="_blank">Siemens NX</a>'s internal Expression system (bypassing the bottleneck API) to create a parametric design engine that auto-generates crate assemblies from specifications.</p>
            <p>Built a <a href="https://react.dev" target="_blank">React</a>/<a href="https://www.python.org" target="_blank">Python</a> GUI where users input dimensions, weight, and compliance requirements. The engine outputs:</p>
            <ul>
                <li><a href="https://www.astm.org/d6251-13.html" target="_blank">ASTM D6251</a>-compliant technical drawings</li>
                <li>3D <a href="https://en.wikipedia.org/wiki/ISO_10303-21" target="_blank">STEP</a> assembly files (NX-importable)</li>
                <li>Bill of Materials (BOM)</li>
                <li>Cut lists for manufacturing</li>
            </ul>
        </div>

        <div class="case-study-highlight">
            <strong>Impact: 5 days → 1 hour</strong> design time reduction<br>
            Custom crate adoption increased from <strong>20% → 50%</strong>
        </div>

        <div class="case-study-section">
            <h2>Technical Approach</h2>
            <p>The key breakthrough was discovering that NX Expressions (an internal formula system) could be manipulated programmatically without the slow Open API. I wrote a Python script that:</p>
            <ul>
                <li>Parses user specs (L×W×H, weight, shock/vibration requirements)</li>
                <li>Calculates structural members per <a href="https://www.astm.org/d6251-13.html" target="_blank">ASTM D6251</a> formulas</li>
                <li>Generates NX Expression files that rebuild the assembly parametrically</li>
                <li>Exports drawings with <a href="https://en.wikipedia.org/wiki/Geometric_dimensioning_and_tolerancing" target="_blank">GD&T</a> annotations for manufacturing</li>
            </ul>
        </div>

        <div class="case-study-tags">
            <span class="case-study-tag">Siemens NX API</span>
            <span class="case-study-tag">Python</span>
            <span class="case-study-tag">React</span>
            <span class="case-study-tag">ASTM D6251</span>
            <span class="case-study-tag">Parametric CAD</span>
            <span class="case-study-tag">MBD</span>
        </div>
        "##,

        "surgical" => r##"
        <button id="case-study-close">&times;</button>
        <div class="case-study-header">
            <h1>Surgical Robot Registration</h1>
            <div class="case-study-meta">ARI (stealth startup → <a href="https://www.zimmerbiomet.com" target="_blank">Zimmer Biomet</a>) · 2020 · <a href="https://www.fda.gov/medical-devices/premarket-submissions-selecting-and-preparing-correct-submission/premarket-notification-510k" target="_blank">FDA 510(k)</a> Compliance</div>
        </div>

        <div class="case-study-section">
            <h2>Problem</h2>
            <p>Robotic knee replacement surgery requires precise alignment between the patient's femur (known from pre-op MRI) and the surgical drill's real-time position (tracked via <a href="https://optitrack.com" target="_blank">OptiTrack</a> motion capture).</p>
            <p>The registration algorithm must compute the 6-DOF transformation matrix in real-time with sub-millimeter accuracy to enable autonomous bone milling.</p>
        </div>

        <div class="case-study-section">
            <h2>Solution</h2>
            <p>Developed a C++ registration algorithm using <a href="https://vtk.org" target="_blank">VTK</a> (Visualization Toolkit) and <a href="https://pointclouds.org" target="_blank">PCL</a> (Point Cloud Library) that implements <a href="https://en.wikipedia.org/wiki/Iterative_closest_point" target="_blank">Iterative Closest Point (ICP)</a> to align two 3D point clouds:</p>
            <ul>
                <li><strong>Source:</strong> Patient's femur from MRI (pre-op 3D model)</li>
                <li><strong>Target:</strong> Intra-operative drill position from <a href="https://optitrack.com" target="_blank">OptiTrack</a> markers</li>
            </ul>
        </div>

        <div class="case-study-highlight">
            <strong>0.1mm translation</strong> and <strong>0.3° rotation</strong> accuracy<br>
            Registration time: <strong>300ms</strong> (20s data collection)
        </div>

        <div class="case-study-section">
            <h2>Technical Implementation</h2>
            <ul>
                <li>Processed 10,000+ point clouds per registration cycle</li>
                <li>Implemented <a href="https://en.wikipedia.org/wiki/Iterative_closest_point" target="_blank">ICP</a> with normal-based correspondence filtering</li>
                <li>Integrated with 6-DOF <a href="https://www.kuka.com" target="_blank">Kuka Robot</a> via <a href="https://www.beckhoff.com" target="_blank">Beckhoff</a> PLC (1ms cycle time)</li>
                <li><a href="https://www.dds-foundation.org" target="_blank">DDS</a> middleware + <a href="https://protobuf.dev" target="_blank">Protobuf</a> serialization for real-time data (5ms latency)</li>
                <li><a href="https://www.fda.gov/medical-devices/premarket-submissions-selecting-and-preparing-correct-submission/premarket-notification-510k" target="_blank">FDA 510(k)</a> compliance under <a href="https://www.iso.org/standard/38421.html" target="_blank">IEC 62304</a> safety standards</li>
            </ul>
        </div>

        <div class="case-study-section">
            <h2>Industry Impact</h2>
            <p>This "Active Milling" approach (autonomous cutting) was a first for ARI, moving beyond standard "jig-holding" robotics. The innovation contributed to ARI's acquisition by Monogram, which was subsequently acquired by <a href="https://www.zimmerbiomet.com" target="_blank">Zimmer Biomet</a> in 2025.</p>
        </div>

        <div class="case-study-tags">
            <span class="case-study-tag">C++</span>
            <span class="case-study-tag">VTK</span>
            <span class="case-study-tag">PCL</span>
            <span class="case-study-tag">ICP Algorithm</span>
            <span class="case-study-tag">FDA 510(k)</span>
            <span class="case-study-tag">IEC 62304</span>
            <span class="case-study-tag">Kuka Robot</span>
            <span class="case-study-tag">Beckhoff PLC</span>
        </div>
        "##,

        "forensics" => r##"
        <button id="case-study-close">&times;</button>
        <div class="case-study-header">
            <h1>Multi-Modal Robotic Forensics</h1>
            <div class="case-study-meta">Advanced Engineering Services · 2022-2023 · <a href="https://about.meta.com/realitylabs/" target="_blank">Meta Reality Labs</a></div>
        </div>

        <div class="case-study-section">
            <h2>Problem</h2>
            <p><a href="https://about.meta.com" target="_blank">Meta</a> needed an automated solution to detect counterfeit components in datacenter server hardware. Manual inspection was slow, inconsistent, and couldn't catch sophisticated counterfeits. A single sensing modality (e.g., visual inspection alone) wasn't sufficient to identify all types of fraud.</p>
        </div>

        <div class="case-study-section">
            <h2>Solution</h2>
            <p>Designed and built a complete robotic workcell integrating four complementary sensing modalities to create unique component "fingerprints":</p>
            <ul>
                <li><strong>RF Spectrum Analysis:</strong> <a href="https://www.tek.com/en/products/spectrum-analyzers" target="_blank">RF Spectrum Analyzer</a> to capture electromagnetic signatures and detect cloned chips</li>
                <li><strong>Thermal Camera:</strong> ±0.5°C precision <a href="https://www.flir.com" target="_blank">thermal imaging</a> to identify anomalous heat dissipation patterns</li>
                <li><strong>Capacitance Probing:</strong> Measures electrical properties of PCB traces and component pins</li>
                <li><strong>Optical Inspection:</strong> 85mm industrial lens with <a href="https://opencv.org" target="_blank">machine vision</a> for visual defect detection</li>
            </ul>
        </div>

        <div class="case-study-highlight">
            <strong>0.5% repeatability</strong> across test cycles<br>
            Multi-modal fusion reduced false positives by <strong>40%</strong> vs single-sensor
        </div>

        <div class="case-study-section">
            <h2>Hardware Development</h2>
            <p>Developed a custom end effector that integrated all four sensors into a single robotic tool:</p>
            <ul>
                <li>Mechanical design for multi-sensor mounting with micron-level alignment tolerances</li>
                <li>Integrated RF Spectrum Analyzer with shielded cabling (critical for signal integrity)</li>
                <li>Mounted thermal camera with isolation to prevent cross-contamination from other sensors</li>
                <li>Capacitance probe array with spring-loaded pins for consistent contact</li>
                <li>Industrial camera mount with adjustable focus for varying component heights</li>
                <li>Quick-change interface for different component form factors (CPUs, GPUs, memory modules)</li>
            </ul>
        </div>

        <div class="case-study-section">
            <h2>Software & Control</h2>
            <ul>
                <li><strong><a href="https://www.ni.com/en/shop/labview.html" target="_blank">LabVIEW</a> UI:</strong> Developed operator interface for test sequencing, real-time sensor visualization, and data analytics</li>
                <li>6-axis robotic arm trajectory planning for automated multi-point scanning</li>
                <li>Sensor fusion algorithm combining RF, thermal, capacitance, and optical data streams</li>
                <li><a href="https://opencv.org" target="_blank">OpenCV</a>-based Computer Vision pipeline for optical defect classification</li>
                <li>Automated data logging and statistical analysis to establish component "ground truth" profiles</li>
                <li>Test sequencing automation with pass/fail criteria</li>
            </ul>
        </div>

        <div class="case-study-section">
            <h2>Impact</h2>
            <p>Enabled <a href="https://about.meta.com" target="_blank">Meta</a> to verify component authenticity at production scale. The multi-modal approach successfully identified counterfeits that single-sensor methods missed, protecting datacenter hardware supply chain integrity. The workcell achieved 0.5% measurement repeatability across thousands of test cycles.</p>
        </div>

        <div class="case-study-tags">
            <span class="case-study-tag">Robotics</span>
            <span class="case-study-tag">RF Spectrum Analysis</span>
            <span class="case-study-tag">Thermal Imaging</span>
            <span class="case-study-tag">Sensor Fusion</span>
            <span class="case-study-tag">OpenCV</span>
            <span class="case-study-tag">LabVIEW</span>
            <span class="case-study-tag">End Effector Design</span>
            <span class="case-study-tag">Meta</span>
        </div>
        "##,

        _ => return, // Unknown project
    };

    panel.set_inner_html(content);
    container.append_child(&panel).ok();

    // Close button handler
    if let Some(close_btn) = document.get_element_by_id("case-study-close") {
        let container_clone = container.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            container_clone.set_attribute("style", "display: none;").ok();
        }) as Box<dyn FnMut(_)>);
        close_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // Click outside to close
    let container_clone = container.clone();
    let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
        if let Some(target) = e.target() {
            if target == container_clone.clone().into() {
                container_clone.set_attribute("style", "display: none;").ok();
            }
        }
    }) as Box<dyn FnMut(_)>);
    container
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .ok();
    closure.forget();
}

/// Set up hashchange event listener
fn setup_routing(document: &Document) {
    let window = window().unwrap();
    let document_clone = document.clone();

    let closure = Closure::wrap(Box::new(move || {
        handle_route_change(&document_clone);
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    // Set up back button click handler
    if let Some(back_btn) = document.get_element_by_id("back-button") {
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            e.prevent_default();
            navigate_home();
        }) as Box<dyn FnMut(_)>);

        back_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // Render initial route
    handle_route_change(document);
}

#[allow(clippy::too_many_arguments)]
fn draw_organism(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    angle: f64,
    color: &str,
    base_size: f64,
    role: BoidRole,
    state: BoidState,
    size_mult: f32,
) {
    ctx.save();
    ctx.translate(x, y).unwrap();
    ctx.rotate(angle).unwrap();

    let size = base_size * size_mult as f64;

    // Thruster Glow (Engine)
    let glow_color = match role {
        BoidRole::Herbivore => "rgba(0, 255, 255, 0.4)", // Cyan
        BoidRole::Carnivore => "rgba(255, 50, 50, 0.6)", // Red
        BoidRole::Scavenger => "rgba(255, 200, 0, 0.4)", // Orange
    };

    // Engine trail/glow behind
    ctx.begin_path();
    ctx.move_to(-size * 1.5, 0.0);
    ctx.arc(-size * 1.2, 0.0, size * 0.5, 0.0, std::f64::consts::TAU)
        .unwrap();
    ctx.set_fill_style(&JsValue::from_str(glow_color));
    ctx.fill();

    // Main Body Styling
    ctx.set_line_width(1.5);
    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_fill_style(&JsValue::from_str("#0a0a12")); // Dark metallic core

    match role {
        BoidRole::Herbivore => {
            // Scout Drone (Arrowhead)
            ctx.begin_path();
            ctx.move_to(size, 0.0); // Nose
            ctx.line_to(-size, -size * 0.6); // Left Wing
            ctx.line_to(-size * 0.5, 0.0); // Engine recess
            ctx.line_to(-size, size * 0.6); // Right Wing
            ctx.close_path();

            ctx.fill();
            ctx.stroke();

            // Detail: Cockpit/Sensor
            ctx.set_fill_style(&JsValue::from_str(color));
            ctx.begin_path();
            ctx.arc(0.0, 0.0, size * 0.2, 0.0, std::f64::consts::TAU)
                .unwrap();
            ctx.fill();
        }
        BoidRole::Carnivore => {
            // Interceptor (Sharp, Aggressive)
            ctx.begin_path();
            ctx.move_to(size * 1.2, 0.0); // Long Nose
            ctx.line_to(-size, -size); // Wide Wing L
            ctx.line_to(-size * 0.2, 0.0); // Body
            ctx.line_to(-size, size); // Wide Wing R
            ctx.close_path();

            ctx.fill();
            ctx.stroke();

            if state == BoidState::Hunt {
                // Weapon bays open / Spikes
                ctx.begin_path();
                ctx.move_to(0.0, -size);
                ctx.line_to(size * 0.5, -size * 1.2);
                ctx.stroke();
                ctx.begin_path();
                ctx.move_to(0.0, size);
                ctx.line_to(size * 0.5, size * 1.2);
                ctx.stroke();
            }
        }
        BoidRole::Scavenger => {
            // Harvester (Boxy, Functional)
            ctx.begin_path();
            ctx.move_to(size * 0.8, -size * 0.5);
            ctx.line_to(size * 0.8, size * 0.5);
            ctx.line_to(-size * 0.8, size * 0.5);
            ctx.line_to(-size * 0.8, -size * 0.5);
            ctx.close_path();

            ctx.fill();
            ctx.stroke();

            // Collector Arms
            ctx.begin_path();
            ctx.move_to(size * 0.8, -size * 0.3);
            ctx.line_to(size * 1.2, -size * 0.5);
            ctx.move_to(size * 0.8, size * 0.3);
            ctx.line_to(size * 1.2, size * 0.5);
            ctx.stroke();
        }
    }

    // Shield/Field effect if fleeing
    if state == BoidState::Flee {
        ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 255, 0.5)"));
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.arc(0.0, 0.0, size * 1.8, 0.0, std::f64::consts::TAU)
            .unwrap(); // Energy Shield
        ctx.stroke();

        // Dash lines
        ctx.set_line_dash(&js_sys::Array::of2(
            &JsValue::from_f64(2.0),
            &JsValue::from_f64(4.0),
        ))
        .unwrap();
        ctx.stroke();
        ctx.set_line_dash(&js_sys::Array::new()).unwrap(); // Reset
    }

    ctx.restore();
}

fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();

    // Establish viewport mode early (drives CSS + tuning)
    let initial_mode = update_viewport_mode(&document);
    let initial_tuning = initial_mode.tuning();
    let viewport_tuning: Rc<RefCell<ViewportTuning>> = Rc::new(RefCell::new(initial_tuning));

    // Set up routing and render initial bubbles
    setup_routing(&document);

    // Update commit info display
    update_commit_info(&document);

    let canvas = document
        .get_element_by_id("simulation")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let paused = is_paused();

    // Get simulation area dimensions (canvas parent, excludes telemetry bar)
    let sim_area = document.get_element_by_id("simulation-area");
    let (w, h) = if let Some(area) = &sim_area {
        let rect = area.get_bounding_client_rect();
        (rect.width(), rect.height())
    } else {
        (
            window.inner_width().unwrap().as_f64().unwrap(),
            window.inner_height().unwrap().as_f64().unwrap(),
        )
    };
    canvas.set_width(w as u32);
    canvas.set_height(h as u32);

    // Resize/orientation handler (canvas + viewport mode + constellation rerender)
    {
        let canvas = canvas.clone();
        let document_for_closure = document.clone();
        let window_for_closure = window.clone();
        let tuning_for_resize = Rc::clone(&viewport_tuning);
        let closure = Closure::wrap(Box::new(move || {
            let sim_area = document_for_closure.get_element_by_id("simulation-area");
            let (w, h) = if let Some(area) = &sim_area {
                let rect = area.get_bounding_client_rect();
                (rect.width(), rect.height())
            } else {
                (
                    window_for_closure.inner_width().unwrap().as_f64().unwrap(),
                    window_for_closure.inner_height().unwrap().as_f64().unwrap(),
                )
            };
            canvas.set_width(w as u32);
            canvas.set_height(h as u32);

            // Update viewport mode + tuning, then rerender constellation for new layout
            let mode = update_viewport_mode(&document_for_closure);
            *tuning_for_resize.borrow_mut() = mode.tuning();
            handle_route_change(&document_for_closure);
        }) as Box<dyn FnMut()>);
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let width = w as f32;
    let height = h as f32;

    // Get initial exclusion zones and chakravyu around monoliths
    let (exclusion_zones, chakravyu) = scan_exclusion_zones(&document);

    // Initialize arena with starting population (avoid exclusion zones)
    let arena: BoidArena<ARENA_CAPACITY> = BoidArena::new();

    // Removed initial population loop - The Circle is the Source
    // Start empty and let the fountain fill the world.

    let grid = SpatialGrid::new(width, height, VISION_RADIUS);
    let obstacles = scan_dom_obstacles(&document);

    // No fixed food sources anymore, boids forage the network
    let food_sources = vec![];

    // Initialize Fungal Network
    let mut fungal_network = FungalNetwork::new(width, height);

    // Initial seeding (avoid exclusion zones)
    for _ in 0..10 {
        fungal_network.spawn_root();
    }

    // Initialize Background Effect
    let background = BackgroundEffect::new(width as f64, height as f64);

    let config = SimConfig {
        carrying_capacity: initial_tuning.max_boids.max(1),
        reproduction_threshold: 140.0,
        base_mortality: 0.00001, // Reduced mortality to allow population growth
        ..SimConfig::default()
    };

    let state = Rc::new(RefCell::new(World {
        arena,
        grid,
        obstacles,
        exclusion_zones,
        chakravyu,
        food_sources,
        fungal_network,
        background,
        predators: Vec::new(),
        season: SeasonCycle::new(),
        config,
        width,
        height,
        popups: Vec::new(),
        miasma: Vec::new(),
    }));

    // Cache DOM element references
    let stat_pop = document.get_element_by_id("stat-pop");
    let stat_gen = document.get_element_by_id("stat-gen");
    let stat_fps = document.get_element_by_id("stat-fps");

    let performance: Performance = window.performance().unwrap();

    let f: AnimationCallback = Rc::new(RefCell::new(None));
    let g = f.clone();

    let state_clone = state.clone();
    let document_clone = document.clone();
    let tuning_for_loop = Rc::clone(&viewport_tuning);
    let mut frame_count: u32 = 0;
    let mut last_time = performance.now();
    let mut fps_accumulator = 0.0;
    let mut fps_frame_count = 0;
    let mut center_anim_timer = 0.0;
    let mut center_state = 0; // 0: Logo, 1: Antimony, 2: Sb, 3: Hindi

    let mut stats = SimulationStats {
        max_speed_record: 0.0,
        max_generation: 0,
        low_diversity_frames: 0,
    };
    let mut last_tuning = initial_tuning;
    let mut spawn_interval_frames = compute_spawn_interval_frames(6, last_tuning.spawn_rate_mult);
    let mut telemetry = TelemetryState::new(performance.now());

    *g.borrow_mut() = Some(Closure::new(move || {
        let mut s = state_clone.borrow_mut();
        frame_count += 1;

        // Update sim tuning if viewport mode changed (set by resize handler)
        let tuning_now = { *tuning_for_loop.borrow() };
        if tuning_now.max_boids != last_tuning.max_boids
            || (tuning_now.spawn_rate_mult - last_tuning.spawn_rate_mult).abs() > f32::EPSILON
        {
            spawn_interval_frames = compute_spawn_interval_frames(6, tuning_now.spawn_rate_mult);
            // Apply to simulation config (used for reproduction + population pressure)
            s.config.carrying_capacity = tuning_now.max_boids.max(1);
            last_tuning = tuning_now;
        }

        // FPS calculation
        let current_time = performance.now();
        let delta = current_time - last_time;
        last_time = current_time;
        fps_accumulator += delta;
        fps_frame_count += 1;

        // Update Center Animation (Rust Controlled)
        center_anim_timer += 0.016; // Approx dt

        let (text, text_op, logo_op, glitch) = match center_state {
            0 => {
                // Sb (3s)
                if center_anim_timer > 3.0 {
                    center_state = 1;
                    center_anim_timer = 0.0;
                }
                ("Sb", 1.0, 0.0, false)
            }
            1 => {
                // ANTIMONY (2s)
                if center_anim_timer > 2.0 {
                    center_state = 2;
                    center_anim_timer = 0.0;
                }
                // Basic corruption simulation
                (
                    "ANTIMONY",
                    0.8 + (center_anim_timer as f32 * 10.0).sin() * 0.2,
                    0.0,
                    true,
                )
            }
            2 => {
                // Hindi (3s)
                if center_anim_timer > 3.0 {
                    center_state = 0;
                    center_anim_timer = 0.0;
                }
                ("शिवम् भारद्वाज", 1.0, 0.0, false)
            }
            _ => ("Sb", 1.0, 0.0, false),
        };

        // Call JS updater
        // We need to define `update_center_text` in extern block.
        // And we need to handle the `rng` for corruption if we want it in Rust.
        // For now, let's stick to the simple state machine above.
        // But wait, I can't use `rng` easily here without initializing one.
        // Let's just pass the base text and let the JS helper do the jitter/blur.
        // I added `useGlitch` param.

        // Call JS updater to update center text animation
        update_center_text(text, text_op, logo_op, glitch);

        // Rescan DOM obstacles and exclusion zones occasionally
        if frame_count.is_multiple_of(60) {
            s.obstacles = scan_dom_obstacles(&document_clone);
            let (zones, chakravyu) = scan_exclusion_zones(&document_clone);
            s.exclusion_zones = zones;
            s.chakravyu = chakravyu;
        }

        // === FOUNTAIN OF LIFE ===
        // Spawn new boids from the circle edge periodically (mode-tuned)
        if s.arena.alive_count < last_tuning.max_boids && frame_count % spawn_interval_frames == 0 {
            if let Some(chakravyu) = s.chakravyu {
                use rand::Rng;
                let mut rng = rand::thread_rng();

                // Spawn just outside the kill zone
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let spawn_radius = chakravyu.radius * 1.1;
                let spawn_pos =
                    chakravyu.center + Vec2::new(angle.cos(), angle.sin()) * spawn_radius;

                // Outward velocity
                let spawn_vel = (spawn_pos - chakravyu.center).normalize() * 2.0;

                // Role is handled by the homogenization logic below
                s.arena.spawn(spawn_pos, spawn_vel, Genome::random());
            }
        }

        // Update dashboard every 30 frames
        if frame_count.is_multiple_of(30) {
            let alive_count = s.arena.alive_count;

            if let Some(ref el) = stat_pop {
                el.set_text_content(Some(&format!("POP: {}", alive_count)));
            }

            // Find max generation
            let mut max_gen: u16 = 0;
            let mut max_speed: f32 = 0.0;
            for idx in s.arena.iter_alive() {
                max_gen = max_gen.max(s.arena.generation[idx]);
                max_speed = max_speed.max(s.arena.genes[idx].max_speed);
            }

            if let Some(ref el) = stat_gen {
                el.set_text_content(Some(&format!("GEN: {}", max_gen)));
            }

            if fps_frame_count > 0 && fps_accumulator > 0.0 {
                let avg_fps = (fps_frame_count as f64 / fps_accumulator) * 1000.0;
                if let Some(ref el) = stat_fps {
                    el.set_text_content(Some(&format!("FPS: {:.0}", avg_fps)));
                }
                fps_accumulator = 0.0;
                fps_frame_count = 0;
            }

            // Update speed record internally (for stats tracking, but don't log to console)
            if max_speed > stats.max_speed_record + 0.1 {
                stats.max_speed_record = max_speed;
                // Speed record tracking kept for internal stats, but not displayed to avoid clutter
            }

            // Log events
            if max_gen > stats.max_generation {
                stats.max_generation = max_gen;
                if max_gen.is_multiple_of(5) {
                    log_event(
                        &document_clone,
                        &format!("🧬 GEN {} reached", max_gen),
                        "event-birth",
                    );
                }
            }
        }

        // Update canvas dimensions
        let canvas_w = ctx.canvas().unwrap().width() as f32;
        let canvas_h = ctx.canvas().unwrap().height() as f32;

        if s.width != canvas_w || s.height != canvas_h {
            s.width = canvas_w;
            s.height = canvas_h;
            s.grid.resize(canvas_w, canvas_h);
            s.fungal_network.resize(canvas_w, canvas_h);
            s.background.resize(canvas_w as f64, canvas_h as f64);
        }

        // === SIMULATION STEP ===

        // Destructure to get separate borrows
        let World {
            arena,
            grid,
            obstacles,
            exclusion_zones,
            chakravyu,
            food_sources,
            fungal_network,
            background,
            predators,
            season,
            config,
            width: world_w,
            height: world_h,
            popups: _, // Popups managed via s.popups
            miasma,
            ..
        } = &mut *s;

        // Update season
        season.update(1.0);

        // Update Fungal Network with exclusion zones
        fungal_network.update_with_exclusions(exclusion_zones);

        // Boids interactions with network
        // 1. Spore Trail: Chance to seed new root at boid pos
        // 2. Infect / Interact: Boids contacting nodes

        // Collect interaction results and push forces first to avoid borrow conflicts
        let mut interactions = Vec::new();
        let mut push_forces: Vec<(usize, Vec2)> = Vec::new();
        let chakravyu_victims: Vec<usize> = Vec::new();

        // Get chakravyu zone info
        let chakravyu_zone = *chakravyu;

        // Collect side effects to apply later
        let energy_adjustments: Vec<(usize, f32)> = Vec::new();
        let moksh_candidates: Vec<usize> = Vec::new();

        // CHAKRAVYU MECHANICS - Deadly Trap
        // Boids are pulled inward and drained.
        // Main logic handled in the per-boid loop below

        // Collect forces and side effects first
        let mut kill_list: Vec<usize> = Vec::new();
        let mut new_miasma: Vec<Miasma> = Vec::new();
        let mut infertility_list: Vec<usize> = Vec::new();
        let mut life_support: Vec<(usize, f32, f32)> = Vec::new(); // (idx, new_energy, new_age)
        let mut role_enforcement: Vec<(usize, BoidRole)> = Vec::new();

        for idx in arena.iter_alive() {
            let pos = arena.positions[idx];
            let role = arena.roles[idx];

            // Homogenization: Enforce Herbivore dominance on newborns
            if arena.age[idx] < 1.0 {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                // 95% Herbivore, 4% Scavenger, 1% Carnivore
                let roll = rng.gen::<f32>();
                let new_role = if roll < 0.95 {
                    BoidRole::Herbivore
                } else if roll < 0.99 {
                    BoidRole::Scavenger
                } else {
                    BoidRole::Carnivore
                };

                if role != new_role {
                    role_enforcement.push((idx, new_role));
                }
            }

            // Seed (Spore) - only herbivores spread spores, not in exclusion zones
            if role == BoidRole::Herbivore && !is_in_exclusion_zone(pos, exclusion_zones) {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < 0.005 {
                    fungal_network.seed_at_safe(pos, exclusion_zones);
                }
            }

            // CHAKRAVYU LOGIC
            if let Some(chakravyu) = chakravyu_zone {
                let dist_to_center = pos.distance(chakravyu.center);

                // Inside deadly radius? (Touch the line = Death)
                if dist_to_center < chakravyu.radius * 0.98 {
                    // 2% margin inside the line
                    kill_list.push(idx);
                    use rand::Rng;
                    let mut rng = rand::thread_rng();

                    let miasma_color = match role {
                        BoidRole::Herbivore => "rgba(100, 255, 218, {})", // Cyan/Green
                        BoidRole::Carnivore => "rgba(255, 50, 50, {})",   // Red
                        BoidRole::Scavenger => "rgba(255, 200, 0, {})",   // Orange
                    };

                    new_miasma.push(Miasma {
                        pos,
                        vel: Vec2::new(0.0, -0.5 + rng.gen::<f32>()),
                        life: 1.0,
                        radius: 2.0 + rng.gen::<f32>() * 3.0,
                        color: miasma_color.to_string(),
                    });
                    continue;
                }

                // HIJACK DEATH:
                // Only 5% of boids are chosen to rush to the center.
                // Deterministic choice based on ID to avoid flickering decision.
                let is_chosen_one = (idx % 20) == 0;

                let threshold_age = config.max_age * 0.8;
                let mut current_age = arena.age[idx];
                let current_energy = arena.energy[idx];

                // Convert starving to doomed ONLY if they are chosen
                if is_chosen_one && current_energy < 40.0 && current_age < threshold_age {
                    current_age = threshold_age + 1.0; // Make it old
                    life_support.push((idx, 50.0, current_age)); // Boost energy, set age
                }

                let is_dying = current_age > threshold_age;

                if is_dying && is_chosen_one {
                    // RUSH IN
                    let dir_to_center = (chakravyu.center - pos).normalize();
                    let strength = 8.0;
                    push_forces.push((idx, dir_to_center * strength));
                    push_forces.push((idx, -arena.velocities[idx] * 0.1));

                    infertility_list.push(idx);

                    // IMMORTALITY (until trap):
                    let safe_age = config.max_age - 50.0;
                    let safe_energy = 50.0;
                    life_support.push((idx, safe_energy, safe_age));
                } else if dist_to_center < chakravyu.radius * 1.2 {
                    // Healthy or Unchosen boids get pushed away
                    if dist_to_center < chakravyu.radius {
                        let dir_to_center = (chakravyu.center - pos).normalize();
                        let strength = 3.0;
                        push_forces.push((idx, -dir_to_center * strength));
                    }
                }
            }

            // Exclusion zone repulsion (for icons) - kept from original
            for zone in exclusion_zones.iter() {
                let dist = pos.distance(zone.center);
                if dist < zone.radius && dist > zone.radius * 0.8 && dist > 0.001 {
                    let push = (pos - zone.center).normalize() * 1.5;
                    push_forces.push((idx, push));
                }
            }

            // Fungal interaction
            if (role == BoidRole::Herbivore || role == BoidRole::Scavenger)
                && frame_count.is_multiple_of(2)
            {
                let result = fungal_network.interact(pos, BOID_SIZE * 3.0);
                if result != InteractionResult::None {
                    interactions.push((idx, result));
                }
            }
        }

        // Apply Kills
        for idx in kill_list {
            arena.energy[idx] = -100.0; // Kill instantly
        }

        // Apply Life Support (Immortality for rushing)
        for (idx, energy, age) in life_support {
            arena.energy[idx] = energy;
            arena.age[idx] = age;
        }

        // Apply Infertility (Prevent reproduction for rushing boids)
        for idx in infertility_list {
            if arena.energy[idx] > config.reproduction_threshold - 1.0 {
                arena.energy[idx] = config.reproduction_threshold - 1.0;
            }
        }

        // Add Miasma
        miasma.extend(new_miasma);

        // Apply position updates with wrap-around
        let mut wrap_updates: Vec<(usize, Vec2)> = Vec::new();

        for idx in arena.iter_alive() {
            // Wrap around screen edges
            let mut pos = arena.positions[idx];
            let mut changed = false;

            if pos.x < 0.0 {
                pos.x = *world_w;
                changed = true;
            } else if pos.x > *world_w {
                pos.x = 0.0;
                changed = true;
            }

            if pos.y < 0.0 {
                pos.y = *world_h;
                changed = true;
            } else if pos.y > *world_h {
                pos.y = 0.0;
                changed = true;
            }

            if changed {
                wrap_updates.push((idx, pos));
            }
        }

        for (idx, pos) in wrap_updates {
            arena.positions[idx] = pos;
        }

        // Apply push forces
        for (idx, push) in push_forces {
            arena.velocities[idx] += push;
        }

        // Apply energy adjustments from side effects (e.g. Moksh fade)
        for (idx, amount) in energy_adjustments {
            arena.energy[idx] += amount;
        }

        // Apply Moksh deaths
        for idx in moksh_candidates {
            arena.energy[idx] = -100.0;
        }

        // Apply Chakravyu energy drain - rapid death inside the circle
        if let Some(chakravyu) = chakravyu_zone {
            for idx in chakravyu_victims {
                // Force state to Dead if not already to ensure behavior override
                // But simulation_step handles state transitions. We just drain energy.

                // EXTREME DRAIN: Kill in < 1 second.
                // Increased drain to 25.0 per frame to ensure VERY faster death
                arena.energy[idx] -= 25.0;

                // Accelerated death for those deep inside
                let dist = arena.positions[idx].distance(chakravyu.center);
                if dist < chakravyu.radius * 0.8 {
                    // Instant obliteration zone - kill extremely fast
                    arena.energy[idx] -= 50.0;
                }

                // Force kill check immediately for predators if energy drops below zero
                // This prevents them from lingering due to high health/strength genes
                if arena.energy[idx] <= 0.0 {
                    arena.kill(idx);
                }
            }
        }

        // Apply interactions
        for (idx, result) in interactions {
            match result {
                InteractionResult::Nutrient(amt) => {
                    arena.energy[idx] = (arena.energy[idx] + amt).min(200.0);
                }
                InteractionResult::Damage(amt) => {
                    arena.energy[idx] -= amt;
                }
                InteractionResult::Death => {
                    arena.energy[idx] = -100.0; // Ensure death in next sim step
                }
                InteractionResult::None => {}
            }
        }

        // Update predators
        for pred in predators.iter_mut() {
            pred.update(1.0);
        }
        predators.retain(|p| p.active);

        // 1. Build spatial grid
        grid.build(arena);

        // 2. Compute flocking forces (writes to arena.scratch_accel)
        compute_flocking_forces(arena, grid, VISION_RADIUS, obstacles);

        // 3. Feed from food sources (season-affected)
        // Replaced by feeding from fungal network?
        // For now, let's keep food_sources empty and maybe add logic later to feed from network nodes.
        feed_from_sources(arena, food_sources, season);

        // Obstacle feeding - still works near monoliths
        let obstacle_feeders: Vec<usize> = (0..ARENA_CAPACITY)
            .filter(|&idx| arena.alive[idx])
            .filter(|&idx| {
                obstacles
                    .iter()
                    .any(|obs| arena.positions[idx].distance(obs.center) < 150.0)
            })
            // FIX: Prevent feeding if near the deadly center!
            // This stops rushing boids from healing and cancelling their 'dying' status.
            .filter(|&idx| {
                if let Some(chakravyu) = chakravyu_zone {
                    arena.positions[idx].distance(chakravyu.center) > chakravyu.radius * 1.5
                } else {
                    true
                }
            })
            .collect();

        for idx in obstacle_feeders {
            arena.energy[idx] = (arena.energy[idx] + 0.8 * season.food_multiplier()).min(200.0);
        }

        // Apply predator damage
        let predator_kills = apply_predator_zones(arena, predators);
        if predator_kills > 0 {
            log_event(
                &document_clone,
                &format!("🩸 Predator claimed {} victims!", predator_kills),
                "event-death",
            );
        }

        // 4. Run simulation step (movement, reproduction, death)
        let (births, deaths) = simulation_step(arena, grid, config, *world_w, *world_h, 1.0);

        if deaths > 15 {
            log_event(
                &document_clone,
                &format!("☠ {} died", deaths),
                "event-death",
            );
        }

        // Accumulate births/deaths continuously (sampled at 1Hz)
        telemetry.birth_acc = telemetry.birth_acc.saturating_add(births as u32);
        telemetry.death_acc = telemetry.death_acc.saturating_add(deaths as u32);

        // 1Hz telemetry sampling + DOM updates (sparklines + peek attributes)
        if current_time - telemetry.last_sample_ms >= 1000.0 {
            telemetry.last_sample_ms = current_time;

            // Push births/deaths window sample (clamp to prevent u16 overflow)
            let b = telemetry.birth_acc.min(u16::MAX as u32) as u16;
            let d = telemetry.death_acc.min(u16::MAX as u32) as u16;
            telemetry.births_buf.push(b);
            telemetry.deaths_buf.push(d);
            telemetry.latest_births = b;
            telemetry.latest_deaths = d;
            telemetry.birth_acc = 0;
            telemetry.death_acc = 0;

            // Role counts
            let (h, c, s_count) = count_roles(arena);
            telemetry.herbivore_buf.push(h);
            telemetry.carnivore_buf.push(c);
            telemetry.scavenger_buf.push(s_count);
            telemetry.latest_h = h;
            telemetry.latest_c = c;
            telemetry.latest_s = s_count;

            // Diversity (0–1)
            if arena.alive_count > 10 {
                let div = compute_diversity(arena).clamp(0.0, 1.0);
                telemetry.diversity_buf.push(div);
                telemetry.latest_div = div;
            }

            update_sparklines(&document_clone, &mut telemetry);
            update_peek_attributes(&document_clone, &telemetry);
        }

        // === MASS EXTINCTION CHECK ===
        // When diversity collapses, trigger a reset event
        if frame_count.is_multiple_of(60) && arena.alive_count > 50 {
            let diversity = compute_diversity(arena);

            if diversity < 0.25 {
                stats.low_diversity_frames += 1;

                // Sustained low diversity triggers extinction
                if stats.low_diversity_frames > 10 {
                    log_event(
                        &document_clone,
                        "☄ MASS EXTINCTION - Ecosystem collapsing!",
                        "event-death",
                    );
                    trigger_mass_extinction(arena, 0.8, *world_w, *world_h); // Kill 80%

                    // Also trim the fungal network
                    for node in fungal_network.nodes.iter_mut() {
                        if node.active {
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            if rng.gen::<f32>() < 0.5 {
                                node.active = false;
                            }
                        }
                    }

                    stats.low_diversity_frames = 0;
                    log_event(&document_clone, "🌱 New founders seeded...", "event-birth");
                }
            } else {
                // Reset counter if diversity recovers
                stats.low_diversity_frames = 0;
            }
        }

        // === RENDERING ===

        // Update background effect
        background.update(0.016); // Approx 60fps dt
        background.draw(&ctx);

        // Draw Fungal Network
        fungal_network.draw(&ctx);

        // Draw predators
        for pred in &s.predators {
            if !pred.active {
                continue;
            }

            let pulse = 0.5 + 0.5 * (pred.lifetime * 5.0).sin();
            let alpha = 0.3 * pulse;

            // Tech Danger Zone
            ctx.set_stroke_style(&JsValue::from_str(&format!("rgba(255, 0, 50, {})", alpha)));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.arc(
                pred.position.x as f64,
                pred.position.y as f64,
                pred.radius as f64,
                0.0,
                std::f64::consts::TAU,
            )
            .unwrap();
            ctx.stroke();
        }

        // Draw Chakravyu Sanskrit Shield
        if let Some(chakravyu) = s.chakravyu {
            ctx.save();
            ctx.translate(chakravyu.center.x as f64, chakravyu.center.y as f64)
                .unwrap();

            // Slow rotation for the text ring
            let time = performance.now() * 0.0002;
            ctx.rotate(time).unwrap();

            ctx.set_font("14px 'IBM Plex Mono', monospace");
            ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 170, 0.3)"));
            ctx.set_text_align("center");

            // "रागद्वेषवियुक्तैस्तु विषयानिन्द्रियैश्चरन्। आत्मवश्यैर्विधेयात्मा प्रसादमधिगच्छति॥"
            // Split into code-like segments
            let text = "::रागद्वेषवियुक्तैस्तु::void* // <आत्मवश्यैर्विधेयात्मा>; // fn(प्रसादमधिगच्छति) -> Peace";

            // Draw text in a circle
            let radius = (chakravyu.radius as f64 - 10.0).max(0.0);
            if radius > 1.0 {
                let chars: Vec<char> = text.chars().collect();
                let angle_step = std::f64::consts::TAU / (chars.len() as f64);

                for (i, char) in chars.iter().enumerate() {
                    ctx.save();
                    let angle = i as f64 * angle_step;
                    ctx.rotate(angle).unwrap();
                    ctx.translate(0.0, -radius).unwrap();
                    ctx.fill_text(&char.to_string(), 0.0, 0.0).unwrap();
                    ctx.restore();
                }
            }

            // Inner faint shield circle
            let inner_radius = (radius - 15.0).max(0.0);
            if inner_radius > 0.0 {
                ctx.begin_path();
                ctx.arc(0.0, 0.0, inner_radius, 0.0, std::f64::consts::TAU)
                    .unwrap();

                ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 170, 0.1)"));
                ctx.set_line_width(1.0);
                ctx.stroke();
            }

            ctx.restore();
        }

        // Update and draw popups
        s.popups.retain_mut(|p| {
            p.life -= 0.02;
            p.pos.y -= 0.5; // Float up
            p.life > 0.0
        });

        ctx.set_font("bold 12px 'IBM Plex Mono', monospace");
        ctx.set_text_align("center");
        for p in &s.popups {
            let alpha = p.life;
            // Replace the placeholder {} with alpha
            let color = p.color.replace("{}", &alpha.to_string());
            ctx.set_fill_style(&JsValue::from_str(&color));
            ctx.fill_text(&p.text, p.pos.x as f64, p.pos.y as f64)
                .unwrap();
        }

        // Update and draw Miasma (Smoke/Soul)
        s.miasma.retain_mut(|m| {
            m.life -= 0.015;
            m.pos += m.vel;
            m.radius += 0.2; // Expand
            m.life > 0.0
        });

        for m in &s.miasma {
            ctx.begin_path();
            ctx.arc(
                m.pos.x as f64,
                m.pos.y as f64,
                m.radius as f64,
                0.0,
                std::f64::consts::TAU,
            )
            .unwrap();
            let alpha = m.life * 0.4;
            let color = m.color.replace("{}", &alpha.to_string());
            ctx.set_fill_style(&JsValue::from_str(&color));
            ctx.fill();
        }

        // Draw Organisms (Boids)
        for idx in s.arena.iter_alive() {
            let pos = s.arena.positions[idx];
            let vel = s.arena.velocities[idx];
            let angle = vel.y.atan2(vel.x);
            let (_hue, sat, light) = get_boid_color(&s.arena, idx);
            let role = s.arena.roles[idx];
            let state = s.arena.states[idx];
            let size_mult = s.arena.genes[idx].size;

            // Individual variation - subtle wobble in size and color
            let time = performance.now() * 0.001;
            let wobble = (idx as f64 * 0.1 + time * 2.0).sin() * 0.1;
            let individual_size = size_mult as f64 * (1.0 + wobble);

            // Individual color variation
            let (hue, _sat, _light) = get_boid_color(&s.arena, idx);
            let hue_var = (idx % 20) as i16 - 10;
            let final_hue = (hue as i16 + hue_var).rem_euclid(360);

            let color = format!("hsl({}, {}%, {}%)", final_hue, sat, light);
            draw_organism(
                &ctx,
                pos.x as f64,
                pos.y as f64,
                angle as f64,
                &color,
                BOID_SIZE as f64,
                role,
                state,
                individual_size as f32,
            );
        }

        // Trails
        ctx.set_global_alpha(0.2);
        for idx in s.arena.iter_alive() {
            if s.arena.energy[idx] > 100.0 {
                let pos = s.arena.positions[idx];
                let vel = s.arena.velocities[idx];
                let speed = vel.length();
                if speed > 2.0 {
                    let trail_end = pos - vel.normalize() * speed * 8.0; // Longer trails

                    ctx.begin_path();
                    ctx.move_to(pos.x as f64, pos.y as f64);
                    ctx.line_to(trail_end.x as f64, trail_end.y as f64);

                    let (h, s_val, l) = get_boid_color(&s.arena, idx);
                    ctx.set_stroke_style(&JsValue::from_str(&format!(
                        "hsl({}, {}%, {}%)",
                        h, s_val, l
                    )));
                    ctx.set_line_width(1.0);
                    ctx.stroke();
                }
            }
        }
        ctx.set_global_alpha(1.0);

        if !paused {
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }
    }));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
