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

/// Set up demo controls based on lesson (new curriculum: 17 lessons)
fn setup_demo_controls(lesson_id: usize) {
    let controls_html = match lesson_id {
        // Edge Detection (Canny) - Lesson 5
        5 => {
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
        // Noise Reduction - Lesson 6
        6 => {
            r#"
            <div class="control-group">
                <label>Blur Radius: <span id="blur-val">2</span></label>
                <input type="range" id="blur-radius" min="1" max="10" value="2">
            </div>
            "#
        }
        // Corner Detection - Lesson 7
        7 => {
            r#"
            <div class="control-group">
                <label>Threshold: <span id="threshold-val">100</span></label>
                <input type="range" id="corner-threshold" min="0" max="255" value="100">
            </div>
            "#
        }
        // Thresholding - Lesson 9
        9 => {
            r#"
            <div class="control-group">
                <label>Threshold: <span id="threshold-val">128</span></label>
                <input type="range" id="threshold" min="0" max="255" value="128">
            </div>
            "#
        }
        // Color Tracking - Lesson 15
        15 => {
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

/// Wire up slider change events (new curriculum: 17 lessons)
fn wire_slider_events(lesson_id: usize) {
    match lesson_id {
        5 => {
            // Edge Detection sliders - Lesson 5
            wire_slider("low-threshold", "low-val", |val| {
                STATE.with(|s| s.borrow_mut().low_threshold = val as f32);
            });
            wire_slider("high-threshold", "high-val", |val| {
                STATE.with(|s| s.borrow_mut().high_threshold = val as f32);
            });
        }
        6 => {
            // Blur slider - Lesson 6
            wire_slider("blur-radius", "blur-val", |val| {
                STATE.with(|s| s.borrow_mut().blur_radius = val as u32);
            });
        }
        9 => {
            // Threshold slider - Lesson 9
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

    // Process based on lesson (new curriculum: 17 lessons across 6 phases)
    let processed = match lesson_id {
        5 => image_processing::canny_edge(&image_data, low_thresh, high_thresh), // Edge Detection
        6 => image_processing::gaussian_blur(&image_data, blur_radius),          // Noise Reduction
        7 => image_processing::harris_corners(&image_data, low_thresh),          // Corner Detection
        8 => image_processing::simple_blob_detection(&image_data),               // Blob Detection
        9 => image_processing::threshold(&image_data, low_thresh as u8),         // Thresholding
        12 => image_processing::find_contours(&image_data, low_thresh as u8), // Contour Detection
        15 => image_processing::color_tracking(&image_data, low_thresh as u8, 20), // Color Tracking
        16 => image_processing::simple_face_detection(&image_data),           // Face Detection
        _ => image_processing::grayscale(&image_data),
    };

    // Put processed image back
    if let Ok(new_image_data) = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&processed),
        width,
        height,
    ) {
        let _ = ctx.put_image_data(&new_image_data, 0.0, 0.0);
    }
}

/// Render a static canvas demo
fn render_canvas_demo(lesson_id: usize) {
    match lesson_id {
        1 => render_color_space_demo(),   // Pixels & Color Spaces
        2 => render_pinhole_demo(),       // Pinhole Camera Model
        3 => render_camera_matrix_demo(), // Camera Matrix & Projection
        4 => render_convolution_demo(),   // Convolution
        10 => render_transform_demo(),    // Image Transformations
        11 => render_homography_demo(),   // Homography
        13 => render_stereo_demo(),       // Stereo Vision & Depth
        14 => render_epipolar_demo(),     // Epipolar Geometry
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

/// Lesson 2: Pinhole Camera Model demo
fn render_pinhole_demo() {
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

            // Draw 3D object (simple house shape)
            ctx.set_stroke_style(&JsValue::from_str("#00CC88"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(50.0, 180.0);
            ctx.line_to(50.0, 120.0);
            ctx.line_to(80.0, 90.0);
            ctx.line_to(110.0, 120.0);
            ctx.line_to(110.0, 180.0);
            ctx.line_to(50.0, 180.0);
            ctx.stroke();

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("11px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("3D Object", 80.0, 200.0);
            let _ = ctx.fill_text("(X, Y, Z)", 80.0, 215.0);

            // Draw pinhole
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.begin_path();
            let _ = ctx.arc(250.0, 150.0, 8.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            let _ = ctx.fill_text("Pinhole", 250.0, 180.0);
            let _ = ctx.fill_text("(O)", 250.0, 195.0);

            // Draw image plane
            ctx.set_stroke_style(&JsValue::from_str("#CC6600"));
            ctx.set_line_width(3.0);
            ctx.begin_path();
            ctx.move_to(400.0, 80.0);
            ctx.line_to(400.0, 220.0);
            ctx.stroke();

            // Draw projected (inverted) house
            ctx.set_stroke_style(&JsValue::from_str("#CC6600"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(410.0, 120.0);
            ctx.line_to(410.0, 160.0);
            ctx.line_to(430.0, 180.0);
            ctx.line_to(450.0, 160.0);
            ctx.line_to(450.0, 120.0);
            ctx.line_to(410.0, 120.0);
            ctx.stroke();

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            let _ = ctx.fill_text("Image", 430.0, 240.0);
            let _ = ctx.fill_text("(x, y)", 430.0, 255.0);

            // Draw rays
            ctx.set_stroke_style(&JsValue::from_str("#666"));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(80.0, 90.0);
            ctx.line_to(250.0, 150.0);
            ctx.line_to(430.0, 180.0);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(80.0, 180.0);
            ctx.line_to(250.0, 150.0);
            ctx.line_to(430.0, 120.0);
            ctx.stroke();

            // Formula
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("12px JetBrains Mono, monospace");
            let _ = ctx.fill_text("x = f · X/Z    y = f · Y/Z", 250.0, 280.0);
        }
    }
}

/// Lesson 3: Camera Matrix demo
fn render_camera_matrix_demo() {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(500);
        canvas.set_height(320);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
            ctx.fill_rect(0.0, 0.0, 500.0, 320.0);

            // Title
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("14px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("The Camera Intrinsic Matrix K", 250.0, 25.0);

            // Draw matrix K
            ctx.set_font("16px JetBrains Mono, monospace");
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            let _ = ctx.fill_text("K = ", 70.0, 90.0);

            // Matrix brackets
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(100.0, 50.0);
            ctx.line_to(95.0, 50.0);
            ctx.line_to(95.0, 130.0);
            ctx.line_to(100.0, 130.0);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(230.0, 50.0);
            ctx.line_to(235.0, 50.0);
            ctx.line_to(235.0, 130.0);
            ctx.line_to(230.0, 130.0);
            ctx.stroke();

            // Matrix elements
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.set_font("14px JetBrains Mono, monospace");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("fx", 120.0, 75.0);
            let _ = ctx.fill_text("0", 165.0, 75.0);
            let _ = ctx.fill_text("cx", 210.0, 75.0);
            let _ = ctx.fill_text("0", 120.0, 95.0);
            let _ = ctx.fill_text("fy", 165.0, 95.0);
            let _ = ctx.fill_text("cy", 210.0, 95.0);
            let _ = ctx.fill_text("0", 120.0, 115.0);
            let _ = ctx.fill_text("0", 165.0, 115.0);
            let _ = ctx.fill_text("1", 210.0, 115.0);

            // Parameter descriptions
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("11px Inter, sans-serif");
            ctx.set_text_align("left");
            let _ = ctx.fill_text("fx, fy = focal lengths (pixels)", 280.0, 75.0);
            let _ = ctx.fill_text("cx, cy = principal point", 280.0, 95.0);
            let _ = ctx.fill_text("(image center offset)", 280.0, 110.0);

            // Full projection equation
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("12px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Full Projection: p = K · [R | t] · P", 250.0, 160.0);

            // Draw diagram of projection
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            let _ = ctx.fill_text("World Point", 80.0, 200.0);
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            let _ = ctx.fill_text("[R|t]", 180.0, 200.0);
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            let _ = ctx.fill_text("Camera Coords", 280.0, 200.0);
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            let _ = ctx.fill_text("K", 370.0, 200.0);
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            let _ = ctx.fill_text("Pixel", 430.0, 200.0);

            // Arrows
            ctx.set_stroke_style(&JsValue::from_str("#666"));
            ctx.set_line_width(1.5);
            for x in [140.0, 340.0, 400.0].iter() {
                ctx.begin_path();
                ctx.move_to(*x, 195.0);
                ctx.line_to(*x + 20.0, 195.0);
                ctx.move_to(*x + 15.0, 190.0);
                ctx.line_to(*x + 20.0, 195.0);
                ctx.line_to(*x + 15.0, 200.0);
                ctx.stroke();
            }

            // Intrinsic vs Extrinsic
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("K = Intrinsic (camera internals)", 150.0, 260.0);
            let _ = ctx.fill_text("[R|t] = Extrinsic (camera pose)", 350.0, 260.0);

            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("10px Inter, sans-serif");
            let _ = ctx.fill_text("Calibrate once", 150.0, 280.0);
            let _ = ctx.fill_text("Changes every frame", 350.0, 280.0);
        }
    }
}

/// Lesson 4: Interactive convolution demo
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

                    ctx.set_fill_style(&JsValue::from_str(&format!(
                        "rgb({},{},{})",
                        val, val, val
                    )));
                    ctx.fill_rect(px, py, cell_size - 2.0, cell_size - 2.0);

                    ctx.set_fill_style(&JsValue::from_str(if val > 128 { "#000" } else { "#fff" }));
                    ctx.set_font("10px JetBrains Mono, monospace");
                    let _ = ctx.fill_text(
                        &val.to_string(),
                        px + cell_size / 2.0 - 1.0,
                        py + cell_size / 2.0 + 4.0,
                    );
                }
            }

            // Draw kernel (3x3 blur)
            let kernel: [[f32; 3]; 3] = [
                [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
                [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
                [1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
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
                    let _ = ctx.fill_text(
                        &format!("{:.2}", val),
                        px + cell_size / 2.0 - 1.0,
                        py + cell_size / 2.0 + 4.0,
                    );
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
            let _ = ctx.fill_text(
                "Geometric transforms change position, rotation, and scale",
                250.0,
                280.0,
            );
        }
    }
}

/// Lesson 11: Homography demo
fn render_homography_demo() {
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

            // Title
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("14px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Homography: Plane-to-Plane Mapping", 250.0, 25.0);

            // Draw source rectangle (tilted perspective)
            ctx.set_stroke_style(&JsValue::from_str("#00CC88"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(40.0, 80.0);
            ctx.line_to(140.0, 60.0);
            ctx.line_to(160.0, 180.0);
            ctx.line_to(60.0, 200.0);
            ctx.close_path();
            ctx.stroke();

            // Corner points on source
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            for (x, y) in [(40.0, 80.0), (140.0, 60.0), (160.0, 180.0), (60.0, 200.0)] {
                ctx.begin_path();
                let _ = ctx.arc(x, y, 5.0, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("Source", 100.0, 230.0);
            let _ = ctx.fill_text("(photographed)", 100.0, 245.0);

            // Arrow
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(180.0, 140.0);
            ctx.line_to(230.0, 140.0);
            ctx.move_to(220.0, 130.0);
            ctx.line_to(230.0, 140.0);
            ctx.line_to(220.0, 150.0);
            ctx.stroke();

            // H matrix
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("14px JetBrains Mono, monospace");
            let _ = ctx.fill_text("H", 205.0, 120.0);

            // Draw destination rectangle (rectified)
            ctx.set_stroke_style(&JsValue::from_str("#CC6600"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(280.0, 70.0);
            ctx.line_to(400.0, 70.0);
            ctx.line_to(400.0, 190.0);
            ctx.line_to(280.0, 190.0);
            ctx.close_path();
            ctx.stroke();

            // Corner points on destination
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            for (x, y) in [(280.0, 70.0), (400.0, 70.0), (400.0, 190.0), (280.0, 190.0)] {
                ctx.begin_path();
                let _ = ctx.arc(x, y, 5.0, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }

            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("Destination", 340.0, 230.0);
            let _ = ctx.fill_text("(rectified)", 340.0, 245.0);

            // Formula at bottom
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("11px JetBrains Mono, monospace");
            let _ = ctx.fill_text(
                "p' = H · p    (4 point pairs → solve for 3×3 matrix H)",
                250.0,
                280.0,
            );
        }
    }
}

/// Lesson 13: Stereo Vision demo
fn render_stereo_demo() {
    if let Some(canvas) = get_canvas("demo-canvas") {
        canvas.set_width(500);
        canvas.set_height(320);

        if let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .map(|c| c.unchecked_into::<CanvasRenderingContext2d>())
        {
            ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
            ctx.fill_rect(0.0, 0.0, 500.0, 320.0);

            // Title
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("14px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Stereo Vision: Depth from Disparity", 250.0, 25.0);

            // Draw two cameras
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.begin_path();
            ctx.move_to(100.0, 80.0);
            ctx.line_to(80.0, 100.0);
            ctx.line_to(120.0, 100.0);
            ctx.close_path();
            ctx.fill();
            let _ = ctx.fill_text("Left", 100.0, 115.0);

            ctx.begin_path();
            ctx.move_to(300.0, 80.0);
            ctx.line_to(280.0, 100.0);
            ctx.line_to(320.0, 100.0);
            ctx.close_path();
            ctx.fill();
            let _ = ctx.fill_text("Right", 300.0, 115.0);

            // Baseline
            ctx.set_stroke_style(&JsValue::from_str("#666"));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(120.0, 90.0);
            ctx.line_to(280.0, 90.0);
            ctx.stroke();
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("10px Inter, sans-serif");
            let _ = ctx.fill_text("baseline (b)", 200.0, 85.0);

            // 3D point
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.begin_path();
            let _ = ctx.arc(200.0, 200.0, 10.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("P", 200.0, 225.0);

            // Rays from cameras to point
            ctx.set_stroke_style(&JsValue::from_str("#666"));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(100.0, 100.0);
            ctx.line_to(200.0, 200.0);
            ctx.move_to(300.0, 100.0);
            ctx.line_to(200.0, 200.0);
            ctx.stroke();

            // Disparity visualization
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("12px Inter, sans-serif");
            let _ = ctx.fill_text("Left Image", 100.0, 255.0);
            let _ = ctx.fill_text("Right Image", 300.0, 255.0);

            // Left image strip
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.set_line_width(2.0);
            ctx.stroke_rect(40.0, 260.0, 120.0, 20.0);
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.begin_path();
            let _ = ctx.arc(110.0, 270.0, 6.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            // Right image strip
            ctx.stroke_rect(240.0, 260.0, 120.0, 20.0);
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.begin_path();
            let _ = ctx.arc(280.0, 270.0, 6.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();

            // Disparity arrow
            ctx.set_stroke_style(&JsValue::from_str("#CC6600"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(110.0, 290.0);
            ctx.line_to(280.0, 290.0);
            ctx.stroke();
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("disparity (d)", 195.0, 305.0);

            // Formula
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("12px JetBrains Mono, monospace");
            let _ = ctx.fill_text("Z = f·b / d", 430.0, 200.0);
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("10px Inter, sans-serif");
            let _ = ctx.fill_text("depth", 430.0, 215.0);
        }
    }
}

/// Lesson 14: Epipolar Geometry demo
fn render_epipolar_demo() {
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

            // Title
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("14px Inter, sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("Epipolar Geometry", 250.0, 25.0);

            // Left image frame
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.set_line_width(2.0);
            ctx.stroke_rect(30.0, 50.0, 150.0, 120.0);
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text("Left Image", 105.0, 185.0);

            // Point p in left image
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.begin_path();
            let _ = ctx.arc(100.0, 100.0, 6.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            ctx.set_font("10px Inter, sans-serif");
            let _ = ctx.fill_text("p", 110.0, 100.0);

            // Epipole in left image
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            ctx.begin_path();
            let _ = ctx.arc(160.0, 110.0, 5.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            let _ = ctx.fill_text("e", 168.0, 115.0);

            // Right image frame
            ctx.set_stroke_style(&JsValue::from_str("#0099ff"));
            ctx.stroke_rect(320.0, 50.0, 150.0, 120.0);
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            let _ = ctx.fill_text("Right Image", 395.0, 185.0);

            // Epipolar line in right image
            ctx.set_stroke_style(&JsValue::from_str("#00CC88"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(330.0, 80.0);
            ctx.line_to(460.0, 140.0);
            ctx.stroke();

            // Epipole in right image
            ctx.set_fill_style(&JsValue::from_str("#CC6600"));
            ctx.begin_path();
            let _ = ctx.arc(330.0, 110.0, 5.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            ctx.set_font("10px Inter, sans-serif");
            let _ = ctx.fill_text("e'", 315.0, 115.0);

            // Point p' on epipolar line
            ctx.set_fill_style(&JsValue::from_str("#00CC88"));
            ctx.begin_path();
            let _ = ctx.arc(400.0, 112.0, 6.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
            let _ = ctx.fill_text("p'", 410.0, 108.0);

            // Arrow between images
            ctx.set_stroke_style(&JsValue::from_str("#666"));
            ctx.set_line_width(1.5);
            ctx.begin_path();
            ctx.move_to(190.0, 110.0);
            ctx.line_to(310.0, 110.0);
            ctx.move_to(300.0, 105.0);
            ctx.line_to(310.0, 110.0);
            ctx.line_to(300.0, 115.0);
            ctx.stroke();

            // Label
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("11px JetBrains Mono, monospace");
            let _ = ctx.fill_text("l' = F·p", 250.0, 105.0);

            // Explanation
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.set_font("11px Inter, sans-serif");
            let _ = ctx.fill_text(
                "Point p maps to epipolar line l' in other image",
                250.0,
                220.0,
            );
            let _ = ctx.fill_text("Corresponding point p' must lie on l'", 250.0, 240.0);

            // Formula
            ctx.set_fill_style(&JsValue::from_str("#0099ff"));
            ctx.set_font("12px JetBrains Mono, monospace");
            let _ = ctx.fill_text("p'ᵀ · F · p = 0", 250.0, 275.0);
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
                let _ = ctx.fill_text(canvas_id, 150.0, 100.0);
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
