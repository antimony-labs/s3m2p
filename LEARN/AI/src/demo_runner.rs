//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: demo_runner.rs | AI/src/demo_runner.rs
//! PURPOSE: Demo runners for AI/ML interactive lessons
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → AI
//! ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use learn_core::demos::{
    AttentionDemo, Cell, CnnFilterDemo, GridWorldDemo, LinearRegressionDemo, NeuralNetworkDemo,
    PerceptronDemo,
};
use learn_core::Demo;
use learn_web::{AnimationLoop, Canvas};

// Thread-local state for the currently running demos
thread_local! {
    static LINEAR_REGRESSION_DEMO: RefCell<Option<DemoRunner<LinearRegressionDemo>>> = const { RefCell::new(None) };
    static PERCEPTRON_DEMO: RefCell<Option<DemoRunner<PerceptronDemo>>> = const { RefCell::new(None) };
    static NEURAL_NETWORK_DEMO: RefCell<Option<DemoRunner<NeuralNetworkDemo>>> = const { RefCell::new(None) };
    static CNN_FILTER_DEMO: RefCell<Option<DemoRunner<CnnFilterDemo>>> = const { RefCell::new(None) };
    static ATTENTION_DEMO: RefCell<Option<DemoRunner<AttentionDemo>>> = const { RefCell::new(None) };
    static GRID_WORLD_DEMO: RefCell<Option<DemoRunner<GridWorldDemo>>> = const { RefCell::new(None) };
}

/// Generic demo runner wrapper
struct DemoRunner<D: Demo> {
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    demo: D,
    training: bool,
}

/// Dispatch to the appropriate demo based on lesson index
pub fn start_demo_for_lesson(lesson_idx: usize, canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    match lesson_idx {
        0 => PlaceholderDemoRunner::start(canvas_id, "What is AI?"),
        1 => start_linear_regression_demo(canvas_id, seed),
        2 => PlaceholderDemoRunner::start(canvas_id, "Decision Boundaries"),
        3 => start_perceptron_demo(canvas_id, seed),
        4 => start_neural_network_demo(canvas_id, seed),
        5 => PlaceholderDemoRunner::start(canvas_id, "Backpropagation"),
        6 => start_cnn_filter_demo(canvas_id, seed),
        7 => PlaceholderDemoRunner::start(canvas_id, "RNNs"),
        8 => start_attention_demo(canvas_id, seed),
        9 => PlaceholderDemoRunner::start(canvas_id, "Transformers"),
        10 => PlaceholderDemoRunner::start(canvas_id, "Scaling Laws"),
        11 => start_grid_world_demo(canvas_id, seed),
        12 => PlaceholderDemoRunner::start(canvas_id, "RLHF"),
        13 => PlaceholderDemoRunner::start(canvas_id, "Generative AI"),
        14 => PlaceholderDemoRunner::start(canvas_id, "Multimodal AI"),
        15 => PlaceholderDemoRunner::start(canvas_id, "AI Safety"),
        _ => Ok(()),
    }
}

/// Stop all running demos
pub fn stop_demo() {
    LINEAR_REGRESSION_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
    PERCEPTRON_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
    NEURAL_NETWORK_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
    CNN_FILTER_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
    ATTENTION_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
    GRID_WORLD_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
}

// ═══════════════════════════════════════════════════════════════════════════════
// LINEAR REGRESSION DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_linear_regression_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = LinearRegressionDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: false,
    };

    LINEAR_REGRESSION_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |_dt| {
        LINEAR_REGRESSION_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                if runner.training {
                    runner.demo.step(0.016);
                }
                render_linear_regression(runner);
            }
        });
    });

    animation.start();

    LINEAR_REGRESSION_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_linear_regression_controls()?;
    Ok(())
}

fn render_linear_regression(runner: &DemoRunner<LinearRegressionDemo>) {
    let ctx = runner.canvas.ctx();
    let w = runner.canvas.width();
    let h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 40.0;
    let plot_size = (w - 2.0 * margin - 200.0).min(h - 2.0 * margin - 60.0);
    let plot_x = margin;
    let plot_y = margin + 30.0;

    // Coordinate transform
    let to_canvas_x = |x: f32| -> f64 { plot_x + ((x + 1.0) / 2.0) as f64 * plot_size };
    let to_canvas_y = |y: f32| -> f64 { plot_y + (1.0 - (y + 1.0) / 2.0) as f64 * plot_size };

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(100, 255, 218, 0.1)"));
    ctx.set_line_width(1.0);
    for i in 0..=10 {
        let v = i as f64 * plot_size / 10.0;
        ctx.begin_path();
        ctx.move_to(plot_x + v, plot_y);
        ctx.line_to(plot_x + v, plot_y + plot_size);
        ctx.move_to(plot_x, plot_y + v);
        ctx.line_to(plot_x + plot_size, plot_y + v);
        ctx.stroke();
    }

    // Draw axes
    ctx.set_stroke_style(&JsValue::from_str("rgba(100, 255, 218, 0.5)"));
    ctx.set_line_width(2.0);
    ctx.begin_path();
    ctx.move_to(plot_x, plot_y + plot_size);
    ctx.line_to(plot_x + plot_size, plot_y + plot_size);
    ctx.move_to(plot_x, plot_y);
    ctx.line_to(plot_x, plot_y + plot_size);
    ctx.stroke();

    // Draw target line (ground truth)
    let (target_w, target_b) = demo.target();
    ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 100, 0.4)"));
    ctx.set_line_width(2.0);
    ctx.set_line_dash(&js_sys::Array::of2(&JsValue::from(5), &JsValue::from(5)))
        .ok();
    ctx.begin_path();
    let y0 = -target_w + target_b;
    let y1 = target_w * 1.0 + target_b;
    ctx.move_to(to_canvas_x(-1.0), to_canvas_y(y0));
    ctx.line_to(to_canvas_x(1.0), to_canvas_y(y1));
    ctx.stroke();
    ctx.set_line_dash(&js_sys::Array::new()).ok();

    // Draw learned line
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(3.0);
    ctx.begin_path();
    let y0 = -demo.w + demo.b;
    let y1 = demo.w * 1.0 + demo.b;
    ctx.move_to(to_canvas_x(-1.0), to_canvas_y(y0));
    ctx.line_to(to_canvas_x(1.0), to_canvas_y(y1));
    ctx.stroke();

    // Draw data points
    ctx.set_fill_style(&JsValue::from_str("#00aaff"));
    for p in demo.points() {
        ctx.begin_path();
        ctx.arc(
            to_canvas_x(p.x),
            to_canvas_y(p.y),
            5.0,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .ok();
        ctx.fill();
    }

    // Draw info panel
    let info_x = plot_x + plot_size + 30.0;
    let info_y = plot_y + 20.0;

    ctx.set_font("bold 16px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Parameters", info_x, info_y).ok();

    ctx.set_font("14px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
    ctx.fill_text(&format!("w = {:.3}", demo.w), info_x, info_y + 30.0)
        .ok();
    ctx.fill_text(&format!("b = {:.3}", demo.b), info_x, info_y + 50.0)
        .ok();
    ctx.fill_text(
        &format!("Steps: {}", demo.step_count()),
        info_x,
        info_y + 80.0,
    )
    .ok();

    let loss = demo.compute_loss();
    ctx.fill_text(&format!("Loss: {:.4}", loss), info_x, info_y + 100.0)
        .ok();

    // Draw loss history
    let history = demo.loss_history();
    if history.len() > 1 {
        let loss_y = info_y + 140.0;
        let loss_h = 80.0;
        let loss_w = 150.0;

        ctx.set_font("bold 14px 'Rajdhani', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
        ctx.fill_text("Loss History", info_x, loss_y).ok();

        let max_loss = history.iter().cloned().fold(0.0f32, f32::max).max(0.01);

        ctx.set_stroke_style(&JsValue::from_str("#ff6644"));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        for (i, &loss) in history.iter().enumerate() {
            let x = info_x + (i as f64 / history.len() as f64) * loss_w;
            let y = loss_y + 20.0 + (1.0 - (loss / max_loss) as f64) * loss_h;
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();
    }

    // Title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("Linear Regression with Gradient Descent", margin, 25.0)
        .ok();
}

fn wire_linear_regression_controls() -> Result<(), JsValue> {
    if let Some(slider) = get_element("lr-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("lr-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        LINEAR_REGRESSION_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("learning_rate", value);
                            }
                        });
                        update_text("lr-value", &format!("{:.2}", value));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("train-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            LINEAR_REGRESSION_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.training = !runner.training;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some(if runner.training { "Pause" } else { "Train" }));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("reset-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            LINEAR_REGRESSION_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.demo.reset(42);
                    runner.training = false;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some("Train"));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("step-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            LINEAR_REGRESSION_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.demo.step(0.016);
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERCEPTRON DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_perceptron_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = PerceptronDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: false,
    };

    PERCEPTRON_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |_dt| {
        PERCEPTRON_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                if runner.training {
                    runner.demo.step(0.016);
                }
                render_perceptron(runner);
            }
        });
    });

    animation.start();

    PERCEPTRON_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_perceptron_controls()?;
    Ok(())
}

fn render_perceptron(runner: &DemoRunner<PerceptronDemo>) {
    let ctx = runner.canvas.ctx();
    let w = runner.canvas.width();
    let h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 40.0;
    let plot_size = (w - 2.0 * margin - 200.0).min(h - 2.0 * margin - 60.0);
    let plot_x = margin;
    let plot_y = margin + 30.0;

    // Draw decision boundary as colored background
    let grid = &demo.decision_boundary;
    if !grid.is_empty() {
        let n = grid.len();
        // Each cell covers a range in data space - calculate cell size to cover the full plot
        let cell_w = plot_size / (n - 1) as f64;
        let cell_h = plot_size / (n - 1) as f64;

        for (iy, row) in grid.iter().enumerate() {
            for (ix, &val) in row.iter().enumerate() {
                let color = if val > 0.5 {
                    format!("rgba(0, 170, 255, {})", (val - 0.5) * 1.2)
                } else {
                    format!("rgba(255, 100, 100, {})", (0.5 - val) * 1.2)
                };
                ctx.set_fill_style(&JsValue::from_str(&color));

                // Position cells at their correct data coordinates
                // Grid[iy][ix] = prediction at x = ix/(n-1)*2-1, y = iy/(n-1)*2-1
                let cx = plot_x + (ix as f64 / (n - 1) as f64) * plot_size - cell_w / 2.0;
                let cy = plot_y + (1.0 - iy as f64 / (n - 1) as f64) * plot_size - cell_h / 2.0;

                ctx.fill_rect(cx, cy, cell_w + 1.0, cell_h + 1.0);
            }
        }
    }

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(100, 255, 218, 0.15)"));
    ctx.set_line_width(1.0);
    for i in 0..=10 {
        let v = i as f64 * plot_size / 10.0;
        ctx.begin_path();
        ctx.move_to(plot_x + v, plot_y);
        ctx.line_to(plot_x + v, plot_y + plot_size);
        ctx.move_to(plot_x, plot_y + v);
        ctx.line_to(plot_x + plot_size, plot_y + v);
        ctx.stroke();
    }

    // Coordinate transform
    let to_canvas_x = |x: f32| -> f64 { plot_x + ((x + 1.0) / 2.0) as f64 * plot_size };
    let to_canvas_y = |y: f32| -> f64 { plot_y + (1.0 - (y + 1.0) / 2.0) as f64 * plot_size };

    // Draw data points
    for p in &demo.points {
        let color = if p.label { "#00aaff" } else { "#ff6644" };
        ctx.set_fill_style(&JsValue::from_str(color));
        ctx.begin_path();
        ctx.arc(
            to_canvas_x(p.pos.x),
            to_canvas_y(p.pos.y),
            5.0,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .ok();
        ctx.fill();
    }

    // Draw info panel
    let info_x = plot_x + plot_size + 30.0;
    let info_y = plot_y + 20.0;

    ctx.set_font("bold 16px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Model Info", info_x, info_y).ok();

    ctx.set_font("14px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));

    let mode = if demo.use_hidden_layer {
        "MLP (4 hidden)"
    } else {
        "Perceptron"
    };
    ctx.fill_text(&format!("Mode: {}", mode), info_x, info_y + 30.0)
        .ok();
    ctx.fill_text(
        &format!("Dataset: {}", demo.dataset.name()),
        info_x,
        info_y + 50.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Steps: {}", demo.step_count),
        info_x,
        info_y + 70.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Accuracy: {:.1}%", demo.accuracy * 100.0),
        info_x,
        info_y + 90.0,
    )
    .ok();

    // Draw loss history
    if demo.loss_history.len() > 1 {
        let loss_y = info_y + 130.0;
        let loss_h = 80.0;
        let loss_w = 150.0;

        ctx.set_font("bold 14px 'Rajdhani', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
        ctx.fill_text("Loss History", info_x, loss_y).ok();

        let max_loss = demo.loss_history.iter().cloned().fold(0.01f32, f32::max);

        ctx.set_stroke_style(&JsValue::from_str("#ff6644"));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        for (i, &loss) in demo.loss_history.iter().enumerate() {
            let x = info_x + (i as f64 / demo.loss_history.len() as f64) * loss_w;
            let y = loss_y + 20.0 + (1.0 - (loss / max_loss) as f64) * loss_h;
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();
    }

    // Title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("Perceptron Decision Boundary", margin, 25.0)
        .ok();
}

fn wire_perceptron_controls() -> Result<(), JsValue> {
    if let Some(btn) = get_element("train-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            PERCEPTRON_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.training = !runner.training;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some(if runner.training { "Pause" } else { "Train" }));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("reset-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            PERCEPTRON_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.demo.reset(42);
                    runner.training = false;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some("Train"));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(select) = get_element("dataset-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("dataset-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    PERCEPTRON_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("dataset", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(checkbox) = get_element("hidden-layer-checkbox") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(cb) = get_element("hidden-layer-checkbox") {
                if let Ok(input) = cb.dyn_into::<web_sys::HtmlInputElement>() {
                    let checked = input.checked();
                    PERCEPTRON_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner
                                .demo
                                .set_param("hidden_layer", if checked { 1.0 } else { 0.0 });
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        checkbox.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEURAL NETWORK PLAYGROUND DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_neural_network_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = NeuralNetworkDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: false,
    };

    NEURAL_NETWORK_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |_dt| {
        NEURAL_NETWORK_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                if runner.training {
                    runner.demo.step(0.016);
                }
                render_neural_network(runner);
            }
        });
    });

    animation.start();

    NEURAL_NETWORK_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_neural_network_controls()?;
    Ok(())
}

fn render_neural_network(runner: &DemoRunner<NeuralNetworkDemo>) {
    let ctx = runner.canvas.ctx();
    let w = runner.canvas.width();
    let h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 40.0;
    let plot_size = (w - 2.0 * margin - 220.0).min(h - 2.0 * margin - 60.0);
    let plot_x = margin;
    let plot_y = margin + 30.0;

    // Draw decision boundary as colored background
    let grid = &demo.decision_grid;
    if !grid.is_empty() {
        let n = grid.len();
        let cell_w = plot_size / (n - 1) as f64;
        let cell_h = plot_size / (n - 1) as f64;

        for (iy, row) in grid.iter().enumerate() {
            for (ix, &val) in row.iter().enumerate() {
                // Map [-1, 1] to color
                let normalized = (val + 1.0) / 2.0;
                let color = if normalized > 0.5 {
                    format!("rgba(0, 170, 255, {})", (normalized - 0.5) * 1.2)
                } else {
                    format!("rgba(255, 100, 100, {})", (0.5 - normalized) * 1.2)
                };
                ctx.set_fill_style(&JsValue::from_str(&color));

                // Position cells at their correct data coordinates
                let cx = plot_x + (ix as f64 / (n - 1) as f64) * plot_size - cell_w / 2.0;
                let cy = plot_y + (1.0 - iy as f64 / (n - 1) as f64) * plot_size - cell_h / 2.0;

                ctx.fill_rect(cx, cy, cell_w + 1.0, cell_h + 1.0);
            }
        }
    }

    // Coordinate transform
    let to_canvas_x = |x: f32| -> f64 { plot_x + ((x + 1.0) / 2.0) as f64 * plot_size };
    let to_canvas_y = |y: f32| -> f64 { plot_y + (1.0 - (y + 1.0) / 2.0) as f64 * plot_size };

    // Draw data points
    for p in &demo.points {
        let color = if p.label > 0 { "#00aaff" } else { "#ff6644" };
        ctx.set_fill_style(&JsValue::from_str(color));
        ctx.begin_path();
        ctx.arc(
            to_canvas_x(p.x),
            to_canvas_y(p.y),
            4.0,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .ok();
        ctx.fill();
    }

    // Draw info panel
    let info_x = plot_x + plot_size + 30.0;
    let info_y = plot_y + 20.0;

    ctx.set_font("bold 16px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Network Info", info_x, info_y).ok();

    ctx.set_font("14px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));

    let arch: Vec<String> = demo.layer_sizes.iter().map(|n| n.to_string()).collect();
    ctx.fill_text(&format!("Arch: {}", arch.join("-")), info_x, info_y + 25.0)
        .ok();
    ctx.fill_text(
        &format!("Activation: {}", demo.activation.name()),
        info_x,
        info_y + 45.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Dataset: {}", demo.dataset.name()),
        info_x,
        info_y + 65.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Steps: {}", demo.step_count),
        info_x,
        info_y + 85.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Accuracy: {:.1}%", demo.accuracy * 100.0),
        info_x,
        info_y + 105.0,
    )
    .ok();

    // Draw network diagram (simplified)
    let net_y = info_y + 140.0;
    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Network", info_x, net_y).ok();

    let layer_spacing = 35.0;
    let neuron_radius = 8.0;
    for (l, &size) in demo.layer_sizes.iter().enumerate() {
        let lx = info_x + l as f64 * layer_spacing + 10.0;
        let neurons = size.min(6);
        let start_y = net_y + 20.0;

        for n in 0..neurons {
            let ny = start_y + n as f64 * 18.0;
            ctx.set_fill_style(&JsValue::from_str("rgba(0, 170, 255, 0.6)"));
            ctx.begin_path();
            ctx.arc(lx, ny, neuron_radius, 0.0, std::f64::consts::PI * 2.0)
                .ok();
            ctx.fill();
        }

        if size > 6 {
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.fill_text("...", lx - 5.0, start_y + 6.0 * 18.0).ok();
        }
    }

    // Title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("Neural Network Playground", margin, 25.0)
        .ok();
}

fn wire_neural_network_controls() -> Result<(), JsValue> {
    if let Some(btn) = get_element("train-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            NEURAL_NETWORK_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.training = !runner.training;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some(if runner.training { "Pause" } else { "Train" }));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("reset-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            NEURAL_NETWORK_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.demo.reset(42);
                    runner.training = false;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some("Train"));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Dataset selector
    if let Some(select) = get_element("nn-dataset-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("nn-dataset-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    NEURAL_NETWORK_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("dataset", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Layers slider
    if let Some(slider) = get_element("layers-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("layers-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        NEURAL_NETWORK_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("hidden_layers", value);
                            }
                        });
                        update_text("layers-value", &format!("{}", value as i32));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Neurons slider
    if let Some(slider) = get_element("neurons-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("neurons-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        NEURAL_NETWORK_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("neurons", value);
                            }
                        });
                        update_text("neurons-value", &format!("{}", value as i32));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// CNN FILTER DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_cnn_filter_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = CnnFilterDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: true, // Auto-animate by default
    };

    CNN_FILTER_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |dt| {
        CNN_FILTER_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.demo.step(dt as f32);
                render_cnn_filter(runner);
            }
        });
    });

    animation.start();

    CNN_FILTER_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_cnn_filter_controls()?;
    Ok(())
}

fn render_cnn_filter(runner: &DemoRunner<CnnFilterDemo>) {
    let ctx = runner.canvas.ctx();
    let _w = runner.canvas.width();
    let _h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 30.0;
    let grid_pixel = 20.0;
    let input_size = demo.input_size as f64 * grid_pixel;

    // Layout positions
    let input_x = margin;
    let input_y = margin + 40.0;
    let kernel_x = input_x + input_size + 40.0;
    let kernel_y = input_y + 40.0;
    let output_x = kernel_x + 120.0;
    let output_y = input_y;

    // Draw title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("CNN Convolution Visualization", margin, 25.0)
        .ok();

    // Draw input image
    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Input Image", input_x, input_y - 10.0).ok();

    for (y, row) in demo.input.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let brightness = (val * 255.0) as u8;
            ctx.set_fill_style(&JsValue::from_str(&format!(
                "rgb({},{},{})",
                brightness, brightness, brightness
            )));
            ctx.fill_rect(
                input_x + x as f64 * grid_pixel,
                input_y + y as f64 * grid_pixel,
                grid_pixel - 1.0,
                grid_pixel - 1.0,
            );
        }
    }

    // Highlight current convolution window
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(3.0);
    ctx.stroke_rect(
        input_x + demo.current_x as f64 * grid_pixel - 1.0,
        input_y + demo.current_y as f64 * grid_pixel - 1.0,
        3.0 * grid_pixel + 2.0,
        3.0 * grid_pixel + 2.0,
    );

    // Draw kernel
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Kernel (3x3)", kernel_x, kernel_y - 10.0)
        .ok();

    ctx.set_font("12px 'JetBrains Mono', monospace");
    for y in 0..3 {
        for x in 0..3 {
            let val = demo.kernel[y][x];
            let color = if val > 0.0 {
                format!("rgba(0, 170, 255, {})", (val.abs() / 8.0).min(1.0))
            } else if val < 0.0 {
                format!("rgba(255, 100, 100, {})", (val.abs() / 8.0).min(1.0))
            } else {
                "rgba(128, 128, 128, 0.3)".to_string()
            };
            ctx.set_fill_style(&JsValue::from_str(&color));
            ctx.fill_rect(
                kernel_x + x as f64 * 30.0,
                kernel_y + y as f64 * 30.0,
                28.0,
                28.0,
            );

            ctx.set_fill_style(&JsValue::from_str("#ffffff"));
            ctx.fill_text(
                &format!("{:.1}", val),
                kernel_x + x as f64 * 30.0 + 4.0,
                kernel_y + y as f64 * 30.0 + 18.0,
            )
            .ok();
        }
    }

    // Draw output
    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Output Feature Map", output_x, output_y - 10.0)
        .ok();

    let normalized = demo.normalized_output();
    for (y, row) in normalized.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let brightness = (val * 255.0) as u8;
            ctx.set_fill_style(&JsValue::from_str(&format!(
                "rgb({},{},{})",
                brightness, brightness, brightness
            )));
            ctx.fill_rect(
                output_x + x as f64 * grid_pixel,
                output_y + y as f64 * grid_pixel,
                grid_pixel - 1.0,
                grid_pixel - 1.0,
            );
        }
    }

    // Highlight current output position
    if demo.current_y < normalized.len()
        && demo.current_x < normalized.first().map(|r| r.len()).unwrap_or(0)
    {
        ctx.set_stroke_style(&JsValue::from_str("#ffaa00"));
        ctx.set_line_width(2.0);
        ctx.stroke_rect(
            output_x + demo.current_x as f64 * grid_pixel - 1.0,
            output_y + demo.current_y as f64 * grid_pixel - 1.0,
            grid_pixel + 2.0,
            grid_pixel + 2.0,
        );
    }

    // Show current computation
    ctx.set_font("14px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
    ctx.fill_text(
        &format!("Position: ({}, {})", demo.current_x, demo.current_y),
        kernel_x,
        kernel_y + 120.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Sum: {:.2}", demo.current_sum),
        kernel_x,
        kernel_y + 140.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Filter: {}", demo.filter_type.name()),
        kernel_x,
        kernel_y + 160.0,
    )
    .ok();
}

fn wire_cnn_filter_controls() -> Result<(), JsValue> {
    if let Some(select) = get_element("filter-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("filter-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    CNN_FILTER_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("filter", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(select) = get_element("pattern-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("pattern-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    CNN_FILTER_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("pattern", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(checkbox) = get_element("animate-checkbox") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(cb) = get_element("animate-checkbox") {
                if let Ok(input) = cb.dyn_into::<web_sys::HtmlInputElement>() {
                    let checked = input.checked();
                    CNN_FILTER_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner
                                .demo
                                .set_param("animate", if checked { 1.0 } else { 0.0 });
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        checkbox.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// ATTENTION DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_attention_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = AttentionDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: false,
    };

    ATTENTION_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |_dt| {
        ATTENTION_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                render_attention(runner);
            }
        });
    });

    animation.start();

    ATTENTION_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_attention_controls()?;
    Ok(())
}

fn render_attention(runner: &DemoRunner<AttentionDemo>) {
    let ctx = runner.canvas.ctx();
    let w = runner.canvas.width();
    let _h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 30.0;
    let token_width = 70.0;
    let token_height = 35.0;
    let tokens_y = margin + 60.0;

    // Title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("Attention Mechanism Visualization", margin, 25.0)
        .ok();

    // Draw tokens
    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Tokens:", margin, tokens_y - 15.0).ok();

    for (i, token) in demo.tokens.iter().enumerate() {
        let x = margin + i as f64 * (token_width + 10.0);

        // Token box
        let is_selected = i == demo.selected_query;
        let color = if is_selected {
            "#00ffaa"
        } else {
            "rgba(100, 255, 218, 0.3)"
        };
        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(if is_selected { 3.0 } else { 1.0 });
        ctx.stroke_rect(x, tokens_y, token_width, token_height);

        // Token text
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_text_align("center");
        ctx.fill_text(token, x + token_width / 2.0, tokens_y + 22.0)
            .ok();
    }
    ctx.set_text_align("left");

    // Draw attention weights as arcs
    let attention = demo.selected_attention();
    if !attention.is_empty() {
        let arc_y = tokens_y + token_height + 30.0;

        ctx.set_font("bold 14px 'Rajdhani', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
        ctx.fill_text(
            &format!(
                "Attention from '{}':",
                demo.tokens
                    .get(demo.selected_query)
                    .unwrap_or(&"?".to_string())
            ),
            margin,
            arc_y,
        )
        .ok();

        let bar_y = arc_y + 20.0;
        let bar_height = 25.0;
        let max_width = 150.0;

        for (i, &weight) in attention.iter().enumerate() {
            let y = bar_y + i as f64 * (bar_height + 5.0);

            // Token label
            ctx.set_font("12px 'JetBrains Mono', monospace");
            ctx.set_fill_style(&JsValue::from_str("#888"));
            ctx.fill_text(
                demo.tokens.get(i).unwrap_or(&"?".to_string()),
                margin,
                y + 17.0,
            )
            .ok();

            // Weight bar
            let bar_x = margin + 80.0;
            let bar_w = weight as f64 * max_width;
            let alpha = 0.3 + weight as f64 * 0.7;
            ctx.set_fill_style(&JsValue::from_str(&format!("rgba(0, 170, 255, {})", alpha)));
            ctx.fill_rect(bar_x, y, bar_w, bar_height);

            // Weight value
            ctx.set_fill_style(&JsValue::from_str("#ffffff"));
            ctx.fill_text(&format!("{:.2}", weight), bar_x + bar_w + 10.0, y + 17.0)
                .ok();
        }
    }

    // Draw attention matrix heatmap
    let matrix_x = w - 250.0;
    let matrix_y = tokens_y;
    let cell_size = 30.0;

    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Attention Matrix", matrix_x, matrix_y - 15.0)
        .ok();

    let n = demo.attention_weights.len().min(6);
    for q in 0..n {
        for k in 0..n.min(demo.attention_weights.get(q).map(|r| r.len()).unwrap_or(0)) {
            let weight = demo.attention_weights[q][k];
            let brightness = (weight * 255.0) as u8;
            ctx.set_fill_style(&JsValue::from_str(&format!(
                "rgb(0, {}, {})",
                brightness, brightness
            )));
            ctx.fill_rect(
                matrix_x + k as f64 * cell_size,
                matrix_y + q as f64 * cell_size,
                cell_size - 1.0,
                cell_size - 1.0,
            );
        }
    }

    // Highlight selected row
    ctx.set_stroke_style(&JsValue::from_str("#00ffaa"));
    ctx.set_line_width(2.0);
    if demo.selected_query < n {
        ctx.stroke_rect(
            matrix_x - 2.0,
            matrix_y + demo.selected_query as f64 * cell_size - 2.0,
            n as f64 * cell_size + 4.0,
            cell_size + 4.0,
        );
    }
}

fn wire_attention_controls() -> Result<(), JsValue> {
    if let Some(select) = get_element("sentence-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("sentence-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    ATTENTION_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("sentence", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(slider) = get_element("query-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("query-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        ATTENTION_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("selected_query", value);
                                update_text(
                                    "query-value",
                                    runner
                                        .demo
                                        .tokens
                                        .get(value as usize)
                                        .unwrap_or(&"?".to_string()),
                                );
                            }
                        });
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(slider) = get_element("temp-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("temp-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        ATTENTION_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("temperature", value);
                            }
                        });
                        update_text("temp-value", &format!("{:.1}", value));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// GRID WORLD RL DEMO
// ═══════════════════════════════════════════════════════════════════════════════

fn start_grid_world_demo(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;
    let mut demo = GridWorldDemo::default();
    demo.reset(seed);

    let runner = DemoRunner {
        canvas,
        animation: None,
        demo,
        training: true, // Auto-train
    };

    GRID_WORLD_DEMO.with(|d| {
        *d.borrow_mut() = Some(runner);
    });

    let animation = AnimationLoop::new(move |dt| {
        GRID_WORLD_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                if runner.training {
                    runner.demo.step(dt as f32);
                }
                render_grid_world(runner);
            }
        });
    });

    animation.start();

    GRID_WORLD_DEMO.with(|d| {
        if let Some(runner) = d.borrow_mut().as_mut() {
            runner.animation = Some(Rc::new(animation));
        }
    });

    wire_grid_world_controls()?;
    Ok(())
}

fn render_grid_world(runner: &DemoRunner<GridWorldDemo>) {
    let ctx = runner.canvas.ctx();
    let _w = runner.canvas.width();
    let h = runner.canvas.height();
    let demo = &runner.demo;

    runner.canvas.clear("#0a0a12");

    let margin = 30.0;
    let cell_size = ((h - 2.0 * margin - 100.0) / demo.height as f64).min(50.0);
    let grid_x = margin;
    let grid_y = margin + 40.0;

    // Title
    ctx.set_font("bold 20px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.fill_text("Grid World Q-Learning", margin, 25.0).ok();

    // Get normalized values for coloring
    let values = demo.normalized_values();
    let policy = demo.policy();

    // Draw grid
    for y in 0..demo.height {
        for x in 0..demo.width {
            let cx = grid_x + x as f64 * cell_size;
            let cy = grid_y + y as f64 * cell_size;
            let cell = demo.grid[y][x];

            // Cell background based on type or value
            let color = match cell {
                Cell::Wall => "#333333",
                Cell::Goal => "#00ff88",
                Cell::Pit => "#ff4444",
                Cell::Start => "#4488ff",
                Cell::Empty => {
                    let v = values.get(y).and_then(|r| r.get(x)).copied().unwrap_or(0.5);
                    &format!("rgba(100, 200, 255, {})", v * 0.5)
                }
            };

            if matches!(cell, Cell::Empty) {
                let v = values.get(y).and_then(|r| r.get(x)).copied().unwrap_or(0.5);
                ctx.set_fill_style(&JsValue::from_str(&format!(
                    "rgba(100, 200, 255, {})",
                    v * 0.5
                )));
            } else {
                ctx.set_fill_style(&JsValue::from_str(color));
            }
            ctx.fill_rect(cx, cy, cell_size - 2.0, cell_size - 2.0);

            // Draw policy arrow
            if demo.show_policy {
                if let Some(Some(action)) = policy.get(y).and_then(|r| r.get(x)) {
                    ctx.set_font("20px sans-serif");
                    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
                    ctx.set_text_align("center");
                    ctx.fill_text(
                        &action.symbol().to_string(),
                        cx + cell_size / 2.0,
                        cy + cell_size / 2.0 + 7.0,
                    )
                    .ok();
                }
            }
        }
    }
    ctx.set_text_align("left");

    // Draw agent
    let ax = grid_x + demo.agent_x as f64 * cell_size + cell_size / 2.0;
    let ay = grid_y + demo.agent_y as f64 * cell_size + cell_size / 2.0;
    ctx.set_fill_style(&JsValue::from_str("#ffaa00"));
    ctx.begin_path();
    ctx.arc(ax, ay, cell_size / 3.0, 0.0, std::f64::consts::PI * 2.0)
        .ok();
    ctx.fill();

    // Draw info panel
    let info_x = grid_x + demo.width as f64 * cell_size + 40.0;
    let info_y = grid_y + 20.0;

    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Learning Stats", info_x, info_y).ok();

    ctx.set_font("14px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
    ctx.fill_text(&format!("Episode: {}", demo.episode), info_x, info_y + 25.0)
        .ok();
    ctx.fill_text(
        &format!("Epsilon: {:.2}", demo.epsilon),
        info_x,
        info_y + 45.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Avg Reward: {:.2}", demo.avg_reward()),
        info_x,
        info_y + 65.0,
    )
    .ok();
    ctx.fill_text(
        &format!("Layout: {}", demo.layout.name()),
        info_x,
        info_y + 85.0,
    )
    .ok();

    // Legend
    let legend_y = info_y + 120.0;
    ctx.set_font("bold 14px 'Rajdhani', sans-serif");
    ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
    ctx.fill_text("Legend", info_x, legend_y).ok();

    let items = [
        ("#00ff88", "Goal (+1)"),
        ("#ff4444", "Pit (-1)"),
        ("#4488ff", "Start"),
        ("#333333", "Wall"),
        ("#ffaa00", "Agent"),
    ];

    ctx.set_font("12px 'JetBrains Mono', monospace");
    for (i, (color, label)) in items.iter().enumerate() {
        let y = legend_y + 20.0 + i as f64 * 20.0;
        ctx.set_fill_style(&JsValue::from_str(color));
        ctx.fill_rect(info_x, y, 15.0, 15.0);
        ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
        ctx.fill_text(label, info_x + 25.0, y + 12.0).ok();
    }
}

fn wire_grid_world_controls() -> Result<(), JsValue> {
    if let Some(btn) = get_element("train-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            GRID_WORLD_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.training = !runner.training;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some(if runner.training { "Pause" } else { "Train" }));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(btn) = get_element("reset-btn") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            GRID_WORLD_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    runner.demo.reset(42);
                    runner.training = true;
                    if let Some(btn) = get_element("train-btn") {
                        btn.set_text_content(Some("Pause"));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(select) = get_element("layout-select") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(select) = get_element("layout-select") {
                if let Ok(sel) = select.dyn_into::<web_sys::HtmlSelectElement>() {
                    let idx = sel.selected_index() as usize;
                    GRID_WORLD_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.set_param("layout", idx as f32);
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    if let Some(slider) = get_element("lr-slider") {
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(slider) = get_element("lr-slider") {
                if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        GRID_WORLD_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("learning_rate", value);
                            }
                        });
                        update_text("lr-value", &format!("{:.2}", value));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// PLACEHOLDER DEMO
// ═══════════════════════════════════════════════════════════════════════════════

struct PlaceholderDemoRunner;

impl PlaceholderDemoRunner {
    pub fn start(canvas_id: &str, title: &str) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let ctx = canvas.ctx();
        let w = canvas.width();
        let h = canvas.height();

        canvas.clear("#0a0a12");

        ctx.set_font("bold 24px 'Rajdhani', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#00ffaa"));
        ctx.set_text_align("center");
        ctx.fill_text(title, w / 2.0, h / 2.0 - 20.0).ok();

        ctx.set_font("16px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888888"));
        ctx.fill_text("Interactive demo coming soon...", w / 2.0, h / 2.0 + 20.0)
            .ok();

        ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 170, 0.2)"));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.arc(w / 2.0, h / 2.0, 80.0, 0.0, std::f64::consts::PI * 2.0)
            .ok();
        ctx.stroke();

        ctx.begin_path();
        ctx.arc(w / 2.0, h / 2.0, 100.0, 0.0, std::f64::consts::PI * 2.0)
            .ok();
        ctx.stroke();

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn get_element(id: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

fn update_text(id: &str, text: &str) {
    if let Some(el) = get_element(id) {
        el.set_text_content(Some(text));
    }
}
