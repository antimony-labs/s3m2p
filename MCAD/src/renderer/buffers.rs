//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: buffers.rs | MCAD/src/renderer/buffers.rs
//! PURPOSE: VBO/VAO mesh buffer management for WebGL2 rendering
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

use cad_engine::{Solid, FaceId, solid_to_mesh};
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext as GL, WebGlBuffer, WebGlVertexArrayObject};

/// GPU mesh buffers for rendering a solid
pub struct MeshBuffers {
    /// Vertex positions (3 floats per vertex)
    pub position_buffer: WebGlBuffer,
    /// Vertex normals (3 floats per vertex)
    pub normal_buffer: WebGlBuffer,
    /// Face IDs (1 float per vertex, for selection highlighting)
    pub face_id_buffer: WebGlBuffer,
    /// Triangle indices for shaded rendering
    pub triangle_indices: WebGlBuffer,
    /// Edge indices for wireframe rendering
    pub edge_indices: WebGlBuffer,
    /// VAO for shaded rendering
    pub shaded_vao: WebGlVertexArrayObject,
    /// VAO for wireframe rendering
    pub wireframe_vao: WebGlVertexArrayObject,
    /// Number of triangles
    pub triangle_count: u32,
    /// Number of edges
    pub edge_count: u32,
    /// Number of vertices
    pub vertex_count: u32,
    /// Triangle-to-face mapping (for picking)
    pub triangle_to_face: Vec<FaceId>,
}

impl MeshBuffers {
    /// Create GPU buffers from a solid
    pub fn from_solid(gl: &GL, solid: &Solid) -> Result<Self, JsValue> {
        let mesh = solid_to_mesh(solid);

        // Build expanded vertex data (per-triangle vertices for flat shading)
        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut face_ids: Vec<f32> = Vec::new();
        let mut triangle_indices: Vec<u32> = Vec::new();
        let mut triangle_to_face: Vec<FaceId> = Vec::new();

        let mut vertex_idx = 0u32;

        // Map triangles to faces
        let face_for_triangle = map_triangles_to_faces(solid, &mesh);

        for (tri_idx, triangle) in mesh.triangles.iter().enumerate() {
            let normal = if tri_idx < mesh.normals.len() {
                mesh.normals[tri_idx]
            } else {
                cad_engine::Vector3::new(0.0, 0.0, 1.0)
            };

            let face_id = face_for_triangle.get(tri_idx).copied().unwrap_or(FaceId(0));
            triangle_to_face.push(face_id);

            for &idx in triangle {
                if idx < mesh.vertices.len() {
                    let v = mesh.vertices[idx];
                    positions.extend_from_slice(&[v.x, v.y, v.z]);
                    normals.extend_from_slice(&[normal.x, normal.y, normal.z]);
                    face_ids.push(face_id.0 as f32);
                    triangle_indices.push(vertex_idx);
                    vertex_idx += 1;
                }
            }
        }

        // Build edge indices for wireframe
        let edge_indices_data = build_edge_indices(solid);

        // Create buffers
        let position_buffer = create_buffer(gl, &positions)?;
        let normal_buffer = create_buffer(gl, &normals)?;
        let face_id_buffer = create_buffer(gl, &face_ids)?;
        let triangle_index_buffer = create_index_buffer(gl, &triangle_indices)?;
        let edge_index_buffer = create_index_buffer(gl, &edge_indices_data)?;

        // Create VAOs
        let shaded_vao = gl.create_vertex_array().ok_or("Failed to create shaded VAO")?;
        let wireframe_vao = gl.create_vertex_array().ok_or("Failed to create wireframe VAO")?;

        Ok(Self {
            position_buffer,
            normal_buffer,
            face_id_buffer,
            triangle_indices: triangle_index_buffer,
            edge_indices: edge_index_buffer,
            shaded_vao,
            wireframe_vao,
            triangle_count: mesh.triangles.len() as u32,
            edge_count: (edge_indices_data.len() / 2) as u32,
            vertex_count: vertex_idx,
            triangle_to_face,
        })
    }

    /// Setup VAO for shaded rendering
    pub fn setup_shaded_vao(&self, gl: &GL, program: &web_sys::WebGlProgram) {
        gl.bind_vertex_array(Some(&self.shaded_vao));

        // Position attribute (location 0)
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        let pos_loc = gl.get_attrib_location(program, "a_position");
        if pos_loc >= 0 {
            gl.enable_vertex_attrib_array(pos_loc as u32);
            gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 3, GL::FLOAT, false, 0, 0);
        }

        // Normal attribute (location 1)
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normal_buffer));
        let norm_loc = gl.get_attrib_location(program, "a_normal");
        if norm_loc >= 0 {
            gl.enable_vertex_attrib_array(norm_loc as u32);
            gl.vertex_attrib_pointer_with_i32(norm_loc as u32, 3, GL::FLOAT, false, 0, 0);
        }

        // Face ID attribute (location 2)
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.face_id_buffer));
        let fid_loc = gl.get_attrib_location(program, "a_face_id");
        if fid_loc >= 0 {
            gl.enable_vertex_attrib_array(fid_loc as u32);
            gl.vertex_attrib_pointer_with_i32(fid_loc as u32, 1, GL::FLOAT, false, 0, 0);
        }

        // Element buffer
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.triangle_indices));

        gl.bind_vertex_array(None);
    }

    /// Setup VAO for wireframe rendering
    pub fn setup_wireframe_vao(&self, gl: &GL, program: &web_sys::WebGlProgram) {
        gl.bind_vertex_array(Some(&self.wireframe_vao));

        // Position attribute only
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        let pos_loc = gl.get_attrib_location(program, "a_position");
        if pos_loc >= 0 {
            gl.enable_vertex_attrib_array(pos_loc as u32);
            gl.vertex_attrib_pointer_with_i32(pos_loc as u32, 3, GL::FLOAT, false, 0, 0);
        }

        // Element buffer (edges)
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.edge_indices));

        gl.bind_vertex_array(None);
    }
}

/// Create a float buffer
fn create_buffer(gl: &GL, data: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let array = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    Ok(buffer)
}

/// Create an index buffer
fn create_index_buffer(gl: &GL, data: &[u32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("Failed to create index buffer")?;
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let array = js_sys::Uint32Array::view(data);
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    Ok(buffer)
}

/// Map triangles to their source faces
fn map_triangles_to_faces(solid: &Solid, mesh: &cad_engine::TriangleMesh) -> Vec<FaceId> {
    // Simple mapping: assume triangles are generated face-by-face
    // Each face generates (n-2) triangles where n is vertex count
    let mut result = Vec::with_capacity(mesh.triangles.len());

    let mut tri_idx = 0;
    for face in &solid.faces {
        // Count vertices in face's outer loop
        let vertex_count = face.outer_loop.edges.len();
        let triangles_for_face = if vertex_count >= 3 { vertex_count - 2 } else { 0 };

        for _ in 0..triangles_for_face {
            if tri_idx < mesh.triangles.len() {
                result.push(face.id);
                tri_idx += 1;
            }
        }
    }

    // Fill remaining with FaceId(0) if mesh has more triangles
    while result.len() < mesh.triangles.len() {
        result.push(FaceId(0));
    }

    result
}

/// Build edge indices for wireframe rendering
fn build_edge_indices(solid: &Solid) -> Vec<u32> {
    let mut indices: Vec<u32> = Vec::new();

    for edge in &solid.edges {
        indices.push(edge.start.0);
        indices.push(edge.end.0);
    }

    indices
}
