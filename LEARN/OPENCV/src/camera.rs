//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: camera.rs | OPENCV/src/camera.rs
//! PURPOSE: Camera access and video stream handling
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlVideoElement, MediaStreamConstraints};

/// Check if camera is available on this device
#[allow(dead_code)]
pub fn check_camera_support() -> bool {
    if let Some(window) = window() {
        let navigator = window.navigator();
        navigator.media_devices().is_ok()
    } else {
        false
    }
}

/// Set up the camera enable button
pub fn setup_camera_button() {
    let document = match window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

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

/// Request camera access from the user
async fn request_camera() {
    let window = match window() {
        Some(w) => w,
        None => return,
    };

    let navigator = window.navigator();

    if let Ok(media_devices) = navigator.media_devices() {
        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::TRUE);
        constraints.set_audio(&JsValue::FALSE);

        match media_devices.get_user_media_with_constraints(&constraints) {
            Ok(promise) => match JsFuture::from(promise).await {
                Ok(stream) => {
                    let stream: web_sys::MediaStream = stream.unchecked_into();

                    let document = window.document().unwrap();

                    // Attach stream to video element
                    if let Some(video) = document.get_element_by_id("camera-video") {
                        let video: HtmlVideoElement = video.unchecked_into();
                        video.set_src_object(Some(&stream));
                        let _ = video.play();
                    }

                    // Hide placeholder
                    if let Some(placeholder) = document.get_element_by_id("camera-placeholder") {
                        let _ = placeholder.set_attribute("style", "display: none;");
                    }

                    // Notify demo runner that camera is ready
                    crate::demo_runner::on_camera_ready();

                    web_sys::console::log_1(&"Camera streaming".into());
                }
                Err(_) => {
                    update_camera_placeholder("Camera permission denied");
                    web_sys::console::error_1(&"Camera permission denied".into());
                }
            },
            Err(_) => {
                update_camera_placeholder("Failed to access camera");
                web_sys::console::error_1(&"Failed to access camera".into());
            }
        }
    }
}

/// Update the camera placeholder text
fn update_camera_placeholder(message: &str) {
    if let Some(document) = window().and_then(|w| w.document()) {
        if let Some(text) = document.get_element_by_id("camera-status-text") {
            text.set_text_content(Some(message));
        }
    }
}

/// Get the video element if available
pub fn get_video_element() -> Option<HtmlVideoElement> {
    window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("camera-video"))
        .map(|e| e.unchecked_into())
}
