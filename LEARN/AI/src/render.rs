//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | AI/src/render.rs
//! PURPOSE: DOM rendering for AI/ML lessons with intuition-first layout
//! MODIFIED: 2025-01-02
//! LAYER: LEARN → AI
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, GLOSSARY};
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
                <h1>AI & Machine Learning</h1>
                <p class="subtitle">From Neural Networks to Transformers - An Interactive Journey</p>
                <span class="lesson-count">16 Interactive Lessons</span>
            </header>
        "#,
        );

        // Phase 1: The Big Picture
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">🌍</span> Phase 1: The Big Picture</h2>
                <p class="phase-intro">Build intuition for what AI actually does - no math required.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id <= 2) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 2: Neural Network Foundations
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">🧠</span> Phase 2: Neural Network Foundations</h2>
                <p class="phase-intro">Understand how neural networks learn from data.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 3 && l.id <= 5) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 3: Specialized Architectures
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">🏗️</span> Phase 3: Specialized Architectures</h2>
                <p class="phase-intro">Explore architectures designed for images and sequences.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 6 && l.id <= 7) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 4: Modern Deep Learning
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">🚀</span> Phase 4: Modern Deep Learning</h2>
                <p class="phase-intro">The breakthroughs that power today's AI: attention, transformers, and scale.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 8 && l.id <= 10) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 5: Reinforcement Learning & Beyond
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">🎮</span> Phase 5: RL & The Future</h2>
                <p class="phase-intro">Learning from rewards, aligning AI with humans, and what's next.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 11) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

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

    fn render_lesson_card(&self, lesson: &Lesson) -> String {
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
    }

    /// Apply glossary tooltips - wrap technical terms with tooltip spans
    fn apply_glossary(text: &str) -> String {
        let mut result = text.to_string();
        for term in GLOSSARY {
            let pattern = term.word;
            if let Some(pos) = result.to_lowercase().find(&pattern.to_lowercase()) {
                let original = &result[pos..pos + pattern.len()];
                let tooltip = format!(
                    r#"<span class="term" data-tooltip="{}">{}</span>"#,
                    term.short, original
                );
                result = format!(
                    "{}{}{}",
                    &result[..pos],
                    tooltip,
                    &result[pos + pattern.len()..]
                );
            }
        }
        result
    }

    pub fn render_lesson(&self, lesson: &Lesson, total_lessons: usize) -> Result<(), JsValue> {
        // Build key takeaways list
        let takeaways_html: String = lesson
            .key_takeaways
            .iter()
            .map(|t| format!(r#"<li class="takeaway-item">{}</li>"#, t))
            .collect::<Vec<_>>()
            .join("");

        // Demo controls for each lesson
        let demo_controls = self.get_demo_controls(lesson.id);

        // Apply glossary tooltips to intuition text
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

                    <!-- 2. Intuition (No jargon, builds understanding) -->
                    <section class="intuition">
                        <h3>💡 The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    <!-- 3. Interactive Demo -->
                    <section class="visualization">
                        <div class="visualization-header">
                            <h3>🎮 Try It Yourself</h3>
                            <button class="popout-btn" onclick="toggleDemoPopout()">⛶ Pop Out</button>
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
            title = lesson.title,
            subtitle = lesson.subtitle,
            why_it_matters = lesson.why_it_matters,
            intuition = intuition_html,
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
            next_btn = if lesson.id < total_lessons - 1 {
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

    fn get_demo_controls(&self, lesson_id: usize) -> String {
        match lesson_id {
            0 => {
                // Lesson 0: What is AI? - Placeholder
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Pattern Recognition Game</h4>
                        <div class="demo-hint">
                            <strong>Coming soon:</strong> An interactive game where you classify patterns
                            and see how your brain does the same thing AI does.
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            1 => {
                // Lesson 1: Linear Regression with Gradient Descent
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Gradient Descent</h4>
                        <div class="control-row">
                            <label>Learning Rate (α): <span id="lr-value">0.10</span></label>
                            <input type="range" id="lr-slider" min="0.01" max="1.0" step="0.01" value="0.1">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                            <button id="train-btn" class="demo-btn primary">Train</button>
                            <button id="step-btn" class="demo-btn">Step</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Press Train to watch gradient descent find the best-fit line.
                        The dashed yellow line is the target; the green line is what the model learns.
                    </div>
                </div>
                "#.to_string()
            }
            2 => {
                // Lesson 2: Decision Boundaries - Placeholder
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Decision Boundaries</h4>
                        <div class="demo-hint">
                            <strong>Coming soon:</strong> Visualize how classifiers divide the feature space.
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            3 => {
                // Lesson 3: Perceptron with XOR Problem
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Dataset</h4>
                        <div class="control-row">
                            <label>Problem Type:</label>
                            <select id="dataset-select" class="demo-select">
                                <option value="0" selected>Linear</option>
                                <option value="1">XOR</option>
                                <option value="2">Circle</option>
                                <option value="3">Spiral</option>
                            </select>
                        </div>
                        <div class="control-row">
                            <label>
                                <input type="checkbox" id="hidden-layer-checkbox"> Use Hidden Layer (MLP)
                            </label>
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                            <button id="train-btn" class="demo-btn primary">Train</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Select XOR - a single perceptron fails! Enable the hidden layer
                        to see how a multi-layer network solves non-linear problems.
                    </div>
                </div>
                "#.to_string()
            }
            4 => {
                // Lesson 4: Neural Network Playground
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Architecture</h4>
                        <div class="control-row">
                            <label>Hidden Layers: <span id="layers-value">2</span></label>
                            <input type="range" id="layers-slider" min="1" max="4" step="1" value="2">
                        </div>
                        <div class="control-row">
                            <label>Neurons/Layer: <span id="neurons-value">4</span></label>
                            <input type="range" id="neurons-slider" min="2" max="8" step="1" value="4">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Dataset</h4>
                        <div class="control-row">
                            <select id="nn-dataset-select" class="demo-select">
                                <option value="0" selected>Circle</option>
                                <option value="1">XOR</option>
                                <option value="2">Gaussian</option>
                                <option value="3">Spiral</option>
                            </select>
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                            <button id="train-btn" class="demo-btn primary">Train</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Start with Spiral - watch the decision boundary evolve
                        as the network learns. Add more layers for complex patterns!
                    </div>
                </div>
                "#.to_string()
            }
            5 => {
                // Lesson 5: Backpropagation - Placeholder
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Gradient Flow</h4>
                        <div class="demo-hint">
                            <strong>Coming soon:</strong> Visualize how gradients flow backward through the network.
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            6 => {
                // Lesson 6: CNN Filter Visualization
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Filter Type</h4>
                        <div class="control-row">
                            <select id="filter-select" class="demo-select">
                                <option value="0">Edge (Horizontal)</option>
                                <option value="1">Edge (Vertical)</option>
                                <option value="2" selected>Edge (Sobel)</option>
                                <option value="3">Sharpen</option>
                                <option value="4">Blur (Box)</option>
                                <option value="5">Emboss</option>
                            </select>
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Input Pattern</h4>
                        <div class="control-row">
                            <select id="pattern-select" class="demo-select">
                                <option value="0" selected>Checkerboard</option>
                                <option value="1">Stripes</option>
                                <option value="2">Circle</option>
                                <option value="3">Gradient</option>
                                <option value="4">Noise</option>
                                <option value="5">Letter</option>
                            </select>
                        </div>
                        <div class="control-row">
                            <label>
                                <input type="checkbox" id="animate-checkbox" checked> Animate Sliding Window
                            </label>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Watch the green box slide across the input image.
                        Different filters detect different features - edge detection highlights boundaries!
                    </div>
                </div>
                "#.to_string()
            }
            7 => {
                // Lesson 7: RNNs - Placeholder
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Sequence Memory</h4>
                        <div class="demo-hint">
                            <strong>Coming soon:</strong> See how recurrent networks remember sequences.
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            8 => {
                // Lesson 8: Attention Mechanism
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Sentence</h4>
                        <div class="control-row">
                            <select id="sentence-select" class="demo-select">
                                <option value="0" selected>The cat sat on the mat</option>
                                <option value="1">I love machine learning</option>
                                <option value="2">Attention is all you need</option>
                            </select>
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Query Token</h4>
                        <div class="control-row">
                            <label>Select: <span id="query-value">The</span></label>
                            <input type="range" id="query-slider" min="0" max="5" step="1" value="0">
                        </div>
                        <div class="control-row">
                            <label>Temperature: <span id="temp-value">1.0</span></label>
                            <input type="range" id="temp-slider" min="0.1" max="3.0" step="0.1" value="1.0">
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Select different query tokens to see which other tokens
                        they attend to. Lower temperature = sharper attention, higher = more uniform.
                    </div>
                </div>
                "#.to_string()
            }
            9 | 10 => {
                // Lessons 9-10: Transformers / Scaling - Placeholders
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Advanced Topics</h4>
                        <div class="demo-hint">
                            <strong>Coming soon:</strong> Interactive visualization for transformers and scaling laws.
                        </div>
                    </div>
                </div>
                "#.to_string()
            }
            11 => {
                // Lesson 11: Grid World Q-Learning
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Environment</h4>
                        <div class="control-row">
                            <select id="layout-select" class="demo-select">
                                <option value="0" selected>Simple</option>
                                <option value="1">Maze</option>
                                <option value="2">Cliff Walk</option>
                                <option value="3">Four Rooms</option>
                            </select>
                        </div>
                        <div class="control-row">
                            <label>Learning Rate: <span id="lr-value">0.10</span></label>
                            <input type="range" id="lr-slider" min="0.01" max="0.5" step="0.01" value="0.1">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                            <button id="train-btn" class="demo-btn primary">Pause</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Watch the agent (orange) learn to find the goal (green).
                        Arrows show the learned policy. Blue shading = higher value estimates.
                    </div>
                </div>
                "#.to_string()
            }
            _ => {
                // Default placeholder for other lessons (12-15)
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Interactive Demo</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                            <button id="train-btn" class="demo-btn primary">Train</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Coming soon:</strong> Interactive visualization for this lesson.
                    </div>
                </div>
                "#
                .to_string()
            }
        }
    }
}
