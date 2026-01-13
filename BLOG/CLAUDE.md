# Blog - Markdown Blog Engine

Rust/WASM blog engine with markdown support and AI-generated content indicators.

## Build & Run

```bash
trunk serve BLOG/index.html --open
trunk build --release BLOG/index.html
```

## Architecture

```
BLOG/
  src/
    lib.rs       # Core types, markdown parsing, heading ID generation
    router.rs    # Client-side routing
    render.rs    # DOM rendering, mermaid integration
  posts/         # Markdown blog posts
  index.html     # Entry point, styles, theme toggle, navigation
```

## Writing Posts

Posts are markdown files in `posts/` with YAML frontmatter:

```markdown
---
title: "Post Title"
slug: "post-slug"
date: "2025-01-15"
tags: [rust, wasm, tutorial]
summary: "Brief description for listings"
draft: false
ai_generated: true
---

**Jump to:** [Section 1](#section-1-title) | [Section 2](#section-2-title) | ...

## Introduction

Your content here...

## Section 1: Title

More content...
```

### Frontmatter Fields

| Field | Required | Description |
|-------|----------|-------------|
| title | Yes | Display title |
| slug | Yes | URL path (/post/{slug}) |
| date | Yes | Publication date (YYYY-MM-DD) |
| tags | No | Array of tags for filtering |
| summary | No | Short description for cards |
| draft | No | If true, not shown in listings |
| ai_generated | No | Shows AI badge if true |

## Style Guide

### Writing Standards

1. **No em-dashes**: Use regular hyphens (-) instead of em-dashes (—). Em-dashes feel AI-generated.
   - Bad: `robots — machines that move`
   - Good: `robots - machines that move`

2. **No horizontal rule dividers**: Do not use `---` between sections. Let headings provide natural separation.

3. **No excessive formatting**: Avoid overuse of bold, italics, or decorative elements.

4. **Natural transitions**: Use prose to transition between sections, not visual dividers.

5. **Text is justified**: Paragraphs and list items use `text-align: justify`.

6. **Line height**: Set to 1.5 for readability.

### Jump Navigation

For long posts, add a "Jump to" navigation at the top:

```markdown
**Jump to:** [Section Name](#section-slug) | [Another Section](#another-section) | ...
```

Anchor IDs are auto-generated from heading text:
- "Part 1: The Problem" → `#part-1-the-problem`
- "Vision-Language Models" → `#vision-language-models`
- Colons are removed, spaces become hyphens, all lowercase

### Diagrams

**Mermaid diagrams** (preferred for flowcharts/architecture):
```html
<div class="mermaid">
flowchart LR
    A[Input] --> B[Process]
    B --> C[Output]
</div>
```
- Automatically centered
- Themed for light/dark mode
- Use `flowchart`, `sequenceDiagram`, `classDiagram`, etc.

**Images** (center-aligned):
```html
<div style="text-align: center;">

![Description](path/to/image.png)

</div>
```

**ASCII diagrams**: Use code blocks for ASCII art.

## UI Features

### Theme Toggle
- Animated SVG sun/moon icons
- Sun: rays slowly extend/retract
- Moon: gentle opacity pulse
- Persisted in localStorage
- Respects system preference

### Sticky Navigation
- Top nav bar stays visible on scroll
- Frosted glass effect (backdrop-filter blur)
- Contains: logo, Archive, About, Home, theme toggle

### Floating Navigation Buttons
Three buttons in bottom-right corner:
- **↑** Back to top
- **◀** Previous section (h2/h3)
- **▶** Next section (h2/h3)

Buttons auto-disable when at start/end of content.

### Background
- Subtle cyan dot pattern
- 24px spacing, 15% opacity
- Adapts to light/dark theme

## Color Scheme

Single accent color (cyan/teal) for both themes:

**Dark mode:**
- Background: `#050508`
- Surface: `#0a0a10`
- Accent: `#00d4d4`
- Text: `#c8c8c8`

**Light mode:**
- Background: `#fafafa`
- Surface: `#ffffff`
- Accent: `#007a7a`
- Text: `#2a2a2a`

## Technical Implementation

### Heading IDs (lib.rs)
The `render_html()` function post-processes HTML to add IDs to headings:
- Extracts heading text
- Converts to lowercase
- Keeps only alphanumeric, spaces, hyphens
- Replaces spaces with hyphens
- Example: `<h2>Part 1: Hello</h2>` → `<h2 id="part-1-hello">Part 1: Hello</h2>`

### Anchor Link Handling (index.html)
JavaScript intercepts clicks on `#anchor` links:
1. First tries `document.getElementById(targetId)`
2. Falls back to searching headings by normalized text
3. Smooth scrolls with -80px offset (for sticky nav)

### Mermaid Integration (render.rs)
After WASM renders content, calls `window.renderMermaid()` which:
- Re-initializes mermaid with current theme colors
- Runs mermaid on `.mermaid` elements

## Adding a New Post

1. Create `posts/your-post-slug.md` with frontmatter
2. Add filename to `posts/index.json`
3. Include "Jump to" navigation for long posts
4. Use mermaid for diagrams
5. Avoid em-dashes and horizontal rules
6. Test both light and dark themes

## Implementation Status

- [x] Markdown parsing with frontmatter
- [x] Client-side router
- [x] Post rendering with heading IDs
- [x] Tag filtering
- [x] Mermaid diagram support
- [x] Light/dark theme with toggle
- [x] Sticky navigation
- [x] Floating section navigation
- [x] Jump-to anchor links
- [ ] Search functionality
- [ ] RSS feed generation
- [ ] Syntax highlighting (hljs integration)
- [ ] Comments (via external service)
