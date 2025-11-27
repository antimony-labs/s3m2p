//! Zone and exclusion area utilities
//!
//! Provides spatial zone primitives for:
//! - Exclusion zones (areas to avoid)
//! - Dangerous zones (damage/effect areas)
//!
//! ## Traceability
//! - Used by: too.foo (boid avoidance, fungal network), future simulations
//! - Tests: test_exclusion_zone_contains, test_any_zone_check

use glam::Vec2;

/// An exclusion zone - a circular area that entities should avoid or be blocked from
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExclusionZone {
    pub center: Vec2,
    pub radius: f32,
}

impl ExclusionZone {
    /// Create a new exclusion zone
    #[inline]
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check if a point is inside this exclusion zone
    #[inline]
    pub fn contains(&self, pos: Vec2) -> bool {
        pos.distance(self.center) < self.radius
    }

    /// Check if a point is inside this zone with a margin
    #[inline]
    pub fn contains_with_margin(&self, pos: Vec2, margin: f32) -> bool {
        pos.distance(self.center) < self.radius + margin
    }

    /// Get distance from zone edge (negative if inside)
    #[inline]
    pub fn distance_to_edge(&self, pos: Vec2) -> f32 {
        pos.distance(self.center) - self.radius
    }
}

/// Check if a position is inside any of the provided exclusion zones
#[inline]
pub fn is_in_any_exclusion(pos: Vec2, zones: &[ExclusionZone]) -> bool {
    zones.iter().any(|z| z.contains(pos))
}

/// Find the nearest exclusion zone to a point
pub fn nearest_exclusion_zone(pos: Vec2, zones: &[ExclusionZone]) -> Option<(usize, f32)> {
    zones
        .iter()
        .enumerate()
        .map(|(i, z)| (i, pos.distance(z.center)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}

/// Trait for zones that apply effects to entities
pub trait ZoneEffect {
    /// Apply the zone's effect to an entity at the given position
    /// Returns the force/velocity modification and energy change
    fn apply_effect(&self, pos: Vec2, vel: Vec2) -> (Vec2, f32);

    /// Check if position is within the zone's area of effect
    fn is_in_range(&self, pos: Vec2) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclusion_zone_contains() {
        let zone = ExclusionZone::new(Vec2::new(100.0, 100.0), 50.0);

        // Center should be inside
        assert!(zone.contains(Vec2::new(100.0, 100.0)));

        // Edge should be inside (just barely)
        assert!(zone.contains(Vec2::new(149.0, 100.0)));

        // Outside should not be inside
        assert!(!zone.contains(Vec2::new(151.0, 100.0)));
        assert!(!zone.contains(Vec2::new(200.0, 200.0)));
    }

    #[test]
    fn test_exclusion_zone_with_margin() {
        let zone = ExclusionZone::new(Vec2::new(100.0, 100.0), 50.0);

        // Just outside radius but within margin
        assert!(zone.contains_with_margin(Vec2::new(155.0, 100.0), 10.0));

        // Outside even with margin
        assert!(!zone.contains_with_margin(Vec2::new(165.0, 100.0), 10.0));
    }

    #[test]
    fn test_is_in_any_exclusion() {
        let zones = vec![
            ExclusionZone::new(Vec2::new(100.0, 100.0), 30.0),
            ExclusionZone::new(Vec2::new(300.0, 300.0), 50.0),
        ];

        // In first zone
        assert!(is_in_any_exclusion(Vec2::new(100.0, 100.0), &zones));

        // In second zone
        assert!(is_in_any_exclusion(Vec2::new(300.0, 300.0), &zones));

        // In neither zone
        assert!(!is_in_any_exclusion(Vec2::new(200.0, 200.0), &zones));
    }

    #[test]
    fn test_distance_to_edge() {
        let zone = ExclusionZone::new(Vec2::new(100.0, 100.0), 50.0);

        // At center: -50 (inside by 50 units)
        assert!((zone.distance_to_edge(Vec2::new(100.0, 100.0)) - (-50.0)).abs() < 0.001);

        // At edge: 0
        assert!((zone.distance_to_edge(Vec2::new(150.0, 100.0)) - 0.0).abs() < 0.001);

        // Outside: positive
        assert!(zone.distance_to_edge(Vec2::new(160.0, 100.0)) > 0.0);
    }

    #[test]
    fn test_nearest_exclusion_zone() {
        let zones = vec![
            ExclusionZone::new(Vec2::new(100.0, 100.0), 30.0),
            ExclusionZone::new(Vec2::new(300.0, 300.0), 50.0),
        ];

        // Closer to first zone
        let (idx, _dist) = nearest_exclusion_zone(Vec2::new(150.0, 150.0), &zones).unwrap();
        assert_eq!(idx, 0);

        // Closer to second zone
        let (idx, _dist) = nearest_exclusion_zone(Vec2::new(280.0, 280.0), &zones).unwrap();
        assert_eq!(idx, 1);
    }
}
