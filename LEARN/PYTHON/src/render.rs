//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | PYTHON/src/render.rs
//! PURPOSE: DOM rendering for Python lessons
//! MODIFIED: 2026-01-31
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, LESSONS, PHASES, GLOSSARY};
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

    fn playground_html() -> String {
        r#"
        <section class="playground-section" id="python-playground">
            <div class="playground-header">
                <h2>Python Playground</h2>
                <p class="subtitle">Run snippets instantly in-browser. Imports load packages on demand.</p>
                <div class="playground-toolbar">
                    <div class="playground-actions">
                        <button id="py-run-btn" class="btn primary">Run</button>
                        <button id="py-clear-btn" class="btn">Clear</button>
                        <button id="py-reset-btn" class="btn">Reset</button>
                    </div>
                    <span id="py-runtime-status" class="runtime-status">Runtime idle</span>
                </div>
            </div>
            <div class="playground-grid">
                <div class="playground-editor">
                    <textarea id="py-code" spellcheck="false"></textarea>
                </div>
                <div class="playground-output">
                    <div class="output-title">Output</div>
                    <div id="py-output" class="output-console"></div>
                </div>
            </div>
            <p class="playground-note">Packages: numpy, pandas, matplotlib. Use import and they load automatically.</p>
        </section>
        "#
        .to_string()
    }

    fn call_js_hook(name: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(func) = js_sys::Reflect::get(&window, &JsValue::from_str(name)) {
                if let Ok(func) = func.dyn_into::<js_sys::Function>() {
                    let _ = func.call0(&JsValue::NULL);
                }
            }
        }
    }

    /// Apply glossary tooltips - wrap technical terms with tooltip spans
    fn apply_glossary(text: &str) -> String {
        let mut result = text.to_string();
        for term in GLOSSARY {
            let pattern = term.word;
            if let Some(pos) = result.to_lowercase().find(&pattern.to_lowercase()) {
                let original = &result[pos..pos + pattern.len()];
                let tooltip = format!(
                    r#"<span class="glossary-term" data-tooltip="{}">{}</span>"#,
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

    fn glossary_tooltip_for(term: &str) -> Option<&'static str> {
        for entry in GLOSSARY {
            if entry.word.eq_ignore_ascii_case(term) {
                return Some(entry.short);
            }
        }
        None
    }

    pub fn render_home(&self, _lessons: &[Lesson]) -> Result<(), JsValue> {
        let mut html = String::from(
            r#"
            <header class="hero">
                <h1>Python + DSA</h1>
                <p class="subtitle">Build Python fundamentals, then level up for interviews and LeetCode</p>
                <p class="lesson-count">12 Lessons · Foundations to Advanced Core</p>
            </header>
        "#,
        );

        html.push_str(&Self::playground_html());

        for phase in PHASES.iter() {
            let phase_lessons: Vec<&Lesson> = LESSONS.iter().filter(|l| l.phase == *phase).collect();

            if phase_lessons.is_empty() {
                continue;
            }

            let phase_icon = match *phase {
                "Foundations" => "🧭",
                "Core Python" => "⚙️",
                "Data Structures" => "🧱",
                "Problem Solving" => "🧠",
                "OOP + Practice" => "🏗️",
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
            <section class="resources">
                <h2>Resources</h2>
                <div class="resource-grid">
                    <a href="python_dsa_master_notes.md" target="_blank" class="resource-card">
                        <span class="resource-icon">📒</span>
                        <h3>Master Notes</h3>
                        <p>One-spot revision doc (download)</p>
                    </a>
                    <a href="https://docs.python.org/3/" target="_blank" class="resource-card">
                        <span class="resource-icon">📘</span>
                        <h3>Python Docs</h3>
                        <p>Official reference</p>
                    </a>
                    <a href="https://leetcode.com" target="_blank" class="resource-card">
                        <span class="resource-icon">🧩</span>
                        <h3>LeetCode</h3>
                        <p>Daily practice</p>
                    </a>
                </div>
            </section>
            <footer>
                <a href="https://too.foo">← back to too.foo</a>
            </footer>
        "#,
        );

        self.root.set_inner_html(&html);
        Self::call_js_hook("initPythonPlayground");
        Ok(())
    }

    fn render_lesson_progress(&self, current_idx: usize, total: usize) -> String {
        let mut html = String::from(r#"<div class=\"lesson-progress-nav\">"#);

        let start = current_idx.saturating_sub(4);
        let end = (current_idx + 5).min(total);

        if start > 0 {
            html.push_str(r#"<span class=\"progress-ellipsis\">...</span>"#);
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
                r#"<button class=\"{}\" onclick=\"go_to_lesson({})\" title=\"{}\">
                    <span class=\"bubble-num\">{}</span>
                </button>"#,
                class,
                i,
                lesson_title,
                i + 1
            ));

            if i < end - 1 {
                html.push_str(r#"<span class=\"progress-line\"></span>"#);
            }
        }

        if end < total {
            html.push_str(r#"<span class=\"progress-ellipsis\">...</span>"#);
        }

        html.push_str(r#"</div>"#);
        html
    }

    pub fn render_lesson(&self, lesson: &Lesson, total_lessons: usize) -> Result<(), JsValue> {
        let concepts_html: String = lesson
            .key_concepts
            .iter()
            .map(|c| {
                if let Some(def) = Self::glossary_tooltip_for(c) {
                    format!(r#"<span class="concept" data-tooltip="{}">{}</span>"#, def, c)
                } else {
                    format!(r#"<span class="concept">{}</span>"#, c)
                }
            })
            .collect::<Vec<_>>()
            .join("");

        let progress_nav = self.render_lesson_progress(lesson.id, total_lessons);

        let intuition_html = Self::apply_glossary(&convert_markdown_to_html(lesson.intuition));
        let content_html = convert_markdown_to_html(lesson.content);
        let why_it_matters = Self::apply_glossary(lesson.why_it_matters);

        let dos_donts_html = if !lesson.dos_and_donts.is_empty() {
            format!(
                r#"
                <details class=\"dos-donts\">
                    <summary><h3>✅ Dos & Don'ts</h3></summary>
                    <div class=\"dos-donts-content\">{}</div>
                </details>
                "#,
                convert_markdown_to_html(lesson.dos_and_donts)
            )
        } else {
            String::new()
        };

        let going_deeper_html = if !lesson.going_deeper.is_empty() {
            format!(
                r#"
                <details class=\"going-deeper\">
                    <summary><h3>🔬 Going Deeper</h3></summary>
                    <div class=\"going-deeper-content\">{}</div>
                </details>
                "#,
                convert_markdown_to_html(lesson.going_deeper)
            )
        } else {
            String::new()
        };

        let common_mistakes_html = if !lesson.common_mistakes.is_empty() {
            format!(
                r#"
                <details class=\"common-mistakes\">
                    <summary><h3>⚠️ Common Bugs & Fixes</h3></summary>
                    <div class=\"common-mistakes-content\">{}</div>
                </details>
                "#,
                convert_markdown_to_html(lesson.common_mistakes)
            )
        } else {
            String::new()
        };

        let prev_button = if lesson.id > 0 {
            format!(
                r#"<button onclick=\"go_to_lesson({})\" class=\"nav-btn\">← Previous</button>"#,
                lesson.id - 1
            )
        } else {
            String::new()
        };

        let next_button = if lesson.id + 1 < total_lessons {
            format!(
                r#"<button onclick=\"go_to_lesson({})\" class=\"nav-btn\">Next →</button>"#,
                lesson.id + 1
            )
        } else {
            String::new()
        };

        let takeaways_html: String = lesson
            .key_takeaways
            .iter()
            .map(|t| format!(r#"<li class=\"takeaway-item\">{}</li>"#, t))
            .collect::<Vec<_>>()
            .join("");

        let html = format!(
            r#"
            <article class="lesson-view">
                <nav class="lesson-nav">
                    <button onclick="go_home()" class="back-btn">← All Lessons</button>
                    <span class="lesson-progress">{current} / {total}</span>
                </nav>

                {progress_nav}

                <header class="lesson-header">
                    <span class="lesson-icon-large">{icon}</span>
                    <div>
                        <span class="phase-badge">{phase}</span>
                        <h1>{title}</h1>
                        <p class="subtitle">{subtitle}</p>
                    </div>
                </header>

                <div class="lesson-content">
                    <section class="why-it-matters">
                        <p class="hook">{why_it_matters}</p>
                    </section>

                    <section class="concepts">
                        <h3>Key Concepts</h3>
                        <div class="concept-list">{concepts}</div>
                    </section>

                    <section class="intuition">
                        <h3>💡 The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    {playground}

                    <section class="main-content">
                        {content}
                    </section>

                    <section class="takeaways">
                        <h3>📝 Key Takeaways</h3>
                        <ul class="takeaway-list">{takeaways}</ul>
                    </section>

                    {dos_donts}
                    {going_deeper}
                    {common_mistakes}
                </div>

                <nav class="lesson-footer">
                    {prev_btn}
                    {next_btn}
                </nav>
            </article>
        "#,
            current = lesson.id + 1,
            total = total_lessons,
            progress_nav = progress_nav,
            icon = lesson.icon,
            phase = lesson.phase,
            title = lesson.title,
            subtitle = lesson.subtitle,
            why_it_matters = why_it_matters,
            concepts = concepts_html,
            intuition = intuition_html,
            playground = Self::playground_html(),
            content = content_html,
            takeaways = takeaways_html,
            dos_donts = dos_donts_html,
            going_deeper = going_deeper_html,
            common_mistakes = common_mistakes_html,
            prev_btn = prev_button,
            next_btn = next_button,
        );

        self.root.set_inner_html(&html);
        Self::call_js_hook("initPythonPlayground");
        Ok(())
    }
}

/// Track list type for proper closing tags
#[derive(Clone, Copy, PartialEq)]
enum ListType {
    None,
    Unordered,
    Ordered,
}

/// Simple markdown to HTML converter for lesson content
fn convert_markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_table = false;
    let mut list_type = ListType::None;

    let close_list = |html: &mut String, list_type: &mut ListType| {
        match *list_type {
            ListType::Unordered => html.push_str("</ul>\n"),
            ListType::Ordered => html.push_str("</ol>\n"),
            ListType::None => {}
        }
        *list_type = ListType::None;
    };

    for line in md.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                let code_lang = trimmed.trim_start_matches("```");
                let lang_class = if code_lang.is_empty() {
                    String::new()
                } else {
                    format!(r#" class=\"language-{}\""#, code_lang)
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

        if trimmed.is_empty() {
            close_list(&mut html, &mut list_type);
            if in_table {
                html.push_str("</table>\n");
                in_table = false;
            }
            continue;
        }

        if trimmed.starts_with("## ") {
            close_list(&mut html, &mut list_type);
            html.push_str(&format!("<h2>{}</h2>\n", format_inline(&trimmed[3..])));
            continue;
        }
        if trimmed.starts_with("### ") {
            close_list(&mut html, &mut list_type);
            html.push_str(&format!("<h3>{}</h3>\n", format_inline(&trimmed[4..])));
            continue;
        }

        if trimmed == "---" {
            close_list(&mut html, &mut list_type);
            html.push_str("<hr>\n");
            continue;
        }

        if trimmed.starts_with("> ") {
            close_list(&mut html, &mut list_type);
            html.push_str(&format!("<blockquote>{}</blockquote>\n", format_inline(&trimmed[2..])));
            continue;
        }

        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            close_list(&mut html, &mut list_type);
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

        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if list_type == ListType::Ordered {
                close_list(&mut html, &mut list_type);
            }
            if list_type == ListType::None {
                html.push_str("<ul>\n");
                list_type = ListType::Unordered;
            }
            let content = &trimmed[2..];
            html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
            continue;
        }

        if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && trimmed.contains(". ")
        {
            if list_type == ListType::Unordered {
                close_list(&mut html, &mut list_type);
            }
            if list_type == ListType::None {
                html.push_str("<ol>\n");
                list_type = ListType::Ordered;
            }
            let content = trimmed.splitn(2, ". ").nth(1).unwrap_or("");
            html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
            continue;
        }

        close_list(&mut html, &mut list_type);
        html.push_str(&format!("<p>{}</p>\n", format_inline(trimmed)));
    }

    if in_table {
        html.push_str("</table>\n");
    }

    close_list(&mut html, &mut list_type);
    html
}

/// Simple inline formatter for bold + inline code
fn format_inline(text: &str) -> String {
    let mut output = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            chars.next();
            let mut bold = String::new();
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'*') {
                    chars.next();
                    break;
                }
                bold.push(c);
            }
            output.push_str(&format!("<strong>{}</strong>", bold));
            continue;
        }

        if ch == '`' {
            let mut code = String::new();
            while let Some(c) = chars.next() {
                if c == '`' {
                    break;
                }
                code.push(c);
            }
            output.push_str(&format!("<code>{}</code>", code));
            continue;
        }

        output.push(ch);
    }

    output
}
