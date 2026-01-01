//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | GIT/src/lib.rs
//! PURPOSE: Git version control learning platform
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → GIT
//! ═══════════════════════════════════════════════════════════════════════════════
#![allow(unexpected_cfgs)]

use wasm_bindgen::prelude::*;

pub mod lessons;
pub mod render;

use lessons::LESSONS;
use render::LessonRenderer;

/// Expose functions to window for onclick handlers
fn expose_to_window() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;

    let go_to_lesson_fn = Closure::wrap(Box::new(|idx: usize| {
        go_to_lesson(idx);
    }) as Box<dyn Fn(usize)>);

    let go_home_fn = Closure::wrap(Box::new(|| {
        go_home();
    }) as Box<dyn Fn()>);

    js_sys::Reflect::set(&window, &"go_to_lesson".into(), go_to_lesson_fn.as_ref())?;
    js_sys::Reflect::set(&window, &"go_home".into(), go_home_fn.as_ref())?;

    go_to_lesson_fn.forget();
    go_home_fn.forget();

    Ok(())
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    expose_to_window()?;

    // Render home page
    if let Ok(renderer) = LessonRenderer::new("app") {
        let _ = renderer.render_home(LESSONS);
    }

    web_sys::console::log_1(&"Git platform initialized".into());
    Ok(())
}

/// Navigate to lesson
#[wasm_bindgen]
pub fn go_to_lesson(idx: usize) {
    if let Ok(renderer) = LessonRenderer::new("app") {
        if let Some(lesson) = LESSONS.get(idx) {
            let _ = renderer.render_lesson(lesson);

            // Scroll to top AFTER content renders
            let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
                if let Some(window) = web_sys::window() {
                    let _ = window.scroll_with_x_and_y(0.0, 0.0);
                }
            });
            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    100,
                );
        }
    }
}

/// Go back to home
#[wasm_bindgen]
pub fn go_home() {
    if let Ok(renderer) = LessonRenderer::new("app") {
        let _ = renderer.render_home(LESSONS);
    }
}
