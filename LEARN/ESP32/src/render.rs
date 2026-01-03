//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | ESP32/src/render.rs
//! PURPOSE: DOM rendering for ESP32 lessons (intuition-first + interactive labs)
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → ESP32
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, GLOSSARY, DemoType};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

pub struct LessonRenderer {
    #[allow(dead_code)]
    document: Document,
    root: Element,
}

impl LessonRenderer {
    pub fn new(root_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let root = document
            .get_element_by_id(root_id)
            .ok_or("Root not found")?;

        Ok(Self { document, root })
    }

    pub fn render_home(&self, lessons: &[Lesson]) -> Result<(), JsValue> {
        let mut html = String::from(
            r#"
            <header class="hero">
                <h1>Electronics</h1>
                <p class="subtitle">From Basic Circuits to ESP32 Capstone • Learn by doing</p>
            </header>
            <section class="phase">
                <h2>Learning Path</h2>
                <p class="phase-intro">
                    Start with <strong>basic electronics</strong> (Ohm's Law, components), master <strong>microcontroller fundamentals</strong> \
                    (GPIO, PWM, ADC, I²C), dive into <strong>ESP32 specifics</strong> (deep sleep, Wi‑Fi), and finally build a \
                    <strong>battery-powered environmental monitor</strong> that runs for months.
                </p>
            </section>
        "#,
        );

        let card = |lesson: &Lesson| -> String {
            format!(
                r#"
                <div class="lesson-card" onclick="go_to_lesson({})">
                    <span class="lesson-icon">{}</span>
                    <h3>{}</h3>
                    <p class="lesson-subtitle">{}</p>
                </div>
            "#,
                lesson.id, lesson.icon, lesson.title, lesson.subtitle
            )
        };

        // Group lessons by phase
        let mut phases: std::collections::BTreeMap<&str, Vec<&Lesson>> = std::collections::BTreeMap::new();
        for lesson in lessons {
            phases.entry(lesson.phase).or_insert_with(Vec::new).push(lesson);
        }

        // Render each phase
        for (phase_name, phase_lessons) in phases {
            html.push_str(&format!(r#"<section class="phase"><h2>{}</h2><div class="lesson-grid">"#, phase_name));
            for lesson in phase_lessons {
                html.push_str(&card(lesson));
            }
            html.push_str(r#"</div></section>"#);
        }

        html.push_str(
            r#"
            <footer>
                <a href="https://too.foo">← back to too.foo</a>
            </footer>
        "#,
        );

        self.root.set_inner_html(&html);
        Ok(())
    }

    /// Apply glossary tooltips - wrap technical terms with tooltip spans.
    /// (Avoid touching Mermaid blocks, which must remain plain text.)
    fn apply_glossary(text: &str) -> String {
        fn apply_plain(mut s: String) -> String {
            for term in GLOSSARY {
                let pat = term.word.to_lowercase();
                let lower = s.to_lowercase();
                if let Some(pos) = lower.find(&pat) {
                    let original = &s[pos..pos + term.word.len()];
                    let tooltip = format!(
                        r#"<span class="term" data-tooltip="{}">{}</span>"#,
                        term.short, original
                    );
                    s = format!(
                        "{}{}{}",
                        &s[..pos],
                        tooltip,
                        &s[pos + term.word.len()..]
                    );
                }
            }
            s
        }

        let start_tag = r#"<div class="mermaid">"#;
        let end_tag = "</div>";
        let mut out = String::new();
        let mut rest = text;

        while let Some(start) = rest.find(start_tag) {
            let (before, after_start) = rest.split_at(start);
            out.push_str(&apply_plain(before.to_string()));

            // Copy Mermaid block verbatim until its closing </div>
            if let Some(end_rel) = after_start.find(end_tag) {
                let end = end_rel + end_tag.len();
                out.push_str(&after_start[..end]);
                rest = &after_start[end..];
            } else {
                out.push_str(after_start);
                rest = "";
                break;
            }
        }

        out.push_str(&apply_plain(rest.to_string()));
        out
    }

    pub fn render_lesson(&self, lesson: &Lesson) -> Result<(), JsValue> {
        let takeaways_html: String = lesson
            .key_takeaways
            .iter()
            .map(|t| format!(r#"<li class="takeaway-item">{}</li>"#, t))
            .collect::<Vec<_>>()
            .join("");

        // Demo controls for each lesson - match by demo_type and lesson ID
        let demo_controls = match (lesson.demo_type, lesson.id) {
            (DemoType::Canvas, 8) => {
                // GPIO Debounce (lesson 8)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Signal + Filter</h4>
                        <div class="control-row">
                            <label>Bounce Severity: <span id="bounce-value">0.50</span></label>
                            <input type="range" id="bounce-slider" min="0.1" max="1.0" step="0.05" value="0.50">
                        </div>
                        <div class="control-row">
                            <label>Sample Rate: <span id="sample-value">1000</span> Hz</label>
                            <input type="range" id="sample-slider" min="100" max="5000" step="100" value="1000">
                        </div>
                        <div class="control-row">
                            <label>Toggle Period: <span id="toggle-value">2.0</span> s</label>
                            <input type="range" id="toggle-slider" min="0.5" max="5.0" step="0.5" value="2.0">
                        </div>
                        <div class="control-row">
                            <label>Debounce Window: <span id="window-value">20</span> ms</label>
                            <input type="range" id="window-slider" min="5" max="100" step="5" value="20">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Playback</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Increase bounce severity. Then reduce the debounce window until false triggers return.
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 9) => {
                // PWM Control (lesson 9)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>PWM Parameters</h4>
                        <div class="control-row">
                            <label>Duty: <span id="duty-value">50</span>% (q <span id="quantized-duty-value">50.2</span>%)</label>
                            <input type="range" id="duty-slider" min="0" max="100" step="1" value="50">
                        </div>
                        <div class="control-row">
                            <label>Frequency: <span id="freq-value">500</span> Hz</label>
                            <input type="range" id="freq-slider" min="10" max="2000" step="10" value="500">
                        </div>
                        <div class="control-row">
                            <label>Resolution: <span id="res-value">8</span> bits</label>
                            <input type="range" id="res-slider" min="1" max="15" step="1" value="8">
                        </div>
                        <div class="control-row">
                            <label>Smoothing: <span id="tau-value">30</span> ms</label>
                            <input type="range" id="tau-slider" min="5" max="200" step="5" value="30">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Playback</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Increase frequency while keeping resolution high. Notice quantized duty steps widen as resolution drops.
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 10) => {
                // ADC Reading (lesson 10)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>ADC Parameters</h4>
                        <div class="control-row">
                            <label>Resolution: <span id="adc-bits-value">12</span> bits</label>
                            <input type="range" id="adc-bits-slider" min="6" max="12" step="1" value="12">
                        </div>
                        <div class="control-row">
                            <label>Sample Rate: <span id="adc-sample-value">120</span> Hz</label>
                            <input type="range" id="adc-sample-slider" min="5" max="500" step="5" value="120">
                        </div>
                        <div class="control-row">
                            <label>Noise: <span id="adc-noise-value">0.03</span> V</label>
                            <input type="range" id="adc-noise-slider" min="0" max="0.2" step="0.01" value="0.03">
                        </div>
                        <div class="control-row">
                            <label>Averaging: <span id="adc-avg-value">8</span> samples</label>
                            <input type="range" id="adc-avg-slider" min="1" max="64" step="1" value="8">
                        </div>
                        <div class="control-row">
                            <label>Attenuation: <span id="adc-att-value">11dB (~3.3V)</span></label>
                            <input type="range" id="adc-att-slider" min="0" max="3" step="1" value="3">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Playback</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Add noise, then increase averaging. Watch the filtered line stabilize.
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 11) => {
                // I²C Communication (lesson 11)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>I²C Parameters</h4>
                        <div class="control-row">
                            <label>Address: <span id="i2c-addr-value">0x3C</span></label>
                            <input type="range" id="i2c-addr-slider" min="8" max="119" step="1" value="60">
                        </div>
                        <div class="control-row">
                            <label>R/W: <span id="i2c-rw-value">Write</span></label>
                            <input type="range" id="i2c-rw-slider" min="0" max="1" step="1" value="0">
                        </div>
                        <div class="control-row">
                            <label>Clock: <span id="i2c-clock-value">100</span> kHz (slowed)</label>
                            <input type="range" id="i2c-clock-slider" min="10" max="400" step="10" value="100">
                        </div>
                        <div class="control-row">
                            <label>NAK chance: <span id="i2c-nak-value">0.00</span></label>
                            <input type="range" id="i2c-nak-slider" min="0" max="1" step="0.05" value="0">
                        </div>
                        <div class="control-row">
                            <label>Clock stretch: <span id="i2c-stretch-value">0.00</span></label>
                            <input type="range" id="i2c-stretch-slider" min="0" max="1" step="0.05" value="0">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Playback</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Increase NAK chance and see how a transaction aborts. Add clock stretching and watch SCL low extend.
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 2) => {
                // Ohm's Law + Power (lesson 2) - placeholder for new demo
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Ohm's Law Calculator</h4>
                        <div class="control-row">
                            <label>Voltage: <span id="voltage-value">3.3</span> V</label>
                            <input type="range" id="voltage-slider" min="0" max="12" step="0.1" value="3.3">
                        </div>
                        <div class="control-row">
                            <label>Resistance: <span id="resistance-value">1000</span> Ω</label>
                            <input type="range" id="resistance-slider" min="10" max="10000" step="10" value="1000">
                        </div>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 5) => {
                // RC Time Constant (lesson 5) - placeholder for new demo
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>RC Parameters</h4>
                        <div class="control-row">
                            <label>Resistance: <span id="rc-r-value">10000</span> Ω</label>
                            <input type="range" id="rc-r-slider" min="1000" max="100000" step="1000" value="10000">
                        </div>
                        <div class="control-row">
                            <label>Capacitance: <span id="rc-c-value">100</span> µF</label>
                            <input type="range" id="rc-c-slider" min="1" max="1000" step="1" value="100">
                        </div>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Canvas, 19) => {
                // Power Budget (lesson 19) - placeholder for new demo
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Power Budget</h4>
                        <div class="control-row">
                            <label>Active Current: <span id="active-current-value">80</span> mA</label>
                            <input type="range" id="active-current-slider" min="10" max="200" step="5" value="80">
                        </div>
                        <div class="control-row">
                            <label>Active Time: <span id="active-time-value">3</span> s</label>
                            <input type="range" id="active-time-slider" min="0.5" max="10" step="0.5" value="3">
                        </div>
                        <div class="control-row">
                            <label>Sleep Current: <span id="sleep-current-value">10</span> µA</label>
                            <input type="range" id="sleep-current-slider" min="1" max="100" step="1" value="10">
                        </div>
                        <div class="control-row">
                            <label>Sleep Time: <span id="sleep-time-value">297</span> s</label>
                            <input type="range" id="sleep-time-slider" min="10" max="600" step="1" value="297">
                        </div>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Calculator, _) => {
                // Calculator-style demo (voltage divider, etc.)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Calculator</h4>
                        <p>Use the interactive calculator above to explore different values.</p>
                    </div>
                </div>
                "#.to_string()
            }
            (DemoType::Static, _) => {
                // No demo controls for static lessons
                String::new()
            }
            _ => String::new(),
        };

        // Canvas visibility based on demo_type
        let canvas_html = match lesson.demo_type {
            DemoType::Canvas => r#"
                    <section class="visualization">
                        <h3>🎮 Try It Yourself</h3>
                        <canvas id="lesson-canvas" width="800" height="450"></canvas>
                        <div class="demo-explanation">
                            <p>{demo_explanation}</p>
                        </div>
                        {controls}
                    </section>
            "#,
            DemoType::Calculator => r#"
                    <section class="visualization">
                        <h3>🧮 Interactive Calculator</h3>
                        <div class="calculator-widget">
                            <p>Use the calculator below to explore different values.</p>
                            <div class="demo-explanation">
                                <p>{demo_explanation}</p>
                            </div>
                            {controls}
                        </div>
                    </section>
            "#,
            DemoType::Static => r#"
                    <section class="visualization">
                        <div class="demo-explanation">
                            <p>{demo_explanation}</p>
                        </div>
                    </section>
            "#,
        };

        let intuition_html = Self::apply_glossary(lesson.intuition);

        let canvas_html_filled = canvas_html
            .replace("{demo_explanation}", lesson.demo_explanation)
            .replace("{controls}", &demo_controls);

        let html = format!(
            r#"
            <article class="lesson-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">← All Lessons</button>
                </nav>

                <header class="lesson-header">
                    <span class="lesson-icon-large">{icon}</span>
                    <div>
                        <h1>{title}</h1>
                        <p class="subtitle">{subtitle}</p>
                    </div>
                </header>

                <div class="lesson-content">
                    <!-- 1. Why It Matters (Hook) -->
                    <section class="why-it-matters">
                        <p class="hook">{why_it_matters}</p>
                    </section>

                    <!-- 2. Intuition + Lab -->
                    <section class="intuition">
                        <h3>💡 The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    <!-- 3. Interactive Demo (conditional) -->
                    {canvas_html_filled}

                    <!-- 4. Key Takeaways -->
                    <section class="takeaways">
                        <h3>📝 Key Takeaways</h3>
                        <ul class="takeaway-list">{takeaways}</ul>
                    </section>

                    <!-- 5. Going Deeper -->
                    <details class="going-deeper">
                        <summary><h3>🔬 Going Deeper</h3></summary>
                        <div class="going-deeper-content">{going_deeper}</div>
                    </details>

                    <!-- 6. Math Details -->
                    <details class="math-details">
                        <summary><h3>📐 Timing / Details</h3></summary>
                        <pre class="math-text">{math_details}</pre>
                    </details>

                    <!-- 7. Implementation Guide -->
                    <details class="implementation-guide">
                        <summary><h3>💻 Implementation Guide</h3></summary>
                        <div class="impl-content">{implementation}</div>
                    </details>
                </div>

                <nav class="lesson-footer">
                    {prev_btn}
                    {next_btn}
                </nav>
            </article>
        "#,
            icon = lesson.icon,
            title = lesson.title,
            subtitle = lesson.subtitle,
            why_it_matters = lesson.why_it_matters,
            intuition = intuition_html,
            canvas_html_filled = canvas_html_filled,
            takeaways = takeaways_html,
            going_deeper = lesson.going_deeper,
            math_details = lesson.math_details,
            implementation = lesson.implementation,
            prev_btn = if lesson.id > 0 {
                format!(
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">← Previous</button>"#,
                    lesson.id - 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
            next_btn = if lesson.id < 20 {
                format!(
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">Next →</button>"#,
                    lesson.id + 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
        );

        self.root.set_inner_html(&html);

        // Trigger Mermaid rendering (if present)
        if let Some(window) = web_sys::window() {
            if let Ok(run_mermaid) = js_sys::Reflect::get(&window, &"runMermaid".into()) {
                if let Ok(func) = run_mermaid.dyn_into::<js_sys::Function>() {
                    let _ = func.call0(&JsValue::NULL);
                }
            }
            // Trigger KaTeX rendering (if present)
            if let Ok(render_katex) = js_sys::Reflect::get(&window, &"renderKaTeX".into()) {
                if let Ok(func) = render_katex.dyn_into::<js_sys::Function>() {
                    let _ = func.call0(&JsValue::NULL);
                }
            }
        }

        Ok(())
    }
}
