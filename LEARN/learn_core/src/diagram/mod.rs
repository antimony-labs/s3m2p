//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | learn_core/src/diagram/mod.rs
//! PURPOSE: Custom diagram rendering system for technical visualizations
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → diagram
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod filesystem_tree;
pub mod permission_matrix;

pub use filesystem_tree::FilesystemTree;
pub use permission_matrix::PermissionMatrix;

/// Text alignment options
#[derive(Clone, Copy, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Core diagram trait - pure data structure, no rendering logic
pub trait Diagram {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn render(&self, renderer: &mut dyn DiagramRenderer);
}

/// Renderer trait - implemented by Canvas, SVG, or other backends
pub trait DiagramRenderer {
    fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: Option<&str>);
    fn draw_circle(&mut self, x: f64, y: f64, r: f64, fill: &str, stroke: Option<&str>);
    fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64);
    fn draw_text(&mut self, text: &str, x: f64, y: f64, font: &str, color: &str, align: TextAlign);
    fn draw_arrow(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64);
}
