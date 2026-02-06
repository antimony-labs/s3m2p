// ═══════════════════════════════════════════════════════════════════════════════
// FILE: projection.rs | ATLAS/CORE/GEO_ENGINE/src/projection.rs
// PURPOSE: Map projections (WGS84 to/from Web Mercator and screen coordinates)
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use crate::geometry::{BoundingBox, Coord};
use std::f32::consts::PI;

/// Web Mercator projection (EPSG:3857)
/// Used by Google Maps, OpenStreetMap, and most web mapping services
pub struct WebMercator;

impl WebMercator {
    /// Earth radius in meters (WGS84 semi-major axis)
    pub const EARTH_RADIUS: f32 = 6_378_137.0;

    /// Maximum latitude for Web Mercator (cuts off at ~85.05 degrees)
    pub const MAX_LATITUDE: f32 = 85.051128;

    /// World bounds in meters
    pub const WORLD_EXTENT: f32 = 20_037_508.34; // PI * EARTH_RADIUS

    /// Project WGS84 lon/lat to Web Mercator meters
    pub fn project(lon: f32, lat: f32) -> (f32, f32) {
        let lat = lat.clamp(-Self::MAX_LATITUDE, Self::MAX_LATITUDE);

        let x = lon.to_radians() * Self::EARTH_RADIUS;
        let y = ((PI / 4.0 + lat.to_radians() / 2.0).tan()).ln() * Self::EARTH_RADIUS;

        (x, y)
    }

    /// Unproject Web Mercator meters to WGS84 lon/lat
    pub fn unproject(x: f32, y: f32) -> (f32, f32) {
        let lon = (x / Self::EARTH_RADIUS).to_degrees();
        let lat = (2.0 * (y / Self::EARTH_RADIUS).exp().atan() - PI / 2.0).to_degrees();

        (lon, lat)
    }

    /// Project a coordinate
    pub fn project_coord(coord: Coord) -> Coord {
        let (x, y) = Self::project(coord.x, coord.y);
        Coord::new(x, y)
    }

    /// Unproject a coordinate
    pub fn unproject_coord(coord: Coord) -> Coord {
        let (lon, lat) = Self::unproject(coord.x, coord.y);
        Coord::new(lon, lat)
    }

    /// Project lon/lat to normalized tile coordinates (0-1 range)
    pub fn lonlat_to_tile_fraction(lon: f32, lat: f32, zoom: u8) -> (f32, f32) {
        let n = (1u32 << zoom) as f32;
        let lat = lat.clamp(-Self::MAX_LATITUDE, Self::MAX_LATITUDE);
        let lat_rad = lat.to_radians();

        let x = (lon + 180.0) / 360.0 * n;
        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n;

        (x, y)
    }

    /// Convert tile coordinates to lon/lat
    pub fn tile_to_lonlat(tile_x: f32, tile_y: f32, zoom: u8) -> (f32, f32) {
        let n = (1u32 << zoom) as f32;

        let lon = tile_x / n * 360.0 - 180.0;
        let lat_rad = (PI * (1.0 - 2.0 * tile_y / n)).sinh().atan();
        let lat = lat_rad.to_degrees();

        (lon, lat)
    }

    /// Get meters per pixel at a given latitude and zoom level
    pub fn meters_per_pixel(lat: f32, zoom: u8) -> f32 {
        let tile_size = 256.0; // Standard tile size
        let lat_rad = lat.to_radians();
        let circumference = 2.0 * PI * Self::EARTH_RADIUS * lat_rad.cos();
        let tiles = (1u32 << zoom) as f32;
        circumference / (tiles * tile_size)
    }

    /// Get appropriate zoom level for a given scale (meters per pixel)
    pub fn zoom_for_scale(meters_per_pixel: f32, lat: f32) -> u8 {
        let tile_size = 256.0;
        let lat_rad = lat.to_radians();
        let circumference = 2.0 * PI * Self::EARTH_RADIUS * lat_rad.cos();
        let tiles = circumference / (meters_per_pixel * tile_size);
        let zoom = tiles.log2().floor() as i32;
        zoom.clamp(0, 20) as u8
    }
}

/// Viewport transform for screen rendering
#[derive(Clone, Debug)]
pub struct ViewportTransform {
    /// Pan offset in world coordinates
    pub pan: Coord,
    /// Zoom level (can be fractional for smooth zooming)
    pub zoom: f32,
    /// Screen dimensions
    pub width: f32,
    pub height: f32,
}

impl ViewportTransform {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pan: Coord::new(0.0, 0.0),
            zoom: 2.0,
            width,
            height,
        }
    }

    /// Set center coordinate (lon/lat)
    pub fn set_center(&mut self, lon: f32, lat: f32) {
        self.pan = Coord::new(lon, lat);
    }

    /// World coordinate to screen coordinate
    pub fn world_to_screen(&self, coord: Coord) -> Coord {
        // Convert to tile coordinates at this zoom level
        let (tx, ty) = WebMercator::lonlat_to_tile_fraction(coord.x, coord.y, self.zoom as u8);
        let (cx, cy) =
            WebMercator::lonlat_to_tile_fraction(self.pan.x, self.pan.y, self.zoom as u8);

        // 256 pixels per tile
        let tile_size = 256.0;
        let x = (tx - cx) * tile_size + self.width / 2.0;
        let y = (ty - cy) * tile_size + self.height / 2.0;

        Coord::new(x, y)
    }

    /// Screen coordinate to world coordinate
    pub fn screen_to_world(&self, screen: Coord) -> Coord {
        let (cx, cy) =
            WebMercator::lonlat_to_tile_fraction(self.pan.x, self.pan.y, self.zoom as u8);
        let tile_size = 256.0;

        let tx = cx + (screen.x - self.width / 2.0) / tile_size;
        let ty = cy + (screen.y - self.height / 2.0) / tile_size;

        let (lon, lat) = WebMercator::tile_to_lonlat(tx, ty, self.zoom as u8);
        Coord::new(lon, lat)
    }

    /// Get the visible world bounds
    pub fn visible_bounds(&self) -> BoundingBox {
        let tl = self.screen_to_world(Coord::new(0.0, 0.0));
        let br = self.screen_to_world(Coord::new(self.width, self.height));
        BoundingBox::new(
            Coord::new(tl.x.min(br.x), tl.y.min(br.y)),
            Coord::new(tl.x.max(br.x), tl.y.max(br.y)),
        )
    }

    /// Apply zoom centered on a screen point
    pub fn zoom_at(&mut self, screen_point: Coord, delta: f32) {
        let world_before = self.screen_to_world(screen_point);
        self.zoom = (self.zoom + delta).clamp(0.5, 20.0);
        let world_after = self.screen_to_world(screen_point);

        // Adjust pan to keep point under cursor
        self.pan.x += world_before.x - world_after.x;
        self.pan.y += world_before.y - world_after.y;
    }

    /// Pan by screen pixels
    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        let scale = 360.0 / (256.0 * (1u32 << (self.zoom as u8)) as f32);
        self.pan.x -= dx * scale;
        self.pan.y += dy * scale; // Y is inverted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_mercator_roundtrip() {
        let lon = -122.4194; // San Francisco
        let lat = 37.7749;

        let (x, y) = WebMercator::project(lon, lat);
        let (lon2, lat2) = WebMercator::unproject(x, y);

        assert!((lon - lon2).abs() < 1e-4);
        assert!((lat - lat2).abs() < 1e-4);
    }

    #[test]
    fn test_web_mercator_at_equator() {
        let (x, y) = WebMercator::project(0.0, 0.0);
        assert!((x).abs() < 1e-6);
        assert!((y).abs() < 1e-6);
    }

    #[test]
    fn test_web_mercator_clamps_latitude() {
        // Should not panic or produce NaN at extreme latitudes
        let (_, y) = WebMercator::project(0.0, 90.0);
        assert!(y.is_finite());

        let (_, y) = WebMercator::project(0.0, -90.0);
        assert!(y.is_finite());
    }

    #[test]
    fn test_meters_per_pixel() {
        // At equator, zoom 0: ~156km per pixel
        let mpp = WebMercator::meters_per_pixel(0.0, 0);
        assert!(mpp > 100_000.0);

        // Higher zoom = smaller distance per pixel
        let mpp_z10 = WebMercator::meters_per_pixel(0.0, 10);
        assert!(mpp_z10 < mpp);
    }

    #[test]
    fn test_tile_coordinates() {
        // Null Island (0, 0) at zoom 0 should be center of tile
        let (x, y) = WebMercator::lonlat_to_tile_fraction(0.0, 0.0, 0);
        assert!((x - 0.5).abs() < 1e-4);
        assert!((y - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_viewport_transform() {
        let mut viewport = ViewportTransform::new(800.0, 600.0);
        viewport.set_center(0.0, 0.0);
        viewport.zoom = 4.0;

        // Screen center should map back to world center
        let center = viewport.screen_to_world(Coord::new(400.0, 300.0));
        assert!((center.x).abs() < 1.0);
        assert!((center.y).abs() < 1.0);
    }
}
