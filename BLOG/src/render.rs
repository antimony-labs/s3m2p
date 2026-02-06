//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render.rs | BLOG/src/render.rs
//! PURPOSE: DOM rendering engine for blog posts, listings, and tag pages
//! MODIFIED: 2025-11-29
//! LAYER: BLOG
//! ═══════════════════════════════════════════════════════════════════════════════

// DOM rendering for blog
// Generates HTML elements from blog data

use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

use crate::{BlogIndex, Post, PostMeta};
use std::collections::HashMap;

const DEFAULT_AUTHOR: &str = "Antimony Labs";
const BASE_URL: &str = "https://blog.too.foo";

fn render_mermaid() {
    if let Some(window) = web_sys::window() {
        if let Ok(func) = js_sys::Reflect::get(&window, &JsValue::from_str("renderMermaid")) {
            if let Some(f) = func.dyn_ref::<js_sys::Function>() {
                let _ = f.call0(&window);
            }
        }
    }
}

fn author_for(meta: &PostMeta) -> &str {
    meta.author.as_deref().unwrap_or(DEFAULT_AUTHOR)
}

fn topic_for(meta: &PostMeta) -> String {
    meta.tags
        .first()
        .cloned()
        .unwrap_or_else(|| "General".to_string())
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn search_blob(meta: &PostMeta) -> String {
    let mut blob = String::new();
    blob.push_str(&meta.title);
    blob.push(' ');
    blob.push_str(&meta.summary);
    blob.push(' ');
    for tag in &meta.tags {
        blob.push_str(tag);
        blob.push(' ');
    }
    blob.push_str(&meta.slug);
    blob.to_lowercase()
}

fn top_tags(index: &BlogIndex, limit: usize) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for post in &index.posts {
        for tag in &post.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    let mut tags: Vec<(String, usize)> = counts.into_iter().collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    tags.into_iter().take(limit).map(|(t, _)| t).collect()
}

fn related_posts<'a>(index: &'a BlogIndex, post: &Post, limit: usize) -> Vec<&'a PostMeta> {
    let mut scored: Vec<(&PostMeta, usize)> = index
        .posts
        .iter()
        .filter(|meta| meta.slug != post.meta.slug)
        .map(|meta| {
            let shared = post
                .meta
                .tags
                .iter()
                .filter(|tag| meta.tags.contains(tag))
                .count();
            (meta, shared)
        })
        .filter(|(_, score)| *score > 0)
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.date.cmp(&a.0.date)));
    scored.into_iter().take(limit).map(|(m, _)| m).collect()
}

pub struct BlogRenderer {
    document: Document,
    root: Element,
}

impl BlogRenderer {
    pub fn new(root_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let root = document
            .get_element_by_id(root_id)
            .ok_or("Root element not found")?;

        Ok(Self { document, root })
    }

    /// Clear the root element
    pub fn clear(&self) {
        self.root.set_inner_html("");
    }

    fn set_title(&self, title: &str) {
        self.document.set_title(title);
    }

    fn set_meta_content(&self, selector: &str, content: &str) {
        if let Ok(Some(el)) = self.document.query_selector(selector) {
            let _ = el.set_attribute("content", content);
        }
    }

    fn set_canonical(&self, url: &str) {
        if let Some(el) = self.document.get_element_by_id("canonical-url") {
            let _ = el.set_attribute("href", url);
        }
    }

    fn set_structured_data(&self, payload: &str) {
        if let Some(el) = self.document.get_element_by_id("structured-data") {
            el.set_text_content(Some(payload));
        }
    }

    fn update_home_meta(&self) {
        let title = "Blog | too.foo";
        let description =
            "Notes on building simulations, tools, and machines - practical engineering essays and field notes.";
        let url = format!("{}/", BASE_URL);

        self.set_title(title);
        self.set_meta_content("meta[name=\"description\"]", description);
        self.set_meta_content("meta[property=\"og:title\"]", title);
        self.set_meta_content("meta[property=\"og:description\"]", description);
        self.set_meta_content("meta[property=\"og:url\"]", &url);
        self.set_meta_content("meta[property=\"og:type\"]", "website");
        self.set_meta_content("meta[name=\"twitter:title\"]", title);
        self.set_meta_content("meta[name=\"twitter:description\"]", description);
        self.set_canonical(&url);

        let structured = json!({
            "@context": "https://schema.org",
            "@type": "Blog",
            "name": "too.foo Blog",
            "url": url,
            "description": description,
            "publisher": {
                "@type": "Organization",
                "name": "Antimony Labs"
            }
        });
        self.set_structured_data(&structured.to_string());
    }

    fn update_post_meta(&self, post: &Post) {
        let meta = &post.meta;
        let title = format!("{} | too.foo", meta.title);
        let description = meta.summary.as_str();
        let url = format!("{}/post/{}", BASE_URL, meta.slug);
        let image = meta.hero.clone().unwrap_or_default();

        self.set_title(&title);
        self.set_meta_content("meta[name=\"description\"]", description);
        self.set_meta_content("meta[property=\"og:title\"]", &title);
        self.set_meta_content("meta[property=\"og:description\"]", description);
        self.set_meta_content("meta[property=\"og:url\"]", &url);
        self.set_meta_content("meta[property=\"og:type\"]", "article");
        if !image.is_empty() {
            self.set_meta_content("meta[property=\"og:image\"]", &image);
            self.set_meta_content("meta[name=\"twitter:image\"]", &image);
        }
        self.set_meta_content("meta[name=\"twitter:title\"]", &title);
        self.set_meta_content("meta[name=\"twitter:description\"]", description);
        self.set_canonical(&url);

        let updated = meta.updated.as_deref().unwrap_or(&meta.date);
        let structured = json!({
            "@context": "https://schema.org",
            "@type": "BlogPosting",
            "headline": meta.title,
            "datePublished": meta.date,
            "dateModified": updated,
            "author": {
                "@type": "Person",
                "name": author_for(meta)
            },
            "keywords": meta.tags.join(", "),
            "description": meta.summary,
            "url": url
        });
        self.set_structured_data(&structured.to_string());
    }

    /// Render the home page with recent posts
    pub fn render_home(&self, index: &BlogIndex) -> Result<(), JsValue> {
        self.clear();
        self.update_home_meta();

        let container = self.create_element("div", "blog-home")?;

        // Header
        let header = self.create_element("header", "blog-header")?;
        header.set_inner_html(
            r#"
            <p class="eyebrow">Too.Foo Journal</p>
            <h1>Blog</h1>
            <p class="subtitle">Field notes on building simulations, tools, and machines.</p>
            <p class="mission">Practical engineering essays, build logs, and design notes. Short, honest, and useful.</p>
        "#,
        );
        container.append_child(&header)?;

        let topics = top_tags(index, 12);
        let topics_html: String = topics
            .iter()
            .map(|tag| format!(r#"<a class="topic-chip" href="/tag/{}">#{}</a>"#, tag, tag))
            .collect();

        let mut start_here_posts: Vec<&PostMeta> =
            index.posts.iter().filter(|m| m.start_here).collect();
        if start_here_posts.is_empty() {
            start_here_posts = index.recent(3);
        }

        let mut editor_picks: Vec<&PostMeta> = index.posts.iter().filter(|m| m.featured).collect();
        if editor_picks.is_empty() {
            editor_picks = index
                .recent(8)
                .into_iter()
                .filter(|m| !start_here_posts.iter().any(|s| s.slug == m.slug))
                .take(3)
                .collect();
        }

        let start_here_html: String = start_here_posts
            .iter()
            .map(|meta| self.render_feature_card(meta))
            .collect();

        let picks_html: String = editor_picks
            .iter()
            .map(|meta| self.render_feature_card(meta))
            .collect();

        let recent_html: String = index
            .recent(10)
            .iter()
            .map(|meta| self.render_post_card(meta))
            .collect();

        // Main content column
        let posts_section = self.create_element("section", "recent-posts")?;
        posts_section.set_inner_html(&format!(
            r#"
            <div class="home-intro">
                <div class="search-box">
                    <label for="post-search">Search the archive</label>
                    <input id="post-search" type="search" placeholder="Search posts, tags, or topics" />
                    <p class="search-hint">Tip: try “simulation”, “cad”, or “wasm”.</p>
                    <p id="search-empty" class="search-empty">No posts match that search.</p>
                </div>
                <div class="topic-filters">
                    <span class="topic-label">Topics</span>
                    <div class="topic-chips">{}</div>
                </div>
            </div>
            <section class="curated-section" id="start-here">
                <div class="section-header">
                    <h2>Start here</h2>
                    <p>Three entry points to the work, selected for clarity and scope.</p>
                </div>
                <div class="feature-grid">
                    {}
                </div>
            </section>
            <section class="curated-section" id="editors-picks">
                <div class="section-header">
                    <h2>Editor’s picks</h2>
                    <p>Deep dives worth bookmarking and coming back to.</p>
                </div>
                <div class="feature-grid">
                    {}
                </div>
            </section>
            <section class="recent-posts-list">
                <h2>Recent posts</h2>
                {}
            </section>
        "#,
            topics_html, start_here_html, picks_html, recent_html
        ));
        container.append_child(&posts_section)?;

        let about_section = r#"
            <section class="about-panel">
                <div class="author-avatar">Sb</div>
                <div class="author-meta">
                    <h3>About the author</h3>
                    <p>Antimony Labs builds simulation-first tools and engineering systems from scratch. These notes capture the decisions behind the work.</p>
                    <a class="about-link" href="/about">Read the story</a>
                </div>
            </section>
        "#;

        let tags: String = index
            .all_tags()
            .iter()
            .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
            .collect::<Vec<_>>()
            .join(" ");

        let sidebar = self.create_element("aside", "home-sidebar")?;
        sidebar.set_inner_html(&format!(
            r#"
            {}
            <section class="tags-sidebar">
                <h3>Tags</h3>
                <div class="tag-cloud">{}</div>
            </section>
        "#,
            about_section, tags
        ));
        container.append_child(&sidebar)?;

        self.root.append_child(&container)?;
        Ok(())
    }

    /// Render a single post
    pub fn render_post(&self, post: &Post, index: &BlogIndex) -> Result<(), JsValue> {
        self.clear();
        self.update_post_meta(post);

        let article = self.create_element("article", "blog-post")?;

        let meta = &post.meta;
        let content_html = post.render_html();
        let reading_time = post.reading_time();
        let author = author_for(meta);
        let topic = topic_for(meta);
        let topic_html = if !meta.tags.is_empty() {
            format!(
                r#"<a href="/tag/{}" class="topic-link">#{}</a>"#,
                meta.tags[0], topic
            )
        } else {
            format!(r#"<span class="topic-link">#{}</span>"#, topic)
        };

        let updated_html = meta
            .updated
            .as_ref()
            .map(|updated| format!(r#"<span class="meta-updated">Updated {}</span>"#, updated))
            .unwrap_or_default();

        let tags_html: String = meta
            .tags
            .iter()
            .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
            .collect::<Vec<_>>()
            .join(" ");

        let ai_badge = if meta.ai_generated {
            r#"<span class="ai-badge">AI Assisted</span>"#
        } else {
            ""
        };

        let ai_note = if meta.ai_generated {
            r#"<div class="callout ai-note">This post was drafted with AI assistance and edited for accuracy and clarity.</div>"#
        } else {
            ""
        };

        let hero_html = if let Some(hero) = &meta.hero {
            let caption = meta
                .hero_caption
                .as_ref()
                .map(|c| format!(r#"<figcaption>{}</figcaption>"#, c))
                .unwrap_or_default();
            format!(
                r#"<figure class="post-hero"><img src="{}" alt="{}" loading="lazy"/>{}</figure>"#,
                hero, meta.title, caption
            )
        } else {
            String::new()
        };

        let series_html = if let Some(series) = &meta.series {
            let mut series_posts: Vec<&PostMeta> = index
                .posts
                .iter()
                .filter(|p| p.series.as_deref() == Some(series.as_str()))
                .collect();

            series_posts.sort_by(|a, b| match (a.series_part, b.series_part) {
                (Some(a_part), Some(b_part)) => a_part.cmp(&b_part),
                _ => a.date.cmp(&b.date),
            });

            let total = series_posts.len();
            let current_idx = series_posts
                .iter()
                .position(|p| p.slug == meta.slug)
                .unwrap_or(0);

            let prev = if current_idx > 0 {
                Some(series_posts[current_idx - 1])
            } else {
                None
            };
            let next = if current_idx + 1 < total {
                Some(series_posts[current_idx + 1])
            } else {
                None
            };

            let list_html: String = series_posts
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let active = if p.slug == meta.slug { "current" } else { "" };
                    format!(
                        r#"<li class="series-item {}"><a href="/post/{}">{}.</a><span>{}</span></li>"#,
                        active,
                        p.slug,
                        i + 1,
                        p.title
                    )
                })
                .collect();

            let nav_html = format!(
                r#"
                <div class="series-nav-links">
                    {}
                    {}
                </div>
            "#,
                prev.map(|p| format!(
                    r#"<a href="/post/{}" class="series-link">← {}</a>"#,
                    p.slug, p.title
                ))
                .unwrap_or_default(),
                next.map(|p| format!(
                    r#"<a href="/post/{}" class="series-link">{} →</a>"#,
                    p.slug, p.title
                ))
                .unwrap_or_default(),
            );

            format!(
                r#"
                <section class="series-nav">
                    <div class="series-header">
                        <span class="series-label">Series</span>
                        <h3>{}</h3>
                        <p>Part {} of {}</p>
                    </div>
                    <ol class="series-list">
                        {}
                    </ol>
                    {}
                </section>
            "#,
                series,
                current_idx + 1,
                total,
                list_html,
                nav_html
            )
        } else {
            String::new()
        };

        let related = related_posts(index, post, 3);
        let related_html = if !related.is_empty() {
            let cards: String = related
                .iter()
                .map(|meta| self.render_related_card(meta))
                .collect();
            format!(
                r#"
                <section class="related-posts">
                    <h3>Related posts</h3>
                    <div class="related-grid">{}</div>
                </section>
            "#,
                cards
            )
        } else {
            String::new()
        };

        let deck_html = if !meta.summary.is_empty() {
            format!(r#"<p class="post-deck">{}</p>"#, meta.summary)
        } else {
            String::new()
        };

        article.set_inner_html(&format!(
            r#"
            {hero}
            <header class="post-header">
                <h1>{title}</h1>
                {deck}
                <div class="post-meta">
                    <span class="meta-author">By {author}</span>
                    <span class="meta-topic">{topic}</span>
                    <time>Published {date}</time>
                    {updated}
                    <span class="reading-time">{reading} min read</span>
                    {ai_badge}
                </div>
                <div class="post-tags">{tags}</div>
            </header>
            {series}
            {ai_note}
            <div class="post-content">
                {content}
            </div>
            {related}
            <footer class="post-footer">
                <a href="/" class="back-link">← Back to Blog</a>
            </footer>
        "#,
            hero = hero_html,
            title = meta.title,
            deck = deck_html,
            author = author,
            topic = topic_html,
            date = meta.date,
            updated = updated_html,
            reading = reading_time,
            ai_badge = ai_badge,
            tags = tags_html,
            series = series_html,
            ai_note = ai_note,
            content = content_html,
            related = related_html
        ));

        self.root.append_child(&article)?;

        // Render mermaid diagrams after content is in DOM
        render_mermaid();

        Ok(())
    }

    /// Render a post card for listings
    fn render_post_card(&self, meta: &PostMeta) -> String {
        let search = escape_attr(&search_blob(meta));
        let topic = topic_for(meta);
        let tags: String = meta
            .tags
            .iter()
            .take(3)
            .map(|t| format!(r#"<span class="card-tag">{}</span>"#, t))
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"
            <article class="post-card" data-search="{}">
                <a href="/post/{}">
                    <div class="card-meta">
                        <span class="card-topic">#{}</span>
                        <time>{}</time>
                    </div>
                    <h3>{}</h3>
                    <p>{}</p>
                    <div class="card-tags">{}</div>
                </a>
            </article>
        "#,
            search, meta.slug, topic, meta.date, meta.title, meta.summary, tags
        )
    }

    fn render_feature_card(&self, meta: &PostMeta) -> String {
        let search = escape_attr(&search_blob(meta));
        let topic = topic_for(meta);
        format!(
            r#"
            <article class="feature-card" data-search="{}">
                <a href="/post/{}">
                    <span class="feature-topic">#{}</span>
                    <h3>{}</h3>
                    <p>{}</p>
                    <div class="feature-meta">
                        <time>{}</time>
                    </div>
                </a>
            </article>
        "#,
            search, meta.slug, topic, meta.title, meta.summary, meta.date
        )
    }

    fn render_related_card(&self, meta: &PostMeta) -> String {
        format!(
            r#"
            <a class="related-card" href="/post/{}">
                <span class="related-topic">#{}</span>
                <h4>{}</h4>
                <p>{}</p>
            </a>
        "#,
            meta.slug,
            topic_for(meta),
            meta.title,
            meta.summary
        )
    }

    /// Render posts filtered by tag
    pub fn render_tag(&self, tag: &str, index: &BlogIndex) -> Result<(), JsValue> {
        self.clear();

        let title = format!("#{} | Blog | too.foo", tag);
        let description = format!("Posts tagged with #{}.", tag);
        let url = format!("{}/tag/{}", BASE_URL, tag);
        self.set_title(&title);
        self.set_meta_content("meta[name=\"description\"]", &description);
        self.set_meta_content("meta[property=\"og:title\"]", &title);
        self.set_meta_content("meta[property=\"og:description\"]", &description);
        self.set_meta_content("meta[property=\"og:url\"]", &url);
        self.set_meta_content("meta[property=\"og:type\"]", "website");
        self.set_meta_content("meta[name=\"twitter:title\"]", &title);
        self.set_meta_content("meta[name=\"twitter:description\"]", &description);
        self.set_canonical(&url);

        let container = self.create_element("div", "blog-tag-page")?;

        let posts = index.by_tag(tag);
        let posts_html: String = posts
            .iter()
            .map(|meta| self.render_post_card(meta))
            .collect();

        container.set_inner_html(&format!(
            r#"
            <header class="tag-header">
                <h1>#{}</h1>
                <p>{} posts</p>
                <a href="/" class="back-link">← All Posts</a>
            </header>
            <section class="tag-posts">
                <div class="search-box">
                    <label for="post-search">Search this tag</label>
                    <input id="post-search" type="search" placeholder="Search posts in this tag" />
                    <p id="search-empty" class="search-empty">No posts match that search.</p>
                </div>
                {}
            </section>
        "#,
            tag,
            posts.len(),
            posts_html
        ));

        self.root.append_child(&container)?;
        Ok(())
    }

    /// Render 404 page
    pub fn render_404(&self) -> Result<(), JsValue> {
        self.clear();
        let container = self.create_element("div", "not-found")?;
        container.set_inner_html(
            r#"
            <h1>404</h1>
            <p>Post not found</p>
            <a href="/">← Back to Blog</a>
        "#,
        );
        self.root.append_child(&container)?;
        Ok(())
    }

    fn create_element(&self, tag: &str, class: &str) -> Result<Element, JsValue> {
        let el = self.document.create_element(tag)?;
        el.set_class_name(class);
        Ok(el)
    }
}
