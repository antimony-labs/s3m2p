//! Product Manufacturing Information (PMI) entities (ISO 10303-242)
//!
//! Dimensional tolerances, material designations, and manufacturing annotations

use super::entities::*;
use std::io::{self, Write};

/// DIMENSIONAL_LOCATION - linear distance between two features
pub struct DimensionalLocation {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub relating_shape_aspect: EntityId,
    pub related_shape_aspect: EntityId,
}

impl StepEntity for DimensionalLocation {
    fn entity_name(&self) -> &'static str {
        "DIMENSIONAL_LOCATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(
            w,
            ",{},{}",
            self.relating_shape_aspect, self.related_shape_aspect
        )
    }
}

/// DIMENSIONAL_SIZE - dimensional size constraint
pub struct DimensionalSize {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub applies_to: EntityId, // ShapeAspect
}

impl StepEntity for DimensionalSize {
    fn entity_name(&self) -> &'static str {
        "DIMENSIONAL_SIZE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.applies_to)
    }
}

/// PLUS_MINUS_TOLERANCE - symmetric tolerance (e.g., ±0.125")
pub struct PlusMinusTolerance {
    pub id: EntityId,
    pub name: String,
    pub nominal_value: f64,
    pub upper_bound: f64, // positive tolerance
    pub lower_bound: f64, // negative tolerance
    pub unit: EntityId,   // Length unit
}

impl StepEntity for PlusMinusTolerance {
    fn entity_name(&self) -> &'static str {
        "PLUS_MINUS_TOLERANCE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(
            w,
            ",{:.6},{:.6},{:.6},{}",
            self.nominal_value, self.upper_bound, self.lower_bound, self.unit
        )
    }
}

/// LIMITS_AND_FITS - tolerance specified by upper and lower limits
pub struct LimitsAndFits {
    pub id: EntityId,
    pub name: String,
    pub upper_limit: f64,
    pub lower_limit: f64,
    pub unit: EntityId,
}

impl StepEntity for LimitsAndFits {
    fn entity_name(&self) -> &'static str {
        "LIMITS_AND_FITS"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(
            w,
            ",{:.6},{:.6},{}",
            self.upper_limit, self.lower_limit, self.unit
        )
    }
}

/// MATERIAL_DESIGNATION - material specification (e.g., "ASTM D245 No.2 Southern Pine")
pub struct MaterialDesignation {
    pub id: EntityId,
    pub name: String,
    pub specification: String, // e.g., "ASTM D245 No.2 Southern Pine"
}

impl StepEntity for MaterialDesignation {
    fn entity_name(&self) -> &'static str {
        "MATERIAL_DESIGNATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_step_string(&self.specification, w)
    }
}

/// MATERIAL_PROPERTY - specific material property value
pub struct MaterialProperty {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub property_value: f64,
    pub unit: Option<EntityId>,
}

impl StepEntity for MaterialProperty {
    fn entity_name(&self) -> &'static str {
        "MATERIAL_PROPERTY"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_step_string(&self.description, w)?;
        write!(w, ",{:.6}", self.property_value)?;
        if let Some(unit) = self.unit {
            write!(w, ",{}", unit)?;
        } else {
            write!(w, ",$")?;
        }
        Ok(())
    }
}

/// DESCRIPTIVE_REPRESENTATION_ITEM - text annotation
pub struct DescriptiveRepresentationItem {
    pub id: EntityId,
    pub name: String,
    pub description: String,
}

impl StepEntity for DescriptiveRepresentationItem {
    fn entity_name(&self) -> &'static str {
        "DESCRIPTIVE_REPRESENTATION_ITEM"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_step_string(&self.description, w)
    }
}

/// DRAUGHTING_ANNOTATION - 2D annotation for drawings
pub struct DraughtingAnnotation {
    pub id: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub relates_to: EntityId, // ShapeAspect or ProductDefinitionShape
}

impl StepEntity for DraughtingAnnotation {
    fn entity_name(&self) -> &'static str {
        "DRAUGHTING_ANNOTATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.relates_to)
    }
}

/// DIMENSIONAL_CHARACTERISTIC_REPRESENTATION - full dimension with tolerance
pub struct DimensionalCharacteristicRepresentation {
    pub id: EntityId,
    pub dimension: EntityId,      // DimensionalSize or DimensionalLocation
    pub representation: EntityId, // PlusMinusTolerance or LimitsAndFits
}

impl StepEntity for DimensionalCharacteristicRepresentation {
    fn entity_name(&self) -> &'static str {
        "DIMENSIONAL_CHARACTERISTIC_REPRESENTATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "{},{}", self.dimension, self.representation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensional_location() {
        let dim = DimensionalLocation {
            id: EntityId(100),
            name: "Skid Spacing".to_string(),
            description: Some("Distance between skids".to_string()),
            relating_shape_aspect: EntityId(50),
            related_shape_aspect: EntityId(51),
        };

        let mut output = Vec::new();
        dim.write_entity(dim.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("DIMENSIONAL_LOCATION"));
        assert!(result.contains("Skid Spacing"));
        assert!(result.contains("Distance between skids"));
    }

    #[test]
    fn test_plus_minus_tolerance() {
        let tol = PlusMinusTolerance {
            id: EntityId(200),
            name: "Length Tolerance".to_string(),
            nominal_value: 120.0,
            upper_bound: 0.125,
            lower_bound: 0.125,
            unit: EntityId(10),
        };

        let mut output = Vec::new();
        tol.write_entity(tol.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("PLUS_MINUS_TOLERANCE"));
        assert!(result.contains("120.000000"));
        assert!(result.contains("0.125000"));
    }

    #[test]
    fn test_material_designation() {
        let mat = MaterialDesignation {
            id: EntityId(300),
            name: "Skid Material".to_string(),
            specification: "ASTM D245 No.2 Southern Pine".to_string(),
        };

        let mut output = Vec::new();
        mat.write_entity(mat.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("MATERIAL_DESIGNATION"));
        assert!(result.contains("Skid Material"));
        assert!(result.contains("ASTM D245"));
        assert!(result.contains("No.2 Southern Pine"));
    }

    #[test]
    fn test_dimensional_size() {
        let size = DimensionalSize {
            id: EntityId(400),
            name: "Lumber Width".to_string(),
            description: Some("Actual 3.5 inches".to_string()),
            applies_to: EntityId(100),
        };

        let mut output = Vec::new();
        size.write_entity(size.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("DIMENSIONAL_SIZE"));
        assert!(result.contains("Lumber Width"));
    }

    #[test]
    fn test_limits_and_fits() {
        let laf = LimitsAndFits {
            id: EntityId(500),
            name: "Hole Size".to_string(),
            upper_limit: 0.175,
            lower_limit: 0.150,
            unit: EntityId(10),
        };

        let mut output = Vec::new();
        laf.write_entity(laf.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("LIMITS_AND_FITS"));
        assert!(result.contains("0.175000"));
        assert!(result.contains("0.150000"));
    }

    #[test]
    fn test_descriptive_representation_item() {
        let desc = DescriptiveRepresentationItem {
            id: EntityId(600),
            name: "Assembly Note".to_string(),
            description: "Apply wood glue before nailing".to_string(),
        };

        let mut output = Vec::new();
        desc.write_entity(desc.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("DESCRIPTIVE_REPRESENTATION_ITEM"));
        assert!(result.contains("Assembly Note"));
        assert!(result.contains("wood glue"));
    }
}
