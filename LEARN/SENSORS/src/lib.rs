//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | SENSORS/src/lib.rs
//! PURPOSE: Library crate root module with public API exports
//! MODIFIED: 2025-12-09
//! LAYER: LEARN → SENSORS
//! ═══════════════════════════════════════════════════════════════════════════════
#![allow(unexpected_cfgs)]
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, AnalyserNode, AudioContext, AudioContextOptions, CanvasRenderingContext2d,
    DeviceOrientationEvent, HtmlCanvasElement, HtmlVideoElement, ImageData, MediaStreamConstraints,
    MediaTrackConstraints,
};

const GRAPH_HISTORY_SIZE: usize = 200;

#[derive(Default)]
#[allow(dead_code)]
struct AccelData {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Default)]
#[allow(dead_code)]
struct GyroData {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Default)]
#[allow(dead_code)]
struct MagData {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Default)]
#[allow(dead_code)]
struct OrientData {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

struct VisualSLAM {
    position_x: f64,
    position_y: f64,
    position_z: f64,
    speed: f64,
    distance: f64,
    // Processing frame size (downsampled) and video size (for overlay scaling)
    frame_w: u32,
    frame_h: u32,
    video_w: u32,
    video_h: u32,
    last_keypoints: Vec<(f64, f64)>,
    last_descriptors: Vec<[u32; 8]>, // 256-bit BRIEF-like descriptor
    current_keypoints: Vec<(f64, f64)>,
    current_matches: Vec<((f64, f64), (f64, f64), u32)>, // (prev, curr, hamming)
    map_points: Vec<(f64, f64, f64)>,                    // x, y, z positions of landmarks
    trajectory: Vec<(f64, f64)>,                         // x,y in "map" frame
    tracking_quality: f64,                               // 0..1
    is_tracking: bool,
    initialized: bool,
}

impl Default for VisualSLAM {
    fn default() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            speed: 0.0,
            distance: 0.0,
            frame_w: 0,
            frame_h: 0,
            video_w: 0,
            video_h: 0,
            last_keypoints: Vec::new(),
            last_descriptors: Vec::new(),
            current_keypoints: Vec::new(),
            current_matches: Vec::new(),
            map_points: Vec::new(),
            trajectory: vec![(0.0, 0.0)],
            tracking_quality: 0.0,
            is_tracking: false,
            initialized: false,
        }
    }
}

struct SensorState {
    accel_history_x: Vec<f64>,
    accel_history_y: Vec<f64>,
    accel_history_z: Vec<f64>,
    gyro_history_x: Vec<f64>,
    gyro_history_y: Vec<f64>,
    gyro_history_z: Vec<f64>,
    current_accel: AccelData,
    current_gyro: GyroData,
    current_mag: MagData,
    current_orient: OrientData,
    ambient_light: f64,
    proximity: Option<f64>,
    sensors_available: bool,
    camera_available: bool,
    current_camera_facing: String, // "user" (front) or "environment" (back)
    current_stream: Option<web_sys::MediaStream>,
    visual_slam: VisualSLAM,
}

impl SensorState {
    fn new() -> Self {
        Self {
            accel_history_x: vec![0.0; GRAPH_HISTORY_SIZE],
            accel_history_y: vec![0.0; GRAPH_HISTORY_SIZE],
            accel_history_z: vec![0.0; GRAPH_HISTORY_SIZE],
            gyro_history_x: vec![0.0; GRAPH_HISTORY_SIZE],
            gyro_history_y: vec![0.0; GRAPH_HISTORY_SIZE],
            gyro_history_z: vec![0.0; GRAPH_HISTORY_SIZE],
            current_accel: AccelData::default(),
            current_gyro: GyroData::default(),
            current_mag: MagData::default(),
            current_orient: OrientData::default(),
            ambient_light: 0.0,
            proximity: None,
            sensors_available: false,
            camera_available: false,
            current_camera_facing: "environment".to_string(), // Start with back camera
            current_stream: None,
            visual_slam: VisualSLAM::default(),
        }
    }

    fn push_accel(&mut self, x: f64, y: f64, z: f64) {
        self.current_accel = AccelData { x, y, z };

        self.accel_history_x.remove(0);
        self.accel_history_x.push(x);

        self.accel_history_y.remove(0);
        self.accel_history_y.push(y);

        self.accel_history_z.remove(0);
        self.accel_history_z.push(z);
    }

    fn push_gyro(&mut self, x: f64, y: f64, z: f64) {
        self.current_gyro = GyroData { x, y, z };

        self.gyro_history_x.remove(0);
        self.gyro_history_x.push(x);

        self.gyro_history_y.remove(0);
        self.gyro_history_y.push(y);

        self.gyro_history_z.remove(0);
        self.gyro_history_z.push(z);
    }
}

thread_local! {
    static STATE: RefCell<SensorState> = RefCell::new(SensorState::new());
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Check for sensor support and set up
    check_sensor_support();

    // Check for camera support
    check_camera_support();

    // Set up camera button
    setup_camera_button();

    // Set up camera switch button
    setup_camera_switch_button();

    // Set up microphone button
    setup_microphone_button();

    // Set up sensor permission button (for iOS)
    setup_sensor_button();

    // Set up visual SLAM reset button
    setup_visual_slam_reset_button();

    // Start visual SLAM processing from camera
    start_visual_slam_loop();

    // Start render loop
    start_render_loop();

    // Log startup
    web_sys::console::log_1(&"Sensor test initialized".into());
}

fn check_sensor_support() {
    let window = window().expect("no window");

    // Check if DeviceMotionEvent exists
    let has_motion = js_sys::Reflect::get(&window, &"DeviceMotionEvent".into())
        .map(|v| !v.is_undefined())
        .unwrap_or(false);

    let has_orientation = js_sys::Reflect::get(&window, &"DeviceOrientationEvent".into())
        .map(|v| !v.is_undefined())
        .unwrap_or(false);

    if has_motion || has_orientation {
        // Check if we need permission (iOS 13+)
        let needs_permission = check_needs_motion_permission();

        if needs_permission {
            // Show permission button
            show_sensor_permission_button();
            update_sensor_status("waiting", "Permission Required");
        } else {
            // Try to listen directly
            setup_motion_listeners();
        }
    } else {
        update_sensor_status("unavailable", "Not Supported");
        STATE.with(|s| s.borrow_mut().sensors_available = false);
    }
}

fn check_needs_motion_permission() -> bool {
    let window = window().expect("no window");

    // Check if DeviceMotionEvent.requestPermission exists (iOS 13+)
    if let Ok(motion_event) = js_sys::Reflect::get(&window, &"DeviceMotionEvent".into()) {
        if let Ok(request_perm) = js_sys::Reflect::get(&motion_event, &"requestPermission".into()) {
            return request_perm.is_function();
        }
    }
    false
}

fn show_sensor_permission_button() {
    let document = window().unwrap().document().unwrap();
    if let Some(btn) = document.get_element_by_id("sensor-btn") {
        let _ = btn.set_attribute("style", "display: block;");
    }
}

fn setup_sensor_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("sensor-btn") {
        let closure = Closure::wrap(Box::new(move || {
            request_motion_permission();
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

fn request_motion_permission() {
    let window = window().expect("no window");

    // iOS 13+ permission request
    if let Ok(motion_event) = js_sys::Reflect::get(&window, &"DeviceMotionEvent".into()) {
        if let Ok(request_perm) = js_sys::Reflect::get(&motion_event, &"requestPermission".into()) {
            if request_perm.is_function() {
                let func: js_sys::Function = request_perm.unchecked_into();
                if let Ok(promise) = func.call0(&motion_event) {
                    let promise: js_sys::Promise = promise.unchecked_into();

                    let on_granted = Closure::wrap(Box::new(move |result: JsValue| {
                        let result_str = result.as_string().unwrap_or_default();
                        if result_str == "granted" {
                            setup_motion_listeners();
                            hide_sensor_permission_button();
                        } else {
                            update_sensor_status("error", "Permission Denied");
                        }
                    })
                        as Box<dyn FnMut(JsValue)>);

                    let on_error = Closure::wrap(Box::new(move |_: JsValue| {
                        update_sensor_status("error", "Permission Error");
                    }) as Box<dyn FnMut(JsValue)>);

                    let _ = promise.then2(&on_granted, &on_error);
                    on_granted.forget();
                    on_error.forget();
                }
            }
        }
    }
}

fn hide_sensor_permission_button() {
    let document = window().unwrap().document().unwrap();
    if let Some(btn) = document.get_element_by_id("sensor-btn") {
        let _ = btn.set_attribute("style", "display: none;");
    }
}

fn setup_motion_listeners() {
    let window = window().expect("no window");

    // Device Motion (accelerometer, gyroscope, magnetometer) - use JsValue since typed accessors aren't fully available
    let motion_closure = Closure::wrap(Box::new(move |event: JsValue| {
        // Access accelerationIncludingGravity via js_sys::Reflect
        if let Ok(accel) = js_sys::Reflect::get(&event, &"accelerationIncludingGravity".into()) {
            if !accel.is_null() && !accel.is_undefined() {
                let x = js_sys::Reflect::get(&accel, &"x".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let y = js_sys::Reflect::get(&accel, &"y".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let z = js_sys::Reflect::get(&accel, &"z".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                STATE.with(|s| {
                    let mut state = s.borrow_mut();
                    state.push_accel(x, y, z);
                    state.sensors_available = true;
                });

                update_accel_display(x, y, z);
            }
        }

        // Extract gyroscope data from rotationRate
        if let Ok(rotation_rate) = js_sys::Reflect::get(&event, &"rotationRate".into()) {
            if !rotation_rate.is_null() && !rotation_rate.is_undefined() {
                let x = js_sys::Reflect::get(&rotation_rate, &"alpha".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let y = js_sys::Reflect::get(&rotation_rate, &"beta".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let z = js_sys::Reflect::get(&rotation_rate, &"gamma".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                STATE.with(|s| {
                    let mut state = s.borrow_mut();
                    state.push_gyro(x, y, z);
                });

                update_gyro_display(x, y, z);
            }
        }

        // Extract magnetometer data
        if let Ok(magnetic_field) = js_sys::Reflect::get(&event, &"magneticField".into()) {
            if !magnetic_field.is_null() && !magnetic_field.is_undefined() {
                let x = js_sys::Reflect::get(&magnetic_field, &"x".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let y = js_sys::Reflect::get(&magnetic_field, &"y".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let z = js_sys::Reflect::get(&magnetic_field, &"z".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                STATE.with(|s| {
                    let mut state = s.borrow_mut();
                    state.current_mag = MagData { x, y, z };
                });

                update_magnetometer_display(x, y, z);
            }
        }
    }) as Box<dyn Fn(JsValue)>);

    let _ = window
        .add_event_listener_with_callback("devicemotion", motion_closure.as_ref().unchecked_ref());
    motion_closure.forget();

    // Try to set up Ambient Light Sensor (if available)
    setup_ambient_light_sensor();

    // Try to set up Proximity Sensor (if available)
    setup_proximity_sensor();

    // Device Orientation
    let orientation_closure = Closure::wrap(Box::new(move |event: DeviceOrientationEvent| {
        let alpha = event.alpha().unwrap_or(0.0);
        let beta = event.beta().unwrap_or(0.0);
        let gamma = event.gamma().unwrap_or(0.0);

        STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.current_orient = OrientData { alpha, beta, gamma };
        });

        update_orientation_display(alpha, beta, gamma);
    }) as Box<dyn Fn(DeviceOrientationEvent)>);

    let _ = window.add_event_listener_with_callback(
        "deviceorientation",
        orientation_closure.as_ref().unchecked_ref(),
    );
    orientation_closure.forget();

    update_sensor_status("available", "Active");
    STATE.with(|s| s.borrow_mut().sensors_available = true);
}

fn check_camera_support() {
    let window = window().expect("no window");
    let navigator = window.navigator();

    // Check if mediaDevices exists
    if navigator.media_devices().is_ok() {
        update_camera_status("waiting", "Ready");
        STATE.with(|s| s.borrow_mut().camera_available = true);
    } else {
        update_camera_status("unavailable", "Not Supported");
        update_camera_placeholder("Camera not supported on this device");
        STATE.with(|s| s.borrow_mut().camera_available = false);
    }
}

fn setup_camera_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("camera-btn") {
        let closure = Closure::wrap(Box::new(move || {
            wasm_bindgen_futures::spawn_local(async {
                request_camera().await;
            });
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

fn setup_microphone_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("mic-btn") {
        let closure = Closure::wrap(Box::new(move || {
            wasm_bindgen_futures::spawn_local(async {
                request_microphone().await;
            });
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

async fn request_camera() {
    request_camera_with_facing(None).await;
}

async fn request_camera_with_facing(facing_mode: Option<&str>) {
    let window = window().expect("no window");
    let navigator = window.navigator();

    // Stop existing stream if any
    STATE.with(|s| {
        let state = s.borrow();
        if let Some(ref stream) = state.current_stream {
            let tracks = stream.get_tracks();
            for i in 0..tracks.length() {
                let track_val = tracks.get(i);
                if let Ok(track) = track_val.dyn_into::<web_sys::MediaStreamTrack>() {
                    let _ = track.stop();
                }
            }
        }
    });

    // Determine facing mode
    let facing_str = if let Some(fm) = facing_mode {
        fm.to_string()
    } else {
        STATE.with(|s| s.borrow().current_camera_facing.clone())
    };

    if let Ok(media_devices) = navigator.media_devices() {
        let constraints = MediaStreamConstraints::new();
        constraints.set_audio(&JsValue::FALSE);

        // Create video constraints object with facing mode
        let video_constraints = MediaTrackConstraints::new();
        video_constraints.set_facing_mode(&JsValue::from_str(&facing_str));
        constraints.set_video(&JsValue::from(video_constraints));

        match media_devices.get_user_media_with_constraints(&constraints) {
            Ok(promise) => {
                match JsFuture::from(promise).await {
                    Ok(stream) => {
                        let stream: web_sys::MediaStream = stream.unchecked_into();

                        let document = window.document().unwrap();
                        if let Some(video) = document.get_element_by_id("camera-video") {
                            let video: HtmlVideoElement = video.unchecked_into();
                            video.set_src_object(Some(&stream));
                            let _ = video.play();
                        }

                        // Update state
                        STATE.with(|s| {
                            let mut state = s.borrow_mut();
                            state.current_stream = Some(stream.clone());
                            state.current_camera_facing = facing_str.clone();
                        });

                        // Hide placeholder via style
                        if let Some(placeholder) = document.get_element_by_id("camera-placeholder")
                        {
                            let _ = placeholder.set_attribute("style", "display: none;");
                        }

                        // Show switch button
                        if let Some(btn) = document.get_element_by_id("switch-camera-btn") {
                            let _ = btn.set_attribute("style", "display: block; margin-top: 8px;");
                        }

                        let camera_name = if facing_str == "user" { "Front" } else { "Back" };
                        update_camera_status("available", &format!("Streaming ({})", camera_name));
                    }
                    Err(_) => {
                        update_camera_status("error", "Permission Denied");
                        update_camera_placeholder("Camera permission denied");
                    }
                }
            }
            Err(_) => {
                update_camera_status("error", "Error");
                update_camera_placeholder("Failed to access camera");
            }
        }
    }
}

fn setup_camera_switch_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("switch-camera-btn") {
        let closure = Closure::wrap(Box::new(move || {
            wasm_bindgen_futures::spawn_local(async {
                switch_camera().await;
            });
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

async fn switch_camera() {
    let new_facing = STATE.with(|s| {
        let state = s.borrow();
        if state.current_camera_facing == "user" {
            "environment"
        } else {
            "user"
        }
    });

    request_camera_with_facing(Some(new_facing)).await;
}

async fn request_microphone() {
    let window = window().expect("no window");
    let navigator = window.navigator();

    if let Ok(media_devices) = navigator.media_devices() {
        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::FALSE);
        constraints.set_audio(&JsValue::TRUE);

        match media_devices.get_user_media_with_constraints(&constraints) {
            Ok(promise) => {
                match JsFuture::from(promise).await {
                    Ok(stream) => {
                        let stream: web_sys::MediaStream = stream.unchecked_into();

                        // Hide button
                        let document = window.document().unwrap();
                        if let Some(btn) = document.get_element_by_id("mic-btn") {
                            let _ = btn.set_attribute("style", "display: none;");
                        }

                        // Set up audio analysis
                        setup_audio_analysis(stream);

                        update_microphone_status("available", "Active");
                    }
                    Err(_) => {
                        update_microphone_status("error", "Permission Denied");
                    }
                }
            }
            Err(_) => {
                update_microphone_status("error", "Error");
            }
        }
    }
}

fn setup_audio_analysis(stream: web_sys::MediaStream) {
    // Create AudioContext
    let options = AudioContextOptions::new();
    if let Ok(audio_ctx) = AudioContext::new_with_context_options(&options) {
        // Create MediaStreamAudioSourceNode
        if let Ok(source) = audio_ctx.create_media_stream_source(&stream) {
            // Create AnalyserNode
            if let Ok(analyser) = audio_ctx.create_analyser() {
                analyser.set_fft_size(256);
                analyser.set_smoothing_time_constant(0.8);

                // Connect source to analyser
                let _ = source.connect_with_audio_node(&analyser);

                // Start analyzing
                start_audio_analysis_loop(analyser);
            }
        }
    }
}

fn start_audio_analysis_loop(analyser: AnalyserNode) {
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let buffer_length = analyser.frequency_bin_count();
        let mut data_array = vec![0u8; buffer_length as usize];

        analyser.get_byte_frequency_data(&mut data_array);

        // Calculate average volume
        let sum: u32 = data_array.iter().map(|&x| x as u32).sum();
        let average = sum / buffer_length;
        let percentage = (average as f64 / 255.0 * 100.0).min(100.0);

        update_microphone_display(percentage);

        // Schedule next analysis
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn update_microphone_display(percentage: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("mic-level") {
        el.set_text_content(Some(&format!("{:.0}%", percentage)));
    }

    if let Some(bar) = document.get_element_by_id("mic-bar") {
        let bar_el: web_sys::HtmlElement = bar.unchecked_into();
        let _ = bar_el.set_attribute("style", &format!("width: {:.1}%; background: linear-gradient(90deg, #1a1a2e 0%, #4ecdc4 50%, #00ffff 100%);", percentage));
    }
}

fn update_microphone_status(status: &str, text: &str) {
    let document = window().unwrap().document().unwrap();

    if let Some(dot) = document.get_element_by_id("mic-dot") {
        let _ = dot.set_attribute("class", &format!("status-dot {}", status));
    }
    if let Some(el) = document.get_element_by_id("mic-status") {
        el.set_text_content(Some(text));
        let _ = el.set_attribute("class", &format!("status-text {}", status));
    }
}

fn start_visual_slam_loop() {
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        process_visual_slam_frame();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn process_visual_slam_frame() {
    let document = window().unwrap().document().unwrap();

    // Get video element
    let video: HtmlVideoElement = match document.get_element_by_id("camera-video") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    // Check if video is ready
    if video.ready_state() < 2 {
        return; // Not ready
    }

    // Use hidden capture canvas (downsampled) to keep SLAM cheap on mobile
    let canvas: HtmlCanvasElement = match document.get_element_by_id("slam-capture") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    // Processing resolution (keeps CPU reasonable)
    let proc_w: u32 = 200;
    let proc_h: u32 = 150;
    canvas.set_width(proc_w);
    canvas.set_height(proc_h);

    let ctx: CanvasRenderingContext2d = match canvas.get_context("2d") {
        Ok(Some(ctx)) => ctx.unchecked_into(),
        _ => return,
    };

    // Draw video frame to canvas (scaled)
    let _ = ctx.draw_image_with_html_video_element_and_dw_and_dh(
        &video,
        0.0,
        0.0,
        proc_w as f64,
        proc_h as f64,
    );

    // Get image data
    let image_data =
        match ctx.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64) {
            Ok(data) => data,
            Err(_) => return,
        };

    // Process frame for visual SLAM
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.visual_slam.frame_w = proc_w;
        state.visual_slam.frame_h = proc_h;
        state.visual_slam.video_w = video.video_width();
        state.visual_slam.video_h = video.video_height();
        update_visual_slam(&mut state.visual_slam, &image_data);
    });

    update_visual_slam_display();
}

// === ORB-like Visual SLAM (browser-friendly) ===================================
// This is not full ORB-SLAM (no BA / loop-closure), but it uses:
// - FAST corners
// - BRIEF/ORB-style binary descriptors
// - Hamming-distance matching + ratio test
// - Robust motion estimate (median translation)

const ORB_MAX_KEYPOINTS: usize = 320;
const FAST_THRESHOLD: i32 = 20;
const BRIEF_RADIUS: i32 = 8;

thread_local! {
    static BRIEF_PATTERN: Vec<(i8,i8,i8,i8)> = make_brief_pattern();
}

fn make_brief_pattern() -> Vec<(i8, i8, i8, i8)> {
    // Deterministic pseudo-random pattern (256 pairs) within [-BRIEF_RADIUS, BRIEF_RADIUS]
    let mut out = Vec::with_capacity(256);
    let mut x: u32 = 0xC0FFEE42;
    for _ in 0..256 {
        // xorshift32
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let a = (x & 0xFF) as i16;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let b = (x & 0xFF) as i16;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let c = (x & 0xFF) as i16;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let d = (x & 0xFF) as i16;

        let r = BRIEF_RADIUS as i16;
        let dx1 = (a % (2 * r + 1)) - r;
        let dy1 = (b % (2 * r + 1)) - r;
        let dx2 = (c % (2 * r + 1)) - r;
        let dy2 = (d % (2 * r + 1)) - r;
        out.push((dx1 as i8, dy1 as i8, dx2 as i8, dy2 as i8));
    }
    out
}

fn image_to_grayscale(image_data: &ImageData) -> (Vec<u8>, usize, usize) {
    let w = image_data.width() as usize;
    let h = image_data.height() as usize;
    let data = image_data.data();
    let mut gray = vec![0u8; w * h];
    for i in 0..(w * h) {
        let idx = i * 4;
        if idx + 2 < data.len() {
            let r = data[idx] as u32;
            let g = data[idx + 1] as u32;
            let b = data[idx + 2] as u32;
            gray[i] = ((r + g + b) / 3) as u8;
        }
    }
    (gray, w, h)
}

fn fast_corners(gray: &[u8], w: usize, h: usize) -> Vec<(usize, usize, u16)> {
    // FAST-16 with simplified "count" test (no contiguous arc requirement)
    let circle: [(i32, i32); 16] = [
        (0, -3),
        (1, -3),
        (2, -2),
        (3, -1),
        (3, 0),
        (3, 1),
        (2, 2),
        (1, 3),
        (0, 3),
        (-1, 3),
        (-2, 2),
        (-3, 1),
        (-3, 0),
        (-3, -1),
        (-2, -2),
        (-1, -3),
    ];

    let border = 4usize;
    let mut corners = Vec::new();
    for y in border..(h - border) {
        for x in border..(w - border) {
            let c = gray[y * w + x] as i32;
            let t_hi = c + FAST_THRESHOLD;
            let t_lo = c - FAST_THRESHOLD;
            let mut brighter = 0;
            let mut darker = 0;
            let mut score: u16 = 0;
            for (dx, dy) in circle {
                let xx = (x as i32 + dx) as usize;
                let yy = (y as i32 + dy) as usize;
                let p = gray[yy * w + xx] as i32;
                if p > t_hi {
                    brighter += 1;
                    score = score.saturating_add((p - t_hi) as u16);
                } else if p < t_lo {
                    darker += 1;
                    score = score.saturating_add((t_lo - p) as u16);
                }
            }
            if brighter >= 12 || darker >= 12 {
                corners.push((x, y, score));
            }
        }
    }
    corners
}

fn brief_descriptor(gray: &[u8], w: usize, x: usize, y: usize) -> [u32; 8] {
    let mut desc = [0u32; 8];
    BRIEF_PATTERN.with(|pat| {
        for (i, (dx1, dy1, dx2, dy2)) in pat.iter().enumerate() {
            let xx1 = (x as i32 + *dx1 as i32) as usize;
            let yy1 = (y as i32 + *dy1 as i32) as usize;
            let xx2 = (x as i32 + *dx2 as i32) as usize;
            let yy2 = (y as i32 + *dy2 as i32) as usize;
            let p1 = gray[yy1 * w + xx1];
            let p2 = gray[yy2 * w + xx2];
            if p1 < p2 {
                let word = i / 32;
                let bit = i % 32;
                desc[word] |= 1u32 << bit;
            }
        }
    });
    desc
}

fn hamming(a: &[u32; 8], b: &[u32; 8]) -> u32 {
    let mut d = 0u32;
    for i in 0..8 {
        d += (a[i] ^ b[i]).count_ones();
    }
    d
}

fn orb_extract(image_data: &ImageData, max_kp: usize) -> (Vec<(f64, f64)>, Vec<[u32; 8]>) {
    let (gray, w, h) = image_to_grayscale(image_data);
    let mut corners = fast_corners(&gray, w, h);
    // sort by score descending
    corners.sort_by(|a, b| b.2.cmp(&a.2));

    let mut keypoints = Vec::new();
    let mut descriptors = Vec::new();

    let r = BRIEF_RADIUS as usize;
    for (x, y, _score) in corners.into_iter() {
        if x < r || y < r || x + r >= w || y + r >= h {
            continue;
        }
        let d = brief_descriptor(&gray, w, x, y);
        keypoints.push((x as f64, y as f64));
        descriptors.push(d);
        if keypoints.len() >= max_kp {
            break;
        }
    }

    (keypoints, descriptors)
}

fn orb_match(
    prev_kp: &[(f64, f64)],
    prev_desc: &[[u32; 8]],
    curr_kp: &[(f64, f64)],
    curr_desc: &[[u32; 8]],
) -> Vec<((f64, f64), (f64, f64), u32)> {
    let mut matches = Vec::new();
    if prev_desc.is_empty() || curr_desc.is_empty() {
        return matches;
    }

    // mild motion gate (in pixels of processing frame)
    let gate_px: f64 = 60.0;
    let max_hamming: u32 = 80;

    for (i, a) in prev_desc.iter().enumerate() {
        let (ax, ay) = prev_kp[i];
        let mut best = (u32::MAX, usize::MAX);
        let mut second = (u32::MAX, usize::MAX);

        for (j, b) in curr_desc.iter().enumerate() {
            let (bx, by) = curr_kp[j];
            if (bx - ax).abs() > gate_px || (by - ay).abs() > gate_px {
                continue;
            }
            let d = hamming(a, b);
            if d < best.0 {
                second = best;
                best = (d, j);
            } else if d < second.0 {
                second = (d, j);
            }
        }

        if best.1 != usize::MAX && best.0 <= max_hamming {
            // ratio test (best significantly better than second)
            if second.1 == usize::MAX || (best.0 as f64) <= (second.0 as f64 * 0.8) {
                matches.push((prev_kp[i], curr_kp[best.1], best.0));
            }
        }
    }

    matches
}

fn median(vals: &mut [f64]) -> f64 {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals[vals.len() / 2]
}

fn estimate_translation_median(matches: &[((f64, f64), (f64, f64), u32)]) -> Option<(f64, f64)> {
    if matches.len() < 12 {
        return None;
    }
    let mut dxs = Vec::with_capacity(matches.len());
    let mut dys = Vec::with_capacity(matches.len());
    for (a, b, d) in matches {
        if *d > 70 {
            continue;
        }
        dxs.push(b.0 - a.0);
        dys.push(b.1 - a.1);
    }
    if dxs.len() < 10 {
        return None;
    }
    let dx = median(&mut dxs);
    let dy = median(&mut dys);
    Some((dx, dy))
}

fn update_visual_slam(slam: &mut VisualSLAM, current_frame: &ImageData) {
    // Extract ORB-like keypoints + descriptors from current frame
    let (kps, descs) = orb_extract(current_frame, ORB_MAX_KEYPOINTS);
    slam.current_keypoints = kps.clone();
    slam.current_matches.clear();
    slam.tracking_quality = 0.0;
    slam.is_tracking = false;

    if !slam.initialized {
        slam.last_keypoints = kps;
        slam.last_descriptors = descs;
        slam.initialized = true;
        return;
    }

    let matches = orb_match(&slam.last_keypoints, &slam.last_descriptors, &kps, &descs);

    let kp_count = kps.len().max(1);
    slam.tracking_quality = (matches.len() as f64 / kp_count as f64).clamp(0.0, 1.0);
    slam.is_tracking = matches.len() >= 25;
    slam.current_matches = matches.iter().take(80).cloned().collect();

    if slam.is_tracking {
        if let Some((dx_pix, dy_pix)) = estimate_translation_median(&matches) {
            // Convert processing-frame pixel motion to "map units"
            // NOTE: scale is arbitrary; goal is stable visualization.
            let fw = slam.frame_w.max(1) as f64;
            let fh = slam.frame_h.max(1) as f64;
            let dx = -(dx_pix / fw) * 0.35;
            let dy = -(dy_pix / fh) * 0.35;
            let dz = 0.0;

            slam.position_x += dx;
            slam.position_y += dy;
            slam.position_z += dz;

            let step = (dx * dx + dy * dy + dz * dz).sqrt();
            slam.distance += step;
            slam.speed = step / (1.0 / 30.0);

            slam.trajectory.push((slam.position_x, slam.position_y));
            if slam.trajectory.len() > 600 {
                slam.trajectory.remove(0);
            }

            // Add a few "landmarks" around current pose (visual-only)
            if slam.map_points.len() < 2000 {
                for (idx, (x, y)) in kps.iter().enumerate().step_by(12) {
                    if idx % 24 != 0 {
                        continue;
                    }
                    let nx = (*x / fw) - 0.5;
                    let ny = (*y / fh) - 0.5;
                    slam.map_points.push((
                        slam.position_x + nx * 0.8,
                        slam.position_y + ny * 0.8,
                        1.0,
                    ));
                }
            }
        }
    }

    slam.last_keypoints = kps;
    slam.last_descriptors = descs;
}

fn update_visual_slam_display() {
    let document = window().unwrap().document().unwrap();

    STATE.with(|s| {
        let state = s.borrow();
        let slam = &state.visual_slam;

        // HUD: status + metrics
        let (status_text, status_class) = if !slam.initialized {
            ("Waiting", "hud-value warn")
        } else if slam.is_tracking {
            ("Tracking", "hud-value good")
        } else if slam.current_keypoints.len() > 40 {
            ("Searching", "hud-value warn")
        } else {
            ("Lost", "hud-value bad")
        };

        if let Some(el) = document.get_element_by_id("slam-status") {
            el.set_text_content(Some(status_text));
            let _ = el.set_attribute("class", status_class);
        }
        if let Some(el) = document.get_element_by_id("slam-keypoints") {
            el.set_text_content(Some(&format!("{}", slam.current_keypoints.len())));
        }
        if let Some(el) = document.get_element_by_id("slam-matches") {
            el.set_text_content(Some(&format!("{}", slam.current_matches.len())));
        }
        if let Some(el) = document.get_element_by_id("slam-quality") {
            let pct = (slam.tracking_quality * 100.0).round() as i32;
            el.set_text_content(Some(&format!("{}%", pct)));
            let cls = if slam.tracking_quality >= 0.25 {
                "hud-value good"
            } else if slam.tracking_quality >= 0.12 {
                "hud-value warn"
            } else {
                "hud-value bad"
            };
            let _ = el.set_attribute("class", cls);
        }
        if let Some(el) = document.get_element_by_id("slam-pos") {
            el.set_text_content(Some(&format!(
                "{:.2}, {:.2}",
                slam.position_x, slam.position_y
            )));
        }
        if let Some(el) = document.get_element_by_id("slam-distance") {
            el.set_text_content(Some(&format!("{:.2} m", slam.distance)));
        }
        if let Some(el) = document.get_element_by_id("slam-landmarks") {
            el.set_text_content(Some(&format!("{}", slam.map_points.len())));
        }
    });
}

fn setup_visual_slam_reset_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("reset-slam-btn") {
        let closure = Closure::wrap(Box::new(move || {
            reset_visual_slam();
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

fn reset_visual_slam() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.visual_slam = VisualSLAM::default();
    });

    update_visual_slam_display();
}

fn update_camera_placeholder(message: &str) {
    let document = window().unwrap().document().unwrap();
    if let Some(text) = document.get_element_by_id("camera-status-text") {
        text.set_text_content(Some(message));
    }
}

fn update_accel_display(x: f64, y: f64, z: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("accel-x") {
        el.set_text_content(Some(&format!("{:.2}", x)));
    }
    if let Some(el) = document.get_element_by_id("accel-y") {
        el.set_text_content(Some(&format!("{:.2}", y)));
    }
    if let Some(el) = document.get_element_by_id("accel-z") {
        el.set_text_content(Some(&format!("{:.2}", z)));
    }
}

fn update_orientation_display(alpha: f64, beta: f64, gamma: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("orient-alpha") {
        el.set_text_content(Some(&format!("{:.0}°", alpha)));
    }
    if let Some(el) = document.get_element_by_id("orient-beta") {
        el.set_text_content(Some(&format!("{:.0}°", beta)));
    }
    if let Some(el) = document.get_element_by_id("orient-gamma") {
        el.set_text_content(Some(&format!("{:.0}°", gamma)));
    }
}

fn update_gyro_display(x: f64, y: f64, z: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("gyro-x") {
        el.set_text_content(Some(&format!("{:.2}", x)));
    }
    if let Some(el) = document.get_element_by_id("gyro-y") {
        el.set_text_content(Some(&format!("{:.2}", y)));
    }
    if let Some(el) = document.get_element_by_id("gyro-z") {
        el.set_text_content(Some(&format!("{:.2}", z)));
    }
}

fn update_magnetometer_display(x: f64, y: f64, z: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("mag-x") {
        el.set_text_content(Some(&format!("{:.2}", x)));
    }
    if let Some(el) = document.get_element_by_id("mag-y") {
        el.set_text_content(Some(&format!("{:.2}", y)));
    }
    if let Some(el) = document.get_element_by_id("mag-z") {
        el.set_text_content(Some(&format!("{:.2}", z)));
    }

    // Calculate heading (compass direction)
    let heading = y.atan2(x) * 180.0 / std::f64::consts::PI;
    let heading_normalized = (heading + 360.0) % 360.0;

    if let Some(el) = document.get_element_by_id("mag-heading") {
        el.set_text_content(Some(&format!("{:.0}°", heading_normalized)));
    }
}

fn setup_ambient_light_sensor() {
    let window = window().expect("no window");

    // Check if AmbientLightSensor is available (Generic Sensor API)
    if let Ok(ambient_light_sensor) = js_sys::Reflect::get(&window, &"AmbientLightSensor".into()) {
        if !ambient_light_sensor.is_undefined() {
            // Try to create sensor instance
            if let Ok(sensor) = js_sys::Reflect::construct(
                ambient_light_sensor.unchecked_ref(),
                &js_sys::Array::new(),
            ) {
                let sensor_obj = sensor;

                // Set up reading event
                let reading_closure = Closure::wrap(Box::new(move |event: JsValue| {
                    if let Ok(illuminance) = js_sys::Reflect::get(&event, &"illuminance".into()) {
                        if let Some(lux) = illuminance.as_f64() {
                            STATE.with(|s| {
                                s.borrow_mut().ambient_light = lux;
                            });

                            update_ambient_light_display(lux);
                        }
                    }
                }) as Box<dyn Fn(JsValue)>);

                if let Ok(add_listener_val) =
                    js_sys::Reflect::get(&sensor_obj, &"addEventListener".into())
                {
                    if add_listener_val.is_function() {
                        let add_listener: js_sys::Function = add_listener_val.unchecked_into();
                        let _ = add_listener.call2(
                            &sensor_obj,
                            &JsValue::from_str("reading"),
                            reading_closure.as_ref().unchecked_ref(),
                        );
                        reading_closure.forget();

                        // Start the sensor
                        if let Ok(start_fn) = js_sys::Reflect::get(&sensor_obj, &"start".into()) {
                            if start_fn.is_function() {
                                let start: js_sys::Function = start_fn.unchecked_into();
                                let _ = start.call0(&sensor_obj);
                            }
                        }
                    } else {
                        reading_closure.forget();
                    }
                } else {
                    reading_closure.forget();
                }
            }
        }
    }
}

fn setup_proximity_sensor() {
    let window = window().expect("no window");

    // Check if ProximitySensor is available (Generic Sensor API)
    if let Ok(proximity_sensor) = js_sys::Reflect::get(&window, &"ProximitySensor".into()) {
        if !proximity_sensor.is_undefined() {
            // Try to create sensor instance
            if let Ok(sensor) =
                js_sys::Reflect::construct(proximity_sensor.unchecked_ref(), &js_sys::Array::new())
            {
                let sensor_obj = sensor;

                // Set up reading event
                let reading_closure = Closure::wrap(Box::new(move |event: JsValue| {
                    if let Ok(distance) = js_sys::Reflect::get(&event, &"distance".into()) {
                        if let Some(dist) = distance.as_f64() {
                            STATE.with(|s| {
                                s.borrow_mut().proximity = Some(dist);
                            });

                            update_proximity_display(Some(dist));
                        }
                    } else if let Ok(near) = js_sys::Reflect::get(&event, &"near".into()) {
                        if let Some(is_near) = near.as_bool() {
                            update_proximity_display(if is_near { Some(0.0) } else { Some(100.0) });
                        }
                    }
                }) as Box<dyn Fn(JsValue)>);

                if let Ok(add_listener_val) =
                    js_sys::Reflect::get(&sensor_obj, &"addEventListener".into())
                {
                    if add_listener_val.is_function() {
                        let add_listener: js_sys::Function = add_listener_val.unchecked_into();
                        let _ = add_listener.call2(
                            &sensor_obj,
                            &JsValue::from_str("reading"),
                            reading_closure.as_ref().unchecked_ref(),
                        );
                        reading_closure.forget();

                        // Start the sensor
                        if let Ok(start_fn) = js_sys::Reflect::get(&sensor_obj, &"start".into()) {
                            if start_fn.is_function() {
                                let start: js_sys::Function = start_fn.unchecked_into();
                                let _ = start.call0(&sensor_obj);
                            }
                        }
                    } else {
                        reading_closure.forget();
                    }
                } else {
                    reading_closure.forget();
                }
            }
        }
    }
}

fn update_ambient_light_display(lux: f64) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("light-value") {
        el.set_text_content(Some(&format!("{:.0}", lux)));
    }

    // Update light bar (normalize to 0-100% for display, assuming max ~1000 lux)
    let max_lux = 1000.0;
    let percentage = (lux / max_lux * 100.0).clamp(0.0, 100.0);

    if let Some(bar) = document.get_element_by_id("light-bar") {
        let bar_el: web_sys::HtmlElement = bar.unchecked_into();
        let _ = bar_el.set_attribute("style", &format!("width: {:.1}%", percentage));
    }
}

fn update_proximity_display(distance: Option<f64>) {
    let document = window().unwrap().document().unwrap();

    if let Some(el) = document.get_element_by_id("proximity-value") {
        match distance {
            Some(dist) => {
                el.set_text_content(Some(&format!("{:.1} cm", dist)));
            }
            None => {
                el.set_text_content(Some("N/A"));
            }
        }
    }

    // Update proximity indicator
    if let Some(indicator) = document.get_element_by_id("proximity-indicator") {
        let indicator_el: web_sys::HtmlElement = indicator.unchecked_into();
        match distance {
            Some(dist) if dist < 5.0 => {
                let _ = indicator_el.set_attribute("class", "proximity-indicator near");
            }
            Some(_) => {
                let _ = indicator_el.set_attribute("class", "proximity-indicator far");
            }
            None => {
                let _ = indicator_el.set_attribute("class", "proximity-indicator");
            }
        }
    }
}

fn update_sensor_status(status: &str, text: &str) {
    let document = window().unwrap().document().unwrap();

    if let Some(dot) = document.get_element_by_id("sensor-dot") {
        let _ = dot.set_attribute("class", &format!("status-dot {}", status));
    }
    if let Some(el) = document.get_element_by_id("sensor-status") {
        el.set_text_content(Some(text));
        let _ = el.set_attribute("class", &format!("status-text {}", status));
    }
}

fn update_camera_status(status: &str, text: &str) {
    let document = window().unwrap().document().unwrap();

    if let Some(dot) = document.get_element_by_id("camera-dot") {
        let _ = dot.set_attribute("class", &format!("status-dot {}", status));
    }
    if let Some(el) = document.get_element_by_id("camera-status") {
        el.set_text_content(Some(text));
        let _ = el.set_attribute("class", &format!("status-text {}", status));
    }
}

fn start_render_loop() {
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        render_accel_graph();
        render_gyro_graph();
        render_slam_overlay();
        render_slam_map();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn render_slam_overlay() {
    let document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = match document.get_element_by_id("slam-overlay") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;
    if width == 0 || height == 0 {
        return;
    }

    let dpr = window().unwrap().device_pixel_ratio();
    canvas.set_width((width as f64 * dpr) as u32);
    canvas.set_height((height as f64 * dpr) as u32);

    let ctx: CanvasRenderingContext2d = match canvas.get_context("2d") {
        Ok(Some(ctx)) => ctx.unchecked_into(),
        _ => return,
    };
    let _ = ctx.scale(dpr, dpr);

    let w = width as f64;
    let h = height as f64;
    ctx.clear_rect(0.0, 0.0, w, h);

    STATE.with(|s| {
        let state = s.borrow();
        let slam = &state.visual_slam;
        if !slam.initialized
            || slam.frame_w == 0
            || slam.frame_h == 0
            || slam.video_w == 0
            || slam.video_h == 0
        {
            return;
        }

        let vw = slam.video_w as f64;
        let vh = slam.video_h as f64;

        // object-fit: cover mapping
        let scale = (w / vw).max(h / vh);
        let offx = (w - vw * scale) / 2.0;
        let offy = (h - vh * scale) / 2.0;

        let fx = slam.frame_w as f64;
        let fy = slam.frame_h as f64;

        let map_pt = |px: f64, py: f64| -> (f64, f64) {
            let vx = px * vw / fx;
            let vy = py * vh / fy;
            (vx * scale + offx, vy * scale + offy)
        };

        // Draw match vectors
        ctx.set_line_width(1.0);
        for (a, b, ham) in slam.current_matches.iter().take(40) {
            let (x1, y1) = map_pt(a.0, a.1);
            let (x2, y2) = map_pt(b.0, b.1);
            let alpha = (1.0 - (*ham as f64 / 90.0)).clamp(0.15, 0.9);
            ctx.set_stroke_style(&JsValue::from_str(&format!(
                "rgba(78, 205, 196, {:.3})",
                alpha
            )));
            ctx.begin_path();
            ctx.move_to(x1, y1);
            ctx.line_to(x2, y2);
            ctx.stroke();
        }

        // Draw keypoints
        let kp_color = if slam.is_tracking {
            "rgba(0, 255, 255, 0.85)"
        } else {
            "rgba(255, 230, 109, 0.85)"
        };
        ctx.set_fill_style(&JsValue::from_str(kp_color));
        for (x, y) in slam.current_keypoints.iter().take(260) {
            let (cx, cy) = map_pt(*x, *y);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, 2.2, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }
    });
}

fn render_slam_map() {
    let document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = match document.get_element_by_id("slam-map") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;
    if width == 0 || height == 0 {
        return;
    }

    let dpr = window().unwrap().device_pixel_ratio();
    canvas.set_width((width as f64 * dpr) as u32);
    canvas.set_height((height as f64 * dpr) as u32);

    let ctx: CanvasRenderingContext2d = match canvas.get_context("2d") {
        Ok(Some(ctx)) => ctx.unchecked_into(),
        _ => return,
    };
    let _ = ctx.scale(dpr, dpr);

    let w = width as f64;
    let h = height as f64;

    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.22)"));
    ctx.fill_rect(0.0, 0.0, w, h);

    // Grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 255, 0.08)"));
    ctx.set_line_width(1.0);
    let grid = 40.0;
    let mut x = 0.0;
    while x <= w {
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, h);
        ctx.stroke();
        x += grid;
    }
    let mut y = 0.0;
    while y <= h {
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(w, y);
        ctx.stroke();
        y += grid;
    }

    STATE.with(|s| {
        let state = s.borrow();
        let slam = &state.visual_slam;
        if slam.trajectory.is_empty() {
            return;
        }

        let (cx, cy) = *slam.trajectory.last().unwrap_or(&(0.0, 0.0));
        let scale = 120.0; // px per map-unit

        let to_screen = |px: f64, py: f64| -> (f64, f64) {
            (w / 2.0 + (px - cx) * scale, h / 2.0 + (py - cy) * scale)
        };

        // Landmarks
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 230, 109, 0.55)"));
        for (lx, ly, _lz) in slam.map_points.iter().rev().take(600) {
            let (sx, sy) = to_screen(*lx, *ly);
            if sx < -10.0 || sx > w + 10.0 || sy < -10.0 || sy > h + 10.0 {
                continue;
            }
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, 1.5, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }

        // Trajectory
        ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 255, 0.9)"));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for (i, (tx, ty)) in slam.trajectory.iter().enumerate() {
            let (sx, sy) = to_screen(*tx, *ty);
            if i == 0 {
                ctx.move_to(sx, sy);
            } else {
                ctx.line_to(sx, sy);
            }
        }
        ctx.stroke();

        // Current pose marker (center)
        ctx.set_fill_style(&JsValue::from_str("rgba(78, 205, 196, 1.0)"));
        ctx.begin_path();
        let _ = ctx.arc(w / 2.0, h / 2.0, 4.0, 0.0, std::f64::consts::PI * 2.0);
        ctx.fill();
    });
}

fn render_accel_graph() {
    let document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = match document.get_element_by_id("accel-graph") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    // Get canvas element dimensions via computed style or direct attributes
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;

    if width == 0 || height == 0 {
        return;
    }

    // Set canvas internal resolution
    let dpr = window().unwrap().device_pixel_ratio();
    canvas.set_width((width as f64 * dpr) as u32);
    canvas.set_height((height as f64 * dpr) as u32);

    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();

    let _ = ctx.scale(dpr, dpr);

    let w = width as f64;
    let h = height as f64;

    // Clear
    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.3)"));
    ctx.fill_rect(0.0, 0.0, w, h);

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 255, 0.1)"));
    ctx.set_line_width(1.0);

    // Horizontal center line
    ctx.begin_path();
    ctx.move_to(0.0, h / 2.0);
    ctx.line_to(w, h / 2.0);
    ctx.stroke();

    // Draw data
    STATE.with(|s| {
        let state = s.borrow();

        let scale = h / 40.0; // ±20 m/s² range
        let center = h / 2.0;
        let step = w / (GRAPH_HISTORY_SIZE as f64 - 1.0);

        // X axis (red)
        draw_line(&ctx, &state.accel_history_x, step, center, scale, "#ff6b6b");

        // Y axis (teal)
        draw_line(&ctx, &state.accel_history_y, step, center, scale, "#4ecdc4");

        // Z axis (yellow)
        draw_line(&ctx, &state.accel_history_z, step, center, scale, "#ffe66d");
    });

    // Legend
    ctx.set_font("12px 'JetBrains Mono', monospace");

    ctx.set_fill_style(&JsValue::from_str("#ff6b6b"));
    let _ = ctx.fill_text("X", 10.0, 16.0);

    ctx.set_fill_style(&JsValue::from_str("#4ecdc4"));
    let _ = ctx.fill_text("Y", 30.0, 16.0);

    ctx.set_fill_style(&JsValue::from_str("#ffe66d"));
    let _ = ctx.fill_text("Z", 50.0, 16.0);
}

fn render_gyro_graph() {
    let document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = match document.get_element_by_id("gyro-graph") {
        Some(el) => el.unchecked_into(),
        None => return,
    };

    // Get canvas element dimensions via computed style or direct attributes
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;

    if width == 0 || height == 0 {
        return;
    }

    // Set canvas internal resolution
    let dpr = window().unwrap().device_pixel_ratio();
    canvas.set_width((width as f64 * dpr) as u32);
    canvas.set_height((height as f64 * dpr) as u32);

    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();

    let _ = ctx.scale(dpr, dpr);

    let w = width as f64;
    let h = height as f64;

    // Clear
    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.3)"));
    ctx.fill_rect(0.0, 0.0, w, h);

    // Draw grid
    ctx.set_stroke_style(&JsValue::from_str("rgba(0, 255, 255, 0.1)"));
    ctx.set_line_width(1.0);

    // Horizontal center line
    ctx.begin_path();
    ctx.move_to(0.0, h / 2.0);
    ctx.line_to(w, h / 2.0);
    ctx.stroke();

    // Draw data
    STATE.with(|s| {
        let state = s.borrow();

        let scale = h / 200.0; // ±100 deg/s range for gyroscope
        let center = h / 2.0;
        let step = w / (GRAPH_HISTORY_SIZE as f64 - 1.0);

        // X axis (red)
        draw_line(&ctx, &state.gyro_history_x, step, center, scale, "#ff6b6b");

        // Y axis (teal)
        draw_line(&ctx, &state.gyro_history_y, step, center, scale, "#4ecdc4");

        // Z axis (yellow)
        draw_line(&ctx, &state.gyro_history_z, step, center, scale, "#ffe66d");
    });

    // Legend
    ctx.set_font("12px 'JetBrains Mono', monospace");

    ctx.set_fill_style(&JsValue::from_str("#ff6b6b"));
    let _ = ctx.fill_text("X", 10.0, 16.0);

    ctx.set_fill_style(&JsValue::from_str("#4ecdc4"));
    let _ = ctx.fill_text("Y", 30.0, 16.0);

    ctx.set_fill_style(&JsValue::from_str("#ffe66d"));
    let _ = ctx.fill_text("Z", 50.0, 16.0);
}

fn draw_line(
    ctx: &CanvasRenderingContext2d,
    data: &[f64],
    step: f64,
    center: f64,
    scale: f64,
    color: &str,
) {
    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_line_width(2.0);
    ctx.begin_path();

    for (i, &val) in data.iter().enumerate() {
        let x = i as f64 * step;
        let y = center - val * scale;

        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }

    ctx.stroke();
}
