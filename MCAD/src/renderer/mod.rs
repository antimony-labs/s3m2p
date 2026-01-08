//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | MCAD/src/renderer/mod.rs
//! PURPOSE: WebGL2 rendering module for MCAD 3D viewport
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod camera;
pub mod shaders;
pub mod buffers;
pub mod context;

pub use camera::Camera;
pub use shaders::RenderMode;
pub use buffers::MeshBuffers;
pub use context::WebGLContext;
