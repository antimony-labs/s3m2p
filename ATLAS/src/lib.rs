// ═══════════════════════════════════════════════════════════════════════════════
// FILE: lib.rs | ATLAS/src/lib.rs
// PURPOSE: WASM entry point for ATLAS interactive vector maps with LOD
// MODIFIED: 2026-01-26
// ═══════════════════════════════════════════════════════════════════════════════

#![allow(unexpected_cfgs)]

use geo_engine::{
    format::geojson, projection::ViewportTransform, BoundingBox, Coord, FeatureCollection,
    FeatureId, Geometry, LineString, Polygon,
};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, WheelEvent};

// JavaScript console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = updateZoomDisplay)]
    fn update_zoom_display(zoom: f32);

    #[wasm_bindgen(js_name = updateCenterDisplay)]
    fn update_center_display(lon: f32, lat: f32);

    #[wasm_bindgen(js_name = updateFeatureCount)]
    fn update_feature_count(count: u32);

    #[wasm_bindgen(js_name = updateCursorDisplay)]
    fn update_cursor_display(lon: f32, lat: f32);

    #[wasm_bindgen(js_name = showFeatureInfo)]
    fn show_feature_info(name: &str, iso: &str, population: u64);

    #[wasm_bindgen(js_name = hideFeatureInfo)]
    fn hide_feature_info();

    #[wasm_bindgen(js_name = hideLoading)]
    fn hide_loading();

    #[wasm_bindgen(js_name = setLoadingText)]
    fn set_loading_text(text: &str);
}

/// LOD level definitions
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LodLevel {
    Global,   // zoom 0-3: countries only (110m)
    Regional, // zoom 3-5: countries (50m) + major cities
    Country,  // zoom 5-7: countries (10m) + states + cities
    Detail,   // zoom 7+: states + all cities + rivers + lakes
}

impl LodLevel {
    fn from_zoom(zoom: f32) -> Self {
        if zoom < 3.0 {
            LodLevel::Global
        } else if zoom < 5.0 {
            LodLevel::Regional
        } else if zoom < 7.0 {
            LodLevel::Country
        } else {
            LodLevel::Detail
        }
    }
}

/// Map layer with its data and styling
#[allow(dead_code)]
struct MapLayer {
    name: String,
    data: FeatureCollection,
    visible: bool,
    loaded: bool,
    min_zoom: f32,
    max_zoom: f32,
}

impl MapLayer {
    fn new(name: &str, min_zoom: f32, max_zoom: f32) -> Self {
        Self {
            name: name.to_string(),
            data: FeatureCollection::new(),
            visible: true,
            loaded: false,
            min_zoom,
            max_zoom,
        }
    }

    fn should_render(&self, zoom: f32) -> bool {
        self.visible && self.loaded && zoom >= self.min_zoom && zoom <= self.max_zoom
    }
}

/// Application state
#[allow(dead_code)]
struct AppState {
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    viewport: ViewportTransform,

    // Multi-resolution country data
    countries_110m: FeatureCollection,
    countries_50m: FeatureCollection,
    countries_10m: FeatureCollection,

    // Additional layers
    states: FeatureCollection,
    cities: FeatureCollection,
    rivers: FeatureCollection,
    lakes: FeatureCollection,

    // Loading state
    countries_110m_loaded: bool,
    countries_50m_loaded: bool,
    countries_10m_loaded: bool,
    states_loaded: bool,
    cities_loaded: bool,
    rivers_loaded: bool,
    lakes_loaded: bool,

    // UI state
    hovered_feature: Option<FeatureId>,
    is_dragging: bool,
    last_mouse: Coord,
    current_lod: LodLevel,

    // Layer visibility
    show_countries: bool,
    show_states: bool,
    show_cities: bool,
    show_rivers: bool,
    show_lakes: bool,
    show_labels: bool,
    show_borders: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            canvas: None,
            ctx: None,
            viewport: ViewportTransform::new(800.0, 600.0),

            countries_110m: FeatureCollection::new(),
            countries_50m: FeatureCollection::new(),
            countries_10m: FeatureCollection::new(),
            states: FeatureCollection::new(),
            cities: FeatureCollection::new(),
            rivers: FeatureCollection::new(),
            lakes: FeatureCollection::new(),

            countries_110m_loaded: false,
            countries_50m_loaded: false,
            countries_10m_loaded: false,
            states_loaded: false,
            cities_loaded: false,
            rivers_loaded: false,
            lakes_loaded: false,

            hovered_feature: None,
            is_dragging: false,
            last_mouse: Coord::new(0.0, 0.0),
            current_lod: LodLevel::Global,

            show_countries: true,
            show_states: true,
            show_cities: true,
            show_rivers: true,
            show_lakes: true,
            show_labels: true,
            show_borders: true,
        }
    }
}

thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

/// Initialize the map application
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    log("ATLAS: Initializing with LOD system...");

    wasm_bindgen_futures::spawn_local(async {
        init_app().await;
    });
}

async fn init_app() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    let canvas: HtmlCanvasElement = document
        .get_element_by_id("map-canvas")
        .expect("no canvas")
        .dyn_into()
        .expect("not a canvas");

    // Set canvas size to match container
    let container = canvas.parent_element().expect("no parent");
    let width = container.client_width() as u32;
    let height = container.client_height() as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("get_context failed")
        .expect("no 2d context")
        .dyn_into()
        .expect("not a 2d context");

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.viewport = ViewportTransform::new(width as f32, height as f32);
        s.viewport.set_center(0.0, 20.0);
        s.viewport.zoom = 2.0;
        s.canvas = Some(canvas.clone());
        s.ctx = Some(ctx);
    });

    setup_event_listeners(&canvas);

    // Load data in priority order - start with low-res for fast initial display
    load_initial_data().await;

    // Initial render
    render();
    update_ui();
    hide_loading();

    // Load remaining data in background
    wasm_bindgen_futures::spawn_local(async {
        load_additional_data().await;
    });

    log("ATLAS: Ready");
}

async fn load_initial_data() {
    // Load 110m countries first for fast initial render
    set_loading_text("Loading world map...");
    if let Ok(data) = fetch_layer("countries", "110m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.countries_110m = data;
            state.countries_110m_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} countries (110m)", count));
    }
}

async fn load_additional_data() {
    // Load 50m countries
    if let Ok(data) = fetch_layer("countries", "50m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.countries_50m = data;
            state.countries_50m_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} countries (50m)", count));
        render();
    }

    // Load cities (important for labels)
    if let Ok(data) = fetch_layer("places", "10m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.cities = data;
            state.cities_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} cities", count));
        render();
    }

    // Load 10m countries
    if let Ok(data) = fetch_layer("countries", "10m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.countries_10m = data;
            state.countries_10m_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} countries (10m)", count));
        render();
    }

    // Load states/provinces
    if let Ok(data) = fetch_layer("states", "10m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.states = data;
            state.states_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} states/provinces", count));
        render();
    }

    // Load lakes
    if let Ok(data) = fetch_layer("lakes", "10m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.lakes = data;
            state.lakes_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} lakes", count));
        render();
    }

    // Load rivers
    if let Ok(data) = fetch_layer("rivers", "10m").await {
        let count = data.len();
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.rivers = data;
            state.rivers_loaded = true;
        });
        log(&format!("ATLAS: Loaded {} rivers", count));
        render();
    }

    log("ATLAS: All data loaded");
}

/// Fetch a map layer from storage server or local assets
async fn fetch_layer(layer: &str, resolution: &str) -> Result<FeatureCollection, String> {
    let window = web_sys::window().ok_or("no window")?;
    let location = window.location();
    let hostname = location.hostname().unwrap_or_default();

    // Build URL based on environment
    let url = if hostname == "localhost" || hostname == "127.0.0.1" {
        // Development: use local assets
        format!("assets/{}_{}.geojson", layer, resolution)
    } else {
        // Production: use storage server
        format!("http://144.126.145.3/v1/atlas/{}/{}", layer, resolution)
    };

    log(&format!("ATLAS: Fetching {}", url));
    fetch_geojson(&url).await
}

async fn fetch_geojson(url: &str) -> Result<FeatureCollection, String> {
    let window = web_sys::window().ok_or("no window")?;

    let resp_value = JsFuture::from(window.fetch_with_str(url))
        .await
        .map_err(|e| format!("fetch failed: {:?}", e))?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "response cast failed")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text = JsFuture::from(resp.text().map_err(|_| "text() failed")?)
        .await
        .map_err(|e| format!("text await failed: {:?}", e))?;

    let json_str = text.as_string().ok_or("not a string")?;

    geojson::parse_feature_collection(&json_str).map_err(|e| format!("parse failed: {}", e))
}

fn setup_event_listeners(canvas: &HtmlCanvasElement) {
    // Mouse down
    let mousedown_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        event.prevent_default();
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.is_dragging = true;
            s.last_mouse = Coord::new(event.offset_x() as f32, event.offset_y() as f32);
        });
    }) as Box<dyn FnMut(_)>);
    canvas.set_onmousedown(Some(mousedown_closure.as_ref().unchecked_ref()));
    mousedown_closure.forget();

    // Mouse move
    let mousemove_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        STATE.with(|state| {
            let mut s = state.borrow_mut();

            let world = s.viewport.screen_to_world(Coord::new(x, y));
            update_cursor_display(world.x, world.y);

            if s.is_dragging {
                let dx = x - s.last_mouse.x;
                let dy = y - s.last_mouse.y;
                s.viewport.pan_by(dx, dy);
                s.last_mouse = Coord::new(x, y);
                drop(s);
                render();
                update_ui();
            }
        });
    }) as Box<dyn FnMut(_)>);
    canvas.set_onmousemove(Some(mousemove_closure.as_ref().unchecked_ref()));
    mousemove_closure.forget();

    // Mouse up
    let mouseup_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.is_dragging = false;
        });
    }) as Box<dyn FnMut(_)>);
    canvas.set_onmouseup(Some(mouseup_closure.as_ref().unchecked_ref()));
    mouseup_closure.forget();

    // Mouse leave
    let mouseleave_closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.is_dragging = false;
        });
    }) as Box<dyn FnMut(_)>);
    canvas.set_onmouseleave(Some(mouseleave_closure.as_ref().unchecked_ref()));
    mouseleave_closure.forget();

    // Wheel
    let wheel_closure = Closure::wrap(Box::new(move |event: WheelEvent| {
        event.prevent_default();
        let delta = -event.delta_y() as f32 * 0.002;
        let x = event.offset_x() as f32;
        let y = event.offset_y() as f32;

        let old_lod = STATE.with(|s| s.borrow().current_lod);

        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.viewport.zoom_at(Coord::new(x, y), delta);
            s.current_lod = LodLevel::from_zoom(s.viewport.zoom);
        });

        let new_lod = STATE.with(|s| s.borrow().current_lod);
        if old_lod != new_lod {
            log(&format!("ATLAS: LOD changed to {:?}", new_lod));
        }

        render();
        update_ui();
    }) as Box<dyn FnMut(_)>);
    canvas.set_onwheel(Some(wheel_closure.as_ref().unchecked_ref()));
    wheel_closure.forget();
}

/// Render the map with LOD-based layer selection
fn render() {
    STATE.with(|state| {
        let s = state.borrow();
        let ctx = match &s.ctx {
            Some(ctx) => ctx,
            None => return,
        };
        let canvas = match &s.canvas {
            Some(c) => c,
            None => return,
        };

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let zoom = s.viewport.zoom;
        let lod = LodLevel::from_zoom(zoom);
        let visible = s.viewport.visible_bounds();

        // Clear with ocean color
        ctx.set_fill_style(&JsValue::from_str("#1a4a6a"));
        ctx.fill_rect(0.0, 0.0, width, height);

        // Render lakes first (under land)
        if s.show_lakes && s.lakes_loaded && zoom >= 4.0 {
            render_water_bodies(ctx, &s.lakes, &s.viewport, &visible);
        }

        // Render countries based on LOD
        if s.show_countries {
            let countries = match lod {
                LodLevel::Global => {
                    if s.countries_110m_loaded {
                        Some(&s.countries_110m)
                    } else {
                        None
                    }
                }
                LodLevel::Regional => {
                    if s.countries_50m_loaded {
                        Some(&s.countries_50m)
                    } else if s.countries_110m_loaded {
                        Some(&s.countries_110m)
                    } else {
                        None
                    }
                }
                LodLevel::Country | LodLevel::Detail => {
                    if s.countries_10m_loaded {
                        Some(&s.countries_10m)
                    } else if s.countries_50m_loaded {
                        Some(&s.countries_50m)
                    } else if s.countries_110m_loaded {
                        Some(&s.countries_110m)
                    } else {
                        None
                    }
                }
            };

            if let Some(data) = countries {
                render_countries(ctx, data, &s.viewport, &visible, s.show_borders);
            }
        }

        // Render states at higher zoom levels
        if s.show_states && s.states_loaded && zoom >= 4.0 {
            render_states(ctx, &s.states, &s.viewport, &visible, zoom);
        }

        // Render rivers
        if s.show_rivers && s.rivers_loaded && zoom >= 3.0 {
            render_rivers(ctx, &s.rivers, &s.viewport, &visible, zoom);
        }

        // Render city labels
        if s.show_labels && s.cities_loaded {
            render_city_labels(ctx, &s.cities, &s.viewport, &visible, zoom);
        }

        // Render country labels at low zoom
        if s.show_labels && zoom >= 2.0 && zoom < 6.0 {
            let countries = if s.countries_50m_loaded {
                Some(&s.countries_50m)
            } else if s.countries_110m_loaded {
                Some(&s.countries_110m)
            } else {
                None
            };
            if let Some(data) = countries {
                render_country_labels(ctx, data, &s.viewport, &visible, zoom);
            }
        }
    });
}

fn render_countries(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
    show_borders: bool,
) {
    for feature in data.query_bounds(visible) {
        let fill_color = "#2d4a3e";
        let stroke_color = "#1a3028";

        match &feature.geometry {
            Geometry::Polygon(poly) => {
                render_polygon(ctx, poly, viewport, fill_color, stroke_color, show_borders);
            }
            Geometry::MultiPolygon(mp) => {
                for poly in &mp.polygons {
                    render_polygon(ctx, poly, viewport, fill_color, stroke_color, show_borders);
                }
            }
            _ => {}
        }
    }
}

fn render_states(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
    zoom: f32,
) {
    // Only draw state borders, not fill
    let stroke_color = if zoom >= 6.0 {
        "rgba(100, 140, 120, 0.6)"
    } else {
        "rgba(100, 140, 120, 0.3)"
    };

    ctx.set_stroke_style(&JsValue::from_str(stroke_color));
    ctx.set_line_width(if zoom >= 6.0 { 1.0 } else { 0.5 });

    for feature in data.query_bounds(visible) {
        match &feature.geometry {
            Geometry::Polygon(poly) => {
                render_polygon_outline(ctx, poly, viewport);
            }
            Geometry::MultiPolygon(mp) => {
                for poly in &mp.polygons {
                    render_polygon_outline(ctx, poly, viewport);
                }
            }
            _ => {}
        }
    }
}

fn render_water_bodies(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
) {
    let fill_color = "#1a4a6a"; // Same as ocean
    let stroke_color = "#2a5a7a";

    for feature in data.query_bounds(visible) {
        match &feature.geometry {
            Geometry::Polygon(poly) => {
                render_polygon(ctx, poly, viewport, fill_color, stroke_color, true);
            }
            Geometry::MultiPolygon(mp) => {
                for poly in &mp.polygons {
                    render_polygon(ctx, poly, viewport, fill_color, stroke_color, true);
                }
            }
            _ => {}
        }
    }
}

fn render_rivers(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
    zoom: f32,
) {
    let stroke_color = "#3a6a8a";
    let line_width = if zoom >= 6.0 { 1.5 } else { 0.8 };

    ctx.set_stroke_style(&JsValue::from_str(stroke_color));
    ctx.set_line_width(line_width);

    for feature in data.query_bounds(visible) {
        // Filter by scalerank if available
        if let Some(sr) = feature.properties.scalerank {
            let min_rank = if zoom < 4.0 {
                3
            } else if zoom < 6.0 {
                6
            } else {
                10
            };
            if sr > min_rank {
                continue;
            }
        }

        match &feature.geometry {
            Geometry::LineString(ls) => {
                render_linestring_simple(ctx, ls, viewport);
            }
            Geometry::MultiPolygon(mp) => {
                // Some rivers are polygons
                for poly in &mp.polygons {
                    render_polygon_outline(ctx, poly, viewport);
                }
            }
            _ => {}
        }
    }
}

fn render_city_labels(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
    zoom: f32,
) {
    // Filter cities by importance based on zoom
    let min_scalerank = if zoom < 3.0 {
        1 // Only capitals/mega cities
    } else if zoom < 4.0 {
        2
    } else if zoom < 5.0 {
        4
    } else if zoom < 6.0 {
        6
    } else if zoom < 8.0 {
        8
    } else {
        10
    };

    let font_size = if zoom < 4.0 {
        10
    } else if zoom < 6.0 {
        11
    } else {
        12
    };

    ctx.set_font(&format!("{}px 'SF Mono', monospace", font_size));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    for feature in data.query_bounds(visible) {
        // Filter by scalerank
        if let Some(sr) = feature.properties.scalerank {
            if sr > min_scalerank {
                continue;
            }
        }

        if let Geometry::Point(coord) = &feature.geometry {
            let screen = viewport.world_to_screen(*coord);

            // Skip if off screen
            if screen.x < -50.0
                || screen.x > viewport.width + 50.0
                || screen.y < -20.0
                || screen.y > viewport.height + 20.0
            {
                continue;
            }

            let name = feature.name();
            if name.is_empty() || name == "Unknown" {
                continue;
            }

            // Draw dot
            ctx.begin_path();
            let _ = ctx.arc(
                screen.x as f64,
                screen.y as f64,
                2.5,
                0.0,
                std::f64::consts::TAU,
            );
            ctx.set_fill_style(&JsValue::from_str("#ff9966"));
            ctx.fill();

            // Draw text with shadow
            ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,0.7)"));
            let _ = ctx.fill_text(name, screen.x as f64 + 1.0, screen.y as f64 - 7.0);
            ctx.set_fill_style(&JsValue::from_str("#ffffff"));
            let _ = ctx.fill_text(name, screen.x as f64, screen.y as f64 - 8.0);
        }
    }
}

fn render_country_labels(
    ctx: &CanvasRenderingContext2d,
    data: &FeatureCollection,
    viewport: &ViewportTransform,
    visible: &BoundingBox,
    zoom: f32,
) {
    let font_size = if zoom < 3.0 { 9 } else { 11 };

    ctx.set_font(&format!("bold {}px 'SF Mono', monospace", font_size));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    for feature in data.query_bounds(visible) {
        let name = feature.name();
        if name.is_empty() || name == "Unknown" {
            continue;
        }

        // Use bounding box center for label position
        let center = feature.bounds.center();
        let screen = viewport.world_to_screen(center);

        // Skip if off screen
        if screen.x < 0.0
            || screen.x > viewport.width
            || screen.y < 0.0
            || screen.y > viewport.height
        {
            continue;
        }

        // Draw text with shadow
        ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,0.5)"));
        let _ = ctx.fill_text(name, screen.x as f64 + 1.0, screen.y as f64 + 1.0);
        ctx.set_fill_style(&JsValue::from_str("rgba(255,255,255,0.8)"));
        let _ = ctx.fill_text(name, screen.x as f64, screen.y as f64);
    }
}

fn render_polygon(
    ctx: &CanvasRenderingContext2d,
    poly: &Polygon,
    viewport: &ViewportTransform,
    fill_color: &str,
    stroke_color: &str,
    show_borders: bool,
) {
    if poly.exterior.coords.len() < 3 {
        return;
    }

    ctx.begin_path();

    let first = viewport.world_to_screen(poly.exterior.coords[0]);
    ctx.move_to(first.x as f64, first.y as f64);
    for coord in poly.exterior.coords.iter().skip(1) {
        let screen = viewport.world_to_screen(*coord);
        ctx.line_to(screen.x as f64, screen.y as f64);
    }
    ctx.close_path();

    for hole in &poly.holes {
        if hole.coords.len() >= 3 {
            let first = viewport.world_to_screen(hole.coords[0]);
            ctx.move_to(first.x as f64, first.y as f64);
            for coord in hole.coords.iter().skip(1) {
                let screen = viewport.world_to_screen(*coord);
                ctx.line_to(screen.x as f64, screen.y as f64);
            }
            ctx.close_path();
        }
    }

    ctx.set_fill_style(&JsValue::from_str(fill_color));
    ctx.fill();

    if show_borders {
        ctx.set_stroke_style(&JsValue::from_str(stroke_color));
        ctx.set_line_width(1.0);
        ctx.stroke();
    }
}

fn render_polygon_outline(ctx: &CanvasRenderingContext2d, poly: &Polygon, viewport: &ViewportTransform) {
    if poly.exterior.coords.len() < 3 {
        return;
    }

    ctx.begin_path();
    let first = viewport.world_to_screen(poly.exterior.coords[0]);
    ctx.move_to(first.x as f64, first.y as f64);
    for coord in poly.exterior.coords.iter().skip(1) {
        let screen = viewport.world_to_screen(*coord);
        ctx.line_to(screen.x as f64, screen.y as f64);
    }
    ctx.close_path();
    ctx.stroke();
}

fn render_linestring_simple(
    ctx: &CanvasRenderingContext2d,
    ls: &LineString,
    viewport: &ViewportTransform,
) {
    if ls.coords.len() < 2 {
        return;
    }

    ctx.begin_path();
    let first = viewport.world_to_screen(ls.coords[0]);
    ctx.move_to(first.x as f64, first.y as f64);
    for coord in ls.coords.iter().skip(1) {
        let screen = viewport.world_to_screen(*coord);
        ctx.line_to(screen.x as f64, screen.y as f64);
    }
    ctx.stroke();
}

fn update_ui() {
    STATE.with(|state| {
        let s = state.borrow();
        update_zoom_display(s.viewport.zoom);
        update_center_display(s.viewport.pan.x, s.viewport.pan.y);

        // Count visible features
        let count = if s.countries_10m_loaded {
            s.countries_10m.len()
        } else if s.countries_50m_loaded {
            s.countries_50m.len()
        } else {
            s.countries_110m.len()
        };
        update_feature_count(count as u32);
    });
}

// WASM exports for JavaScript

#[wasm_bindgen(js_name = atlasZoomIn)]
pub fn zoom_in() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let center = Coord::new(s.viewport.width / 2.0, s.viewport.height / 2.0);
        s.viewport.zoom_at(center, 0.5);
        s.current_lod = LodLevel::from_zoom(s.viewport.zoom);
    });
    render();
    update_ui();
}

#[wasm_bindgen(js_name = atlasZoomOut)]
pub fn zoom_out() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let center = Coord::new(s.viewport.width / 2.0, s.viewport.height / 2.0);
        s.viewport.zoom_at(center, -0.5);
        s.current_lod = LodLevel::from_zoom(s.viewport.zoom);
    });
    render();
    update_ui();
}

#[wasm_bindgen(js_name = atlasToggleLayer)]
pub fn toggle_layer(layer: &str, visible: bool) {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        match layer {
            "countries" => s.show_countries = visible,
            "states" => s.show_states = visible,
            "cities" => s.show_cities = visible,
            "rivers" => s.show_rivers = visible,
            "lakes" => s.show_lakes = visible,
            "borders" => s.show_borders = visible,
            "labels" => s.show_labels = visible,
            _ => {}
        }
    });
    render();
}

#[wasm_bindgen(js_name = atlasResetView)]
pub fn reset_view() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.viewport.set_center(0.0, 20.0);
        s.viewport.zoom = 2.0;
        s.current_lod = LodLevel::Global;
    });
    render();
    update_ui();
}
