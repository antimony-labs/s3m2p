//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | TOOLS/POWER_CIRCUITS/src/lib.rs
//! PURPOSE: Power circuit designer WASM application for supply and converter design
//! MODIFIED: 2025-12-09
//! LAYER: TOOLS → POWER_CIRCUITS
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Power Circuits Designer initialized".into());
    Ok(())
}
