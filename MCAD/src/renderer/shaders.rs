//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: shaders.rs | MCAD/src/renderer/shaders.rs
//! PURPOSE: GLSL shaders for WebGL2 CAD rendering (shaded, wireframe, picking)
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

/// Render mode for 3D viewport
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenderMode {
    /// Edge lines only (fast, classic CAD look)
    #[default]
    Wireframe,
    /// Filled faces with Phong lighting
    Shaded,
    /// Wireframe with hidden line removal
    HiddenLine,
}

/// Vertex shader for shaded rendering
pub const SHADED_VERTEX: &str = r#"#version 300 es
precision highp float;

in vec3 a_position;
in vec3 a_normal;
in float a_face_id;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform mat3 u_normal_matrix;

out vec3 v_position;
out vec3 v_normal;
out float v_face_id;

void main() {
    vec4 world_pos = u_model * vec4(a_position, 1.0);
    v_position = world_pos.xyz;
    v_normal = normalize(u_normal_matrix * a_normal);
    v_face_id = a_face_id;

    gl_Position = u_projection * u_view * world_pos;
}
"#;

/// Fragment shader for shaded rendering (Phong lighting)
pub const SHADED_FRAGMENT: &str = r#"#version 300 es
precision highp float;

in vec3 v_position;
in vec3 v_normal;
in float v_face_id;

uniform vec3 u_light_dir;
uniform vec3 u_camera_pos;
uniform vec3 u_base_color;
uniform vec3 u_select_color;
uniform vec3 u_hover_color;
uniform float u_selected_face;
uniform float u_hover_face;
uniform float u_ambient;
uniform float u_diffuse;
uniform float u_specular;
uniform float u_shininess;

out vec4 fragColor;

void main() {
    vec3 N = normalize(v_normal);
    vec3 L = normalize(u_light_dir);
    vec3 V = normalize(u_camera_pos - v_position);
    vec3 H = normalize(L + V);

    // Phong lighting
    float ambient = u_ambient;
    float diffuse = u_diffuse * max(dot(N, L), 0.0);
    float specular = u_specular * pow(max(dot(N, H), 0.0), u_shininess);

    // Two-sided lighting for backfaces
    if (!gl_FrontFacing) {
        diffuse = u_diffuse * max(dot(-N, L), 0.0) * 0.5;
        specular = 0.0;
    }

    // Face color (base, selected, or hovered)
    vec3 color = u_base_color;
    if (abs(v_face_id - u_selected_face) < 0.5) {
        color = u_select_color;
    } else if (abs(v_face_id - u_hover_face) < 0.5) {
        color = u_hover_color;
    }

    float lighting = ambient + diffuse + specular;
    fragColor = vec4(color * lighting, 1.0);
}
"#;

/// Vertex shader for wireframe rendering
pub const WIREFRAME_VERTEX: &str = r#"#version 300 es
precision highp float;

in vec3 a_position;

uniform mat4 u_mvp;

void main() {
    gl_Position = u_mvp * vec4(a_position, 1.0);
}
"#;

/// Fragment shader for wireframe rendering
pub const WIREFRAME_FRAGMENT: &str = r#"#version 300 es
precision highp float;

uniform vec3 u_color;
uniform float u_alpha;

out vec4 fragColor;

void main() {
    fragColor = vec4(u_color, u_alpha);
}
"#;

/// Vertex shader for face ID picking
pub const PICKING_VERTEX: &str = r#"#version 300 es
precision highp float;

in vec3 a_position;
in float a_face_id;

uniform mat4 u_mvp;

flat out float v_face_id;

void main() {
    v_face_id = a_face_id;
    gl_Position = u_mvp * vec4(a_position, 1.0);
}
"#;

/// Fragment shader for face ID picking (encodes ID as RGB)
pub const PICKING_FRAGMENT: &str = r#"#version 300 es
precision highp float;

flat in float v_face_id;

out vec4 fragColor;

void main() {
    // Encode face ID as RGB color (supports up to 16.7M faces)
    // ID 0 = background (no selection)
    uint id = uint(v_face_id) + 1u;
    float r = float((id >> 16u) & 0xFFu) / 255.0;
    float g = float((id >> 8u) & 0xFFu) / 255.0;
    float b = float(id & 0xFFu) / 255.0;

    fragColor = vec4(r, g, b, 1.0);
}
"#;
