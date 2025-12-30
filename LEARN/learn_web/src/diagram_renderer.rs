//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: diagram_renderer.rs | learn_web/src/diagram_renderer.rs
//! PURPOSE: Canvas-based diagram renderer implementation
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_web
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::Canvas;
use learn_core::diagram::{Diagram, DiagramRenderer, TextAlign};
use wasm_bindgen::JsValue;

pub struct CanvasDiagramRenderer<'a> {
    canvas: &'a Canvas,
}

impl<'a> CanvasDiagramRenderer<'a> {
    pub fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }
}

impl<'a> DiagramRenderer for CanvasDiagramRenderer<'a> {
    fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: Option<&str>) {
        self.canvas.fill_rect(x, y, w, h, fill);
        if let Some(color) = stroke {
            self.canvas.stroke_rect(x, y, w, h, color, 1.0);
        }
    }

    fn draw_circle(&mut self, x: f64, y: f64, r: f64, fill: &str, stroke: Option<&str>) {
        self.canvas.fill_circle(x, y, r, fill);
        if let Some(color) = stroke {
            self.canvas.stroke_circle(x, y, r, color, 1.0);
        }
    }

    fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64) {
        self.canvas.line(x1, y1, x2, y2, color, width);
    }

    fn draw_text(&mut self, text: &str, x: f64, y: f64, font: &str, color: &str, align: TextAlign) {
        let ctx = self.canvas.ctx();
        ctx.set_text_align(match align {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
        });
        self.canvas.text(text, x, y, color, font);
    }

    fn draw_arrow(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64) {
        // Draw line
        self.canvas.line(x1, y1, x2, y2, color, width);

        // Draw arrowhead (triangle at x2, y2)
        let angle = (y2 - y1).atan2(x2 - x1);
        let size = 10.0;

        let ctx = self.canvas.ctx();
        ctx.save();
        ctx.set_fill_style(&JsValue::from_str(color));
        ctx.begin_path();
        ctx.move_to(x2, y2);
        ctx.line_to(
            x2 - size * (angle - 0.5).cos(),
            y2 - size * (angle - 0.5).sin(),
        );
        ctx.line_to(
            x2 - size * (angle + 0.5).cos(),
            y2 - size * (angle + 0.5).sin(),
        );
        ctx.close_path();
        ctx.fill();
        ctx.restore();
    }
}

/// Render a diagram to canvas
pub fn render_diagram(canvas: &Canvas, diagram: &dyn Diagram) {
    // Clear canvas
    canvas.clear("#0a0a12");

    // Create renderer and render diagram
    let mut renderer = CanvasDiagramRenderer::new(canvas);
    diagram.render(&mut renderer);
}
