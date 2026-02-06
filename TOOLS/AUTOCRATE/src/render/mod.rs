//! Rendering module for AutoCrate visualization
//!
//! Provides both 3D (WebGL2) and 2D (Canvas2D) rendering:
//! - `webgl` - 3D interactive visualization with orbit controls
//! - `canvas2d` - 2D technical drawing with dimensions
//! - `textures` - Procedural wood grain generation
//! - `materials` - Visual material properties (lumber, plywood, metal)

pub mod canvas2d;
pub mod materials;
pub mod mesh;
pub mod textures;
pub mod webgl;

pub use canvas2d::Canvas2DRenderer;
pub use materials::{Material, MaterialType};
pub use mesh::{Mesh, MeshBuffer};
pub use textures::{generate_wood_grain, WoodTexture};
pub use webgl::{Camera, ProjectionType, RenderMode, WebGLRenderer};

/// View mode toggle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    /// 3D interactive view with orbit controls
    ThreeD,
    /// 2D technical drawing (orthographic projection)
    TwoD,
}
