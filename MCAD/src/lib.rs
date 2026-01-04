//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | MCAD/src/lib.rs
//! PURPOSE: MCAD solid modeler WASM application with 3D wireframe visualization
//! MODIFIED: 2026-01-04
//! LAYER: MCAD (L1 Bubble)
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]
use std::cell::RefCell;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, HtmlInputElement,
    HtmlSelectElement, MouseEvent, WheelEvent, Blob, Url, HtmlAnchorElement,
};

use cad_engine::{
    is_manifold, make_box, make_cone, make_cylinder, make_sphere, surface_area, volume, Point3,
    Solid, solid_to_step, solid_to_stl, union, difference, intersection,
    Sketch, SketchPlane, Point2, SketchEntity, SketchEntityId, SketchPointId,
    Constraint, GeometricConstraint, DimensionalConstraint,
    ConstraintSolver, ExtrudeParams, extrude_sketch,
};

// Global state for the current solid and view
thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

/// Application mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppMode {
    View3D,
    Sketch2D,
}

/// Sketch drawing tool
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SketchTool {
    Select,
    Line,
    Arc,
    Circle,
    Point,
}

struct AppState {
    // 3D View mode
    solid: Option<Solid>,
    solid_a: Option<Solid>,  // For Boolean operations
    solid_b: Option<Solid>,  // For Boolean operations
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,

    // Sketch mode
    mode: AppMode,
    current_sketch: Option<Sketch>,
    sketch_tool: SketchTool,
    temp_points: Vec<Point2>,  // For multi-click tools
    sketch_constraints: Vec<Constraint>,  // Track constraints separately
    selected_entities: Vec<SketchEntityId>,
    pan_2d: Point2,
    zoom_2d: f32,

    // Interaction
    dragging: bool,
    last_mouse_x: i32,
    last_mouse_y: i32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            solid: None,
            solid_a: None,
            solid_b: None,
            rotation_x: 0.5,
            rotation_y: 0.75,
            zoom: 2.0,

            mode: AppMode::View3D,
            current_sketch: None,
            sketch_tool: SketchTool::Line,
            temp_points: Vec::new(),
            sketch_constraints: Vec::new(),
            selected_entities: Vec::new(),
            pan_2d: Point2::new(0.0, 0.0),
            zoom_2d: 1.0,

            dragging: false,
            last_mouse_x: 0,
            last_mouse_y: 0,
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"CAD Modeler initialized".into());

    if let Err(e) = init_ui() {
        web_sys::console::error_1(&format!("Failed to initialize UI: {:?}", e).into());
    }

    Ok(())
}

fn init_ui() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    // Set up create button
    if let Some(btn) = document.get_element_by_id("create-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = create_solid() {
                web_sys::console::error_1(&format!("Create failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up export STEP button
    if let Some(btn) = document.get_element_by_id("export-step-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = export_step() {
                web_sys::console::error_1(&format!("Export failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up export STL button
    if let Some(btn) = document.get_element_by_id("export-stl-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = export_stl() {
                web_sys::console::error_1(&format!("Export failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up Boolean operation buttons
    if let Some(btn) = document.get_element_by_id("boolean-union-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = perform_boolean("union") {
                web_sys::console::error_1(&format!("Boolean union failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    if let Some(btn) = document.get_element_by_id("boolean-diff-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = perform_boolean("difference") {
                web_sys::console::error_1(&format!("Boolean difference failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    if let Some(btn) = document.get_element_by_id("boolean-intersect-btn") {
        let btn: HtmlElement = btn.dyn_into()?;
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = perform_boolean("intersection") {
                web_sys::console::error_1(&format!("Boolean intersection failed: {:?}", e).into());
            }
        }) as Box<dyn FnMut()>);
        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up viewport mouse events
    setup_viewport_events(&document)?;

    // Export view function to JS
    let set_view_closure = Closure::wrap(Box::new(|view: String| {
        set_view(&view);
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setViewAngle"),
        set_view_closure.as_ref(),
    )?;
    set_view_closure.forget();

    // Export mode toggle to JS
    let toggle_mode_closure = Closure::wrap(Box::new(|| {
        toggle_mode();
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("toggleMode"),
        toggle_mode_closure.as_ref(),
    )?;
    toggle_mode_closure.forget();

    // Export sketch tool selection to JS
    let set_tool_closure = Closure::wrap(Box::new(|tool: String| {
        set_sketch_tool(&tool);
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setSketchTool"),
        set_tool_closure.as_ref(),
    )?;
    set_tool_closure.forget();

    // Export extrude function to JS
    let extrude_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = extrude_current_sketch() {
            web_sys::console::error_1(&format!("Extrude failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("extrudeSketch"),
        extrude_closure.as_ref(),
    )?;
    extrude_closure.forget();

    // Create initial solid
    create_solid()?;

    Ok(())
}

fn setup_viewport_events(document: &Document) -> Result<(), JsValue> {
    let canvas = document
        .get_element_by_id("viewport-canvas")
        .ok_or("Canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    // Mouse click - for sketch mode
    {
        let canvas_clone = canvas.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            STATE.with(|state| {
                let state_ref = state.borrow();
                if state_ref.mode == AppMode::Sketch2D {
                    drop(state_ref);  // Release borrow
                    // Get canvas coordinates
                    let rect = canvas_clone.get_bounding_client_rect();
                    let x = event.client_x() as f64 - rect.left();
                    let y = event.client_y() as f64 - rect.top();
                    let _ = handle_sketch_click(x, y);
                }
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse down - for 3D view dragging
    {
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if state.mode == AppMode::View3D {
                    state.dragging = true;
                }
                state.last_mouse_x = event.client_x();
                state.last_mouse_y = event.client_y();
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmousedown(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse up
    {
        let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            STATE.with(|state| {
                state.borrow_mut().dragging = false;
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmouseup(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse leave
    {
        let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            STATE.with(|state| {
                state.borrow_mut().dragging = false;
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmouseleave(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse move
    {
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                if state.dragging {
                    let dx = event.client_x() - state.last_mouse_x;
                    let dy = event.client_y() - state.last_mouse_y;
                    state.rotation_y += dx as f32 * 0.01;
                    state.rotation_x += dy as f32 * 0.01;
                    state.last_mouse_x = event.client_x();
                    state.last_mouse_y = event.client_y();
                    drop(state);
                    let _ = render();
                }
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmousemove(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse wheel for zoom
    {
        let closure = Closure::wrap(Box::new(move |event: WheelEvent| {
            event.prevent_default();
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                let delta = event.delta_y() as f32 * 0.001;
                state.zoom = (state.zoom - delta).clamp(0.5, 10.0);
                drop(state);
                let _ = render();
            });
        }) as Box<dyn FnMut(WheelEvent)>);
        canvas.set_onwheel(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}

fn set_view(view: &str) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match view {
            "front" => {
                state.rotation_x = 0.0;
                state.rotation_y = 0.0;
            }
            "top" => {
                state.rotation_x = PI / 2.0;
                state.rotation_y = 0.0;
            }
            "right" => {
                state.rotation_x = 0.0;
                state.rotation_y = PI / 2.0;
            }
            _ => {
                // "iso" or any other view defaults to isometric
                state.rotation_x = 0.5;
                state.rotation_y = 0.75;
            }
        }
    });
    let _ = render();
}

fn create_solid() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    // Get primitive type
    let prim_select = document
        .get_element_by_id("primitive-type")
        .ok_or("Select not found")?;
    let prim_select: HtmlSelectElement = prim_select.dyn_into()?;
    let prim_type = prim_select.value();

    let solid = match prim_type.as_str() {
        "box" => {
            let width = get_input_value(&document, "width")? as f32;
            let depth = get_input_value(&document, "depth")? as f32;
            let height = get_input_value(&document, "height")? as f32;
            make_box(width, depth, height)
        }
        "cylinder" => {
            let radius = get_input_value(&document, "cyl-radius")? as f32;
            let height = get_input_value(&document, "cyl-height")? as f32;
            let segments = get_input_value(&document, "cyl-segments")? as u32;
            make_cylinder(radius, height, segments)
        }
        "sphere" => {
            let radius = get_input_value(&document, "sph-radius")? as f32;
            let u_seg = get_input_value(&document, "sph-u-seg")? as u32;
            let v_seg = get_input_value(&document, "sph-v-seg")? as u32;
            make_sphere(radius, u_seg, v_seg)
        }
        "cone" => {
            let radius = get_input_value(&document, "cone-radius")? as f32;
            let height = get_input_value(&document, "cone-height")? as f32;
            let segments = get_input_value(&document, "cone-segments")? as u32;
            make_cone(radius, height, segments)
        }
        _ => make_box(100.0, 50.0, 25.0),
    };

    // Update state - store as solid_a if empty, solid_b if solid_a exists, otherwise main solid
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.solid_a.is_none() {
            state.solid_a = Some(solid.clone());
            web_sys::console::log_1(&"Stored as Solid A".into());
        } else if state.solid_b.is_none() {
            state.solid_b = Some(solid.clone());
            web_sys::console::log_1(&"Stored as Solid B - ready for Boolean operations".into());
        }
        state.solid = Some(solid);
    });

    // Update display
    display_properties(&document)?;
    render()?;

    Ok(())
}

fn display_properties(document: &Document) -> Result<(), JsValue> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(ref solid) = state.solid {
            set_text(
                document,
                "result-vertices",
                &solid.vertices.len().to_string(),
            )
            .ok();
            set_text(document, "result-edges", &solid.edges.len().to_string()).ok();
            set_text(document, "result-faces", &solid.faces.len().to_string()).ok();

            let vol = volume(solid);
            let vol_text = if vol >= 1e6 {
                format!("{:.2} cm³", vol / 1e3)
            } else {
                format!("{:.1} mm³", vol)
            };
            set_text(document, "result-volume", &vol_text).ok();

            let area = surface_area(solid);
            let area_text = if area >= 1e4 {
                format!("{:.2} cm²", area / 100.0)
            } else {
                format!("{:.1} mm²", area)
            };
            set_text(document, "result-area", &area_text).ok();

            let manifold = if is_manifold(solid) { "Yes" } else { "No" };
            set_text(document, "result-manifold", manifold).ok();
        }
    });
    Ok(())
}

fn render() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let canvas = document
        .get_element_by_id("viewport-canvas")
        .ok_or("Canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    let dpr = window.device_pixel_ratio();

    let canvas_element: Element = canvas.clone().into();
    let rect = canvas_element.get_bounding_client_rect();
    let css_width = rect.width();
    let css_height = rect.height();

    let target_width = (css_width * dpr) as u32;
    let target_height = (css_height * dpr) as u32;

    if (canvas.width() as i32 - target_width as i32).abs() > 2
        || (canvas.height() as i32 - target_height as i32).abs() > 2
    {
        canvas.set_width(target_width);
        canvas.set_height(target_height);
    }

    let ctx = canvas
        .get_context("2d")?
        .ok_or("No 2D context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
    ctx.scale(dpr, dpr)?;

    // Clear
    ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
    ctx.fill_rect(0.0, 0.0, css_width, css_height);

    STATE.with(|state| {
        let state = state.borrow();
        match state.mode {
            AppMode::View3D => {
                if let Some(ref solid) = state.solid {
                    draw_wireframe(
                        &ctx,
                        solid,
                        css_width,
                        css_height,
                        state.rotation_x,
                        state.rotation_y,
                        state.zoom,
                    )
                    .ok();
                }
            }
            AppMode::Sketch2D => {
                if let Some(ref sketch) = state.current_sketch {
                    draw_sketch_2d(
                        &ctx,
                        sketch,
                        &state.temp_points,
                        state.sketch_tool,
                        css_width,
                        css_height,
                        state.pan_2d,
                        state.zoom_2d,
                    )
                    .ok();
                }
            }
        }
    });

    Ok(())
}

fn draw_wireframe(
    ctx: &CanvasRenderingContext2d,
    solid: &Solid,
    width: f64,
    height: f64,
    rot_x: f32,
    rot_y: f32,
    zoom: f32,
) -> Result<(), JsValue> {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let scale = (zoom as f64) * (height.min(width) / 300.0);

    // Calculate sin/cos for rotation
    let (sin_x, cos_x) = rot_x.sin_cos();
    let (sin_y, cos_y) = rot_y.sin_cos();

    // Project 3D point to 2D
    let project = |p: &Point3| -> (f64, f64) {
        // Center the model
        let x = p.x;
        let y = p.y;
        let z = p.z;

        // Rotate around Y axis
        let x1 = x * cos_y - z * sin_y;
        let z1 = x * sin_y + z * cos_y;

        // Rotate around X axis
        let y1 = y * cos_x - z1 * sin_x;
        let _z2 = y * sin_x + z1 * cos_x;

        // Simple orthographic projection
        let px = cx + (x1 as f64) * scale;
        let py = cy - (y1 as f64) * scale;

        (px, py)
    };

    // Draw edges
    ctx.set_stroke_style(&JsValue::from_str("#ff6b35"));
    ctx.set_line_width(1.5);

    for edge in &solid.edges {
        if let (Some(start_vertex), Some(end_vertex)) =
            (solid.vertex(edge.start), solid.vertex(edge.end))
        {
            let (x1, y1) = project(&start_vertex.point);
            let (x2, y2) = project(&end_vertex.point);

            ctx.begin_path();
            ctx.move_to(x1, y1);
            ctx.line_to(x2, y2);
            ctx.stroke();
        }
    }

    // Draw vertices
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    for vertex in &solid.vertices {
        let (px, py) = project(&vertex.point);
        ctx.begin_path();
        ctx.arc(px, py, 2.0, 0.0, 2.0 * std::f64::consts::PI)?;
        ctx.fill();
    }

    // Draw axis indicator
    draw_axis_indicator(ctx, width, height, sin_x, cos_x, sin_y, cos_y)?;

    Ok(())
}

fn draw_sketch_2d(
    ctx: &CanvasRenderingContext2d,
    sketch: &Sketch,
    temp_points: &[Point2],
    tool: SketchTool,
    width: f64,
    height: f64,
    pan: Point2,
    zoom: f32,
) -> Result<(), JsValue> {
    let cx = width / 2.0;
    let cy = height / 2.0;

    // Transform sketch coordinates to screen
    let to_screen = |p: Point2| -> (f64, f64) {
        let sx = cx + (p.x as f64 + pan.x as f64) * zoom as f64;
        let sy = cy - (p.y as f64 + pan.y as f64) * zoom as f64;  // Y inverted
        (sx, sy)
    };

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(255, 107, 53, 0.1)"));
    ctx.set_line_width(0.5);
    let grid_spacing = 50.0 * zoom as f64;
    let start_x = (cx % grid_spacing - grid_spacing);
    let start_y = (cy % grid_spacing - grid_spacing);

    // Vertical grid lines
    let mut x = start_x;
    while x < width {
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, height);
        ctx.stroke();
        x += grid_spacing;
    }

    // Horizontal grid lines
    let mut y = start_y;
    while y < height {
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
        ctx.stroke();
        y += grid_spacing;
    }

    // Draw origin axes (thicker)
    ctx.set_stroke_style(&JsValue::from_str("rgba(255, 107, 53, 0.3)"));
    ctx.set_line_width(1.5);
    ctx.begin_path();
    ctx.move_to(cx, 0.0);
    ctx.line_to(cx, height);
    ctx.stroke();
    ctx.begin_path();
    ctx.move_to(0.0, cy);
    ctx.line_to(width, cy);
    ctx.stroke();

    // Draw sketch entities
    ctx.set_stroke_style(&JsValue::from_str("#ff6b35"));
    ctx.set_line_width(2.0);

    for entity in &sketch.entities {
        match entity {
            SketchEntity::Line { start, end, .. } => {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    let (x1, y1) = to_screen(p1.position);
                    let (x2, y2) = to_screen(p2.position);
                    ctx.begin_path();
                    ctx.move_to(x1, y1);
                    ctx.line_to(x2, y2);
                    ctx.stroke();
                }
            }
            SketchEntity::Circle { center, radius, .. } => {
                if let Some(c) = sketch.point(*center) {
                    let (cx, cy) = to_screen(c.position);
                    let r = *radius as f64 * zoom as f64;
                    ctx.begin_path();
                    ctx.arc(cx, cy, r, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.stroke();
                }
            }
            _ => {}
        }
    }

    // Draw points
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    for point in &sketch.points {
        let (px, py) = to_screen(point.position);
        ctx.begin_path();
        ctx.arc(px, py, 4.0, 0.0, 2.0 * std::f64::consts::PI)?;
        ctx.fill();
    }

    // Draw temp preview (rubber-banding)
    if !temp_points.is_empty() {
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 107, 53, 0.5)"));
        ctx.set_line_width(1.0);
        ctx.set_line_dash(&js_sys::Array::of2(&JsValue::from_f64(5.0), &JsValue::from_f64(5.0)))?;

        match tool {
            SketchTool::Line if temp_points.len() == 1 => {
                // Show preview line from first point to cursor (would need mouse pos)
                let (x1, y1) = to_screen(temp_points[0]);
                ctx.begin_path();
                ctx.arc(x1, y1, 4.0, 0.0, 2.0 * std::f64::consts::PI)?;
                ctx.fill();
            }
            _ => {}
        }
        ctx.set_line_dash(&js_sys::Array::new())?;  // Reset dash
    }

    Ok(())
}

fn draw_axis_indicator(
    ctx: &CanvasRenderingContext2d,
    _width: f64,
    height: f64,
    sin_x: f32,
    cos_x: f32,
    sin_y: f32,
    cos_y: f32,
) -> Result<(), JsValue> {
    let origin_x = 50.0;
    let origin_y = height - 50.0;
    let axis_len = 30.0;

    // Project axis vectors
    let project_axis = |ax: f32, ay: f32, az: f32| -> (f64, f64) {
        let x1 = ax * cos_y - az * sin_y;
        let y1 = ay * cos_x - (ax * sin_y + az * cos_y) * sin_x;
        (x1 as f64 * axis_len, -y1 as f64 * axis_len)
    };

    // X axis (red)
    let (dx, dy) = project_axis(1.0, 0.0, 0.0);
    ctx.set_stroke_style(&JsValue::from_str("#ff4444"));
    ctx.set_line_width(2.0);
    ctx.begin_path();
    ctx.move_to(origin_x, origin_y);
    ctx.line_to(origin_x + dx, origin_y + dy);
    ctx.stroke();
    ctx.set_fill_style(&JsValue::from_str("#ff4444"));
    ctx.set_font("10px Monaco");
    ctx.fill_text("X", origin_x + dx + 5.0, origin_y + dy)?;

    // Y axis (green)
    let (dx, dy) = project_axis(0.0, 1.0, 0.0);
    ctx.set_stroke_style(&JsValue::from_str("#44ff44"));
    ctx.begin_path();
    ctx.move_to(origin_x, origin_y);
    ctx.line_to(origin_x + dx, origin_y + dy);
    ctx.stroke();
    ctx.set_fill_style(&JsValue::from_str("#44ff44"));
    ctx.fill_text("Y", origin_x + dx + 5.0, origin_y + dy)?;

    // Z axis (blue)
    let (dx, dy) = project_axis(0.0, 0.0, 1.0);
    ctx.set_stroke_style(&JsValue::from_str("#4444ff"));
    ctx.begin_path();
    ctx.move_to(origin_x, origin_y);
    ctx.line_to(origin_x + dx, origin_y + dy);
    ctx.stroke();
    ctx.set_fill_style(&JsValue::from_str("#4444ff"));
    ctx.fill_text("Z", origin_x + dx + 5.0, origin_y + dy)?;

    Ok(())
}

fn get_input_value(document: &Document, id: &str) -> Result<f64, JsValue> {
    let input = document
        .get_element_by_id(id)
        .ok_or(format!("Input {} not found", id))?;
    let input: HtmlInputElement = input.dyn_into()?;
    input
        .value()
        .parse::<f64>()
        .map_err(|_| JsValue::from_str("Invalid number"))
}

fn set_text(document: &Document, id: &str, text: &str) -> Result<(), JsValue> {
    if let Some(elem) = document.get_element_by_id(id) {
        let elem: HtmlElement = elem.dyn_into()?;
        elem.set_inner_text(text);
    }
    Ok(())
}

fn download_file(filename: &str, content: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No body")?;

    let parts = js_sys::Array::of1(&JsValue::from_str(content));
    let properties = web_sys::BlobPropertyBag::new();
    properties.set_type("text/plain");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &properties)?;

    let url = Url::create_object_url_with_blob(&blob)?;
    let a = document.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;
    a.set_href(&url);
    a.set_download(filename);
    a.style().set_property("display", "none")?;

    body.append_child(&a)?;
    a.click();
    body.remove_child(&a)?;
    Url::revoke_object_url(&url)?;

    Ok(())
}

fn export_step() -> Result<(), JsValue> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(ref solid) = state.solid {
            web_sys::console::log_1(&"Exporting STEP file...".into());
            let step_content = solid_to_step(solid, "solid");
            download_file("model.step", &step_content)?;
            web_sys::console::log_1(&"STEP export complete".into());
        } else {
            web_sys::console::warn_1(&"No solid to export - create one first".into());
        }
        Ok(())
    })
}

fn download_binary_file(filename: &str, content: &[u8]) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No body")?;

    let array = js_sys::Uint8Array::from(content);
    let parts = js_sys::Array::of1(&array);
    let properties = web_sys::BlobPropertyBag::new();
    properties.set_type("application/octet-stream");
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &properties)?;

    let url = Url::create_object_url_with_blob(&blob)?;
    let a = document.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;
    a.set_href(&url);
    a.set_download(filename);
    a.style().set_property("display", "none")?;

    body.append_child(&a)?;
    a.click();
    body.remove_child(&a)?;
    Url::revoke_object_url(&url)?;

    Ok(())
}

fn export_stl() -> Result<(), JsValue> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(ref solid) = state.solid {
            web_sys::console::log_1(&"Exporting STL file...".into());
            let stl_binary = solid_to_stl(solid, "solid");
            download_binary_file("model.stl", &stl_binary)?;
            web_sys::console::log_1(&"STL export complete".into());
        } else {
            web_sys::console::warn_1(&"No solid to export - create one first".into());
        }
        Ok(())
    })
}

fn perform_boolean(operation: &str) -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        let solid_a = state.solid_a.as_ref()
            .ok_or_else(|| JsValue::from_str("No Solid A - create first solid"))?;
        let solid_b = state.solid_b.as_ref()
            .ok_or_else(|| JsValue::from_str("No Solid B - create second solid"))?;

        web_sys::console::log_1(&format!("Performing Boolean {} operation...", operation).into());

        let result = match operation {
            "union" => union(solid_a, solid_b),
            "difference" => difference(solid_a, solid_b),
            "intersection" => intersection(solid_a, solid_b),
            _ => return Err(JsValue::from_str("Unknown operation")),
        };

        match result {
            Ok(new_solid) => {
                state.solid = Some(new_solid);
                state.solid_a = None;  // Reset for next operation
                state.solid_b = None;
                drop(state);

                web_sys::console::log_1(&format!("Boolean {} complete", operation).into());

                let window = web_sys::window().ok_or("No window")?;
                let document = window.document().ok_or("No document")?;
                display_properties(&document)?;
                render()?;
                Ok(())
            }
            Err(e) => {
                Err(JsValue::from_str(&format!("Boolean operation failed: {:?}", e)))
            }
        }
    })
}

fn toggle_mode() {
    let new_mode = STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mode = match s.mode {
            AppMode::View3D => {
                web_sys::console::log_1(&"Entering Sketch mode".into());
                s.current_sketch = Some(Sketch::new(SketchPlane::XY));
                s.temp_points.clear();
                AppMode::Sketch2D
            }
            AppMode::Sketch2D => {
                web_sys::console::log_1(&"Entering 3D View mode".into());
                AppMode::View3D
            }
        };
        s.mode
    });

    // Update UI panels
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Toggle sketch tools visibility
            if let Some(sketch_tools) = document.get_element_by_id("sketch-tools") {
                let sketch_tools: HtmlElement = sketch_tools.dyn_into().ok().unwrap();
                sketch_tools.style().set_property("display",
                    if new_mode == AppMode::Sketch2D { "block" } else { "none" }
                ).ok();
            }

            // Toggle primitive tools visibility
            if let Some(primitive_tools) = document.get_element_by_id("primitive-tools") {
                let primitive_tools: HtmlElement = primitive_tools.dyn_into().ok().unwrap();
                primitive_tools.style().set_property("display",
                    if new_mode == AppMode::View3D { "block" } else { "none" }
                ).ok();
            }
        }
    }

    let _ = render();
}

fn set_sketch_tool(tool: &str) {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.sketch_tool = match tool {
            "line" => SketchTool::Line,
            "arc" => SketchTool::Arc,
            "circle" => SketchTool::Circle,
            "point" => SketchTool::Point,
            "select" => SketchTool::Select,
            _ => SketchTool::Select,
        };
        s.temp_points.clear();  // Reset tool state
        web_sys::console::log_1(&format!("Tool changed to: {}", tool).into());
    });
}

fn extrude_current_sketch() -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        let sketch = s.current_sketch.as_ref()
            .ok_or_else(|| JsValue::from_str("No sketch to extrude"))?;

        web_sys::console::log_1(&"Extruding sketch...".into());

        let params = ExtrudeParams {
            distance: 50.0,  // TODO: get from UI input
            symmetric: false,
        };

        match extrude_sketch(sketch, &params) {
            Ok(solid) => {
                s.solid = Some(solid);
                s.mode = AppMode::View3D;  // Switch back to 3D view
                drop(s);

                web_sys::console::log_1(&"Extrude complete - switched to 3D view".into());

                let window = web_sys::window().ok_or("No window")?;
                let document = window.document().ok_or("No document")?;
                display_properties(&document)?;
                render()?;
                Ok(())
            }
            Err(e) => {
                Err(JsValue::from_str(&format!("Extrude failed: {:?}", e)))
            }
        }
    })
}

fn handle_sketch_click(screen_x: f64, screen_y: f64) -> Result<(), JsValue> {
    // Get actual canvas dimensions
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas = document.get_element_by_id("viewport-canvas").ok_or("Canvas not found")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;
    let canvas_element: Element = canvas.into();
    let rect = canvas_element.get_bounding_client_rect();
    let css_width = rect.width();
    let css_height = rect.height();

    // Convert screen coordinates to sketch coordinates
    let (sketch_x, sketch_y) = STATE.with(|state| {
        let s = state.borrow();
        let zoom = s.zoom_2d;
        let pan = s.pan_2d;

        let cx = css_width / 2.0;
        let cy = css_height / 2.0;

        let sx = ((screen_x - cx) / zoom as f64 - pan.x as f64) as f32;
        let sy = (-(screen_y - cy) / zoom as f64 - pan.y as f64) as f32;
        (sx, sy)
    });

    let pos = Point2::new(sketch_x, sketch_y);
    web_sys::console::log_1(&format!("Click at screen ({}, {}) → sketch ({}, {})", screen_x, screen_y, sketch_x, sketch_y).into());

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let tool = s.sketch_tool;
        let has_temp_point = !s.temp_points.is_empty();
        let first_temp = if has_temp_point { Some(s.temp_points[0]) } else { None };

        if let Some(sketch) = s.current_sketch.as_mut() {
            match tool {
                SketchTool::Line => {
                    if let Some(p1) = first_temp {
                        // Second click - create line
                        let p1_id = sketch.add_point(p1);
                        let p2_id = sketch.add_point(pos);
                        let line_id = SketchEntityId(sketch.entities.len() as u32);

                        sketch.add_entity(SketchEntity::Line {
                            id: line_id,
                            start: p1_id,
                            end: p2_id,
                        });

                        s.temp_points.clear();
                        web_sys::console::log_1(&"Line created".into());
                    } else {
                        // First click - store point
                        s.temp_points.push(pos);
                        web_sys::console::log_1(&"Line: first point placed".into());
                    }
                }
                SketchTool::Circle => {
                    if let Some(center) = first_temp {
                        // Second click - create circle
                        let center_id = sketch.add_point(center);
                        let radius = center.distance(&pos);
                        let circle_id = SketchEntityId(sketch.entities.len() as u32);

                        sketch.add_entity(SketchEntity::Circle {
                            id: circle_id,
                            center: center_id,
                            radius,
                        });

                        s.temp_points.clear();
                        web_sys::console::log_1(&format!("Circle created with radius {}", radius).into());
                    } else {
                        // First click - center
                        s.temp_points.push(pos);
                        web_sys::console::log_1(&"Circle: center placed".into());
                    }
                }
                SketchTool::Point => {
                    sketch.add_point(pos);
                    web_sys::console::log_1(&format!("Point created at ({}, {})", sketch_x, sketch_y).into());
                }
                _ => {}
            }
        }

        drop(s);
        render()?;
        Ok(())
    })
}
