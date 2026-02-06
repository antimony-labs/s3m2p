//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | MCAD/src/renderer/mod.rs
//! PURPOSE: WebGL2 rendering module for MCAD 3D viewport
//! MODIFIED: 2026-01-07
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod buffers;
pub mod camera;
pub mod context;
pub mod shaders;

pub use buffers::MeshBuffers;
pub use camera::Camera;
pub use context::WebGLContext;
pub use shaders::RenderMode;
