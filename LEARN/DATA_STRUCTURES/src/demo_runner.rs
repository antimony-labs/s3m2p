//! ===============================================================================
//! FILE: demo_runner.rs | DATA_STRUCTURES/src/demo_runner.rs
//! PURPOSE: Demo runners for Data Structures interactive lessons
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> DATA_STRUCTURES
//! ===============================================================================

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use learn_web::Canvas;
use learn_core::{Demo, demos::*};
use learn_core::demos::pseudocode::Pseudocode;
use std::cell::RefCell;

// ===============================================================================
// THEME DETECTION AND COLORS
// ===============================================================================

/// Colors for pseudocode panel rendering
struct PseudocodeColors {
    panel_bg: &'static str,
    panel_border: &'static str,
    title_color: &'static str,
    code_normal: &'static str,
    code_highlighted: &'static str,
    highlight_bg: &'static str,
    highlight_bar: &'static str,
    line_number: &'static str,
}

/// Dark theme colors (default)
const DARK_COLORS: PseudocodeColors = PseudocodeColors {
    panel_bg: "rgba(20, 20, 30, 0.95)",
    panel_border: "#333344",
    title_color: "#00d4aa",
    code_normal: "#888899",
    code_highlighted: "#ffffff",
    highlight_bg: "rgba(0, 212, 170, 0.2)",
    highlight_bar: "#00d4aa",
    line_number: "#555566",
};

/// Light theme colors
const LIGHT_COLORS: PseudocodeColors = PseudocodeColors {
    panel_bg: "rgba(255, 255, 255, 0.98)",
    panel_border: "#d0d0d8",
    title_color: "#00897B",
    code_normal: "#555566",
    code_highlighted: "#1A1A2E",
    highlight_bg: "rgba(0, 137, 123, 0.15)",
    highlight_bar: "#00897B",
    line_number: "#9999aa",
};

/// Detect current theme from document's data-theme attribute
fn get_current_theme_colors() -> &'static PseudocodeColors {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(doc_element) = document.document_element() {
                if let Some(theme) = doc_element.get_attribute("data-theme") {
                    if theme == "light" {
                        return &LIGHT_COLORS;
                    }
                }
            }
        }
    }
    &DARK_COLORS
}

// Thread-local state for active demo
thread_local! {
    static ACTIVE_DEMO: RefCell<Option<ActiveDemo>> = const { RefCell::new(None) };
    static ANIMATION_ID: RefCell<Option<i32>> = const { RefCell::new(None) };
}

enum ActiveDemo {
    Array(ArrayDemo),
    LinkedList(LinkedListDemo),
    Stack(StackDemo),
    Queue(QueueDemo),
    BinaryTree(BinaryTreeDemo),
    Bst(BstDemo),
    Heap(HeapDemo),
    HashTable(HashTableDemo),
    Graph(GraphDemo),
    BalancedTree(BalancedTreeDemo),
}

/// Dispatch to the appropriate demo based on lesson index
pub fn start_demo_for_lesson(lesson_idx: usize, canvas_id: &str, seed: u64) -> Result<(), JsValue> {
    stop_demo();

    let demo = match lesson_idx {
        0 => {
            let mut d = ArrayDemo::default();
            d.reset(seed);
            ActiveDemo::Array(d)
        }
        1 => {
            let mut d = LinkedListDemo::default();
            d.reset(seed);
            ActiveDemo::LinkedList(d)
        }
        2 => {
            let mut d = StackDemo::default();
            d.reset(seed);
            ActiveDemo::Stack(d)
        }
        3 => {
            let mut d = QueueDemo::default();
            d.reset(seed);
            ActiveDemo::Queue(d)
        }
        4 => {
            let mut d = BinaryTreeDemo::default();
            d.reset(seed);
            ActiveDemo::BinaryTree(d)
        }
        5 => {
            let mut d = BstDemo::default();
            d.reset(seed);
            ActiveDemo::Bst(d)
        }
        6 => {
            let mut d = HeapDemo::default();
            d.reset(seed);
            ActiveDemo::Heap(d)
        }
        7 => {
            let mut d = HashTableDemo::default();
            d.reset(seed);
            ActiveDemo::HashTable(d)
        }
        8 => {
            let mut d = GraphDemo::default();
            d.reset(seed);
            ActiveDemo::Graph(d)
        }
        9 => {
            let mut d = BalancedTreeDemo::default();
            d.reset(seed);
            ActiveDemo::BalancedTree(d)
        }
        _ => return Err(JsValue::from_str("Invalid lesson index")),
    };

    ACTIVE_DEMO.with(|d| {
        *d.borrow_mut() = Some(demo);
    });

    // Initial render
    render_demo(canvas_id)?;

    // Start animation loop
    start_animation_loop(canvas_id.to_string());

    Ok(())
}

/// Stop all running demos
pub fn stop_demo() {
    ANIMATION_ID.with(|id| {
        if let Some(anim_id) = id.borrow_mut().take() {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(anim_id).ok();
            }
        }
    });

    ACTIVE_DEMO.with(|d| {
        *d.borrow_mut() = None;
    });
}

fn start_animation_loop(canvas_id: String) {
    let callback = Closure::wrap(Box::new(move || {
        ACTIVE_DEMO.with(|demo| {
            if let Some(d) = demo.borrow_mut().as_mut() {
                match d {
                    ActiveDemo::Array(demo) => demo.step(0.016),
                    ActiveDemo::LinkedList(demo) => demo.step(0.016),
                    ActiveDemo::Stack(demo) => demo.step(0.016),
                    ActiveDemo::Queue(demo) => demo.step(0.016),
                    ActiveDemo::BinaryTree(demo) => demo.step(0.016),
                    ActiveDemo::Bst(demo) => demo.step(0.016),
                    ActiveDemo::Heap(demo) => demo.step(0.016),
                    ActiveDemo::HashTable(demo) => demo.step(0.016),
                    ActiveDemo::Graph(demo) => demo.step(0.016),
                    ActiveDemo::BalancedTree(demo) => demo.step(0.016),
                }
            }
        });

        let _ = render_demo(&canvas_id);
        start_animation_loop(canvas_id.clone());
    }) as Box<dyn FnMut()>);

    if let Some(window) = web_sys::window() {
        let id = window.request_animation_frame(callback.as_ref().unchecked_ref()).ok();
        ANIMATION_ID.with(|anim_id| {
            *anim_id.borrow_mut() = id;
        });
    }

    callback.forget();
}

fn render_demo(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = Canvas::new(canvas_id)?;

    ACTIVE_DEMO.with(|demo| {
        if let Some(d) = demo.borrow().as_ref() {
            match d {
                ActiveDemo::Array(demo) => render_array(&canvas, demo),
                ActiveDemo::LinkedList(demo) => render_linked_list(&canvas, demo),
                ActiveDemo::Stack(demo) => render_stack(&canvas, demo),
                ActiveDemo::Queue(demo) => render_queue(&canvas, demo),
                ActiveDemo::BinaryTree(demo) => render_binary_tree(&canvas, demo),
                ActiveDemo::Bst(demo) => render_bst(&canvas, demo),
                ActiveDemo::Heap(demo) => render_heap(&canvas, demo),
                ActiveDemo::HashTable(demo) => render_hash_table(&canvas, demo),
                ActiveDemo::Graph(demo) => render_graph(&canvas, demo),
                ActiveDemo::BalancedTree(demo) => render_balanced_tree(&canvas, demo),
            }
        }
    });

    Ok(())
}

// ===============================================================================
// RENDERING FUNCTIONS
// ===============================================================================

fn render_array(canvas: &Canvas, demo: &ArrayDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let box_w = 60.0;
    let box_h = 50.0;
    let start_x = (w - demo.capacity as f64 * box_w) / 2.0;
    let y = h / 2.0 - box_h / 2.0;

    // Draw array boxes
    for i in 0..demo.capacity {
        let x = start_x + i as f64 * box_w + demo.get_shift_offset(i) as f64 * box_w;

        // Background
        let fill = if Some(i) == demo.highlight_index {
            "rgba(0, 212, 170, 0.3)"
        } else if i < demo.size {
            "rgba(0, 212, 170, 0.1)"
        } else {
            "rgba(100, 100, 100, 0.05)"
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.fill_rect(x, y, box_w - 4.0, box_h);

        // Border
        let stroke = if Some(i) == demo.highlight_index {
            "#00d4aa"
        } else if i < demo.size {
            "#00d4aa"
        } else {
            "#444444"
        };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, y, box_w - 4.0, box_h);

        // Value
        if let Some(val) = demo.get(i) {
            // Check for fading
            let alpha = demo.is_fading(i).unwrap_or(1.0);
            ctx.set_fill_style(&JsValue::from_str(&format!("rgba(255, 255, 255, {})", alpha)));
            ctx.set_font("bold 18px 'JetBrains Mono', monospace");
            ctx.set_text_align("center");
            ctx.fill_text(&val.to_string(), x + box_w / 2.0 - 2.0, y + box_h / 2.0 + 6.0).ok();
        }

        // Appearing element
        if let Some((val, alpha)) = demo.is_appearing(i) {
            ctx.set_fill_style(&JsValue::from_str(&format!("rgba(0, 255, 200, {})", alpha)));
            ctx.fill_text(&val.to_string(), x + box_w / 2.0 - 2.0, y + box_h / 2.0 + 6.0).ok();
        }

        // Index label
        ctx.set_fill_style(&JsValue::from_str("#666666"));
        ctx.set_font("12px 'JetBrains Mono', monospace");
        ctx.fill_text(&i.to_string(), x + box_w / 2.0 - 2.0, y + box_h + 18.0).ok();
    }

    // Status message
    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_linked_list(canvas: &Canvas, demo: &LinkedListDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_size = 45.0;
    let order = demo.get_traversal_order();

    // Draw edges first
    ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
    ctx.set_line_width(2.0);

    for &idx in &order {
        if let Some(next_idx) = demo.nodes[idx].next {
            let from = demo.nodes[idx].position;
            let to = demo.nodes[next_idx].position;

            ctx.begin_path();
            ctx.move_to(from.x as f64 + node_size / 2.0, from.y as f64);
            ctx.line_to(to.x as f64 - node_size / 2.0, to.y as f64);
            ctx.stroke();

            // Arrow head
            ctx.begin_path();
            ctx.move_to(to.x as f64 - node_size / 2.0 - 10.0, to.y as f64 - 6.0);
            ctx.line_to(to.x as f64 - node_size / 2.0, to.y as f64);
            ctx.line_to(to.x as f64 - node_size / 2.0 - 10.0, to.y as f64 + 6.0);
            ctx.stroke();
        }
    }

    // Draw nodes
    for (i, &idx) in order.iter().enumerate() {
        let node = &demo.nodes[idx];
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        // Background
        let fill = if demo.highlight == Some(idx) {
            "rgba(0, 212, 170, 0.4)"
        } else {
            "rgba(0, 212, 170, 0.15)"
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_size / 2.0, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Value
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 16px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&node.value.to_string(), x, y + 5.0).ok();

        // Position label (HEAD, position, NULL)
        ctx.set_fill_style(&JsValue::from_str("#666666"));
        ctx.set_font("11px 'Inter', sans-serif");
        if i == 0 {
            ctx.fill_text("HEAD", x, y - node_size / 2.0 - 8.0).ok();
        }
        if node.next.is_none() {
            ctx.fill_text("→ NULL", x + node_size / 2.0 + 25.0, y + 4.0).ok();
        }
    }

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_stack(canvas: &Canvas, demo: &StackDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let box_w = 100.0;
    let box_h = 40.0;
    let x = (w - box_w) / 2.0;
    let base_y = h - 50.0;

    // Draw stack frame
    ctx.set_stroke_style(&JsValue::from_str("#333333"));
    ctx.set_line_width(2.0);
    ctx.begin_path();
    ctx.move_to(x - 10.0, base_y + 10.0);
    ctx.line_to(x - 10.0, base_y - (demo.max_capacity as f64 + 1.0) * box_h);
    ctx.move_to(x + box_w + 10.0, base_y + 10.0);
    ctx.line_to(x + box_w + 10.0, base_y - (demo.max_capacity as f64 + 1.0) * box_h);
    ctx.stroke();

    // Draw base
    ctx.begin_path();
    ctx.move_to(x - 20.0, base_y + 10.0);
    ctx.line_to(x + box_w + 20.0, base_y + 10.0);
    ctx.stroke();

    // Draw elements
    for (i, &elem) in demo.elements.iter().enumerate() {
        let y = base_y - (i + 1) as f64 * box_h;
        let is_top = i == demo.elements.len() - 1;

        // Check for pop animation
        let alpha = if is_top {
            demo.popping_progress().map(|p| 1.0 - p).unwrap_or(1.0)
        } else {
            1.0
        };

        // Background
        let fill = if is_top && demo.is_peeking() {
            format!("rgba(0, 212, 170, {})", 0.4 * alpha)
        } else if is_top {
            format!("rgba(0, 212, 170, {})", 0.25 * alpha)
        } else {
            format!("rgba(0, 212, 170, {})", 0.1 * alpha)
        };

        ctx.set_fill_style(&JsValue::from_str(&fill));
        ctx.fill_rect(x, y, box_w, box_h - 4.0);

        ctx.set_stroke_style(&JsValue::from_str(&format!("rgba(0, 212, 170, {})", alpha)));
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, y, box_w, box_h - 4.0);

        // Value
        ctx.set_fill_style(&JsValue::from_str(&format!("rgba(255, 255, 255, {})", alpha)));
        ctx.set_font("bold 18px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&elem.to_string(), x + box_w / 2.0, y + box_h / 2.0 + 2.0).ok();

        // TOP label
        if is_top {
            ctx.set_fill_style(&JsValue::from_str("#00d4aa"));
            ctx.set_font("12px 'Inter', sans-serif");
            ctx.fill_text("← TOP", x + box_w + 30.0, y + box_h / 2.0 + 2.0).ok();
        }
    }

    // Pushing element
    if let Some((value, progress)) = demo.pushing_value() {
        let y = base_y - (demo.elements.len() + 1) as f64 * box_h;
        let start_y = y - 60.0;
        let current_y = start_y + (y - start_y) * progress as f64;

        ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 200, 0.3)"));
        ctx.fill_rect(x, current_y, box_w, box_h - 4.0);
        ctx.set_stroke_style(&JsValue::from_str("#00ffc8"));
        ctx.stroke_rect(x, current_y, box_w, box_h - 4.0);

        ctx.set_fill_style(&JsValue::from_str("#00ffc8"));
        ctx.set_font("bold 18px 'JetBrains Mono', monospace");
        ctx.fill_text(&value.to_string(), x + box_w / 2.0, current_y + box_h / 2.0 + 2.0).ok();
    }

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_queue(canvas: &Canvas, demo: &QueueDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let box_w = 60.0;
    let box_h = 50.0;
    let y = h / 2.0 - box_h / 2.0;
    let start_x = (w - demo.max_capacity as f64 * box_w) / 2.0;

    // Draw queue frame
    ctx.set_stroke_style(&JsValue::from_str("#333333"));
    ctx.set_line_width(2.0);

    // Front label
    ctx.set_fill_style(&JsValue::from_str("#00d4aa"));
    ctx.set_font("12px 'Inter', sans-serif");
    ctx.set_text_align("center");
    ctx.fill_text("FRONT", start_x - 30.0, y + box_h / 2.0 + 4.0).ok();

    // Back label
    ctx.fill_text("BACK", start_x + demo.max_capacity as f64 * box_w + 30.0, y + box_h / 2.0 + 4.0).ok();

    // Draw elements
    for (i, &elem) in demo.elements.iter().enumerate() {
        let x = start_x + i as f64 * box_w;
        let is_front = i == 0;
        let is_back = i == demo.elements.len() - 1;

        // Check for dequeue animation
        let alpha = if is_front {
            demo.dequeuing_progress().map(|p| 1.0 - p).unwrap_or(1.0)
        } else {
            1.0
        };

        // Background
        let fill = if is_front && demo.is_peeking() {
            format!("rgba(0, 212, 170, {})", 0.4 * alpha)
        } else if is_front || is_back {
            format!("rgba(0, 212, 170, {})", 0.25 * alpha)
        } else {
            format!("rgba(0, 212, 170, {})", 0.1 * alpha)
        };

        ctx.set_fill_style(&JsValue::from_str(&fill));
        ctx.fill_rect(x, y, box_w - 4.0, box_h);

        ctx.set_stroke_style(&JsValue::from_str(&format!("rgba(0, 212, 170, {})", alpha)));
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, y, box_w - 4.0, box_h);

        // Value
        ctx.set_fill_style(&JsValue::from_str(&format!("rgba(255, 255, 255, {})", alpha)));
        ctx.set_font("bold 18px 'JetBrains Mono', monospace");
        ctx.fill_text(&elem.to_string(), x + box_w / 2.0 - 2.0, y + box_h / 2.0 + 6.0).ok();
    }

    // Enqueuing element
    if let Some((value, progress)) = demo.enqueuing_value() {
        let target_x = start_x + demo.elements.len() as f64 * box_w;
        let start_x_anim = target_x + 80.0;
        let current_x = start_x_anim + (target_x - start_x_anim) * progress as f64;

        ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 200, 0.3)"));
        ctx.fill_rect(current_x, y, box_w - 4.0, box_h);
        ctx.set_stroke_style(&JsValue::from_str("#00ffc8"));
        ctx.stroke_rect(current_x, y, box_w - 4.0, box_h);

        ctx.set_fill_style(&JsValue::from_str("#00ffc8"));
        ctx.set_font("bold 18px 'JetBrains Mono', monospace");
        ctx.fill_text(&value.to_string(), current_x + box_w / 2.0 - 2.0, y + box_h / 2.0 + 6.0).ok();
    }

    // Draw dequeue arrow
    if demo.dequeuing_progress().is_some() {
        ctx.set_stroke_style(&JsValue::from_str("#ff6666"));
        ctx.begin_path();
        ctx.move_to(start_x - 50.0, y + box_h / 2.0);
        ctx.line_to(start_x - 70.0, y + box_h / 2.0);
        ctx.stroke();
    }

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_binary_tree(canvas: &Canvas, demo: &BinaryTreeDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_r = 22.0;

    // Draw edges
    ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
    ctx.set_line_width(2.0);

    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        if let Some(left_idx) = node.left {
            let left = &demo.nodes[left_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(left.position.x as f64, left.position.y as f64 - node_r);
            ctx.stroke();
        }

        if let Some(right_idx) = node.right {
            let right = &demo.nodes[right_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(right.position.x as f64, right.position.y as f64 - node_r);
            ctx.stroke();
        }
    }

    // Draw nodes
    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        // Background
        let fill = if node.highlight {
            "rgba(0, 212, 170, 0.5)"
        } else {
            "rgba(0, 212, 170, 0.15)"
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_r, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        let stroke = if node.highlight { "#00ffc8" } else { "#00d4aa" };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(if node.highlight { 3.0 } else { 2.0 });
        ctx.stroke();

        // Value
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 14px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&node.value.to_string(), x, y + 5.0).ok();
    }

    // Traversal result
    if !demo.traversal_path.is_empty() {
        let values = demo.get_traversal_values();
        let text: String = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" → ");
        ctx.set_fill_style(&JsValue::from_str("#888888"));
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.fill_text(&text, w / 2.0, h - 30.0).ok();
    }

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_bst(canvas: &Canvas, demo: &BstDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_r = 22.0;

    // Draw edges
    ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
    ctx.set_line_width(2.0);

    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        if let Some(left_idx) = node.left {
            let left = &demo.nodes[left_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(left.position.x as f64, left.position.y as f64 - node_r);
            ctx.stroke();
        }

        if let Some(right_idx) = node.right {
            let right = &demo.nodes[right_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(right.position.x as f64, right.position.y as f64 - node_r);
            ctx.stroke();
        }
    }

    // Draw nodes
    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        // Background based on highlight state
        let fill = match node.highlight {
            HighlightState::Found => "rgba(0, 255, 100, 0.5)",
            HighlightState::Searching => "rgba(255, 200, 0, 0.4)",
            HighlightState::Path => "rgba(0, 212, 170, 0.3)",
            HighlightState::Inserting => "rgba(0, 255, 200, 0.5)",
            HighlightState::None => "rgba(0, 212, 170, 0.15)",
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_r, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        let stroke = match node.highlight {
            HighlightState::Found => "#00ff64",
            HighlightState::Searching => "#ffc800",
            HighlightState::Inserting => "#00ffc8",
            _ => "#00d4aa",
        };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Value
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 14px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&node.value.to_string(), x, y + 5.0).ok();
    }

    // Stats
    ctx.set_fill_style(&JsValue::from_str("#666666"));
    ctx.set_font("12px 'Inter', sans-serif");
    ctx.fill_text(&format!("Height: {}  |  Comparisons: {}", demo.height(), demo.comparisons), w / 2.0, h - 30.0).ok();

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_heap(canvas: &Canvas, demo: &HeapDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_r = 22.0;

    // Draw tree representation
    // Draw edges first
    ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
    ctx.set_line_width(2.0);

    for (i, _) in demo.elements.iter().enumerate() {
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < demo.elements.len() {
            if i < demo.positions.len() && left < demo.positions.len() {
                let from = demo.positions[i];
                let to = demo.positions[left];
                ctx.begin_path();
                ctx.move_to(from.x as f64, from.y as f64 + node_r);
                ctx.line_to(to.x as f64, to.y as f64 - node_r);
                ctx.stroke();
            }
        }

        if right < demo.elements.len() {
            if i < demo.positions.len() && right < demo.positions.len() {
                let from = demo.positions[i];
                let to = demo.positions[right];
                ctx.begin_path();
                ctx.move_to(from.x as f64, from.y as f64 + node_r);
                ctx.line_to(to.x as f64, to.y as f64 - node_r);
                ctx.stroke();
            }
        }
    }

    // Draw nodes
    for (i, &elem) in demo.elements.iter().enumerate() {
        if i >= demo.positions.len() { break; }

        let pos = demo.positions[i];
        let x = pos.x as f64;
        let y = pos.y as f64;

        // Background
        let fill = if demo.highlight.contains(&i) {
            "rgba(0, 212, 170, 0.5)"
        } else if i == 0 {
            "rgba(0, 212, 170, 0.25)"
        } else {
            "rgba(0, 212, 170, 0.15)"
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_r, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        let stroke = if demo.highlight.contains(&i) { "#00ffc8" } else { "#00d4aa" };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Value
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 14px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&elem.to_string(), x, y + 5.0).ok();

        // Index
        ctx.set_fill_style(&JsValue::from_str("#666666"));
        ctx.set_font("10px 'JetBrains Mono', monospace");
        ctx.fill_text(&format!("[{}]", i), x, y + node_r + 14.0).ok();
    }

    // Heap type indicator
    let type_text = match demo.heap_type {
        HeapType::MaxHeap => "Max-Heap",
        HeapType::MinHeap => "Min-Heap",
    };
    ctx.set_fill_style(&JsValue::from_str("#00d4aa"));
    ctx.set_font("14px 'Inter', sans-serif");
    ctx.fill_text(type_text, w / 2.0, 30.0).ok();

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_hash_table(canvas: &Canvas, demo: &HashTableDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let bucket_w = 80.0;
    let bucket_h = 35.0;
    let entry_w = 100.0;
    let x = 100.0;
    let start_y = 50.0;

    // Draw buckets
    for i in 0..demo.num_buckets {
        let y = start_y + i as f64 * (bucket_h + 8.0);

        // Highlight
        let fill = if demo.highlight_bucket == Some(i) {
            "rgba(0, 212, 170, 0.3)"
        } else {
            "rgba(0, 212, 170, 0.1)"
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.fill_rect(x, y, bucket_w, bucket_h);

        let stroke = if demo.highlight_bucket == Some(i) { "#00ffc8" } else { "#00d4aa" };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, y, bucket_w, bucket_h);

        // Index
        ctx.set_fill_style(&JsValue::from_str("#00d4aa"));
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&format!("[{}]", i), x + bucket_w / 2.0, y + bucket_h / 2.0 + 5.0).ok();

        // Draw chain
        let bucket = demo.get_bucket(i);
        for (j, entry) in bucket.iter().enumerate() {
            let entry_x = x + bucket_w + 20.0 + j as f64 * (entry_w + 30.0);

            // Arrow
            ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
            ctx.begin_path();
            ctx.move_to(entry_x - 20.0, y + bucket_h / 2.0);
            ctx.line_to(entry_x - 5.0, y + bucket_h / 2.0);
            ctx.stroke();

            // Entry box
            let entry_fill = if demo.highlight_bucket == Some(i) && demo.highlight_chain == Some(j) {
                "rgba(0, 255, 200, 0.3)"
            } else {
                "rgba(0, 212, 170, 0.15)"
            };

            ctx.set_fill_style(&JsValue::from_str(entry_fill));
            ctx.fill_rect(entry_x, y, entry_w, bucket_h);
            ctx.stroke_rect(entry_x, y, entry_w, bucket_h);

            // Entry text
            ctx.set_fill_style(&JsValue::from_str("#ffffff"));
            ctx.set_font("12px 'JetBrains Mono', monospace");
            ctx.set_text_align("center");
            let text = format!("{}:{}", entry.key, entry.value);
            ctx.fill_text(&text, entry_x + entry_w / 2.0, y + bucket_h / 2.0 + 4.0).ok();
        }
    }

    // Stats
    ctx.set_fill_style(&JsValue::from_str("#666666"));
    ctx.set_font("12px 'Inter', sans-serif");
    ctx.set_text_align("center");
    ctx.fill_text(&format!("Load Factor: {:.2}  |  Size: {}", demo.load_factor(), demo.size), w / 2.0, h - 30.0).ok();

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_graph(canvas: &Canvas, demo: &GraphDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_r = 25.0;

    // Draw edges
    for edge in &demo.edges {
        let from = &demo.vertices[edge.from];
        let to = &demo.vertices[edge.to];

        let stroke = if edge.highlighted { "#00ffc8" } else { "#00d4aa" };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(if edge.highlighted { 3.0 } else { 2.0 });

        ctx.begin_path();
        ctx.move_to(from.position.x as f64, from.position.y as f64);
        ctx.line_to(to.position.x as f64, to.position.y as f64);
        ctx.stroke();
    }

    // Draw vertices
    for vertex in &demo.vertices {
        let x = vertex.position.x as f64;
        let y = vertex.position.y as f64;

        // Background based on state
        let fill = match vertex.state {
            VertexState::Current => "rgba(255, 200, 0, 0.5)",
            VertexState::Discovered => "rgba(0, 212, 170, 0.4)",
            VertexState::Visited => "rgba(0, 255, 100, 0.3)",
            VertexState::Unvisited => "rgba(0, 212, 170, 0.15)",
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_r, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        let stroke = match vertex.state {
            VertexState::Current => "#ffc800",
            VertexState::Visited => "#00ff64",
            _ => "#00d4aa",
        };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Label
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 16px 'Inter', sans-serif");
        ctx.set_text_align("center");
        ctx.fill_text(&vertex.label, x, y + 6.0).ok();
    }

    // Traversal result
    if !demo.traversal_order.is_empty() {
        let labels = demo.get_traversal_labels();
        let text = labels.join(" → ");
        ctx.set_fill_style(&JsValue::from_str("#888888"));
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.fill_text(&text, w / 2.0, h - 30.0).ok();
    }

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn render_balanced_tree(canvas: &Canvas, demo: &BalancedTreeDemo) {
    let ctx = canvas.ctx();
    let w = canvas.width();
    let h = canvas.height();

    canvas.clear("#0a0a12");

    let node_r = 22.0;

    // Draw edges
    ctx.set_stroke_style(&JsValue::from_str("#00d4aa"));
    ctx.set_line_width(2.0);

    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        if let Some(left_idx) = node.left {
            let left = &demo.nodes[left_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(left.position.x as f64, left.position.y as f64 - node_r);
            ctx.stroke();
        }

        if let Some(right_idx) = node.right {
            let right = &demo.nodes[right_idx];
            ctx.begin_path();
            ctx.move_to(x, y + node_r);
            ctx.line_to(right.position.x as f64, right.position.y as f64 - node_r);
            ctx.stroke();
        }
    }

    // Draw nodes
    for node in &demo.nodes {
        let x = node.position.x as f64;
        let y = node.position.y as f64;

        // Background based on highlight
        let fill = match node.highlight {
            HighlightType::Inserted => "rgba(0, 255, 200, 0.5)",
            HighlightType::Rotating => "rgba(255, 200, 0, 0.4)",
            HighlightType::Unbalanced => "rgba(255, 100, 100, 0.4)",
            HighlightType::Path => "rgba(0, 212, 170, 0.3)",
            HighlightType::None => "rgba(0, 212, 170, 0.15)",
        };

        ctx.set_fill_style(&JsValue::from_str(fill));
        ctx.begin_path();
        ctx.arc(x, y, node_r, 0.0, std::f64::consts::PI * 2.0).ok();
        ctx.fill();

        // Border
        let stroke = match node.highlight {
            HighlightType::Inserted => "#00ffc8",
            HighlightType::Rotating => "#ffc800",
            HighlightType::Unbalanced => "#ff6464",
            _ => "#00d4aa",
        };
        ctx.set_stroke_style(&JsValue::from_str(stroke));
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Value
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.set_font("bold 14px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        ctx.fill_text(&node.value.to_string(), x, y + 5.0).ok();

        // Height label
        ctx.set_fill_style(&JsValue::from_str("#666666"));
        ctx.set_font("10px 'JetBrains Mono', monospace");
        ctx.fill_text(&format!("h={}", node.height), x, y + node_r + 14.0).ok();
    }

    // Stats
    ctx.set_fill_style(&JsValue::from_str("#666666"));
    ctx.set_font("12px 'Inter', sans-serif");
    ctx.fill_text(
        &format!("Height: {}  |  Rotations: {}  |  Balanced: {}",
            demo.tree_height(), demo.rotation_count,
            if demo.is_balanced() { "Yes" } else { "No" }),
        w / 2.0, h - 30.0
    ).ok();

    draw_message(ctx, w, h, &demo.message);

    // Pseudocode panel
    draw_pseudocode(ctx, w, h, &demo.pseudocode);
}

fn draw_message(ctx: &web_sys::CanvasRenderingContext2d, w: f64, _h: f64, message: &str) {
    if !message.is_empty() {
        ctx.set_fill_style(&JsValue::from_str("#00d4aa"));
        ctx.set_font("14px 'Inter', sans-serif");
        ctx.set_text_align("center");
        ctx.fill_text(message, w / 2.0, 30.0).ok();
    }
}

/// Draw the pseudocode panel on the right side of the canvas
fn draw_pseudocode(ctx: &web_sys::CanvasRenderingContext2d, w: f64, _h: f64, pseudocode: &Pseudocode) {
    if pseudocode.lines.is_empty() {
        return;
    }

    // Get theme-appropriate colors
    let colors = get_current_theme_colors();

    let panel_width = 320.0;
    let panel_x = w - panel_width - 20.0;
    let panel_y = 50.0;
    let line_height = 22.0;
    let padding = 15.0;
    let panel_height = (pseudocode.lines.len() as f64 * line_height) + padding * 2.0 + 30.0;

    // Draw panel background
    ctx.set_fill_style(&JsValue::from_str(colors.panel_bg));
    ctx.fill_rect(panel_x, panel_y, panel_width, panel_height);

    // Draw panel border
    ctx.set_stroke_style(&JsValue::from_str(colors.panel_border));
    ctx.set_line_width(1.0);
    ctx.stroke_rect(panel_x, panel_y, panel_width, panel_height);

    // Draw operation title
    ctx.set_fill_style(&JsValue::from_str(colors.title_color));
    ctx.set_font("bold 13px 'JetBrains Mono', monospace");
    ctx.set_text_align("left");
    ctx.fill_text(pseudocode.operation, panel_x + padding, panel_y + 22.0).ok();

    // Draw each line of pseudocode
    ctx.set_font("12px 'JetBrains Mono', monospace");

    for (i, line) in pseudocode.lines.iter().enumerate() {
        let y = panel_y + 45.0 + (i as f64 * line_height);
        let x = panel_x + padding + (line.indent as f64 * 16.0);

        let is_highlighted = pseudocode.current_line == Some(i);

        // Draw highlight background for current line
        if is_highlighted {
            ctx.set_fill_style(&JsValue::from_str(colors.highlight_bg));
            ctx.fill_rect(panel_x + 5.0, y - 14.0, panel_width - 10.0, line_height);

            // Draw highlight indicator bar
            ctx.set_fill_style(&JsValue::from_str(colors.highlight_bar));
            ctx.fill_rect(panel_x + 5.0, y - 14.0, 3.0, line_height);
        }

        // Draw line number
        ctx.set_fill_style(&JsValue::from_str(colors.line_number));
        ctx.fill_text(&format!("{:2}", i + 1), panel_x + padding, y).ok();

        // Draw code text
        if is_highlighted {
            ctx.set_fill_style(&JsValue::from_str(colors.code_highlighted));
        } else {
            ctx.set_fill_style(&JsValue::from_str(colors.code_normal));
        }
        ctx.fill_text(line.text, x + 25.0, y).ok();
    }
}

// ===============================================================================
// WASM-EXPOSED DEMO CONTROLS
// ===============================================================================

#[wasm_bindgen]
pub fn ds_demo_action(action: &str, value: i32) {
    ACTIVE_DEMO.with(|demo| {
        if let Some(d) = demo.borrow_mut().as_mut() {
            match d {
                ActiveDemo::Array(demo) => {
                    match action {
                        "access" => demo.access(value as usize),
                        "insert" => demo.insert(value as usize, value + 10),
                        "delete" => demo.delete(value as usize),
                        _ => {}
                    }
                }
                ActiveDemo::LinkedList(demo) => {
                    match action {
                        "insert_head" => demo.insert_head(value),
                        "insert_tail" => demo.insert_tail(value),
                        "delete_head" => demo.delete_head(),
                        "search" => demo.search(value),
                        _ => {}
                    }
                }
                ActiveDemo::Stack(demo) => {
                    match action {
                        "push" => demo.push(value),
                        "pop" => demo.pop(),
                        "peek" => demo.peek(),
                        _ => {}
                    }
                }
                ActiveDemo::Queue(demo) => {
                    match action {
                        "enqueue" => demo.enqueue(value),
                        "dequeue" => demo.dequeue(),
                        "peek" => demo.peek(),
                        _ => {}
                    }
                }
                ActiveDemo::BinaryTree(demo) => {
                    match action {
                        "insert" => demo.insert(value),
                        "preorder" => demo.traverse(TraversalOrder::PreOrder),
                        "inorder" => demo.traverse(TraversalOrder::InOrder),
                        "postorder" => demo.traverse(TraversalOrder::PostOrder),
                        "levelorder" => demo.traverse(TraversalOrder::LevelOrder),
                        _ => {}
                    }
                }
                ActiveDemo::Bst(demo) => {
                    match action {
                        "insert" => demo.insert(value),
                        "search" => demo.search(value),
                        _ => {}
                    }
                }
                ActiveDemo::Heap(demo) => {
                    match action {
                        "insert" => demo.insert(value),
                        "extract" => demo.extract(),
                        "toggle_type" => {
                            let new_type = match demo.heap_type {
                                HeapType::MaxHeap => HeapType::MinHeap,
                                HeapType::MinHeap => HeapType::MaxHeap,
                            };
                            demo.set_heap_type(new_type);
                        }
                        _ => {}
                    }
                }
                ActiveDemo::HashTable(demo) => {
                    match action {
                        "insert" => demo.insert(format!("key{}", value), value),
                        "search" => demo.search(format!("key{}", value)),
                        _ => {}
                    }
                }
                ActiveDemo::Graph(demo) => {
                    match action {
                        "bfs" => demo.bfs(value as usize),
                        "dfs" => demo.dfs(value as usize),
                        _ => {}
                    }
                }
                ActiveDemo::BalancedTree(demo) => {
                    match action {
                        "insert" => demo.insert(value),
                        _ => {}
                    }
                }
            }
        }
    });
}

#[wasm_bindgen]
pub fn ds_array_insert(index: usize, value: i32) {
    ACTIVE_DEMO.with(|demo| {
        if let Some(d) = demo.borrow_mut().as_mut() {
            if let ActiveDemo::Array(demo) = d {
                demo.insert(index, value);
            }
        }
    });
}

#[wasm_bindgen]
pub fn ds_demo_reset(seed: u32) {
    ACTIVE_DEMO.with(|demo| {
        if let Some(d) = demo.borrow_mut().as_mut() {
            match d {
                ActiveDemo::Array(demo) => demo.reset(seed as u64),
                ActiveDemo::LinkedList(demo) => demo.reset(seed as u64),
                ActiveDemo::Stack(demo) => demo.reset(seed as u64),
                ActiveDemo::Queue(demo) => demo.reset(seed as u64),
                ActiveDemo::BinaryTree(demo) => demo.reset(seed as u64),
                ActiveDemo::Bst(demo) => demo.reset(seed as u64),
                ActiveDemo::Heap(demo) => demo.reset(seed as u64),
                ActiveDemo::HashTable(demo) => demo.reset(seed as u64),
                ActiveDemo::Graph(demo) => demo.reset(seed as u64),
                ActiveDemo::BalancedTree(demo) => demo.reset(seed as u64),
            }
        }
    });
}

#[wasm_bindgen]
pub fn ds_demo_set_speed(speed: f32) {
    ACTIVE_DEMO.with(|demo| {
        if let Some(d) = demo.borrow_mut().as_mut() {
            match d {
                ActiveDemo::Array(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::LinkedList(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::Stack(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::Queue(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::BinaryTree(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::Bst(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::Heap(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::HashTable(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::Graph(demo) => { demo.set_param("speed", speed); }
                ActiveDemo::BalancedTree(demo) => { demo.set_param("speed", speed); }
            }
        }
    });
}
