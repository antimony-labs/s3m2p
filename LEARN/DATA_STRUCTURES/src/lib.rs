//! ===============================================================================
//! FILE: lib.rs | DATA_STRUCTURES/src/lib.rs
//! PURPOSE: Data Structures interactive tutorial platform
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> DATA_STRUCTURES
//! ===============================================================================
#![allow(unexpected_cfgs)]

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod demo_runner;
pub mod lessons;
pub mod render;

use learn_core::demos::problems::PROBLEMS;
use lessons::LESSONS;
use render::LessonRenderer;

/// Total number of lessons
pub const LESSON_COUNT: usize = 10;

/// Active section of the app
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Section {
    Learn,
    Practice,
}

// Thread-local for current state
thread_local! {
    static CURRENT_LESSON: RefCell<usize> = const { RefCell::new(0) };
    static CURRENT_SECTION: RefCell<Section> = const { RefCell::new(Section::Learn) };
    static CURRENT_PROBLEM: RefCell<Option<usize>> = const { RefCell::new(None) };
}

/// Expose functions to window for onclick handlers
fn expose_to_window() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;

    // Learn section navigation
    let go_to_lesson_fn = Closure::wrap(Box::new(|idx: usize| {
        go_to_lesson(idx);
    }) as Box<dyn Fn(usize)>);

    let go_home_fn = Closure::wrap(Box::new(|| {
        go_home();
    }) as Box<dyn Fn()>);

    // Section switching
    let go_to_section_fn = Closure::wrap(Box::new(|section: usize| match section {
        0 => switch_section(Section::Learn),
        1 => switch_section(Section::Practice),
        _ => {}
    }) as Box<dyn Fn(usize)>);

    // Practice section navigation
    let go_to_problem_fn = Closure::wrap(Box::new(|idx: usize| {
        go_to_problem(idx);
    }) as Box<dyn Fn(usize)>);

    js_sys::Reflect::set(&window, &"go_to_lesson".into(), go_to_lesson_fn.as_ref())?;
    js_sys::Reflect::set(&window, &"go_home".into(), go_home_fn.as_ref())?;
    js_sys::Reflect::set(&window, &"go_to_section".into(), go_to_section_fn.as_ref())?;
    js_sys::Reflect::set(&window, &"go_to_problem".into(), go_to_problem_fn.as_ref())?;

    go_to_lesson_fn.forget();
    go_home_fn.forget();
    go_to_section_fn.forget();
    go_to_problem_fn.forget();

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

    web_sys::console::log_1(&"Data Structures platform initialized".into());
    Ok(())
}

/// Navigate to lesson (called from JS)
#[wasm_bindgen]
pub fn go_to_lesson(idx: usize) {
    // Scroll to top of page
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }

    // Stop any running demo
    demo_runner::stop_demo();

    // Store current lesson
    CURRENT_LESSON.with(|l| *l.borrow_mut() = idx);

    if let Ok(renderer) = LessonRenderer::new("app") {
        if let Some(lesson) = LESSONS.get(idx) {
            let _ = renderer.render_lesson(lesson, LESSON_COUNT);

            // Start the appropriate demo based on lesson id
            let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
                let result = demo_runner::start_demo_for_lesson(idx, "lesson-canvas", 42);
                if let Err(e) = result {
                    web_sys::console::error_1(&e);
                }
                // Set up event listeners after demo starts
                setup_demo_controls(idx);
            });
            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    50,
                );
        }
    }
}

/// Set up event listeners for demo controls
fn setup_demo_controls(lesson_idx: usize) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    // Helper to get slider value
    fn get_slider_value(doc: &web_sys::Document, id: &str) -> i32 {
        doc.get_element_by_id(id)
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|input| input.value().parse::<i32>().unwrap_or(0))
            .unwrap_or(0)
    }

    // Helper to add click listener
    fn add_click_listener(doc: &web_sys::Document, id: &str, callback: impl Fn() + 'static) {
        if let Some(el) = doc.get_element_by_id(id) {
            let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);
            let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }

    // Update slider displays
    fn setup_slider_display(doc: &web_sys::Document, slider_id: &str, display_id: &str) {
        if let Some(slider) = doc.get_element_by_id(slider_id) {
            let display_id = display_id.to_string();
            let doc_clone = doc.clone();
            let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                        if let Some(display) = doc_clone.get_element_by_id(&display_id) {
                            display.set_text_content(Some(&input.value()));
                        }
                    }
                }
            }) as Box<dyn Fn(web_sys::Event)>);
            let _ =
                slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    }

    // Set up common slider displays
    setup_slider_display(&document, "value-slider", "value-display");
    setup_slider_display(&document, "index-slider", "index-display");
    setup_slider_display(&document, "vertex-slider", "vertex-display");

    // Reset button (common to all)
    {
        let closure = Closure::wrap(Box::new(move || {
            demo_runner::ds_demo_reset(42);
        }) as Box<dyn Fn()>);
        if let Some(el) = document.get_element_by_id("reset-btn") {
            let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        }
        closure.forget();
    }

    match lesson_idx {
        0 => {
            // Arrays
            let doc = document.clone();
            add_click_listener(&document, "access-btn", move || {
                let idx = get_slider_value(&doc, "index-slider");
                demo_runner::ds_demo_action("access", idx);
            });

            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let idx = get_slider_value(&doc, "index-slider") as usize;
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_array_insert(idx, val);
            });

            let doc = document.clone();
            add_click_listener(&document, "delete-btn", move || {
                let idx = get_slider_value(&doc, "index-slider");
                demo_runner::ds_demo_action("delete", idx);
            });
        }
        1 => {
            // Linked Lists
            let doc = document.clone();
            add_click_listener(&document, "insert-head-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert_head", val);
            });

            let doc = document.clone();
            add_click_listener(&document, "insert-tail-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert_tail", val);
            });

            add_click_listener(&document, "delete-btn", move || {
                demo_runner::ds_demo_action("delete_head", 0);
            });

            let doc = document.clone();
            add_click_listener(&document, "search-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("search", val);
            });
        }
        2 => {
            // Stacks
            let doc = document.clone();
            add_click_listener(&document, "push-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("push", val);
            });

            add_click_listener(&document, "pop-btn", move || {
                demo_runner::ds_demo_action("pop", 0);
            });

            add_click_listener(&document, "peek-btn", move || {
                demo_runner::ds_demo_action("peek", 0);
            });
        }
        3 => {
            // Queues
            let doc = document.clone();
            add_click_listener(&document, "enqueue-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("enqueue", val);
            });

            add_click_listener(&document, "dequeue-btn", move || {
                demo_runner::ds_demo_action("dequeue", 0);
            });

            add_click_listener(&document, "peek-btn", move || {
                demo_runner::ds_demo_action("peek", 0);
            });
        }
        4 => {
            // Binary Trees
            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert", val);
            });

            add_click_listener(&document, "preorder-btn", move || {
                demo_runner::ds_demo_action("preorder", 0);
            });

            add_click_listener(&document, "inorder-btn", move || {
                demo_runner::ds_demo_action("inorder", 0);
            });

            add_click_listener(&document, "postorder-btn", move || {
                demo_runner::ds_demo_action("postorder", 0);
            });

            add_click_listener(&document, "levelorder-btn", move || {
                demo_runner::ds_demo_action("levelorder", 0);
            });
        }
        5 => {
            // BST
            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert", val);
            });

            let doc = document.clone();
            add_click_listener(&document, "search-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("search", val);
            });
        }
        6 => {
            // Heaps
            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert", val);
            });

            add_click_listener(&document, "extract-btn", move || {
                demo_runner::ds_demo_action("extract", 0);
            });

            add_click_listener(&document, "toggle-type-btn", move || {
                demo_runner::ds_demo_action("toggle_type", 0);
            });
        }
        7 => {
            // Hash Tables
            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert", val);
            });

            let doc = document.clone();
            add_click_listener(&document, "search-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("search", val);
            });
        }
        8 => {
            // Graphs
            add_click_listener(&document, "bfs-btn", move || {
                demo_runner::ds_demo_action("bfs", 0);
            });

            add_click_listener(&document, "dfs-btn", move || {
                demo_runner::ds_demo_action("dfs", 0);
            });
        }
        9 => {
            // Balanced Trees (AVL)
            let doc = document.clone();
            add_click_listener(&document, "insert-btn", move || {
                let val = get_slider_value(&doc, "value-slider");
                demo_runner::ds_demo_action("insert", val);
            });
        }
        _ => {}
    }
}

/// Go back to home
#[wasm_bindgen]
pub fn go_home() {
    // Scroll to top of page
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }

    demo_runner::stop_demo();
    CURRENT_PROBLEM.with(|p| *p.borrow_mut() = None);

    let current_section = CURRENT_SECTION.with(|s| *s.borrow());

    if let Ok(renderer) = LessonRenderer::new("app") {
        match current_section {
            Section::Learn => {
                let _ = renderer.render_home(LESSONS);
            }
            Section::Practice => {
                let _ = renderer.render_practice_home(PROBLEMS);
            }
        }
    }
}

/// Switch between Learn and Practice sections
pub fn switch_section(section: Section) {
    // Scroll to top of page
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }

    demo_runner::stop_demo();
    CURRENT_SECTION.with(|s| *s.borrow_mut() = section);
    CURRENT_PROBLEM.with(|p| *p.borrow_mut() = None);

    if let Ok(renderer) = LessonRenderer::new("app") {
        match section {
            Section::Learn => {
                let _ = renderer.render_home(LESSONS);
            }
            Section::Practice => {
                let _ = renderer.render_practice_home(PROBLEMS);
            }
        }
    }
}

/// Navigate to a specific problem
#[wasm_bindgen]
pub fn go_to_problem(idx: usize) {
    // Scroll to top of page
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }

    demo_runner::stop_demo();
    CURRENT_PROBLEM.with(|p| *p.borrow_mut() = Some(idx));

    if let Ok(renderer) = LessonRenderer::new("app") {
        if let Some(problem) = PROBLEMS.get(idx) {
            let _ = renderer.render_problem(problem, PROBLEMS.len());

            // Start the problem demo after a short delay (to let DOM render)
            let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
                let result = demo_runner::start_problem_demo(idx, "problem-canvas");
                if let Err(e) = result {
                    web_sys::console::error_1(&e);
                }
                // Set up problem controls
                setup_problem_controls();
            });
            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    50,
                );
        }
    }
}

/// Set up event listeners for problem controls
fn setup_problem_controls() {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    // Step button
    if let Some(el) = document.get_element_by_id("step-btn") {
        let closure = Closure::wrap(Box::new(|| {
            demo_runner::problem_step();
        }) as Box<dyn Fn()>);
        let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Reset button
    if let Some(el) = document.get_element_by_id("reset-btn") {
        let closure = Closure::wrap(Box::new(|| {
            demo_runner::problem_reset();
        }) as Box<dyn Fn()>);
        let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Play button - auto-step every 500ms
    if let Some(el) = document.get_element_by_id("play-btn") {
        let closure = Closure::wrap(Box::new(|| {
            // Toggle auto-play
            auto_play_toggle();
        }) as Box<dyn Fn()>);
        let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

thread_local! {
    static AUTO_PLAY_INTERVAL: RefCell<Option<i32>> = const { RefCell::new(None) };
}

fn auto_play_toggle() {
    AUTO_PLAY_INTERVAL.with(|interval| {
        let mut interval_ref = interval.borrow_mut();
        if let Some(id) = interval_ref.take() {
            // Stop auto-play
            if let Some(window) = web_sys::window() {
                window.clear_interval_with_handle(id);
            }
            // Update button text
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(btn) = document.get_element_by_id("play-btn") {
                    btn.set_text_content(Some("Play"));
                }
            }
        } else {
            // Start auto-play
            let callback = Closure::wrap(Box::new(|| {
                demo_runner::problem_step();
            }) as Box<dyn Fn()>);

            if let Some(window) = web_sys::window() {
                let id = window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        500,
                    )
                    .ok();
                *interval_ref = id;
            }
            callback.forget();

            // Update button text
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(btn) = document.get_element_by_id("play-btn") {
                    btn.set_text_content(Some("Pause"));
                }
            }
        }
    });
}
