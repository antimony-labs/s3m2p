//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/data/mod.rs
//! PURPOSE: Data structures for simulation and spatial queries
//! LAYER: DNA → DATA
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! DATA provides efficient data structures:
//! - arena.rs        - Generic arena allocator with generational indices
//! - spatial_grid.rs - Uniform spatial grid for O(1) neighbor queries
//! - mesh.rs         - Triangle/quad mesh (scaffold)
//! - graph.rs        - Node/edge graph (scaffold)
//!
//! Future:
//! - quadtree.rs     - Hierarchical 2D spatial partitioning
//! - octree.rs       - Hierarchical 3D spatial partitioning
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// ACTIVE SUBMODULES
// ─────────────────────────────────────────────────────────────────────────────────

/// Generic arena allocator with generational indices
pub mod arena;
pub use arena::{Arena, Handle};

/// Uniform spatial grid for O(1) neighbor queries
pub mod spatial_grid;
pub use spatial_grid::UniformGrid;

/// Triangle/quad mesh (scaffold for future CAD)
pub mod mesh;

/// Node/edge graph (scaffold for circuits, pathfinding)
pub mod graph;

// ─────────────────────────────────────────────────────────────────────────────────
// FUTURE SUBMODULES
// ─────────────────────────────────────────────────────────────────────────────────

// pub mod quadtree;     // TODO: Hierarchical 2D
// pub mod octree;       // TODO: Hierarchical 3D
