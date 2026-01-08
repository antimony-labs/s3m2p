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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_camera_default_values() {
        let camera = Camera::new();
        assert!((camera.rotation_x - 0.5).abs() < 1e-6);
        assert!((camera.rotation_y - 0.75).abs() < 1e-6);
        assert!((camera.distance - 200.0).abs() < 1e-6);
        assert!(camera.pan_offset.length() < 1e-6);
    }

    #[test]
    fn test_position_not_at_origin() {
        let camera = Camera::new();
        let pos = camera.position();
        // Camera position should not be at origin
        assert!(pos.length() > 0.1);
    }

    #[test]
    fn test_position_at_expected_distance() {
        let camera = Camera::new();
        let pos = camera.position();
        // Distance from origin should match camera distance
        assert!((pos.length() - camera.distance).abs() < 1e-4);
    }

    #[test]
    fn test_view_matrix_invertible() {
        let camera = Camera::new();
        let view = camera.view_matrix();
        let det = view.determinant();
        // View matrix should be invertible (non-zero determinant)
        assert!(det.abs() > 0.01);
    }

    #[test]
    fn test_projection_matrix_invertible() {
        let mut camera = Camera::new();
        camera.set_aspect(800.0, 600.0);
        let proj = camera.projection_matrix();
        let det = proj.determinant();
        // Projection matrix should be invertible
        assert!(det.abs() > 0.0);
    }

    #[test]
    fn test_view_projection_combined() {
        let camera = Camera::new();
        let vp = camera.view_projection_matrix();
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();

        // VP should equal P * V
        let combined = proj * view;
        for i in 0..16 {
            let row = i / 4;
            let col = i % 4;
            assert!((vp.col(col)[row] - combined.col(col)[row]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_orbit_changes_rotation() {
        let mut camera = Camera::new();
        let initial_x = camera.rotation_x;
        let initial_y = camera.rotation_y;

        camera.orbit(100.0, 50.0);

        assert!(camera.rotation_y > initial_y);
        assert!(camera.rotation_x > initial_x);
    }

    #[test]
    fn test_orbit_clamps_pitch() {
        let mut camera = Camera::new();

        // Try to rotate beyond limits
        camera.orbit(0.0, 1000.0);
        assert!(camera.rotation_x <= 1.5);

        camera.orbit(0.0, -2000.0);
        assert!(camera.rotation_x >= -1.5);
    }

    #[test]
    fn test_pan_changes_offset() {
        let mut camera = Camera::new();
        assert!(camera.pan_offset.length() < 1e-6);

        camera.pan(100.0, 50.0);

        assert!(camera.pan_offset.length() > 0.1);
    }

    #[test]
    fn test_pan_target_moves() {
        let mut camera = Camera::new();
        let initial_target = camera.target();

        camera.pan(100.0, 50.0);

        let new_target = camera.target();
        assert!((new_target - initial_target).length() > 0.1);
    }

    #[test]
    fn test_zoom_in_decreases_distance() {
        let mut camera = Camera::new();
        let initial_distance = camera.distance;

        camera.zoom(1.0); // Positive delta = zoom in

        assert!(camera.distance < initial_distance);
    }

    #[test]
    fn test_zoom_out_increases_distance() {
        let mut camera = Camera::new();
        let initial_distance = camera.distance;

        camera.zoom(-1.0); // Negative delta = zoom out

        assert!(camera.distance > initial_distance);
    }

    #[test]
    fn test_zoom_clamps_minimum() {
        let mut camera = Camera::new();
        camera.distance = 10.0;

        // Zoom in a lot
        for _ in 0..100 {
            camera.zoom(1.0);
        }

        assert!(camera.distance >= 1.0);
    }

    #[test]
    fn test_zoom_clamps_maximum() {
        let mut camera = Camera::new();
        camera.distance = 4000.0;

        // Zoom out a lot
        for _ in 0..100 {
            camera.zoom(-1.0);
        }

        assert!(camera.distance <= 5000.0);
    }

    #[test]
    fn test_set_aspect_ratio() {
        let mut camera = Camera::new();
        camera.set_aspect(1920.0, 1080.0);

        let expected = 1920.0 / 1080.0;
        assert!((camera.aspect - expected).abs() < 1e-6);
    }

    #[test]
    fn test_set_aspect_zero_height_safe() {
        let mut camera = Camera::new();
        let initial_aspect = camera.aspect;

        camera.set_aspect(800.0, 0.0); // Should not crash or change

        assert!((camera.aspect - initial_aspect).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_front() {
        let mut camera = Camera::new();
        camera.set_view("front");

        assert!((camera.rotation_x - 0.0).abs() < 1e-6);
        assert!((camera.rotation_y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_top() {
        let mut camera = Camera::new();
        camera.set_view("top");

        use std::f32::consts::FRAC_PI_2;
        assert!((camera.rotation_x - FRAC_PI_2).abs() < 1e-6);
        assert!((camera.rotation_y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_right() {
        let mut camera = Camera::new();
        camera.set_view("right");

        use std::f32::consts::FRAC_PI_2;
        assert!((camera.rotation_x - 0.0).abs() < 1e-6);
        assert!((camera.rotation_y - FRAC_PI_2).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_back() {
        let mut camera = Camera::new();
        camera.set_view("back");

        use std::f32::consts::PI;
        assert!((camera.rotation_x - 0.0).abs() < 1e-6);
        assert!((camera.rotation_y - PI).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_iso() {
        let mut camera = Camera::new();
        camera.rotation_x = 0.0;
        camera.rotation_y = 0.0;

        camera.set_view("iso");

        // Should restore to isometric defaults
        assert!((camera.rotation_x - 0.5).abs() < 1e-6);
        assert!((camera.rotation_y - 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_preset_view_unknown_defaults_to_iso() {
        let mut camera = Camera::new();
        camera.rotation_x = 0.0;
        camera.rotation_y = 0.0;

        camera.set_view("unknown_preset");

        // Should default to isometric
        assert!((camera.rotation_x - 0.5).abs() < 1e-6);
        assert!((camera.rotation_y - 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_reset_restores_defaults() {
        let mut camera = Camera::new();
        camera.rotation_x = 1.2;
        camera.rotation_y = 2.3;
        camera.distance = 500.0;
        camera.pan_offset = Vec3::new(100.0, 200.0, 300.0);

        camera.reset();

        assert!((camera.rotation_x - 0.5).abs() < 1e-6);
        assert!((camera.rotation_y - 0.75).abs() < 1e-6);
        assert!((camera.distance - 200.0).abs() < 1e-6);
        assert!(camera.pan_offset.length() < 1e-6);
    }

    #[test]
    fn test_right_vector_perpendicular_to_up() {
        let camera = Camera::new();
        let right = camera.right();
        let up = camera.up();

        // Right and up should be perpendicular
        let dot = right.dot(up);
        assert!(dot.abs() < 1e-4);
    }

    #[test]
    fn test_right_and_up_are_unit_vectors() {
        let camera = Camera::new();
        let right = camera.right();
        let up = camera.up();

        assert!((right.length() - 1.0).abs() < 1e-4);
        assert!((up.length() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_target_matches_pan_offset() {
        let mut camera = Camera::new();
        camera.pan_offset = Vec3::new(10.0, 20.0, 30.0);

        let target = camera.target();
        assert!((target - camera.pan_offset).length() < 1e-6);
    }

    #[test]
    fn test_front_view_position_on_z_axis() {
        let mut camera = Camera::new();
        camera.set_view("front");

        let pos = camera.position();
        // Front view: camera should be on positive Z axis
        assert!(pos.z.abs() > 1.0); // Significant Z
        assert!(pos.x.abs() < 1e-4); // Near zero X
        assert!(pos.y.abs() < 1e-4); // Near zero Y
    }

    #[test]
    fn test_top_view_position_on_y_axis() {
        let mut camera = Camera::new();
        camera.set_view("top");

        let pos = camera.position();
        // Top view: camera should be on positive Y axis
        assert!(pos.y > 1.0); // Significant Y
        assert!(pos.x.abs() < 1e-4); // Near zero X
        // Z might have slight value due to trigonometry
    }
}
