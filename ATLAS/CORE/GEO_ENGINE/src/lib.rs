// ═══════════════════════════════════════════════════════════════════════════════
// FILE: lib.rs | ATLAS/CORE/GEO_ENGINE/src/lib.rs
// PURPOSE: Geographic engine - vector geometry, tiles, projections, and binary format
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

pub mod feature;
pub mod format;
pub mod geometry;
pub mod projection;
pub mod simplify;
pub mod tile;

pub use feature::{Feature, FeatureCollection, FeatureId, FeatureType, Properties};
pub use geometry::{BoundingBox, Coord, Geometry, LineString, MultiPolygon, Polygon, Ring};
pub use projection::WebMercator;
pub use simplify::douglas_peucker;
pub use tile::{LodLevel, TileKey};
