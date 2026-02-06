// AutoCrate - ASTM Standard Shipping Crate Generator
// Rust/WASM port of the original TypeScript application
#![allow(unexpected_cfgs)]

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod assembly;
pub mod calculator;
pub mod constants;
pub mod export;
pub mod generator;
pub mod geometry;
pub mod manufacturing;
pub mod render;
pub mod step_converter;

pub use assembly::{ComponentType, CrateAssembly};
pub use constants::LumberSize;
pub use generator::generate_crate;
pub use geometry::*;
pub use render::{Camera, Canvas2DRenderer, ProjectionType, ViewMode, WebGLRenderer};

/// Product dimensions input (in inches)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
    pub weight: f32,
}

impl Default for ProductDimensions {
    fn default() -> Self {
        Self {
            length: 120.0,
            width: 120.0,
            height: 120.0,
            weight: 10000.0,
        }
    }
}

/// Clearances around product (in inches)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clearances {
    pub side: f32,
    pub end: f32,
    pub top: f32,
}

/// Crate construction style per ASTM D6039
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrateStyle {
    /// Style A - Open frame crate (cleated frame only, no sheathing)
    /// Heavy-duty, suitable for severe handling
    A,
    /// Style B - Sheathed/covered crate (cleated frame with plywood panels)
    /// Light-duty, suitable for light-to-moderate handling
    B,
}

impl Default for Clearances {
    fn default() -> Self {
        Self {
            side: 2.0,
            end: 2.0,
            top: 3.0,
        }
    }
}

/// Complete crate specification
#[derive(Clone, Debug)]
pub struct CrateSpec {
    pub product: ProductDimensions,
    pub clearances: Clearances,
    pub style: CrateStyle,
    pub skid_count: u8,
    pub skid_size: LumberSize,
    pub floorboard_size: LumberSize,
    pub cleat_size: LumberSize,
}

impl Default for CrateSpec {
    fn default() -> Self {
        Self {
            product: ProductDimensions::default(),
            clearances: Clearances::default(),
            style: CrateStyle::B,
            skid_count: 3,
            skid_size: LumberSize::L4x4,
            floorboard_size: LumberSize::L2x6,
            cleat_size: LumberSize::L1x4,
        }
    }
}

impl Default for CrateStyle {
    fn default() -> Self {
        CrateStyle::B
    }
}

/// Generated crate geometry
#[derive(Clone, Debug)]
pub struct CrateGeometry {
    pub overall_length: f32,
    pub overall_width: f32,
    pub overall_height: f32,
    pub base_height: f32,
    pub skids: Vec<SkidGeometry>,
    pub floorboards: Vec<BoardGeometry>,
    pub panels: PanelSet,
    pub cleats: Vec<CleatGeometry>,
}

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::{
    window, Blob, HtmlAnchorElement, HtmlCanvasElement, HtmlInputElement, HtmlSelectElement,
    MouseEvent, Url, WheelEvent,
};

fn download_file(filename: &str, content: &str) -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let body = document.body().ok_or("no body")?;

    let parts = js_sys::Array::of1(&JsValue::from_str(content));
    let mut properties = web_sys::BlobPropertyBag::new();
    properties.set_type("text/plain");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &properties)?;

    let url = Url::create_object_url_with_blob(&blob)?;
    let a = document
        .create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;
    a.set_href(&url);
    a.set_download(filename);
    a.style().set_property("display", "none")?;

    body.append_child(&a)?;
    a.click();
    body.remove_child(&a)?;
    Url::revoke_object_url(&url)?;

    Ok(())
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"AutoCrate 3D Visualization Demo".into());

    // Get canvas element
    let document = window()
        .ok_or("No window")?
        .document()
        .ok_or("No document")?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("No canvas element")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Set canvas size
    let width = window().unwrap().inner_width()?.as_f64().unwrap_or(800.0) as u32;
    let height = window().unwrap().inner_height()?.as_f64().unwrap_or(600.0) as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    // Initialize WebGL renderer
    let mut renderer =
        render::WebGLRenderer::new(canvas.clone()).map_err(|e| JsValue::from_str(&e))?;

    renderer.init_shaders().map_err(|e| JsValue::from_str(&e))?;

    web_sys::console::log_1(&"WebGL renderer initialized!".into());

    // Set up camera (looking at a crate)
    {
        let camera = renderer.camera_mut();
        camera.distance = 150.0;
        camera.azimuth = std::f32::consts::PI / 4.0; // 45 degrees
        camera.elevation = std::f32::consts::PI / 6.0; // 30 degrees
        camera.target = glam::Vec3::new(0.0, 0.0, 20.0);
    }

    // UI Elements
    let input_length = document
        .get_element_by_id("length")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;
    let input_width = document
        .get_element_by_id("width")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;
    let input_height = document
        .get_element_by_id("height")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;
    let input_weight = document
        .get_element_by_id("weight")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;
    let input_style = document
        .get_element_by_id("style")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()?;
    let btn_generate = document.get_element_by_id("generate").unwrap();
    let btn_export = document.get_element_by_id("export-step").unwrap();
    let btn_export_cut_list = document.get_element_by_id("export-cut-list").unwrap();
    let btn_export_bom = document.get_element_by_id("export-bom").unwrap();
    let btn_export_nailing = document.get_element_by_id("export-nailing").unwrap();
    let btn_export_json = document.get_element_by_id("export-json").unwrap();
    let btn_export_gcode = document.get_element_by_id("export-gcode").unwrap();
    let btn_view3d = document.get_element_by_id("view-3d").unwrap();
    let btn_view2d = document.get_element_by_id("view-2d").unwrap();

    // Shared State
    let assembly = Rc::new(RefCell::new(CrateAssembly::default()));
    let mesh_buffers_rc = Rc::new(RefCell::new(Vec::new()));
    let colors_rc = Rc::new(RefCell::new(Vec::new()));
    let manufacturing_data = Rc::new(RefCell::new(None::<manufacturing::ManufacturingData>));
    let spec_rc = Rc::new(RefCell::new(CrateSpec::default()));

    // Renderer must be wrapped to be shared among event listeners
    let renderer = Rc::new(RefCell::new(renderer));

    // Function to regenerate crate and update render buffers
    let update_scene = {
        let assembly = assembly.clone();
        let mesh_buffers_rc = mesh_buffers_rc.clone();
        let colors_rc = colors_rc.clone();
        let renderer = renderer.clone();
        let manufacturing_data = manufacturing_data.clone();
        let spec_rc = spec_rc.clone();
        let input_length = input_length.clone();
        let input_width = input_width.clone();
        let input_height = input_height.clone();
        let input_weight = input_weight.clone();
        let input_style = input_style.clone();

        Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"Generating crate...".into());

            // Parse inputs
            let length = input_length.value_as_number() as f32;
            let width = input_width.value_as_number() as f32;
            let height = input_height.value_as_number() as f32;
            let weight = input_weight.value_as_number() as f32;
            let style = match input_style.value().as_str() {
                "A" => CrateStyle::A,
                _ => CrateStyle::B,
            };

            // Update spec
            let mut spec = CrateSpec::default();
            spec.product.length = length;
            spec.product.width = width;
            spec.product.height = height;
            spec.product.weight = weight;
            spec.style = style;

            // Generate assembly
            let new_assembly = generator::generate_crate(&spec, style);
            *assembly.borrow_mut() = new_assembly;

            // Generate manufacturing data
            let mfg_data = manufacturing::generate_manufacturing_data(&assembly.borrow(), weight);
            *manufacturing_data.borrow_mut() = Some(mfg_data);
            *spec_rc.borrow_mut() = spec.clone();

            // Update camera orthographic size based on dimensions
            let max_dim = length.max(width).max(height);
            renderer.borrow_mut().camera_mut().orthographic_size = max_dim * 1.5;

            // Update WebGL buffers
            let gl = renderer.borrow().gl.clone();
            let result = process_assembly_for_rendering(&gl, &assembly.borrow());

            if let Ok((new_bufs, new_colors)) = result {
                *mesh_buffers_rc.borrow_mut() = new_bufs;
                *colors_rc.borrow_mut() = new_colors;

                // Render immediately
                let r = renderer.borrow();
                r.begin_frame();
                for (i, buf) in mesh_buffers_rc.borrow().iter().enumerate() {
                    r.draw_mesh(buf, colors_rc.borrow()[i]);
                }
                r.end_frame();

                web_sys::console::log_1(&"Crate generated and rendered.".into());
            } else {
                web_sys::console::error_1(&"Failed to process assembly for rendering".into());
            }
        }) as Box<dyn FnMut()>)
    };

    // Initial generation
    update_scene
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .call0(&JsValue::NULL)
        .unwrap();

    // Bind Generate Button
    btn_generate
        .add_event_listener_with_callback("click", update_scene.as_ref().unchecked_ref())?;
    update_scene.forget(); // Keep alive

    // Bind Export STEP Button
    {
        let assembly = assembly.clone();
        let closure = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"Exporting STEP file...".into());
            let step_writer = step_converter::convert_assembly_to_step(&assembly.borrow());
            let step_content = step_writer.to_string();
            let _ = download_file("crate.step", &step_content);
        }) as Box<dyn FnMut()>);
        btn_export.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Export Cut List Button
    {
        let manufacturing_data = manufacturing_data.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(mfg) = manufacturing_data.borrow().as_ref() {
                let csv = export::csv::export_cut_list_csv(&mfg.cut_list);
                let _ = download_file("autocrate_cut_list.csv", &csv);
                web_sys::console::log_1(&"Cut list exported".into());
            } else {
                web_sys::console::warn_1(&"No manufacturing data - generate crate first".into());
            }
        }) as Box<dyn FnMut()>);
        btn_export_cut_list
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Export BOM Button
    {
        let manufacturing_data = manufacturing_data.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(mfg) = manufacturing_data.borrow().as_ref() {
                let csv = export::csv::export_bom_csv(&mfg.bom);
                let _ = download_file("autocrate_bom.csv", &csv);
                web_sys::console::log_1(&"BOM exported".into());
            } else {
                web_sys::console::warn_1(&"No manufacturing data - generate crate first".into());
            }
        }) as Box<dyn FnMut()>);
        btn_export_bom
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Export Nailing Coordinates Button
    {
        let manufacturing_data = manufacturing_data.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(mfg) = manufacturing_data.borrow().as_ref() {
                let csv = export::csv::export_nailing_csv(&mfg.nailing_coords);
                let _ = download_file("autocrate_nailing_coords.csv", &csv);
                web_sys::console::log_1(&"Nailing coordinates exported".into());
            } else {
                web_sys::console::warn_1(&"No manufacturing data - generate crate first".into());
            }
        }) as Box<dyn FnMut()>);
        btn_export_nailing
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Export JSON Button
    {
        let assembly = assembly.clone();
        let spec_rc = spec_rc.clone();
        let manufacturing_data = manufacturing_data.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(mfg) = manufacturing_data.borrow().as_ref() {
                let json =
                    export::json::export_assembly_json(&assembly.borrow(), &spec_rc.borrow(), mfg);
                let _ = download_file("autocrate_assembly.json", &json);
                web_sys::console::log_1(&"Assembly JSON exported".into());
            } else {
                web_sys::console::warn_1(&"No manufacturing data - generate crate first".into());
            }
        }) as Box<dyn FnMut()>);
        btn_export_json
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Export G-code Button
    {
        let manufacturing_data = manufacturing_data.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(mfg) = manufacturing_data.borrow().as_ref() {
                let gcode = export::gcode::export_cnc_gcode(
                    &mfg.cnc_program,
                    export::gcode::GcodeUnits::Imperial,
                );
                let _ = download_file("autocrate_cnc.gcode", &gcode);
                web_sys::console::log_1(&"CNC G-code exported".into());
            } else {
                web_sys::console::warn_1(&"No manufacturing data - generate crate first".into());
            }
        }) as Box<dyn FnMut()>);
        btn_export_gcode
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind View Toggle Buttons
    {
        let renderer = renderer.clone();
        let mesh_buffers_rc = mesh_buffers_rc.clone();
        let colors_rc = colors_rc.clone();
        let btn_view3d_clone = btn_view3d.clone();
        let btn_view2d_clone = btn_view2d.clone();

        let closure = Closure::wrap(Box::new(move || {
            let mut r = renderer.borrow_mut();
            r.camera_mut().projection_type = ProjectionType::Perspective;
            // Reset to perspective defaults
            r.camera_mut().azimuth = std::f32::consts::PI / 4.0;
            r.camera_mut().elevation = std::f32::consts::PI / 6.0;

            // Update UI classes
            let _ = btn_view3d_clone.class_list().add_1("active");
            let _ = btn_view2d_clone.class_list().remove_1("active");

            // Re-render
            r.begin_frame();
            let bufs = mesh_buffers_rc.borrow();
            let cols = colors_rc.borrow();
            for (i, buf) in bufs.iter().enumerate() {
                r.draw_mesh(buf, cols[i]);
            }
            r.end_frame();
        }) as Box<dyn FnMut()>);
        btn_view3d.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let renderer = renderer.clone();
        let mesh_buffers_rc = mesh_buffers_rc.clone();
        let colors_rc = colors_rc.clone();
        let btn_view3d_clone = btn_view3d.clone();
        let btn_view2d_clone = btn_view2d.clone();

        let closure = Closure::wrap(Box::new(move || {
            let mut r = renderer.borrow_mut();
            r.camera_mut().projection_type = ProjectionType::Orthographic;
            // Set to Top View
            r.camera_mut().azimuth = 0.0;
            r.camera_mut().elevation = std::f32::consts::PI / 2.0 - 0.01; // Almost 90 degrees (gimbal lock avoidance)
            r.camera_mut().target = glam::Vec3::ZERO;

            // Update UI classes
            let _ = btn_view2d_clone.class_list().add_1("active");
            let _ = btn_view3d_clone.class_list().remove_1("active");

            // Re-render
            r.begin_frame();
            let bufs = mesh_buffers_rc.borrow();
            let cols = colors_rc.borrow();
            for (i, buf) in bufs.iter().enumerate() {
                r.draw_mesh(buf, cols[i]);
            }
            r.end_frame();
        }) as Box<dyn FnMut()>);
        btn_view2d.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Bind Mouse Events (Orbit Controls)
    // Mouse down handler
    {
        let renderer = renderer.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut r = renderer.borrow_mut();
            r.is_dragging = true;
            r.last_mouse_x = event.client_x() as f32;
            r.last_mouse_y = event.client_y() as f32;
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse move handler (orbit camera)
    {
        let renderer = renderer.clone();
        let mesh_buffers_rc = mesh_buffers_rc.clone();
        let colors_rc = colors_rc.clone();

        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut r = renderer.borrow_mut();
            if r.is_dragging {
                let dx = event.client_x() as f32 - r.last_mouse_x;
                let dy = event.client_y() as f32 - r.last_mouse_y;

                if r.camera().projection_type == ProjectionType::Perspective {
                    // Orbit camera
                    let sensitivity = 0.005;
                    r.camera_mut().orbit(dx * sensitivity, -dy * sensitivity);
                } else {
                    // Pan camera in 2D
                    r.camera_mut().pan(-dx, dy);
                }

                r.last_mouse_x = event.client_x() as f32;
                r.last_mouse_y = event.client_y() as f32;

                // Re-render
                r.begin_frame();
                let bufs = mesh_buffers_rc.borrow();
                let cols = colors_rc.borrow();
                for (i, buf) in bufs.iter().enumerate() {
                    r.draw_mesh(buf, cols[i]);
                }
                r.end_frame();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse up handler
    {
        let renderer = renderer.clone();
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            renderer.borrow_mut().is_dragging = false;
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Wheel handler (zoom)
    {
        let renderer = renderer.clone();
        let mesh_buffers_rc = mesh_buffers_rc.clone();
        let colors_rc = colors_rc.clone();

        let closure = Closure::wrap(Box::new(move |event: WheelEvent| {
            event.prevent_default();
            let mut r = renderer.borrow_mut();

            // Zoom camera
            let delta = event.delta_y() as f32 * 0.1;

            if r.camera().projection_type == ProjectionType::Perspective {
                r.camera_mut().zoom(delta);
            } else {
                // Adjust orthographic size
                r.camera_mut().orthographic_size =
                    (r.camera().orthographic_size + delta).max(10.0).min(1000.0);
            }

            // Re-render
            r.begin_frame();
            let bufs = mesh_buffers_rc.borrow();
            let cols = colors_rc.borrow();
            for (i, buf) in bufs.iter().enumerate() {
                r.draw_mesh(buf, cols[i]);
            }
            r.end_frame();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    web_sys::console::log_1(&"Interactive controls enabled! Drag to orbit, scroll to zoom".into());

    Ok(())
}

fn process_assembly_for_rendering(
    gl: &web_sys::WebGl2RenderingContext,
    assembly: &CrateAssembly,
) -> Result<(Vec<render::MeshBuffer>, Vec<glam::Vec3>), JsValue> {
    let mut mesh_buffers = Vec::new();
    let mut colors = Vec::new();

    for node in &assembly.nodes {
        if node.id == assembly.root_id {
            continue; // Skip root node, it's just a container
        }

        // Create mesh in local space
        let mesh = render::Mesh::create_box(node.bounds.min.to_vec3(), node.bounds.max.to_vec3());

        // Create transform matrix
        let transform = glam::Mat4::from_rotation_translation(
            node.transform.rotation,
            node.transform.translation.to_vec3(),
        );

        // Apply transform to mesh
        let transformed_mesh = mesh.transformed(&transform);

        let color = match &node.component_type {
            ComponentType::Skid { .. } => glam::Vec3::new(0.65, 0.45, 0.30), // Darker brown
            ComponentType::Floorboard { .. } => glam::Vec3::new(0.85, 0.75, 0.60), // Lighter tan
            ComponentType::Cleat { .. } => glam::Vec3::new(0.75, 0.60, 0.45), // Medium brown
            ComponentType::Panel { .. } => glam::Vec3::new(0.80, 0.70, 0.55), // Light wood tone
            ComponentType::Nail { .. } => glam::Vec3::new(0.60, 0.65, 0.70), // Galvanized steel
            _ => glam::Vec3::new(0.5, 0.5, 0.5),                             // Default for unknown
        };

        mesh_buffers.push(render::MeshBuffer::from_mesh(gl, &transformed_mesh)?);
        colors.push(color);
    }

    Ok((mesh_buffers, colors))
}
