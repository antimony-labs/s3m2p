//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | GIT/src/render.rs
//! PURPOSE: DOM rendering for Git lessons with SLAM-style layout
//! MODIFIED: 2026-01-01
//! LAYER: LEARN → GIT
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::lessons::{Lesson, GLOSSARY, LESSONS, PHASES};
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

    /// Apply glossary tooltips - wrap technical terms with tooltip spans
    fn apply_glossary(text: &str) -> String {
        let mut result = text.to_string();
        for term in GLOSSARY {
            // Case-insensitive search and replace (first occurrence only)
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

    pub fn render_home(&self, _lessons: &[Lesson]) -> Result<(), JsValue> {
        let mut html = String::from(
            r##"
            <header class="hero">
                <h1>Git Version Control</h1>
                <p class="subtitle">From History to Mastery - The Complete Guide</p>
                <p class="lesson-count">18 Lessons · 7 Phases · Beginner to Advanced</p>
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

            // Determine phase icon based on Git phases
            let phase_icon = match *phase {
                "Origins & Philosophy" => "📜",
                "Foundations" => "🎯",
                "Branching & Merging" => "🌿",
                "Collaboration" => "🤝",
                "Advanced Workflows" => "⚡",
                "Best Practices" => "✨",
                "Software Engineering" => "🏗️",
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
                html.push_str(&format!(
                    r##"
                    <div class="lesson-card" onclick="go_to_lesson({})">
                        <span class="lesson-icon">{}</span>
                        <h3>{}</h3>
                        <p class="lesson-subtitle">{}</p>
                    </div>
                "##,
                    lesson.id, lesson.icon, lesson.title, lesson.subtitle
                ));
            }

            html.push_str(
                r##"
                    </div>
                </section>
            "##,
            );
        }

        // Resources section
        html.push_str(
            r##"
            <section class="resources">
                <h2>Resources</h2>
                <div class="resource-grid">
                    <a href="https://git-scm.com/doc" target="_blank" class="resource-card">
                        <span class="resource-icon">📚</span>
                        <h3>Official Docs</h3>
                        <p>Git reference documentation</p>
                    </a>
                    <a href="https://github.com" target="_blank" class="resource-card">
                        <span class="resource-icon">🐙</span>
                        <h3>GitHub</h3>
                        <p>World's largest code host</p>
                    </a>
                    <a href="https://learngitbranching.js.org/" target="_blank" class="resource-card">
                        <span class="resource-icon">🎮</span>
                        <h3>Learn Git Branching</h3>
                        <p>Interactive branching tutorial</p>
                    </a>
                </div>
            </section>
            <footer>
                <a href="https://too.foo">← back to too.foo</a>
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
        // Build key takeaways list
        let takeaways_html: String = lesson
            .key_takeaways
            .iter()
            .map(|t| format!(r#"<li class="takeaway-item">{}</li>"#, t))
            .collect::<Vec<_>>()
            .join("");

        // Render concepts with tooltips if definitions exist
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

        // Generate progress navigation bubbles
        let progress_nav = self.render_lesson_progress(lesson.id, LESSONS.len());

        // Convert intuition markdown to HTML, then apply glossary tooltips
        let intuition_html = Self::apply_glossary(&convert_markdown_to_html(lesson.intuition));

        // Convert main content markdown to HTML
        let content_html = convert_markdown_to_html(lesson.content);

        // Convert dos_and_donts markdown to HTML
        let dos_donts_html = if !lesson.dos_and_donts.is_empty() {
            format!(
                r##"
                <details class="dos-donts">
                    <summary><h3>✅ Dos & Don'ts</h3></summary>
                    <div class="dos-donts-content">{}</div>
                </details>
                "##,
                convert_markdown_to_html(lesson.dos_and_donts)
            )
        } else {
            String::new()
        };

        // Going deeper section
        let going_deeper_html = if !lesson.going_deeper.is_empty() {
            format!(
                r##"
                <details class="going-deeper">
                    <summary><h3>🔬 Going Deeper</h3></summary>
                    <div class="going-deeper-content">{}</div>
                </details>
                "##,
                convert_markdown_to_html(lesson.going_deeper)
            )
        } else {
            String::new()
        };

        // Common mistakes section
        let common_mistakes_html = if !lesson.common_mistakes.is_empty() {
            format!(
                r##"
                <details class="common-mistakes">
                    <summary><h3>⚠️ Common Mistakes</h3></summary>
                    <div class="common-mistakes-content">{}</div>
                </details>
                "##,
                convert_markdown_to_html(lesson.common_mistakes)
            )
        } else {
            String::new()
        };

        let total_lessons = LESSONS.len();

        let prev_button = if lesson.id > 0 {
            format!(
                r##"<button onclick="go_to_lesson({})" class="nav-btn">← Previous</button>"##,
                lesson.id - 1
            )
        } else {
            String::from("<span></span>")
        };

        let next_button = if lesson.id < total_lessons - 1 {
            format!(
                r##"<button onclick="go_to_lesson({})" class="nav-btn">Next →</button>"##,
                lesson.id + 1
            )
        } else {
            String::from("<span></span>")
        };

        let html = format!(
            r##"
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
                    <!-- 1. Why It Matters (Hook) -->
                    <section class="why-it-matters">
                        <p class="hook">{why_it_matters}</p>
                    </section>

                    <!-- 2. Key Concepts -->
                    <section class="concepts">
                        <h3>Key Concepts</h3>
                        <div class="concept-list">{concepts}</div>
                    </section>

                    <!-- 3. Intuition (Plain language explanation) -->
                    <section class="intuition">
                        <h3>💡 The Idea</h3>
                        <div class="intuition-text">{intuition}</div>
                    </section>

                    <!-- 4. Main Content -->
                    <section class="main-content">
                        {content}
                    </section>

                    <!-- 5. Key Takeaways -->
                    <section class="takeaways">
                        <h3>📝 Key Takeaways</h3>
                        <ul class="takeaway-list">{takeaways}</ul>
                    </section>

                    <!-- 6. Dos & Don'ts (Collapsible) -->
                    {dos_donts}

                    <!-- 7. Going Deeper (Collapsible) -->
                    {going_deeper}

                    <!-- 8. Common Mistakes (Collapsible) -->
                    {common_mistakes}
                </div>

                <nav class="lesson-footer">
                    {prev_btn}
                    {next_btn}
                </nav>
            </article>
        "##,
            current = lesson.id + 1,
            total = total_lessons,
            progress_nav = progress_nav,
            icon = lesson.icon,
            phase = lesson.phase,
            title = lesson.title,
            subtitle = lesson.subtitle,
            why_it_matters = lesson.why_it_matters,
            concepts = concepts_html,
            intuition = intuition_html,
            content = content_html,
            takeaways = takeaways_html,
            dos_donts = dos_donts_html,
            going_deeper = going_deeper_html,
            common_mistakes = common_mistakes_html,
            prev_btn = prev_button,
            next_btn = next_button,
        );

        self.root.set_inner_html(&html);
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
    let mut code_lang;

    // Helper to close current list
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
            close_list(&mut html, &mut list_type);
            if in_table {
                html.push_str("</table>\n");
                in_table = false;
            }
            continue;
        }

        // Headers
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

        // Horizontal rule
        if trimmed == "---" {
            close_list(&mut html, &mut list_type);
            html.push_str("<hr>\n");
            continue;
        }

        // Blockquotes
        if trimmed.starts_with("> ") {
            close_list(&mut html, &mut list_type);
            html.push_str(&format!(
                "<blockquote>{}</blockquote>\n",
                format_inline(&trimmed[2..])
            ));
            continue;
        }

        // Tables
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            close_list(&mut html, &mut list_type);
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

        // Unordered lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            // Close ordered list if switching types
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

        // Ordered lists
        if trimmed
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && trimmed.contains(". ")
        {
            if let Some(pos) = trimmed.find(". ") {
                // Close unordered list if switching types
                if list_type == ListType::Unordered {
                    close_list(&mut html, &mut list_type);
                }
                if list_type == ListType::None {
                    html.push_str("<ol>\n");
                    list_type = ListType::Ordered;
                }
                let content = &trimmed[pos + 2..];
                html.push_str(&format!("<li>{}</li>\n", format_inline(content)));
                continue;
            }
        }

        // Non-list content closes any open list
        close_list(&mut html, &mut list_type);

        // Pass through existing HTML tags without wrapping in <p>
        if trimmed.starts_with('<') && !trimmed.starts_with("<!") {
            html.push_str(trimmed);
            html.push('\n');
            continue;
        }

        // Regular paragraph
        html.push_str(&format!("<p>{}</p>\n", format_inline(trimmed)));
    }

    // Close any open tags
    if in_code_block {
        html.push_str("</code></pre>\n");
    }
    close_list(&mut html, &mut list_type);
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
