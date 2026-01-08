//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | TOOLS/POWER_CIRCUITS/src/lib.rs
//! PURPOSE: Power circuit designer WASM application with interactive design and visualization
//! MODIFIED: 2026-01-07
//! LAYER: TOOLS → POWER_CIRCUITS
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]
use std::cell::RefCell;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement,
    HtmlInputElement, HtmlSelectElement, MouseEvent, WheelEvent,
};
use js_sys;

use power_engine::{
    design_boost, design_buck, design_ldo, BoostDesign, BoostRequirements, BuckDesign, BuckRequirements,
    DesignPriority, DesignReport, LDODesign, LDORequirements, PowerDesignResult, RippleSpec,
    TopologyType, VoltageRange,
    // Transient simulation
    buck_duty_for_vout, boost_duty_for_vout, simulate_buck, simulate_boost,
    TransientConfig, TransientResult,
};

// ============================================================================
// STATE MANAGEMENT
// ============================================================================

thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

#[derive(Clone, Debug)]
enum CurrentDesign {
    None,
    Buck(BuckDesign),
    Boost(BoostDesign),
    LDO(LDODesign),
}

/// Waveform view state for zoom and pan
#[derive(Clone, Debug)]
struct WaveformView {
    /// Start time of visible window (seconds)
    t_start: f64,
    /// End time of visible window (seconds)
    t_end: f64,
    /// Full duration of simulation data (seconds)
    t_max: f64,
    /// Whether user is currently dragging to pan
    is_dragging: bool,
    /// X coordinate where drag started (pixels)
    drag_start_x: f64,
    /// View t_start when drag began
    drag_t_start: f64,
}

impl Default for WaveformView {
    fn default() -> Self {
        Self {
            t_start: 0.0,
            t_end: 1.0,
            t_max: 1.0,
            is_dragging: false,
            drag_start_x: 0.0,
            drag_t_start: 0.0,
        }
    }
}

impl WaveformView {
    /// Reset view to show full data range
    fn reset(&mut self) {
        self.t_start = 0.0;
        self.t_end = self.t_max;
    }

    /// Get view duration in seconds
    fn duration(&self) -> f64 {
        (self.t_end - self.t_start).max(1e-9)
    }

    /// Zoom by factor centered on a position (0.0 = left, 1.0 = right)
    fn zoom(&mut self, factor: f64, center: f64) {
        let current_duration = self.duration();
        let new_duration = (current_duration * factor).clamp(self.t_max * 0.001, self.t_max);

        // Time at center position
        let t_center = self.t_start + center * current_duration;

        // Adjust start/end keeping center fixed
        self.t_start = (t_center - center * new_duration).max(0.0);
        self.t_end = (self.t_start + new_duration).min(self.t_max);

        // Clamp to valid range
        if self.t_end > self.t_max {
            self.t_end = self.t_max;
            self.t_start = (self.t_end - new_duration).max(0.0);
        }
    }

}

struct AppState {
    topology: TopologyType,
    design: CurrentDesign,
    // Input parameters
    vin_nom: f64,
    vout: f64,
    iout: f64,
    fsw_khz: f64,
    priority: DesignPriority,
    // Simulation results
    sim_result: Option<TransientResult>,
    // Waveform view state
    waveform_view: WaveformView,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            topology: TopologyType::Buck,
            design: CurrentDesign::None,
            vin_nom: 12.0,
            vout: 5.0,
            iout: 2.0,
            fsw_khz: 500.0,
            priority: DesignPriority::Efficiency,
            sim_result: None,
            waveform_view: WaveformView::default(),
        }
    }
}

// ============================================================================
// WASM ENTRY POINT
// ============================================================================

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Power Circuit Designer initialized".into());

    if let Err(e) = init_ui() {
        web_sys::console::error_1(&format!("Failed to initialize UI: {:?}", e).into());
    }

    Ok(())
}

// ============================================================================
// UI INITIALIZATION
// ============================================================================

fn init_ui() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    // Set up event listeners for inputs
    setup_input_listeners(&document)?;

    // Set up design button
    if let Some(btn) = document.get_element_by_id("design-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = run_design() {
                web_sys::console::error_1(&format!("Design failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up auto-detect button
    if let Some(btn) = document.get_element_by_id("auto-detect-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = auto_detect_topology() {
                web_sys::console::error_1(&format!("Auto-detect failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up simulate button
    if let Some(btn) = document.get_element_by_id("simulate-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = run_simulation() {
                web_sys::console::error_1(&format!("Simulation failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up reset view button
    if let Some(btn) = document.get_element_by_id("reset-view-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            reset_waveform_view();
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up waveform canvas zoom/pan event handlers
    setup_waveform_events(&document)?;

    // Initialize waveform canvas with placeholder
    clear_waveform_canvas(&document).ok();

    // Run initial design with default values
    run_design()?;

    Ok(())
}

fn setup_input_listeners(document: &Document) -> Result<(), JsValue> {
    // Set up topology selector listener
    if let Some(topo_select) = document.get_element_by_id("topology") {
        let topo_select: HtmlSelectElement = topo_select.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            update_topology_from_ui();
            if let Err(e) = run_design() {
                web_sys::console::error_1(&format!("Design update failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        topo_select.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up priority selector listener
    if let Some(priority_select) = document.get_element_by_id("priority") {
        let priority_select: HtmlSelectElement = priority_select.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            update_priority_from_ui();
            if let Err(e) = run_design() {
                web_sys::console::error_1(&format!("Design update failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        priority_select.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up number input listeners
    const INPUT_IDS: &[&str] = &["vin", "vout", "iout", "fsw"];

    for &id in INPUT_IDS {
        if let Some(input) = document.get_element_by_id(id) {
            let input: HtmlInputElement = input.dyn_into()?;
            let closure = Closure::wrap(Box::new(move || {
                if let Err(e) = run_design() {
                    web_sys::console::error_1(&format!("Design update failed: {:?}", e).into());
                }
            }) as Box<dyn FnMut()>);
            input.set_oninput(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
    }

    Ok(())
}

fn setup_waveform_events(document: &Document) -> Result<(), JsValue> {
    let canvas = match document.get_element_by_id("waveform-canvas") {
        Some(c) => c,
        None => return Ok(()), // Canvas not found, skip
    };
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    // Constants for canvas layout (must match draw_waveforms)
    const MARGIN_LEFT: f64 = 60.0;
    const MARGIN_RIGHT: f64 = 20.0;

    // Mouse wheel zoom
    let wheel_closure = Closure::wrap(Box::new(move |event: WheelEvent| {
        event.prevent_default();

        let canvas_width = event.target()
            .and_then(|t| t.dyn_into::<HtmlCanvasElement>().ok())
            .map(|c| c.width() as f64)
            .unwrap_or(600.0);
        let plot_width = canvas_width - MARGIN_LEFT - MARGIN_RIGHT;

        // Get mouse position relative to plot area
        let x = event.offset_x() as f64;
        let center = ((x - MARGIN_LEFT) / plot_width).clamp(0.0, 1.0);

        // Zoom factor: scroll up = zoom in, scroll down = zoom out
        let delta = event.delta_y();
        let zoom_factor = if delta > 0.0 { 1.2 } else { 1.0 / 1.2 };

        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.waveform_view.zoom(zoom_factor, center);
        });

        redraw_waveform();
    }) as Box<dyn FnMut(WheelEvent)>);
    canvas.add_event_listener_with_callback("wheel", wheel_closure.as_ref().unchecked_ref())?;
    wheel_closure.forget();

    // Mouse down: start drag
    let mousedown_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let x = event.offset_x() as f64;
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.waveform_view.is_dragging = true;
            s.waveform_view.drag_start_x = x;
            s.waveform_view.drag_t_start = s.waveform_view.t_start;
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas.add_event_listener_with_callback("mousedown", mousedown_closure.as_ref().unchecked_ref())?;
    mousedown_closure.forget();

    // Mouse move: pan if dragging
    let mousemove_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let is_dragging = STATE.with(|state| state.borrow().waveform_view.is_dragging);
        if !is_dragging {
            return;
        }

        let canvas_width = event.target()
            .and_then(|t| t.dyn_into::<HtmlCanvasElement>().ok())
            .map(|c| c.width() as f64)
            .unwrap_or(600.0);
        let plot_width = canvas_width - MARGIN_LEFT - MARGIN_RIGHT;

        let x = event.offset_x() as f64;

        STATE.with(|state| {
            let mut s = state.borrow_mut();
            let dx_pixels = x - s.waveform_view.drag_start_x;
            let duration = s.waveform_view.duration();
            let dt = -dx_pixels * duration / plot_width;

            // Calculate new t_start from drag origin
            let new_t_start = (s.waveform_view.drag_t_start + dt)
                .clamp(0.0, s.waveform_view.t_max - duration);
            s.waveform_view.t_start = new_t_start;
            s.waveform_view.t_end = new_t_start + duration;
        });

        redraw_waveform();
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas.add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())?;
    mousemove_closure.forget();

    // Mouse up: end drag
    let mouseup_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        STATE.with(|state| {
            state.borrow_mut().waveform_view.is_dragging = false;
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas.add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())?;
    mouseup_closure.forget();

    // Mouse leave: end drag
    let mouseleave_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        STATE.with(|state| {
            state.borrow_mut().waveform_view.is_dragging = false;
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas.add_event_listener_with_callback("mouseleave", mouseleave_closure.as_ref().unchecked_ref())?;
    mouseleave_closure.forget();

    // Double-click: reset view
    let dblclick_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        reset_waveform_view();
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas.add_event_listener_with_callback("dblclick", dblclick_closure.as_ref().unchecked_ref())?;
    dblclick_closure.forget();

    Ok(())
}

/// Reset waveform view to show full data range
fn reset_waveform_view() {
    STATE.with(|state| {
        state.borrow_mut().waveform_view.reset();
    });
    redraw_waveform();
}

/// Redraw waveform with current view settings
fn redraw_waveform() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let (sim_result, vout) = STATE.with(|state| {
        let s = state.borrow();
        (s.sim_result.clone(), s.vout)
    });

    if let Some(result) = sim_result {
        if let Err(e) = draw_waveforms(&document, &result, vout) {
            web_sys::console::error_1(&format!("Redraw failed: {:?}", e).into());
        }
    }
}

// ============================================================================
// DESIGN FUNCTIONS
// ============================================================================

fn update_topology_from_ui() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    if let Some(elem) = document.get_element_by_id("topology") {
        if let Ok(select) = elem.dyn_into::<HtmlSelectElement>() {
            let value = select.value();
            let topology = match value.as_str() {
                "boost" => TopologyType::Boost,
                "ldo" => TopologyType::LDO,
                _ => TopologyType::Buck,
            };
            STATE.with(|state| {
                state.borrow_mut().topology = topology;
            });

            // Update theory panel visibility
            update_theory_panel(&document, &value);
        }
    }
}

fn update_theory_panel(document: &Document, topology: &str) {
    // Hide all theory texts
    for id in &["theory-buck", "theory-boost", "theory-ldo"] {
        if let Some(elem) = document.get_element_by_id(id) {
            if let Ok(elem) = elem.dyn_into::<HtmlElement>() {
                let _ = elem.style().set_property("display", "none");
            }
        }
    }

    // Show selected theory
    let show_id = match topology {
        "boost" => "theory-boost",
        "ldo" => "theory-ldo",
        _ => "theory-buck",
    };

    if let Some(elem) = document.get_element_by_id(show_id) {
        if let Ok(elem) = elem.dyn_into::<HtmlElement>() {
            let _ = elem.style().set_property("display", "block");
        }
    }
}

fn update_priority_from_ui() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    if let Some(elem) = document.get_element_by_id("priority") {
        if let Ok(select) = elem.dyn_into::<HtmlSelectElement>() {
            let priority = match select.value().as_str() {
                "size" => DesignPriority::Size,
                "cost" => DesignPriority::Cost,
                "noise" => DesignPriority::Noise,
                _ => DesignPriority::Efficiency,
            };
            STATE.with(|state| {
                state.borrow_mut().priority = priority;
            });
        }
    }
}

fn auto_detect_topology() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let vin = get_input_value(&document, "vin")?;
    let vout = get_input_value(&document, "vout")?;
    let iout = get_input_value(&document, "iout")?;

    let priority = STATE.with(|state| state.borrow().priority);
    let recommendation = power_engine::recommend_topology(vin, vout, iout, priority);

    // Update the topology selector
    if let Some(elem) = document.get_element_by_id("topology") {
        let select: HtmlSelectElement = elem.dyn_into()?;
        let value = match recommendation.recommended {
            TopologyType::Buck => "buck",
            TopologyType::Boost => "boost",
            TopologyType::LDO => "ldo",
            _ => "buck",
        };
        select.set_value(value);
    }

    // Update state
    STATE.with(|state| {
        state.borrow_mut().topology = recommendation.recommended;
    });

    // Show recommendation reasoning
    if let Some(elem) = document.get_element_by_id("recommendation-text") {
        let elem: HtmlElement = elem.dyn_into()?;
        elem.set_inner_html(&recommendation.reasoning);
    }

    run_design()
}

fn run_design() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    // Clear any previous error
    if let Some(elem) = document.get_element_by_id("error-msg") {
        let elem: HtmlElement = elem.dyn_into()?;
        elem.style().set_property("display", "none")?;
    }

    // Read input values
    let vin = get_input_value(&document, "vin")?;
    let vout = get_input_value(&document, "vout")?;
    let iout = get_input_value(&document, "iout")?;
    let fsw_khz = get_input_value(&document, "fsw")?;

    // Update state
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.vin_nom = vin;
        s.vout = vout;
        s.iout = iout;
        s.fsw_khz = fsw_khz;
    });

    let topology = STATE.with(|state| state.borrow().topology);

    // Run appropriate design
    let result = match topology {
        TopologyType::Buck => {
            let req = BuckRequirements {
                vin: VoltageRange::range(vin * 0.9, vin * 1.1),
                vout,
                iout_max: iout,
                iout_min: iout * 0.1,
                switching_freq_hz: fsw_khz * 1000.0,
                ripple: RippleSpec::default(),
                ambient_temp_c: 25.0,
            };
            design_buck(&req).map(|d| {
                STATE.with(|state| {
                    state.borrow_mut().design = CurrentDesign::Buck(d.clone());
                });
                PowerDesignResult::Buck(d)
            })
        }
        TopologyType::Boost => {
            let req = BoostRequirements {
                vin: VoltageRange::range(vin * 0.9, vin * 1.1),
                vout,
                iout_max: iout,
                iout_min: iout * 0.1,
                switching_freq_hz: fsw_khz * 1000.0,
                ripple: RippleSpec::default(),
                ambient_temp_c: 25.0,
            };
            design_boost(&req).map(|d| {
                STATE.with(|state| {
                    state.borrow_mut().design = CurrentDesign::Boost(d.clone());
                });
                PowerDesignResult::Boost(d)
            })
        }
        TopologyType::LDO => {
            let req = LDORequirements {
                vin: VoltageRange::range(vin * 0.9, vin * 1.1),
                vout,
                iout_max: iout,
                ..Default::default()
            };
            design_ldo(&req).map(|d| {
                STATE.with(|state| {
                    state.borrow_mut().design = CurrentDesign::LDO(d.clone());
                });
                PowerDesignResult::LDO(d)
            })
        }
        _ => Err("Unsupported topology".to_string()),
    };

    match result {
        Ok(design) => {
            let report = design.to_report();
            display_results(&document, &report)?;
            draw_schematic(&document, topology)?;
            Ok(())
        }
        Err(e) => {
            if let Some(elem) = document.get_element_by_id("error-msg") {
                let elem: HtmlElement = elem.dyn_into()?;
                elem.set_inner_html(&format!("Design Error: {}", e));
                elem.style().set_property("display", "block")?;
            }
            Ok(())
        }
    }
}

fn get_input_value(document: &Document, id: &str) -> Result<f64, JsValue> {
    let input = document
        .get_element_by_id(id)
        .ok_or_else(|| format!("Input '{}' not found", id))?;
    let input: HtmlInputElement = input.dyn_into()?;
    let value: f64 = input
        .value()
        .parse()
        .map_err(|_| format!("Invalid value for {}", id))?;
    Ok(value)
}

// ============================================================================
// TRANSIENT SIMULATION
// ============================================================================

fn run_simulation() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    // Get current design parameters from state
    let (topology, vin, vout, iout, fsw_khz) = STATE.with(|state| {
        let s = state.borrow();
        (s.topology, s.vin_nom, s.vout, s.iout, s.fsw_khz)
    });

    // Get simulation duration from UI
    let duration_us = get_input_value(&document, "sim-duration").unwrap_or(500.0);
    let duration_s = duration_us * 1e-6;

    // Calculate duty cycle based on topology
    let duty_cycle = match topology {
        TopologyType::Buck => buck_duty_for_vout(vin, vout),
        TopologyType::Boost => boost_duty_for_vout(vin, vout),
        TopologyType::LDO => {
            // LDO doesn't have transient simulation (no switching)
            web_sys::console::log_1(&"LDO has no transient simulation (linear regulator)".into());
            clear_waveform_canvas(&document)?;
            return Ok(());
        }
        _ => return Ok(()),
    };

    // Get component values from current design
    let (inductance, capacitance) = STATE.with(|state| {
        let s = state.borrow();
        match &s.design {
            CurrentDesign::Buck(d) => (d.inductor.selected_value, d.output_capacitor.selected_value),
            CurrentDesign::Boost(d) => (d.inductor.selected_value, d.output_capacitor.selected_value),
            _ => (22e-6, 100e-6), // Default values
        }
    });

    // Calculate load resistance from Vout/Iout
    let load_resistance = if iout > 0.0 { vout / iout } else { 100.0 };

    // Create simulation config
    let config = TransientConfig {
        vin,
        duty_cycle,
        fsw: fsw_khz * 1000.0,
        inductance,
        capacitance,
        load_resistance,
        duration: duration_s,
        output_step: duration_s / 500.0, // ~500 points
        steps_per_cycle: 100,
        initial_il: 0.0,
        initial_vc: 0.0,
    };

    // Run simulation
    web_sys::console::log_1(&format!(
        "Running {} simulation: Vin={:.1}V, Vout={:.1}V, D={:.3}, L={:.1}uH, C={:.1}uF, R={:.1}Ohm, dur={:.0}us",
        match topology { TopologyType::Buck => "Buck", TopologyType::Boost => "Boost", _ => "?" },
        vin, vout, duty_cycle, inductance * 1e6, capacitance * 1e6, load_resistance, duration_s * 1e6
    ).into());

    let result = match topology {
        TopologyType::Buck => simulate_buck(&config),
        TopologyType::Boost => simulate_boost(&config),
        _ => return Ok(()),
    };

    // Get actual time range from result
    let t_max = result.time.last().copied().unwrap_or(duration_s);

    // Store result and reset view to full range
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.sim_result = Some(result.clone());
        // Reset view to show full simulation
        s.waveform_view.t_start = 0.0;
        s.waveform_view.t_end = t_max;
        s.waveform_view.t_max = t_max;
    });

    // Draw waveforms
    draw_waveforms(&document, &result, vout)?;

    // Log stats
    web_sys::console::log_1(&format!(
        "Simulation complete: {} cycles, Vout_avg={:.2}V, Efficiency~{:.1}%",
        result.stats.cycles,
        result.stats.avg_vout,
        result.stats.efficiency_estimate * 100.0
    ).into());

    Ok(())
}

fn clear_waveform_canvas(document: &Document) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let canvas = document
        .get_element_by_id("waveform-canvas")
        .ok_or("Waveform canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Could not get 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Handle high-DPI displays
    let dpr = window.device_pixel_ratio();
    let css_width = 600.0;
    let css_height = 180.0;

    canvas.set_width((css_width * dpr) as u32);
    canvas.set_height((css_height * dpr) as u32);
    let _ = canvas.style().set_property("width", &format!("{}px", css_width));
    let _ = canvas.style().set_property("height", &format!("{}px", css_height));

    ctx.scale(dpr, dpr)?;

    let width = css_width;
    let height = css_height;

    // Clear with background
    ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
    ctx.fill_rect(0.0, 0.0, width, height);

    // Draw placeholder text
    ctx.set_fill_style(&JsValue::from_str("#404050"));
    ctx.set_font("14px Monaco, monospace");
    ctx.set_text_align("center");
    let _ = ctx.fill_text("Click 'Run Simulation' to see waveforms", width / 2.0, height / 2.0);
    ctx.set_text_align("left");

    Ok(())
}

fn draw_waveforms(document: &Document, result: &TransientResult, target_vout: f64) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let canvas = document
        .get_element_by_id("waveform-canvas")
        .ok_or("Waveform canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Could not get 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Handle high-DPI displays
    let dpr = window.device_pixel_ratio();
    let css_width = 600.0;
    let css_height = 180.0;

    // Set canvas size for high-DPI
    canvas.set_width((css_width * dpr) as u32);
    canvas.set_height((css_height * dpr) as u32);
    let _ = canvas.style().set_property("width", &format!("{}px", css_width));
    let _ = canvas.style().set_property("height", &format!("{}px", css_height));

    // Scale context for high-DPI
    ctx.scale(dpr, dpr)?;

    let width = css_width;
    let height = css_height;

    // Clear canvas
    ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
    ctx.fill_rect(0.0, 0.0, width, height);

    // Margins
    let margin_left = 60.0;
    let margin_right = 20.0;
    let margin_top = 20.0;
    let margin_bottom = 30.0;

    let plot_width = width - margin_left - margin_right;
    let plot_height = height - margin_top - margin_bottom;

    // Get view window from state
    let (view_t_start, view_t_end) = STATE.with(|state| {
        let s = state.borrow();
        (s.waveform_view.t_start, s.waveform_view.t_end)
    });
    let view_duration = (view_t_end - view_t_start).max(1e-9);

    // Get full data range
    let t_max_data = result.time.last().copied().unwrap_or(1.0).max(1e-9);

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("#1a1a24"));
    ctx.set_line_width(0.5);
    for i in 0..=4 {
        let y = margin_top + (i as f64 / 4.0) * plot_height;
        ctx.begin_path();
        ctx.move_to(margin_left, y);
        ctx.line_to(width - margin_right, y);
        ctx.stroke();
    }
    for i in 0..=5 {
        let x = margin_left + (i as f64 / 5.0) * plot_width;
        ctx.begin_path();
        ctx.move_to(x, margin_top);
        ctx.line_to(x, height - margin_bottom);
        ctx.stroke();
    }

    // Find data indices within view window (with small margin for continuity)
    let margin_t = view_duration * 0.01;
    let start_idx = result.time.iter()
        .position(|&t| t >= view_t_start - margin_t)
        .unwrap_or(0);
    let end_idx = result.time.iter()
        .position(|&t| t > view_t_end + margin_t)
        .unwrap_or(result.time.len());

    // Get voltage range for visible data
    let v_max = result.v_out[start_idx..end_idx].iter()
        .copied()
        .fold(0.0_f64, f64::max)
        .max(target_vout * 1.1);
    let v_min = result.v_out[start_idx..end_idx].iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let v_range = (v_max - v_min).max(0.1);

    // Get current range for visible data
    let i_max = result.i_l[start_idx..end_idx].iter()
        .copied()
        .fold(0.0_f64, f64::max)
        .max(0.001);

    // Helper to map data to canvas coordinates (using view window)
    let map_x = |t: f64| margin_left + ((t - view_t_start) / view_duration) * plot_width;
    let map_v = |v: f64| margin_top + plot_height - ((v - v_min) / v_range) * plot_height;
    let map_i = |i: f64| margin_top + plot_height - (i / i_max) * plot_height * 0.3;

    // Draw output voltage waveform (green)
    if end_idx > start_idx {
        ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(map_x(result.time[start_idx]), map_v(result.v_out[start_idx]));
        for i in (start_idx + 1)..end_idx {
            ctx.line_to(map_x(result.time[i]), map_v(result.v_out[i]));
        }
        ctx.stroke();
    }

    // Draw inductor current waveform (orange)
    if end_idx > start_idx {
        ctx.set_stroke_style(&JsValue::from_str("#ffaa00"));
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(map_x(result.time[start_idx]), map_i(result.i_l[start_idx]));
        for i in (start_idx + 1)..end_idx {
            ctx.line_to(map_x(result.time[i]), map_i(result.i_l[i]));
        }
        ctx.stroke();
    }

    // Draw target voltage line (dashed, white)
    if target_vout >= v_min && target_vout <= v_max {
        ctx.set_stroke_style(&JsValue::from_str("#ffffff40"));
        ctx.set_line_width(1.0);
        let dash_pattern = js_sys::Array::new();
        dash_pattern.push(&JsValue::from_f64(5.0));
        dash_pattern.push(&JsValue::from_f64(5.0));
        ctx.set_line_dash(&dash_pattern)?;
        let target_y = map_v(target_vout);
        ctx.begin_path();
        ctx.move_to(margin_left, target_y);
        ctx.line_to(width - margin_right, target_y);
        ctx.stroke();
        ctx.set_line_dash(&js_sys::Array::new())?;
    }

    // Labels
    ctx.set_fill_style(&JsValue::from_str("#808090"));
    ctx.set_font("11px Monaco, monospace");

    // Y-axis labels
    ctx.set_text_align("right");
    let _ = ctx.fill_text(&format!("{:.2}V", v_max), margin_left - 5.0, margin_top + 5.0);
    let _ = ctx.fill_text(&format!("{:.2}V", v_min), margin_left - 5.0, height - margin_bottom);

    // X-axis labels (show current view range)
    ctx.set_text_align("center");
    let t_start_us = view_t_start * 1e6;
    let t_end_us = view_t_end * 1e6;
    let _ = ctx.fill_text(&format!("{:.1}us", t_start_us), margin_left, height - margin_bottom + 15.0);
    let _ = ctx.fill_text(&format!("{:.1}us", t_end_us), width - margin_right, height - margin_bottom + 15.0);

    // Middle time label
    let t_mid_us = (t_start_us + t_end_us) / 2.0;
    let _ = ctx.fill_text(&format!("{:.1}us", t_mid_us), width / 2.0, height - margin_bottom + 15.0);

    // Legend (left side)
    ctx.set_text_align("left");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    let _ = ctx.fill_text("Vout", margin_left + 10.0, margin_top + 12.0);
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text(&format!("iL ({:.2}A)", i_max), margin_left + 50.0, margin_top + 12.0);

    // Stats (right side)
    let ripple_mv = result.output_ripple_pp() * 1000.0;
    let zoom_pct = (t_max_data / view_duration * 100.0).min(9999.0);
    ctx.set_text_align("right");
    ctx.set_fill_style(&JsValue::from_str("#808090"));
    let _ = ctx.fill_text(
        &format!("Ripple: {:.1}mV | Settle: {:.0}us | {:.0}x",
                 ripple_mv, result.stats.settling_time * 1e6, zoom_pct / 100.0),
        width - margin_right,
        margin_top + 12.0
    );

    Ok(())
}

// ============================================================================
// RESULTS DISPLAY
// ============================================================================

fn display_results(document: &Document, report: &DesignReport) -> Result<(), JsValue> {
    // Update summary
    if let Some(elem) = document.get_element_by_id("design-summary") {
        let elem: HtmlElement = elem.dyn_into()?;
        elem.set_inner_html(&report.summary);
    }

    // Update components table
    if let Some(elem) = document.get_element_by_id("components-table") {
        let elem: HtmlElement = elem.dyn_into()?;
        let mut html = String::new();
        for comp in &report.components {
            html.push_str(&format!(
                "<tr><td class=\"component-name\">{}</td><td class=\"component-value\">{}</td></tr>",
                comp.name, comp.value
            ));
            for note in &comp.notes {
                html.push_str(&format!(
                    "<tr><td></td><td class=\"component-note\">{}</td></tr>",
                    note
                ));
            }
        }
        elem.set_inner_html(&html);
    }

    // Update performance metrics
    if let Some(elem) = document.get_element_by_id("performance-table") {
        let elem: HtmlElement = elem.dyn_into()?;
        let mut html = String::new();
        for perf in &report.performance {
            html.push_str(&format!(
                "<tr><td>{}</td><td class=\"perf-value\">{} {}</td></tr>",
                perf.metric, perf.value, perf.unit
            ));
        }
        elem.set_inner_html(&html);
    }

    // Update warnings
    if let Some(elem) = document.get_element_by_id("warnings-list") {
        let elem: HtmlElement = elem.dyn_into()?;
        if report.warnings.is_empty() {
            elem.set_inner_html("<li class=\"no-warnings\">No warnings</li>");
        } else {
            let html: String = report
                .warnings
                .iter()
                .map(|w| format!("<li class=\"warning-item\">{}</li>", w))
                .collect();
            elem.set_inner_html(&html);
        }
    }

    Ok(())
}

// ============================================================================
// SCHEMATIC DRAWING
// ============================================================================

fn draw_schematic(document: &Document, topology: TopologyType) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let canvas = document
        .get_element_by_id("schematic-canvas")
        .ok_or("Canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Could not get 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Handle high-DPI displays
    let dpr = window.device_pixel_ratio();
    let css_width = 600.0;
    let css_height = 250.0;

    canvas.set_width((css_width * dpr) as u32);
    canvas.set_height((css_height * dpr) as u32);
    let _ = canvas.style().set_property("width", &format!("{}px", css_width));
    let _ = canvas.style().set_property("height", &format!("{}px", css_height));

    ctx.scale(dpr, dpr)?;

    let width = css_width;
    let height = css_height;

    // Clear canvas
    ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
    ctx.fill_rect(0.0, 0.0, width, height);

    // Draw grid
    draw_grid(&ctx, width, height);

    // Get component values from current design
    let (vin, vout, l_val, cout_val, cin_val, r_load) = STATE.with(|state| {
        let s = state.borrow();
        let r_load = if s.iout > 0.0 { s.vout / s.iout } else { 0.0 };
        match &s.design {
            CurrentDesign::Buck(d) => (
                s.vin_nom,
                s.vout,
                d.inductor.selected_value,
                d.output_capacitor.selected_value,
                d.input_capacitor.selected_value,
                r_load,
            ),
            CurrentDesign::Boost(d) => (
                s.vin_nom,
                s.vout,
                d.inductor.selected_value,
                d.output_capacitor.selected_value,
                d.input_capacitor.selected_value,
                r_load,
            ),
            CurrentDesign::LDO(d) => (
                s.vin_nom,
                s.vout,
                0.0,
                d.output_capacitor.selected_value,
                d.input_capacitor.selected_value,
                r_load,
            ),
            CurrentDesign::None => (s.vin_nom, s.vout, 0.0, 0.0, 0.0, r_load),
        }
    });

    // Draw topology-specific schematic with values
    match topology {
        TopologyType::Buck => draw_buck_schematic(&ctx, width, height, vin, vout, l_val, cout_val, r_load),
        TopologyType::Boost => draw_boost_schematic(&ctx, width, height, vin, vout, l_val, cout_val, r_load),
        TopologyType::LDO => draw_ldo_schematic(&ctx, width, height, vin, vout, cin_val, cout_val, r_load),
        _ => Ok(()),
    }
}

/// Format component value with appropriate SI prefix
fn format_value(val: f64, unit: &str) -> String {
    if val == 0.0 {
        return format!("--{}", unit);
    }
    if val >= 1.0 {
        format!("{:.1}{}", val, unit)
    } else if val >= 1e-3 {
        format!("{:.1}m{}", val * 1e3, unit)
    } else if val >= 1e-6 {
        format!("{:.1}u{}", val * 1e6, unit)
    } else if val >= 1e-9 {
        format!("{:.1}n{}", val * 1e9, unit)
    } else {
        format!("{:.1}p{}", val * 1e12, unit)
    }
}

fn draw_grid(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_stroke_style(&JsValue::from_str("#1a1a24"));
    ctx.set_line_width(0.5);

    let grid_size = 20.0;
    let mut x = 0.0;
    while x <= width {
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, height);
        ctx.stroke();
        x += grid_size;
    }

    let mut y = 0.0;
    while y <= height {
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
        ctx.stroke();
        y += grid_size;
    }
}

fn draw_buck_schematic(ctx: &CanvasRenderingContext2d, width: f64, height: f64, vin: f64, vout: f64, l_val: f64, cout_val: f64, r_load: f64) -> Result<(), JsValue> {
    let cx = width / 2.0;
    let cy = height / 2.0;

    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);
    ctx.set_font("11px Monaco, monospace");

    // Buck converter topology:
    // Vin+ --[Q]--+--[L]--+--[Load]
    //             |       |
    //            [D]    [Cout]
    //             |       |
    // Vin- -------+-------+--GND

    let top_y = cy - 50.0;      // Top rail (Vin)
    let mid_y = cy - 10.0;      // Switch output / inductor input level
    let bot_y = cy + 70.0;      // Bottom rail (ground)
    let left_x = cx - 220.0;    // Vin position
    let sw_x = cx - 120.0;      // High-side switch
    let junc_x = cx - 40.0;     // Junction (SW output, D cathode, L input)
    let ind_x = cx + 60.0;      // Inductor center
    let out_x = cx + 160.0;     // Output node
    let load_x = cx + 220.0;    // Load position

    // Title
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text("Buck Converter", cx - 50.0, 25.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // Draw Vin source with value
    let vin_label = format!("{:.0}V", vin);
    draw_voltage_source(ctx, left_x, cy, &vin_label);

    // Draw high-side switch (horizontal on top rail)
    draw_nmos_h(ctx, sw_x, top_y, "Q");

    // Draw freewheeling diode (vertical, cathode up at junction)
    draw_diode_v(ctx, junc_x, cy + 25.0, "D");

    // Draw inductor with value
    let l_label = format_value(l_val, "H");
    draw_inductor_h(ctx, ind_x, mid_y, &l_label);

    // Draw output capacitor with value
    let cout_label = format_value(cout_val, "F");
    draw_capacitor_v(ctx, out_x, cy + 20.0, &cout_label);

    // Draw load resistor with value
    let load_label = format!("{:.1}R", r_load);
    draw_resistor_v(ctx, load_x, cy + 20.0, &load_label);

    // Draw Vout label
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text(&format!("{:.1}V", vout), out_x + 10.0, mid_y - 10.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // === WIRING ===
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);

    // Vin+ to switch input
    ctx.begin_path();
    ctx.move_to(left_x, cy - 20.0);
    ctx.line_to(left_x, top_y);
    ctx.line_to(sw_x - 25.0, top_y);
    ctx.stroke();

    // Switch output down to junction level
    ctx.begin_path();
    ctx.move_to(sw_x + 25.0, top_y);
    ctx.line_to(junc_x, top_y);
    ctx.line_to(junc_x, mid_y);
    ctx.stroke();

    // Junction dot
    ctx.begin_path();
    ctx.arc(junc_x, mid_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Junction to inductor
    ctx.begin_path();
    ctx.move_to(junc_x, mid_y);
    ctx.line_to(ind_x - 40.0, mid_y);
    ctx.stroke();

    // Diode cathode to junction
    ctx.begin_path();
    ctx.move_to(junc_x, cy + 10.0);
    ctx.line_to(junc_x, mid_y);
    ctx.stroke();

    // Inductor to output
    ctx.begin_path();
    ctx.move_to(ind_x + 40.0, mid_y);
    ctx.line_to(out_x, mid_y);
    ctx.stroke();

    // Output node dot
    ctx.begin_path();
    ctx.arc(out_x, mid_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Output to Cout
    ctx.begin_path();
    ctx.move_to(out_x, mid_y);
    ctx.line_to(out_x, cy + 5.0);
    ctx.stroke();

    // Output to Load
    ctx.begin_path();
    ctx.move_to(out_x, mid_y);
    ctx.line_to(load_x, mid_y);
    ctx.line_to(load_x, cy - 5.0);
    ctx.stroke();

    // === GROUND RAIL ===
    ctx.begin_path();
    ctx.move_to(left_x, cy + 20.0);
    ctx.line_to(left_x, bot_y);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Diode anode to ground
    ctx.begin_path();
    ctx.move_to(junc_x, cy + 40.0);
    ctx.line_to(junc_x, bot_y);
    ctx.stroke();

    // Cout to ground
    ctx.begin_path();
    ctx.move_to(out_x, cy + 35.0);
    ctx.line_to(out_x, bot_y);
    ctx.stroke();

    // Load to ground
    ctx.begin_path();
    ctx.move_to(load_x, cy + 45.0);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Ground symbol
    draw_ground(ctx, cx, bot_y);

    Ok(())
}

fn draw_boost_schematic(ctx: &CanvasRenderingContext2d, width: f64, height: f64, vin: f64, vout: f64, l_val: f64, cout_val: f64, r_load: f64) -> Result<(), JsValue> {
    let cx = width / 2.0;
    let cy = height / 2.0;

    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);
    ctx.set_font("11px Monaco, monospace");

    // Boost converter topology:
    // Vin+ --[L]--+--[D]--+--[Load]
    //             |       |
    //            [SW]   [Cout]
    //             |       |
    // Vin- -------+-------+--GND

    let top_y = cy - 50.0;      // Top rail (positive)
    let bot_y = cy + 70.0;      // Bottom rail (ground)
    let left_x = cx - 220.0;    // Vin position
    let ind_x = cx - 100.0;     // Inductor center
    let junc_x = cx;            // Junction point (L output, SW top, D input)
    let diode_x = cx + 80.0;    // Diode center
    let out_x = cx + 160.0;     // Output node
    let load_x = cx + 220.0;    // Load position

    // Title
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text("Boost Converter", cx - 50.0, 25.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // Draw Vin source with value
    let vin_label = format!("{:.0}V", vin);
    draw_voltage_source(ctx, left_x, cy, &vin_label);

    // Draw inductor with value
    let l_label = format_value(l_val, "H");
    draw_inductor_h(ctx, ind_x, top_y, &l_label);

    // Draw switch (vertical, from junction to ground)
    draw_nmos_vertical(ctx, junc_x, cy + 10.0, "Q");

    // Draw diode (horizontal, from junction to output) - pointing right
    draw_diode_h(ctx, diode_x, top_y, "D");

    // Draw output capacitor with value
    let cout_label = format_value(cout_val, "F");
    draw_capacitor_v(ctx, out_x, cy, &cout_label);

    // Draw load resistor with value
    let load_label = format!("{:.1}R", r_load);
    draw_resistor_v(ctx, load_x, cy, &load_label);

    // Draw Vout label
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text(&format!("{:.1}V", vout), out_x + 10.0, top_y - 10.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // === WIRING ===
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);

    // Vin+ to inductor input
    ctx.begin_path();
    ctx.move_to(left_x, cy - 20.0);  // Top of Vin source
    ctx.line_to(left_x, top_y);
    ctx.line_to(ind_x - 40.0, top_y);
    ctx.stroke();

    // Inductor output to junction
    ctx.begin_path();
    ctx.move_to(ind_x + 40.0, top_y);
    ctx.line_to(junc_x, top_y);
    ctx.stroke();

    // Junction dot
    ctx.begin_path();
    ctx.arc(junc_x, top_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Junction down to switch
    ctx.begin_path();
    ctx.move_to(junc_x, top_y);
    ctx.line_to(junc_x, cy - 15.0);
    ctx.stroke();

    // Junction to diode input
    ctx.begin_path();
    ctx.move_to(junc_x, top_y);
    ctx.line_to(diode_x - 15.0, top_y);
    ctx.stroke();

    // Diode output to output node
    ctx.begin_path();
    ctx.move_to(diode_x + 15.0, top_y);
    ctx.line_to(out_x, top_y);
    ctx.stroke();

    // Output node dot
    ctx.begin_path();
    ctx.arc(out_x, top_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Output to Cout top
    ctx.begin_path();
    ctx.move_to(out_x, top_y);
    ctx.line_to(out_x, cy - 15.0);
    ctx.stroke();

    // Output to load top
    ctx.begin_path();
    ctx.move_to(out_x, top_y);
    ctx.line_to(load_x, top_y);
    ctx.line_to(load_x, cy - 25.0);
    ctx.stroke();

    // === GROUND RAIL ===
    ctx.begin_path();
    ctx.move_to(left_x, cy + 20.0);  // Bottom of Vin
    ctx.line_to(left_x, bot_y);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Switch to ground
    ctx.begin_path();
    ctx.move_to(junc_x, cy + 35.0);
    ctx.line_to(junc_x, bot_y);
    ctx.stroke();

    // Cout to ground
    ctx.begin_path();
    ctx.move_to(out_x, cy + 15.0);
    ctx.line_to(out_x, bot_y);
    ctx.stroke();

    // Load to ground
    ctx.begin_path();
    ctx.move_to(load_x, cy + 25.0);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Ground symbol at center of bottom rail
    draw_ground(ctx, cx, bot_y);

    Ok(())
}

fn draw_ldo_schematic(ctx: &CanvasRenderingContext2d, width: f64, height: f64, vin: f64, vout: f64, cin_val: f64, cout_val: f64, r_load: f64) -> Result<(), JsValue> {
    let cx = width / 2.0;
    let cy = height / 2.0;

    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);
    ctx.set_font("11px Monaco, monospace");

    // LDO topology (simple linear regulator):
    // Vin+ --[Cin]--[LDO]--[Cout]--[Load]
    //          |      |      |       |
    // GND -----+------+------+-------+

    let top_y = cy - 50.0;      // Top rail
    let bot_y = cy + 70.0;      // Ground rail
    let left_x = cx - 220.0;    // Vin position
    let cin_x = cx - 120.0;     // Input cap
    let ldo_x = cx;             // LDO IC center
    let cout_x = cx + 120.0;    // Output cap
    let load_x = cx + 200.0;    // Load

    // Title
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text("LDO Regulator", cx - 45.0, 25.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // Draw Vin source with value
    let vin_label = format!("{:.0}V", vin);
    draw_voltage_source(ctx, left_x, cy, &vin_label);

    // Draw input capacitor with value
    let cin_label = format_value(cin_val, "F");
    draw_capacitor_v(ctx, cin_x, cy, &cin_label);

    // Draw LDO IC as box with pins
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.stroke_rect(ldo_x - 35.0, cy - 25.0, 70.0, 50.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    let _ = ctx.fill_text("LDO", ldo_x - 12.0, cy + 5.0);
    // Pin labels
    ctx.set_font("10px Monaco, monospace");
    let _ = ctx.fill_text("IN", ldo_x - 30.0, cy - 30.0);
    let _ = ctx.fill_text("OUT", ldo_x + 15.0, cy - 30.0);
    let _ = ctx.fill_text("GND", ldo_x - 8.0, cy + 38.0);
    ctx.set_font("11px Monaco, monospace");

    // Draw output capacitor with value
    let cout_label = format_value(cout_val, "F");
    draw_capacitor_v(ctx, cout_x, cy, &cout_label);

    // Draw load resistor with value
    let load_label = format!("{:.1}R", r_load);
    draw_resistor_v(ctx, load_x, cy, &load_label);

    // Draw Vout label
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    let _ = ctx.fill_text(&format!("{:.1}V", vout), cout_x + 15.0, top_y - 10.0);
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));

    // === WIRING ===
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);

    // Vin+ to input cap
    ctx.begin_path();
    ctx.move_to(left_x, cy - 20.0);
    ctx.line_to(left_x, top_y);
    ctx.line_to(cin_x, top_y);
    ctx.line_to(cin_x, cy - 15.0);
    ctx.stroke();

    // Input cap node
    ctx.begin_path();
    ctx.arc(cin_x, top_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Input cap to LDO IN
    ctx.begin_path();
    ctx.move_to(cin_x, top_y);
    ctx.line_to(ldo_x - 35.0, top_y);
    ctx.line_to(ldo_x - 35.0, cy - 25.0);
    ctx.stroke();

    // LDO OUT to output cap
    ctx.begin_path();
    ctx.move_to(ldo_x + 35.0, cy - 25.0);
    ctx.line_to(ldo_x + 35.0, top_y);
    ctx.line_to(cout_x, top_y);
    ctx.line_to(cout_x, cy - 15.0);
    ctx.stroke();

    // Output node
    ctx.begin_path();
    ctx.arc(cout_x, top_y, 4.0, 0.0, 2.0 * PI).ok();
    ctx.fill();

    // Output to load
    ctx.begin_path();
    ctx.move_to(cout_x, top_y);
    ctx.line_to(load_x, top_y);
    ctx.line_to(load_x, cy - 25.0);
    ctx.stroke();

    // === GROUND RAIL ===
    ctx.begin_path();
    ctx.move_to(left_x, cy + 20.0);
    ctx.line_to(left_x, bot_y);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Input cap to ground
    ctx.begin_path();
    ctx.move_to(cin_x, cy + 15.0);
    ctx.line_to(cin_x, bot_y);
    ctx.stroke();

    // LDO GND to ground
    ctx.begin_path();
    ctx.move_to(ldo_x, cy + 25.0);
    ctx.line_to(ldo_x, bot_y);
    ctx.stroke();

    // Output cap to ground
    ctx.begin_path();
    ctx.move_to(cout_x, cy + 15.0);
    ctx.line_to(cout_x, bot_y);
    ctx.stroke();

    // Load to ground
    ctx.begin_path();
    ctx.move_to(load_x, cy + 25.0);
    ctx.line_to(load_x, bot_y);
    ctx.stroke();

    // Ground symbol
    draw_ground(ctx, cx, bot_y);

    Ok(())
}

// ============================================================================
// COMPONENT SYMBOLS
// ============================================================================

fn draw_voltage_source(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Circle
    ctx.begin_path();
    ctx.arc(x, y, 20.0, 0.0, 2.0 * PI).ok();
    ctx.stroke();

    // Plus sign
    ctx.begin_path();
    ctx.move_to(x - 8.0, y - 8.0);
    ctx.line_to(x + 8.0, y - 8.0);
    ctx.move_to(x, y - 16.0);
    ctx.line_to(x, y);
    ctx.stroke();

    // Minus sign
    ctx.begin_path();
    ctx.move_to(x - 8.0, y + 8.0);
    ctx.line_to(x + 8.0, y + 8.0);
    ctx.stroke();

    // Label
    let _ = ctx.fill_text(label, x - 10.0, y - 28.0);
}


fn draw_ground(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    ctx.begin_path();
    ctx.move_to(x - 10.0, y);
    ctx.line_to(x + 10.0, y);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(x - 6.0, y + 4.0);
    ctx.line_to(x + 6.0, y + 4.0);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(x - 2.0, y + 8.0);
    ctx.line_to(x + 2.0, y + 8.0);
    ctx.stroke();
}

// ============================================================================
// ORIENTED COMPONENT SYMBOLS
// ============================================================================

/// Horizontal inductor (coils along x-axis)
fn draw_inductor_h(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // 4 semicircle coils, horizontal
    for i in 0..4 {
        let coil_x = x - 28.0 + (i as f64 * 14.0);
        ctx.begin_path();
        ctx.arc(coil_x, y, 7.0, PI, 0.0).ok();
        ctx.stroke();
    }
    // Label above
    let _ = ctx.fill_text(label, x - 5.0, y - 15.0);
}

/// Vertical resistor (zigzag along y-axis)
fn draw_resistor_v(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    ctx.begin_path();
    ctx.move_to(x, y - 25.0);
    ctx.line_to(x, y - 18.0);
    ctx.line_to(x - 6.0, y - 14.0);
    ctx.line_to(x + 6.0, y - 6.0);
    ctx.line_to(x - 6.0, y + 2.0);
    ctx.line_to(x + 6.0, y + 10.0);
    ctx.line_to(x, y + 18.0);
    ctx.line_to(x, y + 25.0);
    ctx.stroke();
    let _ = ctx.fill_text(label, x + 10.0, y + 5.0);
}

/// Vertical capacitor (plates along y-axis)
fn draw_capacitor_v(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Top lead
    ctx.begin_path();
    ctx.move_to(x, y - 15.0);
    ctx.line_to(x, y - 5.0);
    ctx.stroke();
    // Top plate
    ctx.begin_path();
    ctx.move_to(x - 10.0, y - 5.0);
    ctx.line_to(x + 10.0, y - 5.0);
    ctx.stroke();
    // Bottom plate
    ctx.begin_path();
    ctx.move_to(x - 10.0, y + 5.0);
    ctx.line_to(x + 10.0, y + 5.0);
    ctx.stroke();
    // Bottom lead
    ctx.begin_path();
    ctx.move_to(x, y + 5.0);
    ctx.line_to(x, y + 15.0);
    ctx.stroke();
    let _ = ctx.fill_text(label, x + 12.0, y + 5.0);
}

/// Horizontal diode (triangle pointing right, anode left, cathode right)
fn draw_diode_h(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Triangle (anode)
    ctx.begin_path();
    ctx.move_to(x - 8.0, y - 8.0);
    ctx.line_to(x - 8.0, y + 8.0);
    ctx.line_to(x + 6.0, y);
    ctx.close_path();
    ctx.stroke();
    // Cathode bar
    ctx.begin_path();
    ctx.move_to(x + 6.0, y - 8.0);
    ctx.line_to(x + 6.0, y + 8.0);
    ctx.stroke();
    // Label
    let _ = ctx.fill_text(label, x - 3.0, y - 12.0);
}

/// Vertical diode (triangle pointing up = cathode up, anode down)
fn draw_diode_v(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Top lead (cathode)
    ctx.begin_path();
    ctx.move_to(x, y - 15.0);
    ctx.line_to(x, y - 6.0);
    ctx.stroke();
    // Cathode bar
    ctx.begin_path();
    ctx.move_to(x - 8.0, y - 6.0);
    ctx.line_to(x + 8.0, y - 6.0);
    ctx.stroke();
    // Triangle (anode at bottom)
    ctx.begin_path();
    ctx.move_to(x - 8.0, y + 6.0);
    ctx.line_to(x + 8.0, y + 6.0);
    ctx.line_to(x, y - 6.0);
    ctx.close_path();
    ctx.stroke();
    // Bottom lead (anode)
    ctx.begin_path();
    ctx.move_to(x, y + 6.0);
    ctx.line_to(x, y + 15.0);
    ctx.stroke();
    let _ = ctx.fill_text(label, x + 12.0, y);
}

/// Horizontal NMOS switch (drain left, source right, gate at bottom)
fn draw_nmos_h(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Simplified switch symbol: two contacts with gap
    ctx.begin_path();
    ctx.move_to(x - 25.0, y);
    ctx.line_to(x - 8.0, y);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(x + 8.0, y);
    ctx.line_to(x + 25.0, y);
    ctx.stroke();

    // Switch element (tilted line for "switch open" appearance, but we show closed)
    ctx.begin_path();
    ctx.move_to(x - 8.0, y);
    ctx.line_to(x + 8.0, y);
    ctx.stroke();

    // Gate connection down
    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(x, y + 12.0);
    ctx.stroke();

    // Small circle at junction
    ctx.begin_path();
    ctx.arc(x, y, 3.0, 0.0, 2.0 * PI).ok();
    ctx.stroke();

    let _ = ctx.fill_text(label, x - 5.0, y - 8.0);
}

/// Vertical NMOS (drain at top, source at bottom, gate left)
fn draw_nmos_vertical(ctx: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    // Vertical channel bar
    ctx.begin_path();
    ctx.move_to(x + 5.0, y - 12.0);
    ctx.line_to(x + 5.0, y + 12.0);
    ctx.stroke();

    // Gate bar (parallel to channel, on left)
    ctx.begin_path();
    ctx.move_to(x - 3.0, y - 10.0);
    ctx.line_to(x - 3.0, y + 10.0);
    ctx.stroke();

    // Gate lead
    ctx.begin_path();
    ctx.move_to(x - 3.0, y);
    ctx.line_to(x - 15.0, y);
    ctx.stroke();

    // Drain (top)
    ctx.begin_path();
    ctx.move_to(x + 5.0, y - 12.0);
    ctx.line_to(x + 5.0, y - 25.0);
    ctx.stroke();

    // Source (bottom) with arrow
    ctx.begin_path();
    ctx.move_to(x + 5.0, y + 12.0);
    ctx.line_to(x + 5.0, y + 25.0);
    ctx.stroke();

    // Arrow on source (pointing into channel)
    ctx.begin_path();
    ctx.move_to(x + 5.0, y + 8.0);
    ctx.line_to(x + 10.0, y + 12.0);
    ctx.line_to(x + 5.0, y + 16.0);
    ctx.stroke();

    let _ = ctx.fill_text(label, x + 15.0, y);
}
