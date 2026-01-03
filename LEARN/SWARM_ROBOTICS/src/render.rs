//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | SWARM_ROBOTICS/src/render.rs
//! PURPOSE: DOM rendering for Swarm Robotics lessons with intuition-first layout
//! MODIFIED: 2025-01-XX
//! LAYER: LEARN → SWARM_ROBOTICS
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, PHASES};
use wasm_bindgen::prelude::*;
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
                <h1>Swarm Robotics</h1>
                <p class="subtitle">From Local Rules to Coordinated Swarms - An Interactive Journey</p>
                <p class="lesson-count">20 Lessons - 7 Phases - Beginner to Advanced</p>
            </header>
        "#,
        );

        // Render lessons grouped by phase
        for phase in PHASES.iter() {
            let phase_lessons: Vec<&Lesson> = lessons.iter().filter(|l| l.phase == *phase).collect();

            if phase_lessons.is_empty() {
                continue;
            }

            // Determine phase icon
            let phase_icon = match *phase {
                "Welcome to Swarms" => "🐜",
                "Local Rules → Emergent Motion" => "🐦",
                "Consensus (The Backbone)" => "🤝",
                "Coordinated Motion & Formations" => "🔷",
                "Task Allocation" => "📦",
                "Coverage & Exploration" => "🗺️",
                "Robustness & Capstone" => "🛡️",
                _ => "📚",
            };

            html.push_str(&format!(
                r#"
                <section class="phase">
                    <h2>{} {}</h2>
                    <div class="lesson-grid">
            "#,
                phase_icon, phase
            ));

            for lesson in phase_lessons {
                html.push_str(&format!(
                    r#"
                    <div class="lesson-card" onclick="go_to_lesson({})">
                        <span class="lesson-icon">{}</span>
                        <h3>{}</h3>
                        <p class="lesson-subtitle">{}</p>
                        <span class="badge badge-interactive">Interactive</span>
                    </div>
                "#,
                    lesson.id, lesson.icon, lesson.title, lesson.subtitle
                ));
            }

            html.push_str(
                r#"
                    </div>
                </section>
            "#,
            );
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

    pub fn render_lesson(&self, lesson: &Lesson) -> Result<(), JsValue> {
        // Build key takeaways list
        let takeaways_html: String = lesson
            .key_takeaways
            .iter()
            .map(|t| format!(r#"<li class="takeaway-item">{}</li>"#, t))
            .collect::<Vec<_>>()
            .join("");

        // Demo controls for each lesson
        let demo_controls = match lesson.id {
            0 | 1 | 3 => {
                // Boids demo controls
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Boids Parameters</h4>
                        <div class="control-row">
                            <label>Agents: <span id="num_agents-value">120</span></label>
                            <input type="range" id="num_agents-slider" min="20" max="400" step="10" value="120">
                        </div>
                        <div class="control-row">
                            <label>Neighbor Radius: <span id="neighbor_radius-value">0.12</span></label>
                            <input type="range" id="neighbor_radius-slider" min="0.03" max="0.25" step="0.01" value="0.12">
                        </div>
                        <div class="control-row">
                            <label>Separation: <span id="k_sep-value">1.4</span></label>
                            <input type="range" id="k_sep-slider" min="0" max="3" step="0.1" value="1.4">
                        </div>
                        <div class="control-row">
                            <label>Alignment: <span id="k_ali-value">1.0</span></label>
                            <input type="range" id="k_ali-slider" min="0" max="3" step="0.1" value="1.0">
                        </div>
                        <div class="control-row">
                            <label>Cohesion: <span id="k_coh-value">0.8</span></label>
                            <input type="range" id="k_coh-slider" min="0" max="3" step="0.1" value="0.8">
                        </div>
                        <div class="control-row">
                            <label>Obstacle Avoidance: <span id="k_obs-value">2.0</span></label>
                            <input type="range" id="k_obs-slider" min="0" max="6" step="0.2" value="2.0">
                        </div>
                        <div class="control-row">
                            <label>Max Speed: <span id="v_max-value">0.35</span></label>
                            <input type="range" id="v_max-slider" min="0.05" max="1.0" step="0.05" value="0.35">
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
                        <strong>Try this:</strong> Turn off separation (set to 0) to see collisions! 
                        Turn off alignment to see chaos. All three together create beautiful flocking.
                    </div>
                </div>
                "#
                .to_string()
            }
            _ => {
                // Generic controls for other demos
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Simulation Controls</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">🔄 Reset</button>
                            <button id="pause-btn" class="demo-btn">⏸ Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Coming Soon:</strong> Demo implementation in progress!
                    </div>
                </div>
                "#
                .to_string()
            }
        };

        let html = format!(
            r#"
            <article class="lesson-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">← All Lessons</button>
                </nav>

                <header class="lesson-header">
                    <span class="lesson-icon-large">{icon}</span>
                    <div>
                        <span class="phase-badge">{phase}</span>
                        <h1>{title}</h1>
                        <p class="subtitle">{subtitle}</p>
                    </div>
                </header>

                <div class="lesson-content">
                    <!-- 1. Why It Matters (Hook) -->
                    <section class="why-it-matters">
                        <p class="hook">{why_it_matters}</p>
                    </section>

                    <!-- 2. Intuition (No jargon, builds understanding) -->
                    <section class="intuition">
                        <h3>💡 The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    <!-- 3. Interactive Demo -->
                    <section class="visualization">
                        <div class="visualization-header">
                            <h3>🎮 Try It Yourself</h3>
                        </div>
                        <canvas id="lesson-canvas" width="1600" height="1000"></canvas>
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

                    <!-- 5. Going Deeper (Expandable) -->
                    <details class="going-deeper">
                        <summary><h3>🔬 Going Deeper</h3></summary>
                        <p>{going_deeper}</p>
                    </details>

                    <!-- 6. Math Details (Hidden by default) -->
                    <details class="math-details">
                        <summary><h3>📐 Mathematical Details</h3></summary>
                        <div class="math-content">{math_details}</div>
                    </details>

                    <!-- 7. Implementation Guide (Hidden by default) -->
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
            phase = lesson.phase,
            title = lesson.title,
            subtitle = lesson.subtitle,
            why_it_matters = lesson.why_it_matters,
            intuition = lesson.intuition,
            demo_explanation = lesson.demo_explanation,
            controls = demo_controls,
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
            next_btn = if lesson.id < 19 {
                format!(
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">Next →</button>"#,
                    lesson.id + 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
        );

        self.root.set_inner_html(&html);

        // Trigger Mermaid rendering
        if let Some(window) = web_sys::window() {
            if let Ok(run_mermaid) = js_sys::Reflect::get(&window, &"runMermaid".into()) {
                if let Ok(func) = run_mermaid.dyn_into::<js_sys::Function>() {
                    let _ = func.call0(&JsValue::NULL);
                }
            }
        }

        // Trigger KaTeX rendering
        if let Some(window) = web_sys::window() {
            if let Ok(render_katex) = js_sys::Reflect::get(&window, &"renderKaTeX".into()) {
                if let Ok(func) = render_katex.dyn_into::<js_sys::Function>() {
                    let _ = func.call0(&JsValue::NULL);
                }
            }
        }

        Ok(())
    }
}

