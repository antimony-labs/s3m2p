//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: demo_runner.rs | SWARM_ROBOTICS/src/demo_runner.rs
//! PURPOSE: Demo runners for all Swarm Robotics lessons
//! MODIFIED: 2025-01-XX
//! LAYER: LEARN → SWARM_ROBOTICS
//! ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use learn_core::{Demo, demos::BoidsDemo};
use learn_web::{AnimationLoop, Canvas};

// Thread-local state for the currently running demos
thread_local! {
    static BOIDS_DEMO: RefCell<Option<BoidsDemoRunner>> = RefCell::new(None);
    static CURRENT_DEMO: RefCell<Option<SwarmDemoRunner>> = RefCell::new(None);
}

/// Dispatch to the appropriate demo based on lesson index
pub fn start_demo_for_lesson(_lesson_idx: usize, canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    // For now, use BoidsDemo for lessons 0, 1, 3
    // Other lessons will be wired as demos are implemented
    BoidsDemoRunner::start(canvas_id, seed)
}

/// Stop the current demo
pub fn stop_demo() {
    BOIDS_DEMO.with(|d| {
        *d.borrow_mut() = None;
    });
    CURRENT_DEMO.with(|d| {
        *d.borrow_mut() = None;
    });
}

/// Trigger an immediate render (called from JS for instant feedback)
#[wasm_bindgen]
pub fn trigger_render() {
    BOIDS_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.render();
        }
    });
}

/// Boids demo runner
pub struct BoidsDemoRunner {
    demo: BoidsDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl BoidsDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = BoidsDemo::default();
        demo.reset(seed);

        let runner = BoidsDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        BOIDS_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;

        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            BOIDS_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        BOIDS_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        // Wire parameter sliders from BoidsDemo::params()
        for param in BoidsDemo::params() {
            let param_name = param.name.to_string();
            let slider_id = format!("{}-slider", param_name);
            let value_id = format!("{}-value", param_name);
            
            if let Some(slider) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(&slider_id))
                .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
            {
                let param_name_clone = param_name.clone();
                let value_id_clone = value_id.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Some(slider) = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.get_element_by_id(&format!("{}-slider", param_name_clone)))
                        .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
                    {
                        if let Ok(value) = slider.value().parse::<f32>() {
                            let param_updated = BOIDS_DEMO.with(|d| {
                                if let Some(runner) = d.borrow_mut().as_mut() {
                                    runner.demo.set_param(&param_name_clone, value)
                                } else {
                                    false
                                }
                            });
                            
                            if param_updated {
                                // Update value display
                                if let Some(value_el) = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.get_element_by_id(&value_id_clone))
                                {
                                    value_el.set_text_content(Some(&format!("{:.2}", value)));
                                }
                            } else {
                                web_sys::console::warn_1(&format!("Failed to update parameter: {}", param_name_clone).into());
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
                closure.forget();
            } else {
                web_sys::console::warn_1(&format!("Slider not found: {}", slider_id).into());
            }
        }

        // Reset button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                BOIDS_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.demo.reset(seed);
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Pause button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pause-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                BOIDS_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        runner.paused = !runner.paused;
                        if let Some(btn) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id("pause-btn"))
                        {
                            btn.set_text_content(Some(if runner.paused {
                                "▶ Play"
                            } else {
                                "⏸ Pause"
                            }));
                        }
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }

    fn render(&mut self) {
        let ctx = self.canvas.ctx();
        let w = self.canvas.width();
        let h = self.canvas.height();

        // Clear background
        self.canvas.clear("#0a0a12");

        // Draw world bounds
        let margin = 20.0;
        let world_w = w - 2.0 * margin;
        let world_h = h - 2.0 * margin - 100.0; // Leave room for HUD

        // Coordinate transform: [0,1] -> canvas
        let to_x = |x: f32| -> f64 { margin + (x as f64) * world_w };
        let to_y = |y: f32| -> f64 { margin + (1.0 - y as f64) * world_h };

        // Draw obstacles
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 100, 100, 0.5)"));
        for obs in &self.demo.world.obstacles {
            let x = to_x(obs.center.x);
            let y = to_y(obs.center.y);
            let r = (obs.radius * world_w as f32) as f64;
            ctx.begin_path();
            let _ = ctx.arc(x, y, r, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // Draw agents
        for agent in &self.demo.world.agents {
            let x = to_x(agent.pos.x);
            let y = to_y(agent.pos.y);
            let r = 4.0;

            // Draw velocity vector
            ctx.set_stroke_style(&JsValue::from_str("rgba(100, 255, 218, 0.5)"));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(x, y);
            let vx = agent.vel.x * (world_w as f32) * 0.1;
            let vy = -agent.vel.y * (world_h as f32) * 0.1; // Flip Y
            ctx.line_to(x + vx as f64, y + vy as f64);
            ctx.stroke();

            // Draw agent circle
            ctx.set_fill_style(&JsValue::from_str("#64ffda"));
            ctx.begin_path();
            let _ = ctx.arc(x, y, r, 0.0, std::f64::consts::TAU);
            ctx.fill();

            // Draw heading indicator
            ctx.set_stroke_style(&JsValue::from_str("#ffffff"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(x, y);
            let hx = agent.heading.cos() * (r as f32) * 1.5;
            let hy = -agent.heading.sin() * (r as f32) * 1.5; // Flip Y
            ctx.line_to(x + hx as f64, y + hy as f64);
            ctx.stroke();
        }

        // Draw HUD
        ctx.set_fill_style(&JsValue::from_str("#888"));
        ctx.set_font("12px 'Inter', sans-serif");
        let collisions = self.demo.world.compute_collisions(0.02);
        let min_sep = self.demo.world.compute_min_separation();
        let components = self.demo.world.compute_components(self.demo.neighbor_radius);
        let hud_text = format!(
            "Collisions: {} | Min Separation: {:.3} | Components: {} | Agents: {}",
            collisions, min_sep, components, self.demo.world.agents.len()
        );
        let _ = ctx.fill_text(&hud_text, margin, h - 20.0);
    }
}

/// Swarm demo runner (placeholder - will be replaced with actual implementations)
pub struct SwarmDemoRunner {
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
    seed: u64,
}

impl SwarmDemoRunner {
    /// Start the demo
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;

        let runner = SwarmDemoRunner {
            canvas,
            animation: None,
            paused: false,
            seed,
        };

        CURRENT_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        // Start animation loop
        Self::start_animation()?;

        // Wire controls
        Self::wire_controls()?;

        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            CURRENT_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        // Demo step will be implemented here
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        CURRENT_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        // Reset button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                CURRENT_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.seed = seed;
                        // Reset demo here
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Pause button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pause-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                CURRENT_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        runner.paused = !runner.paused;
                        if let Some(btn) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id("pause-btn"))
                        {
                            btn.set_text_content(Some(if runner.paused {
                                "▶ Play"
                            } else {
                                "⏸ Pause"
                            }));
                        }
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // New seed button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("new-seed-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                CURRENT_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.seed = seed;
                        // Reset with new seed
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }

    fn render(&mut self) {
        let ctx = self.canvas.ctx();
        let w = self.canvas.width();
        let h = self.canvas.height();

        // Clear background
        self.canvas.clear("#0a0a12");

        // Placeholder removed - actual rendering happens in BoidsDemoRunner

        // Draw seed info
        ctx.set_fill_style(&JsValue::from_str("#888"));
        ctx.set_font("12px 'Inter', sans-serif");
        let seed_text = format!("Seed: {}", self.seed);
        let _ = ctx.fill_text(&seed_text, 20.0, h - 20.0);
    }
}

