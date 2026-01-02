//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: demo_runner.rs | OPENCV/src/demo_runner.rs
//! PURPOSE: Demo lifecycle management and animation loop
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, ImageData};

use crate::image_processing;

/// Current demo state
struct DemoState {
    running: bool,
    lesson_id: usize,
    animation_id: Option<i32>,
    camera_ready: bool,
    // Demo parameters
    low_threshold: f32,
    high_threshold: f32,
    blur_radius: u32,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            running: false,
            lesson_id: 0,
            animation_id: None,
            camera_ready: false,
            low_threshold: 50.0,
            high_threshold: 150.0,
            blur_radius: 2,
        }
    }
}

thread_local! {
    static STATE: RefCell<DemoState> = RefCell::new(DemoState::default());
}

/// Stop any running demo
pub fn stop_demo() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.running = false;
        state.camera_ready = false;

        // Cancel animation frame if running
        if let Some(id) = state.animation_id.take() {
            if let Some(window) = window() {
                let _ = window.cancel_animation_frame(id);
            }
        }
    });
}

/// Start a camera-based demo
pub fn start_camera_demo(lesson_id: usize) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.running = true;
        state.lesson_id = lesson_id;
        state.camera_ready = false;
    });

    // Set up controls based on lesson
    setup_demo_controls(lesson_id);
}

/// Called when camera permission is granted
pub fn on_camera_ready() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.camera_ready = true;
    });

    // Start the animation loop
    start_animation_loop();
}

/// Start a canvas-only demo (no camera)
pub fn start_canvas_demo(lesson_id: usize) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.running = true;
        state.lesson_id = lesson_id;
    });

    setup_demo_controls(lesson_id);
    render_canvas_demo(lesson_id);
}

/// Start a side-by-side comparison demo
pub fn start_sidebyside_demo(lesson_id: usize) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.running = true;
        state.lesson_id = lesson_id;
    });

    setup_demo_controls(lesson_id);
    render_sidebyside_demo(lesson_id);
}

/// Set up demo controls based on lesson
fn setup_demo_controls(lesson_id: usize) {
    let controls_html = match lesson_id {
        // Edge Detection (Canny)
        3 => {
            r#"
            <div class="control-group">
                <label>Low Threshold: <span id="low-val">50</span></label>
                <input type="range" id="low-threshold" min="0" max="255" value="50">
            </div>
            <div class="control-group">
                <label>High Threshold: <span id="high-val">150</span></label>
                <input type="range" id="high-threshold" min="0" max="255" value="150">
            </div>
            "#
        }
        // Noise Reduction
        4 => {
            r#"
            <div class="control-group">
                <label>Blur Radius: <span id="blur-val">2</span></label>
                <input type="range" id="blur-radius" min="1" max="10" value="2">
            </div>
            "#
        }
        // Corner Detection
        5 => {
            r#"
            <div class="control-group">
                <label>Threshold: <span id="threshold-val">100</span></label>
                <input type="range" id="corner-threshold" min="0" max="255" value="100">
            </div>
            "#
        }
        // Thresholding
        7 => {
            r#"
            <div class="control-group">
                <label>Threshold: <span id="threshold-val">128</span></label>
                <input type="range" id="threshold" min="0" max="255" value="128">
            </div>
            "#
        }
        // Color Tracking
        10 => {
            r#"
            <div class="control-group">
                <label>Hue: <span id="hue-val">120</span></label>
                <input type="range" id="hue" min="0" max="180" value="120">
            </div>
            <div class="control-group">
                <label>Hue Range: <span id="hue-range-val">20</span></label>
                <input type="range" id="hue-range" min="5" max="50" value="20">
            </div>
            "#
        }
        _ => "",
    };

    if let Some(controls) = get_element("demo-controls") {
        controls.set_inner_html(controls_html);
    }

    // Wire up slider event listeners
    wire_slider_events(lesson_id);
}

/// Wire up slider change events
fn wire_slider_events(lesson_id: usize) {
    match lesson_id {
        3 => {
            // Edge Detection sliders
            wire_slider("low-threshold", "low-val", |val| {
                STATE.with(|s| s.borrow_mut().low_threshold = val as f32);
            });
            wire_slider("high-threshold", "high-val", |val| {
                STATE.with(|s| s.borrow_mut().high_threshold = val as f32);
            });
        }
        4 => {
            // Blur slider
            wire_slider("blur-radius", "blur-val", |val| {
                STATE.with(|s| s.borrow_mut().blur_radius = val as u32);
            });
        }
        7 => {
            // Threshold slider
            wire_slider("threshold", "threshold-val", |val| {
                STATE.with(|s| s.borrow_mut().low_threshold = val as f32);
            });
        }
        _ => {}
    }
}

/// Wire a single slider to update state and display
fn wire_slider(slider_id: &str, display_id: &str, update_fn: fn(i32)) {
    let document = match window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    if let Some(slider) = document.get_element_by_id(slider_id) {
        let slider: HtmlInputElement = slider.unchecked_into();
        let display_id = display_id.to_string();
        let slider_id_owned = slider_id.to_string();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(slider) = document.get_element_by_id(&slider_id_owned) {
                    let slider: HtmlInputElement = slider.unchecked_into();
                    let val: i32 = slider.value().parse().unwrap_or(0);

                    // Update display
                    if let Some(display) = document.get_element_by_id(&display_id) {
                        display.set_text_content(Some(&val.to_string()));
                    }

                    // Update state
                    update_fn(val);
                }
            }
        }) as Box<dyn Fn(web_sys::Event)>);

        let _ = slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

/// Start the animation loop for camera processing
fn start_animation_loop() {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let should_continue = STATE.with(|s| {
            let state = s.borrow();
            state.running && state.camera_ready
        });

        if should_continue {
            process_frame();

            // Request next frame
            if let Some(window) = window() {
                if let Some(ref closure) = *f.borrow() {
                    let id = window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .unwrap_or(0);
                    STATE.with(|s| s.borrow_mut().animation_id = Some(id));
                }
            }
        }
    }));

    // Start the loop
    if let Some(window) = window() {
        if let Some(ref closure) = *g.borrow() {
            let id = window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .unwrap_or(0);
            STATE.with(|s| s.borrow_mut().animation_id = Some(id));
        }
    }
}

/// Process a single video frame
fn process_frame() {
    let (lesson_id, low_thresh, high_thresh, blur_radius) = STATE.with(|s| {
        let state = s.borrow();
        (
            state.lesson_id,
            state.low_threshold,
            state.high_threshold,
            state.blur_radius,
        )
    });

    // Get video and canvas elements
    let video = match crate::camera::get_video_element() {
        Some(v) => v,
        None => return,
    };

    let canvas = match get_canvas("output-canvas") {
        Some(c) => c,
        None => return,
    };

    let ctx = match canvas
        .get_context("2d")
        .ok()
        .flatten()
        .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
    {
        Some(c) => c,
        None => return,
    };

    // Get video dimensions
    let width = video.video_width();
    let height = video.video_height();

    if width == 0 || height == 0 {
        return;
    }

    // Set canvas size to match video
    canvas.set_width(width);
    canvas.set_height(height);

    // Draw video frame to canvas
    let _ = ctx.draw_image_with_html_video_element(&video, 0.0, 0.0);

    // Get image data
    let image_data = match ctx.get_image_data(0.0, 0.0, width as f64, height as f64) {
        Ok(data) => data,
        Err(_) => return,
    };

    // Process based on lesson
    let processed = match lesson_id {
        3 => image_processing::canny_edge(&image_data, low_thresh, high_thresh),
        4 => image_processing::gaussian_blur(&image_data, blur_radius),
        5 => image_processing::harris_corners(&image_data, low_thresh),
        6 => image_processing::simple_blob_detection(&image_data),
        7 => image_processing::threshold(&image_data, low_thresh as u8),
        9 => image_processing::find_contours(&image_data, low_thresh as u8),
        10 => image_processing::color_tracking(&image_data, low_thresh as u8, 20),
        11 => image_processing::simple_face_detection(&image_data),
        _ => image_processing::grayscale(&image_data),
    };

    // Put processed image back
    if let Ok(new_image_data) =
        ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&processed), width, height)
    {
        let _ = ctx.put_image_data(&new_image_data, 0.0, 0.0);
    }
}

/// Render a static canvas demo
fn render_canvas_demo(lesson_id: usize) {
    match lesson_id {
        1 => render_color_space_demo(),
        2 => render_convolution_demo(),
        8 => render_transform_demo(),
        _ => render_placeholder_demo(lesson_id),
    }
}

/// Lesson 1: Interactive color space demo
fn render_color_space_demo() {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(500);
        canvas.set_height(300);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            // Background
            ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
            ctx.fill_rect(0.0, 0.0, 500.0, 300.0);

            // Draw RGB color bars
            let bar_height = 40.0;
            let start_x = 120.0;
            let start_y = 30.0;

            // Red gradient
            for i in 0..256 {
                ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},0,0)", i)));
                ctx.fill_rect(start_x + i as f64, start_y, 1.0, bar_height);
            }
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("14px Inter, sans-serif");
            ctx.set_text_align("right");
            let _ = ctx.fill_text("Red", start_x - 10.0, start_y + 25.0);

            // Green gradient
            for i in 0..256 {
                ctx.set_fill_style(&JsValue::from_str(&format!("rgb(0,{},0)", i)));
                ctx.fill_rect(start_x + i as f64, start_y + 50.0, 1.0, bar_height);
            }
            let _ = ctx.fill_text("Green", start_x - 10.0, start_y + 75.0);

            // Blue gradient
            for i in 0..256 {
                ctx.set_fill_style(&JsValue::from_str(&format!("rgb(0,0,{})", i)));
                ctx.fill_rect(start_x + i as f64, start_y + 100.0, 1.0, bar_height);
            }
            let _ = ctx.fill_text("Blue", start_x - 10.0, start_y + 125.0);

            // Grayscale gradient
            for i in 0..256 {
                ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", i, i, i)));
                ctx.fill_rect(start_x + i as f64, start_y + 160.0, 1.0, bar_height);
            }
            let _ = ctx.fill_text("Gray", start_x - 10.0, start_y + 185.0);

            // Color wheel hint
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("12px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("RGB values range from 0-255", 250.0, 280.0);
        }
    }
}

/// Lesson 2: Interactive convolution demo
fn render_convolution_demo() {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(500);
        canvas.set_height(350);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            // Background
            ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
            ctx.fill_rect(0.0, 0.0, 500.0, 350.0);

            let cell_size = 40.0;

            // Sample image patch (5x5)
            let image: [[u8; 5]; 5] = [
                [50, 50, 50, 50, 50],
                [50, 200, 200, 200, 50],
                [50, 200, 255, 200, 50],
                [50, 200, 200, 200, 50],
                [50, 50, 50, 50, 50],
            ];

            // Draw image grid
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("12px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Input Image", 100.0, 20.0);

            for (y, row) in image.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    let px = 20.0 + x as f64 * cell_size;
                    let py = 30.0 + y as f64 * cell_size;

                    ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", val, val, val)));
                    ctx.fill_rect(px, py, cell_size - 2.0, cell_size - 2.0);

                    ctx.set_fill_style(&JsValue::from_str(if val > 128 { "#000" } else { "#fff" }));
                    ctx.set_font("10px JetBrains Mono, monospace");
                    let _ = ctx.fill_text(&val.to_string(), px + cell_size / 2.0 - 1.0, py + cell_size / 2.0 + 4.0);
                }
            }

            // Draw kernel (3x3 blur)
            let kernel: [[f32; 3]; 3] = [
                [1.0/9.0, 1.0/9.0, 1.0/9.0],
                [1.0/9.0, 1.0/9.0, 1.0/9.0],
                [1.0/9.0, 1.0/9.0, 1.0/9.0],
            ];

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("12px Inter, sans-serif");
            let _ = ctx.fill_text("Blur Kernel", 330.0, 20.0);

            for (y, row) in kernel.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    let px = 260.0 + x as f64 * cell_size;
                    let py = 30.0 + y as f64 * cell_size;

                    ctx.set_fill_style(&JsValue::from_str("#1a3a5a"));
                    ctx.fill_rect(px, py, cell_size - 2.0, cell_size - 2.0);

                    ctx.set_fill_style(&JsValue::from_str("#0099ff"));
                    ctx.set_font("10px JetBrains Mono, monospace");
                    let _ = ctx.fill_text(&format!("{:.2}", val), px + cell_size / 2.0 - 1.0, py + cell_size / 2.0 + 4.0);
                }
            }

            // Arrow
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(230.0, 120.0);
            ctx.line_to(250.0, 120.0);
            ctx.move_to(245.0, 115.0);
            ctx.line_to(250.0, 120.0);
            ctx.line_to(245.0, 125.0);
            ctx.stroke();

            // Output explanation
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("11px Inter, sans-serif");
            ctx.set_text_align("left");
            let _ = ctx.fill_text("Convolution slides the kernel over the image,", 20.0, 260.0);
            let _ = ctx.fill_text("multiplying and summing at each position.", 20.0, 278.0);

            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            let _ = ctx.fill_text("Center pixel (255) becomes:", 20.0, 310.0);
            let _ = ctx.fill_text("(50+200+50+200+255+200+50+200+50) / 9 = 139", 20.0, 328.0);
        }
    }
}

/// Lesson 8: Transform demo placeholder
fn render_transform_demo() {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(500);
        canvas.set_height(300);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
            ctx.fill_rect(0.0, 0.0, 500.0, 300.0);

            // Draw original square
            ctx.set_fill_style(&JsValue::from_str("#0066CC"));
            ctx.fill_rect(50.0, 100.0, 80.0, 80.0);

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("12px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Original", 90.0, 210.0);

            // Draw rotated square
            ctx.save();
            let _ = ctx.translate(220.0, 140.0);
            let _ = ctx.rotate(0.5);
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.fill_rect(-40.0, -40.0, 80.0, 80.0);
            ctx.restore();
            let _ = ctx.fill_text("Rotated", 220.0, 210.0);

            // Draw scaled square
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            ctx.fill_rect(310.0, 80.0, 120.0, 120.0);
            let _ = ctx.fill_text("Scaled", 370.0, 230.0);

            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("Geometric transforms change position, rotation, and scale", 250.0, 280.0);
        }
    }
}

/// Fallback placeholder
fn render_placeholder_demo(lesson_id: usize) {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(400);
        canvas.set_height(200);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            ctx.set_fill_style(&JsValue::from_str("#1a1a24"));
            ctx.fill_rect(0.0, 0.0, 400.0, 200.0);

            ctx.set_fill_style(&JsValue::from_str("#0066CC"));
            ctx.set_font("16px sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(
                &format!("Demo for Lesson {} coming soon", lesson_id + 1),
                200.0,
                100.0,
            );
        }
    }
}

/// Render a side-by-side comparison demo
fn render_sidebyside_demo(_lesson_id: usize) {
    // Placeholder for before/after canvases
    for canvas_id in &["before-canvas", "after-canvas"] {
        if let Some(canvas) = get_canvas(canvas_id) {
            canvas.set_width(300);
            canvas.set_height(200);

            if let Some(ctx) = canvas
                .get_context("2d")
                .ok()
                .flatten()
                .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
            {
                ctx.set_fill_style(&JsValue::from_str("#1a1a24"));
                ctx.fill_rect(0.0, 0.0, 300.0, 200.0);

                ctx.set_fill_style(&JsValue::from_str("#0066CC"));
                ctx.set_font("14px sans-serif");
                ctx.set_text_align("center");
                let _ = ctx.fill_text(*canvas_id, 150.0, 100.0);
            }
        }
    }
}

/// Get an HTML element by ID
fn get_element(id: &str) -> Option<web_sys::Element> {
    window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

/// Get a canvas element by ID
fn get_canvas(id: &str) -> Option<HtmlCanvasElement> {
    get_element(id).map(|e| e.unchecked_into())
}
