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
in vec3 a_position;
in float a_size;
in vec3 a_color;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
uniform float u_point_scale;
out vec3 v_color;
out float v_size;
void main() {
    vec4 view_pos = u_view_matrix * vec4(a_position, 1.0);
    gl_Position = u_projection_matrix * view_pos;
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
in vec3 a_position;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
void main() {
    vec4 view_pos = u_view_matrix * vec4(a_position, 1.0);
    gl_Position = u_projection_matrix * view_pos;
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
// Uses billboard rendering: quad always faces camera
const CIRCLE_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
uniform vec3 u_center;
uniform float u_radius;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
out vec2 v_uv;
void main() {
    // Transform center to view space
    vec4 center_view = u_view_matrix * vec4(u_center, 1.0);
    // Expand quad in view space (perpendicular to camera) for billboard effect
    vec4 offset_view = center_view + vec4(a_position.x * u_radius, a_position.y * u_radius, 0.0, 0.0);
    gl_Position = u_projection_matrix * offset_view;
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
// MILKY WAY PROCEDURAL BACKGROUND SHADER
// ============================================================================
// Renders a dense star field with realistic Milky Way band
// Uses procedural generation for millions of stars on GPU
// Galactic plane tilted 60° from ecliptic plane

const MILKYWAY_VERT: &str = r#"#version 300 es
precision highp float;
in vec2 a_position;
out vec2 v_uv;
void main() {
    v_uv = a_position * 0.5 + 0.5;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

const MILKYWAY_FRAG: &str = r#"#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_zoom;
uniform vec2 u_center;
uniform float u_tilt;
uniform float u_rotation;

in vec2 v_uv;
out vec4 fragColor;

const float PI = 3.14159265359;
const float GALACTIC_TILT = 1.0507;  // 60.2 degrees in radians

// ─────────────────────────────────────────────────────────────────────────────
// NOISE FUNCTIONS
// ─────────────────────────────────────────────────────────────────────────────

float hash21(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p + 34.23);
    return fract(p.x * p.y);
}

float hash31(vec3 p) {
    p = fract(p * vec3(0.1031, 0.1030, 0.0973));
    p += dot(p, p.yxz + 33.33);
    return fract((p.x + p.y) * p.z);
}

vec3 hash33(vec3 p) {
    p = fract(p * vec3(0.1031, 0.1030, 0.0973));
    p += dot(p, p.yxz + 33.33);
    return fract((p.xxy + p.yxx) * p.zyx);
}

// Value noise
float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);

    float a = hash21(i);
    float b = hash21(i + vec2(1.0, 0.0));
    float c = hash21(i + vec2(0.0, 1.0));
    float d = hash21(i + vec2(1.0, 1.0));

    return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}

// Fractal Brownian motion
float fbm(vec2 p, int octaves) {
    float value = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;

    for (int i = 0; i < 6; i++) {
        if (i >= octaves) break;
        value += amplitude * noise(p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}

// ─────────────────────────────────────────────────────────────────────────────
// COORDINATE TRANSFORMATIONS
// ─────────────────────────────────────────────────────────────────────────────

// Convert screen position to a ray direction on celestial sphere
vec3 screenToRay(vec2 uv) {
    // Treat screen as a hemispherical view
    // UV is in range roughly -1 to 1 after aspect correction

    // Create ray direction - looking outward from origin
    // Z is forward (into screen), X is right, Y is up
    vec3 ray = normalize(vec3(uv.x, uv.y, 1.0));

    // Apply camera tilt (rotation around X axis)
    float ct = cos(u_tilt);
    float st = sin(u_tilt);
    ray = vec3(ray.x, ray.y * ct - ray.z * st, ray.y * st + ray.z * ct);

    // Apply camera rotation (rotation around Y axis)
    float cr = cos(u_rotation);
    float sr = sin(u_rotation);
    ray = vec3(ray.x * cr + ray.z * sr, ray.y, -ray.x * sr + ray.z * cr);

    return ray;
}

// Transform from ecliptic (HCI) coordinates to galactic coordinates
// Returns galactic longitude and latitude
vec2 eclipticToGalactic(vec3 eclipticDir) {
    // Galactic north pole in ecliptic coordinates (derived from IAU values)
    // The galactic north pole is at ecliptic coords approximately:
    // longitude ~180°, latitude ~30° (complement of 60° tilt)

    // Rotation matrix from ecliptic to galactic
    // This accounts for:
    // 1. The 60.2° tilt between planes
    // 2. The orientation of galactic center direction

    // Simplified rotation: rotate around X by galactic tilt angle
    // Then adjust longitude offset for galactic center
    float cosG = cos(GALACTIC_TILT);
    float sinG = sin(GALACTIC_TILT);

    // Rotate ecliptic direction to galactic frame
    vec3 galDir;
    galDir.x = eclipticDir.x;
    galDir.y = eclipticDir.y * cosG + eclipticDir.z * sinG;
    galDir.z = -eclipticDir.y * sinG + eclipticDir.z * cosG;

    // Convert to galactic longitude/latitude
    float gal_lon = atan(galDir.y, galDir.x);
    float gal_lat = asin(clamp(galDir.z, -1.0, 1.0));

    return vec2(gal_lon, gal_lat);
}

// Get galactic coordinates for a screen position
vec2 screenToGalactic(vec2 uv) {
    vec3 ray = screenToRay(uv);
    return eclipticToGalactic(ray);
}

// ─────────────────────────────────────────────────────────────────────────────
// STAR FIELD GENERATION
// ─────────────────────────────────────────────────────────────────────────────

// Generate star density at a point
float starField(vec2 uv, float scale, float threshold) {
    vec2 gv = fract(uv * scale) - 0.5;
    vec2 id = floor(uv * scale);

    float d = 1.0;

    // Check 3x3 neighborhood for nearest star
    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            vec2 offset = vec2(float(x), float(y));
            vec2 cellId = id + offset;

            float h = hash21(cellId);
            if (h > threshold) continue;  // No star in this cell

            // Random position within cell
            vec2 starPos = hash33(vec3(cellId, 1.0)).xy - 0.5;
            vec2 toStar = gv - offset - starPos;
            float dist = length(toStar);
            d = min(d, dist);
        }
    }

    return d;
}

// Get star brightness with realistic distribution
float getStarBrightness(vec2 id) {
    float h = hash21(id);
    // Inverse power law - many dim stars, few bright ones
    return pow(h, 3.0) * 2.0;
}

// Get star color based on temperature distribution
vec3 getStarColor(vec2 id) {
    float temp = hash21(id * 7.31);

    if (temp < 0.5) {
        // Red/orange stars (cooler, most common)
        return mix(vec3(1.0, 0.6, 0.4), vec3(1.0, 0.8, 0.6), temp * 2.0);
    } else if (temp < 0.8) {
        // Yellow/white stars
        float t = (temp - 0.5) / 0.3;
        return mix(vec3(1.0, 0.9, 0.7), vec3(1.0, 1.0, 1.0), t);
    } else {
        // Blue-white stars (hot, rare)
        float t = (temp - 0.8) / 0.2;
        return mix(vec3(0.9, 0.95, 1.0), vec3(0.7, 0.8, 1.0), t);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MILKY WAY BAND
// ─────────────────────────────────────────────────────────────────────────────

// Milky Way density based on galactic latitude
float milkyWayDensity(float gal_lat) {
    // Thin disk - main concentration (scale height ~5°)
    float thin = exp(-abs(gal_lat) / 0.087);  // ~5° scale height

    // Thick disk - wider halo (scale height ~15°)
    float thick = 0.3 * exp(-abs(gal_lat) / 0.26);  // ~15° scale height

    return thin + thick;
}

// Dust lane darkness
float dustLanes(vec2 gal_coords) {
    float gal_lon = gal_coords.x;
    float gal_lat = gal_coords.y;

    // Only in the galactic plane
    if (abs(gal_lat) > 0.2) return 1.0;

    // Dust concentrated toward galactic center
    float centerDist = abs(gal_lon);
    float dustBase = smoothstep(1.5, 0.0, centerDist);

    // Filamentary structure
    float filaments = fbm(vec2(gal_lon * 3.0, gal_lat * 20.0), 4);
    filaments = smoothstep(0.3, 0.7, filaments);

    float dust = dustBase * filaments * smoothstep(0.15, 0.0, abs(gal_lat));
    return 1.0 - dust * 0.6;  // Max 60% darkening
}

// Nebula glow color
vec3 nebulaColor(vec2 gal_coords) {
    float gal_lon = gal_coords.x;
    float gal_lat = gal_coords.y;

    // Base color varies with galactic longitude
    float t = (gal_lon / PI + 1.0) * 0.5;  // 0-1 across galaxy

    // Galactic center region (Sagittarius) - golden/orange
    vec3 centerColor = vec3(1.0, 0.7, 0.3);

    // Cygnus region (opposite) - blue/cyan
    vec3 cygnusColor = vec3(0.5, 0.7, 1.0);

    // General band - purple/pink
    vec3 bandColor = vec3(0.7, 0.5, 0.8);

    float centerWeight = exp(-gal_lon * gal_lon * 2.0);
    float cygnusWeight = exp(-(gal_lon - PI) * (gal_lon - PI) * 2.0) +
                         exp(-(gal_lon + PI) * (gal_lon + PI) * 2.0);

    vec3 baseColor = mix(bandColor, centerColor, centerWeight);
    baseColor = mix(baseColor, cygnusColor, cygnusWeight * 0.5);

    return baseColor;
}

// ─────────────────────────────────────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────────────────────────────────────

void main() {
    // Normalize coordinates to -1 to 1
    vec2 uv = v_uv * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;  // Aspect correction

    // Fixed field of view - stars are infinitely far so zoom doesn't change their positions
    // Just scale slightly for visual comfort
    uv *= 1.2;

    // Transform to galactic coordinates
    vec2 gal = screenToGalactic(uv);
    float gal_lon = gal.x;
    float gal_lat = gal.y;

    // ─────────────────────────────────────────
    // LAYER 1: Background color gradient
    // ─────────────────────────────────────────
    vec3 color = vec3(0.0, 0.0, 0.02);  // Deep space black with slight blue

    // ─────────────────────────────────────────
    // LAYER 2: Milky Way diffuse glow
    // ─────────────────────────────────────────
    float mwDensity = milkyWayDensity(gal_lat);

    // Add noise for structure
    float structureNoise = fbm(gal * 3.0 + u_time * 0.001, 5);
    mwDensity *= 0.7 + 0.3 * structureNoise;

    // Apply dust lanes
    float dust = dustLanes(gal);
    mwDensity *= dust;

    // Get nebula color
    vec3 mwColor = nebulaColor(gal);

    // Add diffuse glow - stronger in the band
    color += mwColor * mwDensity * 0.25;

    // Add extra subtle glow for the dense core
    float coreGlow = pow(mwDensity, 2.0) * 0.1;
    color += mwColor * coreGlow;

    // ─────────────────────────────────────────
    // LAYER 3: Dense star field
    // ─────────────────────────────────────────

    vec3 starColors = vec3(0.0);

    // Density boost in Milky Way band
    float bandBoost = 1.0 + mwDensity * 3.0;

    // Scale 1: Bright prominent stars (sparse but visible everywhere)
    float s1 = starField(gal, 30.0, 0.15);
    float b1 = smoothstep(0.04, 0.0, s1) * 1.5;
    vec2 id1 = floor(gal * 30.0);
    starColors += getStarColor(id1) * b1 * (1.0 + getStarBrightness(id1) * 2.0);

    // Scale 2: Medium bright stars
    float s2 = starField(gal, 80.0, 0.2);
    float b2 = smoothstep(0.03, 0.0, s2) * 1.0;
    vec2 id2 = floor(gal * 80.0);
    starColors += getStarColor(id2) * b2;

    // Scale 3: Common stars (more dense)
    float s3 = starField(gal, 200.0, 0.25 * bandBoost);
    float b3 = smoothstep(0.025, 0.0, s3) * 0.8;
    vec2 id3 = floor(gal * 200.0);
    starColors += getStarColor(id3) * b3;

    // Scale 4: Faint stars (dense field)
    float s4 = starField(gal, 500.0, 0.3 * bandBoost);
    float b4 = smoothstep(0.02, 0.0, s4) * 0.5;
    vec2 id4 = floor(gal * 500.0);
    starColors += getStarColor(id4) * b4;

    // Scale 5: Very faint stars (Milky Way emphasis)
    float s5 = starField(gal, 1200.0, 0.35 * bandBoost);
    float b5 = smoothstep(0.015, 0.0, s5) * 0.35;
    starColors += vec3(0.9, 0.95, 1.0) * b5;

    // Scale 6: Tiny point stars (extremely dense in band)
    float s6 = starField(gal, 3000.0, 0.4 * bandBoost);
    float b6 = smoothstep(0.01, 0.0, s6) * 0.25;
    starColors += vec3(0.85, 0.9, 1.0) * b6;

    // Scale 7: Sub-pixel stars creating continuous glow in band
    float s7 = starField(gal * 1.37, 6000.0, 0.5 * mwDensity);
    float b7 = smoothstep(0.008, 0.0, s7) * 0.15;
    starColors += vec3(0.8, 0.85, 0.95) * b7;

    // Apply dust to stars too
    starColors *= dust;

    // Add stars to color
    color += starColors;

    // ─────────────────────────────────────────
    // LAYER 4: Subtle twinkling
    // ─────────────────────────────────────────
    float twinkle = 0.97 + 0.03 * sin(u_time * 2.0 + hash21(gal * 100.0) * 100.0);
    color *= twinkle;

    // ─────────────────────────────────────────
    // Final output
    // ─────────────────────────────────────────

    // Tone mapping for HDR-ish effect
    color = color / (color + 0.5);

    // Slight gamma correction
    color = pow(color, vec3(0.9));

    fragColor = vec4(color, 1.0);
}
"#;

// Heliosphere shader - comet-shaped boundary surfaces
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

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_zoom;
uniform vec2 u_center;
uniform float u_tilt;
uniform float u_rotation;
uniform float u_ts_radius;    // Termination shock radius in AU
uniform float u_hp_radius;    // Heliopause radius in AU
uniform float u_bow_radius;   // Bow shock radius in AU

in vec2 v_uv;
out vec4 fragColor;

const float PI = 3.14159265359;
const int MAX_STEPS = 128;
const float MAX_DIST = 1000.0;
const float SURF_DIST = 1.0;

// Get camera position in 3D space based on tilt and rotation
vec3 getCameraPosition(float dist) {
    // Start with camera on -Z axis
    vec3 camPos = vec3(0.0, 0.0, -dist);

    // Apply tilt (rotation around X axis) - pitch
    float ct = cos(-u_tilt);
    float st = sin(-u_tilt);
    camPos = vec3(camPos.x, camPos.y * ct - camPos.z * st, camPos.y * st + camPos.z * ct);

    // Apply rotation (rotation around Y axis) - yaw
    float cr = cos(-u_rotation);
    float sr = sin(-u_rotation);
    camPos = vec3(camPos.x * cr + camPos.z * sr, camPos.y, -camPos.x * sr + camPos.z * cr);

    return camPos;
}

// Convert screen UV to 3D ray direction
vec3 getRayDirection(vec2 uv, vec3 camPos) {
    // Ray from camera through screen point, looking at origin
    vec3 target = vec3(0.0, 0.0, 0.0);
    vec3 forward = normalize(target - camPos);

    // Camera right and up vectors
    vec3 worldUp = vec3(0.0, 1.0, 0.0);
    vec3 right = normalize(cross(forward, worldUp));
    vec3 up = cross(right, forward);

    // Ray direction through this pixel
    return normalize(forward + uv.x * right + uv.y * up);
}

// 3D comet-shaped SDF
// The heliosphere is centered at origin, nose points in +Z direction
float cometSDF3D(vec3 p, float baseRadius, float noseFactor, float tailFactor) {
    float r = length(p.xy);  // Distance from Z axis
    float z = p.z;

    // Compute angle from +Z axis (nose direction)
    float dist3D = length(p);
    if (dist3D < 0.001) return -baseRadius;

    float cosAngle = z / dist3D;  // cos of angle from nose direction

    // Compute boundary radius at this angle
    float boundaryR;
    if (cosAngle > 0.0) {
        // Nose side - compressed
        boundaryR = baseRadius * (noseFactor + (1.0 - noseFactor) * (1.0 - cosAngle));
    } else {
        // Tail side - extended
        boundaryR = baseRadius * (1.0 + (tailFactor - 1.0) * (-cosAngle));
    }

    return dist3D - boundaryR;
}

// Raymarch to find distance to heliosphere boundaries
float sceneSDF(vec3 p) {
    float ts = cometSDF3D(p, u_ts_radius, 0.7, 2.0);
    float hp = cometSDF3D(p, u_hp_radius, 0.6, 2.5);
    float bow = cometSDF3D(p, u_bow_radius, 0.5, 3.0);
    return min(ts, min(hp, bow));
}

// Raymarch along ray to find intersection
float raymarch(vec3 ro, vec3 rd) {
    float dO = 0.0;

    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * dO;
        float dS = sceneSDF(p);
        dO += dS;
        if(dO > MAX_DIST || abs(dS) < SURF_DIST) break;
    }

    return dO;
}

void main() {
    // Convert UV to NDC
    vec2 uv = v_uv * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;

    // Camera positioned to view the heliosphere
    // Distance from origin based on zoom (larger zoom = farther away to see more)
    float camDist = u_bow_radius * 2.5 * u_zoom;

    // Get camera position (orbits around origin based on tilt/rotation)
    vec3 ro = getCameraPosition(camDist);

    // Get ray direction (camera looks at origin)
    vec3 rd = getRayDirection(uv, ro);

    // Raymarch to find intersection
    float d = raymarch(ro, rd);

    vec3 color = vec3(0.0);
    float alpha = 0.0;

    if (d < MAX_DIST) {
        vec3 p = ro + rd * d;

        // Compute distance to each boundary at this point
        float ts_sdf = cometSDF3D(p, u_ts_radius, 0.7, 2.0);
        float hp_sdf = cometSDF3D(p, u_hp_radius, 0.6, 2.5);
        float bow_sdf = cometSDF3D(p, u_bow_radius, 0.5, 3.0);

        // Shell widths
        float shellWidth = u_ts_radius * 0.15;

        // Bow shock - orange (brightest, widest)
        if (abs(bow_sdf) < shellWidth * 4.0) {
            float t = 1.0 - abs(bow_sdf) / (shellWidth * 4.0);
            color += vec3(0.9, 0.4, 0.2) * t;
            alpha += t * 0.8;
        }

        // Heliopause - purple
        if (abs(hp_sdf) < shellWidth * 3.0) {
            float t = 1.0 - abs(hp_sdf) / (shellWidth * 3.0);
            color += vec3(0.6, 0.3, 0.9) * t * 1.2;
            alpha += t * 0.9;
        }

        // Termination shock - cyan (brightest)
        if (abs(ts_sdf) < shellWidth * 2.0) {
            float t = 1.0 - abs(ts_sdf) / (shellWidth * 2.0);
            color += vec3(0.2, 0.8, 1.0) * t * 1.5;
            alpha += t;
        }

        // Inner solar wind glow
        if (ts_sdf < 0.0) {
            float inner = smoothstep(-u_ts_radius, 0.0, ts_sdf);
            color += vec3(1.0, 0.9, 0.5) * inner * 0.3;
            alpha += inner * 0.4;
        }

        // Heliosheath region
        if (ts_sdf > 0.0 && hp_sdf < 0.0) {
            float sheath = smoothstep(0.0, shellWidth * 4.0, ts_sdf) *
                           smoothstep(0.0, -shellWidth * 4.0, hp_sdf);
            color += vec3(0.3, 0.5, 0.8) * sheath * 0.3;
            alpha += sheath * 0.3;
        }
    } else {
        // Debug: show rays that miss (should be rare)
        // Uncomment to debug: color = vec3(0.1, 0.0, 0.0); alpha = 0.1;
    }

    // Clamp and gamma correct
    alpha = clamp(alpha, 0.0, 0.95);
    color = pow(max(color, vec3(0.0)), vec3(0.9));

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
    point_u_view_matrix: Option<WebGlUniformLocation>,
    point_u_projection_matrix: Option<WebGlUniformLocation>,
    point_u_point_scale: Option<WebGlUniformLocation>,

    // Line rendering (orbits)
    line_program: Option<WebGlProgram>,
    line_vao: Option<WebGlVertexArrayObject>,
    line_vbo: Option<WebGlBuffer>,
    line_u_view_matrix: Option<WebGlUniformLocation>,
    line_u_projection_matrix: Option<WebGlUniformLocation>,
    line_u_color: Option<WebGlUniformLocation>,

    // Circle rendering (sun, planets, moons)
    circle_program: Option<WebGlProgram>,
    circle_vao: Option<WebGlVertexArrayObject>,
    circle_vbo: Option<WebGlBuffer>,
    circle_u_view_matrix: Option<WebGlUniformLocation>,
    circle_u_projection_matrix: Option<WebGlUniformLocation>,
    circle_u_center: Option<WebGlUniformLocation>,
    circle_u_radius: Option<WebGlUniformLocation>,
    circle_u_color: Option<WebGlUniformLocation>,
    circle_u_glow: Option<WebGlUniformLocation>,

    // Heliosphere shader
    helio_program: Option<WebGlProgram>,
    helio_vao: Option<WebGlVertexArrayObject>,
    helio_vbo: Option<WebGlBuffer>,
    helio_u_resolution: Option<WebGlUniformLocation>,
    helio_u_time: Option<WebGlUniformLocation>,
    helio_u_zoom: Option<WebGlUniformLocation>,
    helio_u_center: Option<WebGlUniformLocation>,
    helio_u_tilt: Option<WebGlUniformLocation>,
    helio_u_rotation: Option<WebGlUniformLocation>,
    helio_u_ts_radius: Option<WebGlUniformLocation>,
    helio_u_hp_radius: Option<WebGlUniformLocation>,
    helio_u_bow_radius: Option<WebGlUniformLocation>,

    // Milky Way background
    milkyway_program: Option<WebGlProgram>,
    milkyway_vao: Option<WebGlVertexArrayObject>,
    milkyway_vbo: Option<WebGlBuffer>,
    milkyway_u_resolution: Option<WebGlUniformLocation>,
    milkyway_u_time: Option<WebGlUniformLocation>,
    milkyway_u_zoom: Option<WebGlUniformLocation>,
    milkyway_u_center: Option<WebGlUniformLocation>,
    milkyway_u_tilt: Option<WebGlUniformLocation>,
    milkyway_u_rotation: Option<WebGlUniformLocation>,

    // State
    heliosphere_params: HeliosphereParameters,
    quality: RenderQuality,

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
            point_u_view_matrix: None, point_u_projection_matrix: None, point_u_point_scale: None,
            line_program: None, line_vao: None, line_vbo: None,
            line_u_view_matrix: None, line_u_projection_matrix: None, line_u_color: None,
            circle_program: None, circle_vao: None, circle_vbo: None,
            circle_u_view_matrix: None, circle_u_projection_matrix: None, circle_u_center: None, circle_u_radius: None,
            circle_u_color: None, circle_u_glow: None,
            helio_program: None, helio_vao: None, helio_vbo: None,
            helio_u_resolution: None, helio_u_time: None,
            helio_u_zoom: None, helio_u_center: None,
            helio_u_tilt: None, helio_u_rotation: None, helio_u_ts_radius: None,
            helio_u_hp_radius: None, helio_u_bow_radius: None,
            milkyway_program: None, milkyway_vao: None, milkyway_vbo: None,
            milkyway_u_resolution: None, milkyway_u_time: None, milkyway_u_zoom: None,
            milkyway_u_center: None, milkyway_u_tilt: None, milkyway_u_rotation: None,
            heliosphere_params, quality,
            orbit_buffer: Vec::with_capacity(ORBIT_SEGMENTS * 3 * 8),  // 3 floats per vertex (x, y, z)
            point_buffer: Vec::with_capacity(10000 * 7),  // 7 floats per point (x, y, z, size, r, g, b)
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

        self.point_u_view_matrix = gl.get_uniform_location(&program, "u_view_matrix");
        self.point_u_projection_matrix = gl.get_uniform_location(&program, "u_projection_matrix");
        self.point_u_point_scale = gl.get_uniform_location(&program, "u_point_scale");

        let vao = gl.create_vertex_array().ok_or("Failed to create point VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create point VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        // Layout: x, y, z, size, r, g, b (7 floats per point)
        let stride = 7 * 4;
        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        let size_loc = gl.get_attrib_location(&program, "a_size") as u32;
        let color_loc = gl.get_attrib_location(&program, "a_color") as u32;

        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 3, WebGl2RenderingContext::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(size_loc);
        gl.vertex_attrib_pointer_with_i32(size_loc, 1, WebGl2RenderingContext::FLOAT, false, stride, 12);
        gl.enable_vertex_attrib_array(color_loc);
        gl.vertex_attrib_pointer_with_i32(color_loc, 3, WebGl2RenderingContext::FLOAT, false, stride, 16);

        gl.bind_vertex_array(None);

        self.point_program = Some(program);
        self.point_vao = Some(vao);
        self.point_vbo = Some(vbo);
        Ok(())
    }

    fn init_line_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, LINE_VERT, LINE_FRAG)?;

        self.line_u_view_matrix = gl.get_uniform_location(&program, "u_view_matrix");
        self.line_u_projection_matrix = gl.get_uniform_location(&program, "u_projection_matrix");
        self.line_u_color = gl.get_uniform_location(&program, "u_color");

        let vao = gl.create_vertex_array().ok_or("Failed to create line VAO")?;
        let vbo = gl.create_buffer().ok_or("Failed to create line VBO")?;

        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_loc);
        gl.vertex_attrib_pointer_with_i32(pos_loc, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

        gl.bind_vertex_array(None);

        self.line_program = Some(program);
        self.line_vao = Some(vao);
        self.line_vbo = Some(vbo);
        Ok(())
    }

    fn init_circle_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, CIRCLE_VERT, CIRCLE_FRAG)?;

        self.circle_u_view_matrix = gl.get_uniform_location(&program, "u_view_matrix");
        self.circle_u_projection_matrix = gl.get_uniform_location(&program, "u_projection_matrix");
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

        self.helio_u_resolution = gl.get_uniform_location(&program, "u_resolution");
        self.helio_u_time = gl.get_uniform_location(&program, "u_time");
        self.helio_u_zoom = gl.get_uniform_location(&program, "u_zoom");
        self.helio_u_center = gl.get_uniform_location(&program, "u_center");
        self.helio_u_tilt = gl.get_uniform_location(&program, "u_tilt");
        self.helio_u_rotation = gl.get_uniform_location(&program, "u_rotation");
        self.helio_u_ts_radius = gl.get_uniform_location(&program, "u_ts_radius");
        self.helio_u_hp_radius = gl.get_uniform_location(&program, "u_hp_radius");
        self.helio_u_bow_radius = gl.get_uniform_location(&program, "u_bow_radius");

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

        self.helio_program = Some(program);
        self.helio_vao = Some(vao);
        self.helio_vbo = Some(vbo);
        Ok(())
    }

    fn init_milkyway_shader(&mut self) -> Result<(), String> {
        let gl = &self.gl;
        let program = create_program(gl, MILKYWAY_VERT, MILKYWAY_FRAG)?;

        self.milkyway_u_resolution = gl.get_uniform_location(&program, "u_resolution");
        self.milkyway_u_time = gl.get_uniform_location(&program, "u_time");
        self.milkyway_u_zoom = gl.get_uniform_location(&program, "u_zoom");
        self.milkyway_u_center = gl.get_uniform_location(&program, "u_center");
        self.milkyway_u_tilt = gl.get_uniform_location(&program, "u_tilt");
        self.milkyway_u_rotation = gl.get_uniform_location(&program, "u_rotation");

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


    /// Build proper view and projection matrices from CelestialCamera
    /// Returns (view_matrix, projection_matrix) for real 3D rendering
    fn build_matrices(&self, state: &SimulationState) -> ([f32; 16], [f32; 16]) {
        // Use CelestialCamera's correct 3D implementations
        let view_matrix = state.camera.view_matrix();
        let projection_matrix = state.camera.projection_matrix();
        (view_matrix, projection_matrix)
    }

    // ========================================================================
    // RENDER PASSES
    // ========================================================================

    pub fn render(&mut self, state: &SimulationState, time: f64) {
        let dpr = web_sys::window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0) as f32;
        let width = (state.view.width as f32 * dpr) as i32;
        let height = (state.view.height as f32 * dpr) as i32;

        self.gl.viewport(0, 0, width, height);

        // Enable depth testing for proper 3D occlusion
        self.gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.gl.depth_func(WebGl2RenderingContext::LEQUAL);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        let (view_matrix, proj_matrix) = self.build_matrices(state);
        let is_helio_view = state.camera.scale_level == crate::cca_projection::ScaleLevel::Heliosphere;

        // Render order: back to front
        // 1. Milky Way background (procedural shader - always rendered first)
        self.render_milkyway(state, time);

        // 2. Named/database stars (on top of procedural background)
        self.render_stars(state, &view_matrix, &proj_matrix, time);

        // 3. Heliosphere boundaries (SDF-based comet shape)
        if is_helio_view {
            self.render_heliosphere(state, time);
        }

        // 4. Solar system objects (depth buffer handles occlusion)
        self.render_oort_cloud(state, &view_matrix, &proj_matrix);
        self.render_orbits(state, &view_matrix, &proj_matrix);
        self.render_asteroid_belt(state, &view_matrix, &proj_matrix);
        self.render_sun(state, &view_matrix, &proj_matrix, time);
        self.render_planets(state, &view_matrix, &proj_matrix, time);
        self.render_moons(state, &view_matrix, &proj_matrix);

        // Disable depth test after solar system rendering
        self.gl.disable(WebGl2RenderingContext::DEPTH_TEST);
    }

    fn render_milkyway(&self, state: &SimulationState, time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.milkyway_program, &self.milkyway_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));

            // Pass uniforms
            gl.uniform2f(
                self.milkyway_u_resolution.as_ref(),
                state.view.width as f32,
                state.view.height as f32
            );
            gl.uniform1f(self.milkyway_u_time.as_ref(), time as f32);
            gl.uniform1f(self.milkyway_u_zoom.as_ref(), state.view.zoom as f32);
            gl.uniform2f(
                self.milkyway_u_center.as_ref(),
                state.view.center_x as f32,
                state.view.center_y as f32
            );
            gl.uniform1f(self.milkyway_u_tilt.as_ref(), state.view.tilt as f32);
            gl.uniform1f(self.milkyway_u_rotation.as_ref(), state.view.rotation as f32);

            // Draw fullscreen triangle
            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

            gl.bind_vertex_array(None);
        }
    }

    fn render_stars(&mut self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16], _time: f64) {
        let gl = &self.gl;

        let stars = state.star_mgr.visible_instances();
        if stars.is_empty() { return; }

        self.point_buffer.clear();
        for star in stars {
            // Use actual 3D star position (in AU, HCI frame)
            let x = star.position.x as f32;
            let y = star.position.y as f32;
            let z = star.position.z as f32;

            let size = (6.0 - star.magnitude as f64).max(1.0).min(8.0) as f32;
            // Convert RGB array to floats
            let r = star.color_rgb[0] as f32 / 255.0;
            let g = star.color_rgb[1] as f32 / 255.0;
            let b = star.color_rgb[2] as f32 / 255.0;

            // 7 floats per point: x, y, z, size, r, g, b
            self.point_buffer.extend_from_slice(&[x, y, z, size, r, g, b]);
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

            gl.uniform_matrix4fv_with_f32_array(self.point_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.point_u_projection_matrix.as_ref(), false, proj_matrix);
            gl.uniform1f(self.point_u_point_scale.as_ref(), 1.0 / state.view.zoom as f32);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 7) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_orbits(&mut self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16]) {
        let gl = &self.gl;

        if let (Some(program), Some(vao), Some(vbo)) = (&self.line_program, &self.line_vao, &self.line_vbo) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));
            gl.uniform_matrix4fv_with_f32_array(self.line_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.line_u_projection_matrix.as_ref(), false, proj_matrix);

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
                let (base_r, base_g, base_b) = orbit_colors.get(i).copied().unwrap_or((0.4, 0.4, 0.5));

                // Use pre-computed 3D orbit paths (x, y, z per segment)
                self.orbit_buffer.clear();
                for j in 0..ORBIT_SEGMENTS {
                    let x = state.orbit_paths[i][j * 3] as f32;
                    let y = state.orbit_paths[i][j * 3 + 1] as f32;
                    let z = state.orbit_paths[i][j * 3 + 2] as f32;
                    self.orbit_buffer.extend_from_slice(&[x, y, z]);
                }

                unsafe {
                    let arr = Float32Array::view(&self.orbit_buffer);
                    gl.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER, &arr, WebGl2RenderingContext::DYNAMIC_DRAW
                    );
                }

                // Draw full orbit (depth buffer handles front/back)
                gl.uniform4f(self.line_u_color.as_ref(), base_r, base_g, base_b, 0.5);
                gl.draw_arrays(WebGl2RenderingContext::LINE_LOOP, 0, ORBIT_SEGMENTS as i32);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_sun(&self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16], time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_projection_matrix.as_ref(), false, proj_matrix);

            // Sun at origin with pulsating size
            let base_radius = 0.00465; // Solar radius in AU
            let pulse = 1.0 + 0.05 * (time * 0.5).sin() as f32;
            // Much smaller minimum - just visible as a dot
            let radius = (base_radius as f32 * pulse).max(state.view.zoom as f32 * 0.5);

            // Sun at origin (0, 0, 0) in 3D
            gl.uniform3f(self.circle_u_center.as_ref(), 0.0, 0.0, 0.0);
            gl.uniform1f(self.circle_u_radius.as_ref(), radius);
            gl.uniform3f(self.circle_u_color.as_ref(), 1.0, 0.9, 0.3);
            gl.uniform1f(self.circle_u_glow.as_ref(), 0.3); // Subtle glow

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            gl.bind_vertex_array(None);
        }
    }

    fn render_planets(&self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16], _time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_projection_matrix.as_ref(), false, proj_matrix);

            for i in 0..state.planet_count {
                // Use actual 3D planet position
                let x = state.planet_x[i] as f32;
                let y = state.planet_y[i] as f32;
                let z = state.planet_z[i] as f32;

                // Planet radius - keep small but visible
                // Min radius in world units = zoom * 0.3 (much smaller than before)
                let au_km = 149597870.7;
                let radius_au = (state.planet_radii_km[i] / au_km) as f32;
                let min_radius = (state.view.zoom * 0.3) as f32;
                let radius = radius_au.max(min_radius);

                let (r, g, b) = parse_color(state.planet_colors[i]);

                // 3D position
                gl.uniform3f(self.circle_u_center.as_ref(), x, y, z);
                gl.uniform1f(self.circle_u_radius.as_ref(), radius);
                gl.uniform3f(self.circle_u_color.as_ref(), r, g, b);
                gl.uniform1f(self.circle_u_glow.as_ref(), 0.0); // No glow - crisp planets

                gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_moons(&self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16]) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.circle_program, &self.circle_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.circle_u_projection_matrix.as_ref(), false, proj_matrix);

            for i in 0..state.moon_count {
                let parent_idx = state.moon_parent_planet[i];
                if parent_idx >= state.planet_count { continue; }

                // Use pre-computed 3D world coordinates
                let mx = state.moon_world_x[i] as f32;
                let my = state.moon_world_y[i] as f32;
                let mz = state.moon_world_z[i] as f32;

                // Moons are tiny - much smaller than planets
                let radius = (state.view.zoom * 0.15) as f32;

                // 3D position
                gl.uniform3f(self.circle_u_center.as_ref(), mx, my, mz);
                gl.uniform1f(self.circle_u_radius.as_ref(), radius);
                gl.uniform3f(self.circle_u_color.as_ref(), 0.7, 0.7, 0.7);
                gl.uniform1f(self.circle_u_glow.as_ref(), 0.0);

                gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render_asteroid_belt(&mut self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16]) {
        let gl = &self.gl;

        // Only render when zoomed in enough to see the belt
        if state.view.zoom > 0.5 { return; }

        self.point_buffer.clear();

        // Sample subset for performance
        let step = if state.view.zoom > 0.05 { 4 } else if state.view.zoom > 0.01 { 2 } else { 1 };

        for i in (0..state.asteroid_count).step_by(step) {
            let r = state.asteroid_distances[i] as f32;
            let angle = state.asteroid_angles[i] as f32;
            let incl = state.asteroid_inclinations[i] as f32;

            // Full 3D position in ecliptic coordinates
            let x = r * angle.cos();
            let y = r * angle.sin();
            let z = r * incl.sin() * 0.1; // Z offset for inclination (small vertical scatter)

            // Tiny fixed size - asteroids should be 1-2 pixel dots
            let size = 1.0 + ((i as f32 * 1.618).fract() * 1.5);

            // Brownish-gray color with slight variation
            let v = (i as f32 * 0.7182).fract();
            let r_col = 0.5 + v * 0.2;
            let g_col = 0.45 + v * 0.15;
            let b_col = 0.35 + v * 0.1;

            // 7 floats per point: x, y, z, size, r, g, b
            self.point_buffer.extend_from_slice(&[x, y, z, size, r_col, g_col, b_col]);
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

            gl.uniform_matrix4fv_with_f32_array(self.point_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.point_u_projection_matrix.as_ref(), false, proj_matrix);
            // Fixed small point scale - asteroids are tiny!
            gl.uniform1f(self.point_u_point_scale.as_ref(), 1.0);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 7) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_oort_cloud(&mut self, state: &SimulationState, view_matrix: &[f32; 16], proj_matrix: &[f32; 16]) {
        let gl = &self.gl;

        // Only render at far zoom levels
        if state.view.zoom < 10.0 { return; }

        self.point_buffer.clear();

        for i in 0..state.oort_count {
            let r = state.oort_distances[i] as f32;
            let theta = state.oort_theta[i] as f32;
            let phi = state.oort_phi[i] as f32;

            // Full 3D spherical to Cartesian conversion
            let x = r * theta.sin() * phi.cos();
            let y = r * theta.sin() * phi.sin();
            let z = r * theta.cos();

            // Fade with distance
            let alpha = (1.0 - r / 100000.0).max(0.1);
            // 7 floats per point: x, y, z, size, r, g, b
            self.point_buffer.extend_from_slice(&[x, y, z, 1.0, 0.3 * alpha, 0.3 * alpha, 0.4 * alpha]);
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

            gl.uniform_matrix4fv_with_f32_array(self.point_u_view_matrix.as_ref(), false, view_matrix);
            gl.uniform_matrix4fv_with_f32_array(self.point_u_projection_matrix.as_ref(), false, proj_matrix);
            gl.uniform1f(self.point_u_point_scale.as_ref(), 0.5 / state.view.zoom as f32);

            gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, (self.point_buffer.len() / 7) as i32);
            gl.bind_vertex_array(None);
        }
    }

    fn render_heliosphere(&self, state: &SimulationState, time: f64) {
        let gl = &self.gl;

        if let (Some(program), Some(vao)) = (&self.helio_program, &self.helio_vao) {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));

            gl.uniform2f(self.helio_u_resolution.as_ref(), state.view.width as f32, state.view.height as f32);
            gl.uniform1f(self.helio_u_time.as_ref(), time as f32);
            gl.uniform1f(self.helio_u_zoom.as_ref(), state.view.zoom as f32);
            gl.uniform2f(self.helio_u_center.as_ref(), state.view.center_x as f32, state.view.center_y as f32);
            gl.uniform1f(self.helio_u_tilt.as_ref(), state.view.tilt as f32);
            gl.uniform1f(self.helio_u_rotation.as_ref(), state.view.rotation as f32);

            // Pass boundary radii
            gl.uniform1f(self.helio_u_ts_radius.as_ref(), state.termination_shock_au as f32);
            gl.uniform1f(self.helio_u_hp_radius.as_ref(), state.heliopause_au as f32);
            gl.uniform1f(self.helio_u_bow_radius.as_ref(), state.bow_shock_au as f32);

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

            gl.bind_vertex_array(None);
        }
    }

    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.quality = quality;
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
