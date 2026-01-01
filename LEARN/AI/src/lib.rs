//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | AI/src/lib.rs
//! PURPOSE: Library crate root module with public API exports
//! MODIFIED: 2025-12-09
//! LAYER: LEARN → AI
//! ═══════════════════════════════════════════════════════════════════════════════
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"AI/ML Learning Platform initialized".into());
    Ok(())
}
