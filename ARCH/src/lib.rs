//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | ARCH/src/lib.rs
//! PURPOSE: Interactive canvas-based architecture explorer showing crates, layers, and dependencies
//! MODIFIED: 2025-12-09
//! LAYER: ARCH (architecture explorer)
//! ═══════════════════════════════════════════════════════════════════════════════

//! ARCH - Architecture Visualization
//!
//! Hierarchical card-based view of the antimony-labs monorepo.

#![allow(unexpected_cfgs)]

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, WheelEvent};

mod audit;
mod graph;
pub use audit::{CrateAudit, GitMetadata, ValidationStatus};
pub use graph::{CrateInfo, CrateLayer, DependencyGraph};

const WORKSPACE_DATA: &str = include_str!("workspace_data.json");
const DOC_DB_JSON: &str = include_str!("db.json");

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
struct DocEntry {
    path: String,
    name: String,
    purpose: String,
    main_function: String,
}

// ============================================================================
// COLORS
// ============================================================================

struct Colors;
#[allow(dead_code)]
impl Colors {
    const BG: &'static str = "#0a0a0f";
    const CARD_BG: &'static str = "#14141f";
    const CARD_BORDER: &'static str = "#2a2a3a"; // Future: non-hover border
    const CARD_HOVER: &'static str = "#3b82f6";
    const TEXT_PRIMARY: &'static str = "#ffffff";
    const TEXT_SECONDARY: &'static str = "#888899";
    const TEXT_MUTED: &'static str = "#555566";

    const DNA: &'static str = "#3b82f6"; // Blue
    const CORE: &'static str = "#14b8a6"; // Teal
    const PROJECT: &'static str = "#a855f7"; // Purple
    const TOOL: &'static str = "#f59e0b"; // Amber
    const LEARN: &'static str = "#22c55e"; // Green
}

// ============================================================================
// CARD LAYOUT
// ============================================================================

#[derive(Clone)]
#[allow(dead_code)]
struct Card {
    name: String,
    description: String,
    color: &'static str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    children: Vec<String>,
    expanded: bool,
    audit: Option<CrateAudit>,
    source_path: String,
}

struct AppState {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    scroll_y: f64,
    hovered_card: Option<String>,
    cards: Vec<Card>,
    docs: HashMap<String, DocEntry>,
    modal_open: bool,
    selected_card_docs: Vec<DocEntry>,
}

impl AppState {
    fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
        // Handle high DPI displays
        let window = window().unwrap();
        let dpr = window.device_pixel_ratio();
        let rect = canvas.get_bounding_client_rect();

        let width = rect.width() * dpr;
        let height = rect.height() * dpr;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        ctx.scale(dpr, dpr).ok();

        let graph: DependencyGraph = serde_json::from_str(WORKSPACE_DATA).unwrap_or_default();
        let docs: HashMap<String, DocEntry> = serde_json::from_str(DOC_DB_JSON).unwrap_or_default();

        let mut state = Self {
            canvas,
            ctx,
            width: rect.width(),   // Logical width
            height: rect.height(), // Logical height
            scroll_y: 0.0,
            hovered_card: None,
            cards: Vec::new(),
            docs,
            modal_open: false,
            selected_card_docs: Vec::new(),
        };

        state.build_cards(&graph);
        state
    }

    fn build_cards(&mut self, graph: &DependencyGraph) {
        let card_width = 160.0;
        let card_height = 50.0;
        let padding = 20.0;
        let section_gap = 60.0;

        let center_x = self.width / 2.0;
        let mut y = 70.0;

        // 1. DNA (Top)
        let dna_width = 240.0;
        self.cards.push(Card {
            name: "DNA".to_string(),
            description: "Foundation (Physics, CAD)".to_string(),
            color: Colors::DNA,
            x: center_x - dna_width / 2.0,
            y,
            width: dna_width,
            height: 60.0,
            children: vec![],
            expanded: true,
            audit: Some(CrateAudit::new("DNA".to_string())),
            source_path: "DNA".to_string(),
        });
        y += 60.0 + section_gap;

        // 2. The 5 Pillars: TOOLS | SIMULATION | HELIOS | BLOG | LEARN
        let pillars = vec![
            ("TOOLS", Colors::TOOL, "Engineering Utils"),
            ("SIMULATION", Colors::CORE, "Physics Sims"),
            ("HELIOS", Colors::PROJECT, "Solar Data"),
            ("BLOG", Colors::PROJECT, "Bitinary Labs"),
            ("LEARN", Colors::LEARN, "Tutorials"),
        ];

        let pillar_width = 160.0;
        let total_pillars_width = 5.0 * pillar_width + 4.0 * padding;
        let start_x = (self.width - total_pillars_width) / 2.0;

        let mut current_x = start_x;
        let pillar_y = y;

        for (name, color, desc) in pillars {
            self.cards.push(Card {
                name: name.to_string(),
                description: desc.to_string(),
                color,
                x: current_x,
                y: pillar_y,
                width: pillar_width,
                height: 50.0,
                children: vec![], // Populated via docs?
                expanded: true,
                audit: None,
                source_path: name.to_string(), // Folder path match
            });

            // 3. Children under each pillar
            // We search for crates where path starts with "{Pillar}/"
            // OR if it is a CORE item like "TOOLS/CORE/..." or "SIMULATION/CORE/..."

            // Engines first (Core)
            let mut sub_y = pillar_y + 50.0 + 20.0;

            // Find Core/Engines for this pillar
            // E.g. TOOLS -> TOOLS/CORE/*
            // SIMULATIONS -> SIMULATION/CORE/*
            let core_items: Vec<_> = graph
                .crates
                .iter()
                .filter(|c| c.path.starts_with(&format!("{}/CORE", name)))
                .collect();

            for item in core_items {
                let display_name = item.name.replace("-engine", "").to_uppercase();
                self.cards.push(Card {
                    name: item.name.clone(),
                    description: display_name, // Engine name
                    color: Colors::CORE,       // Highlight as core
                    x: current_x + 10.0,       // Indent
                    y: sub_y,
                    width: pillar_width - 20.0,
                    height: 40.0,
                    children: vec![],
                    expanded: false,
                    audit: Some(CrateAudit::new(item.name.clone())),
                    source_path: item.path.clone(),
                });
                sub_y += 45.0;
            }

            // Regular items (Non-Core)
            // e.g. TOOLS/AUTOCRATE (but not TOOLS/CORE/...)
            let items: Vec<_> = graph
                .crates
                .iter()
                .filter(|c| c.path.starts_with(name) && !c.path.contains("/CORE"))
                // Exclude the pillar itself if it's in the list (HELIOS, BLOG)
                .filter(|c| c.path != name.to_string())
                .collect();

            for item in items {
                // Shorten name
                let display_name = item
                    .name
                    .replace("-learn", "")
                    .replace("_", " ")
                    .to_uppercase();

                self.cards.push(Card {
                    name: item.name.clone(),
                    description: display_name,
                    color: color, // Inherit pillar color
                    x: current_x + 10.0,
                    y: sub_y,
                    width: pillar_width - 20.0,
                    height: 40.0,
                    children: vec![],
                    expanded: false,
                    audit: Some(CrateAudit::new(item.name.clone())),
                    source_path: item.path.clone(),
                });
                sub_y += 45.0;
            }

            current_x += pillar_width + padding;
        }
    }

    fn card_at(&self, x: f64, y: f64) -> Option<String> {
        let scroll_y = y + self.scroll_y;
        for card in self.cards.iter().rev() {
            if x >= card.x
                && x <= card.x + card.width
                && scroll_y >= card.y
                && scroll_y <= card.y + card.height
            {
                return Some(card.name.clone());
            }
        }
        None
    }

    fn render(&self) {
        let ctx = &self.ctx;

        // Clear
        ctx.set_fill_style(&JsValue::from_str(Colors::BG));
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        // Apply scroll
        ctx.save();
        ctx.translate(0.0, -self.scroll_y).ok();

        // Draw cards
        for card in &self.cards {
            self.draw_card(card);
        }

        ctx.restore();

        // Draw header
        self.draw_header();

        // Draw Modal
        self.draw_modal();
    }

    fn draw_modal(&self) {
        if !self.modal_open {
            return;
        }
        let ctx = &self.ctx;

        // Overlay
        ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.85)"));
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        // Modal Window
        let modal_w = 800.0;
        let modal_h = 600.0;
        let x = (self.width - modal_w) / 2.0;
        let y = (self.height - modal_h) / 2.0;

        ctx.set_fill_style(&JsValue::from_str("#0f0f12"));
        self.rounded_rect(x, y, modal_w, modal_h, 8.0);
        ctx.fill();

        ctx.set_stroke_style(&JsValue::from_str("#3b82f6"));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Header
        ctx.set_fill_style(&JsValue::from_str("#1f1f2e"));
        self.rounded_rect(x, y, modal_w, 50.0, 8.0); // Top rounded only? Simplification: just fill
        ctx.fill_rect(x, y, modal_w, 50.0);

        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 16px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        ctx.fill_text("DOCUMENTATION TERMINAL", x + 20.0, y + 30.0)
            .ok();

        // Content
        let mut content_y = y + 80.0;
        ctx.set_font("14px 'JetBrains Mono', monospace");

        if self.selected_card_docs.is_empty() {
            ctx.set_fill_style(&JsValue::from_str("#888899"));
            ctx.fill_text(
                "No documentation found for this module.",
                x + 20.0,
                content_y,
            )
            .ok();
        } else {
            for doc in self.selected_card_docs.iter().take(15) {
                // Limit items
                ctx.set_fill_style(&JsValue::from_str("#22c55e")); // Green
                ctx.fill_text(&format!("FILE: {}", doc.name), x + 20.0, content_y)
                    .ok();
                content_y += 20.0;

                ctx.set_fill_style(&JsValue::from_str("#aaaaaa"));
                ctx.fill_text(&format!("PURPOSE: {}", doc.purpose), x + 40.0, content_y)
                    .ok();
                content_y += 20.0;

                ctx.set_fill_style(&JsValue::from_str("#3b82f6"));
                ctx.fill_text(&format!("MAIN: {}", doc.main_function), x + 40.0, content_y)
                    .ok();
                content_y += 30.0;
            }
        }
    }

    fn draw_card(&self, card: &Card) {
        let ctx = &self.ctx;
        let is_hovered = self.hovered_card.as_ref() == Some(&card.name);

        // Card background
        ctx.set_fill_style(&JsValue::from_str(Colors::CARD_BG));
        self.rounded_rect(card.x, card.y, card.width, card.height, 8.0);
        ctx.fill();

        // Border
        let border_color = if is_hovered {
            Colors::CARD_HOVER
        } else {
            card.color
        };
        ctx.set_stroke_style(&JsValue::from_str(border_color));
        ctx.set_line_width(if is_hovered { 2.0 } else { 1.0 });
        self.rounded_rect(card.x, card.y, card.width, card.height, 8.0);
        ctx.stroke();

        // Left accent bar
        ctx.set_fill_style(&JsValue::from_str(card.color));
        self.rounded_rect(card.x, card.y, 4.0, card.height, 2.0);
        ctx.fill();

        // Title
        ctx.set_fill_style(&JsValue::from_str(Colors::TEXT_PRIMARY));
        ctx.set_font("bold 14px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        ctx.fill_text(&card.description, card.x + 16.0, card.y + 12.0)
            .ok();

        // Subtitle (name if different)
        if card.name != card.description && !card.children.is_empty() {
            ctx.set_fill_style(&JsValue::from_str(Colors::TEXT_MUTED));
            ctx.set_font("11px 'JetBrains Mono', monospace");
            ctx.fill_text(
                &format!("{} items", card.children.len()),
                card.x + 16.0,
                card.y + 32.0,
            )
            .ok();
        }
    }

    fn rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        let ctx = &self.ctx;
        ctx.begin_path();
        ctx.move_to(x + r, y);
        ctx.line_to(x + w - r, y);
        ctx.arc_to(x + w, y, x + w, y + r, r).ok();
        ctx.line_to(x + w, y + h - r);
        ctx.arc_to(x + w, y + h, x + w - r, y + h, r).ok();
        ctx.line_to(x + r, y + h);
        ctx.arc_to(x, y + h, x, y + h - r, r).ok();
        ctx.line_to(x, y + r);
        ctx.arc_to(x, y, x + r, y, r).ok();
        ctx.close_path();
    }

    fn draw_header(&self) {
        let ctx = &self.ctx;

        // Header background
        ctx.set_fill_style(&JsValue::from_str("rgba(10, 10, 15, 0.95)"));
        ctx.fill_rect(0.0, 0.0, self.width, 50.0);

        // Title
        ctx.set_fill_style(&JsValue::from_str(Colors::TEXT_PRIMARY));
        ctx.set_font("bold 16px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        ctx.fill_text("ARCH", 20.0, 25.0).ok();

        ctx.set_fill_style(&JsValue::from_str(Colors::TEXT_SECONDARY));
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.fill_text("Architecture Explorer", 80.0, 25.0).ok();

        // Stats
        ctx.set_text_align("right");
        ctx.set_fill_style(&JsValue::from_str(Colors::TEXT_MUTED));
        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.fill_text("antimony-labs monorepo", self.width - 20.0, 25.0)
            .ok();
    }
}

// ============================================================================
// WASM ENTRY
// ============================================================================

thread_local! {
    static APP: RefCell<Option<AppState>> = const { RefCell::new(None) };
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("No canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    let _container = document
        .get_element_by_id("canvas-container")
        .ok_or("No container")?;

    let ctx = canvas
        .get_context("2d")?
        .ok_or("No 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let state = AppState::new(canvas.clone(), ctx);
    APP.with(|app| *app.borrow_mut() = Some(state));

    render();
    setup_events(&document, &canvas)?;

    Ok(())
}

fn setup_events(_document: &web_sys::Document, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    // Note: _document prefixed to suppress unused warning
    // Mouse move
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let should_render = APP.with(|app| {
            if let Some(ref mut state) = *app.borrow_mut() {
                let rect = state.canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                let old_hover = state.hovered_card.clone();
                state.hovered_card = state.card_at(x, y);
                state.hovered_card != old_hover
            } else {
                false
            }
        });
        if should_render {
            render();
        }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Click
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let should_render = APP.with(|app| {
            if let Some(ref mut state) = *app.borrow_mut() {
                let rect = state.canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();

                if state.modal_open {
                    state.modal_open = false;
                    true
                } else if let Some(card_name) = state.card_at(x, y) {
                    if let Some(card) = state.cards.iter().find(|c| c.name == card_name).cloned() {
                        state.modal_open = true;
                        let search_path = &card.source_path;
                        if !search_path.is_empty() {
                            state.selected_card_docs = state
                                .docs
                                .values()
                                .filter(|d| d.path.starts_with(search_path))
                                .cloned()
                                .collect();

                            state.selected_card_docs.sort_by(|a, b| {
                                let score_a = if a.name.starts_with("README") {
                                    0
                                } else if a.name == "lib.rs" || a.name == "main.rs" {
                                    1
                                } else if a.name == "mod.rs" {
                                    2
                                } else {
                                    3
                                };
                                let score_b = if b.name.starts_with("README") {
                                    0
                                } else if b.name == "lib.rs" || b.name == "main.rs" {
                                    1
                                } else if b.name == "mod.rs" {
                                    2
                                } else {
                                    3
                                };
                                if score_a != score_b {
                                    score_a.cmp(&score_b)
                                } else {
                                    a.path.cmp(&b.path)
                                }
                            });
                        } else {
                            state.selected_card_docs.clear();
                        }
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });
        if should_render {
            render();
        }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Wheel scroll
    let closure = Closure::wrap(Box::new(move |event: WheelEvent| {
        event.prevent_default();
        APP.with(|app| {
            if let Some(ref mut state) = *app.borrow_mut() {
                state.scroll_y = (state.scroll_y + event.delta_y() * 0.5).max(0.0);
            }
        });
        render();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn render() {
    APP.with(|app| {
        if let Some(ref state) = *app.borrow() {
            state.render();
        }
    });
}
