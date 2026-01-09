//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | SIMULATIONS/HANDTRACK/src/lib.rs
//! PURPOSE: Real-time hand tracking simulation using skin color detection
//! MODIFIED: 2026-01-02
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, ImageData};

#[wasm_bindgen]
pub struct HandTracker {
    min_r: f32,
    min_g: f32,
    min_b: f32,
    min_brightness: f32,
    min_pixels: usize,
}

#[derive(Serialize, Deserialize)]
pub struct HandData {
    pub detected: bool,
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
    pub area: usize,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

#[wasm_bindgen]
impl HandTracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        Self {
            min_r: 80.0,
            min_g: 30.0,
            min_b: 15.0,
            min_brightness: 100.0,
            min_pixels: 500,
        }
    }

    #[wasm_bindgen]
    pub fn process_frame(
        &self,
        video: &HtmlVideoElement,
        canvas: &HtmlCanvasElement,
        ctx: &CanvasRenderingContext2d,
    ) -> JsValue {
        let width = video.video_width();
        let height = video.video_height();

        if width == 0 || height == 0 {
            return serde_wasm_bindgen::to_value(&HandData {
                detected: false,
                center_x: 0.0,
                center_y: 0.0,
                width: 0.0,
                height: 0.0,
                area: 0,
                velocity_x: 0.0,
                velocity_y: 0.0,
            })
            .unwrap();
        }

        canvas.set_width(width);
        canvas.set_height(height);

        // Draw video frame to canvas
        let _ = ctx.draw_image_with_html_video_element(video, 0.0, 0.0);

        // Get image data
        let image_data = match ctx.get_image_data(0.0, 0.0, width as f64, height as f64) {
            Ok(data) => data,
            Err(_) => {
                return serde_wasm_bindgen::to_value(&HandData {
                    detected: false,
                    center_x: 0.0,
                    center_y: 0.0,
                    width: 0.0,
                    height: 0.0,
                    area: 0,
                    velocity_x: 0.0,
                    velocity_y: 0.0,
                })
                .unwrap();
            }
        };

        // Process frame
        let hand_data = self.detect_hand(&image_data, width, height);
        self.draw_overlay(&image_data, &hand_data, width, height, ctx);

        serde_wasm_bindgen::to_value(&hand_data).unwrap()
    }

    fn detect_hand(&self, image_data: &ImageData, width: u32, height: u32) -> HandData {
        let data = image_data.data();
        let width_i = width as i32;
        let height_i = height as i32;

        let mut skin_pixels = Vec::new();
        let mut min_x = width_i;
        let mut max_x = 0;
        let mut min_y = height_i;
        let mut max_y = 0;
        let mut cx_sum = 0i64;
        let mut cy_sum = 0i64;
        let mut count = 0i64;

        for y in 0..height_i {
            for x in 0..width_i {
                let idx = ((y * width_i + x) * 4) as usize;
                let r = data[idx] as f32;
                let g = data[idx + 1] as f32;
                let b = data[idx + 2] as f32;

                // Skin detection
                let is_skin = r > self.min_r
                    && g > self.min_g
                    && b > self.min_b
                    && (r - g).abs() > 10.0
                    && r > g
                    && r > b
                    && (r + g + b) > self.min_brightness;

                if is_skin {
                    skin_pixels.push((x, y));
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    cx_sum += x as i64;
                    cy_sum += y as i64;
                    count += 1;
                }
            }
        }

        if count as usize >= self.min_pixels {
            let cx = (cx_sum / count) as f32;
            let cy = (cy_sum / count) as f32;
            let w = (max_x - min_x) as f32;
            let h = (max_y - min_y) as f32;

            HandData {
                detected: true,
                center_x: cx,
                center_y: cy,
                width: w,
                height: h,
                area: count as usize,
                velocity_x: 0.0, // Could track velocity over frames
                velocity_y: 0.0,
            }
        } else {
            HandData {
                detected: false,
                center_x: 0.0,
                center_y: 0.0,
                width: 0.0,
                height: 0.0,
                area: 0,
                velocity_x: 0.0,
                velocity_y: 0.0,
            }
        }
    }

    fn draw_overlay(
        &self,
        image_data: &ImageData,
        hand_data: &HandData,
        width: u32,
        height: u32,
        ctx: &CanvasRenderingContext2d,
    ) {
        let data = image_data.data();
        let width_i = width as i32;
        let height_i = height as i32;

        // Create output with skin highlighting
        let mut output = vec![0u8; data.len()];

        for y in 0..height_i {
            for x in 0..width_i {
                let idx = ((y * width_i + x) * 4) as usize;

                let r = data[idx] as f32;
                let g = data[idx + 1] as f32;
                let b = data[idx + 2] as f32;

                // Check if skin
                let is_skin = r > self.min_r
                    && g > self.min_g
                    && b > self.min_b
                    && (r - g).abs() > 10.0
                    && r > g
                    && r > b
                    && (r + g + b) > self.min_brightness;

                if is_skin {
                    // Yellow tint for skin
                    output[idx] = ((r * 0.7) + (255.0 * 0.3)) as u8;
                    output[idx + 1] = ((g * 0.7) + (200.0 * 0.3)) as u8;
                    output[idx + 2] = (b * 0.7) as u8;
                    output[idx + 3] = 255;
                } else {
                    output[idx] = data[idx];
                    output[idx + 1] = data[idx + 1];
                    output[idx + 2] = data[idx + 2];
                    output[idx + 3] = data[idx + 3];
                }
            }
        }

        // Put processed image back
        if let Ok(new_image_data) = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&output),
            width,
            height,
        ) {
            let _ = ctx.put_image_data(&new_image_data, 0.0, 0.0);
        }

        // Draw bounding box and crosshair if hand detected
        if hand_data.detected {
            let cx = hand_data.center_x;
            let cy = hand_data.center_y;
            let w = hand_data.width;
            let h = hand_data.height;

            // Draw bounding box (green)
            ctx.set_stroke_style(&JsValue::from_str("#00ff00"));
            ctx.set_line_width(3.0);
            ctx.stroke_rect(
                (cx - w / 2.0) as f64,
                (cy - h / 2.0) as f64,
                w as f64,
                h as f64,
            );

            // Draw crosshair (magenta)
            ctx.set_stroke_style(&JsValue::from_str("#ff00ff"));
            ctx.set_line_width(2.0);
            let size = 15.0;
            ctx.begin_path();
            ctx.move_to((cx - size) as f64, cy as f64);
            ctx.line_to((cx + size) as f64, cy as f64);
            ctx.move_to(cx as f64, (cy - size) as f64);
            ctx.line_to(cx as f64, (cy + size) as f64);
            ctx.stroke();

            // Draw center circle
            ctx.begin_path();
            let _ = ctx.arc(cx as f64, cy as f64, 5.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }
    }

    #[wasm_bindgen]
    pub fn set_sensitivity(&mut self, min_r: f32, min_g: f32, min_b: f32, min_brightness: f32) {
        self.min_r = min_r;
        self.min_g = min_g;
        self.min_b = min_b;
        self.min_brightness = min_brightness;
    }

    #[wasm_bindgen]
    pub fn set_min_pixels(&mut self, min_pixels: usize) {
        self.min_pixels = min_pixels;
    }
}
