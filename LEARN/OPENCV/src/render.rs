//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | OPENCV/src/render.rs
//! PURPOSE: HTML rendering for OpenCV lessons
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════

use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

use crate::lessons::{DemoType, Lesson, LESSONS, PHASES};

/// Renders lessons to the DOM
pub struct LessonRenderer {
    #[allow(dead_code)]
    document: Document,
    root: Element,
}

impl LessonRenderer {
    /// Create a new renderer targeting the given root element ID
    pub fn new(root_id: &str) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let root = document
            .get_element_by_id(root_id)
            .ok_or_else(|| JsValue::from_str(&format!("Root element '{}' not found", root_id)))?;
        Ok(Self { document, root })
    }

    /// Render the home page with all lessons
    pub fn render_home(&self) -> Result<(), JsValue> {
        let mut html = String::from(
            r##"
            <header class="hero">
                <h1>OpenCV Computer Vision</h1>
                <p class="subtitle">From Pixels to Perception — Interactive Tutorials</p>
                <p class="lesson-count">12 Lessons • 5 Phases • Beginner to Intermediate</p>
            </header>
        "##,
        );

        // Group lessons by phase
        for phase in PHASES.iter() {
            let phase_lessons: Vec<&Lesson> = LESSONS.iter().filter(|l| l.phase == *phase).collect();

            if phase_lessons.is_empty() {
                continue;
            }

            let phase_icon = match *phase {
                "The Big Picture" => "👁️",
                "Filtering & Enhancement" => "🔲",
                "Feature Detection" => "🎯",
                "Geometric Transforms" => "🔄",
                "Real-World Applications" => "🚀",
                _ => "📚",
            };

            html.push_str(&format!(
                r##"
                <section class="phase">
                    <h2>{} {}</h2>
                    <div class="lesson-grid">
            "##,
                phase_icon, phase
            ));

            for lesson in phase_lessons {
                let demo_badge = match lesson.demo_type {
                    DemoType::Camera => {
                        r##"<span class="badge badge-camera">📷 Camera</span>"##
                    }
                    DemoType::Canvas => {
                        r##"<span class="badge badge-canvas">🎨 Interactive</span>"##
                    }
                    DemoType::SideBySide => {
                        r##"<span class="badge badge-sidebyside">↔️ Compare</span>"##
                    }
                    DemoType::Static => r##"<span class="badge badge-static">📖 Theory</span>"##,
                };

                html.push_str(&format!(
                    r##"
                    <div class="lesson-card" onclick="go_to_lesson({})">
                        <span class="lesson-icon">{}</span>
                        <h3>{}</h3>
                        <p class="lesson-subtitle">{}</p>
                        {}
                    </div>
                "##,
                    lesson.id, lesson.icon, lesson.title, lesson.subtitle, demo_badge
                ));
            }

            html.push_str(r##"</div></section>"##);
        }

        self.root.set_inner_html(&html);
        Ok(())
    }

    /// Render a single lesson
    pub fn render_lesson(&self, lesson: &Lesson) -> Result<(), JsValue> {
        // Build concepts with tooltips
        let concepts_html: String = if lesson.concept_definitions.is_empty() {
            lesson
                .key_concepts
                .iter()
                .map(|c| format!(r##"<span class="concept">{}</span>"##, c))
                .collect::<Vec<_>>()
                .join("")
        } else {
            lesson
                .concept_definitions
                .iter()
                .map(|(term, def)| {
                    format!(
                        r##"<span class="concept" data-tooltip="{}" tabindex="0">{}</span>"##,
                        def, term
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        };

        // Progress navigation
        let progress_nav = self.render_lesson_progress(lesson.id);

        // Demo section based on type
        let demo_section = match lesson.demo_type {
            DemoType::Camera => {
                String::from(
                    r##"
                <section class="demo-section camera-demo">
                    <h3>Live Demo</h3>
                    <div class="demo-container">
                        <div class="video-container">
                            <video id="camera-video" autoplay playsinline muted></video>
                            <div id="camera-placeholder" class="camera-placeholder">
                                <div class="placeholder-content">
                                    <span class="camera-icon">📷</span>
                                    <p id="camera-status-text">Camera requires permission</p>
                                    <button class="permission-btn" id="camera-btn">Enable Camera</button>
                                </div>
                            </div>
                        </div>
                        <div class="canvas-container">
                            <canvas id="output-canvas"></canvas>
                            <p class="canvas-label">Processed Output</p>
                        </div>
                    </div>
                    <div class="demo-controls" id="demo-controls">
                        <!-- Controls populated by demo_runner -->
                    </div>
                </section>
                "##,
                )
            }
            DemoType::Canvas => {
                String::from(
                    r##"
                <section class="demo-section canvas-demo">
                    <h3>Interactive Demo</h3>
                    <div class="canvas-solo-container">
                        <canvas id="demo-canvas"></canvas>
                    </div>
                    <div class="demo-controls" id="demo-controls">
                        <!-- Controls populated by demo_runner -->
                    </div>
                </section>
                "##,
                )
            }
            DemoType::SideBySide => {
                String::from(
                    r##"
                <section class="demo-section sidebyside-demo">
                    <h3>Before & After</h3>
                    <div class="sidebyside-container">
                        <div class="canvas-container">
                            <canvas id="before-canvas"></canvas>
                            <p class="canvas-label">Original</p>
                        </div>
                        <div class="canvas-container">
                            <canvas id="after-canvas"></canvas>
                            <p class="canvas-label">Processed</p>
                        </div>
                    </div>
                    <div class="demo-controls" id="demo-controls">
                        <!-- Controls populated by demo_runner -->
                    </div>
                </section>
                "##,
                )
            }
            DemoType::Static => String::new(),
        };

        // Navigation buttons
        let prev_button = if lesson.id > 0 {
            format!(
                r##"<button class="nav-btn prev-btn" onclick="go_to_lesson({})">← Previous</button>"##,
                lesson.id - 1
            )
        } else {
            String::from(r##"<button class="nav-btn prev-btn" disabled>← Previous</button>"##)
        };

        let next_button = if lesson.id < LESSONS.len() - 1 {
            format!(
                r##"<button class="nav-btn next-btn" onclick="go_to_lesson({})">Next →</button>"##,
                lesson.id + 1
            )
        } else {
            String::from(r##"<button class="nav-btn next-btn" disabled>Next →</button>"##)
        };

        // Convert markdown content to HTML
        let content_html = convert_markdown_to_html(lesson.content);

        // Build complete lesson view
        let html = format!(
            r##"
            <article class="lesson-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">← All Lessons</button>
                    <span class="lesson-progress-text">{} / {}</span>
                </nav>

                {}

                <header class="lesson-header">
                    <span class="lesson-icon-large">{}</span>
                    <div>
                        <span class="phase-badge">{}</span>
                        <h1>{}</h1>
                        <p class="subtitle">{}</p>
                    </div>
                </header>

                <div class="lesson-content">
                    <section class="description">
                        <p class="lead">{}</p>
                    </section>

                    <section class="concepts">
                        <h3>Key Concepts</h3>
                        <div class="concept-list">{}</div>
                    </section>

                    {}

                    <section class="main-content">
                        {}
                    </section>
                </div>

                <nav class="lesson-footer">
                    {}
                    {}
                </nav>
            </article>
        "##,
            lesson.id + 1,
            LESSONS.len(),
            progress_nav,
            lesson.icon,
            lesson.phase,
            lesson.title,
            lesson.subtitle,
            lesson.description,
            concepts_html,
            demo_section,
            content_html,
            prev_button,
            next_button
        );

        self.root.set_inner_html(&html);
        Ok(())
    }

    /// Render progress bubbles for lesson navigation
    fn render_lesson_progress(&self, current_idx: usize) -> String {
        let total = LESSONS.len();
        let mut html = String::from(r##"<div class="lesson-progress-nav">"##);

        // Show max 9 bubbles (current ± 4)
        let start = current_idx.saturating_sub(4);
        let end = (current_idx + 5).min(total);

        if start > 0 {
            html.push_str(r##"<span class="progress-ellipsis">...</span>"##);
        }

        for i in start..end {
            let class = if i < current_idx {
                "progress-bubble completed"
            } else if i == current_idx {
                "progress-bubble current"
            } else {
                "progress-bubble future"
            };

            let lesson_title = LESSONS.get(i).map(|l| l.title).unwrap_or("Unknown");

            html.push_str(&format!(
                r##"<button class="{}" onclick="go_to_lesson({})" title="{}">
                    <span class="bubble-num">{}</span>
                </button>"##,
                class,
                i,
                lesson_title,
                i + 1
            ));

            if i < end - 1 {
                html.push_str(r##"<span class="progress-line"></span>"##);
            }
        }

        if end < total {
            html.push_str(r##"<span class="progress-ellipsis">...</span>"##);
        }

        html.push_str(r##"</div>"##);
        html
    }
}

/// Simple markdown to HTML converter
fn convert_markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_list = false;
    let mut in_table = false;

    for line in md.lines() {
        let trimmed = line.trim();

        // Code blocks with triple backticks
        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                let lang = trimmed.trim_start_matches("```");
                let lang_class = if lang.is_empty() {
                    String::new()
                } else {
                    format!(r##" class="language-{}""##, lang)
                };
                html.push_str(&format!("<pre><code{}>\n", lang_class));
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            let escaped = line
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            html.push_str(&escaped);
            html.push('\n');
            continue;
        }

        // Close list if we're not in a list item
        if in_list && !trimmed.starts_with("- ") && !trimmed.starts_with("* ") {
            html.push_str("</ul>\n");
            in_list = false;
        }

        // Close table if we're not in a table row
        if in_table && !trimmed.starts_with('|') {
            html.push_str("</tbody></table>\n");
            in_table = false;
        }

        // Horizontal rule
        if trimmed == "---" {
            html.push_str("<hr>\n");
            continue;
        }

        // Headers
        if trimmed.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", format_inline(&trimmed[3..])));
            continue;
        }
        if trimmed.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", format_inline(&trimmed[4..])));
            continue;
        }

        // Blockquotes
        if trimmed.starts_with("> ") {
            html.push_str(&format!(
                "<blockquote>{}</blockquote>\n",
                format_inline(&trimmed[2..])
            ));
            continue;
        }

        // Tables
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            // Skip separator rows
            if trimmed.contains("---") {
                continue;
            }

            if !in_table {
                html.push_str("<table><thead><tr>");
                let cells: Vec<&str> = trimmed
                    .trim_matches('|')
                    .split('|')
                    .map(|s| s.trim())
                    .collect();
                for cell in cells {
                    html.push_str(&format!("<th>{}</th>", format_inline(cell)));
                }
                html.push_str("</tr></thead><tbody>\n");
                in_table = true;
            } else {
                html.push_str("<tr>");
                let cells: Vec<&str> = trimmed
                    .trim_matches('|')
                    .split('|')
                    .map(|s| s.trim())
                    .collect();
                for cell in cells {
                    html.push_str(&format!("<td>{}</td>", format_inline(cell)));
                }
                html.push_str("</tr>\n");
            }
            continue;
        }

        // Unordered lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            let content = &trimmed[2..];
            html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
            continue;
        }

        // Empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Regular paragraphs
        html.push_str(&format!("<p>{}</p>\n", format_inline(trimmed)));
    }

    // Close any open tags
    if in_code_block {
        html.push_str("</code></pre>\n");
    }
    if in_list {
        html.push_str("</ul>\n");
    }
    if in_table {
        html.push_str("</tbody></table>\n");
    }

    html
}

/// Format inline markdown (bold, code, etc.)
fn format_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Inline code: `code`
    let mut formatted = String::new();
    let mut in_code = false;
    for c in result.chars() {
        if c == '`' && !in_code {
            formatted.push_str("<code>");
            in_code = true;
        } else if c == '`' && in_code {
            formatted.push_str("</code>");
            in_code = false;
        } else {
            formatted.push(c);
        }
    }
    result = formatted;

    // Bold: **text**
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let before = &result[..start];
            let content = &result[start + 2..start + 2 + end];
            let after = &result[start + 2 + end + 2..];
            result = format!("{}<strong>{}</strong>{}", before, content, after);
        } else {
            break;
        }
    }

    result
}
