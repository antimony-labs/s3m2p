// ═══════════════════════════════════════════════════════════════════════════════
// FILE: tile.rs | ATLAS/CORE/GEO_ENGINE/src/tile.rs
// PURPOSE: Slippy map tile system with hierarchical indexing and LOD management
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use crate::geometry::{BoundingBox, Coord};
use serde::{Deserialize, Serialize};

/// Tile key following Web Mercator slippy map convention (z/x/y)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileKey {
    pub z: u8,  // zoom level (0-20)
    pub x: u32, // column (0 to 2^z - 1)
    pub y: u32, // row (0 to 2^z - 1)
}

impl TileKey {
    /// Create a new tile key
    #[inline]
    pub const fn new(z: u8, x: u32, y: u32) -> Self {
        Self { z, x, y }
    }

    /// Root tile (zoom 0, entire world)
    pub const ROOT: Self = Self { z: 0, x: 0, y: 0 };

    /// Maximum supported zoom level
    pub const MAX_ZOOM: u8 = 20;

    /// Number of tiles at this zoom level (per axis)
    #[inline]
    pub fn tiles_per_axis(&self) -> u32 {
        1 << self.z
    }

    /// Total number of tiles at this zoom level
    #[inline]
    pub fn total_tiles(&self) -> u64 {
        let n = self.tiles_per_axis() as u64;
        n * n
    }

    /// Get tile containing a WGS84 coordinate (lon/lat in degrees)
    pub fn from_lonlat(lon: f32, lat: f32, zoom: u8) -> Self {
        let n = (1 << zoom) as f32;

        // Clamp latitude to valid Mercator range
        let lat = lat.clamp(-85.051128, 85.051128);
        let lat_rad = lat.to_radians();

        let x = ((lon + 180.0) / 360.0 * n).floor() as u32;
        let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f32::consts::PI) / 2.0
            * n)
            .floor() as u32;

        // Clamp to valid range
        let max = (1 << zoom) - 1;
        Self {
            z: zoom,
            x: x.min(max),
            y: y.min(max),
        }
    }

    /// Get tile containing a coordinate
    pub fn from_coord(coord: Coord, zoom: u8) -> Self {
        Self::from_lonlat(coord.x, coord.y, zoom)
    }

    /// Get bounding box of this tile in WGS84 (lon/lat degrees)
    pub fn bounds(&self) -> BoundingBox {
        let n = self.tiles_per_axis() as f32;

        let lon_min = (self.x as f32) / n * 360.0 - 180.0;
        let lon_max = ((self.x + 1) as f32) / n * 360.0 - 180.0;

        let lat_max = tile_y_to_lat(self.y, n);
        let lat_min = tile_y_to_lat(self.y + 1, n);

        BoundingBox::new(Coord::new(lon_min, lat_min), Coord::new(lon_max, lat_max))
    }

    /// Get center coordinate of this tile
    pub fn center(&self) -> Coord {
        self.bounds().center()
    }

    /// Get parent tile (one zoom level up)
    pub fn parent(&self) -> Option<Self> {
        if self.z == 0 {
            None
        } else {
            Some(Self {
                z: self.z - 1,
                x: self.x / 2,
                y: self.y / 2,
            })
        }
    }

    /// Get the 4 child tiles (one zoom level down)
    pub fn children(&self) -> Option<[Self; 4]> {
        if self.z >= Self::MAX_ZOOM {
            return None;
        }
        let z = self.z + 1;
        let x = self.x * 2;
        let y = self.y * 2;
        Some([
            Self::new(z, x, y),         // top-left
            Self::new(z, x + 1, y),     // top-right
            Self::new(z, x, y + 1),     // bottom-left
            Self::new(z, x + 1, y + 1), // bottom-right
        ])
    }

    /// Get all ancestor tiles from root to parent
    pub fn ancestors(&self) -> Vec<Self> {
        let mut ancestors = Vec::with_capacity(self.z as usize);
        let mut current = *self;
        while let Some(parent) = current.parent() {
            ancestors.push(parent);
            current = parent;
        }
        ancestors.reverse();
        ancestors
    }

    /// Pack tile key into u64 for efficient storage/hashing
    /// Format: z(8 bits) | x(28 bits) | y(28 bits)
    #[inline]
    pub fn to_u64(&self) -> u64 {
        ((self.z as u64) << 56) | ((self.x as u64) << 28) | (self.y as u64)
    }

    /// Unpack tile key from u64
    #[inline]
    pub fn from_u64(packed: u64) -> Self {
        Self {
            z: (packed >> 56) as u8,
            x: ((packed >> 28) & 0x0FFF_FFFF) as u32,
            y: (packed & 0x0FFF_FFFF) as u32,
        }
    }

    /// Check if this tile is an ancestor of another
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        if self.z >= other.z {
            return false;
        }
        let dz = other.z - self.z;
        let expected_x = other.x >> dz;
        let expected_y = other.y >> dz;
        self.x == expected_x && self.y == expected_y
    }

    /// Get all tiles that cover the given bounds at this zoom level
    pub fn covering_tiles(bounds: &BoundingBox, zoom: u8) -> Vec<Self> {
        let tl = Self::from_lonlat(bounds.min.x, bounds.max.y, zoom);
        let br = Self::from_lonlat(bounds.max.x, bounds.min.y, zoom);

        let mut tiles = Vec::new();
        for y in tl.y..=br.y {
            for x in tl.x..=br.x {
                tiles.push(Self::new(zoom, x, y));
            }
        }
        tiles
    }
}

/// Convert tile Y coordinate to latitude
fn tile_y_to_lat(y: u32, n: f32) -> f32 {
    let lat_rad = (std::f32::consts::PI * (1.0 - 2.0 * (y as f32) / n)).sinh().atan();
    lat_rad.to_degrees()
}

/// LOD (Level of Detail) configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LodLevel {
    /// Zoom range where this LOD applies (inclusive)
    pub zoom_range: (u8, u8),
    /// Douglas-Peucker simplification tolerance (in degrees)
    pub simplification_tolerance: f32,
    /// Minimum polygon area to include (in square degrees)
    pub min_polygon_area: f32,
    /// Minimum scalerank to include (0 = show all, higher = filter more)
    pub min_scalerank: u8,
}

impl LodLevel {
    /// Create default LOD levels for world maps
    pub fn default_levels() -> [Self; 4] {
        [
            // Zoom 0-3: World overview
            Self {
                zoom_range: (0, 3),
                simplification_tolerance: 1.0,
                min_polygon_area: 10.0,
                min_scalerank: 0,
            },
            // Zoom 4-6: Continental
            Self {
                zoom_range: (4, 6),
                simplification_tolerance: 0.1,
                min_polygon_area: 1.0,
                min_scalerank: 2,
            },
            // Zoom 7-10: Regional
            Self {
                zoom_range: (7, 10),
                simplification_tolerance: 0.01,
                min_polygon_area: 0.01,
                min_scalerank: 4,
            },
            // Zoom 11+: Local detail
            Self {
                zoom_range: (11, 20),
                simplification_tolerance: 0.001,
                min_polygon_area: 0.0,
                min_scalerank: 10,
            },
        ]
    }

    /// Get the appropriate LOD level for a zoom
    pub fn for_zoom(levels: &[Self], zoom: u8) -> Option<&Self> {
        levels
            .iter()
            .find(|l| zoom >= l.zoom_range.0 && zoom <= l.zoom_range.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_lonlat() {
        // London at zoom 10
        let tile = TileKey::from_lonlat(-0.1276, 51.5074, 10);
        assert_eq!(tile.z, 10);
        // Should be around x=511, y=340 for London
        assert!(tile.x >= 510 && tile.x <= 512);
        assert!(tile.y >= 339 && tile.y <= 341);
    }

    #[test]
    fn test_tile_bounds() {
        let tile = TileKey::new(0, 0, 0);
        let bounds = tile.bounds();
        assert!((bounds.min.x - (-180.0)).abs() < 0.01);
        assert!((bounds.max.x - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_tile_parent_children() {
        let tile = TileKey::new(2, 1, 2);
        let parent = tile.parent().unwrap();
        assert_eq!(parent.z, 1);
        assert_eq!(parent.x, 0);
        assert_eq!(parent.y, 1);

        let children = parent.children().unwrap();
        assert!(children.contains(&tile));
    }

    #[test]
    fn test_tile_pack_unpack() {
        let tile = TileKey::new(10, 500, 340);
        let packed = tile.to_u64();
        let unpacked = TileKey::from_u64(packed);
        assert_eq!(tile, unpacked);
    }

    #[test]
    fn test_tile_is_ancestor() {
        let parent = TileKey::new(5, 10, 15);
        let child = TileKey::new(7, 42, 62);
        assert!(parent.is_ancestor_of(&child));
        assert!(!child.is_ancestor_of(&parent));
    }

    #[test]
    fn test_covering_tiles() {
        let bounds = BoundingBox::new(Coord::new(-10.0, 40.0), Coord::new(10.0, 60.0));
        let tiles = TileKey::covering_tiles(&bounds, 4);
        assert!(!tiles.is_empty());
        // All tiles should be at zoom 4
        for tile in &tiles {
            assert_eq!(tile.z, 4);
        }
    }
}
