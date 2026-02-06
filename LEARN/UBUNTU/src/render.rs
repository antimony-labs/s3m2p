//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | UBUNTU/src/render.rs
//! PURPOSE: DOM rendering for Ubuntu lessons
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → UBUNTU
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{DemoType, Lesson, LESSONS, PHASES};
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

    pub fn render_home(&self, _lessons: &[Lesson]) -> Result<(), JsValue> {
        let mut html = String::from(
            r##"
            <header class="hero">
                <h1>Ubuntu Linux</h1>
                <p class="subtitle">From History to Mastery - An Interactive Journey</p>
                <p class="lesson-count">20 Lessons - 7 Phases - Beginner to Intermediate</p>
            </header>
        "##,
        );

        // Render lessons grouped by phase
        for phase in PHASES.iter() {
            let phase_lessons: Vec<&Lesson> =
                LESSONS.iter().filter(|l| l.phase == *phase).collect();

            if phase_lessons.is_empty() {
                continue;
            }

            // Determine phase icon
            let phase_icon = match *phase {
                "The Story of Linux" => "📖",
                "Getting Started" => "🚀",
                "Filesystem Fundamentals" => "📁",
                "System Administration" => "⚙️",
                "Networking" => "🌐",
                "Developer Workflow" => "💻",
                "Maintenance" => "🔧",
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
                    DemoType::Terminal => {
                        r##"<span class="badge badge-terminal">Interactive</span>"##
                    }
                    DemoType::TerminalDiagram => {
                        r##"<span class="badge badge-terminal">Visual</span>"##
                    }
                    DemoType::Calculator => r##"<span class="badge badge-calc">Calculator</span>"##,
                    DemoType::Static => r##"<span class="badge badge-static">Theory</span>"##,
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

            html.push_str(
                r##"
                    </div>
                </section>
            "##,
            );
        }

        // Cheat sheet download and resources section
        html.push_str(
            r##"
            <section class="resources">
                <h2>Resources</h2>
                <div class="resource-grid">
                    <a href="#cheatsheet" class="resource-card" onclick="window.print(); return false;">
                        <span class="resource-icon">📄</span>
                        <h3>Cheat Sheet</h3>
                        <p>Print-friendly command reference</p>
                    </a>
                    <a href="https://help.ubuntu.com/" target="_blank" class="resource-card">
                        <span class="resource-icon">📚</span>
                        <h3>Official Docs</h3>
                        <p>Ubuntu documentation</p>
                    </a>
                    <a href="https://ubuntu.com/tutorials/command-line-for-beginners" target="_blank" class="resource-card">
                        <span class="resource-icon">🎓</span>
                        <h3>CLI Tutorial</h3>
                        <p>Ubuntu command line guide</p>
                    </a>
                </div>
            </section>
            <footer>
                <a href="https://too.foo">Back to too.foo</a>
            </footer>
        "##,
        );

        self.root.set_inner_html(&html);
        Ok(())
    }

    fn render_lesson_progress(&self, current_idx: usize, total: usize) -> String {
        let mut html = String::from(r##"<div class="lesson-progress-nav">"##);

        // Show max 11 bubbles (current ± 5) for space efficiency
        let start = current_idx.saturating_sub(5);
        let end = (current_idx + 6).min(total);

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

    pub fn render_lesson(&self, lesson: &Lesson) -> Result<(), JsValue> {
        // Render concepts with tooltips if definitions exist
        let concepts_html: String = if lesson.concept_definitions.is_empty() {
            // Fallback: render without tooltips
            lesson
                .key_concepts
                .iter()
                .map(|c| format!(r##"<span class="concept">{}</span>"##, c))
                .collect::<Vec<_>>()
                .join("")
        } else {
            // Render with tooltips from definitions
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

        // Generate progress navigation bubbles
        let progress_nav = self.render_lesson_progress(lesson.id, LESSONS.len());

        // Determine demo section based on lesson type
        let demo_section = match lesson.demo_type {
            DemoType::Terminal => String::from(
                r##"
                <section class="terminal-section">
                    <h3>Interactive Terminal</h3>
                    <div class="terminal" id="terminal">
                        <div class="terminal-output" id="terminal-output"></div>
                        <div class="terminal-input-line">
                            <span class="terminal-prompt" id="terminal-prompt">user@ubuntu:~$ </span>
                            <input type="text" id="terminal-input" class="terminal-input" autocomplete="off" spellcheck="false" autofocus>
                        </div>
                    </div>
                    <div class="terminal-hints">
                        <p>Try: <code>ls -l</code>, <code>cat readme.txt</code>, <code>chmod 777 readme.txt</code>, <code>su root</code>, <code>help</code></p>
                    </div>
                </section>
                "##,
            ),
            DemoType::TerminalDiagram => String::from(
                r##"
                <section class="split-demo">
                    <div class="terminal-half">
                        <h3>Interactive Terminal</h3>
                        <div class="terminal" id="terminal">
                            <div class="terminal-output" id="terminal-output"></div>
                            <div class="terminal-input-line">
                                <span class="terminal-prompt" id="terminal-prompt">user@ubuntu:~$ </span>
                                <input type="text" id="terminal-input" class="terminal-input" autocomplete="off" spellcheck="false" autofocus>
                            </div>
                        </div>
                    </div>
                    <div class="diagram-half">
                        <h3>Live Visualization</h3>
                        <canvas id="diagram-canvas" width="600" height="400"></canvas>
                    </div>
                </section>
                "##,
            ),
            DemoType::Calculator => String::from(
                r##"
                <section class="calculator-section">
                    <h3>Partition Calculator</h3>
                    <div class="calculator">
                        <div class="calc-input">
                            <label for="disk-size">Total Disk Space (GB):</label>
                            <input type="number" id="disk-size" value="500" min="50" max="4000">
                            <button onclick="calculatePartitions()">Calculate</button>
                        </div>
                        <div class="calc-results" id="calc-results">
                            <p>Enter disk size and click Calculate</p>
                        </div>
                    </div>
                </section>
                "##,
            ),
            DemoType::Static => String::new(),
        };

        // Convert content markdown to simple HTML
        let content_html = convert_markdown_to_html(lesson.content);

        let total_lessons = LESSONS.len();

        let prev_button = if lesson.id > 0 {
            format!(
                r##"<button onclick="go_to_lesson({})" class="nav-btn">Previous</button>"##,
                lesson.id - 1
            )
        } else {
            String::from("<span></span>")
        };

        let next_button = if lesson.id < total_lessons - 1 {
            format!(
                r##"<button onclick="go_to_lesson({})" class="nav-btn">Next</button>"##,
                lesson.id + 1
            )
        } else {
            String::from("<span></span>")
        };

        let html = format!(
            r##"
            <article class="lesson-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">All Lessons</button>
                    <span class="lesson-progress">{} / {}</span>
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

                    <section class="main-content">
                        {}
                    </section>

                    {}
                </div>

                <nav class="lesson-footer">
                    {}
                    {}
                </nav>
            </article>
        "##,
            lesson.id + 1,
            total_lessons,
            progress_nav,
            lesson.icon,
            lesson.phase,
            lesson.title,
            lesson.subtitle,
            lesson.description,
            concepts_html,
            content_html,
            demo_section,
            prev_button,
            next_button,
        );

        self.root.set_inner_html(&html);
        Ok(())
    }
}

/// Simple markdown to HTML converter for lesson content
fn convert_markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_table = false;
    let mut in_list = false;
    let mut code_lang;

    for line in md.lines() {
        let trimmed = line.trim();

        // Code blocks
        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                code_lang = trimmed.trim_start_matches("```").to_string();
                let lang_class = if code_lang.is_empty() {
                    String::new()
                } else {
                    format!(r##" class="language-{}""##, code_lang)
                };
                html.push_str(&format!("<pre><code{}>\n", lang_class));
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            // Escape HTML in code blocks
            let escaped = line
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            html.push_str(&escaped);
            html.push('\n');
            continue;
        }

        // Empty lines
        if trimmed.is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            if in_table {
                html.push_str("</table>\n");
                in_table = false;
            }
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

        // Horizontal rule
        if trimmed == "---" {
            html.push_str("<hr>\n");
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
                html.push_str("<table>\n");
                in_table = true;
            }

            let cells: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim())
                .collect();

            html.push_str("<tr>");
            for cell in cells {
                html.push_str(&format!("<td>{}</td>", format_inline(cell)));
            }
            html.push_str("</tr>\n");
            continue;
        } else if in_table {
            html.push_str("</table>\n");
            in_table = false;
        }

        // Lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            let content = &trimmed[2..];
            html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
            continue;
        }
        if trimmed
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && trimmed.contains(". ")
        {
            if let Some(pos) = trimmed.find(". ") {
                if !in_list {
                    html.push_str("<ol>\n");
                    in_list = true;
                }
                let content = &trimmed[pos + 2..];
                html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
                continue;
            }
        }

        if in_list && !trimmed.starts_with("- ") && !trimmed.starts_with("* ") {
            html.push_str("</ul>\n");
            in_list = false;
        }

        // Regular paragraph
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
        html.push_str("</table>\n");
    }

    html
}

/// Format inline markdown (bold, italic, code, links)
fn format_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Inline code (must come before other formatting)
    let mut formatted = String::new();
    let mut in_code = false;
    let chars = result.chars();

    for c in chars {
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

    // Bold (**text**)
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
