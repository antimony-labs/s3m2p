//! STEP product structure entities (ISO 10303-242)
//!
//! Product hierarchy for PMI/GD&T attachment:
//! APPLICATION_CONTEXT -> PRODUCT_CONTEXT -> PRODUCT -> PRODUCT_DEFINITION_FORMATION
//! -> PRODUCT_DEFINITION -> PRODUCT_DEFINITION_SHAPE -> SHAPE_REPRESENTATION

use super::entities::*;
use std::io::{self, Write};

/// APPLICATION_CONTEXT - top level context
pub struct ApplicationContext {
    pub id: EntityId,
    pub application: String,
}

impl StepEntity for ApplicationContext {
    fn entity_name(&self) -> &'static str {
        "APPLICATION_CONTEXT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "'{}'", self.application)
    }
}

/// PRODUCT_CONTEXT - context for product definition
pub struct ProductContext {
    pub id: EntityId,
    pub name: String,
    pub frame_of_reference: EntityId, // ApplicationContext
    pub discipline_type: String,
}

impl StepEntity for ProductContext {
    fn entity_name(&self) -> &'static str {
        "PRODUCT_CONTEXT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",{}", self.frame_of_reference)?;
        write!(w, ",'{}'", self.discipline_type)
    }
}

/// PRODUCT - main product entity
pub struct Product {
    pub id: EntityId,
    pub id_string: String,
    pub name: String,
    pub description: String,
    pub frame_of_reference: Vec<EntityId>, // ProductContext(s)
}

impl StepEntity for Product {
    fn entity_name(&self) -> &'static str {
        "PRODUCT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "'{}'", self.id_string)?;
        write!(w, ",")?;
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_step_string(&self.description, w)?;
        write!(w, ",")?;
        write_entity_list(&self.frame_of_reference, w)
    }
}

/// PRODUCT_DEFINITION_FORMATION - version/release of product
pub struct ProductDefinitionFormation {
    pub id: EntityId,
    pub id_string: String,
    pub description: Option<String>,
    pub of_product: EntityId, // Product
}

impl StepEntity for ProductDefinitionFormation {
    fn entity_name(&self) -> &'static str {
        "PRODUCT_DEFINITION_FORMATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "'{}'", self.id_string)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.of_product)
    }
}

/// PRODUCT_DEFINITION_CONTEXT - context for definition
pub struct ProductDefinitionContext {
    pub id: EntityId,
    pub name: String,
    pub frame_of_reference: EntityId, // ApplicationContext
    pub life_cycle_stage: String,
}

impl StepEntity for ProductDefinitionContext {
    fn entity_name(&self) -> &'static str {
        "PRODUCT_DEFINITION_CONTEXT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",{}", self.frame_of_reference)?;
        write!(w, ",'{}'", self.life_cycle_stage)
    }
}

/// PRODUCT_DEFINITION - specific design version
pub struct ProductDefinition {
    pub id: EntityId,
    pub id_string: String,
    pub description: Option<String>,
    pub formation: EntityId,          // ProductDefinitionFormation
    pub frame_of_reference: EntityId, // ProductDefinitionContext
}

impl StepEntity for ProductDefinition {
    fn entity_name(&self) -> &'static str {
        "PRODUCT_DEFINITION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "'{}'", self.id_string)?;
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.formation)?;
        write!(w, ",{}", self.frame_of_reference)
    }
}

/// PRODUCT_DEFINITION_SHAPE - associates shape with definition
pub struct ProductDefinitionShape {
    pub id: EntityId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub definition: EntityId, // ProductDefinition
}

impl StepEntity for ProductDefinitionShape {
    fn entity_name(&self) -> &'static str {
        "PRODUCT_DEFINITION_SHAPE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",")?;
        if let Some(desc) = &self.description {
            write_step_string(desc, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.definition)
    }
}

/// SHAPE_REPRESENTATION - geometric representation
pub struct ShapeRepresentation {
    pub id: EntityId,
    pub name: String,
    pub items: Vec<EntityId>, // GeometricRepresentationItems (e.g., ManifoldSolidBrep)
    pub context_of_items: EntityId, // GeometricRepresentationContext
}

impl StepEntity for ShapeRepresentation {
    fn entity_name(&self) -> &'static str {
        "SHAPE_REPRESENTATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.name, w)?;
        write!(w, ",")?;
        write_entity_list(&self.items, w)?;
        write!(w, ",{}", self.context_of_items)
    }
}

/// SHAPE_DEFINITION_REPRESENTATION - links shape to representation
pub struct ShapeDefinitionRepresentation {
    pub id: EntityId,
    pub definition: EntityId,          // ProductDefinitionShape
    pub used_representation: EntityId, // ShapeRepresentation
}

impl StepEntity for ShapeDefinitionRepresentation {
    fn entity_name(&self) -> &'static str {
        "SHAPE_DEFINITION_REPRESENTATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write!(w, "{},{}", self.definition, self.used_representation)
    }
}

/// GEOMETRIC_REPRESENTATION_CONTEXT - context for geometry
pub struct GeometricRepresentationContext {
    pub id: EntityId,
    pub context_identifier: String,
    pub context_type: String,
}

impl StepEntity for GeometricRepresentationContext {
    fn entity_name(&self) -> &'static str {
        "( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#ID)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#ID,#ID,#ID)) REPRESENTATION_CONTEXT )"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        write_step_string(&self.context_identifier, w)?;
        write!(w, ",")?;
        write_step_string(&self.context_type, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_entity() {
        let product = Product {
            id: EntityId(100),
            id_string: "AUTOCRATE-001".to_string(),
            name: "AutoCrate Assembly".to_string(),
            description: "ASTM D6039 Shipping Crate".to_string(),
            frame_of_reference: vec![EntityId(10)],
        };

        let mut output = Vec::new();
        product.write_entity(product.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("PRODUCT"));
        assert!(result.contains("AUTOCRATE-001"));
        assert!(result.contains("AutoCrate Assembly"));
    }

    #[test]
    fn test_product_definition() {
        let def = ProductDefinition {
            id: EntityId(200),
            id_string: "DESIGN".to_string(),
            description: Some("Design version".to_string()),
            formation: EntityId(150),
            frame_of_reference: EntityId(50),
        };

        let mut output = Vec::new();
        def.write_entity(def.id, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("PRODUCT_DEFINITION"));
        assert!(result.contains("DESIGN"));
    }
}
