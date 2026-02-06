// ═══════════════════════════════════════════════════════════════════════════════
// FILE: simplify.rs | ATLAS/CORE/GEO_ENGINE/src/simplify.rs
// PURPOSE: Line simplification algorithms for LOD management
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use crate::geometry::{Coord, Geometry, LineString, MultiPolygon, Polygon, Ring};

/// Douglas-Peucker line simplification algorithm
///
/// Reduces the number of points in a polyline while preserving shape.
/// `tolerance` is the maximum perpendicular distance a point can deviate.
pub fn douglas_peucker(coords: &[Coord], tolerance: f32) -> Vec<Coord> {
    if coords.len() < 3 {
        return coords.to_vec();
    }

    let tolerance_sq = tolerance * tolerance;
    let mut keep = vec![false; coords.len()];
    keep[0] = true;
    keep[coords.len() - 1] = true;

    dp_recursive(coords, 0, coords.len() - 1, tolerance_sq, &mut keep);

    coords
        .iter()
        .zip(keep.iter())
        .filter_map(|(c, &k)| if k { Some(*c) } else { None })
        .collect()
}

fn dp_recursive(coords: &[Coord], start: usize, end: usize, tolerance_sq: f32, keep: &mut [bool]) {
    if end <= start + 1 {
        return;
    }

    let mut max_dist_sq = 0.0;
    let mut max_idx = start;

    let p1 = coords[start];
    let p2 = coords[end];

    for i in (start + 1)..end {
        let dist_sq = perpendicular_distance_sq(coords[i], p1, p2);
        if dist_sq > max_dist_sq {
            max_dist_sq = dist_sq;
            max_idx = i;
        }
    }

    if max_dist_sq > tolerance_sq {
        keep[max_idx] = true;
        dp_recursive(coords, start, max_idx, tolerance_sq, keep);
        dp_recursive(coords, max_idx, end, tolerance_sq, keep);
    }
}

/// Calculate squared perpendicular distance from point to line segment
fn perpendicular_distance_sq(point: Coord, line_start: Coord, line_end: Coord) -> f32 {
    let dx = line_end.x - line_start.x;
    let dy = line_end.y - line_start.y;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-10 {
        // Line segment is a point
        return point.distance_squared(&line_start);
    }

    // Project point onto line
    let t = ((point.x - line_start.x) * dx + (point.y - line_start.y) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj = Coord {
        x: line_start.x + t * dx,
        y: line_start.y + t * dy,
    };

    point.distance_squared(&proj)
}

/// Simplify a ring (closed polygon boundary)
pub fn simplify_ring(ring: &Ring, tolerance: f32) -> Ring {
    if ring.coords.len() < 4 {
        return ring.clone();
    }

    let simplified = douglas_peucker(&ring.coords, tolerance);

    // Ensure ring stays closed and valid
    if simplified.len() < 4 {
        // Too few points, return original
        return ring.clone();
    }

    let mut result = Ring::new(simplified);
    result.close();
    result
}

/// Simplify a polygon (exterior + holes)
pub fn simplify_polygon(polygon: &Polygon, tolerance: f32) -> Polygon {
    let exterior = simplify_ring(&polygon.exterior, tolerance);

    // Only keep holes that are still valid after simplification
    let holes: Vec<Ring> = polygon
        .holes
        .iter()
        .map(|h| simplify_ring(h, tolerance))
        .filter(|h| h.coords.len() >= 4)
        .collect();

    Polygon::with_holes(exterior, holes)
}

/// Simplify a multi-polygon
pub fn simplify_multipolygon(mp: &MultiPolygon, tolerance: f32) -> MultiPolygon {
    let polygons: Vec<Polygon> = mp
        .polygons
        .iter()
        .map(|p| simplify_polygon(p, tolerance))
        .filter(|p| p.exterior.coords.len() >= 4)
        .collect();

    MultiPolygon::new(polygons)
}

/// Simplify a line string
pub fn simplify_linestring(ls: &LineString, tolerance: f32) -> LineString {
    if ls.coords.len() < 3 {
        return ls.clone();
    }
    LineString::new(douglas_peucker(&ls.coords, tolerance))
}

/// Simplify any geometry
pub fn simplify_geometry(geom: &Geometry, tolerance: f32) -> Geometry {
    match geom {
        Geometry::Point(c) => Geometry::Point(*c),
        Geometry::LineString(ls) => Geometry::LineString(simplify_linestring(ls, tolerance)),
        Geometry::Polygon(p) => Geometry::Polygon(simplify_polygon(p, tolerance)),
        Geometry::MultiPolygon(mp) => Geometry::MultiPolygon(simplify_multipolygon(mp, tolerance)),
    }
}

/// Visvalingam-Whyatt simplification (area-based)
/// Better for cartographic purposes but slower than Douglas-Peucker
pub fn visvalingam_whyatt(coords: &[Coord], min_area: f32) -> Vec<Coord> {
    if coords.len() < 3 {
        return coords.to_vec();
    }

    let mut points: Vec<(Coord, f32)> = coords.iter().map(|c| (*c, f32::INFINITY)).collect();

    // Calculate initial triangle areas
    for i in 1..points.len() - 1 {
        points[i].1 = triangle_area(points[i - 1].0, points[i].0, points[i + 1].0);
    }

    // Iteratively remove points with smallest area
    while points.len() > 2 {
        // Find point with smallest area
        let min_idx = (1..points.len() - 1)
            .min_by(|&a, &b| points[a].1.partial_cmp(&points[b].1).unwrap())
            .unwrap();

        if points[min_idx].1 >= min_area {
            break;
        }

        // Remove the point
        points.remove(min_idx);

        // Recalculate adjacent areas
        if min_idx > 1 && min_idx < points.len() {
            let prev = min_idx - 1;
            points[prev].1 = triangle_area(
                points
                    .get(prev.wrapping_sub(1))
                    .map(|p| p.0)
                    .unwrap_or(points[prev].0),
                points[prev].0,
                points[min_idx].0,
            );
        }
        if min_idx < points.len() - 1 {
            points[min_idx].1 = triangle_area(
                points[min_idx - 1].0,
                points[min_idx].0,
                points
                    .get(min_idx + 1)
                    .map(|p| p.0)
                    .unwrap_or(points[min_idx].0),
            );
        }
    }

    points.into_iter().map(|(c, _)| c).collect()
}

/// Calculate area of triangle formed by three points
fn triangle_area(a: Coord, b: Coord, c: Coord) -> f32 {
    ((a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2.0).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_douglas_peucker_simple() {
        // Straight line with a detour
        let coords = vec![
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 0.1), // slight deviation
            Coord::new(2.0, 0.0),
            Coord::new(3.0, 0.0),
        ];

        let simplified = douglas_peucker(&coords, 0.2);
        assert!(simplified.len() < coords.len());
        assert_eq!(simplified[0], coords[0]);
        assert_eq!(simplified[simplified.len() - 1], coords[coords.len() - 1]);
    }

    #[test]
    fn test_douglas_peucker_preserves_sharp_corners() {
        // L-shape should preserve corner
        let coords = vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
        ];

        let simplified = douglas_peucker(&coords, 0.1);
        assert_eq!(simplified.len(), 3);
    }

    #[test]
    fn test_simplify_ring() {
        let ring = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(5.0, 0.1),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(5.0, 9.9),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);

        let simplified = simplify_ring(&ring, 0.5);
        assert!(simplified.coords.len() <= ring.coords.len());
        // Should still be closed
        assert_eq!(simplified.coords.first(), simplified.coords.last());
    }

    #[test]
    fn test_perpendicular_distance() {
        let p = Coord::new(1.0, 1.0);
        let a = Coord::new(0.0, 0.0);
        let b = Coord::new(2.0, 0.0);

        let dist_sq = perpendicular_distance_sq(p, a, b);
        // Distance from (1,1) to line y=0 is 1
        assert!((dist_sq - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_triangle_area() {
        let a = Coord::new(0.0, 0.0);
        let b = Coord::new(4.0, 0.0);
        let c = Coord::new(2.0, 3.0);

        let area = triangle_area(a, b, c);
        // Area = 0.5 * base * height = 0.5 * 4 * 3 = 6
        assert!((area - 6.0).abs() < 1e-6);
    }
}
