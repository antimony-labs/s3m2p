//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: demo_runner.rs | UBUNTU/src/demo_runner.rs
//! PURPOSE: Filesystem Permissions demo runner with terminal UI
//! MODIFIED: 2025-12-11
//! LAYER: LEARN → UBUNTU
//! ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};

use crate::terminal_configs::{
    DefaultLessonConfig, Lesson10Config, Lesson11Config, Lesson5Config, Lesson7Config,
    Lesson8Config, Lesson9Config,
};
use learn_core::demos::FsPermissionsDemo;
use learn_core::{Demo, TerminalConfig};

// Thread-local state for the currently running demo
thread_local! {
    static CURRENT_DEMO: RefCell<Option<FsPermissionsDemoRunner>> = const { RefCell::new(None) };
}

/// Filesystem Permissions demo runner
pub struct FsPermissionsDemoRunner {
    demo: FsPermissionsDemo,
}

impl FsPermissionsDemoRunner {
    /// Start the Filesystem Permissions demo with lesson-specific config
    pub fn start(lesson_id: usize) -> Result<(), JsValue> {
        let config = create_terminal_config(lesson_id);
        let mut demo = FsPermissionsDemo::with_config(config);
        demo.reset(42);

        let runner = FsPermissionsDemoRunner { demo };

        CURRENT_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        // Render initial state
        Self::render_output()?;

        // Wire input handler
        Self::wire_input()?;

        Ok(())
    }

    fn wire_input() -> Result<(), JsValue> {
        if let Ok(input) = get_input("terminal-input") {
            let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Enter" {
                    if let Ok(input) = get_input("terminal-input") {
                        let cmd = input.value();
                        input.set_value("");

                        CURRENT_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                // Add command to output
                                let prompt = runner.demo.prompt.clone();
                                Self::append_line(&format!("{}{}", prompt, cmd), false);

                                // Execute command
                                let result = runner.demo.execute(&cmd);

                                // Show output
                                if !result.output.is_empty() {
                                    for line in result.output.lines() {
                                        Self::append_line(line, !result.success);
                                    }
                                }

                                // Update prompt
                                Self::update_prompt(&runner.demo.prompt);

                                // Scroll to bottom
                                Self::scroll_to_bottom();
                            }
                        });
                    }
                }
            }) as Box<dyn FnMut(_)>);
            input.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }

    fn render_output() -> Result<(), JsValue> {
        CURRENT_DEMO.with(|d| {
            if let Some(runner) = d.borrow().as_ref() {
                // Update prompt
                Self::update_prompt(&runner.demo.prompt);

                // Clear and render history
                if let Some(output_el) = get_element("terminal-output") {
                    output_el.set_inner_html("");

                    // Get welcome message from config
                    let welcome = runner.demo.get_welcome_message();

                    // Add welcome message (split by lines)
                    for line in welcome.lines() {
                        Self::append_line(line, false);
                    }
                    Self::append_line("", false);
                }
            }
        });

        Ok(())
    }

    fn append_line(text: &str, is_error: bool) {
        if let Some(output_el) = get_element("terminal-output") {
            let document = web_sys::window()
                .and_then(|w| w.document())
                .expect("document");

            if let Ok(line) = document.create_element("div") {
                line.set_class_name(if is_error {
                    "terminal-line error"
                } else {
                    "terminal-line"
                });
                line.set_text_content(Some(text));
                let _ = output_el.append_child(&line);
            }
        }
    }

    fn update_prompt(prompt: &str) {
        if let Some(prompt_el) = get_element("terminal-prompt") {
            prompt_el.set_text_content(Some(prompt));
        }
    }

    fn scroll_to_bottom() {
        if let Some(output_el) = get_element("terminal-output") {
            let scroll_height = output_el.scroll_height();
            output_el.set_scroll_top(scroll_height);
        }
    }
}

/// Stop the current demo
pub fn stop_demo() {
    CURRENT_DEMO.with(|d| {
        *d.borrow_mut() = None;
    });
}

/// Create terminal config for specific lesson
fn create_terminal_config(lesson_id: usize) -> Box<dyn TerminalConfig> {
    match lesson_id {
        5 => Box::new(Lesson5Config),
        7 => Box::new(Lesson7Config),
        8 => Box::new(Lesson8Config),
        9 => Box::new(Lesson9Config),
        10 => Box::new(Lesson10Config),
        11 => Box::new(Lesson11Config),
        // Lessons 12-20: Use minimal config for non-terminal or advanced lessons
        _ => Box::new(DefaultLessonConfig),
    }
}

fn get_element(id: &str) -> Option<Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

fn get_input(id: &str) -> Result<HtmlInputElement, JsValue> {
    web_sys::window()
        .ok_or("No window")?
        .document()
        .ok_or("No document")?
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element '{}' not found", id)))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("Not an input element"))
}
