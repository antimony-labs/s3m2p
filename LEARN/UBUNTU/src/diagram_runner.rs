//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: diagram_runner.rs | UBUNTU/src/diagram_runner.rs
//! PURPOSE: Diagram visualization runner for lessons
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → UBUNTU
//! ═══════════════════════════════════════════════════════════════════════════════

use wasm_bindgen::prelude::*;
use learn_web::{Canvas, render_diagram};
use learn_core::{FilesystemTree, PermissionMatrix};
use learn_core::demos::FsPermissionsDemo;

/// Start diagram visualization for a lesson
pub fn start_diagram(lesson_id: usize, demo: &FsPermissionsDemo) -> Result<(), JsValue> {
    let canvas = Canvas::new("diagram-canvas")?;

    match lesson_id {
        7 => {
            // Filesystem tree for navigation lesson
            let tree = FilesystemTree::from_demo(demo, 3); // Max depth 3
            render_diagram(&canvas, &tree);
        }

        8 => {
            // Permission matrix for permissions lesson
            let matrix = PermissionMatrix::from_demo(demo);
            render_diagram(&canvas, &matrix);
        }

        _ => {
            // No diagram for this lesson
            canvas.clear("#0a0a12");
        }
    }

    Ok(())
}

/// Update diagram when terminal state changes
pub fn update_diagram(lesson_id: usize, demo: &FsPermissionsDemo) -> Result<(), JsValue> {
    start_diagram(lesson_id, demo)
}
