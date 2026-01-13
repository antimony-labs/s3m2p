//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | BLOG/src/lib.rs
//! PURPOSE: Core blog types and markdown parsing with frontmatter support
//! MODIFIED: 2025-11-29
//! LAYER: BLOG
//! ═══════════════════════════════════════════════════════════════════════════════

// Blog - Markdown-based blog engine
// AI-assisted content, rendered in Rust/WASM
#![allow(unexpected_cfgs)]

use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub mod render;
pub mod router;

/// Blog post metadata (frontmatter)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub slug: String,
    pub date: String,
    pub tags: Vec<String>,
    pub summary: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub ai_generated: bool,
}

/// Full blog post with content
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String, // Raw markdown
}

impl Post {
    /// Render markdown content to HTML with heading IDs for anchor links
    pub fn render_html(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&self.content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Post-process to add IDs to headings
        let mut result = String::new();
        let mut last_end = 0;

        for (i, _) in html_output.match_indices("<h") {
            if i + 3 < html_output.len() {
                let level_char = html_output.chars().nth(i + 2);
                if let Some(c) = level_char {
                    if c.is_ascii_digit() && html_output[i..].starts_with(&format!("<h{}>", c)) {
                        // Find the closing tag
                        let close_tag = format!("</h{}>", c);
                        if let Some(close_idx) = html_output[i..].find(&close_tag) {
                            let heading_content = &html_output[i + 4..i + close_idx];
                            let id = heading_content
                                .to_lowercase()
                                .chars()
                                .filter(|ch| ch.is_alphanumeric() || *ch == ' ' || *ch == '-')
                                .collect::<String>()
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .join("-");

                            result.push_str(&html_output[last_end..i]);
                            result.push_str(&format!("<h{} id=\"{}\">", c, id));
                            last_end = i + 4;
                        }
                    }
                }
            }
        }
        result.push_str(&html_output[last_end..]);

        result
    }

    /// Extract reading time estimate
    pub fn reading_time(&self) -> u32 {
        let word_count = self.content.split_whitespace().count();
        ((word_count as f32 / 200.0).ceil() as u32).max(1) // 200 WPM average
    }
}

/// Blog index (list of all posts)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BlogIndex {
    pub posts: Vec<PostMeta>,
}

impl BlogIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter posts by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&PostMeta> {
        self.posts
            .iter()
            .filter(|p| !p.draft && p.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Get recent posts
    pub fn recent(&self, count: usize) -> Vec<&PostMeta> {
        self.posts.iter().filter(|p| !p.draft).take(count).collect()
    }

    /// Get all unique tags
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .posts
            .iter()
            .flat_map(|p| p.tags.iter().cloned())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

/// Blog state
#[derive(Default)]
pub struct Blog {
    pub index: BlogIndex,
    pub current_post: Option<Post>,
}

impl Blog {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parse markdown frontmatter (YAML-style)
pub fn parse_frontmatter(content: &str) -> Option<(PostMeta, String)> {
    if !content.starts_with("---") {
        return None;
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }

    let frontmatter = parts[1].trim();
    let body = parts[2].trim();

    // Simple key-value parsing (for full YAML, use serde_yaml)
    let mut meta = PostMeta {
        title: String::new(),
        slug: String::new(),
        date: String::new(),
        tags: Vec::new(),
        summary: String::new(),
        draft: false,
        ai_generated: false,
    };

    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "title" => meta.title = value.to_string(),
                "slug" => meta.slug = value.to_string(),
                "date" => meta.date = value.to_string(),
                "summary" => meta.summary = value.to_string(),
                "draft" => meta.draft = value == "true",
                "ai_generated" => meta.ai_generated = value == "true",
                "tags" => {
                    meta.tags = value
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .collect();
                }
                _ => {}
            }
        }
    }

    Some((meta, body.to_string()))
}

/// Posts index structure
#[derive(Deserialize)]
struct PostsIndex {
    posts: Vec<String>,
}

/// Fetch text content from a URL
async fn fetch_text(url: &str) -> Result<String, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let resp_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let text = JsFuture::from(resp.text()?).await?;
    text.as_string().ok_or_else(|| "Not a string".into())
}

/// Load all posts from the posts directory
async fn load_posts() -> Result<(BlogIndex, Vec<Post>), JsValue> {
    // Fetch the posts index
    let index_text = fetch_text("/posts/index.json").await?;
    let posts_index: PostsIndex =
        serde_json::from_str(&index_text).map_err(|e| e.to_string())?;

    let mut all_posts: Vec<Post> = Vec::new();
    let mut index = BlogIndex::new();

    // Fetch each post
    for filename in &posts_index.posts {
        let url = format!("/posts/{}", filename);
        match fetch_text(&url).await {
            Ok(content) => {
                if let Some((meta, body)) = parse_frontmatter(&content) {
                    if !meta.draft {
                        index.posts.push(meta.clone());
                        all_posts.push(Post { meta, content: body });
                    }
                }
            }
            Err(e) => {
                web_sys::console::warn_1(&format!("Failed to load {}: {:?}", filename, e).into());
            }
        }
    }

    // Sort by date descending
    index.posts.sort_by(|a, b| b.date.cmp(&a.date));
    all_posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    Ok((index, all_posts))
}

/// App state
struct App {
    index: BlogIndex,
    posts: Vec<Post>,
    renderer: render::BlogRenderer,
    router: router::Router,
}

impl App {
    fn render_current_route(&self) -> Result<(), JsValue> {
        use router::Route;

        match self.router.current() {
            Route::Home => self.renderer.render_home(&self.index),
            Route::Post(slug) => {
                if let Some(post) = self.posts.iter().find(|p| p.meta.slug == *slug) {
                    self.renderer.render_post(post)
                } else {
                    self.renderer.render_404()
                }
            }
            Route::Tag(tag) => self.renderer.render_tag(tag, &self.index),
            Route::Archive => self.renderer.render_home(&self.index), // TODO: archive page
            Route::About => self.renderer.render_home(&self.index),   // TODO: about page
            Route::NotFound => self.renderer.render_404(),
        }
    }
}

/// Initialize and run the blog
async fn run() -> Result<(), JsValue> {
    let (index, posts) = load_posts().await?;
    let renderer = render::BlogRenderer::new("blog-root")?;
    let router = router::Router::new()?;

    let app = Rc::new(RefCell::new(App {
        index,
        posts,
        renderer,
        router,
    }));

    // Initial render
    app.borrow().render_current_route()?;

    // Handle popstate (back/forward)
    let app_clone = app.clone();
    let closure = Closure::wrap(Box::new(move |_: web_sys::PopStateEvent| {
        let mut app = app_clone.borrow_mut();
        if app.router.sync_from_url().is_ok() {
            let _ = app.render_current_route();
        }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
        .ok_or("No window")?
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Handle clicks on internal links
    let app_clone = app.clone();
    let click_closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        if let Some(target) = e.target() {
            if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                // Walk up to find anchor
                let mut current: Option<web_sys::HtmlElement> = Some(el);
                while let Some(node) = current {
                    if node.tag_name() == "A" {
                        if let Ok(anchor) = node.dyn_into::<web_sys::HtmlAnchorElement>() {
                            let href = anchor.get_attribute("href").unwrap_or_default();
                            // Only handle internal links
                            if href.starts_with('/') && !href.starts_with("//") {
                                e.prevent_default();
                                let route = router::Route::from_path(&href);
                                let mut app = app_clone.borrow_mut();
                                if app.router.navigate(route).is_ok() {
                                    let _ = app.render_current_route();
                                }
                            }
                        }
                        break;
                    }
                    current = node.parent_element().and_then(|p| p.dyn_into().ok());
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
        .ok_or("No window")?
        .document()
        .ok_or("No document")?
        .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
    click_closure.forget();

    web_sys::console::log_1(&format!("Blog loaded: {} posts", app.borrow().posts.len()).into());
    Ok(())
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = run().await {
            web_sys::console::error_1(&format!("Blog error: {:?}", e).into());
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: "Test Post"
slug: "test-post"
date: "2025-01-15"
tags: [rust, wasm]
summary: "A test post"
---

# Hello World

This is the content.
"#;

        let (meta, body) = parse_frontmatter(content).unwrap();
        assert_eq!(meta.title, "Test Post");
        assert_eq!(meta.slug, "test-post");
        assert!(body.contains("Hello World"));
    }

    #[test]
    fn test_render_html() {
        let post = Post {
            meta: PostMeta {
                title: "Test".into(),
                slug: "test".into(),
                date: "2025-01-15".into(),
                tags: vec![],
                summary: "".into(),
                draft: false,
                ai_generated: false,
            },
            content: "# Heading\n\nParagraph with **bold**.".into(),
        };

        let html = post.render_html();
        assert!(html.contains("<h1>Heading</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }
}
