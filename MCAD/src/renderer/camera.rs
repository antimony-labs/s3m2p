//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: camera.rs | MCAD/src/renderer/camera.rs
//! PURPOSE: Orbit camera with view/projection matrices for WebGL2 rendering
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::{Mat4, Vec3};

/// Orbit camera for 3D viewport
#[derive(Clone, Debug)]
pub struct Camera {
    /// Pitch rotation (around X axis)
    pub rotation_x: f32,
    /// Yaw rotation (around Y axis)
    pub rotation_y: f32,
    /// Distance from target (orbit radius)
    pub distance: f32,
    /// Pan offset in world space
    pub pan_offset: Vec3,
    /// Field of view in radians
    pub fov: f32,
    /// Aspect ratio (width / height)
    pub aspect: f32,
    /// Near clip plane
    pub near: f32,
    /// Far clip plane
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            rotation_x: 0.5,    // Initial pitch (30 degrees)
            rotation_y: 0.75,   // Initial yaw (45 degrees)
            distance: 200.0,
            pan_offset: Vec3::ZERO,
            fov: 45.0_f32.to_radians(),
            aspect: 1.0,
            near: 0.1,
            far: 10000.0,
        }
    }
}

impl Camera {
    /// Create a new camera with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute camera position from orbit parameters
    pub fn position(&self) -> Vec3 {
        let x = self.distance * self.rotation_x.cos() * self.rotation_y.sin();
        let y = self.distance * self.rotation_x.sin();
        let z = self.distance * self.rotation_x.cos() * self.rotation_y.cos();
        Vec3::new(x, y, z) + self.pan_offset
    }

    /// Get the camera target (look-at point)
    pub fn target(&self) -> Vec3 {
        self.pan_offset
    }

    /// Compute view matrix (world to camera space)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position(), self.target(), Vec3::Y)
    }

    /// Compute projection matrix (camera to clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Orbit the camera (left-click drag)
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.rotation_y += dx * 0.01;
        self.rotation_x = (self.rotation_x + dy * 0.01).clamp(-1.5, 1.5);
    }

    /// Pan the camera (middle-click drag)
    pub fn pan(&mut self, dx: f32, dy: f32) {
        // Compute right and up vectors in view space
        let pos = self.position();
        let target = self.target();
        let forward = (target - pos).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward);

        // Scale pan by distance for consistent feel
        let scale = self.distance * 0.002;
        self.pan_offset += right * (-dx * scale) + up * (dy * scale);
    }

    /// Zoom the camera (scroll wheel)
    pub fn zoom(&mut self, delta: f32) {
        let factor = 1.0 - delta * 0.1;
        self.distance = (self.distance * factor).clamp(1.0, 5000.0);
    }

    /// Set camera aspect ratio
    pub fn set_aspect(&mut self, width: f32, height: f32) {
        if height > 0.0 {
            self.aspect = width / height;
        }
    }

    /// Set camera to a preset view
    pub fn set_view(&mut self, preset: &str) {
        use std::f32::consts::{FRAC_PI_2, PI};
        match preset {
            "front" => {
                self.rotation_x = 0.0;
                self.rotation_y = 0.0;
            }
            "back" => {
                self.rotation_x = 0.0;
                self.rotation_y = PI;
            }
            "top" => {
                self.rotation_x = FRAC_PI_2;
                self.rotation_y = 0.0;
            }
            "bottom" => {
                self.rotation_x = -FRAC_PI_2;
                self.rotation_y = 0.0;
            }
            "left" => {
                self.rotation_x = 0.0;
                self.rotation_y = -FRAC_PI_2;
            }
            "right" => {
                self.rotation_x = 0.0;
                self.rotation_y = FRAC_PI_2;
            }
            "iso" | _ => {
                self.rotation_x = 0.5;
                self.rotation_y = 0.75;
            }
        }
    }

    /// Reset camera to default state
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Get right vector in world space
    pub fn right(&self) -> Vec3 {
        let pos = self.position();
        let target = self.target();
        let forward = (target - pos).normalize();
        forward.cross(Vec3::Y).normalize()
    }

    /// Get up vector in world space
    pub fn up(&self) -> Vec3 {
        let right = self.right();
        let pos = self.position();
        let target = self.target();
        let forward = (target - pos).normalize();
        right.cross(forward)
    }
}
