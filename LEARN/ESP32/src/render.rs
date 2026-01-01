//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | ESP32/src/render.rs
//! PURPOSE: DOM rendering for ESP32 lessons (intuition-first + interactive labs)
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → ESP32
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, GLOSSARY};
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
                <h1>ESP32</h1>
                <p class="subtitle">ESP‑WROOM‑32 • Learn by doing (demos + labs)</p>
            </header>
            <section class="phase">
                <h2>Learning Path</h2>
                <p class="phase-intro">
                    The same 4 building blocks show up in almost every real ESP32 project:
                    <strong>clean inputs</strong> → <strong>PWM outputs</strong> → <strong>analog sensing</strong> → <strong>I²C peripherals</strong>.
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

        let find = |id: usize| lessons.iter().find(|l| l.id == id);

        html.push_str(r#"<section class="phase"><h2>Phase 1 — Digital I/O</h2><div class="lesson-grid">"#);
        if let Some(l) = find(0) {
            html.push_str(&card(l));
        }
        html.push_str(r#"</div></section>"#);

        html.push_str(r#"<section class="phase"><h2>Phase 2 — Timers & PWM (LEDC)</h2><div class="lesson-grid">"#);
        if let Some(l) = find(1) {
            html.push_str(&card(l));
        }
        html.push_str(r#"</div></section>"#);

        html.push_str(r#"<section class="phase"><h2>Phase 3 — Analog (ADC)</h2><div class="lesson-grid">"#);
        if let Some(l) = find(2) {
            html.push_str(&card(l));
        }
        html.push_str(r#"</div></section>"#);

        html.push_str(r#"<section class="phase"><h2>Phase 4 — Buses (I²C)</h2><div class="lesson-grid">"#);
        if let Some(l) = find(3) {
            html.push_str(&card(l));
        }
        html.push_str(r#"</div></section>"#);

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

        // Demo controls for each lesson (0=Debounce, 1=PWM, 2=ADC, 3=I2C)
        let demo_controls = match lesson.id {
            0 => {
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
            1 => {
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
            2 => {
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
            3 => {
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
            _ => String::new(),
        };

        let intuition_html = Self::apply_glossary(lesson.intuition);

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

                    <!-- 3. Interactive Demo -->
                    <section class="visualization">
                        <h3>🎮 Try It Yourself</h3>
                        <canvas id="lesson-canvas" width="800" height="450"></canvas>
                        <div class="demo-explanation">
                            <p>{demo_explanation}</p>
                        </div>
                        {controls}
                    </section>

                    <!-- 4. Key Takeaways -->
                    <section class="takeaways">
                        <h3>📝 Key Takeaways</h3>
                        <ul class="takeaway-list">{takeaways}</ul>
                    </section>

                    <!-- 5. Going Deeper -->
                    <details class="going-deeper">
                        <summary><h3>🔬 Going Deeper</h3></summary>
                        <p>{going_deeper}</p>
                    </details>

                    <!-- 6. Details -->
                    <details class="math-details">
                        <summary><h3>📐 Timing / Details</h3></summary>
                        <pre class="math-text">{math_details}</pre>
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
            demo_explanation = lesson.demo_explanation,
            controls = demo_controls,
            takeaways = takeaways_html,
            going_deeper = lesson.going_deeper,
            math_details = lesson.math_details,
            prev_btn = if lesson.id > 0 {
                format!(
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">← Previous</button>"#,
                    lesson.id - 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
            next_btn = if lesson.id < 3 {
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
        }

        Ok(())
    }
}
