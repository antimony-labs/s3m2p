//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: render_gl.rs | HELIOS/src/render_gl.rs
//! PURPOSE: Complete WebGL2 rendering engine for solar system visualization
//! MODIFIED: 2026-01-03
//! LAYER: HELIOS (simulation)
//! ═══════════════════════════════════════════════════════════════════════════════

#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use crate::heliosphere::{HeliosphereMorphology, HeliosphereParameters, HeliosphereSurface};
use crate::simulation::{SimulationState, ORBIT_SEGMENTS};
use js_sys::Float32Array;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject
};

const PI: f32 = std::f32::consts::PI;

// ============================================================================
// SHADER SOURCES
// ============================================================================

// Point shader - for stars, asteroids, oort cloud particles
const POINT_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
in float a_size;
in vec3 a_color;
uniform mat4 u_matrix;
uniform float u_point_scale;
out vec3 v_color;
out float v_size;
void main() {
    gl_Position = u_matrix * vec4(a_position, 0.0, 1.0);
    // Clamp point size to reasonable range (1-8 pixels)
    gl_PointSize = clamp(a_size * u_point_scale, 1.0, 8.0);
    v_color = a_color;
    v_size = gl_PointSize;
}
"#;

const POINT_FRAG: &str = r#"#version 300 es
precision highp float;
in vec3 v_color;
in float v_size;
out vec4 fragColor;
void main() {
    float d = length(gl_PointCoord - 0.5);
    if (d > 0.5) discard;
    // Crisp points for small sizes, slight softness for larger
    float alpha = v_size < 3.0 ? 1.0 : (1.0 - smoothstep(0.4, 0.5, d));
    fragColor = vec4(v_color, alpha * 0.9);
}
"#;

// Line shader - for orbits and trails
const LINE_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
uniform mat4 u_matrix;
void main() {
    gl_Position = u_matrix * vec4(a_position, 0.0, 1.0);
}
"#;

const LINE_FRAG: &str = r#"#version 300 es
precision highp float;
uniform vec4 u_color;
out vec4 fragColor;
void main() {
    fragColor = u_color;
}
"#;

// Circle shader - for sun, planets, moons (filled circles with glow)
const CIRCLE_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
uniform vec2 u_center;
uniform float u_radius;
uniform mat4 u_matrix;
out vec2 v_uv;
void main() {
    vec2 worldPos = u_center + a_position * u_radius;
    gl_Position = u_matrix * vec4(worldPos, 0.0, 1.0);
    v_uv = a_position;
}
"#;

const CIRCLE_FRAG: &str = r#"#version 300 es
precision highp float;
uniform vec3 u_color;
uniform float u_glow;
in vec2 v_uv;
out vec4 fragColor;
void main() {
    float d = length(v_uv);
    if (d > 1.0) discard;

    // Sharp edge with subtle anti-aliasing
    float alpha = 1.0 - smoothstep(0.95, 1.0, d);

    // Subtle glow effect (mostly for sun)
    float glow = exp(-d * 3.0) * u_glow;
    vec3 color = u_color + vec3(glow * 0.5);

    fragColor = vec4(color, alpha);
}
"#;

// ============================================================================
// MILKY WAY GALAXY BACKGROUND SHADER
// ============================================================================
// Procedural shader that renders:
// 1. Dense star field with realistic distribution
// 2. Milky Way band tilted 60° from ecliptic plane
// 3. Subtle galactic center glow
// All coordinates in HCI (Heliocentric Inertial) frame

const MILKYWAY_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
out vec2 v_uv;
void main() {
    v_uv = a_position * 0.5 + 0.5;
    gl_Position = vec4(a_position, 0.999, 1.0); // Far Z for background
}
"#;

const MILKYWAY_FRAG: &str = r#"#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_camera_azimuth;
uniform float u_camera_elevation;
uniform float u_zoom;

in vec2 v_uv;
out vec4 fragColor;

// High-quality hash functions
float hash(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float hash2(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

// Noise for milky way structure
float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));
    return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}

float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    for (int i = 0; i < 4; i++) {
        v += a * noise(p);
        p *= 2.0;
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = v_uv;

    // Galactic coordinates - 60 degree tilt from ecliptic
    float galTilt = 1.047;
    vec2 centered = uv - 0.5;
    float rotAngle = galTilt + u_camera_azimuth * 0.02;
    float c = cos(rotAngle);
    float s = sin(rotAngle);
    float galY = centered.x * s + centered.y * c;  // Distance from galactic plane
    float galX = centered.x * c - centered.y * s;  // Along galactic plane

    // Deep space background
    vec3 bgColor = vec3(0.0, 0.002, 0.008);

    // ========== STAR FIELD ==========
    vec3 stars = vec3(0.0);

    // Galactic plane density multiplier - MORE stars near the milky way
    float galacticDensity = 1.0 + 4.0 * exp(-galY * galY / 0.03);

    // Layer 1: Thousands of faint background stars
    for (int layer = 0; layer < 5; layer++) {
        float scale = 300.0 + float(layer) * 200.0;
        vec2 gridUV = uv * scale;
        vec2 gridId = floor(gridUV);
        vec2 gridFract = fract(gridUV);

        float h = hash(gridId + float(layer) * 127.0);

        // Base 40% density, boosted near galactic plane
        float threshold = 0.6 - 0.3 * exp(-galY * galY / 0.05);

        if (h > threshold) {
            vec2 starPos = vec2(hash(gridId * 1.3), hash(gridId * 2.7)) * 0.8 + 0.1;
            float d = length(gridFract - starPos);
            float brightness = smoothstep(0.015, 0.0, d);

            // Vary brightness
            float mag = 0.15 + h * 0.25;

            // Subtle color variation
            vec3 col = mix(vec3(0.8, 0.85, 1.0), vec3(1.0, 0.95, 0.9), hash(gridId * 3.3));
            stars += col * brightness * mag;
        }
    }

    // Layer 2: Medium stars with colors
    for (int layer = 0; layer < 4; layer++) {
        float scale = 120.0 + float(layer) * 80.0;
        vec2 gridUV = uv * scale;
        vec2 gridId = floor(gridUV);
        vec2 gridFract = fract(gridUV);

        float h = hash2(gridId + float(layer) * 89.0);

        // More stars near galactic plane
        float threshold = 0.65 - 0.25 * exp(-galY * galY / 0.04);

        if (h > threshold) {
            vec2 starPos = vec2(hash2(gridId * 1.7), hash2(gridId * 2.3)) * 0.7 + 0.15;
            float d = length(gridFract - starPos);
            float brightness = smoothstep(0.02, 0.0, d);

            // Star colors based on spectral type
            float temp = hash(gridId * 4.1);
            vec3 starCol;
            if (temp < 0.1) {
                starCol = vec3(1.0, 0.4, 0.2);  // M - Red
            } else if (temp < 0.25) {
                starCol = vec3(1.0, 0.6, 0.4);  // K - Orange
            } else if (temp < 0.5) {
                starCol = vec3(1.0, 0.9, 0.7);  // G - Yellow
            } else if (temp < 0.75) {
                starCol = vec3(1.0, 1.0, 1.0);  // F/A - White
            } else {
                starCol = vec3(0.7, 0.8, 1.0);  // B/O - Blue
            }

            float twinkle = 0.8 + 0.2 * sin(u_time * 2.0 + h * 50.0);
            stars += starCol * brightness * twinkle * (0.4 + h * 0.4);
        }
    }

    // Layer 3: Bright prominent stars
    for (int layer = 0; layer < 2; layer++) {
        float scale = 50.0 + float(layer) * 30.0;
        vec2 gridUV = uv * scale;
        vec2 gridId = floor(gridUV);
        vec2 gridFract = fract(gridUV);

        float h = hash(gridId + float(layer) * 199.0);

        if (h > 0.88) {
            vec2 starPos = vec2(hash(gridId * 1.9), hash(gridId * 2.1)) * 0.6 + 0.2;
            float d = length(gridFract - starPos);

            // Core + glow
            float core = smoothstep(0.035, 0.0, d);
            float glow = smoothstep(0.1, 0.0, d) * 0.4;
            float brightness = core + glow;

            // Bright star colors
            float temp = hash(gridId * 5.7);
            vec3 starCol;
            if (temp < 0.15) {
                starCol = vec3(1.0, 0.5, 0.3);  // Orange giant
            } else if (temp < 0.4) {
                starCol = vec3(1.0, 1.0, 0.85); // Yellow-white
            } else if (temp < 0.7) {
                starCol = vec3(1.0, 1.0, 1.0);  // White
            } else {
                starCol = vec3(0.75, 0.85, 1.0); // Blue-white
            }

            float twinkle = 0.85 + 0.15 * sin(u_time * 1.5 + h * 40.0);
            stars += starCol * brightness * twinkle * 1.0;
        }
    }

    // ========== MILKY WAY BAND ==========
    // Main band - gaussian with structure
    float bandWidth = 0.08;
    float band = exp(-galY * galY / (2.0 * bandWidth * bandWidth));

    // Add cloudy structure using fractal noise
    float cloudNoise = fbm(vec2(galX * 8.0, galY * 15.0 + u_time * 0.01));
    float dustLanes = fbm(vec2(galX * 12.0 + 100.0, galY * 20.0));

    // Dark dust lanes cutting through
    float dust = smoothstep(0.3, 0.6, dustLanes);

    // Milky way glow - concentrated star light
    vec3 milkyGlow = vec3(0.6, 0.55, 0.65) * band * (0.5 + 0.5 * cloudNoise) * (0.6 + 0.4 * dust);

    // Extra dense star clusters in the band
    float clusterNoise = fbm(vec2(galX * 20.0, galY * 30.0));
    vec3 starClusters = vec3(0.8, 0.75, 0.85) * band * clusterNoise * 0.4;

    // Galactic center bulge - brighter, slightly orange
    float bulgeX = galX + 0.15; // Offset toward galactic center
    float bulgeDist = length(vec2(bulgeX * 1.5, galY * 2.0));
    float bulge = exp(-bulgeDist * bulgeDist / 0.02);
    vec3 bulgeColor = vec3(0.9, 0.75, 0.5) * bulge * 0.6;

    // Combine milky way
    vec3 milkyWay = milkyGlow * 0.35 + starClusters + bulgeColor;

    // ========== FINAL COMPOSITION ==========
    vec3 finalColor = bgColor + stars + milkyWay;

    // Slight vignette
    float vignette = 1.0 - length(centered) * 0.1;
    finalColor *= vignette;

    // Tone mapping for HDR-like feel
    finalColor = finalColor / (finalColor + 0.5);
    finalColor = pow(finalColor, vec3(0.9));

    fragColor = vec4(finalColor, 1.0);
}
"#;

// Heliosphere volumetric shader
const HELIO_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
out vec2 v_uv;
void main() {
    v_uv = a_position * 0.5 + 0.5;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

const HELIO_FRAG: &str = r#"#version 300 es
precision highp float;

uniform sampler2D u_boundary_tex;
uniform vec2 u_resolution;
uniform float u_time;
uniform float u_max_radius;
uniform float u_zoom;
uniform vec2 u_center;
uniform float u_tilt;
uniform float u_rotation;
uniform int u_steps;
uniform float u_solar_cycle;

in vec2 v_uv;
out vec4 fragColor;

const float PI = 3.14159265359;
const float TEX_MAX_RADIUS = 300.0;

vec2 dirToUV(vec3 dir) {
    dir = normalize(dir);
    float theta = atan(dir.z, dir.x);
    float phi = asin(clamp(dir.y, -1.0, 1.0));
    return vec2(theta / (2.0 * PI) + 0.5, phi / PI + 0.5);
}

vec3 getBoundaries(vec3 pos) {
    vec2 uv = dirToUV(pos);
    vec4 tex = texture(u_boundary_tex, uv);
    return tex.rgb * TEX_MAX_RADIUS;
}

float hash(vec3 p) {
    p = fract(p * 0.3183099 + 0.1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float noise3D(vec3 p) {
    vec3 i = floor(p);
    vec3 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(mix(hash(i), hash(i + vec3(1,0,0)), f.x),
            mix(hash(i + vec3(0,1,0)), hash(i + vec3(1,1,0)), f.x), f.y),
        mix(mix(hash(i + vec3(0,0,1)), hash(i + vec3(1,0,1)), f.x),
            mix(hash(i + vec3(0,1,1)), hash(i + vec3(1,1,1)), f.x), f.y), f.z);
}

void main() {
    vec2 ndc = (v_uv * 2.0 - 1.0);
    ndc.x *= u_resolution.x / u_resolution.y;

    vec2 halfRes = u_resolution * 0.5;
    vec2 worldXY = ndc * halfRes * u_zoom + u_center;

    float cosTilt = cos(u_tilt);
    float sinTilt = sin(u_tilt);
    float cosRot = cos(u_rotation);
    float sinRot = sin(u_rotation);

    vec2 rotatedXY = vec2(
        worldXY.x * cosRot - worldXY.y * sinRot,
        worldXY.x * sinRot + worldXY.y * cosRot
    );

    float cameraHeight = u_max_radius * 2.5;
    vec3 rayOrigin = vec3(
        rotatedXY.x,
        cameraHeight * cosTilt + rotatedXY.y * sinTilt,
        rotatedXY.y * cosTilt - cameraHeight * sinTilt
    );

    vec3 rayDir = normalize(vec3(0.0, -cosTilt, sinTilt));

    vec3 color = vec3(0.0);
    float alpha = 0.0;
    float maxDist = u_max_radius * 5.0;
    float stepSize = maxDist / float(u_steps);

    for (int i = 0; i < 128; i++) {
        if (i >= u_steps) break;

        float t = float(i) * stepSize;
        vec3 pos = rayOrigin + rayDir * t;
        float r = length(pos);

        if (r > u_max_radius * 1.5) continue;
        if (r < 3.0) continue;

        vec3 boundaries = getBoundaries(pos);
        float ts = boundaries.r;
        float hp = boundaries.g;
        float bow = boundaries.b;

        float density = 0.0;
        vec3 regionColor = vec3(0.0);

        if (r < ts) {
            float f = r / ts;
            density = 0.015 * (1.0 - f * f);
            float angle = atan(pos.z, pos.x);
            float streaks = 0.5 + 0.5 * sin(angle * 6.0 + u_time * 0.3);
            regionColor = mix(vec3(1.0, 0.8, 0.2), vec3(1.0, 0.5, 0.1), streaks * 0.4);
            density *= 0.8 + 0.2 * sin(u_solar_cycle * 2.0 * PI);
        } else if (r < hp) {
            float f = (r - ts) / (hp - ts);
            density = 0.025 * (1.0 - abs(f - 0.5) * 1.8);
            float turb = noise3D(pos * 0.08 + vec3(u_time * 0.05));
            regionColor = mix(vec3(0.1, 0.5, 0.9), vec3(0.3, 0.7, 1.0), turb);
            density *= 0.7 + 0.5 * turb;
        } else if (r < bow) {
            float f = (r - hp) / (bow - hp);
            density = 0.015 * (1.0 - f);
            regionColor = mix(vec3(0.5, 0.1, 0.7), vec3(0.7, 0.2, 0.5), f);
        }

        float shellWidth = 3.0;
        float tsShell = 1.0 - smoothstep(0.0, shellWidth, abs(r - ts));
        if (tsShell > 0.01) {
            density += 0.12 * tsShell;
            regionColor = mix(regionColor, vec3(0.2, 0.9, 1.0), tsShell * 0.9);
        }
        float hpShell = 1.0 - smoothstep(0.0, shellWidth, abs(r - hp));
        if (hpShell > 0.01) {
            density += 0.10 * hpShell;
            regionColor = mix(regionColor, vec3(0.6, 0.2, 1.0), hpShell * 0.9);
        }
        float bowShell = 1.0 - smoothstep(0.0, shellWidth * 1.5, abs(r - bow));
        if (bowShell > 0.01) {
            density += 0.06 * bowShell;
            regionColor = mix(regionColor, vec3(1.0, 0.4, 0.2), bowShell * 0.8);
        }

        float sampleAlpha = density * stepSize * 0.15;
        color += regionColor * sampleAlpha * (1.0 - alpha);
        alpha += sampleAlpha * (1.0 - alpha);

        if (alpha > 0.95) break;
    }

    color = pow(color, vec3(0.85));
    fragColor = vec4(color, alpha);
}
"#;

// ============================================================================
// RENDERER STRUCTURE
// ============================================================================

pub struct RendererGl {
    gl: WebGl2RenderingContext,

    // Point rendering (stars, asteroids, oort)
    point_program: Option<WebGlProgram>,
    point_vao: Option<WebGlVertexArrayObject>,
    point_vbo: Option<WebGlBuffer>,
    point_u_matrix: Option<WebGlUniformLocation>,
    point_u_point_scale: Option<WebGlUniformLocation>,

    // Line rendering (orbits)
    line_program: Option<WebGlProgram>,
    line_vao: Option<WebGlVertexArrayObject>,
    line_vbo: Option<WebGlBuffer>,
    line_u_matrix: Option<WebGlUniformLocation>,
    line_u_color: Option<WebGlUniformLocation>,

    // Circle rendering (sun, planets, moons)
    circle_program: Option<WebGlProgram>,
    circle_vao: Option<WebGlVertexArrayObject>,
    circle_vbo: Option<WebGlBuffer>,
    circle_u_matrix: Option<WebGlUniformLocation>,
    circle_u_center: Option<WebGlUniformLocation>,
    circle_u_radius: Option<WebGlUniformLocation>,
    circle_u_color: Option<WebGlUniformLocation>,
    circle_u_glow: Option<WebGlUniformLocation>,

    // Heliosphere volumetric
    helio_program: Option<WebGlProgram>,
    helio_vao: Option<WebGlVertexArrayObject>,
    helio_vbo: Option<WebGlBuffer>,
    helio_texture: Option<WebGlTexture>,
    helio_u_boundary_tex: Option<WebGlUniformLocation>,
    helio_u_resolution: Option<WebGlUniformLocation>,
    helio_u_time: Option<WebGlUniformLocation>,
    helio_u_max_radius: Option<WebGlUniformLocation>,
    helio_u_zoom: Option<WebGlUniformLocation>,
    helio_u_center: Option<WebGlUniformLocation>,
    helio_u_tilt: Option<WebGlUniformLocation>,
    helio_u_rotation: Option<WebGlUniformLocation>,
    helio_u_steps: Option<WebGlUniformLocation>,
    helio_u_solar_cycle: Option<WebGlUniformLocation>,

    // Milky Way background
    milkyway_program: Option<WebGlProgram>,
    milkyway_vao: Option<WebGlVertexArrayObject>,
    milkyway_vbo: Option<WebGlBuffer>,
    milkyway_u_resolution: Option<WebGlUniformLocation>,
    milkyway_u_time: Option<WebGlUniformLocation>,
    milkyway_u_camera_azimuth: Option<WebGlUniformLocation>,
    milkyway_u_camera_elevation: Option<WebGlUniformLocation>,
    milkyway_u_zoom: Option<WebGlUniformLocation>,

    // State
    heliosphere_params: HeliosphereParameters,
    quality: RenderQuality,
    last_solar_cycle: f32,
    texture_dirty: bool,

    // Buffers for dynamic data
    orbit_buffer: Vec<f32>,
    point_buffer: Vec<f32>,
}

#[derive(Clone, Copy, Debug)]
pub enum RenderQuality {
    Low,
    Medium,
    High,
}

impl RenderQuality {
    pub fn raymarch_steps(&self) -> i32 {
        match self {
            RenderQuality::Low => 32,
            RenderQuality::Medium => 64,
            RenderQuality::High => 96,
        }
    }

    pub fn texture_size(&self) -> (u32, u32) {
        match self {
            RenderQuality::Low => (128, 64),
            RenderQuality::Medium => (256, 128),
            RenderQuality::High => (512, 256),
        }
    }
}

impl RendererGl {
    pub fn new(gl: WebGl2RenderingContext) -> Result<Self, String> {
        let quality = if let Some(window) = web_sys::window() {
            let width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(1920.0);
            if width < 768.0 { RenderQuality::Low }
            else if width < 1920.0 { RenderQuality::Medium }
            else { RenderQuality::High }
        } else { RenderQuality::Medium };

        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.02, 1.0);

        let heliosphere_params = HeliosphereParameters::new(
            121.0, 0.78, vec![-1.0, 0.0, 0.0], 0.1, 6300.0, 0.2, 1.0, 400.0,
            HeliosphereMorphology::Croissant, vec![1.5, 0.7, 0.3],
        );

        let mut renderer = Self {
            gl,
            point_program: None, point_vao: None, point_vbo: None,
            point_u_matrix: None, point_u_point_scale: None,
            line_program: None, line_vao: None, line_vbo: None,
            line_u_matrix: None, line_u_color: None,
            circle_program: None, circle_vao: None, circle_vbo: None,
            circle_u_matrix: None, circle_u_center: None, circle_u_radius: None,
            circle_u_color: None, circle_u_glow: None,
            helio_program: None, helio_vao: None, helio_vbo: None, helio_texture: None,
            helio_u_boundary_tex: None, helio_u_resolution: None, helio_u_time: None,
            helio_u_max_radius: None, helio_u_zoom: None, helio_u_center: None,
            helio_u_tilt: None, helio_u_rotation: None, helio_u_steps: None,
            helio_u_solar_cycle: None,
            milkyway_program: None, milkyway_vao: None, milkyway_vbo: None,
            milkyway_u_resolution: None, milkyway_u_time: None,
            milkyway_u_camera_azimuth: None, milkyway_u_camera_elevation: None,
            milkyway_u_zoom: None,
            heliosphere_params, quality, last_solar_cycle: -1.0, texture_dirty: true,
            orbit_buffer: Vec::with_capacity(ORBIT_SEGMENTS * 2 * 8),
            point_buffer: Vec::with_capacity(10000 * 6),
        };

        renderer.init_point_shader()?;
        renderer.init_line_shader()?;
        renderer.init_circle_shader()?;
        renderer.init_helio_shader()?;
        renderer.init_milkyway_shader()?;

        web_sys::console::log_1(&"WebGL2 renderer initialized".into());
        Ok(renderer)
    }

    fn init_point_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, POINT_VERT, POINT_FRAG)?;

        self.point_u_matrix = gl.get_uniform_location(&program, "u_matrix");
        self.point_u_point_scale = gl.get_uniform_location(&program, "u_point_scale");

        let vao = gl.create_vertex_array().ok_or("Failed to create point VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create point VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Layout: x, y, size, r, g, b (6 floats per point)
        let stride = 6 * 4;
        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        let size_loc = gl.get_attrib_location(&program, "a_size") as u32;
        let color_loc = gl.get_attrib_location(&program, "a_color") as u32;

        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(size_loc);
        gl.vertex_attrib_pointer_with_i32(size_loc, 1, WebGl2RenderingContext::FLOAT, false, stride, 8);
        gl.enable_vertex_attrib_array(color_loc);
        gl.vertex_attrib_pointer_with_i32(color_loc, 3, WebGl2RenderingContext::FLOAT, false, stride, 12);

        gl.bind_vertex_array(None);

        self.point_program = Some(program);
        self.point_vao = Some(vao);
        self.point_vbo = Some(vbo);
        Ok(())
    }

    fn init_line_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, LINE_VERT, LINE_FRAG)?;

        self.line_u_matrix = gl.get_uniform_location(&program, "u_matrix");
        self.line_u_color = gl.get_uniform_location(&program, "u_color");

        let vao = gl.create_vertex_array().ok_or("Failed to create line VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create line VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.bind_vertex_array(None);

        self.line_program = Some(program);
        self.line_vao = Some(vao);
        self.line_vbo = Some(vbo);
        Ok(())
    }

    fn init_circle_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, CIRCLE_VERT, CIRCLE_FRAG)?;

        self.circle_u_matrix = gl.get_uniform_location(&program, "u_matrix");
        self.circle_u_center = gl.get_uniform_location(&program, "u_center");
        self.circle_u_radius = gl.get_uniform_location(&program, "u_radius");
        self.circle_u_color = gl.get_uniform_location(&program, "u_color");
        self.circle_u_glow = gl.get_uniform_location(&program, "u_glow");

        let vao = gl.create_vertex_array().ok_or("Failed to create circle VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create circle VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Unit circle quad (-1 to 1)
        let vertices: [f32; 12] = [
            -1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ];
        unsafe {
            let arr = Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::STATIC_DRAW
            );
        }

        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.bind_vertex_array(None);

        self.circle_program = Some(program);
        self.circle_vao = Some(vao);
        self.circle_vbo = Some(vbo);
        Ok(())
    }

    fn init_helio_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, HELIO_VERT, HELIO_FRAG)?;

        self.helio_u_boundary_tex = gl.get_uniform_location(&program, "u_boundary_tex");
        self.helio_u_resolution = gl.get_uniform_location(&program, "u_resolution");
        self.helio_u_time = gl.get_uniform_location(&program, "u_time");
        self.helio_u_max_radius = gl.get_uniform_location(&program, "u_max_radius");
        self.helio_u_zoom = gl.get_uniform_location(&program, "u_zoom");
        self.helio_u_center = gl.get_uniform_location(&program, "u_center");
        self.helio_u_tilt = gl.get_uniform_location(&program, "u_tilt");
        self.helio_u_rotation = gl.get_uniform_location(&program, "u_rotation");
        self.helio_u_steps = gl.get_uniform_location(&program, "u_steps");
        self.helio_u_solar_cycle = gl.get_uniform_location(&program, "u_solar_cycle");

        let vao = gl.create_vertex_array().ok_or("Failed to create helio VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create helio VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Fullscreen triangle
        let vertices: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
        unsafe {
            let arr = Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::STATIC_DRAW
            );
        }

        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.bind_vertex_array(None);

        // Boundary texture
        let texture = gl.create_texture().ok_or("Failed to create helio texture")?;
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        self.helio_program = Some(program);
        self.helio_vao = Some(vao);
        self.helio_vbo = Some(vbo);
        self.helio_texture = Some(texture);
        Ok(())
    }

    fn init_milkyway_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        web_sys::console::log_1(&"Initializing Milky Way shader...".into());

        let program = match create_program(gl, MILKYWAY_VERT, MILKYWAY_FRAG) {
            Ok(p) => {
                web_sys::console::log_1(&"Milky Way shader compiled successfully".into());
                p
            },
            Err(e) => {
                web_sys::console::error_1(&format!("Milky Way shader FAILED: {}", e).into());
                return Err(e);
            }
        };

        self.milkyway_u_resolution = gl.get_uniform_location(&program, "u_resolution");
        self.milkyway_u_time = gl.get_uniform_location(&program, "u_time");
        self.milkyway_u_camera_azimuth = gl.get_uniform_location(&program, "u_camera_azimuth");
        self.milkyway_u_camera_elevation = gl.get_uniform_location(&program, "u_camera_elevation");
        self.milkyway_u_zoom = gl.get_uniform_location(&program, "u_zoom");

        let vao = gl.create_vertex_array().ok_or("Failed to create milkyway VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create milkyway VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Fullscreen triangle (more efficient than quad)
        let vertices: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
        unsafe {
            let arr = Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::STATIC_DRAW
            );
        }

        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.bind_vertex_array(None);

        self.milkyway_program = Some(program);
        self.milkyway_vao = Some(vao);
        self.milkyway_vbo = Some(vbo);

        web_sys::console::log_1(&"Milky Way shader initialized".into());
        Ok(())
    }

    fn update_helio_texture(&mut self, state: &SimulationState) -> Result<(), String> {
        let gl = &self.gl;
        let (width, height) = self.quality.texture_size();

        self.heliosphere_params.r_hp_nose = state.heliopause_au as f32;
        self.heliosphere_params.r_ts_over_hp = (state.termination_shock_au / state.heliopause_au) as f32;

        let surface = HeliosphereSurface::new(self.heliosphere_params.clone());
        let mut data: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            let phi = (y as f32 / height as f32) * PI;
            for x in 0..width {
                let theta = (x as f32 / width as f32) * 2.0 * PI;
                let r_ts = surface.termination_shock_radius(theta, phi);
                let r_hp = surface.heliopause_radius(theta, phi);
                let r_bow = r_hp * 1.9;

                data.push(((r_ts / 300.0).min(1.0) * 255.0) as u8);
                data.push(((r_hp / 300.0).min(1.0) * 255.0) as u8);
                data.push(((r_bow / 300.0).min(1.0) * 255.0) as u8);
                data.push(255);
            }
        }

        if let Some(texture) = &self.helio_texture {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D, 0, WebGl2RenderingContext::RGBA as i32,
                width as i32, height as i32, 0, WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE, Some(&data)
            ).map_err(|e| format!("Texture upload failed: {:?}", e))?;
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        }

        self.texture_dirty = false;
        self.last_solar_cycle = state.solar_cycle_phase as f32;
        Ok(())
    }

    // Build orthographic projection matrix for AU coordinates
    fn build_matrix(&self, state: &SimulationState) -> [f32; 16] {
        let view = &state.view;
        let half_w = (view.width * view.zoom / 2.0) as f32;
        let half_h = (view.height * view.zoom / 2.0) as f32;
        let cx = view.center_x as f32;
        let cy = view.center_y as f32;

        // Orthographic projection: world AU -> NDC
        let sx = 1.0 / half_w;
        let sy = 1.0 / half_h;

        // Include tilt (simple Y-axis shear for pseudo-3D)
        let tilt = view.tilt as f32;
        let cos_t = tilt.cos();

        [
            sx, 0.0, 0.0, 0.0,
            0.0, sy * cos_t, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -cx * sx, -cy * sy * cos_t, 0.0, 1.0,
        ]
    }

    // ========================================================================
    // RENDER PASSES
    // ========================================================================

    pub fn render(&mut self, state: &SimulationState, time: f64) {
        let gl = &self.gl;

        let dpr = web_sys::window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0) as f32;
        let width = (state.view.width as f32 * dpr) as i32;
        let height = (state.view.height as f32 * dpr) as i32;

        gl.viewport(0, 0, width, height);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let matrix = self.build_matrix(state);
        let is_helio_view = state.camera.scale_level == crate::cca_projection::ScaleLevel::Heliosphere;

        // Update heliosphere texture if needed
        if is_helio_view {
            let solar_diff = (state.solar_cycle_phase as f32 - self.last_solar_cycle).abs();
            if self.texture_dirty || solar_diff > 0.01 {
                let _ = self.update_helio_texture(state);
            }
        }

        // Render order: back to front
        // 1. Milky Way background (always behind everything)
        self.render_milkyway(state, time);

        // 2. Catalog stars (on celestial sphere)
        self.render_stars(state, &matrix, time);

        // 3. Heliosphere boundaries (semi-transparent overlay)
        if is_helio_view {
            self.render_heliosphere(state, time);
        }

        // 4. Solar system objects
        self.render_oort_cloud(state, &matrix);
        self.render_orbits(state, &matrix);
        self.render_asteroid_belt(state, &matrix);
        self.render_sun(state, &matrix, time);
        self.render_planets(state, &matrix, time);
        self.render_moons(state, &matrix);
    }

    fn render_milkyway(&self, state: &SimulationState, time: f64) {
        let gl = &self.gl;

        // Log once per second to avoid spam
        static mut LAST_LOG: f64 = 0.0;
        unsafe {
            if time - LAST_LOG > 1.0 {
                web_sys::console::log_1(&format!(
                    "render_milkyway called: program={}, vao={}",
                    self.milkyway_program.is_some(),
                    self.milkyway_vao.is_some()
                ).into());
                LAST_LOG = time;
            }
        }

        if let (Some(program), Some(vao)) = (&self.milkyway_program, &self.milkyway_vao) {
            // Disable depth test for background
            gl.disable(WebGl2RenderingContext::DEPTH_TEST);

            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));

            // Pass uniforms
            gl.uniform2f(
                self.milkyway_u_resolution.as_ref(),
                state.view.width as f32,
                state.view.height as f32
            );
            gl.uniform1f(self.milkyway_u_time.as_ref(), time as f32);
            gl.uniform1f(self.milkyway_u_camera_azimuth.as_ref(), state.camera.azimuth as f32);
            gl.uniform1f(self.milkyway_u_camera_elevation.as_ref(), state.camera.elevation as f32);
            gl.uniform1f(self.milkyway_u_zoom.as_ref(), state.view.zoom as f32);

            // Draw fullscreen triangle
            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

            gl.bind_vertex_array(None);

            // Re-enable depth test for subsequent renders
            gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        } else {
            web_sys::console::warn_1(&"Milky Way shader not initialized".into());
        }
    }

    fn render_stars(&mut self, state: &SimulationState, matrix: &[f32; 16], _time: f64) {
        let gl = &self.gl;

        let stars = state.star_mgr.visible_instances();
        if stars.is_empty() { return; }

        // Sky dome radius - scales with zoom so stars always appear at "infinity"
        // This ensures stars are visible at any zoom level
        let sky_dome_radius = state.view.zoom * state.view.height.max(state.view.width) * 0.8;

        self.point_buffer.clear();
        for star in stars {
            // Normalize position to get direction (unit vector)
            let len = (star.position.x.powi(2) + star.position.y.powi(2) + star.position.z.powi(2)).sqrt();
            if len < 0.001 { continue; } // Skip degenerate positions

            let dir_x = star.position.x / len;
            let dir_y = star.position.y / len;
            let dir_z = star.position.z / len;

            // Place on celestial sphere at sky dome radius
            // Apply camera tilt to Z coordinate
            let cos_tilt = state.view.tilt.cos();
            let sin_tilt = state.view.tilt.sin();

            // Project 3D direction to 2D screen position
            // X stays as-is, Y is combination of Y and Z based on tilt
            let ax = dir_x * sky_dome_radius + state.view.center_x;
            let ay = (dir_y * cos_tilt - dir_z * sin_tilt) * sky_dome_radius + state.view.center_y;

            let size = (6.0 - star.magnitude as f64).max(1.0).min(8.0) as f32;
            // Convert RGB array to floats
            let r = star.color_rgb[0] as f32 / 255.0;
            let g = star.color_rgb[1] as f32 / 255.0;
            let b = star.color_rgb[2] as f32 / 255.0;

            self.point_buffer.extend_from_slice(&[ax as f32, ay as f32, size, r, g, b]);
        }

        if let (Some(program), Some(vao), Some(vbo)) = (&self.point_program, &self.point_vao, &self.point_vbo) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));

            unsafe {
                let arr = Float32Array::view(&self.point_buffer);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                );
            }

            gl.uniform_matrix4fv_with_f32_array(self.point_u_matrix.as_ref(), false, matrix);
            gl.uniform1f(self.point_u_point_scale.as_ref(), 1.0 / state.view.zoom as f32);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 6) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_orbits(&mut self, state: &SimulationState, matrix: &[f32; 16]) {
        let gl = &self.gl;

        if let (Some(program), Some(vao), Some(vbo)) = (&self.line_program, &self.line_vao, &self.line_vbo) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));
            gl.uniform_matrix4fv_with_f32_array(self.line_u_matrix.as_ref(), false, matrix);

            let cos_tilt = state.view.tilt.cos();

            // Orbit colors per planet
            let orbit_colors: [(f32, f32, f32); 8] = [
                (0.5, 0.5, 0.4),   // Mercury
                (0.6, 0.5, 0.4),   // Venus
                (0.3, 0.5, 0.7),   // Earth
                (0.6, 0.4, 0.3),   // Mars
                (0.5, 0.45, 0.35), // Jupiter
                (0.55, 0.5, 0.4),  // Saturn
                (0.4, 0.5, 0.55),  // Uranus
                (0.35, 0.4, 0.6),  // Neptune
            ];

            for i in 0..state.planet_count {
                let orbit = &state.planet_orbits[i];
                let (base_r, base_g, base_b) = orbit_colors.get(i).copied().unwrap_or((0.4, 0.4, 0.5));

                // Draw BACK half first (dimmer) - from PI to 2*PI
                self.orbit_buffer.clear();
                for j in 0..=ORBIT_SEGMENTS / 2 {
                    let angle = std::f64::consts::PI + (j as f64 / (ORBIT_SEGMENTS / 2) as f64) * std::f64::consts::PI;
                    let r = orbit.a * (1.0 - orbit.e * orbit.e) / (1.0 + orbit.e * angle.cos());
                    let x = r * angle.cos();
                    let z = r * angle.sin();
                    let y = z * cos_tilt;
                    self.orbit_buffer.push(x as f32);
                    self.orbit_buffer.push(y as f32);
                }
                unsafe {
                    let arr = Float32Array::view(&self.orbit_buffer);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                    );
                }
                // Back half is dimmer (0.15 alpha)
                gl.uniform4f(self.line_u_color.as_ref(), base_r * 0.5, base_g * 0.5, base_b * 0.5, 0.25);
                gl.draw_arrays(WebGl2RenderingContext::LINE_STRIP, 0, (ORBIT_SEGMENTS / 2 + 1) as i32);

                // Draw FRONT half (brighter) - from 0 to PI
                self.orbit_buffer.clear();
                for j in 0..=ORBIT_SEGMENTS / 2 {
                    let angle = (j as f64 / (ORBIT_SEGMENTS / 2) as f64) * std::f64::consts::PI;
                    let r = orbit.a * (1.0 - orbit.e * orbit.e) / (1.0 + orbit.e * angle.cos());
                    let x = r * angle.cos();
                    let z = r * angle.sin();
                    let y = z * cos_tilt;
                    self.orbit_buffer.push(x as f32);
                    self.orbit_buffer.push(y as f32);
                }
                unsafe {
                    let arr = Float32Array::view(&self.orbit_buffer);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                    );
                }
                // Front half is brighter (0.5 alpha)
                gl.uniform4f(self.line_u_color.as_ref(), base_r, base_g, base_b, 0.5);
                gl.draw_arrays(WebGl2RenderingContext::LINE_STRIP, 0, (ORBIT_SEGMENTS / 2 + 1) as i32);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_sun(&self, state: &SimulationState, matrix: &[f32; 16], time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_matrix.as_ref(), false, matrix);

            // Sun at origin with pulsating size
            let base_radius = 0.00465; // Solar radius in AU
            let pulse = 1.0 + 0.05 * (time * 0.5).sin() as f32;
            // Much smaller minimum - just visible as a dot
            let radius = (base_radius as f32 * pulse).max(state.view.zoom as f32 * 0.5);

            gl.uniform2f(self.circle_u_center.as_ref(), 0.0, 0.0);
            gl.uniform1f(self.circle_u_radius.as_ref(), radius);
            gl.uniform3f(self.circle_u_color.as_ref(), 1.0, 0.9, 0.3);
            gl.uniform1f(self.circle_u_glow.as_ref(), 0.3); // Subtle glow

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            gl.bind_vertex_array(None);
        }
    }

    fn render_planets(&self, state: &SimulationState, matrix: &[f32; 16], _time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_matrix.as_ref(), false, matrix);

            for i in 0..state.planet_count {
                let x = state.planet_x[i] as f32;
                let y = (state.planet_y[i] * state.view.tilt.cos()) as f32;

                // Planet radius - keep small but visible
                // Min radius in world units = zoom * 0.3 (much smaller than before)
                let au_km = 149597870.7;
                let radius_au = (state.planet_radii_km[i] / au_km) as f32;
                let min_radius = (state.view.zoom * 0.3) as f32;
                let radius = radius_au.max(min_radius);

                let (r, g, b) = parse_color(state.planet_colors[i]);

                gl.uniform2f(self.circle_u_center.as_ref(), x, y);
                gl.uniform1f(self.circle_u_radius.as_ref(), radius);
                gl.uniform3f(self.circle_u_color.as_ref(), r, g, b);
                gl.uniform1f(self.circle_u_glow.as_ref(), 0.0); // No glow - crisp planets

                gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_moons(&self, state: &SimulationState, matrix: &[f32; 16]) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_matrix.as_ref(), false, matrix);

            for i in 0..state.moon_count {
                let parent_idx = state.moon_parent_planet[i];
                if parent_idx >= state.planet_count { continue; }

                // Use pre-computed world coordinates
                let mx = state.moon_world_x[i] as f32;
                let my = (state.moon_world_y[i] * state.view.tilt.cos()) as f32;

                // Moons are tiny - much smaller than planets
                let radius = (state.view.zoom * 0.15) as f32;

                gl.uniform2f(self.circle_u_center.as_ref(), mx, my);
                gl.uniform1f(self.circle_u_radius.as_ref(), radius);
                gl.uniform3f(self.circle_u_color.as_ref(), 0.7, 0.7, 0.7);
                gl.uniform1f(self.circle_u_glow.as_ref(), 0.0);

                gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_asteroid_belt(&mut self, state: &SimulationState, matrix: &[f32; 16]) {
        let gl = &self.gl;

        // Only render when zoomed in enough to see the belt
        if state.view.zoom > 0.5 { return; }

        self.point_buffer.clear();
        let cos_tilt = state.view.tilt.cos() as f32;

        // Sample subset for performance
        let step = if state.view.zoom > 0.05 { 4 } else if state.view.zoom > 0.01 { 2 } else { 1 };

        for i in (0..state.asteroid_count).step_by(step) {
            let r = state.asteroid_distances[i] as f32;
            let angle = state.asteroid_angles[i] as f32;
            let incl = state.asteroid_inclinations[i] as f32;

            // 3D position
            let x = r * angle.cos();
            let z = r * angle.sin();
            let y_offset = r * incl.sin() * 0.1; // Small vertical scatter

            // Project to 2D with tilt
            let y = z * cos_tilt + y_offset;

            // Tiny fixed size - asteroids should be 1-2 pixel dots
            let size = 1.0 + ((i as f32 * 1.618).fract() * 1.5);

            // Brownish-gray color with slight variation
            let v = (i as f32 * 0.7182).fract();
            let r_col = 0.5 + v * 0.2;
            let g_col = 0.45 + v * 0.15;
            let b_col = 0.35 + v * 0.1;

            self.point_buffer.extend_from_slice(&[x, y, size, r_col, g_col, b_col]);
        }

        if self.point_buffer.is_empty() { return; }

        if let (Some(program), Some(vao), Some(vbo)) = (&self.point_program, &self.point_vao, &self.point_vbo) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));

            unsafe {
                let arr = Float32Array::view(&self.point_buffer);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                );
            }

            gl.uniform_matrix4fv_with_f32_array(self.point_u_matrix.as_ref(), false, matrix);
            // Fixed small point scale - asteroids are tiny!
            gl.uniform1f(self.point_u_point_scale.as_ref(), 1.0);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 6) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_oort_cloud(&mut self, state: &SimulationState, matrix: &[f32; 16]) {
        let gl = &self.gl;

        // Only render at far zoom levels
        if state.view.zoom < 10.0 { return; }

        self.point_buffer.clear();

        for i in 0..state.oort_count {
            let r = state.oort_distances[i] as f32;
            let theta = state.oort_theta[i] as f32;
            let phi = state.oort_phi[i] as f32;

            let x = r * theta.sin() * phi.cos();
            let y = r * theta.sin() * phi.sin() * (state.view.tilt.cos() as f32);

            // Fade with distance
            let alpha = (1.0 - r / 100000.0).max(0.1);
            self.point_buffer.extend_from_slice(&[x, y, 1.0, 0.3 * alpha, 0.3 * alpha, 0.4 * alpha]);
        }

        if self.point_buffer.is_empty() { return; }

        if let (Some(program), Some(vao), Some(vbo)) = (&self.point_program, &self.point_vao, &self.point_vbo) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));

            unsafe {
                let arr = Float32Array::view(&self.point_buffer);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                );
            }

            gl.uniform_matrix4fv_with_f32_array(self.point_u_matrix.as_ref(), false, matrix);
            gl.uniform1f(self.point_u_point_scale.as_ref(), 0.5 / state.view.zoom as f32);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 6) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_heliosphere(&self, state: &SimulationState, time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao), Some(texture)) = (&self.helio_program, &self.helio_vao, &self.helio_texture) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));

            gl.active_texture(WebGl2RenderingContext::TEXTURE0);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
            gl.uniform1i(self.helio_u_boundary_tex.as_ref(), 0);

            gl.uniform2f(self.helio_u_resolution.as_ref(), state.view.width as f32, state.view.height as f32);
            gl.uniform1f(self.helio_u_time.as_ref(), time as f32);
            gl.uniform1f(self.helio_u_max_radius.as_ref(), (state.heliopause_au * 1.9) as f32);
            gl.uniform1f(self.helio_u_zoom.as_ref(), state.view.zoom as f32);
            gl.uniform2f(self.helio_u_center.as_ref(), state.view.center_x as f32, state.view.center_y as f32);
            gl.uniform1f(self.helio_u_tilt.as_ref(), state.view.tilt as f32);
            gl.uniform1f(self.helio_u_rotation.as_ref(), state.view.rotation as f32);
            gl.uniform1i(self.helio_u_steps.as_ref(), self.quality.raymarch_steps());
            gl.uniform1f(self.helio_u_solar_cycle.as_ref(), state.solar_cycle_phase as f32);

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

            gl.bind_vertex_array(None);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        }
    }

    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.quality = quality;
        self.texture_dirty = true;
    }

    pub fn quality(&self) -> RenderQuality {
        self.quality
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_program(gl: &WebGl2RenderingContext, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String> {
    let vert = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;
    let frag = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;
    link_program(gl, &vert, &frag)
}

fn compile_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<web_sys::WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Failed to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        let log = gl.get_shader_info_log(&shader).unwrap_or_default();
        web_sys::console::error_1(&format!("Shader error: {}", log).into());
        Err(log)
    }
}

fn link_program(gl: &WebGl2RenderingContext, vert: &web_sys::WebGlShader, frag: &web_sys::WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, vert);
    gl.attach_shader(&program, frag);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        let log = gl.get_program_info_log(&program).unwrap_or_default();
        web_sys::console::error_1(&format!("Program error: {}", log).into());
        Err(log)
    }
}

fn parse_color(color: &str) -> (f32, f32, f32) {
    if color.starts_with('#') && color.len() >= 7 {
        let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(255) as f32 / 255.0;
        let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(255) as f32 / 255.0;
        let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(255) as f32 / 255.0;
        (r, g, b)
    } else {
        (1.0, 1.0, 1.0)
    }
}
