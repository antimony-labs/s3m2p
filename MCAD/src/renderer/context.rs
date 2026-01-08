//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: context.rs | MCAD/src/renderer/context.rs
//! PURPOSE: WebGL2 rendering context and shader program management
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

use super::camera::Camera;
use super::shaders::{self, RenderMode};
use super::buffers::MeshBuffers;
use cad_engine::FaceId;
use glam::{Mat4, Vec3, Mat3};
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader,
    WebGlUniformLocation,
};

/// WebGL2 rendering context for MCAD
pub struct WebGLContext {
    /// WebGL2 context
    pub gl: GL,
    /// Shader program for shaded rendering
    pub shaded_program: WebGlProgram,
    /// Shader program for wireframe rendering
    pub wireframe_program: WebGlProgram,
    /// Shader program for face ID picking
    pub picking_program: WebGlProgram,
    /// Current render mode
    pub mode: RenderMode,
    /// Canvas width
    pub width: u32,
    /// Canvas height
    pub height: u32,
}

impl WebGLContext {
    /// Create a new WebGL2 context from a canvas
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl2")?
            .ok_or("WebGL2 not supported")?
            .dyn_into::<GL>()?;

        // Compile shader programs
        let shaded_program = compile_program(&gl, shaders::SHADED_VERTEX, shaders::SHADED_FRAGMENT)?;
        let wireframe_program = compile_program(&gl, shaders::WIREFRAME_VERTEX, shaders::WIREFRAME_FRAGMENT)?;
        let picking_program = compile_program(&gl, shaders::PICKING_VERTEX, shaders::PICKING_FRAGMENT)?;

        let width = canvas.width();
        let height = canvas.height();

        Ok(Self {
            gl,
            shaded_program,
            wireframe_program,
            picking_program,
            mode: RenderMode::default(),
            width,
            height,
        })
    }

    /// Update viewport size
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.gl.viewport(0, 0, width as i32, height as i32);
    }

    /// Render the scene
    pub fn render(
        &self,
        buffers: &MeshBuffers,
        camera: &Camera,
        selected_face: Option<FaceId>,
        hover_face: Option<FaceId>,
    ) {
        let gl = &self.gl;

        // Clear with background color
        gl.clear_color(0.039, 0.039, 0.071, 1.0); // #0a0a12
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);

        match self.mode {
            RenderMode::Wireframe => self.render_wireframe(buffers, camera),
            RenderMode::Shaded => self.render_shaded(buffers, camera, selected_face, hover_face),
            RenderMode::HiddenLine => self.render_hidden_line(buffers, camera),
        }
    }

    /// Render in shaded mode with Phong lighting
    fn render_shaded(
        &self,
        buffers: &MeshBuffers,
        camera: &Camera,
        selected_face: Option<FaceId>,
        hover_face: Option<FaceId>,
    ) {
        let gl = &self.gl;
        gl.use_program(Some(&self.shaded_program));

        // Enable backface culling for solid rendering
        gl.enable(GL::CULL_FACE);
        gl.cull_face(GL::BACK);

        // Compute matrices
        let model = Mat4::IDENTITY;
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        let mvp = projection * view * model;
        let normal_matrix = Mat3::from_mat4(view * model).inverse().transpose();

        // Set matrix uniforms
        set_uniform_mat4(gl, &self.shaded_program, "u_model", &model);
        set_uniform_mat4(gl, &self.shaded_program, "u_view", &view);
        set_uniform_mat4(gl, &self.shaded_program, "u_projection", &projection);
        set_uniform_mat3(gl, &self.shaded_program, "u_normal_matrix", &normal_matrix);

        // Set lighting uniforms
        let light_dir = Vec3::new(0.5, 0.7, 1.0).normalize();
        set_uniform_vec3(gl, &self.shaded_program, "u_light_dir", light_dir);
        set_uniform_vec3(gl, &self.shaded_program, "u_camera_pos", camera.position());
        set_uniform_vec3(gl, &self.shaded_program, "u_base_color", Vec3::new(1.0, 0.42, 0.21)); // #ff6b35
        set_uniform_vec3(gl, &self.shaded_program, "u_select_color", Vec3::new(0.0, 0.8, 1.0)); // Cyan
        set_uniform_vec3(gl, &self.shaded_program, "u_hover_color", Vec3::new(0.5, 0.9, 1.0)); // Light cyan
        set_uniform_f32(gl, &self.shaded_program, "u_ambient", 0.2);
        set_uniform_f32(gl, &self.shaded_program, "u_diffuse", 0.7);
        set_uniform_f32(gl, &self.shaded_program, "u_specular", 0.3);
        set_uniform_f32(gl, &self.shaded_program, "u_shininess", 32.0);

        // Set selection uniforms
        let selected = selected_face.map(|f| f.0 as f32).unwrap_or(-1.0);
        let hovered = hover_face.map(|f| f.0 as f32).unwrap_or(-1.0);
        set_uniform_f32(gl, &self.shaded_program, "u_selected_face", selected);
        set_uniform_f32(gl, &self.shaded_program, "u_hover_face", hovered);

        // Draw triangles
        gl.bind_vertex_array(Some(&buffers.shaded_vao));
        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            (buffers.triangle_count * 3) as i32,
            GL::UNSIGNED_INT,
            0,
        );

        gl.disable(GL::CULL_FACE);
    }

    /// Render in wireframe mode
    fn render_wireframe(&self, buffers: &MeshBuffers, camera: &Camera) {
        let gl = &self.gl;
        gl.use_program(Some(&self.wireframe_program));

        let mvp = camera.view_projection_matrix();
        set_uniform_mat4(gl, &self.wireframe_program, "u_mvp", &mvp);
        set_uniform_vec3(gl, &self.wireframe_program, "u_color", Vec3::new(1.0, 0.42, 0.21)); // #ff6b35
        set_uniform_f32(gl, &self.wireframe_program, "u_alpha", 1.0);

        // Draw edges
        gl.bind_vertex_array(Some(&buffers.wireframe_vao));
        gl.draw_elements_with_i32(
            GL::LINES,
            (buffers.edge_count * 2) as i32,
            GL::UNSIGNED_INT,
            0,
        );
    }

    /// Render in hidden-line mode (wireframe with depth)
    fn render_hidden_line(&self, buffers: &MeshBuffers, camera: &Camera) {
        let gl = &self.gl;

        // Pass 1: Render faces to depth buffer only (no color)
        gl.color_mask(false, false, false, false);
        gl.enable(GL::POLYGON_OFFSET_FILL);
        gl.polygon_offset(1.0, 1.0); // Push faces back

        gl.use_program(Some(&self.shaded_program));
        let model = Mat4::IDENTITY;
        let view = camera.view_matrix();
        let projection = camera.projection_matrix();
        set_uniform_mat4(gl, &self.shaded_program, "u_model", &model);
        set_uniform_mat4(gl, &self.shaded_program, "u_view", &view);
        set_uniform_mat4(gl, &self.shaded_program, "u_projection", &projection);

        gl.bind_vertex_array(Some(&buffers.shaded_vao));
        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            (buffers.triangle_count * 3) as i32,
            GL::UNSIGNED_INT,
            0,
        );

        // Pass 2: Render wireframe with depth test
        gl.color_mask(true, true, true, true);
        gl.disable(GL::POLYGON_OFFSET_FILL);

        self.render_wireframe(buffers, camera);
    }

    /// Set render mode
    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
    }
}

/// Compile a shader program from vertex and fragment sources
fn compile_program(gl: &GL, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, JsValue> {
    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_src)?;
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_src)?;

    let program = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if !gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl.get_program_info_log(&program).unwrap_or_default();
        return Err(JsValue::from_str(&format!("Program link error: {}", info)));
    }

    Ok(program)
}

/// Compile a single shader
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Failed to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl.get_shader_info_log(&shader).unwrap_or_default();
        let shader_type_name = if shader_type == GL::VERTEX_SHADER {
            "Vertex"
        } else {
            "Fragment"
        };
        return Err(JsValue::from_str(&format!(
            "{} shader compile error: {}",
            shader_type_name, info
        )));
    }

    Ok(shader)
}

/// Set a mat4 uniform
fn set_uniform_mat4(gl: &GL, program: &WebGlProgram, name: &str, matrix: &Mat4) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, matrix.as_ref());
    }
}

/// Set a mat3 uniform
fn set_uniform_mat3(gl: &GL, program: &WebGlProgram, name: &str, matrix: &Mat3) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_matrix3fv_with_f32_array(Some(&loc), false, matrix.as_ref());
    }
}

/// Set a vec3 uniform
fn set_uniform_vec3(gl: &GL, program: &WebGlProgram, name: &str, v: Vec3) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform3f(Some(&loc), v.x, v.y, v.z);
    }
}

/// Set a f32 uniform
fn set_uniform_f32(gl: &GL, program: &WebGlProgram, name: &str, value: f32) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform1f(Some(&loc), value);
    }
}
