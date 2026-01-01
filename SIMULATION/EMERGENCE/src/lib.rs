//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | SIMULATION/EMERGENCE/src/lib.rs
//! PURPOSE: Boids ecosystem simulation WASM entry point
//! MODIFIED: 2025-12-14
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]

mod telemetry;
use telemetry::{TelemetryState, update_sparklines, update_telemetry_text};

use simulation_engine::{
    compute_diversity, compute_flocking_forces, feed_from_sources, get_boid_color,
    seed_population, simulation_step, BoidArena, FoodSource, Obstacle, SeasonCycle, SimConfig,
    SpatialGrid,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement};

// Fixed capacity
const ARENA_CAPACITY: usize = 4096;
const CELL_CAPACITY: usize = 64;
const BOID_SIZE: f32 = 6.0;
const VISION_RADIUS: f32 = 60.0;

/// Type alias for animation frame closure
/// App state shared across RAF + UI event handlers
struct App {
    arena: BoidArena<ARENA_CAPACITY>,
    grid: SpatialGrid<CELL_CAPACITY>,
    obstacles: Vec<Obstacle>,
    food_sources: Vec<FoodSource>,
    season: SeasonCycle,
    config: SimConfig,

    // View state / controls
    paused: bool,
    speed_mult: f32,

    // Display + telemetry
    max_generation: u16,
    telemetry: TelemetryState,
    fps_counter: u32,
    last_fps_update_ms: f64,
    last_frame_ms: f64,

    // World size
    world_w: f32,
    world_h: f32,
}

impl App {
    fn new(now_ms: f64, world_w: f32, world_h: f32) -> Self {
        let mut arena = BoidArena::<ARENA_CAPACITY>::new();
        seed_population(&mut arena, 220, world_w, world_h);

        let grid = SpatialGrid::<CELL_CAPACITY>::new(world_w, world_h, VISION_RADIUS);
        let season = SeasonCycle::new();

        let config = SimConfig {
            carrying_capacity: 2000,
            ..SimConfig::default()
        };

        Self {
            arena,
            grid,
            obstacles: vec![],
            food_sources: vec![],
            season,
            config,
            paused: false,
            speed_mult: 1.0,
            max_generation: 0,
            telemetry: TelemetryState::new(now_ms),
            fps_counter: 0,
            last_fps_update_ms: now_ms,
            last_frame_ms: now_ms,
            world_w,
            world_h,
        }
    }

    fn reset(&mut self, now_ms: f64) {
        self.arena = BoidArena::<ARENA_CAPACITY>::new();
        seed_population(&mut self.arena, 220, self.world_w, self.world_h);
        self.max_generation = 0;
        self.telemetry = TelemetryState::new(now_ms);
        self.fps_counter = 0;
        self.last_fps_update_ms = now_ms;
        self.last_frame_ms = now_ms;
    }
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let performance = window.performance().expect("performance should be available");

    // Get canvas
    let canvas = document
        .get_element_by_id("simulation")
        .expect("canvas#simulation not found")
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .expect("failed to get 2D context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Initial sizing
    let (world_w, world_h) = resize_canvas(&canvas, &context);

    let now_ms = performance.now();
    let app: Rc<RefCell<App>> = Rc::new(RefCell::new(App::new(now_ms, world_w, world_h)));

    // Setup controls (shared state)
    setup_controls(&document, Rc::clone(&app))?;

    // Animation loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let app_for_loop = Rc::clone(&app);
    let canvas_for_loop = canvas.clone();
    let ctx_for_loop = context.clone();
    let document_for_loop = document.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now_ms = performance.now();

        // Resize if needed
        {
            let (w, h) = resize_canvas_if_needed(&canvas_for_loop, &ctx_for_loop);
            if let (Some(w), Some(h)) = (w, h) {
                let mut app = app_for_loop.borrow_mut();
                app.world_w = w;
                app.world_h = h;
                app.grid.resize(w, h);
            }
        }

        let is_dark = is_dark_theme(&document_for_loop);

        // Update + render
        {
            let mut app = app_for_loop.borrow_mut();

            // FPS (cheap: count frames in last 1s)
            app.fps_counter = app.fps_counter.saturating_add(1);
            if now_ms - app.last_fps_update_ms >= 1000.0 {
                update_telemetry_text(
                    &document_for_loop,
                    app.arena.alive_count,
                    app.max_generation,
                    app.fps_counter,
                    &app.telemetry,
                );
                app.fps_counter = 0;
                app.last_fps_update_ms = now_ms;
            }

            // dt
            let dt = ((now_ms - app.last_frame_ms) / 1000.0).min(0.05) as f32;
            app.last_frame_ms = now_ms;

            // Split borrows (avoid aliasing app fields)
            let App {
                arena,
                grid,
                obstacles,
                food_sources,
                season,
                config,
                paused,
                speed_mult,
                max_generation,
                telemetry,
                world_w,
                world_h,
                ..
            } = &mut *app;

            if !*paused {
                let dt = dt * (*speed_mult);

                grid.build(arena);
                compute_flocking_forces(arena, grid, VISION_RADIUS, obstacles);
                feed_from_sources(arena, food_sources, season);

                let (births, deaths) = simulation_step(arena, grid, config, *world_w, *world_h, dt);

                telemetry.birth_acc = telemetry.birth_acc.saturating_add(births as u32);
                telemetry.death_acc = telemetry.death_acc.saturating_add(deaths as u32);

                // Track max generation (local first to avoid borrow conflicts)
                let mut max_gen = *max_generation;
                for idx in arena.iter_alive() {
                    max_gen = max_gen.max(arena.generation[idx]);
                }
                *max_generation = max_gen;

                // Sample telemetry at 1Hz
                if telemetry.should_sample(now_ms) {
                    let diversity = if arena.alive_count > 10 {
                        compute_diversity(&*arena).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    telemetry.sample(&*arena, diversity, now_ms);
                    update_sparklines(&document_for_loop, telemetry);
                }
            }

            render_frame(&ctx_for_loop, arena, *world_w, *world_h, is_dark);
        }

        // Schedule next frame
        let _ = web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }) as Box<dyn FnMut()>));

    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

/// Setup control button + slider event handlers
fn setup_controls(document: &Document, app: Rc<RefCell<App>>) -> Result<(), JsValue> {
    // Pause
    if let Some(btn) = document.get_element_by_id("pause-btn") {
        let btn = btn.dyn_into::<HtmlButtonElement>()?;
        let app_for_btn = Rc::clone(&app);
        let btn_for_closure = btn.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let mut app = app_for_btn.borrow_mut();
            app.paused = !app.paused;
            btn_for_closure.set_text_content(Some(if app.paused { "▶ Resume" } else { "⏸ Pause" }));
        }) as Box<dyn FnMut(_)>);

        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Reset
    if let Some(btn) = document.get_element_by_id("reset-btn") {
        let btn = btn.dyn_into::<HtmlButtonElement>()?;
        let app_for_btn = Rc::clone(&app);
        let perf = window().unwrap().performance().unwrap();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let now = perf.now();
            let mut app = app_for_btn.borrow_mut();
            app.reset(now);
        }) as Box<dyn FnMut(_)>);

        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Population cap slider -> config.carrying_capacity
    if let (Some(slider), Some(out)) = (
        document.get_element_by_id("population-slider"),
        document.get_element_by_id("population-value"),
    ) {
        let slider = slider.dyn_into::<HtmlInputElement>()?;
        let out = out.dyn_into::<web_sys::HtmlElement>()?;
        let app_for_slider = Rc::clone(&app);

        // Initialize from DOM value
        if let Ok(val) = slider.value().parse::<usize>() {
            out.set_text_content(Some(&val.to_string()));
            app_for_slider.borrow_mut().config.carrying_capacity = val;
        }

        let slider_clone = slider.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let val = slider_clone.value().parse::<usize>().unwrap_or(800);
            if let Some(out) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("population-value"))
            {
                out.set_text_content(Some(&val.to_string()));
            }
            app_for_slider.borrow_mut().config.carrying_capacity = val;
        }) as Box<dyn FnMut(_)>);

        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Speed slider -> dt multiplier
    if let (Some(slider), Some(out)) = (
        document.get_element_by_id("speed-slider"),
        document.get_element_by_id("speed-value"),
    ) {
        let slider = slider.dyn_into::<HtmlInputElement>()?;
        let out = out.dyn_into::<web_sys::HtmlElement>()?;
        let app_for_slider = Rc::clone(&app);

        // Initialize from DOM value
        if let Ok(val) = slider.value().parse::<f32>() {
            out.set_text_content(Some(&format!("{:.1}x", val)));
            app_for_slider.borrow_mut().speed_mult = val;
        }

        let slider_clone = slider.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let val = slider_clone.value().parse::<f32>().unwrap_or(1.0);
            if let Some(out) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("speed-value"))
            {
                out.set_text_content(Some(&format!("{:.1}x", val)));
            }
            app_for_slider.borrow_mut().speed_mult = val;
        }) as Box<dyn FnMut(_)>);

        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

/// Render one frame of the simulation
fn render_frame(
    ctx: &CanvasRenderingContext2d,
    arena: &BoidArena<ARENA_CAPACITY>,
    world_w: f32,
    world_h: f32,
    is_dark: bool,
) {
    // Clear canvas
    // Theme-aware trail fade (keeps it readable in both light and dark)
    let fade = if is_dark {
        "rgba(5, 5, 8, 0.22)"
    } else {
        "rgba(251, 251, 253, 0.32)"
    };
    ctx.set_fill_style(&JsValue::from_str(fade));
    ctx.fill_rect(0.0, 0.0, world_w as f64, world_h as f64);

    // Draw boids
    for idx in arena.iter_alive() {
        let pos = arena.positions[idx];
        let vel = arena.velocities[idx];

        // Get color based on role
        let (hue, sat, light) = get_boid_color(arena, idx);
        let color = format!("hsl({}, {}%, {}%)", hue, sat, light);

        // Draw boid as triangle pointing in velocity direction
        ctx.save();
        ctx.translate(pos.x as f64, pos.y as f64).ok();

        let angle = vel.y.atan2(vel.x);
        ctx.rotate(angle as f64).ok();

        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.begin_path();
        ctx.move_to(BOID_SIZE as f64, 0.0);
        ctx.line_to(-BOID_SIZE as f64, BOID_SIZE as f64 * 0.6);
        ctx.line_to(-BOID_SIZE as f64, -BOID_SIZE as f64 * 0.6);
        ctx.close_path();
        ctx.fill();

        ctx.restore();
    }
}

fn is_dark_theme(document: &Document) -> bool {
    document
        .document_element()
        .and_then(|el| el.get_attribute("data-theme"))
        .map(|t| t == "dark")
        .unwrap_or(false)
}

fn resize_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> (f32, f32) {
    let w = canvas.client_width().max(1) as u32;
    let h = canvas.client_height().max(1) as u32;
    canvas.set_width(w);
    canvas.set_height(h);
    ctx.set_line_width(1.0);
    (w as f32, h as f32)
}

fn resize_canvas_if_needed(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> (Option<f32>, Option<f32>) {
    let w = canvas.client_width().max(1) as u32;
    let h = canvas.client_height().max(1) as u32;
    if canvas.width() == w && canvas.height() == h {
        return (None, None);
    }
    canvas.set_width(w);
    canvas.set_height(h);
    ctx.set_line_width(1.0);
    (Some(w as f32), Some(h as f32))
}
