// ═══════════════════════════════════════════════════════════════════════════════
// FILE: format.rs | ATLAS/CORE/GEO_ENGINE/src/format.rs
// PURPOSE: Binary .geo format and GeoJSON parsing
// MODIFIED: 2026-01-25
// ═══════════════════════════════════════════════════════════════════════════════

use crate::feature::{Feature, FeatureCollection, FeatureId, FeatureType, Properties};
use crate::geometry::{BoundingBox, Coord, Geometry, LineString, MultiPolygon, Polygon, Ring};

/// GeoJSON parsing (for importing Natural Earth data)
pub mod geojson {
    use super::*;
    use serde_json::Value;

    /// Parse a GeoJSON FeatureCollection
    pub fn parse_feature_collection(json: &str) -> Result<FeatureCollection, ParseError> {
        let value: Value =
            serde_json::from_str(json).map_err(|e| ParseError::Json(e.to_string()))?;

        let features_array = value
            .get("features")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ParseError::InvalidFormat("Missing 'features' array".into()))?;

        let mut collection = FeatureCollection::with_capacity(features_array.len());

        for (i, feature_value) in features_array.iter().enumerate() {
            match parse_feature(feature_value, i as u32) {
                Ok(feature) => collection.push(feature),
                Err(e) => {
                    // Log but continue - skip malformed features
                    #[cfg(debug_assertions)]
                    eprintln!("Skipping feature {}: {:?}", i, e);
                    let _ = e;
                }
            }
        }

        Ok(collection)
    }

    fn parse_feature(value: &Value, index: u32) -> Result<Feature, ParseError> {
        let geometry = value
            .get("geometry")
            .ok_or_else(|| ParseError::InvalidFormat("Missing geometry".into()))?;

        let properties = value.get("properties").unwrap_or(&Value::Null);

        let geom = parse_geometry(geometry)?;
        let props = parse_properties(properties);
        let id = value
            .get("id")
            .and_then(|v| v.as_u64())
            .map(|id| FeatureId::new(id as u32))
            .unwrap_or_else(|| FeatureId::new(index));

        Ok(Feature::new(id, geom, props))
    }

    fn parse_geometry(value: &Value) -> Result<Geometry, ParseError> {
        let geom_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ParseError::InvalidFormat("Missing geometry type".into()))?;

        let coords = value.get("coordinates");

        match geom_type {
            "Point" => {
                let coord = parse_coord(
                    coords
                        .ok_or_else(|| ParseError::InvalidFormat("Missing coordinates".into()))?,
                )?;
                Ok(Geometry::Point(coord))
            }
            "LineString" => {
                let coords = parse_coord_array(
                    coords
                        .ok_or_else(|| ParseError::InvalidFormat("Missing coordinates".into()))?,
                )?;
                Ok(Geometry::LineString(LineString::new(coords)))
            }
            "Polygon" => {
                let rings = parse_polygon_rings(
                    coords
                        .ok_or_else(|| ParseError::InvalidFormat("Missing coordinates".into()))?,
                )?;
                let mut rings_iter = rings.into_iter();
                let exterior = rings_iter.next().unwrap_or_else(|| Ring::new(vec![]));
                let holes: Vec<Ring> = rings_iter.collect();
                Ok(Geometry::Polygon(Polygon::with_holes(exterior, holes)))
            }
            "MultiPolygon" => {
                let polygons_value = coords
                    .ok_or_else(|| ParseError::InvalidFormat("Missing coordinates".into()))?;
                let polygons = parse_multipolygon(polygons_value)?;
                Ok(Geometry::MultiPolygon(MultiPolygon::new(polygons)))
            }
            _ => Err(ParseError::UnsupportedGeometry(geom_type.into())),
        }
    }

    fn parse_coord(value: &Value) -> Result<Coord, ParseError> {
        let arr = value
            .as_array()
            .ok_or_else(|| ParseError::InvalidFormat("Expected coordinate array".into()))?;
        if arr.len() < 2 {
            return Err(ParseError::InvalidFormat(
                "Coordinate needs at least 2 values".into(),
            ));
        }
        let x = arr[0]
            .as_f64()
            .ok_or_else(|| ParseError::InvalidFormat("Invalid x coordinate".into()))?
            as f32;
        let y = arr[1]
            .as_f64()
            .ok_or_else(|| ParseError::InvalidFormat("Invalid y coordinate".into()))?
            as f32;
        Ok(Coord::new(x, y))
    }

    fn parse_coord_array(value: &Value) -> Result<Vec<Coord>, ParseError> {
        let arr = value
            .as_array()
            .ok_or_else(|| ParseError::InvalidFormat("Expected coordinate array".into()))?;
        arr.iter().map(parse_coord).collect()
    }

    fn parse_polygon_rings(value: &Value) -> Result<Vec<Ring>, ParseError> {
        let arr = value
            .as_array()
            .ok_or_else(|| ParseError::InvalidFormat("Expected rings array".into()))?;
        arr.iter()
            .map(|ring_value| {
                let coords = parse_coord_array(ring_value)?;
                Ok(Ring::new(coords))
            })
            .collect()
    }

    fn parse_multipolygon(value: &Value) -> Result<Vec<Polygon>, ParseError> {
        let arr = value
            .as_array()
            .ok_or_else(|| ParseError::InvalidFormat("Expected multipolygon array".into()))?;
        arr.iter()
            .map(|poly_value| {
                let rings = parse_polygon_rings(poly_value)?;
                let mut rings_iter = rings.into_iter();
                let exterior = rings_iter.next().unwrap_or_else(|| Ring::new(vec![]));
                let holes: Vec<Ring> = rings_iter.collect();
                Ok(Polygon::with_holes(exterior, holes))
            })
            .collect()
    }

    fn parse_properties(value: &Value) -> Properties {
        let mut props = Properties::new();

        if let Some(obj) = value.as_object() {
            // Try common property names
            props.name = obj
                .get("name")
                .or_else(|| obj.get("NAME"))
                .or_else(|| obj.get("ADMIN"))
                .and_then(|v| v.as_str())
                .map(String::from);

            props.iso_a2 = obj
                .get("ISO_A2")
                .or_else(|| obj.get("iso_a2"))
                .and_then(|v| v.as_str())
                .filter(|s| s.len() == 2 && *s != "-99")
                .map(String::from);

            props.iso_a3 = obj
                .get("ISO_A3")
                .or_else(|| obj.get("iso_a3"))
                .and_then(|v| v.as_str())
                .filter(|s| s.len() == 3 && *s != "-99")
                .map(String::from);

            props.population = obj
                .get("POP_EST")
                .or_else(|| obj.get("pop_est"))
                .or_else(|| obj.get("population"))
                .and_then(|v| v.as_f64())
                .map(|p| p as u64);

            props.scalerank = obj
                .get("scalerank")
                .or_else(|| obj.get("SCALERANK"))
                .and_then(|v| v.as_u64())
                .map(|s| s as u8);

            // Determine feature type from featurecla or other hints
            let feature_type = obj
                .get("featurecla")
                .or_else(|| obj.get("FEATURECLA"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_lowercase());

            props.feature_type = match feature_type.as_deref() {
                Some(s) if s.contains("country") || s.contains("admin-0") => FeatureType::Country,
                Some(s) if s.contains("state") || s.contains("admin-1") => FeatureType::Province,
                Some(s) if s.contains("city") || s.contains("populated") => FeatureType::City,
                Some(s) if s.contains("river") => FeatureType::River,
                Some(s) if s.contains("lake") => FeatureType::Lake,
                Some(s) if s.contains("coastline") => FeatureType::Coastline,
                Some(s) if s.contains("road") => FeatureType::Road,
                _ => FeatureType::Unknown,
            };
        }

        props
    }
}

/// Parse error types
#[derive(Debug)]
pub enum ParseError {
    Json(String),
    InvalidFormat(String),
    UnsupportedGeometry(String),
    Io(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Json(e) => write!(f, "JSON error: {}", e),
            ParseError::InvalidFormat(e) => write!(f, "Invalid format: {}", e),
            ParseError::UnsupportedGeometry(e) => write!(f, "Unsupported geometry: {}", e),
            ParseError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

/// Binary format constants
pub mod binary {
    pub const MAGIC: [u8; 4] = *b"GEO1";
    pub const VERSION: u16 = 1;

    // Geometry type codes
    pub const GEOM_POINT: u8 = 0;
    pub const GEOM_LINESTRING: u8 = 1;
    pub const GEOM_POLYGON: u8 = 2;
    pub const GEOM_MULTIPOLYGON: u8 = 3;
}

/// Write features to binary format
pub fn write_binary(features: &[Feature]) -> Vec<u8> {
    let mut buf = Vec::new();

    // Header
    buf.extend_from_slice(&binary::MAGIC);
    buf.extend_from_slice(&binary::VERSION.to_le_bytes());
    buf.extend_from_slice(&(features.len() as u32).to_le_bytes());

    // Calculate bounds
    let mut bounds = BoundingBox::EMPTY;
    for f in features {
        bounds.extend_bbox(&f.bounds);
    }
    buf.extend_from_slice(&bounds.min.x.to_le_bytes());
    buf.extend_from_slice(&bounds.min.y.to_le_bytes());
    buf.extend_from_slice(&bounds.max.x.to_le_bytes());
    buf.extend_from_slice(&bounds.max.y.to_le_bytes());

    // Reserved (padding to 32-byte header)
    buf.extend_from_slice(&[0u8; 6]);

    // Feature data (simplified format for now)
    for feature in features {
        buf.extend_from_slice(&feature.id.0.to_le_bytes());
        write_geometry(&feature.geometry, &mut buf);
        write_properties(&feature.properties, &mut buf);
    }

    buf
}

fn write_geometry(geom: &Geometry, buf: &mut Vec<u8>) {
    match geom {
        Geometry::Point(c) => {
            buf.push(binary::GEOM_POINT);
            buf.extend_from_slice(&c.x.to_le_bytes());
            buf.extend_from_slice(&c.y.to_le_bytes());
        }
        Geometry::LineString(ls) => {
            buf.push(binary::GEOM_LINESTRING);
            buf.extend_from_slice(&(ls.coords.len() as u32).to_le_bytes());
            for c in &ls.coords {
                buf.extend_from_slice(&c.x.to_le_bytes());
                buf.extend_from_slice(&c.y.to_le_bytes());
            }
        }
        Geometry::Polygon(p) => {
            buf.push(binary::GEOM_POLYGON);
            write_ring(&p.exterior, buf);
            buf.extend_from_slice(&(p.holes.len() as u32).to_le_bytes());
            for hole in &p.holes {
                write_ring(hole, buf);
            }
        }
        Geometry::MultiPolygon(mp) => {
            buf.push(binary::GEOM_MULTIPOLYGON);
            buf.extend_from_slice(&(mp.polygons.len() as u32).to_le_bytes());
            for poly in &mp.polygons {
                write_ring(&poly.exterior, buf);
                buf.extend_from_slice(&(poly.holes.len() as u32).to_le_bytes());
                for hole in &poly.holes {
                    write_ring(hole, buf);
                }
            }
        }
    }
}

fn write_ring(ring: &Ring, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(ring.coords.len() as u32).to_le_bytes());
    for c in &ring.coords {
        buf.extend_from_slice(&c.x.to_le_bytes());
        buf.extend_from_slice(&c.y.to_le_bytes());
    }
}

fn write_properties(props: &Properties, buf: &mut Vec<u8>) {
    // Feature type
    buf.push(props.feature_type as u8);

    // Name (length-prefixed string)
    if let Some(name) = &props.name {
        let bytes = name.as_bytes();
        buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(bytes);
    } else {
        buf.extend_from_slice(&0u16.to_le_bytes());
    }

    // ISO codes (fixed 2 + 3 bytes, padded with spaces)
    if let Some(iso) = &props.iso_a2 {
        buf.extend_from_slice(iso.as_bytes());
    } else {
        buf.extend_from_slice(b"  ");
    }
    if let Some(iso) = &props.iso_a3 {
        buf.extend_from_slice(iso.as_bytes());
    } else {
        buf.extend_from_slice(b"   ");
    }

    // Population (optional, 0 = not set)
    buf.extend_from_slice(&props.population.unwrap_or(0).to_le_bytes());

    // Scalerank
    buf.push(props.scalerank.unwrap_or(255));
}

/// Read features from binary format
pub fn read_binary(data: &[u8]) -> Result<FeatureCollection, ParseError> {
    if data.len() < 32 {
        return Err(ParseError::InvalidFormat("File too small".into()));
    }

    // Verify magic
    if data[0..4] != binary::MAGIC {
        return Err(ParseError::InvalidFormat("Invalid magic bytes".into()));
    }

    let version = u16::from_le_bytes([data[4], data[5]]);
    if version != binary::VERSION {
        return Err(ParseError::InvalidFormat(format!(
            "Unsupported version: {}",
            version
        )));
    }

    let feature_count = u32::from_le_bytes([data[6], data[7], data[8], data[9]]) as usize;

    let mut collection = FeatureCollection::with_capacity(feature_count);
    let mut offset = 32; // After header

    for _ in 0..feature_count {
        if offset + 4 > data.len() {
            break;
        }
        let id = FeatureId::new(u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]));
        offset += 4;

        let (geom, new_offset) = read_geometry(data, offset)?;
        offset = new_offset;

        let (props, new_offset) = read_properties(data, offset)?;
        offset = new_offset;

        collection.push(Feature::new(id, geom, props));
    }

    Ok(collection)
}

fn read_geometry(data: &[u8], offset: usize) -> Result<(Geometry, usize), ParseError> {
    if offset >= data.len() {
        return Err(ParseError::InvalidFormat("Unexpected end of data".into()));
    }

    let geom_type = data[offset];
    let mut pos = offset + 1;

    match geom_type {
        binary::GEOM_POINT => {
            let x = f32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            let y = f32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Ok((Geometry::Point(Coord::new(x, y)), pos))
        }
        binary::GEOM_LINESTRING => {
            let (coords, new_pos) = read_coords(data, pos)?;
            Ok((Geometry::LineString(LineString::new(coords)), new_pos))
        }
        binary::GEOM_POLYGON => {
            let (exterior, new_pos) = read_ring(data, pos)?;
            pos = new_pos;
            let hole_count =
                u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                    as usize;
            pos += 4;
            let mut holes = Vec::with_capacity(hole_count);
            for _ in 0..hole_count {
                let (hole, new_pos) = read_ring(data, pos)?;
                holes.push(hole);
                pos = new_pos;
            }
            Ok((Geometry::Polygon(Polygon::with_holes(exterior, holes)), pos))
        }
        binary::GEOM_MULTIPOLYGON => {
            let poly_count =
                u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                    as usize;
            pos += 4;
            let mut polygons = Vec::with_capacity(poly_count);
            for _ in 0..poly_count {
                let (exterior, new_pos) = read_ring(data, pos)?;
                pos = new_pos;
                let hole_count =
                    u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                        as usize;
                pos += 4;
                let mut holes = Vec::with_capacity(hole_count);
                for _ in 0..hole_count {
                    let (hole, new_pos) = read_ring(data, pos)?;
                    holes.push(hole);
                    pos = new_pos;
                }
                polygons.push(Polygon::with_holes(exterior, holes));
            }
            Ok((Geometry::MultiPolygon(MultiPolygon::new(polygons)), pos))
        }
        _ => Err(ParseError::UnsupportedGeometry(format!(
            "Unknown type: {}",
            geom_type
        ))),
    }
}

fn read_coords(data: &[u8], offset: usize) -> Result<(Vec<Coord>, usize), ParseError> {
    let count = u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ]) as usize;
    let mut pos = offset + 4;
    let mut coords = Vec::with_capacity(count);

    for _ in 0..count {
        let x = f32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        let y = f32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        coords.push(Coord::new(x, y));
    }

    Ok((coords, pos))
}

fn read_ring(data: &[u8], offset: usize) -> Result<(Ring, usize), ParseError> {
    let (coords, pos) = read_coords(data, offset)?;
    Ok((Ring::new(coords), pos))
}

fn read_properties(data: &[u8], offset: usize) -> Result<(Properties, usize), ParseError> {
    let mut props = Properties::new();
    let mut pos = offset;

    // Feature type
    props.feature_type = FeatureType::from(data[pos]);
    pos += 1;

    // Name
    let name_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;
    if name_len > 0 {
        props.name = Some(String::from_utf8_lossy(&data[pos..pos + name_len]).to_string());
        pos += name_len;
    }

    // ISO codes
    let iso_a2 = String::from_utf8_lossy(&data[pos..pos + 2]).to_string();
    pos += 2;
    if iso_a2.trim() != "" {
        props.iso_a2 = Some(iso_a2);
    }

    let iso_a3 = String::from_utf8_lossy(&data[pos..pos + 3]).to_string();
    pos += 3;
    if iso_a3.trim() != "" {
        props.iso_a3 = Some(iso_a3);
    }

    // Population
    let pop = u64::from_le_bytes([
        data[pos],
        data[pos + 1],
        data[pos + 2],
        data[pos + 3],
        data[pos + 4],
        data[pos + 5],
        data[pos + 6],
        data[pos + 7],
    ]);
    pos += 8;
    if pop > 0 {
        props.population = Some(pop);
    }

    // Scalerank
    let scalerank = data[pos];
    pos += 1;
    if scalerank != 255 {
        props.scalerank = Some(scalerank);
    }

    Ok((props, pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geojson_parse_point() {
        let json = r#"{
            "type": "FeatureCollection",
            "features": [{
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.0, 2.0]
                },
                "properties": {"name": "Test Point"}
            }]
        }"#;

        let collection = geojson::parse_feature_collection(json).unwrap();
        assert_eq!(collection.len(), 1);

        let feature = &collection.features[0];
        assert_eq!(feature.name(), "Test Point");
        match &feature.geometry {
            Geometry::Point(c) => {
                assert!((c.x - 1.0).abs() < 1e-6);
                assert!((c.y - 2.0).abs() < 1e-6);
            }
            _ => panic!("Expected Point geometry"),
        }
    }

    #[test]
    fn test_geojson_parse_polygon() {
        let json = r#"{
            "type": "FeatureCollection",
            "features": [{
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [[[0,0],[10,0],[10,10],[0,10],[0,0]]]
                },
                "properties": {"name": "Test Country", "ISO_A3": "TST"}
            }]
        }"#;

        let collection = geojson::parse_feature_collection(json).unwrap();
        assert_eq!(collection.len(), 1);

        let feature = &collection.features[0];
        assert_eq!(feature.properties.iso_a3.as_deref(), Some("TST"));
    }

    #[test]
    fn test_binary_roundtrip() {
        let ring = Ring::new(vec![
            Coord::new(0.0, 0.0),
            Coord::new(10.0, 0.0),
            Coord::new(10.0, 10.0),
            Coord::new(0.0, 10.0),
            Coord::new(0.0, 0.0),
        ]);
        let geom = Geometry::Polygon(Polygon::new(ring));
        let props = Properties::new()
            .with_name("Test")
            .with_type(FeatureType::Country);
        let feature = Feature::new(FeatureId::new(1), geom, props);

        let binary = write_binary(&[feature.clone()]);
        let parsed = read_binary(&binary).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed.features[0].name(), "Test");
    }
}
