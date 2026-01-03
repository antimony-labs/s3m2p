//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | OPENCV/src/lib.rs
//! PURPOSE: WASM entry point with navigation and camera integration
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════
#![allow(unexpected_cfgs)]

mod camera;
mod demo_runner;
mod image_processing;
mod lessons;
mod render;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use lessons::{DemoType, LESSONS};
use render::LessonRenderer;

/// WASM entry point
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Expose navigation functions to JavaScript
    expose_to_window()?;

    // Render home page
    if let Ok(renderer) = LessonRenderer::new("app") {
        let _ = renderer.render_home();
    }

    web_sys::console::log_1(&"OpenCV Computer Vision platform initialized".into());
    Ok(())
}

/// Expose Rust functions to JavaScript for onclick handlers
fn expose_to_window() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;

    // go_to_lesson(idx)
    let go_to_lesson_fn = Closure::wrap(Box::new(|idx: usize| {
        go_to_lesson(idx);
    }) as Box<dyn Fn(usize)>);

    // go_home()
    let go_home_fn = Closure::wrap(Box::new(|| {
        go_home();
    }) as Box<dyn Fn()>);

    js_sys::Reflect::set(&window, &"go_to_lesson".into(), go_to_lesson_fn.as_ref())?;
    js_sys::Reflect::set(&window, &"go_home".into(), go_home_fn.as_ref())?;

    go_to_lesson_fn.forget();
    go_home_fn.forget();

    Ok(())
}

/// Navigate to a specific lesson
#[wasm_bindgen]
pub fn go_to_lesson(idx: usize) {
    // Stop any running demo
    demo_runner::stop_demo();

    if let Ok(renderer) = LessonRenderer::new("app") {
        if let Some(lesson) = LESSONS.get(idx) {
            let _ = renderer.render_lesson(lesson);

            // Start demo after a short delay to let DOM render
            let lesson_id = lesson.id;
            let demo_type = lesson.demo_type;

            let closure = Closure::once_into_js(move || {
                // Set up demo based on type
                match demo_type {
                    DemoType::Camera => {
                        camera::setup_camera_button();
                        demo_runner::start_camera_demo(lesson_id);
                    }
                    DemoType::Canvas => {
                        demo_runner::start_canvas_demo(lesson_id);
                    }
                    DemoType::SideBySide => {
                        demo_runner::start_sidebyside_demo(lesson_id);
                    }
                    DemoType::Static => {}
                }

                // Scroll to top
                if let Some(window) = web_sys::window() {
                    window.scroll_with_x_and_y(0.0, 0.0);
                }
            });

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    100,
                );
            }
        }
    }
}

/// Navigate back to home page
#[wasm_bindgen]
pub fn go_home() {
    // Scroll to top of page
    if let Some(window) = web_sys::window() {
        let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
    }

    // Stop any running demo
    demo_runner::stop_demo();

    if let Ok(renderer) = LessonRenderer::new("app") {
        let _ = renderer.render_home();
    }

    // Scroll to top
    if let Some(window) = web_sys::window() {
        window.scroll_with_x_and_y(0.0, 0.0);
    }
}
