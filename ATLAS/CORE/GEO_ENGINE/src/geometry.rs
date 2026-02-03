// ═══════════════════════════════════════════════════════════════════════════════
// FILE: geometry.rs | ATLAS/CORE/GEO_ENGINE/src/geometry.rs
// PURPOSE: Core geometry types for vector map representation
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// 2D coordinate (WGS84 lon/lat or projected Web Mercator)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub x: f32, // longitude or easting
    pub y: f32, // latitude or northing
}

impl Coord {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn distance_squared(&self, other: &Coord) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    #[inline]
    pub fn distance(&self, other: &Coord) -> f32 {
        self.distance_squared(other).sqrt()
    }
}

impl From<(f32, f32)> for Coord {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<[f32; 2]> for Coord {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

/// Axis-aligned bounding box
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Coord,
    pub max: Coord,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        min: Coord {
            x: f32::INFINITY,
            y: f32::INFINITY,
        },
        max: Coord {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
        },
    };

    pub const WORLD: Self = Self {
        min: Coord { x: -180.0, y: -90.0 },
        max: Coord { x: 180.0, y: 90.0 },
    };

    #[inline]
    pub fn new(min: Coord, max: Coord) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_coords(coords: &[Coord]) -> Self {
        let mut bbox = Self::EMPTY;
        for c in coords {
            bbox.extend_coord(*c);
        }
        bbox
    }

    #[inline]
    pub fn extend_coord(&mut self, c: Coord) {
        self.min.x = self.min.x.min(c.x);
        self.min.y = self.min.y.min(c.y);
        self.max.x = self.max.x.max(c.x);
        self.max.y = self.max.y.max(c.y);
    }

    #[inline]
    pub fn extend_bbox(&mut self, other: &BoundingBox) {
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
    }

    #[inline]
    pub fn contains(&self, c: Coord) -> bool {
        c.x >= self.min.x && c.x <= self.max.x && c.y >= self.min.y && c.y <= self.max.y
    }

    #[inline]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn center(&self) -> Coord {
        Coord {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
        }
    }

    #[inline]
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Closed ring of coordinates (exterior or hole boundary)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ring {
    pub coords: Vec<Coord>,
}

impl Ring {
    pub fn new(coords: Vec<Coord>) -> Self {
        Self { coords }
    }

    /// Ensure ring is closed (first == last)
    pub fn close(&mut self) {
        if let (Some(first), Some(last)) = (self.coords.first(), self.coords.last()) {
            if first != last {
                self.coords.push(*first);
            }
        }
    }

    /// Calculate signed area (positive = counter-clockwise, negative = clockwise)
    pub fn signed_area(&self) -> f32 {
        if self.coords.len() < 3 {
            return 0.0;
        }
        let mut area = 0.0;
        let n = self.coords.len();
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.coords[i].x * self.coords[j].y;
            area -= self.coords[j].x * self.coords[i].y;
        }
        area * 0.5
    }

    #[inline]
    pub fn area(&self) -> f32 {
        self.signed_area().abs()
    }

    /// Check if ring is counter-clockwise (exterior ring convention)
    #[inline]
    pub fn is_ccw(&self) -> bool {
        self.signed_area() > 0.0
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::from_coords(&self.coords)
    }
}

/// Polygon with exterior ring and optional holes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub exterior: Ring,
    pub holes: Vec<Ring>,
}

impl Polygon {
    pub fn new(exterior: Ring) -> Self {
        Self {
            exterior,
            holes: Vec::new(),
        }
    }

    pub fn with_holes(exterior: Ring, holes: Vec<Ring>) -> Self {
        Self { exterior, holes }
    }

    pub fn bounds(&self) -> BoundingBox {
        self.exterior.bounds()
    }

    pub fn area(&self) -> f32 {
        let mut area = self.exterior.area();
        for hole in &self.holes {
            area -= hole.area();
        }
        area
    }
}

/// Multi-polygon for complex features (countries with islands, etc.)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiPolygon {
    pub polygons: Vec<Polygon>,
}

impl MultiPolygon {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }

    pub fn bounds(&self) -> BoundingBox {
        let mut bbox = BoundingBox::EMPTY;
        for poly in &self.polygons {
            bbox.extend_bbox(&poly.bounds());
        }
        bbox
    }

    pub fn area(&self) -> f32 {
        self.polygons.iter().map(|p| p.area()).sum()
    }
}

/// LineString for linear features (rivers, roads, boundaries)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineString {
    pub coords: Vec<Coord>,
}

impl LineString {
    pub fn new(coords: Vec<Coord>) -> Self {
        Self { coords }
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::from_coords(&self.coords)
    }

    pub fn length(&self) -> f32 {
        if self.coords.len() < 2 {
            return 0.0;
        }
        let mut len = 0.0;
        for i in 1..self.coords.len() {
            len += self.coords[i - 1].distance(&self.coords[i]);
        }
        len
    }
}

/// Unified geometry enum
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
    Point(Coord),
    LineString(LineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
}

impl Geometry {
    pub fn bounds(&self) -> BoundingBox {
        match self {
            Geometry::Point(c) => BoundingBox::new(*c, *c),
            Geometry::LineString(ls) => ls.bounds(),
            Geometry::Polygon(p) => p.bounds(),
            Geometry::MultiPolygon(mp) => mp.bounds(),
        }
    }

    pub fn coord_count(&self) -> usize {
        match self {
            Geometry::Point(_) => 1,
            Geometry::LineString(ls) => ls.coords.len(),
            Geometry::Polygon(p) => {
                p.exterior.coords.len() + p.holes.iter().map(|h| h.coords.len()).sum::<usize>()
            }
            Geometry::MultiPolygon(mp) => mp
                .polygons
                .iter()
                .map(|p| {
                    p.exterior.coords.len() + p.holes.iter().map(|h| h.coords.len()).sum::<usize>()
                })
                .sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_distance() {
        let a = Coord::new(0.0, 0.0);
        let b = Coord::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BoundingBox::new(Coord::new(0.0, 0.0), Coord::new(10.0, 10.0));
        assert!(bbox.contains(Coord::new(5.0, 5.0)));
        assert!(!bbox.contains(Coord::new(15.0, 5.0)));
    }

    #[test]
    fn test_bbox_intersects() {
        let a = BoundingBox::new(Coord::new(0.0, 0.0), Coord::new(10.0, 10.0));
        let b = BoundingBox::new(Coord::new(5.0, 5.0), Coord::new(15.0, 15.0));
        let c = BoundingBox::new(Coord::new(20.0, 20.0), Coord::new(30.0, 30.0));
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_ring_area() {
        // Square 10x10
        let ring = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);
        assert!((ring.area() - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_polygon_area_with_hole() {
        // 10x10 square with 2x2 hole = 100 - 4 = 96
        let exterior = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);
        let hole = Ring::new(vec![
            Coord::new(4.0, 4.0),
            Coord::new(6.0, 4.0),
            Coord::new(6.0, 6.0),
            Coord::new(4.0, 6.0),
            Coord::new(4.0, 4.0),
        ]);
        let poly = Polygon::with_holes(exterior, vec![hole]);
        assert!((poly.area() - 96.0).abs() < 1e-6);
    }
}
