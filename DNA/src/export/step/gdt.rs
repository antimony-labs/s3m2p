//! Geometric Dimensioning and Tolerancing (GD&T) entities (ISO 10303-242)
//!
//! Datum reference frames for manufacturing:
//! - DATUM: Individual datum features (A, B, C)
//! - DATUM_SYSTEM: Collection of datums forming DRF
//! - Geometric tolerances: flatness, perpendicularity, position, straightness
//! - SHAPE_ASPECT: Features/aspects that tolerances apply to

use super::entities::*;
use std::io::{self, Write};

/// SHAPE_ASPECT - feature or aspect of a shape that tolerances can reference
/// Used to identify specific faces, edges, or features for GD&T
pub struct ShapeAspect {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub of_shape: EntityId, // ProductDefinitionShape
    pub product_definitional: bool,
}

impl StepEntity for ShapeAspect {
    fn entity_name(&self) -> &'static str {
        "SHAPE_ASPECT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.of_shape)?;
        write!(w, ",{}", if self.product_definitional { ".T." } else { ".F." })
    }
}

/// DATUM - datum feature for GD&T reference frame
/// identification = "A", "B", or "C"
pub struct Datum {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub of_shape: EntityId, // ShapeAspect or ProductDefinitionShape
    pub product_definitional: bool,
    pub identification: String, // "A", "B", "C"
}

impl StepEntity for Datum {
    fn entity_name(&self) -> &'static str {
        "DATUM"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.of_shape)?;
        write!(w, ",{}", if self.product_definitional { ".T." } else { ".F." })?;
        write!(w, ",'{}'", self.identification)
    }
}

/// DATUM_REFERENCE_COMPARTMENT - individual datum reference within a tolerance
pub struct DatumReferenceCompartment {
    pub id: EntityId,
    pub datum: EntityId, // References a Datum
}

impl StepEntity for DatumReferenceCompartment {
    fn entity_name(&self) -> &'static str {
        "DATUM_REFERENCE_COMPARTMENT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "{}", self.datum)
    }
}

/// DATUM_REFERENCE - reference to a specific datum
pub struct DatumReference {
    pub id: EntityId,
    pub precedence: u32, // 1 = primary, 2 = secondary, 3 = tertiary
    pub referenced_datum: EntityId, // Datum
}

impl StepEntity for DatumReference {
    fn entity_name(&self) -> &'static str {
        "DATUM_REFERENCE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "{},{}", self.precedence, self.referenced_datum)
    }
}

/// DATUM_SYSTEM - collection of datums forming a DRF (A|B|C)
pub struct DatumSystem {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub of_shape: EntityId, // ProductDefinitionShape
    pub product_definitional: bool,
    pub constituents: Vec<EntityId>, // DatumReference entities (primary, secondary, tertiary)
}

impl StepEntity for DatumSystem {
    fn entity_name(&self) -> &'static str {
        "DATUM_SYSTEM"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.of_shape)?;
        write!(w, ",{}", if self.product_definitional { ".T." } else { ".F." })?;
        write!(w, ",")?;
        write_entity_list(&self.constituents, w)
    }
}

/// LENGTH_MEASURE_WITH_UNIT - dimensional value with units (for tolerance magnitudes)
pub struct LengthMeasureWithUnit {
    pub id: EntityId,
    pub value: f64,
    pub unit: EntityId, // Length unit (typically SI_UNIT for mm or inch)
}

impl StepEntity for LengthMeasureWithUnit {
    fn entity_name(&self) -> &'static str {
        "( LENGTH_MEASURE_WITH_UNIT() MEASURE_REPRESENTATION_ITEM() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "'Tolerance Magnitude',{:.6},{}", self.value, self.unit)
    }
}

/// MEASURE_QUALIFIER - qualifies a measure (e.g., "maximum", "minimum")
pub struct MeasureQualifier {
    pub id: EntityId,
    pub name: String,
    pub description: String,
}

impl StepEntity for MeasureQualifier {
    fn entity_name(&self) -> &'static str {
        "MEASURE_QUALIFIER"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_step_string(&self.description, w)
    }
}

/// Base structure for geometric tolerances
pub struct GeometricToleranceBase {
    pub name: String,
    pub description: Option<String>,
    pub magnitude: EntityId, // LengthMeasureWithUnit
    pub toleranced_shape_aspect: EntityId, // ShapeAspect
}

/// FLATNESS_TOLERANCE - flatness tolerance (no datum reference needed)
pub struct FlatnessTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
}

impl StepEntity for FlatnessTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() FLATNESS_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)
    }
}

/// STRAIGHTNESS_TOLERANCE - straightness tolerance
pub struct StraightnessTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
}

impl StepEntity for StraightnessTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() STRAIGHTNESS_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)
    }
}

/// PERPENDICULARITY_TOLERANCE - perpendicularity tolerance with datum reference
pub struct PerpendicularityTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
    pub datum_system: EntityId, // DatumSystem or datum references
}

impl StepEntity for PerpendicularityTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE() PERPENDICULARITY_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)?;
        write!(w, ",({})", self.datum_system)
    }
}

/// POSITION_TOLERANCE - position tolerance with datum reference (for hole/fastener locations)
pub struct PositionTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
    pub datum_system: EntityId, // DatumSystem
}

impl StepEntity for PositionTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE() POSITION_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)?;
        write!(w, ",({})", self.datum_system)
    }
}

/// ANGULARITY_TOLERANCE - angularity tolerance with datum reference
pub struct AngularityTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
    pub datum_system: EntityId,
}

impl StepEntity for AngularityTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE() ANGULARITY_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)?;
        write!(w, ",({})", self.datum_system)
    }
}

/// PARALLELISM_TOLERANCE - parallelism tolerance with datum reference
pub struct ParallelismTolerance {
    pub id: EntityId,
    pub base: GeometricToleranceBase,
    pub datum_system: EntityId,
}

impl StepEntity for ParallelismTolerance {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_TOLERANCE() GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE() PARALLELISM_TOLERANCE() )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.base.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.base.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},{}", self.base.magnitude, self.base.toleranced_shape_aspect)?;
        write!(w, ",({})", self.datum_system)
    }
}

/// GEOMETRIC_TOLERANCE - simplified general tolerance entity
pub struct GeometricTolerance {
    pub id: EntityId,
    pub name: Option<String>,
    pub value: f64,
}

impl StepEntity for GeometricTolerance {
    fn entity_name(&self) -> &'static str {
        "GEOMETRIC_TOLERANCE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{:.6}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_aspect() {
        let aspect = ShapeAspect {
            id: EntityId(100),
            name: "Top Face".to_string(),
            description: Some("Datum feature A".to_string()),
            of_shape: EntityId(50),
            product_definitional: true,
        };

        let mut output = Vec::new();
        aspect.write_entity(aspect.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("SHAPE_ASPECT"));
        assert!(result.contains("Top Face"));
        assert!(result.contains(".T."));
    }

    #[test]
    fn test_datum() {
        let datum = Datum {
            id: EntityId(200),
            name: "Datum A".to_string(),
            description: None,
            of_shape: EntityId(100),
            product_definitional: false,
            identification: "A".to_string(),
        };

        let mut output = Vec::new();
        datum.write_entity(datum.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("DATUM"));
        assert!(result.contains("'A'"));
        assert!(result.contains(".F."));
    }

    #[test]
    fn test_datum_system() {
        let system = DatumSystem {
            id: EntityId(300),
            name: "Primary DRF".to_string(),
            description: Some("A|B|C".to_string()),
            of_shape: EntityId(50),
            product_definitional: true,
            constituents: vec![EntityId(201), EntityId(202), EntityId(203)],
        };

        let mut output = Vec::new();
        system.write_entity(system.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("DATUM_SYSTEM"));
        assert!(result.contains("Primary DRF"));
        assert!(result.contains("#201"));
        assert!(result.contains("#202"));
        assert!(result.contains("#203"));
    }

    #[test]
    fn test_flatness_tolerance() {
        let tolerance = FlatnessTolerance {
            id: EntityId(400),
            base: GeometricToleranceBase {
                name: "Flatness 0.125\"".to_string(),
                description: None,
                magnitude: EntityId(350),
                toleranced_shape_aspect: EntityId(100),
            },
        };

        let mut output = Vec::new();
        tolerance.write_entity(tolerance.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("GEOMETRIC_TOLERANCE"));
        assert!(result.contains("FLATNESS_TOLERANCE"));
        assert!(result.contains("Flatness"));
    }

    #[test]
    fn test_position_tolerance() {
        let tolerance = PositionTolerance {
            id: EntityId(500),
            base: GeometricToleranceBase {
                name: "Position 0.25\" A|B|C".to_string(),
                description: Some("Nailing coordinates".to_string()),
                magnitude: EntityId(450),
                toleranced_shape_aspect: EntityId(100),
            },
            datum_system: EntityId(300),
        };

        let mut output = Vec::new();
        tolerance.write_entity(tolerance.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("GEOMETRIC_TOLERANCE"));
        assert!(result.contains("POSITION_TOLERANCE"));
        assert!(result.contains("GEOMETRIC_TOLERANCE_WITH_DATUM_REFERENCE"));
        assert!(result.contains("#300"));
    }

    #[test]
    fn test_perpendicularity_tolerance() {
        let tolerance = PerpendicularityTolerance {
            id: EntityId(600),
            base: GeometricToleranceBase {
                name: "Perpendicularity 0.0625\"".to_string(),
                description: None,
                magnitude: EntityId(550),
                toleranced_shape_aspect: EntityId(120),
            },
            datum_system: EntityId(300),
        };

        let mut output = Vec::new();
        tolerance.write_entity(tolerance.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("PERPENDICULARITY_TOLERANCE"));
        assert!(result.contains("Perpendicularity"));
    }
}
