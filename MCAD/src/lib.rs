//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | MCAD/src/lib.rs
//! PURPOSE: MCAD solid modeler WASM application with 3D wireframe visualization
//! MODIFIED: 2026-01-04
//! LAYER: MCAD (L1 Bubble)
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]

pub mod renderer;
pub mod selection3d;

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
    Sketch, SketchPlane, SketchCoordinateFrame, Point2, SketchEntity, SketchEntityId, SketchPointId,
    Constraint, GeometricConstraint, DimensionalConstraint,
    ConstraintSolver, ExtrudeParams, extrude_sketch, ConstraintAnalysis, DofStatus,
    circumcenter,
    RevolveParams, RevolveAxis, revolve_sketch,
    linear_pattern, circular_pattern, Vector3,
    FaceId,
};

use crate::renderer::RenderMode;
use crate::selection3d::{Selection3D, SelectionMode3D};

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
    Rectangle,
    Circle,
    Point,
}

/// Status message type for visual feedback
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StatusType {
    Info,
    Warning,
    Error,
    Success,
}

/// Snap type for enhanced snapping
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapType {
    None,
    Point,       // Existing endpoint
    Midpoint,    // Midpoint of line/arc
    Center,      // Center of circle/arc
    Intersection, // Intersection of two entities
    Perpendicular, // Perpendicular to line
    Grid,        // Grid snap
}

/// Result of snap position calculation
#[derive(Clone, Debug)]
pub struct SnapResult {
    pub position: Point2,
    pub snap_type: SnapType,
    pub source_entity: Option<SketchEntityId>,
}

impl SnapResult {
    fn none(pos: Point2) -> Self {
        Self {
            position: pos,
            snap_type: SnapType::None,
            source_entity: None,
        }
    }

    fn grid(pos: Point2, grid_size: f32) -> Self {
        Self {
            position: Point2::new(
                (pos.x / grid_size).round() * grid_size,
                (pos.y / grid_size).round() * grid_size,
            ),
            snap_type: SnapType::Grid,
            source_entity: None,
        }
    }
}

/// Commands for undo/redo system
#[derive(Clone, Debug)]
pub enum SketchCommand {
    /// Added geometry (points and entities)
    AddGeometry {
        point_ids: Vec<SketchPointId>,
        entity_ids: Vec<SketchEntityId>,
    },
    /// Added a constraint
    AddConstraint {
        index: usize,
    },
    /// Toggled construction mode on points
    ToggleConstruction {
        point_ids: Vec<SketchPointId>,
        prev_states: Vec<bool>,
    },
}

/// Command history for undo/redo
#[derive(Default)]
pub struct CommandHistory {
    undo_stack: Vec<SketchCommand>,
    redo_stack: Vec<SketchCommand>,
    max_size: usize,
}

impl CommandHistory {
    fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 100,
        }
    }

    fn push(&mut self, cmd: SketchCommand) {
        self.undo_stack.push(cmd);
        self.redo_stack.clear(); // Clear redo stack on new action
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    fn pop_undo(&mut self) -> Option<SketchCommand> {
        self.undo_stack.pop()
    }

    fn push_redo(&mut self, cmd: SketchCommand) {
        self.redo_stack.push(cmd);
    }

    fn pop_redo(&mut self) -> Option<SketchCommand> {
        self.redo_stack.pop()
    }

    fn push_undo(&mut self, cmd: SketchCommand) {
        self.undo_stack.push(cmd);
    }
}

fn next_entity_id(sketch: &Sketch) -> SketchEntityId {
    let next = sketch
        .entities
        .iter()
        .map(|e| e.id().0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    SketchEntityId(next)
}

fn normalize_angle(mut a: f32) -> f32 {
    let two_pi = 2.0 * PI;
    a = a % two_pi;
    if a < 0.0 {
        a += two_pi;
    }
    a
}

fn angle_in_ccw_sweep(start: f32, end: f32, mid: f32) -> bool {
    // All normalized to [0, 2pi)
    if start <= end {
        mid >= start && mid <= end
    } else {
        mid >= start || mid <= end
    }
}

/// Add a point to the sketch, optionally marking as construction
fn add_point_with_construction(sketch: &mut Sketch, pos: Point2, is_construction: bool) -> SketchPointId {
    let id = sketch.add_point(pos);
    if is_construction {
        if let Some(point) = sketch.point_mut(id) {
            point.is_construction = true;
        }
    }
    id
}

/// Check if an entity uses construction points
fn is_entity_construction(sketch: &Sketch, entity: &SketchEntity) -> bool {
    match entity {
        SketchEntity::Line { start, end, .. } => {
            sketch.point(*start).map(|p| p.is_construction).unwrap_or(false)
                || sketch.point(*end).map(|p| p.is_construction).unwrap_or(false)
        }
        SketchEntity::Circle { center, .. } => {
            sketch.point(*center).map(|p| p.is_construction).unwrap_or(false)
        }
        SketchEntity::Arc { center, start, end, .. } => {
            sketch.point(*center).map(|p| p.is_construction).unwrap_or(false)
                || sketch.point(*start).map(|p| p.is_construction).unwrap_or(false)
                || sketch.point(*end).map(|p| p.is_construction).unwrap_or(false)
        }
        SketchEntity::Point { point, .. } => {
            sketch.point(*point).map(|p| p.is_construction).unwrap_or(false)
        }
    }
}

struct AppState {
    // 3D View mode
    solid: Option<Solid>,
    solid_a: Option<Solid>,  // For Boolean operations
    solid_b: Option<Solid>,  // For Boolean operations
    pattern_instances: Option<Vec<Solid>>,
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
    hover_entity: Option<SketchEntityId>,  // Entity under cursor
    hover_point: Option<SketchPointId>,    // Point under cursor
    mouse_pos: Option<Point2>,             // Current mouse position in sketch coords
    pan_2d: Point2,
    zoom_2d: f32,
    construction_mode: bool,               // If true, new geometry is construction
    current_snap: Option<SnapResult>,      // Current snap result for visual indicator
    command_history: CommandHistory,       // Undo/redo history

    // Interaction
    dragging: bool,
    panning: bool,                         // Middle-mouse panning
    pan_3d: (f32, f32),                    // 3D view pan offset
    last_mouse_x: i32,
    last_mouse_y: i32,

    // Render and selection modes
    render_mode: RenderMode,
    selection3d: Selection3D,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            solid: None,
            solid_a: None,
            solid_b: None,
            pattern_instances: None,
            rotation_x: 0.5,
            rotation_y: 0.75,
            zoom: 2.0,

            mode: AppMode::View3D,
            current_sketch: None,
            sketch_tool: SketchTool::Line,
            temp_points: Vec::new(),
            sketch_constraints: Vec::new(),
            selected_entities: Vec::new(),
            hover_entity: None,
            hover_point: None,
            mouse_pos: None,
            pan_2d: Point2::new(0.0, 0.0),
            zoom_2d: 1.0,
            construction_mode: false,
            current_snap: None,
            command_history: CommandHistory::new(),

            dragging: false,
            panning: false,
            pan_3d: (0.0, 0.0),
            last_mouse_x: 0,
            last_mouse_y: 0,

            render_mode: RenderMode::default(),
            selection3d: Selection3D::default(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    if let Err(e) = init_ui() {
        web_sys::console::error_1(&format!("Failed to initialize UI: {:?}", e).into());
    }

    show_status("Ready", StatusType::Info);
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

    // Export revolve function to JS
    let revolve_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = revolve_current_sketch() {
            web_sys::console::error_1(&format!("Revolve failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("revolveSketch"),
        revolve_closure.as_ref(),
    )?;
    revolve_closure.forget();

    // Export pattern functions to JS
    let linear_pattern_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = apply_linear_pattern() {
            web_sys::console::error_1(&format!("Linear pattern failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("applyLinearPattern"),
        linear_pattern_closure.as_ref(),
    )?;
    linear_pattern_closure.forget();

    let circular_pattern_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = apply_circular_pattern() {
            web_sys::console::error_1(&format!("Circular pattern failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("applyCircularPattern"),
        circular_pattern_closure.as_ref(),
    )?;
    circular_pattern_closure.forget();

    let clear_pattern_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = clear_pattern() {
            web_sys::console::error_1(&format!("Clear pattern failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("clearPattern"),
        clear_pattern_closure.as_ref(),
    )?;
    clear_pattern_closure.forget();

    // Export constraint application to JS
    let constraint_closure = Closure::wrap(Box::new(|constraint_type: String| {
        if let Err(e) = apply_sketch_constraint(&constraint_type) {
            web_sys::console::error_1(&format!("Constraint failed: {:?}", e).into());
        }
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("applySketchConstraint"),
        constraint_closure.as_ref(),
    )?;
    constraint_closure.forget();

    // Export solve function to JS
    let solve_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = solve_current_sketch() {
            web_sys::console::error_1(&format!("Solve failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("solveSketchConstraints"),
        solve_closure.as_ref(),
    )?;
    solve_closure.forget();

    // Export render mode function to JS
    let render_mode_closure = Closure::wrap(Box::new(|mode: String| {
        set_render_mode(&mode);
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setRenderModeWasm"),
        render_mode_closure.as_ref(),
    )?;
    render_mode_closure.forget();

    // Export selection mode function to JS
    let selection_mode_closure = Closure::wrap(Box::new(|mode: String| {
        set_selection_mode(&mode);
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setSelectionModeWasm"),
        selection_mode_closure.as_ref(),
    )?;
    selection_mode_closure.forget();

    // Export sketch plane function to JS
    let sketch_plane_closure = Closure::wrap(Box::new(|plane: String| {
        set_sketch_plane(&plane);
    }) as Box<dyn Fn(String)>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setSketchPlaneWasm"),
        sketch_plane_closure.as_ref(),
    )?;
    sketch_plane_closure.forget();

    // Export sketch on face function to JS
    let sketch_on_face_closure = Closure::wrap(Box::new(|| {
        if let Err(e) = sketch_on_selected_face() {
            web_sys::console::error_1(&format!("Sketch on face failed: {:?}", e).into());
        }
    }) as Box<dyn Fn()>);
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("sketchOnSelectedFaceWasm"),
        sketch_on_face_closure.as_ref(),
    )?;
    sketch_on_face_closure.forget();

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

    // Mouse down - for 3D view dragging and middle-mouse panning
    {
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                match event.button() {
                    0 => {
                        // Left button - rotation in 3D view
                        if state.mode == AppMode::View3D {
                            state.dragging = true;
                        }
                    }
                    1 => {
                        // Middle button - pan
                        event.prevent_default();
                        state.panning = true;
                    }
                    _ => {}
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
                let mut s = state.borrow_mut();
                s.dragging = false;
                s.panning = false;
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmouseup(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse leave
    {
        let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            STATE.with(|state| {
                let mut s = state.borrow_mut();
                s.dragging = false;
                s.panning = false;
            });
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.set_onmouseleave(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Mouse move
    {
        let canvas_clone = canvas.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            STATE.with(|state| {
                let mut state = state.borrow_mut();

                if state.panning {
                    // Middle-mouse panning (works in both modes)
                    let dx = (event.client_x() - state.last_mouse_x) as f32;
                    let dy = (event.client_y() - state.last_mouse_y) as f32;
                    if state.mode == AppMode::View3D {
                        state.pan_3d.0 += dx;
                        state.pan_3d.1 += dy;
                    } else {
                        // In 2D, pan in sketch coords (Y inverted)
                        state.pan_2d.x += dx / state.zoom_2d;
                        state.pan_2d.y -= dy / state.zoom_2d;
                    }
                    state.last_mouse_x = event.client_x();
                    state.last_mouse_y = event.client_y();
                    drop(state);
                    let _ = render();
                } else if state.mode == AppMode::View3D && state.dragging {
                    // 3D view rotation (left-click drag)
                    let dx = event.client_x() - state.last_mouse_x;
                    let dy = event.client_y() - state.last_mouse_y;
                    state.rotation_y += dx as f32 * 0.01;
                    state.rotation_x += dy as f32 * 0.01;
                    state.last_mouse_x = event.client_x();
                    state.last_mouse_y = event.client_y();
                    drop(state);
                    let _ = render();
                } else if state.mode == AppMode::Sketch2D {
                    // Sketch mode - update hover state and mouse position
                    let rect = canvas_clone.get_bounding_client_rect();
                    let screen_x = event.client_x() as f64 - rect.left();
                    let screen_y = event.client_y() as f64 - rect.top();
                    let css_width = rect.width();
                    let css_height = rect.height();

                    // Convert to sketch coords
                    let cx = css_width / 2.0;
                    let cy = css_height / 2.0;
                    let zoom = state.zoom_2d;
                    let pan = state.pan_2d;
                    let sx = ((screen_x - cx) / zoom as f64 - pan.x as f64) as f32;
                    let sy = (-(screen_y - cy) / zoom as f64 - pan.y as f64) as f32;
                    let pos = Point2::new(sx, sy);

                    state.mouse_pos = Some(pos);

                    // Compute snap preview
                    let snap = if let Some(ref sketch) = state.current_sketch {
                        snap_position_enhanced(sketch, pos, 15.0, 50.0)
                    } else {
                        SnapResult::none(pos)
                    };
                    let snap_changed = state.current_snap.as_ref()
                        .map_or(true, |old| old.snap_type != snap.snap_type);
                    state.current_snap = Some(snap);

                    // Update hover state
                    let new_hover = if let Some(ref sketch) = state.current_sketch {
                        find_entity_at_point(sketch, pos, 10.0)
                    } else {
                        None
                    };

                    let hover_changed = state.hover_entity != new_hover;
                    state.hover_entity = new_hover;

                    // Need redraw if: hover changed OR snap changed OR we're in drawing mode
                    let is_drawing = !state.temp_points.is_empty();
                    let needs_redraw = hover_changed || snap_changed || is_drawing;

                    drop(state);
                    if needs_redraw {
                        let _ = render();
                    }
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
                if state.mode == AppMode::View3D {
                    // 3D view zoom
                    let delta = event.delta_y() as f32 * 0.001;
                    state.zoom = (state.zoom - delta).clamp(0.5, 10.0);
                } else {
                    // 2D sketch zoom
                    let delta = event.delta_y() as f32 * 0.001;
                    state.zoom_2d = (state.zoom_2d - delta).clamp(0.25, 5.0);
                }
                drop(state);
                let _ = render();
            });
        }) as Box<dyn FnMut(WheelEvent)>);
        canvas.set_onwheel(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Keyboard shortcuts - handles both 3D and Sketch modes
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            // Ignore if typing in an input field
            if let Some(target) = event.target() {
                if let Ok(elem) = target.dyn_into::<Element>() {
                    let tag = elem.tag_name().to_lowercase();
                    if tag == "input" || tag == "textarea" || tag == "select" {
                        return;
                    }
                }
            }

            let key = event.key();
            let ctrl = event.ctrl_key() || event.meta_key();
            let shift = event.shift_key();

            STATE.with(|state| {
                let mut s = state.borrow_mut();
                let mode = s.mode;

                // === UNIVERSAL SHORTCUTS ===
                // Undo/Redo with Ctrl+Z / Ctrl+Shift+Z
                if ctrl && (key == "z" || key == "Z") {
                    event.prevent_default();
                    drop(s);
                    if shift {
                        let _ = redo_action();
                    } else {
                        let _ = undo_action();
                    }
                    update_status_bar();
                    return;
                }

                match key.as_str() {
                    "Tab" => {
                        event.prevent_default();
                        drop(s);
                        toggle_mode();
                        return;
                    }
                    "Escape" => {
                        s.temp_points.clear();
                        s.selected_entities.clear();
                        drop(s);
                        show_status("Cancelled", StatusType::Info);
                        let _ = render();
                        update_status_bar();
                        return;
                    }
                    "+" | "=" => {
                        if mode == AppMode::View3D {
                            s.zoom = (s.zoom + 0.2).clamp(0.5, 10.0);
                        } else {
                            s.zoom_2d = (s.zoom_2d + 0.1).clamp(0.25, 5.0);
                        }
                        drop(s);
                        let _ = render();
                        return;
                    }
                    "-" => {
                        if mode == AppMode::View3D {
                            s.zoom = (s.zoom - 0.2).clamp(0.5, 10.0);
                        } else {
                            s.zoom_2d = (s.zoom_2d - 0.1).clamp(0.25, 5.0);
                        }
                        drop(s);
                        let _ = render();
                        return;
                    }
                    "Home" => {
                        if mode == AppMode::View3D {
                            s.rotation_x = 0.5;
                            s.rotation_y = 0.75;
                            s.zoom = 2.0;
                        } else {
                            s.pan_2d = Point2::new(0.0, 0.0);
                            s.zoom_2d = 1.0;
                        }
                        drop(s);
                        show_status("View reset", StatusType::Info);
                        let _ = render();
                        return;
                    }
                    _ => {}
                }

                // === 3D VIEW SHORTCUTS ===
                if mode == AppMode::View3D {
                    match key.as_str() {
                        "f" | "F" | "1" => {
                            s.rotation_x = 0.0;
                            s.rotation_y = 0.0;
                            drop(s);
                            show_status("Front view", StatusType::Info);
                            let _ = render();
                            return;
                        }
                        "t" | "T" | "2" => {
                            s.rotation_x = PI / 2.0;
                            s.rotation_y = 0.0;
                            drop(s);
                            show_status("Top view", StatusType::Info);
                            let _ = render();
                            return;
                        }
                        "3" => {
                            s.rotation_x = 0.0;
                            s.rotation_y = PI / 2.0;
                            drop(s);
                            show_status("Right view", StatusType::Info);
                            let _ = render();
                            return;
                        }
                        "i" | "I" | "0" => {
                            s.rotation_x = 0.5;
                            s.rotation_y = 0.75;
                            drop(s);
                            show_status("Isometric view", StatusType::Info);
                            let _ = render();
                            return;
                        }
                        _ => {}
                    }
                    return;
                }

                // === SKETCH MODE SHORTCUTS ===
                let tool_changed = match key.as_str() {
                    "l" | "L" => {
                        s.sketch_tool = SketchTool::Line;
                        s.temp_points.clear();
                        true
                    }
                    "a" | "A" => {
                        s.sketch_tool = SketchTool::Arc;
                        s.temp_points.clear();
                        true
                    }
                    "r" | "R" => {
                        s.sketch_tool = SketchTool::Rectangle;
                        s.temp_points.clear();
                        true
                    }
                    "c" | "C" => {
                        s.sketch_tool = SketchTool::Circle;
                        s.temp_points.clear();
                        true
                    }
                    "p" | "P" => {
                        s.sketch_tool = SketchTool::Point;
                        s.temp_points.clear();
                        true
                    }
                    "s" | "S" => {
                        s.sketch_tool = SketchTool::Select;
                        s.temp_points.clear();
                        true
                    }
                    "x" | "X" => {
                        s.construction_mode = !s.construction_mode;
                        let msg = if s.construction_mode {
                            "Construction mode ON"
                        } else {
                            "Construction mode OFF"
                        };
                        drop(s);
                        show_status(msg, StatusType::Info);
                        update_status_bar();
                        return;
                    }
                    "h" | "H" => {
                        drop(s);
                        let _ = apply_sketch_constraint("horizontal");
                        return;
                    }
                    "v" | "V" => {
                        drop(s);
                        let _ = apply_sketch_constraint("vertical");
                        return;
                    }
                    "z" | "Z" if ctrl => {
                        // Undo/Redo placeholder - will be implemented in Phase 4
                        drop(s);
                        if shift {
                            show_status("Redo not yet implemented", StatusType::Warning);
                        } else {
                            show_status("Undo not yet implemented", StatusType::Warning);
                        }
                        return;
                    }
                    "e" | "E" if ctrl => {
                        event.prevent_default();
                        drop(s);
                        let _ = extrude_current_sketch();
                        return;
                    }
                    "Delete" | "Backspace" => {
                        if !s.selected_entities.is_empty() {
                            let count = s.selected_entities.len();
                            let to_delete = s.selected_entities.clone();
                            s.selected_entities.clear();

                            if let Some(sketch) = s.current_sketch.as_mut() {
                                sketch.entities.retain(|e| {
                                    let id = match e {
                                        SketchEntity::Line { id, .. } => *id,
                                        SketchEntity::Circle { id, .. } => *id,
                                        SketchEntity::Arc { id, .. } => *id,
                                        SketchEntity::Point { id, .. } => *id,
                                    };
                                    !to_delete.contains(&id)
                                });
                            }
                            drop(s);
                            show_status(&format!("Deleted {} entities", count), StatusType::Success);
                            let _ = render();
                            update_status_bar();
                            return;
                        }
                        false
                    }
                    _ => false,
                };
                drop(s);
                if tool_changed {
                    update_status_bar();
                }
                let _ = render();
            });
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        // Attach to document for global keyboard handling
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        document.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
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
            show_status("Solid A created", StatusType::Success);
        } else if state.solid_b.is_none() {
            state.solid_b = Some(solid.clone());
            show_status("Solid B ready for Boolean ops", StatusType::Success);
        } else {
            show_status("Solid created", StatusType::Success);
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
                if let Some(ref solids) = state.pattern_instances {
                    for solid in solids {
                        draw_wireframe(
                            &ctx,
                            solid,
                            css_width,
                            css_height,
                            state.rotation_x,
                            state.rotation_y,
                            state.zoom,
                            state.pan_3d,
                        )
                        .ok();
                    }
                } else if let Some(ref solid) = state.solid {
                    draw_wireframe(
                        &ctx,
                        solid,
                        css_width,
                        css_height,
                        state.rotation_x,
                        state.rotation_y,
                        state.zoom,
                        state.pan_3d,
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
                        &state.selected_entities,
                        state.hover_entity,
                        state.mouse_pos,
                        &state.sketch_constraints,
                        state.current_snap.as_ref(),
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
    pan_3d: (f32, f32),
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

        // Simple orthographic projection with pan offset
        let px = cx + (x1 as f64) * scale + (pan_3d.0 as f64);
        let py = cy - (y1 as f64) * scale + (pan_3d.1 as f64);

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
    selected: &[SketchEntityId],
    hover: Option<SketchEntityId>,
    mouse_pos: Option<Point2>,
    constraints: &[Constraint],
    current_snap: Option<&SnapResult>,
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

    // Draw sketch entities with selection/hover highlighting
    for entity in &sketch.entities {
        let entity_id = match entity {
            SketchEntity::Line { id, .. } => *id,
            SketchEntity::Circle { id, .. } => *id,
            SketchEntity::Arc { id, .. } => *id,
            SketchEntity::Point { id, .. } => *id,
        };

        // Check if this is construction geometry
        let is_construction = is_entity_construction(sketch, entity);

        // Determine color based on state and construction mode
        let is_selected = selected.contains(&entity_id);
        let is_hovered = hover == Some(entity_id);
        let color = if is_construction {
            if is_selected {
                "#88aa00"  // Muted yellow-green for selected construction
            } else if is_hovered {
                "#66cc88"  // Light green for hover construction
            } else {
                "#558833"  // Muted green for construction
            }
        } else if is_selected {
            "#ffdd00"  // Yellow for selected
        } else if is_hovered {
            "#00ddff"  // Cyan for hover
        } else {
            "#ff6b35"  // Default orange
        };

        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(if is_selected || is_hovered { 3.0 } else { 2.0 });

        // Use dashed line for construction geometry
        if is_construction {
            ctx.set_line_dash(&js_sys::Array::of2(&JsValue::from_f64(6.0), &JsValue::from_f64(4.0)))?;
        } else {
            ctx.set_line_dash(&js_sys::Array::new())?;
        }

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
                    let (scx, scy) = to_screen(c.position);
                    let r = *radius as f64 * zoom as f64;
                    ctx.begin_path();
                    ctx.arc(scx, scy, r, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.stroke();
                }
            }
            SketchEntity::Arc {
                center,
                start,
                end,
                radius,
                ccw,
                ..
            } => {
                if let (Some(c), Some(p1), Some(p2)) =
                    (sketch.point(*center), sketch.point(*start), sketch.point(*end))
                {
                    let (scx, scy) = to_screen(c.position);
                    let r = *radius as f64 * zoom as f64;

                    let v1 = Point2::new(p1.position.x - c.position.x, p1.position.y - c.position.y);
                    let v2 = Point2::new(p2.position.x - c.position.x, p2.position.y - c.position.y);
                    let a1 = -(v1.y.atan2(v1.x)) as f64;
                    let a2 = -(v2.y.atan2(v2.x)) as f64;

                    ctx.begin_path();
                    ctx.arc_with_anticlockwise(scx, scy, r, a1, a2, *ccw)?;
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
                // Draw first point marker
                let (x1, y1) = to_screen(temp_points[0]);
                ctx.set_fill_style(&JsValue::from_str("#ff6b35"));
                ctx.begin_path();
                ctx.arc(x1, y1, 6.0, 0.0, 2.0 * std::f64::consts::PI)?;
                ctx.fill();

                // If we have mouse position, draw preview line
                if let Some(mp) = mouse_pos {
                    let (x2, y2) = to_screen(mp);
                    ctx.begin_path();
                    ctx.move_to(x1, y1);
                    ctx.line_to(x2, y2);
                    ctx.stroke();
                }
            }
            SketchTool::Rectangle if temp_points.len() == 1 => {
                let p1 = temp_points[0];
                let (x1, y1) = to_screen(p1);

                // First point marker
                ctx.set_fill_style(&JsValue::from_str("#ff6b35"));
                ctx.begin_path();
                ctx.arc(x1, y1, 6.0, 0.0, 2.0 * std::f64::consts::PI)?;
                ctx.fill();

                if let Some(mp) = mouse_pos {
                    let (x2, y2) = to_screen(mp);
                    let left = x1.min(x2);
                    let right = x1.max(x2);
                    let top = y1.min(y2);
                    let bottom = y1.max(y2);
                    ctx.begin_path();
                    ctx.rect(left, top, right - left, bottom - top);
                    ctx.stroke();
                }
            }
            SketchTool::Circle if temp_points.len() == 1 => {
                // Draw center marker
                let (cx, cy) = to_screen(temp_points[0]);
                ctx.set_fill_style(&JsValue::from_str("#ff6b35"));
                ctx.begin_path();
                ctx.arc(cx, cy, 6.0, 0.0, 2.0 * std::f64::consts::PI)?;
                ctx.fill();

                // If we have mouse position, draw preview circle
                if let Some(mp) = mouse_pos {
                    let (mx, my) = to_screen(mp);
                    let radius = ((mx - cx).powi(2) + (my - cy).powi(2)).sqrt();
                    ctx.begin_path();
                    ctx.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.stroke();
                }
            }
            SketchTool::Arc if temp_points.len() == 1 => {
                let (x1, y1) = to_screen(temp_points[0]);
                ctx.set_fill_style(&JsValue::from_str("#ff6b35"));
                ctx.begin_path();
                ctx.arc(x1, y1, 6.0, 0.0, 2.0 * std::f64::consts::PI)?;
                ctx.fill();

                if let Some(mp) = mouse_pos {
                    let (x2, y2) = to_screen(mp);
                    ctx.begin_path();
                    ctx.move_to(x1, y1);
                    ctx.line_to(x2, y2);
                    ctx.stroke();
                }
            }
            SketchTool::Arc if temp_points.len() == 2 => {
                if let Some(mp) = mouse_pos {
                    let p1 = temp_points[0];
                    let pm = temp_points[1];
                    let p2 = mp;
                    if let Some(c) = circumcenter(p1, pm, p2) {
                        let radius = c.distance(&p1) as f64 * zoom as f64;
                        let (scx, scy) = to_screen(c);

                        let a1 = -((p1.y - c.y).atan2(p1.x - c.x)) as f64;
                        let am = -((pm.y - c.y).atan2(pm.x - c.x)) as f64;
                        let a2 = -((p2.y - c.y).atan2(p2.x - c.x)) as f64;

                        // Decide ccw in sketch space so the sweep passes through mid point.
                        let s = normalize_angle((p1.y - c.y).atan2(p1.x - c.x));
                        let m = normalize_angle((pm.y - c.y).atan2(pm.x - c.x));
                        let e = normalize_angle((p2.y - c.y).atan2(p2.x - c.x));
                        let ccw = angle_in_ccw_sweep(s, e, m);

                        ctx.begin_path();
                        ctx.arc_with_anticlockwise(scx, scy, radius, a1, a2, ccw)?;
                        ctx.stroke();
                    }
                }
            }
            _ => {}
        }
        ctx.set_line_dash(&js_sys::Array::new())?;  // Reset dash
    }

    // Draw constraint indicators
    ctx.set_fill_style(&JsValue::from_str("#00ff88"));
    ctx.set_font("bold 12px monospace");
    for constraint in constraints {
        match constraint {
            Constraint::Geometric(GeometricConstraint::Horizontal { line }) => {
                // Find the line entity and get its midpoint
                if let Some(entity) = sketch.entities.iter().find(|e| {
                    matches!(e, SketchEntity::Line { id, .. } if id == line)
                }) {
                    if let SketchEntity::Line { start, end, .. } = entity {
                        if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                            let mid = Point2::new(
                                (p1.position.x + p2.position.x) / 2.0,
                                (p1.position.y + p2.position.y) / 2.0,
                            );
                            let (sx, sy) = to_screen(mid);
                            ctx.fill_text("H", sx - 5.0, sy - 10.0)?;
                        }
                    }
                }
            }
            Constraint::Geometric(GeometricConstraint::Vertical { line }) => {
                if let Some(entity) = sketch.entities.iter().find(|e| {
                    matches!(e, SketchEntity::Line { id, .. } if id == line)
                }) {
                    if let SketchEntity::Line { start, end, .. } = entity {
                        if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                            let mid = Point2::new(
                                (p1.position.x + p2.position.x) / 2.0,
                                (p1.position.y + p2.position.y) / 2.0,
                            );
                            let (sx, sy) = to_screen(mid);
                            ctx.fill_text("V", sx + 10.0, sy)?;
                        }
                    }
                }
            }
            Constraint::Geometric(GeometricConstraint::Perpendicular { .. }) => {
                // Draw ⊥ symbol at intersection (simplified - just at first line midpoint)
                ctx.fill_text("⊥", 20.0, height - 20.0)?;
            }
            _ => {}
        }
    }

    // Draw snap indicator
    if let Some(snap) = current_snap {
        if snap.snap_type != SnapType::None {
            let (sx, sy) = to_screen(snap.position);
            let size = 8.0;

            match snap.snap_type {
                SnapType::Point => {
                    // Green circle for endpoint snap
                    ctx.set_stroke_style(&JsValue::from_str("#00ff88"));
                    ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 136, 0.3)"));
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.arc(sx, sy, size, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.fill();
                    ctx.stroke();
                }
                SnapType::Midpoint => {
                    // Yellow triangle for midpoint snap
                    ctx.set_stroke_style(&JsValue::from_str("#ffdd00"));
                    ctx.set_fill_style(&JsValue::from_str("rgba(255, 221, 0, 0.3)"));
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.move_to(sx, sy - size);
                    ctx.line_to(sx - size * 0.866, sy + size * 0.5);
                    ctx.line_to(sx + size * 0.866, sy + size * 0.5);
                    ctx.close_path();
                    ctx.fill();
                    ctx.stroke();
                }
                SnapType::Center => {
                    // Orange crosshairs for center snap
                    ctx.set_stroke_style(&JsValue::from_str("#ff8800"));
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.move_to(sx - size, sy);
                    ctx.line_to(sx + size, sy);
                    ctx.move_to(sx, sy - size);
                    ctx.line_to(sx, sy + size);
                    ctx.stroke();
                    // Circle around it
                    ctx.begin_path();
                    ctx.arc(sx, sy, size * 0.6, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.stroke();
                }
                SnapType::Intersection => {
                    // Magenta X for intersection snap
                    ctx.set_stroke_style(&JsValue::from_str("#ff00ff"));
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.move_to(sx - size, sy - size);
                    ctx.line_to(sx + size, sy + size);
                    ctx.move_to(sx - size, sy + size);
                    ctx.line_to(sx + size, sy - size);
                    ctx.stroke();
                }
                SnapType::Perpendicular => {
                    // Cyan square for perpendicular snap
                    ctx.set_stroke_style(&JsValue::from_str("#00ffff"));
                    ctx.set_fill_style(&JsValue::from_str("rgba(0, 255, 255, 0.3)"));
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.rect(sx - size * 0.7, sy - size * 0.7, size * 1.4, size * 1.4);
                    ctx.fill();
                    ctx.stroke();
                }
                SnapType::Grid => {
                    // Small white dot for grid snap
                    ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.5)"));
                    ctx.begin_path();
                    ctx.arc(sx, sy, 3.0, 0.0, 2.0 * std::f64::consts::PI)?;
                    ctx.fill();
                }
                SnapType::None => {}
            }
        }
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

/// Show a status message in the status bar
fn show_status(message: &str, msg_type: StatusType) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(elem) = document.get_element_by_id("status-message-value") {
                if let Ok(elem) = elem.dyn_into::<HtmlElement>() {
                    elem.set_inner_text(message);
                    let class = match msg_type {
                        StatusType::Info => "info",
                        StatusType::Warning => "warning",
                        StatusType::Error => "error",
                        StatusType::Success => "success",
                    };
                    elem.set_class_name(class);
                }
            }
        }
    }
}

/// Update the status bar with current state
fn update_status_bar() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            STATE.with(|state| {
                let s = state.borrow();

                // Update mode
                let mode_text = match s.mode {
                    AppMode::View3D => "3D View",
                    AppMode::Sketch2D => "Sketch 2D",
                };
                set_text(&document, "status-mode", mode_text).ok();

                // Update tool (only meaningful in sketch mode)
                let tool_text = if s.mode == AppMode::Sketch2D {
                    match s.sketch_tool {
                        SketchTool::Select => "Select [S]",
                        SketchTool::Line => "Line [L]",
                        SketchTool::Arc => "Arc [A]",
                        SketchTool::Rectangle => "Rect [R]",
                        SketchTool::Circle => "Circle [C]",
                        SketchTool::Point => "Point [P]",
                    }
                } else {
                    "-"
                };
                set_text(&document, "status-tool", tool_text).ok();

                // Update constraint count
                set_text(&document, "status-constraints", &s.sketch_constraints.len().to_string()).ok();

                // Update DOF status
                if s.mode == AppMode::Sketch2D {
                    if let Some(ref sketch) = s.current_sketch {
                        let analysis = ConstraintAnalysis::analyze(sketch, &s.sketch_constraints);
                        let dof_text = match &analysis.dof_status {
                            DofStatus::FullyConstrained => "Fully".to_string(),
                            DofStatus::UnderConstrained { dof } => format!("-{} DOF", dof),
                            DofStatus::OverConstrained { redundant } => format!("+{} OC", redundant),
                        };
                        set_text(&document, "status-dof", &dof_text).ok();
                    }
                } else {
                    set_text(&document, "status-dof", "-").ok();
                }

                // Update coordinates if in sketch mode with mouse position
                if s.mode == AppMode::Sketch2D {
                    if let Some(pos) = s.mouse_pos {
                        set_text(&document, "status-coords", &format!("X: {:.1} Y: {:.1}", pos.x, pos.y)).ok();
                    }
                } else {
                    set_text(&document, "status-coords", "X: - Y: -").ok();
                }
            });
        }
    }
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
            show_status("Exporting STEP...", StatusType::Info);
            let step_content = solid_to_step(solid, "solid");
            download_file("model.step", &step_content)?;
            show_status("STEP export complete", StatusType::Success);
        } else {
            show_status("No solid to export", StatusType::Warning);
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

fn solid_bbox_center(solid: &Solid) -> Point3 {
    if solid.vertices.is_empty() {
        return Point3::new(0.0, 0.0, 0.0);
    }
    let mut minx = f32::INFINITY;
    let mut miny = f32::INFINITY;
    let mut minz = f32::INFINITY;
    let mut maxx = f32::NEG_INFINITY;
    let mut maxy = f32::NEG_INFINITY;
    let mut maxz = f32::NEG_INFINITY;

    for v in &solid.vertices {
        minx = minx.min(v.point.x);
        miny = miny.min(v.point.y);
        minz = minz.min(v.point.z);
        maxx = maxx.max(v.point.x);
        maxy = maxy.max(v.point.y);
        maxz = maxz.max(v.point.z);
    }

    Point3::new((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)
}

fn apply_linear_pattern() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let count = get_input_value(&document, "pattern-linear-count")? as u32;
    let spacing = get_input_value(&document, "pattern-linear-spacing")? as f32;

    let dir_select = document
        .get_element_by_id("pattern-linear-direction")
        .ok_or("Pattern linear direction select not found")?;
    let dir_select: HtmlSelectElement = dir_select.dyn_into()?;
    let dir = match dir_select.value().as_str() {
        "y" => Vector3::Y,
        "z" => Vector3::Z,
        _ => Vector3::X,
    };

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let solid = s
            .solid
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No solid to pattern"))?;

        let instances = linear_pattern(solid, dir, count.max(1), spacing);
        s.pattern_instances = Some(instances);
        drop(s);
        render()?;
        Ok(())
    })
}

fn apply_circular_pattern() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let count = get_input_value(&document, "pattern-circular-count")? as u32;

    let axis_select = document
        .get_element_by_id("pattern-circular-axis")
        .ok_or("Pattern circular axis select not found")?;
    let axis_select: HtmlSelectElement = axis_select.dyn_into()?;
    let axis = match axis_select.value().as_str() {
        "x" => Vector3::X,
        "y" => Vector3::Y,
        _ => Vector3::Z,
    };

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let solid = s
            .solid
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No solid to pattern"))?;

        let center = solid_bbox_center(solid);
        let instances = circular_pattern(solid, axis, center, count.max(1));
        s.pattern_instances = Some(instances);
        drop(s);
        render()?;
        Ok(())
    })
}

fn clear_pattern() -> Result<(), JsValue> {
    STATE.with(|state| {
        state.borrow_mut().pattern_instances = None;
    });
    render()?;
    Ok(())
}

fn export_stl() -> Result<(), JsValue> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(ref solid) = state.solid {
            show_status("Exporting STL...", StatusType::Info);
            let stl_binary = solid_to_stl(solid, "solid");
            download_binary_file("model.stl", &stl_binary)?;
            show_status("STL export complete", StatusType::Success);
        } else {
            show_status("No solid to export", StatusType::Warning);
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

        show_status(&format!("Boolean {}...", operation), StatusType::Info);

        let result = match operation {
            "union" => union(solid_a, solid_b),
            "difference" => difference(solid_a, solid_b),
            "intersection" => intersection(solid_a, solid_b),
            _ => return Err(JsValue::from_str("Unknown operation")),
        };

        match result {
            Ok(new_solid) => {
                state.solid = Some(new_solid);
                state.pattern_instances = None; // Clear patterns on topology-changing ops
                state.solid_a = None;  // Reset for next operation
                state.solid_b = None;
                drop(state);

                show_status(&format!("Boolean {} complete", operation), StatusType::Success);

                let window = web_sys::window().ok_or("No window")?;
                let document = window.document().ok_or("No document")?;
                display_properties(&document)?;
                render()?;
                Ok(())
            }
            Err(e) => {
                show_status(&format!("Boolean failed: {:?}", e), StatusType::Error);
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
                s.current_sketch = Some(Sketch::new(SketchPlane::XY));
                s.temp_points.clear();
                AppMode::Sketch2D
            }
            AppMode::Sketch2D => {
                AppMode::View3D
            }
        };
        s.mode
    });

    // Update status bar
    let mode_msg = if new_mode == AppMode::Sketch2D {
        "Sketch mode - draw with L/A/R/C/P keys"
    } else {
        "3D View mode"
    };
    show_status(mode_msg, StatusType::Info);
    update_status_bar();

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
            "rectangle" => SketchTool::Rectangle,
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

fn revolve_current_sketch() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let angle = get_input_value(&document, "revolve-angle")? as f32;
    let segments = get_input_value(&document, "revolve-segments")? as u32;

    let axis_select = document
        .get_element_by_id("revolve-axis")
        .ok_or("Revolve axis select not found")?;
    let axis_select: HtmlSelectElement = axis_select.dyn_into()?;
    let axis = match axis_select.value().as_str() {
        "x" => RevolveAxis::X,
        _ => RevolveAxis::Y,
    };

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let sketch = s
            .current_sketch
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No sketch to revolve"))?;

        web_sys::console::log_1(&"Revolving sketch...".into());

        let params = RevolveParams {
            angle_degrees: angle,
            axis,
            segments: segments.max(3),
        };

        match revolve_sketch(sketch, &params) {
            Ok(solid) => {
                s.solid = Some(solid);
                s.mode = AppMode::View3D;
                drop(s);

                web_sys::console::log_1(&"Revolve complete - switched to 3D view".into());
                display_properties(&document)?;
                render()?;
                Ok(())
            }
            Err(e) => Err(JsValue::from_str(&format!("Revolve failed: {:?}", e))),
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

    let raw_pos = Point2::new(sketch_x, sketch_y);

    // Apply enhanced snapping (needs read-only access first)
    let snap_result = STATE.with(|state| {
        let s = state.borrow();
        if let Some(ref sketch) = s.current_sketch {
            snap_position_enhanced(sketch, raw_pos, 15.0, 50.0)  // 15px tolerance, 50mm grid
        } else {
            SnapResult::none(raw_pos)
        }
    });

    let snap_type_str = match snap_result.snap_type {
        SnapType::Point => "Point",
        SnapType::Midpoint => "Mid",
        SnapType::Center => "Center",
        SnapType::Intersection => "Int",
        SnapType::Perpendicular => "Perp",
        SnapType::Grid => "Grid",
        SnapType::None => "-",
    };

    show_status(&format!("Snap: {} ({:.1}, {:.1})", snap_type_str, snap_result.position.x, snap_result.position.y), StatusType::Info);

    let pos = snap_result.position;

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let tool = s.sketch_tool;
        let construction = s.construction_mode;
        let has_temp_point = !s.temp_points.is_empty();
        let first_temp = if has_temp_point { Some(s.temp_points[0]) } else { None };

        if s.current_sketch.is_some() {
            match tool {
                SketchTool::Line => {
                    if let Some(p1) = first_temp {
                        let sketch = s.current_sketch.as_mut().unwrap();
                        let p1_id = add_point_with_construction(sketch, p1, construction);
                        let p2_id = add_point_with_construction(sketch, pos, construction);
                        let line_id = next_entity_id(sketch);

                        sketch.add_entity(SketchEntity::Line {
                            id: line_id,
                            start: p1_id,
                            end: p2_id,
                        });

                        // Record undo command
                        s.command_history.push(SketchCommand::AddGeometry {
                            point_ids: vec![p1_id, p2_id],
                            entity_ids: vec![line_id],
                        });

                        s.temp_points.clear();
                        show_status(&format!("Line: ({:.1},{:.1}) → ({:.1},{:.1})", p1.x, p1.y, pos.x, pos.y), StatusType::Success);
                    } else {
                        s.temp_points.push(pos);
                        show_status(&format!("Line: click end point (start: {:.1}, {:.1})", pos.x, pos.y), StatusType::Info);
                    }
                }
                SketchTool::Circle => {
                    if let Some(center) = first_temp {
                        let sketch = s.current_sketch.as_mut().unwrap();
                        let center_id = add_point_with_construction(sketch, center, construction);
                        let radius = center.distance(&pos);
                        let circle_id = next_entity_id(sketch);

                        sketch.add_entity(SketchEntity::Circle {
                            id: circle_id,
                            center: center_id,
                            radius,
                        });

                        // Record undo command
                        s.command_history.push(SketchCommand::AddGeometry {
                            point_ids: vec![center_id],
                            entity_ids: vec![circle_id],
                        });

                        s.temp_points.clear();
                        show_status(&format!("Circle: center ({:.1},{:.1}), r={:.1}", center.x, center.y, radius), StatusType::Success);
                    } else {
                        s.temp_points.push(pos);
                        show_status(&format!("Circle: click radius (center: {:.1}, {:.1})", pos.x, pos.y), StatusType::Info);
                    }
                }
                SketchTool::Rectangle => {
                    if let Some(p1) = first_temp {
                        let p2 = pos;

                        let x_min = p1.x.min(p2.x);
                        let x_max = p1.x.max(p2.x);
                        let y_min = p1.y.min(p2.y);
                        let y_max = p1.y.max(p2.y);

                        let a = Point2::new(x_min, y_min);
                        let b = Point2::new(x_max, y_min);
                        let c = Point2::new(x_max, y_max);
                        let d = Point2::new(x_min, y_max);

                        let sketch = s.current_sketch.as_mut().unwrap();
                        let a_id = add_point_with_construction(sketch, a, construction);
                        let b_id = add_point_with_construction(sketch, b, construction);
                        let c_id = add_point_with_construction(sketch, c, construction);
                        let d_id = add_point_with_construction(sketch, d, construction);

                        let id0 = next_entity_id(sketch);
                        let id1 = SketchEntityId(id0.0 + 1);
                        let id2 = SketchEntityId(id0.0 + 2);
                        let id3 = SketchEntityId(id0.0 + 3);

                        sketch.add_entity(SketchEntity::Line { id: id0, start: a_id, end: b_id });
                        sketch.add_entity(SketchEntity::Line { id: id1, start: b_id, end: c_id });
                        sketch.add_entity(SketchEntity::Line { id: id2, start: c_id, end: d_id });
                        sketch.add_entity(SketchEntity::Line { id: id3, start: d_id, end: a_id });

                        s.sketch_constraints.push(Constraint::Geometric(GeometricConstraint::Horizontal { line: id0 }));
                        s.sketch_constraints.push(Constraint::Geometric(GeometricConstraint::Vertical { line: id1 }));
                        s.sketch_constraints.push(Constraint::Geometric(GeometricConstraint::Horizontal { line: id2 }));
                        s.sketch_constraints.push(Constraint::Geometric(GeometricConstraint::Vertical { line: id3 }));

                        // Record undo command
                        s.command_history.push(SketchCommand::AddGeometry {
                            point_ids: vec![a_id, b_id, c_id, d_id],
                            entity_ids: vec![id0, id1, id2, id3],
                        });

                        s.temp_points.clear();
                        show_status(&format!("Rectangle: ({:.1},{:.1}) to ({:.1},{:.1})", x_min, y_min, x_max, y_max), StatusType::Success);
                    } else {
                        s.temp_points.push(pos);
                        show_status(&format!("Rectangle: click opposite corner (start: {:.1}, {:.1})", pos.x, pos.y), StatusType::Info);
                    }
                }
                SketchTool::Point => {
                    let sketch = s.current_sketch.as_mut().unwrap();
                    add_point_with_construction(sketch, pos, construction);
                    web_sys::console::log_1(&format!("Point created at ({:.1}, {:.1})", pos.x, pos.y).into());
                }
                SketchTool::Select => {
                    let sketch = s.current_sketch.as_ref().unwrap();
                    if let Some(entity_id) = find_entity_at_point(sketch, pos, 10.0) {
                        if let Some(idx) = s.selected_entities.iter().position(|&id| id == entity_id) {
                            s.selected_entities.remove(idx);
                            web_sys::console::log_1(&format!("Deselected entity {:?}", entity_id).into());
                        } else {
                            s.selected_entities.push(entity_id);
                            web_sys::console::log_1(&format!("Selected entity {:?}", entity_id).into());
                        }
                    } else {
                        s.selected_entities.clear();
                        web_sys::console::log_1(&"Selection cleared".into());
                    }
                }
                SketchTool::Arc => {
                    // 3-point arc: start, mid(on arc), end
                    if s.temp_points.len() < 2 {
                        s.temp_points.push(pos);
                        web_sys::console::log_1(&format!(
                            "Arc: point {} at ({:.1}, {:.1})",
                            s.temp_points.len(),
                            pos.x,
                            pos.y
                        ).into());
                    } else {
                        let p1 = s.temp_points[0];
                        let pm = s.temp_points[1];
                        let p2 = pos;
                        s.temp_points.clear();

                        let Some(center) = circumcenter(p1, pm, p2) else {
                            web_sys::console::warn_1(&"Arc: points are collinear; cancelled".into());
                            return Ok(());
                        };

                        let radius = center.distance(&p1);
                        let start_ang = normalize_angle((p1.y - center.y).atan2(p1.x - center.x));
                        let mid_ang = normalize_angle((pm.y - center.y).atan2(pm.x - center.x));
                        let end_ang = normalize_angle((p2.y - center.y).atan2(p2.x - center.x));
                        let ccw = angle_in_ccw_sweep(start_ang, end_ang, mid_ang);

                        let sketch = s.current_sketch.as_mut().unwrap();
                        let center_id = add_point_with_construction(sketch, center, construction);
                        let start_id = add_point_with_construction(sketch, p1, construction);
                        let end_id = add_point_with_construction(sketch, p2, construction);
                        let arc_id = next_entity_id(sketch);

                        sketch.add_entity(SketchEntity::Arc {
                            id: arc_id,
                            center: center_id,
                            start: start_id,
                            end: end_id,
                            radius,
                            ccw,
                        });

                        web_sys::console::log_1(&format!(
                            "Arc created: center ({:.1},{:.1}) r={:.1} ccw={}",
                            center.x, center.y, radius, ccw
                        ).into());
                    }
                }
            }
        }

        drop(s);
        render()?;
        Ok(())
    })
}

/// Find the closest entity to a point within tolerance
fn find_entity_at_point(sketch: &Sketch, pos: Point2, tolerance: f32) -> Option<SketchEntityId> {
    let mut best_id: Option<SketchEntityId> = None;
    let mut best_dist = tolerance;

    for entity in &sketch.entities {
        let dist = match entity {
            SketchEntity::Line { id, start, end } => {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    point_to_segment_distance(pos, p1.position, p2.position)
                } else {
                    f32::MAX
                }
            }
            SketchEntity::Circle { id, center, radius } => {
                if let Some(c) = sketch.point(*center) {
                    let dist_to_center = pos.distance(&c.position);
                    (dist_to_center - *radius).abs()
                } else {
                    f32::MAX
                }
            }
            SketchEntity::Arc { id, center, radius, .. } => {
                if let Some(c) = sketch.point(*center) {
                    let dist_to_center = pos.distance(&c.position);
                    (dist_to_center - *radius).abs()
                } else {
                    f32::MAX
                }
            }
            SketchEntity::Point { id, point } => {
                if let Some(p) = sketch.point(*point) {
                    pos.distance(&p.position)
                } else {
                    f32::MAX
                }
            }
        };

        if dist < best_dist {
            best_dist = dist;
            best_id = Some(match entity {
                SketchEntity::Line { id, .. } => *id,
                SketchEntity::Circle { id, .. } => *id,
                SketchEntity::Arc { id, .. } => *id,
                SketchEntity::Point { id, .. } => *id,
            });
        }
    }

    best_id
}

/// Find the closest point to a position within tolerance
fn find_point_at_position(sketch: &Sketch, pos: Point2, tolerance: f32) -> Option<SketchPointId> {
    let mut best_id: Option<SketchPointId> = None;
    let mut best_dist = tolerance;

    for point in &sketch.points {
        let dist = pos.distance(&point.position);
        if dist < best_dist {
            best_dist = dist;
            best_id = Some(point.id);
        }
    }

    best_id
}

/// Snap position to grid
fn snap_to_grid(pos: Point2, grid_size: f32) -> Point2 {
    Point2::new(
        (pos.x / grid_size).round() * grid_size,
        (pos.y / grid_size).round() * grid_size,
    )
}

/// Snap to existing point if close enough, otherwise snap to grid
fn snap_position(sketch: &Sketch, pos: Point2, point_tolerance: f32, grid_size: f32) -> (Point2, Option<SketchPointId>) {
    // Priority 1: Snap to existing points
    if let Some(point_id) = find_point_at_position(sketch, pos, point_tolerance) {
        if let Some(point) = sketch.point(point_id) {
            return (point.position, Some(point_id));
        }
    }

    // Priority 2: Snap to grid
    (snap_to_grid(pos, grid_size), None)
}

/// Enhanced snap with multiple snap types
/// Priority: Point > Midpoint > Center > Intersection > Perpendicular > Grid
fn snap_position_enhanced(sketch: &Sketch, pos: Point2, tolerance: f32, grid_size: f32) -> SnapResult {
    let tol_sq = tolerance * tolerance;

    // Priority 1: Snap to existing points (endpoints)
    for point in &sketch.points {
        if point.position.distance_squared(&pos) < tol_sq {
            return SnapResult {
                position: point.position,
                snap_type: SnapType::Point,
                source_entity: None,
            };
        }
    }

    // Priority 2: Snap to midpoints
    for entity in &sketch.entities {
        if let Some((mid, entity_id)) = entity_midpoint(sketch, entity) {
            if mid.distance_squared(&pos) < tol_sq {
                return SnapResult {
                    position: mid,
                    snap_type: SnapType::Midpoint,
                    source_entity: Some(entity_id),
                };
            }
        }
    }

    // Priority 3: Snap to centers (circles and arcs)
    for entity in &sketch.entities {
        match entity {
            SketchEntity::Circle { id, center, .. } | SketchEntity::Arc { id, center, .. } => {
                if let Some(center_pt) = sketch.point(*center) {
                    if center_pt.position.distance_squared(&pos) < tol_sq {
                        return SnapResult {
                            position: center_pt.position,
                            snap_type: SnapType::Center,
                            source_entity: Some(*id),
                        };
                    }
                }
            }
            _ => {}
        }
    }

    // Priority 4: Snap to intersections
    if let Some(intersection) = find_nearest_intersection(sketch, pos, tolerance) {
        return intersection;
    }

    // Priority 5: Snap to perpendicular foot (when near a line)
    for entity in &sketch.entities {
        if let SketchEntity::Line { id, start, end, .. } = entity {
            if let (Some(a), Some(b)) = (sketch.point(*start), sketch.point(*end)) {
                if let Some(foot) = perpendicular_foot(pos, a.position, b.position) {
                    if foot.distance_squared(&pos) < tol_sq {
                        return SnapResult {
                            position: foot,
                            snap_type: SnapType::Perpendicular,
                            source_entity: Some(*id),
                        };
                    }
                }
            }
        }
    }

    // Priority 6: Snap to grid
    SnapResult::grid(pos, grid_size)
}

/// Get midpoint of an entity
fn entity_midpoint(sketch: &Sketch, entity: &SketchEntity) -> Option<(Point2, SketchEntityId)> {
    match entity {
        SketchEntity::Line { id, start, end, .. } => {
            let a = sketch.point(*start)?.position;
            let b = sketch.point(*end)?.position;
            Some((a.midpoint(&b), *id))
        }
        SketchEntity::Arc { id, center, start, end, radius, ccw, .. } => {
            // Arc midpoint is the point on the arc at the middle angle
            let c = sketch.point(*center)?.position;
            let s = sketch.point(*start)?.position;
            let e = sketch.point(*end)?.position;

            let start_angle = (s.y - c.y).atan2(s.x - c.x);
            let end_angle = (e.y - c.y).atan2(e.x - c.x);

            let mut sweep = if *ccw {
                end_angle - start_angle
            } else {
                start_angle - end_angle
            };
            if sweep < 0.0 {
                sweep += 2.0 * PI;
            }

            let mid_angle = if *ccw {
                start_angle + sweep / 2.0
            } else {
                start_angle - sweep / 2.0
            };

            let mid = Point2::new(
                c.x + radius * mid_angle.cos(),
                c.y + radius * mid_angle.sin(),
            );
            Some((mid, *id))
        }
        _ => None,
    }
}

/// Find the closest point on a line segment from a given point (perpendicular foot)
/// Returns None if the foot is outside the segment
fn perpendicular_foot(p: Point2, a: Point2, b: Point2) -> Option<Point2> {
    let ab = Point2::new(b.x - a.x, b.y - a.y);
    let ap = Point2::new(p.x - a.x, p.y - a.y);

    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq < 1e-10 {
        return None;
    }

    // Project p onto ab
    let t = (ap.x * ab.x + ap.y * ab.y) / ab_len_sq;

    // Only return if foot is within segment (with small margin)
    if t >= 0.05 && t <= 0.95 {
        Some(Point2::new(a.x + t * ab.x, a.y + t * ab.y))
    } else {
        None
    }
}

/// Find intersection of two line segments
fn line_line_intersection(a1: Point2, a2: Point2, b1: Point2, b2: Point2) -> Option<Point2> {
    let d1 = Point2::new(a2.x - a1.x, a2.y - a1.y);
    let d2 = Point2::new(b2.x - b1.x, b2.y - b1.y);

    let cross = d1.x * d2.y - d1.y * d2.x;
    if cross.abs() < 1e-10 {
        return None; // Parallel
    }

    let dx = b1.x - a1.x;
    let dy = b1.y - a1.y;

    let t = (dx * d2.y - dy * d2.x) / cross;
    let u = (dx * d1.y - dy * d1.x) / cross;

    // Check if intersection is within both segments
    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Some(Point2::new(a1.x + t * d1.x, a1.y + t * d1.y))
    } else {
        None
    }
}

/// Find the nearest intersection point to the given position
fn find_nearest_intersection(sketch: &Sketch, pos: Point2, tolerance: f32) -> Option<SnapResult> {
    let mut best: Option<(Point2, f32)> = None;
    let tol_sq = tolerance * tolerance;

    // Collect all lines for pairwise intersection testing
    let lines: Vec<_> = sketch
        .entities
        .iter()
        .filter_map(|e| {
            if let SketchEntity::Line { start, end, .. } = e {
                let a = sketch.point(*start)?.position;
                let b = sketch.point(*end)?.position;
                Some((a, b))
            } else {
                None
            }
        })
        .collect();

    // Test all pairs of lines
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if let Some(intersection) = line_line_intersection(lines[i].0, lines[i].1, lines[j].0, lines[j].1) {
                let dist_sq = intersection.distance_squared(&pos);
                if dist_sq < tol_sq {
                    if best.map_or(true, |(_, d)| dist_sq < d) {
                        best = Some((intersection, dist_sq));
                    }
                }
            }
        }
    }

    best.map(|(position, _)| SnapResult {
        position,
        snap_type: SnapType::Intersection,
        source_entity: None,
    })
}

/// Calculate distance from point to line segment
fn point_to_segment_distance(p: Point2, a: Point2, b: Point2) -> f32 {
    let ab = Point2::new(b.x - a.x, b.y - a.y);
    let ap = Point2::new(p.x - a.x, p.y - a.y);

    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq < 1e-10 {
        return ap.x.hypot(ap.y);
    }

    // Project p onto ab, clamping to segment
    let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
    let closest = Point2::new(a.x + t * ab.x, a.y + t * ab.y);

    p.distance(&closest)
}

/// Undo the last sketch action
fn undo_action() -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        if !s.command_history.can_undo() {
            show_status("Nothing to undo", StatusType::Warning);
            return Ok(());
        }

        if let Some(cmd) = s.command_history.pop_undo() {
            match &cmd {
                SketchCommand::AddGeometry { point_ids, entity_ids } => {
                    if let Some(ref mut sketch) = s.current_sketch {
                        // Remove entities first
                        for entity_id in entity_ids.iter().rev() {
                            sketch.entities.retain(|e| e.id() != *entity_id);
                        }
                        // Remove points (from highest index to lowest to avoid shifting issues)
                        let mut sorted_ids: Vec<_> = point_ids.iter().collect();
                        sorted_ids.sort_by(|a, b| b.0.cmp(&a.0));
                        for point_id in sorted_ids {
                            if (point_id.0 as usize) < sketch.points.len() {
                                sketch.points.remove(point_id.0 as usize);
                            }
                        }
                        show_status("Undo: removed geometry", StatusType::Info);
                    }
                }
                SketchCommand::AddConstraint { index } => {
                    if *index < s.sketch_constraints.len() {
                        s.sketch_constraints.remove(*index);
                        show_status("Undo: removed constraint", StatusType::Info);
                    }
                }
                SketchCommand::ToggleConstruction { point_ids, prev_states } => {
                    if let Some(ref mut sketch) = s.current_sketch {
                        for (point_id, &prev) in point_ids.iter().zip(prev_states.iter()) {
                            if let Some(point) = sketch.point_mut(*point_id) {
                                point.is_construction = prev;
                            }
                        }
                        show_status("Undo: toggled construction", StatusType::Info);
                    }
                }
            }
            s.command_history.push_redo(cmd);
        }

        drop(s);
        let _ = render();
        Ok(())
    })
}

/// Redo the last undone sketch action
fn redo_action() -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        if !s.command_history.can_redo() {
            show_status("Nothing to redo", StatusType::Warning);
            return Ok(());
        }

        // Note: Redo is more complex because we need to recreate the geometry
        // For now, we just show a message and skip the redo
        // A full implementation would need to store the actual point/entity data
        if let Some(cmd) = s.command_history.pop_redo() {
            show_status("Redo not yet implemented for this action", StatusType::Warning);
            s.command_history.push_undo(cmd);
        }

        Ok(())
    })
}

/// Apply a constraint to the currently selected entities
fn apply_sketch_constraint(constraint_type: &str) -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        if s.selected_entities.is_empty() {
            web_sys::console::warn_1(&"No entities selected - select an entity first".into());
            return Err(JsValue::from_str("No entities selected"));
        }

        let sketch = s.current_sketch.as_ref()
            .ok_or_else(|| JsValue::from_str("No active sketch"))?;

        // Get the first selected entity
        let entity_id = s.selected_entities[0];

        // Find the entity and create appropriate constraint
        let constraint = match constraint_type {
            "horizontal" => {
                // Horizontal constraint only applies to lines
                if let Some(entity) = sketch.entities.iter().find(|e| {
                    matches!(e, SketchEntity::Line { id, .. } if *id == entity_id)
                }) {
                    if let SketchEntity::Line { start, end, .. } = entity {
                        Some(Constraint::Geometric(GeometricConstraint::Horizontal {
                            line: entity_id,
                        }))
                    } else {
                        None
                    }
                } else {
                    web_sys::console::warn_1(&"Horizontal constraint requires a line".into());
                    None
                }
            }
            "vertical" => {
                if let Some(entity) = sketch.entities.iter().find(|e| {
                    matches!(e, SketchEntity::Line { id, .. } if *id == entity_id)
                }) {
                    if let SketchEntity::Line { .. } = entity {
                        Some(Constraint::Geometric(GeometricConstraint::Vertical {
                            line: entity_id,
                        }))
                    } else {
                        None
                    }
                } else {
                    web_sys::console::warn_1(&"Vertical constraint requires a line".into());
                    None
                }
            }
            "perpendicular" => {
                if s.selected_entities.len() < 2 {
                    web_sys::console::warn_1(&"Perpendicular constraint requires 2 lines".into());
                    None
                } else {
                    let line1 = s.selected_entities[0];
                    let line2 = s.selected_entities[1];
                    Some(Constraint::Geometric(GeometricConstraint::Perpendicular {
                        line1,
                        line2,
                    }))
                }
            }
            _ => {
                web_sys::console::warn_1(&format!("Unknown constraint type: {}", constraint_type).into());
                None
            }
        };

        if let Some(c) = constraint {
            s.sketch_constraints.push(c);
            web_sys::console::log_1(&format!("Applied {} constraint", constraint_type).into());
        }

        drop(s);
        render()?;
        Ok(())
    })
}

/// Solve all constraints on the current sketch
fn solve_current_sketch() -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        if s.sketch_constraints.is_empty() {
            web_sys::console::log_1(&"No constraints to solve".into());
            return Ok(());
        }

        // Clone constraints before getting mutable sketch reference
        let constraints: Vec<Constraint> = s.sketch_constraints.clone();

        let sketch = s.current_sketch.as_mut()
            .ok_or_else(|| JsValue::from_str("No active sketch"))?;

        web_sys::console::log_1(&format!("Solving {} constraints...", constraints.len()).into());

        let solver = ConstraintSolver::default();
        let result = solver.solve(sketch, &constraints);

        if result.converged {
            web_sys::console::log_1(&format!(
                "Solver converged in {} iterations (error: {:.6})",
                result.iterations, result.final_error
            ).into());
        } else {
            web_sys::console::warn_1(&format!(
                "Solver did not converge after {} iterations (error: {:.6})",
                result.iterations, result.final_error
            ).into());
        }

        drop(s);
        render()?;
        Ok(())
    })
}

// ─────────────────────────────────────────────────────────────────────────────────
// RENDER AND SELECTION MODE FUNCTIONS
// ─────────────────────────────────────────────────────────────────────────────────

/// Set the render mode (wireframe, shaded, hidden)
fn set_render_mode(mode: &str) {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.render_mode = match mode {
            "wireframe" => RenderMode::Wireframe,
            "shaded" => RenderMode::Shaded,
            "hidden" => RenderMode::HiddenLine,
            _ => RenderMode::Wireframe,
        };
        web_sys::console::log_1(&format!("Render mode: {:?}", s.render_mode).into());
    });
    let _ = render();
}

/// Set the 3D selection mode (face, edge, vertex)
fn set_selection_mode(mode: &str) {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let new_mode = match mode {
            "face" => SelectionMode3D::Face,
            "edge" => SelectionMode3D::Edge,
            "vertex" => SelectionMode3D::Vertex,
            _ => SelectionMode3D::Face,
        };
        s.selection3d.set_mode(new_mode);
        web_sys::console::log_1(&format!("Selection mode: {:?}", new_mode).into());
    });
}

/// Set the sketch plane
fn set_sketch_plane(plane: &str) {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let new_plane = match plane {
            "xy" => SketchPlane::XY,
            "xz" => SketchPlane::XZ,
            "yz" => SketchPlane::YZ,
            _ => SketchPlane::XY,
        };

        // If we have an active sketch, update its plane
        if let Some(ref mut sketch) = s.current_sketch {
            sketch.plane = new_plane.clone();
            web_sys::console::log_1(&format!("Sketch plane changed to: {:?}", new_plane).into());
        } else {
            // If no sketch, create one with the new plane
            s.current_sketch = Some(Sketch::new(new_plane.clone()));
            web_sys::console::log_1(&format!("New sketch on plane: {:?}", new_plane).into());
        }
    });
    let _ = render();
}

/// Start a sketch on the selected face
fn sketch_on_selected_face() -> Result<(), JsValue> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();

        // Check if we have a selected face
        let face_id = s.selection3d.primary_face()
            .ok_or_else(|| JsValue::from_str("No face selected. Select a face first."))?;

        // Get the solid
        let solid = s.solid.as_ref()
            .ok_or_else(|| JsValue::from_str("No solid available"))?;

        // Find the face in the solid
        let face = solid.face(face_id)
            .ok_or_else(|| JsValue::from_str("Face not found in solid"))?;

        // Create coordinate frame from face
        let frame = SketchCoordinateFrame::from_face(face, solid)
            .ok_or_else(|| JsValue::from_str("Could not create coordinate frame from face"))?;

        // Create new sketch on the arbitrary plane
        let sketch = Sketch::new(SketchPlane::Arbitrary(frame));
        s.current_sketch = Some(sketch);

        // Switch to sketch mode
        s.mode = AppMode::Sketch2D;
        s.sketch_tool = SketchTool::Line;
        s.temp_points.clear();
        s.sketch_constraints.clear();
        s.selected_entities.clear();

        web_sys::console::log_1(&format!("Starting sketch on face {:?}", face_id).into());
        show_status("Sketching on face", StatusType::Success);

        drop(s);
        render()?;
        Ok(())
    })
}
