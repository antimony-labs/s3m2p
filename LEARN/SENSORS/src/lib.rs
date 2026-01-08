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
    window, CanvasRenderingContext2d, DeviceOrientationEvent, HtmlCanvasElement, HtmlVideoElement,
    MediaStreamConstraints, AudioContext, AnalyserNode, AudioContextOptions,
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

struct DeadReckoning {
    velocity_x: f64,
    velocity_y: f64,
    velocity_z: f64,
    speed: f64,
    distance: f64,
    last_time: Option<f64>,
    gravity_x: f64,
    gravity_y: f64,
    gravity_z: f64,
    gravity_calibrated: bool,
}

impl Default for DeadReckoning {
    fn default() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            speed: 0.0,
            distance: 0.0,
            last_time: None,
            gravity_x: 0.0,
            gravity_y: 0.0,
            gravity_z: 0.0,
            gravity_calibrated: false,
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
    dead_reckoning: DeadReckoning,
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
            dead_reckoning: DeadReckoning::default(),
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

    // Set up microphone button
    setup_microphone_button();

    // Set up sensor permission button (for iOS)
    setup_sensor_button();

    // Set up dead reckoning reset button
    setup_dead_reckoning_reset_button();

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
                    
                    // Update dead reckoning
                    update_dead_reckoning(&mut state.dead_reckoning, x, y, z);
                });

                update_accel_display(x, y, z);
                update_dead_reckoning_display();
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
    let window = window().expect("no window");
    let navigator = window.navigator();

    if let Ok(media_devices) = navigator.media_devices() {
        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::TRUE);
        constraints.set_audio(&JsValue::FALSE);

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

                        // Hide placeholder via style
                        if let Some(placeholder) = document.get_element_by_id("camera-placeholder")
                        {
                            let _ = placeholder.set_attribute("style", "display: none;");
                        }

                        update_camera_status("available", "Streaming");
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

fn update_dead_reckoning(dr: &mut DeadReckoning, accel_x: f64, accel_y: f64, accel_z: f64) {
    let window = window().unwrap();
    let performance = window.performance().unwrap();
    let current_time = performance.now() / 1000.0; // Convert to seconds

    // Calibrate gravity when device is stationary (first few readings)
    if !dr.gravity_calibrated {
        // Use first reading as gravity estimate (device should be at rest)
        dr.gravity_x = accel_x;
        dr.gravity_y = accel_y;
        dr.gravity_z = accel_z;
        dr.gravity_calibrated = true;
        dr.last_time = Some(current_time);
        return;
    }

    if let Some(last_time) = dr.last_time {
        let dt = current_time - last_time;
        
        // Only process if dt is reasonable (avoid huge jumps)
        if dt > 0.0 && dt < 1.0 {
            // Subtract gravity to get linear acceleration
            let linear_accel_x = accel_x - dr.gravity_x;
            let linear_accel_y = accel_y - dr.gravity_y;
            let linear_accel_z = accel_z - dr.gravity_z;
            
            // Integrate acceleration to get velocity: v = v0 + a*dt
            dr.velocity_x += linear_accel_x * dt;
            dr.velocity_y += linear_accel_y * dt;
            dr.velocity_z += linear_accel_z * dt;
            
            // Apply damping to reduce drift (simple low-pass filter)
            let damping = 0.95;
            dr.velocity_x *= damping;
            dr.velocity_y *= damping;
            dr.velocity_z *= damping;
            
            // Calculate speed magnitude: |v| = sqrt(vx² + vy² + vz²)
            dr.speed = (dr.velocity_x * dr.velocity_x 
                       + dr.velocity_y * dr.velocity_y 
                       + dr.velocity_z * dr.velocity_z).sqrt();
            
            // Integrate velocity to get distance: d = d0 + v*dt
            let avg_velocity = dr.speed;
            dr.distance += avg_velocity * dt;
        }
    }
    
    dr.last_time = Some(current_time);
}

fn update_dead_reckoning_display() {
    let document = window().unwrap().document().unwrap();

    STATE.with(|s| {
        let state = s.borrow();
        let dr = &state.dead_reckoning;

        if let Some(el) = document.get_element_by_id("vel-x") {
            el.set_text_content(Some(&format!("{:.2}", dr.velocity_x)));
        }
        if let Some(el) = document.get_element_by_id("vel-y") {
            el.set_text_content(Some(&format!("{:.2}", dr.velocity_y)));
        }
        if let Some(el) = document.get_element_by_id("vel-z") {
            el.set_text_content(Some(&format!("{:.2}", dr.velocity_z)));
        }
        if let Some(el) = document.get_element_by_id("speed-value") {
            el.set_text_content(Some(&format!("{:.2} m/s", dr.speed)));
        }
        if let Some(el) = document.get_element_by_id("distance-value") {
            el.set_text_content(Some(&format!("{:.2} m", dr.distance)));
        }
    });
}

fn setup_dead_reckoning_reset_button() {
    let document = window().unwrap().document().unwrap();

    if let Some(btn) = document.get_element_by_id("reset-dead-reckoning-btn") {
        let closure = Closure::wrap(Box::new(move || {
            reset_dead_reckoning();
        }) as Box<dyn Fn()>);

        let _ = btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

fn reset_dead_reckoning() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.dead_reckoning = DeadReckoning::default();
    });
    
    update_dead_reckoning_display();
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

                if let Ok(add_listener_val) = js_sys::Reflect::get(&sensor_obj, &"addEventListener".into()) {
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
            if let Ok(sensor) = js_sys::Reflect::construct(
                proximity_sensor.unchecked_ref(),
                &js_sys::Array::new(),
            ) {
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

                if let Ok(add_listener_val) = js_sys::Reflect::get(&sensor_obj, &"addEventListener".into()) {
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
