//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: demo_runner.rs | ESP32/src/demo_runner.rs
//! PURPOSE: Demo runners for ESP32 lessons (Debounce, PWM, ADC, I2C)
//! MODIFIED: 2025-12-11
//! LAYER: LEARN → ESP32
//! ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use learn_core::demos::{
    AdcAttenuation, AdcReadingDemo, GpioDebounceDemo, I2cBusDemo, I2cPhase, I2cStage,
    OhmsLawPowerDemo, PowerBudgetDemo, PwmControlDemo, RcTimeConstantDemo,
};
use learn_core::Demo;
use learn_web::{AnimationLoop, Canvas};

// Thread-local state for the currently running demos
thread_local! {
    static GPIO_DEBOUNCE_DEMO: RefCell<Option<GpioDebounceDemoRunner>> = const { RefCell::new(None) };
    static PWM_DEMO: RefCell<Option<PwmControlDemoRunner>> = const { RefCell::new(None) };
    static ADC_DEMO: RefCell<Option<AdcReadingDemoRunner>> = const { RefCell::new(None) };
    static I2C_DEMO: RefCell<Option<I2cBusDemoRunner>> = const { RefCell::new(None) };
    static OHMS_LAW_DEMO: RefCell<Option<OhmsLawPowerDemoRunner>> = const { RefCell::new(None) };
    static RC_DEMO: RefCell<Option<RcTimeConstantDemoRunner>> = const { RefCell::new(None) };
    static POWER_BUDGET_DEMO: RefCell<Option<PowerBudgetDemoRunner>> = const { RefCell::new(None) };
}

/// Dispatch to the appropriate demo based on lesson index
/// Order: 2=Ohm's Law, 5=RC, 8=GPIO Debounce, 9=PWM, 10=ADC, 11=I2C, 19=Power Budget
pub fn start_demo_for_lesson(lesson_idx: usize, canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    match lesson_idx {
        2 => OhmsLawPowerDemoRunner::start(canvas_id, seed),
        5 => RcTimeConstantDemoRunner::start(canvas_id, seed),
        8 => GpioDebounceDemoRunner::start(canvas_id, seed),
        9 => PwmControlDemoRunner::start(canvas_id, seed),
        10 => AdcReadingDemoRunner::start(canvas_id, seed),
        11 => I2cBusDemoRunner::start(canvas_id, seed),
        19 => PowerBudgetDemoRunner::start(canvas_id, seed),
        _ => Ok(()),
    }
}

/// GPIO Debounce demo runner
pub struct GpioDebounceDemoRunner {
    demo: GpioDebounceDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl GpioDebounceDemoRunner {
    /// Start the GPIO Debounce demo
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = GpioDebounceDemo::default();
        demo.reset(seed);

        let runner = GpioDebounceDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        GPIO_DEBOUNCE_DEMO.with(|d| {
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
            GPIO_DEBOUNCE_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        GPIO_DEBOUNCE_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        // Bounce severity slider
        if let Ok(slider) = get_input("bounce-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("bounce-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        GPIO_DEBOUNCE_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("bounce_severity", value);
                            }
                        });
                        update_text("bounce-value", &format!("{:.2}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Sample rate slider (Hz)
        if let Ok(slider) = get_input("sample-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("sample-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        GPIO_DEBOUNCE_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("sample_rate", value);
                            }
                        });
                        update_text("sample-value", &format!("{}", value as i32));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Toggle period slider (seconds)
        if let Ok(slider) = get_input("toggle-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("toggle-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        GPIO_DEBOUNCE_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("toggle_period", value);
                            }
                        });
                        update_text("toggle-value", &format!("{:.1}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Debounce window slider (ms to seconds)
        if let Ok(slider) = get_input("window-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("window-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        GPIO_DEBOUNCE_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("debounce_window", value / 1000.0);
                            }
                        });
                        update_text("window-value", &format!("{}", value as i32));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Reset button
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                GPIO_DEBOUNCE_DEMO.with(|d| {
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
                GPIO_DEBOUNCE_DEMO.with(|d| {
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

        let margin = 30.0;
        let timeline_height = 60.0;
        let gap = 40.0;
        let led_size = 40.0;

        // Timeline dimensions
        let timeline_width = w - 2.0 * margin - led_size - 30.0;
        let timeline_x = margin;

        // Draw labels
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("Raw Signal", timeline_x, margin + 10.0);
        let _ = ctx.fill_text(
            "Debounced",
            timeline_x,
            margin + timeline_height + gap + 10.0,
        );

        // Draw raw signal timeline
        let raw_y = margin + 25.0;
        self.draw_timeline(
            timeline_x,
            raw_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.raw_history,
            "#ff6644",
        );

        // Draw debounced signal timeline
        let debounce_y = margin + timeline_height + gap + 25.0;
        self.draw_timeline(
            timeline_x,
            debounce_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.debounced_history,
            "#44ff88",
        );

        // Draw LED indicator
        let led_x = w - margin - led_size / 2.0;
        let led_y = margin + timeline_height + gap / 2.0;

        // LED glow
        if self.demo.debounced_state {
            ctx.set_fill_style(&JsValue::from_str("rgba(68, 255, 136, 0.3)"));
            ctx.begin_path();
            let _ = ctx.arc(led_x, led_y, led_size * 0.8, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // LED body
        let led_color = if self.demo.debounced_state {
            "#44ff88"
        } else {
            "#442222"
        };
        self.canvas
            .fill_circle(led_x, led_y, led_size / 2.0, led_color);

        // LED border
        ctx.set_stroke_style(&JsValue::from_str(if self.demo.debounced_state {
            "#88ffaa"
        } else {
            "#664444"
        }));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        let _ = ctx.arc(led_x, led_y, led_size / 2.0, 0.0, std::f64::consts::TAU);
        ctx.stroke();

        // LED label
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("LED", led_x - 10.0, led_y + led_size / 2.0 + 15.0);

        // Draw bouncing indicator
        let bounce_y = h - margin - 30.0;
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("Status:", margin, bounce_y);

        let (status_text, status_color) = if self.demo.is_bouncing() {
            ("BOUNCING", "#ff6644")
        } else if self.demo.debounced_state {
            ("HIGH (Stable)", "#44ff88")
        } else {
            ("LOW (Stable)", "#666")
        };
        ctx.set_fill_style(&JsValue::from_str(status_color));
        ctx.set_font("bold 14px 'Inter', sans-serif");
        let _ = ctx.fill_text(status_text, margin + 60.0, bounce_y);

        // Draw raw vs debounced state
        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let raw_indicator = if self.demo.raw_state { "1" } else { "0" };
        let deb_indicator = if self.demo.debounced_state { "1" } else { "0" };
        let _ = ctx.fill_text(
            &format!("Raw: {} | Debounced: {}", raw_indicator, deb_indicator),
            margin + 200.0,
            bounce_y,
        );

        // Draw time
        ctx.set_fill_style(&JsValue::from_str("#555"));
        let _ = ctx.fill_text(
            &format!("Time: {:.2}s", self.demo.time),
            w - margin - 80.0,
            bounce_y,
        );
    }

    fn draw_timeline(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        history: &[bool],
        color: &str,
    ) {
        let ctx = self.canvas.ctx();

        // Background
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(x, y, width, height);

        // Border
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, width, height);

        // Draw signal
        if history.is_empty() {
            return;
        }

        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        let py_high = y + 5.0;
        let py_low = y + height - 5.0;

        let len = history.len();
        let step = width / len as f64;

        let mut prev_state = history[0];
        let start_py = if prev_state { py_high } else { py_low };
        ctx.move_to(x, start_py);

        for (i, &state) in history.iter().enumerate() {
            let px = x + (i as f64) * step;

            if state != prev_state {
                // Draw horizontal line at previous level
                let prev_py = if prev_state { py_high } else { py_low };
                ctx.line_to(px, prev_py);
                // Then vertical transition
                let curr_py = if state { py_high } else { py_low };
                ctx.line_to(px, curr_py);
            }

            prev_state = state;
        }

        // Final horizontal segment
        let final_py = if prev_state { py_high } else { py_low };
        ctx.line_to(x + width, final_py);

        ctx.stroke();

        // Draw HIGH/LOW labels
        ctx.set_font("10px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#555"));
        let _ = ctx.fill_text("1", x - 12.0, py_high + 4.0);
        let _ = ctx.fill_text("0", x - 12.0, py_low + 4.0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PWM CONTROL DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// PWM Control demo runner
pub struct PwmControlDemoRunner {
    demo: PwmControlDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl PwmControlDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = PwmControlDemo::default();
        demo.reset(seed);

        let runner = PwmControlDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        PWM_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            PWM_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        PWM_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        // Duty cycle (percent)
        if let Ok(slider) = get_input("duty-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("duty-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        let duty = (value / 100.0).clamp(0.0, 1.0);
                        PWM_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("duty", duty);
                                update_text(
                                    "quantized-duty-value",
                                    &format!("{:.1}", runner.demo.quantized_duty * 100.0),
                                );
                            }
                        });
                        update_text("duty-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Frequency (Hz)
        if let Ok(slider) = get_input("freq-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("freq-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        PWM_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("frequency", value);
                            }
                        });
                        update_text("freq-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Resolution (bits)
        if let Ok(slider) = get_input("res-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("res-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        PWM_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("resolution_bits", value);
                                update_text(
                                    "quantized-duty-value",
                                    &format!("{:.1}", runner.demo.quantized_duty * 100.0),
                                );
                            }
                        });
                        update_text("res-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Smoothing tau (ms -> s)
        if let Ok(slider) = get_input("tau-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("tau-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        PWM_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("filter_tau", value / 1000.0);
                            }
                        });
                        update_text("tau-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Reset
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                PWM_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.demo.reset(seed);
                        update_text(
                            "quantized-duty-value",
                            &format!("{:.1}", runner.demo.quantized_duty * 100.0),
                        );
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Pause
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pause-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                PWM_DEMO.with(|d| {
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

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let timeline_height = 60.0;
        let gap = 40.0;
        let led_size = 44.0;

        let timeline_width = w - 2.0 * margin - led_size - 30.0;
        let timeline_x = margin;

        // Labels
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("PWM Output (Digital)", timeline_x, margin + 10.0);
        let _ = ctx.fill_text(
            "Averaged Output (Analog-ish)",
            timeline_x,
            margin + timeline_height + gap + 10.0,
        );

        // Draw raw PWM waveform
        let raw_y = margin + 25.0;
        self.draw_bool_timeline(
            timeline_x,
            raw_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.raw_history,
            "#ffaa44",
        );

        // Draw averaged waveform
        let avg_y = margin + timeline_height + gap + 25.0;
        self.draw_float_timeline(
            timeline_x,
            avg_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.avg_history,
            "#44ff88",
        );

        // LED indicator
        let led_x = w - margin - led_size / 2.0;
        let led_y = margin + timeline_height + gap / 2.0;
        let bright = (self.demo.avg as f64).clamp(0.0, 1.0);
        let glow = format!("rgba(255, 170, 68, {:.2})", 0.15 + 0.35 * bright);
        let fill = format!("rgba(255, 170, 68, {:.2})", 0.15 + 0.85 * bright);

        ctx.set_fill_style(&JsValue::from_str(&glow));
        ctx.begin_path();
        let _ = ctx.arc(led_x, led_y, led_size * 0.9, 0.0, std::f64::consts::TAU);
        ctx.fill();

        self.canvas.fill_circle(led_x, led_y, led_size / 2.0, &fill);

        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 170, 68, 0.6)"));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        let _ = ctx.arc(led_x, led_y, led_size / 2.0, 0.0, std::f64::consts::TAU);
        ctx.stroke();

        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("LED", led_x - 10.0, led_y + led_size / 2.0 + 15.0);

        // Stats
        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let _ = ctx.fill_text(
            &format!(
                "Duty: {:>5.1}% (q {:>5.1}%) | Freq: {:>4.0} Hz | Res: {} bits | Avg: {:.2}",
                self.demo.duty * 100.0,
                self.demo.quantized_duty * 100.0,
                self.demo.frequency,
                self.demo.resolution_bits,
                self.demo.avg
            ),
            margin,
            h - 12.0,
        );
    }

    fn draw_bool_timeline(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        history: &[bool],
        color: &str,
    ) {
        let ctx = self.canvas.ctx();

        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(x, y, width, height);

        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, width, height);

        if history.is_empty() {
            return;
        }

        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        let py_high = y + 5.0;
        let py_low = y + height - 5.0;
        let len = history.len();
        let step = width / len as f64;

        let mut prev = history[0];
        ctx.move_to(x, if prev { py_high } else { py_low });

        for (i, &state) in history.iter().enumerate() {
            let px = x + (i as f64) * step;
            if state != prev {
                let prev_py = if prev { py_high } else { py_low };
                ctx.line_to(px, prev_py);
                let curr_py = if state { py_high } else { py_low };
                ctx.line_to(px, curr_py);
            }
            prev = state;
        }

        ctx.line_to(x + width, if prev { py_high } else { py_low });
        ctx.stroke();
    }

    fn draw_float_timeline(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        history: &[f32],
        color: &str,
    ) {
        let ctx = self.canvas.ctx();

        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(x, y, width, height);

        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, width, height);

        if history.is_empty() {
            return;
        }

        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        let len = history.len();
        let step = width / (len.max(2) - 1) as f64;
        let py_top = y + 5.0;
        let py_bottom = y + height - 5.0;

        for (i, &v) in history.iter().enumerate() {
            let px = x + (i as f64) * step;
            let vv = (v as f64).clamp(0.0, 1.0);
            let py = py_bottom - vv * (py_bottom - py_top);
            if i == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }

        ctx.stroke();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ADC READING DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// ADC Reading demo runner
pub struct AdcReadingDemoRunner {
    demo: AdcReadingDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl AdcReadingDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = AdcReadingDemo::default();
        demo.reset(seed);

        let runner = AdcReadingDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        ADC_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            ADC_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        ADC_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        if let Ok(slider) = get_input("adc-bits-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("adc-bits-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        ADC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("bits", value);
                            }
                        });
                        update_text("adc-bits-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("adc-sample-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("adc-sample-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        ADC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("sample_rate", value);
                            }
                        });
                        update_text("adc-sample-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("adc-noise-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("adc-noise-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        ADC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("noise", value);
                            }
                        });
                        update_text("adc-noise-value", &format!("{:.2}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("adc-avg-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("adc-avg-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        ADC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("avg_window", value);
                            }
                        });
                        update_text("adc-avg-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("adc-att-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("adc-att-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        ADC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("attenuation", value);
                                let att = AdcAttenuation::from_index(value.round() as u8);
                                update_text(
                                    "adc-att-value",
                                    &match att {
                                        AdcAttenuation::Db0 => "0dB (~1.1V)".to_string(),
                                        AdcAttenuation::Db2p5 => "2.5dB (~1.5V)".to_string(),
                                        AdcAttenuation::Db6 => "6dB (~2.2V)".to_string(),
                                        AdcAttenuation::Db11 => "11dB (~3.3V)".to_string(),
                                    },
                                );
                            }
                        });
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Reset
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                ADC_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.demo.reset(seed);
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Pause
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pause-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                ADC_DEMO.with(|d| {
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

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let plot_x = margin;
        let plot_y = margin + 20.0;
        let plot_w = w - 2.0 * margin;
        let plot_h = h - plot_y - margin - 35.0;

        // Background + border
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(plot_x, plot_y, plot_w, plot_h);
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(plot_x, plot_y, plot_w, plot_h);

        // Grid
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.05)"));
        ctx.set_line_width(1.0);
        for i in 1..5 {
            let t = i as f64 / 5.0;
            ctx.begin_path();
            ctx.move_to(plot_x + t * plot_w, plot_y);
            ctx.line_to(plot_x + t * plot_w, plot_y + plot_h);
            ctx.stroke();
        }

        let vfs = self.demo.v_full_scale() as f64;
        let to_y = |v: f32| -> f64 {
            let vv = (v as f64).clamp(0.0, vfs);
            let pad = 6.0;
            (plot_y + plot_h - pad) - (vv / vfs) * (plot_h - 2.0 * pad)
        };

        let len = self.demo.analog_history.len().max(2);
        let step = plot_w / (len - 1) as f64;

        // Analog (true) line
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.22)"));
        ctx.set_line_width(1.0);
        ctx.begin_path();
        for (i, &v) in self.demo.analog_history.iter().enumerate() {
            let px = plot_x + (i as f64) * step;
            let py = to_y(v);
            if i == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }
        ctx.stroke();

        // Quantized line
        ctx.set_stroke_style(&JsValue::from_str("#ffaa44"));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for (i, &v) in self.demo.quantized_history.iter().enumerate() {
            let px = plot_x + (i as f64) * step;
            let py = to_y(v);
            if i == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }
        ctx.stroke();

        // Filtered line
        ctx.set_stroke_style(&JsValue::from_str("#44ff88"));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for (i, &v) in self.demo.filtered_history.iter().enumerate() {
            let px = plot_x + (i as f64) * step;
            let py = to_y(v);
            if i == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }
        ctx.stroke();

        // Sample points
        for (i, &v) in self.demo.sampled_history.iter().enumerate().step_by(2) {
            let px = plot_x + (i as f64) * step;
            let py = to_y(v);
            self.canvas
                .fill_circle(px, py, 2.0, "rgba(255, 170, 68, 0.45)");
        }

        // Labels
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text(
            "ADC: analog (gray), quantized (orange), filtered (green)",
            plot_x,
            margin + 10.0,
        );

        // Axis labels
        ctx.set_font("10px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#555"));
        let _ = ctx.fill_text(&format!("{:.1}V", vfs), plot_x - 28.0, plot_y + 10.0);
        let _ = ctx.fill_text("0V", plot_x - 20.0, plot_y + plot_h - 2.0);

        // Stats
        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let _ = ctx.fill_text(
            &format!(
                "Vfs:{:.1}V | bits:{} | code:{} | quant:{:.2}V | avg:{:.2}V",
                self.demo.v_full_scale(),
                self.demo.bits,
                self.demo.code,
                self.demo.quantized_v,
                self.demo.filtered_v
            ),
            margin,
            h - 12.0,
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// I2C BUS DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// I2C Bus demo runner
pub struct I2cBusDemoRunner {
    demo: I2cBusDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl I2cBusDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = I2cBusDemo::default();
        demo.reset(seed);

        let runner = I2cBusDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        I2C_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            I2C_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        I2C_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        if let Ok(slider) = get_input("i2c-addr-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("i2c-addr-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        let addr = value.round().clamp(8.0, 119.0) as u8;
                        I2C_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("address", addr as f32);
                            }
                        });
                        update_text("i2c-addr-value", &format!("0x{:02X}", addr));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("i2c-rw-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("i2c-rw-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        I2C_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("rw", value);
                            }
                        });
                        update_text("i2c-rw-value", if value >= 0.5 { "Read" } else { "Write" });
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("i2c-clock-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("i2c-clock-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        I2C_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("clock_khz", value);
                            }
                        });
                        update_text("i2c-clock-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("i2c-nak-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("i2c-nak-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        I2C_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("nak_chance", value);
                            }
                        });
                        update_text("i2c-nak-value", &format!("{:.2}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("i2c-stretch-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("i2c-stretch-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        I2C_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("stretch_chance", value);
                            }
                        });
                        update_text("i2c-stretch-value", &format!("{:.2}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Reset
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("reset-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                I2C_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
                        runner.demo.reset(seed);
                    }
                });
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Pause
        if let Some(btn) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pause-btn"))
        {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                I2C_DEMO.with(|d| {
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

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let timeline_height = 55.0;
        let gap = 35.0;

        let timeline_width = w - 2.0 * margin;
        let x = margin;

        // Labels
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("SCL", x, margin + 10.0);
        let _ = ctx.fill_text("SDA", x, margin + timeline_height + gap + 10.0);

        // Draw SCL
        let scl_y = margin + 25.0;
        self.draw_bool_timeline(
            x,
            scl_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.scl_history,
            "#44ff88",
        );

        // Draw SDA
        let sda_y = margin + timeline_height + gap + 25.0;
        self.draw_bool_timeline(
            x,
            sda_y,
            timeline_width,
            timeline_height - 15.0,
            &self.demo.sda_history,
            "#ffaa44",
        );

        // Status
        let phase = match self.demo.phase {
            I2cPhase::Idle => "IDLE",
            I2cPhase::Start => "START",
            I2cPhase::Bits => "BITS",
            I2cPhase::Ack => "ACK",
            I2cPhase::Stop => "STOP",
        };
        let stage = match self.demo.stage {
            I2cStage::Address => "ADDR",
            I2cStage::WriteData => "DATA(W)",
            I2cStage::ReadData => "DATA(R)",
        };
        let ack_txt = if self.demo.ack { "ACK" } else { "NACK" };

        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let _ = ctx.fill_text(
            &format!(
                "{} {} | addr 0x{:02X} {} | bit {} | {} | tx {}",
                phase,
                stage,
                self.demo.address,
                if self.demo.rw { "R" } else { "W" },
                self.demo.bit_index,
                ack_txt,
                self.demo.transactions
            ),
            margin,
            h - 12.0,
        );
    }

    fn draw_bool_timeline(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        history: &[bool],
        color: &str,
    ) {
        let ctx = self.canvas.ctx();

        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(x, y, width, height);

        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, width, height);

        if history.is_empty() {
            return;
        }

        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(2.0);
        ctx.begin_path();

        let py_high = y + 5.0;
        let py_low = y + height - 5.0;
        let len = history.len();
        let step = width / len as f64;

        let mut prev = history[0];
        ctx.move_to(x, if prev { py_high } else { py_low });

        for (i, &state) in history.iter().enumerate() {
            let px = x + (i as f64) * step;
            if state != prev {
                let prev_py = if prev { py_high } else { py_low };
                ctx.line_to(px, prev_py);
                let curr_py = if state { py_high } else { py_low };
                ctx.line_to(px, curr_py);
            }
            prev = state;
        }

        ctx.line_to(x + width, if prev { py_high } else { py_low });
        ctx.stroke();

        ctx.set_font("10px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#555"));
        let _ = ctx.fill_text("1", x - 12.0, py_high + 4.0);
        let _ = ctx.fill_text("0", x - 12.0, py_low + 4.0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// OHM'S LAW + POWER DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// Ohm's Law + Power demo runner
pub struct OhmsLawPowerDemoRunner {
    demo: OhmsLawPowerDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl OhmsLawPowerDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = OhmsLawPowerDemo::default();
        demo.reset(seed);

        let runner = OhmsLawPowerDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        OHMS_LAW_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            OHMS_LAW_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        OHMS_LAW_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        if let Ok(slider) = get_input("voltage-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("voltage-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        OHMS_LAW_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("voltage", value);
                            }
                        });
                        update_text("voltage-value", &format!("{:.1}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("resistance-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("resistance-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        OHMS_LAW_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("resistance", value);
                            }
                        });
                        update_text("resistance-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Reset and pause buttons (same pattern as other demos)
        wire_reset_pause_buttons("ohms-law")?;

        Ok(())
    }

    fn render(&mut self) {
        let ctx = self.canvas.ctx();
        let w = self.canvas.width();
        let h = self.canvas.height();

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let plot_x = margin;
        let plot_y = margin + 40.0;
        let plot_w = w - 2.0 * margin;
        let plot_h = h - plot_y - margin - 60.0;

        // Draw background
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(plot_x, plot_y, plot_w, plot_h);
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(plot_x, plot_y, plot_w, plot_h);

        // Draw current and power traces
        if !self.demo.current_history.is_empty() {
            let max_current = self
                .demo
                .current_history
                .iter()
                .cloned()
                .fold(0.1_f32, f32::max)
                .max(100.0);
            let max_power = self
                .demo
                .power_history
                .iter()
                .cloned()
                .fold(0.1_f32, f32::max)
                .max(500.0);

            // Current trace (top half)
            ctx.set_stroke_style(&JsValue::from_str("#44ff88"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            let step = plot_w / self.demo.current_history.len().max(1) as f64;
            let current_h = plot_h * 0.4;
            for (i, &current_ma) in self.demo.current_history.iter().enumerate() {
                let x = plot_x + (i as f64) * step;
                let y = plot_y + current_h - (current_ma / max_current) as f64 * current_h;
                if i == 0 {
                    ctx.move_to(x, y);
                } else {
                    ctx.line_to(x, y);
                }
            }
            ctx.stroke();

            // Power trace (bottom half)
            ctx.set_stroke_style(&JsValue::from_str("#ffaa44"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            let power_y_start = plot_y + plot_h * 0.5;
            let power_h = plot_h * 0.4;
            for (i, &power_mw) in self.demo.power_history.iter().enumerate() {
                let x = plot_x + (i as f64) * step;
                let y = power_y_start + power_h - (power_mw / max_power) as f64 * power_h;
                if i == 0 {
                    ctx.move_to(x, y);
                } else {
                    ctx.line_to(x, y);
                }
            }
            ctx.stroke();
        }

        // Labels and stats
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("Current (mA)", plot_x, plot_y - 5.0);
        let _ = ctx.fill_text("Power (mW)", plot_x, plot_y + plot_h * 0.5 - 5.0);

        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let current_color = if self.demo.current_exceeds_limit() {
            "#ff6644"
        } else {
            "#44ff88"
        };
        let power_color = if self.demo.power_exceeds_limit() {
            "#ff6644"
        } else {
            "#ffaa44"
        };

        ctx.set_fill_style(&JsValue::from_str(current_color));
        let _ = ctx.fill_text(
            &format!(
                "I = {:.3}A ({:.1}mA)",
                self.demo.current,
                self.demo.current * 1000.0
            ),
            margin,
            h - 50.0,
        );

        ctx.set_fill_style(&JsValue::from_str(power_color));
        let _ = ctx.fill_text(
            &format!(
                "P = {:.3}W ({:.1}mW)",
                self.demo.power,
                self.demo.power * 1000.0
            ),
            margin,
            h - 30.0,
        );

        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text(
            &format!(
                "V = {:.1}V, R = {:.0}Ω",
                self.demo.voltage, self.demo.resistance
            ),
            margin,
            h - 10.0,
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RC TIME CONSTANT DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// RC Time Constant demo runner
pub struct RcTimeConstantDemoRunner {
    demo: RcTimeConstantDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl RcTimeConstantDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = RcTimeConstantDemo::default();
        demo.reset(seed);

        let runner = RcTimeConstantDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        RC_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            RC_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        RC_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        if let Ok(slider) = get_input("rc-r-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("rc-r-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        RC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("resistance", value);
                            }
                        });
                        update_text("rc-r-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("rc-c-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("rc-c-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        RC_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("capacitance", value);
                            }
                        });
                        update_text("rc-c-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        wire_reset_pause_buttons("rc")?;

        Ok(())
    }

    fn render(&mut self) {
        let ctx = self.canvas.ctx();
        let w = self.canvas.width();
        let h = self.canvas.height();

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let plot_x = margin;
        let plot_y = margin + 40.0;
        let plot_w = w - 2.0 * margin;
        let plot_h = h - plot_y - margin - 60.0;

        // Draw background
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(plot_x, plot_y, plot_w, plot_h);
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(plot_x, plot_y, plot_w, plot_h);

        // Draw charging curve
        if !self.demo.voltage_history.is_empty() {
            let max_time = self
                .demo
                .time_history
                .iter()
                .cloned()
                .fold(0.1_f32, f32::max)
                .max(self.demo.tau * 5.0);
            let max_voltage = self.demo.v_final;

            ctx.set_stroke_style(&JsValue::from_str("#44ff88"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            let step = plot_w / self.demo.voltage_history.len().max(1) as f64;
            for (i, (&voltage, &_time)) in self
                .demo
                .voltage_history
                .iter()
                .zip(self.demo.time_history.iter())
                .enumerate()
            {
                let x = plot_x + (i as f64) * step;
                let y = plot_y + plot_h - (voltage / max_voltage) as f64 * plot_h;
                if i == 0 {
                    ctx.move_to(x, y);
                } else {
                    ctx.line_to(x, y);
                }
            }
            ctx.stroke();

            // Draw τ marker
            let tau_x = plot_x + (self.demo.tau / max_time) as f64 * plot_w;
            ctx.set_stroke_style(&JsValue::from_str("rgba(255, 170, 68, 0.5)"));
            ctx.set_line_width(1.0);
            ctx.set_line_dash(&js_sys::Array::of2(
                &JsValue::from(5.0),
                &JsValue::from(5.0),
            ))
            .ok();
            ctx.begin_path();
            ctx.move_to(tau_x, plot_y);
            ctx.line_to(tau_x, plot_y + plot_h);
            ctx.stroke();
            ctx.set_line_dash(&js_sys::Array::new()).ok();
        }

        // Labels and stats
        ctx.set_font("12px 'Inter', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#888"));
        let _ = ctx.fill_text("Voltage (V)", plot_x, plot_y - 5.0);
        let _ = ctx.fill_text("Time (s)", plot_x, plot_y + plot_h + 15.0);

        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let _ = ctx.fill_text(
            &format!(
                "τ = {:.2}s (R={:.0}Ω, C={:.0}µF)",
                self.demo.tau, self.demo.resistance, self.demo.capacitance
            ),
            margin,
            h - 30.0,
        );
        ctx.set_fill_style(&JsValue::from_str("#44ff88"));
        let _ = ctx.fill_text(
            &format!(
                "V = {:.2}V / {:.1}V ({:.0}%)",
                self.demo.voltage,
                self.demo.v_final,
                (self.demo.voltage / self.demo.v_final * 100.0)
            ),
            margin,
            h - 10.0,
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// POWER BUDGET DEMO RUNNER
// ═══════════════════════════════════════════════════════════════════════════════

/// Power Budget demo runner
pub struct PowerBudgetDemoRunner {
    demo: PowerBudgetDemo,
    canvas: Canvas,
    animation: Option<Rc<AnimationLoop>>,
    paused: bool,
}

impl PowerBudgetDemoRunner {
    pub fn start(canvas_id: &str, seed: u64) -> Result<(), JsValue> {
        let canvas = Canvas::new(canvas_id)?;
        let mut demo = PowerBudgetDemo::default();
        demo.reset(seed);

        let runner = PowerBudgetDemoRunner {
            demo,
            canvas,
            animation: None,
            paused: false,
        };

        POWER_BUDGET_DEMO.with(|d| {
            *d.borrow_mut() = Some(runner);
        });

        Self::start_animation()?;
        Self::wire_controls()?;
        Ok(())
    }

    fn start_animation() -> Result<(), JsValue> {
        let animation = AnimationLoop::new(move |dt| {
            POWER_BUDGET_DEMO.with(|d| {
                if let Some(runner) = d.borrow_mut().as_mut() {
                    if !runner.paused {
                        runner.demo.step(dt as f32);
                    }
                    runner.render();
                }
            });
        });

        animation.start();

        POWER_BUDGET_DEMO.with(|d| {
            if let Some(runner) = d.borrow_mut().as_mut() {
                runner.animation = Some(Rc::new(animation));
            }
        });

        Ok(())
    }

    fn wire_controls() -> Result<(), JsValue> {
        if let Ok(slider) = get_input("active-current-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("active-current-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        POWER_BUDGET_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("active_current", value);
                            }
                        });
                        update_text("active-current-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("active-time-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("active-time-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        POWER_BUDGET_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("active_time", value);
                            }
                        });
                        update_text("active-time-value", &format!("{:.1}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("sleep-current-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("sleep-current-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        POWER_BUDGET_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("sleep_current", value);
                            }
                        });
                        update_text("sleep-current-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        if let Ok(slider) = get_input("sleep-time-slider") {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(slider) = get_input("sleep-time-slider") {
                    if let Ok(value) = slider.value().parse::<f32>() {
                        POWER_BUDGET_DEMO.with(|d| {
                            if let Some(runner) = d.borrow_mut().as_mut() {
                                runner.demo.set_param("sleep_time", value);
                            }
                        });
                        update_text("sleep-time-value", &format!("{:.0}", value));
                    }
                }
            }) as Box<dyn FnMut(_)>);
            slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        wire_reset_pause_buttons("power-budget")?;

        Ok(())
    }

    fn render(&mut self) {
        let ctx = self.canvas.ctx();
        let w = self.canvas.width();
        let h = self.canvas.height();

        self.canvas.clear("#0a0a12");

        let margin = 30.0;
        let plot_x = margin;
        let plot_y = margin + 40.0;
        let plot_w = w - 2.0 * margin;
        let plot_h = h - plot_y - margin - 80.0;

        // Draw background
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.03)"));
        ctx.fill_rect(plot_x, plot_y, plot_w, plot_h);
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.1)"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(plot_x, plot_y, plot_w, plot_h);

        // Draw energy per cycle bar chart
        let max_energy = self.demo.energy_per_cycle.max(500.0);
        let bar_width = plot_w * 0.8;
        let bar_height = (self.demo.energy_per_cycle / max_energy) as f64 * plot_h * 0.6;
        let bar_x = plot_x + (plot_w - bar_width) / 2.0;
        let bar_y = plot_y + plot_h * 0.2;

        ctx.set_fill_style(&JsValue::from_str("#ffaa44"));
        ctx.fill_rect(
            bar_x,
            bar_y + plot_h * 0.6 - bar_height,
            bar_width,
            bar_height,
        );

        // Labels and stats
        ctx.set_font("14px 'Rajdhani', sans-serif");
        ctx.set_fill_style(&JsValue::from_str("#ffaa44"));
        let _ = ctx.fill_text("Energy per Cycle", plot_x, plot_y - 5.0);

        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.set_fill_style(&JsValue::from_str("#777"));
        let _ = ctx.fill_text(
            &format!(
                "Active: {:.0}mA × {:.1}s = {:.0}mAs",
                self.demo.active_current,
                self.demo.active_time,
                self.demo.active_current * self.demo.active_time
            ),
            margin,
            h - 70.0,
        );
        ctx.set_fill_style(&JsValue::from_str("#44ff88"));
        let _ = ctx.fill_text(
            &format!(
                "Sleep: {:.0}µA × {:.0}s = {:.1}mAs",
                self.demo.sleep_current,
                self.demo.sleep_time,
                (self.demo.sleep_current / 1000.0) * self.demo.sleep_time
            ),
            margin,
            h - 50.0,
        );
        ctx.set_fill_style(&JsValue::from_str("#ffaa44"));
        let _ = ctx.fill_text(
            &format!("Total: {:.1}mAs per cycle", self.demo.energy_per_cycle),
            margin,
            h - 30.0,
        );
        ctx.set_fill_style(&JsValue::from_str("#64ffda"));
        let _ = ctx.fill_text(
            &format!(
                "Battery: {:.0}mAh → {:.0} cycles → {:.0} days",
                self.demo.battery_capacity, self.demo.cycles, self.demo.lifetime_days
            ),
            margin,
            h - 10.0,
        );
    }
}

/// Helper to wire reset and pause buttons (shared pattern)
fn wire_reset_pause_buttons(prefix: &str) -> Result<(), JsValue> {
    // Reset button
    if let Some(btn) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("reset-btn"))
    {
        let prefix_clone = prefix.to_string();
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let seed = (js_sys::Math::random() * 1_000_000.0) as u64;
            match prefix_clone.as_str() {
                "ohms-law" => {
                    OHMS_LAW_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.reset(seed);
                        }
                    });
                }
                "rc" => {
                    RC_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.reset(seed);
                        }
                    });
                }
                "power-budget" => {
                    POWER_BUDGET_DEMO.with(|d| {
                        if let Some(runner) = d.borrow_mut().as_mut() {
                            runner.demo.reset(seed);
                        }
                    });
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Pause button
    if let Some(btn) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("pause-btn"))
    {
        let prefix_clone = prefix.to_string();
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let paused = match prefix_clone.as_str() {
                "ohms-law" => OHMS_LAW_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        runner.paused = !runner.paused;
                        runner.paused
                    } else {
                        false
                    }
                }),
                "rc" => RC_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        runner.paused = !runner.paused;
                        runner.paused
                    } else {
                        false
                    }
                }),
                "power-budget" => POWER_BUDGET_DEMO.with(|d| {
                    if let Some(runner) = d.borrow_mut().as_mut() {
                        runner.paused = !runner.paused;
                        runner.paused
                    } else {
                        false
                    }
                }),
                _ => false,
            };
            if let Some(btn) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("pause-btn"))
            {
                btn.set_text_content(Some(if paused { "▶ Play" } else { "⏸ Pause" }));
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

/// Stop any running ESP32 demo
pub fn stop_demo() {
    GPIO_DEBOUNCE_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    PWM_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    ADC_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    I2C_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    OHMS_LAW_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    RC_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });

    POWER_BUDGET_DEMO.with(|d| {
        if let Some(runner) = d.borrow().as_ref() {
            if let Some(animation) = &runner.animation {
                animation.stop();
            }
        }
        *d.borrow_mut() = None;
    });
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

fn update_text(id: &str, text: &str) {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
    {
        el.set_text_content(Some(text));
    }
}
