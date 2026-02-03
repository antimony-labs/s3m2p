// ═══════════════════════════════════════════════════════════════════════════════
// FILE: feature.rs | ATLAS/CORE/GEO_ENGINE/src/feature.rs
// PURPOSE: Geographic feature types with properties and metadata
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use crate::geometry::{BoundingBox, Geometry};
use serde::{Deserialize, Serialize};

/// Compact feature identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FeatureId(pub u32);

impl FeatureId {
    pub const INVALID: Self = Self(u32::MAX);

    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 != u32::MAX
    }
}

impl From<u32> for FeatureId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

/// Feature type classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FeatureType {
    Country = 0,
    Province = 1,
    City = 2,
    River = 3,
    Lake = 4,
    Coastline = 5,
    Road = 6,
    Boundary = 7,
    Unknown = 255,
}

impl From<u8> for FeatureType {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Country,
            1 => Self::Province,
            2 => Self::City,
            3 => Self::River,
            4 => Self::Lake,
            5 => Self::Coastline,
            6 => Self::Road,
            7 => Self::Boundary,
            _ => Self::Unknown,
        }
    }
}

/// Feature properties (name, codes, metadata)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Display name
    pub name: Option<String>,
    /// ISO 3166-1 alpha-3 country code (e.g., "USA", "GBR")
    pub iso_a3: Option<String>,
    /// ISO 3166-1 alpha-2 country code (e.g., "US", "GB")
    pub iso_a2: Option<String>,
    /// Population
    pub population: Option<u64>,
    /// Feature classification
    pub feature_type: FeatureType,
    /// Admin level (0=country, 1=province, 2=county, etc.)
    pub admin_level: Option<u8>,
    /// Scalerank for LOD filtering (0=most important, higher=less important)
    pub scalerank: Option<u8>,
}

impl Properties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_type(mut self, feature_type: FeatureType) -> Self {
        self.feature_type = feature_type;
        self
    }

    pub fn with_iso(mut self, iso_a2: &str, iso_a3: &str) -> Self {
        self.iso_a2 = Some(iso_a2.to_string());
        self.iso_a3 = Some(iso_a3.to_string());
        self
    }
}

impl Default for FeatureType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A geographic feature with geometry and properties
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub id: FeatureId,
    pub geometry: Geometry,
    pub properties: Properties,
    pub bounds: BoundingBox,
}

impl Feature {
    pub fn new(id: FeatureId, geometry: Geometry, properties: Properties) -> Self {
        let bounds = geometry.bounds();
        Self {
            id,
            geometry,
            properties,
            bounds,
        }
    }

    /// Get the display name or a fallback
    pub fn name(&self) -> &str {
        self.properties
            .name
            .as_deref()
            .unwrap_or("Unknown")
    }

    /// Check if this feature should be visible at a given scalerank threshold
    #[inline]
    pub fn visible_at_scalerank(&self, threshold: u8) -> bool {
        self.properties
            .scalerank
            .map(|sr| sr <= threshold)
            .unwrap_or(true)
    }
}

/// Collection of features (e.g., all countries)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounds: BoundingBox,
}

impl FeatureCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            features: Vec::with_capacity(capacity),
            bounds: BoundingBox::EMPTY,
        }
    }

    pub fn push(&mut self, feature: Feature) {
        self.bounds.extend_bbox(&feature.bounds);
        self.features.push(feature);
    }

    pub fn len(&self) -> usize {
        self.features.len()
    }

    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    /// Iterate features that intersect the given bounds
    pub fn query_bounds<'a>(&'a self, bounds: &'a BoundingBox) -> impl Iterator<Item = &'a Feature> {
        self.features
            .iter()
            .filter(move |f| f.bounds.intersects(bounds))
    }

    /// Get feature by ID
    pub fn get(&self, id: FeatureId) -> Option<&Feature> {
        self.features.iter().find(|f| f.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Coord, Polygon, Ring};

    #[test]
    fn test_feature_creation() {
        let ring = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);
        let geom = Geometry::Polygon(Polygon::new(ring));
        let props = Properties::new()
            .with_name("Test Country")
            .with_type(FeatureType::Country);

        let feature = Feature::new(FeatureId::new(1), geom, props);
        assert_eq!(feature.name(), "Test Country");
        assert_eq!(feature.properties.feature_type, FeatureType::Country);
    }

    #[test]
    fn test_feature_collection() {
        let mut collection = FeatureCollection::new();
        assert!(collection.is_empty());

        let ring = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);
        let geom = Geometry::Polygon(Polygon::new(ring));
        let feature = Feature::new(FeatureId::new(1), geom, Properties::default());

        collection.push(feature);
        assert_eq!(collection.len(), 1);
    }
}
