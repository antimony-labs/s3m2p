//! ===============================================================================
//! FILE: render.rs | DATA_STRUCTURES/src/render.rs
//! PURPOSE: DOM rendering for Data Structures lessons with intuition-first layout
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> DATA_STRUCTURES
//! ===============================================================================

use crate::lessons::{Lesson, GLOSSARY};
use learn_core::demos::problems::{Problem, ALL_PATTERNS};
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

    /// Render the section selector header
    fn render_section_selector(&self, active_section: usize) -> String {
        let learn_class = if active_section == 0 { "section-tab active" } else { "section-tab" };
        let practice_class = if active_section == 1 { "section-tab active" } else { "section-tab" };

        format!(
            r#"
            <header class="hero">
                <h1>Data Structures</h1>
                <p class="subtitle">From Arrays to Balanced Trees - Build Your Foundation</p>
                <div class="section-selector">
                    <button class="{learn_class}" onclick="go_to_section(0)">
                        <span class="tab-icon">📚</span>
                        <span class="tab-label">Learn</span>
                        <span class="tab-count">10 Lessons</span>
                    </button>
                    <button class="{practice_class}" onclick="go_to_section(1)">
                        <span class="tab-icon">🎯</span>
                        <span class="tab-label">Practice</span>
                        <span class="tab-count">40 Problems</span>
                    </button>
                </div>
            </header>
        "#,
            learn_class = learn_class,
            practice_class = practice_class,
        )
    }

    pub fn render_home(&self, lessons: &[Lesson]) -> Result<(), JsValue> {
        let mut html = self.render_section_selector(0);

        // Phase 1: Linear Structures
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">[]</span> Phase 1: Linear Structures</h2>
                <p class="phase-intro">Master the fundamentals: contiguous memory, pointers, and LIFO/FIFO access patterns.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id <= 3) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 2: Tree Structures
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">/\</span> Phase 2: Tree Structures</h2>
                <p class="phase-intro">Hierarchical organization for efficient search and ordering.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 4 && l.id <= 6) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        // Phase 3: Advanced Structures
        html.push_str(
            r#"
            <section class="phase">
                <h2><span class="phase-icon">#</span> Phase 3: Advanced Structures</h2>
                <p class="phase-intro">Hash tables, graphs, and self-balancing trees for complex problems.</p>
                <div class="lesson-grid">
        "#,
        );
        for lesson in lessons.iter().filter(|l| l.id >= 7) {
            html.push_str(&self.render_lesson_card(lesson));
        }
        html.push_str("</div></section>");

        html.push_str(
            r#"
            <footer>
                <a href="https://too.foo">back to too.foo</a>
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
                    <button onclick="go_home()" class="back-btn">All Lessons</button>
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
                        <h3>The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    <!-- 3. Interactive Demo -->
                    <section class="visualization">
                        <div class="visualization-header">
                            <h3>Try It Yourself</h3>
                            <button class="popout-btn" onclick="toggleDemoPopout()">Pop Out</button>
                        </div>
                        <canvas id="lesson-canvas" width="1600" height="1000"></canvas>
                        <div class="demo-explanation">
                            <p>{demo_explanation}</p>
                        </div>
                        {controls}
                    </section>

                    <!-- 4. Key Takeaways -->
                    <section class="takeaways">
                        <h3>Key Takeaways</h3>
                        <ul class="takeaway-list">{takeaways}</ul>
                    </section>

                    <!-- 5. Going Deeper (Expandable) -->
                    <details class="going-deeper">
                        <summary><h3>Going Deeper</h3></summary>
                        <p>{going_deeper}</p>
                    </details>

                    <!-- 6. Math Details (Hidden by default) -->
                    <details class="math-details">
                        <summary><h3>Complexity Analysis</h3></summary>
                        <div class="math-content">{math_details}</div>
                    </details>

                    <!-- 7. Implementation Guide (Hidden by default) -->
                    <details class="implementation-guide">
                        <summary><h3>Implementation Guide</h3></summary>
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
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">Previous</button>"#,
                    lesson.id - 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
            next_btn = if lesson.id < total_lessons - 1 {
                format!(
                    r#"<button onclick="go_to_lesson({})" class="nav-btn">Next</button>"#,
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
                // Arrays
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value to insert: <span id="value-display">5</span></label>
                            <input type="range" id="value-slider" min="0" max="99" step="1" value="5">
                        </div>
                        <div class="control-row">
                            <label>Index: <span id="index-display">0</span></label>
                            <input type="range" id="index-slider" min="0" max="9" step="1" value="0">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="access-btn" class="demo-btn primary">Access</button>
                            <button id="insert-btn" class="demo-btn">Insert</button>
                            <button id="delete-btn" class="demo-btn">Delete</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Access is instant (O(1)). Insert and delete cause elements to shift.
                    </div>
                </div>
                "#.to_string()
            }
            1 => {
                // Linked Lists
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">5</span></label>
                            <input type="range" id="value-slider" min="0" max="99" step="1" value="5">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="insert-head-btn" class="demo-btn primary">Insert at Head</button>
                            <button id="insert-tail-btn" class="demo-btn">Insert at Tail</button>
                            <button id="delete-btn" class="demo-btn">Delete Head</button>
                            <button id="search-btn" class="demo-btn">Search</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Notice how insertion only changes pointers - no shifting required!
                    </div>
                </div>
                "#.to_string()
            }
            2 => {
                // Stacks
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">5</span></label>
                            <input type="range" id="value-slider" min="0" max="99" step="1" value="5">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="push-btn" class="demo-btn primary">Push</button>
                            <button id="pop-btn" class="demo-btn">Pop</button>
                            <button id="peek-btn" class="demo-btn">Peek</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Push adds to top, pop removes from top. Last in, first out!
                    </div>
                </div>
                "#.to_string()
            }
            3 => {
                // Queues
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">5</span></label>
                            <input type="range" id="value-slider" min="0" max="99" step="1" value="5">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="enqueue-btn" class="demo-btn primary">Enqueue</button>
                            <button id="dequeue-btn" class="demo-btn">Dequeue</button>
                            <button id="peek-btn" class="demo-btn">Peek</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Enqueue adds to back, dequeue removes from front. First in, first out!
                    </div>
                </div>
                "#.to_string()
            }
            4 => {
                // Binary Trees
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Insert Node</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">50</span></label>
                            <input type="range" id="value-slider" min="1" max="99" step="1" value="50">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Traversals</h4>
                        <div class="control-buttons">
                            <button id="insert-btn" class="demo-btn">Insert</button>
                            <button id="preorder-btn" class="demo-btn">Pre-order</button>
                            <button id="inorder-btn" class="demo-btn primary">In-order</button>
                            <button id="postorder-btn" class="demo-btn">Post-order</button>
                            <button id="levelorder-btn" class="demo-btn">Level-order</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Watch different traversals visit nodes in different orders.
                    </div>
                </div>
                "#.to_string()
            }
            5 => {
                // BST
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">50</span></label>
                            <input type="range" id="value-slider" min="1" max="99" step="1" value="50">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="insert-btn" class="demo-btn primary">Insert</button>
                            <button id="search-btn" class="demo-btn">Search</button>
                            <button id="delete-btn" class="demo-btn">Delete</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Insert sequential values to see how BST can become unbalanced.
                    </div>
                </div>
                "#.to_string()
            }
            6 => {
                // Heaps
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">50</span></label>
                            <input type="range" id="value-slider" min="1" max="99" step="1" value="50">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="insert-btn" class="demo-btn primary">Insert</button>
                            <button id="extract-btn" class="demo-btn">Extract</button>
                            <button id="toggle-type-btn" class="demo-btn">Toggle Min/Max</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Watch elements bubble up on insert and sink down on extract.
                    </div>
                </div>
                "#.to_string()
            }
            7 => {
                // Hash Tables
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Key:</label>
                            <input type="text" id="key-input" class="demo-select" value="hello" style="max-width: 150px">
                        </div>
                        <div class="control-row">
                            <label>Value: <span id="value-display">42</span></label>
                            <input type="range" id="value-slider" min="0" max="99" step="1" value="42">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="insert-btn" class="demo-btn primary">Insert</button>
                            <button id="search-btn" class="demo-btn">Search</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Insert keys that hash to the same bucket to see collision handling.
                    </div>
                </div>
                "#.to_string()
            }
            8 => {
                // Graphs
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Traversal</h4>
                        <div class="control-row">
                            <label>Start vertex: <span id="vertex-display">0</span></label>
                            <input type="range" id="vertex-slider" min="0" max="5" step="1" value="0">
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="bfs-btn" class="demo-btn primary">BFS</button>
                            <button id="dfs-btn" class="demo-btn">DFS</button>
                            <button id="step-btn" class="demo-btn">Step</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> BFS explores level-by-level; DFS goes as deep as possible first.
                    </div>
                </div>
                "#.to_string()
            }
            9 => {
                // Balanced Trees
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Operations</h4>
                        <div class="control-row">
                            <label>Value: <span id="value-display">50</span></label>
                            <input type="range" id="value-slider" min="1" max="99" step="1" value="50">
                        </div>
                        <div class="control-row">
                            <select id="tree-type-select" class="demo-select">
                                <option value="0" selected>AVL Tree</option>
                                <option value="1">Red-Black Tree</option>
                            </select>
                        </div>
                    </div>
                    <div class="control-group">
                        <h4>Actions</h4>
                        <div class="control-buttons">
                            <button id="insert-btn" class="demo-btn primary">Insert</button>
                            <button id="insert-seq-btn" class="demo-btn">Insert 1-10</button>
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Try this:</strong> Insert sequential values - watch rotations keep the tree balanced!
                    </div>
                </div>
                "#.to_string()
            }
            _ => {
                r#"
                <div class="demo-controls" id="demo-controls">
                    <div class="control-group">
                        <h4>Interactive Demo</h4>
                        <div class="control-buttons">
                            <button id="reset-btn" class="demo-btn">Reset</button>
                        </div>
                    </div>
                    <div class="demo-hint">
                        <strong>Coming soon:</strong> Interactive visualization for this lesson.
                    </div>
                </div>
                "#.to_string()
            }
        }
    }

    // =========================================================================
    // PRACTICE SECTION RENDERING
    // =========================================================================

    pub fn render_practice_home(&self, problems: &[Problem]) -> Result<(), JsValue> {
        let mut html = self.render_section_selector(1);

        // Render problems grouped by pattern
        for pattern in ALL_PATTERNS {
            let pattern_problems: Vec<_> = problems.iter()
                .filter(|p| p.pattern == *pattern)
                .collect();

            if pattern_problems.is_empty() {
                continue;
            }

            html.push_str(&format!(
                r#"
                <section class="phase">
                    <h2><span class="phase-icon">{}</span> {}</h2>
                    <div class="problem-grid">
                "#,
                pattern.icon(),
                pattern.label(),
            ));

            for problem in pattern_problems {
                html.push_str(&self.render_problem_card(problem));
            }

            html.push_str("</div></section>");
        }

        html.push_str(
            r#"
            <footer>
                <a href="https://too.foo">back to too.foo</a>
            </footer>
        "#,
        );

        self.root.set_inner_html(&html);
        Ok(())
    }

    fn render_problem_card(&self, problem: &Problem) -> String {
        let difficulty_class = match problem.difficulty {
            learn_core::demos::problems::Difficulty::Easy => "difficulty-easy",
            learn_core::demos::problems::Difficulty::Medium => "difficulty-medium",
            learn_core::demos::problems::Difficulty::Hard => "difficulty-hard",
        };

        format!(
            r#"
            <div class="problem-card" onclick="go_to_problem({id})">
                <div class="problem-header">
                    <span class="problem-number">#{num}</span>
                    <span class="difficulty-badge {diff_class}">{difficulty}</span>
                </div>
                <h3>{title}</h3>
                <p class="problem-pattern">{pattern}</p>
            </div>
        "#,
            id = problem.id,
            num = problem.id + 1,
            diff_class = difficulty_class,
            difficulty = problem.difficulty.label(),
            title = problem.title,
            pattern = problem.pattern.label(),
        )
    }

    pub fn render_problem(&self, problem: &Problem, total_problems: usize) -> Result<(), JsValue> {
        let difficulty_class = match problem.difficulty {
            learn_core::demos::problems::Difficulty::Easy => "difficulty-easy",
            learn_core::demos::problems::Difficulty::Medium => "difficulty-medium",
            learn_core::demos::problems::Difficulty::Hard => "difficulty-hard",
        };

        let html = format!(
            r#"
            <article class="lesson-view problem-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">All Problems</button>
                </nav>

                <header class="lesson-header problem-header">
                    <div class="problem-meta">
                        <span class="problem-number">#{num}</span>
                        <span class="difficulty-badge {diff_class}">{difficulty}</span>
                        <span class="pattern-badge">{pattern_icon} {pattern}</span>
                    </div>
                    <h1>{title}</h1>
                </header>

                <div class="lesson-content">
                    <!-- Problem Statement -->
                    <section class="problem-statement">
                        <h3>Problem</h3>
                        <p>{description}</p>
                    </section>

                    <!-- Interactive Visualization -->
                    <section class="visualization">
                        <div class="visualization-header">
                            <h3>Visualization</h3>
                        </div>
                        <canvas id="problem-canvas" width="1600" height="800"></canvas>
                        <div class="demo-controls" id="demo-controls">
                            <div class="control-group">
                                <div class="control-buttons">
                                    <button id="step-btn" class="demo-btn primary">Step</button>
                                    <button id="play-btn" class="demo-btn">Play</button>
                                    <button id="reset-btn" class="demo-btn">Reset</button>
                                </div>
                            </div>
                        </div>
                    </section>

                    <!-- Hint -->
                    <details class="hint-section">
                        <summary><h3>Hint</h3></summary>
                        <p class="hint-text">{hint}</p>
                    </details>

                    <!-- Complexity -->
                    <section class="complexity-section">
                        <h3>Complexity</h3>
                        <div class="complexity-grid">
                            <div class="complexity-item">
                                <span class="complexity-label">Time</span>
                                <span class="complexity-value">{time}</span>
                            </div>
                            <div class="complexity-item">
                                <span class="complexity-label">Space</span>
                                <span class="complexity-value">{space}</span>
                            </div>
                        </div>
                    </section>
                </div>

                <nav class="lesson-footer">
                    {prev_btn}
                    {next_btn}
                </nav>
            </article>
        "#,
            num = problem.id + 1,
            diff_class = difficulty_class,
            difficulty = problem.difficulty.label(),
            pattern_icon = problem.pattern.icon(),
            pattern = problem.pattern.label(),
            title = problem.title,
            description = problem.description,
            hint = problem.hint,
            time = problem.time_complexity,
            space = problem.space_complexity,
            prev_btn = if problem.id > 0 {
                format!(
                    r#"<button onclick="go_to_problem({})" class="nav-btn">Previous</button>"#,
                    problem.id - 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
            next_btn = if problem.id < total_problems - 1 {
                format!(
                    r#"<button onclick="go_to_problem({})" class="nav-btn">Next</button>"#,
                    problem.id + 1
                )
            } else {
                String::from(r#"<span></span>"#)
            },
        );

        self.root.set_inner_html(&html);
        Ok(())
    }
}
