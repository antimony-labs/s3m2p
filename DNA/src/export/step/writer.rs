//! STEP Part 21 file writer

use super::entities::*;
use super::primitives::{CartesianPoint, Direction, Axis2Placement3D, Vector, Line, Plane};
use super::topology::{VertexPoint, EdgeCurve, OrientedEdge, FaceBound, AdvancedFace, ClosedShell, ManifoldSolidBrep};
use super::product::*;
use super::gdt::*;
use super::pmi::*;
use std::io::{self, Write};

pub struct StepWriter {
    id_gen: EntityIdGenerator,
    entities: Vec<(EntityId, Box<dyn StepEntity>)>,
}

impl StepWriter {
    pub fn new() -> Self {
        Self {
            id_gen: EntityIdGenerator::new(),
            entities: Vec::new(),
        }
    }

    /// Add a cartesian point and return its ID
    pub fn add_point(&mut self, name: Option<&str>, x: f64, y: f64, z: f64) -> EntityId {
        let id = self.id_gen.next();
        let point = CartesianPoint {
            id,
            name: name.map(|s| s.to_string()),
            coordinates: [x, y, z],
        };
        self.entities.push((id, Box::new(point)));
        id
    }

    /// Add an axis2_placement_3d and return its ID
    pub fn add_axis2_placement_3d(&mut self, name: Option<&str>, location: EntityId, axis: Option<EntityId>, ref_direction: Option<EntityId>) -> EntityId {
        let id = self.id_gen.next();
        let axis2_placement_3d = Axis2Placement3D {
            id,
            name: name.map(|s| s.to_string()),
            location,
            axis,
            ref_direction,
        };
        self.entities.push((id, Box::new(axis2_placement_3d)));
        id
    }

    /// Add a direction and return its ID
    pub fn add_direction(&mut self, name: Option<&str>, x: f64, y: f64, z: f64) -> EntityId {
        let id = self.id_gen.next();
        let dir = Direction {
            id,
            name: name.map(|s| s.to_string()),
            ratios: [x, y, z],
        };
        self.entities.push((id, Box::new(dir)));
        id
    }

    /// Add a vector and return its ID
    pub fn add_vector(&mut self, name: Option<&str>, orientation: EntityId, magnitude: f64) -> EntityId {
        let id = self.id_gen.next();
        let vec = Vector {
            id,
            name: name.map(|s| s.to_string()),
            orientation,
            magnitude,
        };
        self.entities.push((id, Box::new(vec)));
        id
    }

    /// Add a line and return its ID
    pub fn add_line(&mut self, name: Option<&str>, pnt: EntityId, dir: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let line = Line {
            id,
            name: name.map(|s| s.to_string()),
            pnt,
            dir,
        };
        self.entities.push((id, Box::new(line)));
        id
    }

    /// Add a plane and return its ID
    pub fn add_plane(&mut self, name: Option<&str>, position: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let plane = Plane {
            id,
            name: name.map(|s| s.to_string()),
            position,
        };
        self.entities.push((id, Box::new(plane)));
        id
    }

    /// Add a vertex point and return its ID
    pub fn add_vertex_point(&mut self, name: Option<&str>, vertex_geometry: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let vertex_point = VertexPoint {
            id,
            name: name.map(|s| s.to_string()),
            vertex_geometry,
        };
        self.entities.push((id, Box::new(vertex_point)));
        id
    }

    /// Add an edge curve and return its ID
    pub fn add_edge_curve(&mut self, name: Option<&str>, edge_start: EntityId, edge_end: EntityId, edge_geometry: EntityId, same_sense: bool) -> EntityId {
        let id = self.id_gen.next();
        let edge_curve = EdgeCurve {
            id,
            name: name.map(|s| s.to_string()),
            edge_start,
            edge_end,
            edge_geometry,
            same_sense,
        };
        self.entities.push((id, Box::new(edge_curve)));
        id
    }

    /// Add an oriented edge and return its ID
    pub fn add_oriented_edge(&mut self, name: Option<&str>, edge_element: EntityId, orientation: bool) -> EntityId {
        let id = self.id_gen.next();
        let oriented_edge = OrientedEdge {
            id,
            name: name.map(|s| s.to_string()),
            edge_element,
            orientation,
        };
        self.entities.push((id, Box::new(oriented_edge)));
        id
    }

    /// Add a face bound and return its ID
    pub fn add_face_bound(&mut self, name: Option<&str>, bound: Vec<EntityId>, orientation: bool) -> EntityId {
        let id = self.id_gen.next();
        let face_bound = FaceBound {
            id,
            name: name.map(|s| s.to_string()),
            bound,
            orientation,
        };
        self.entities.push((id, Box::new(face_bound)));
        id
    }

    /// Add an advanced face and return its ID
    pub fn add_advanced_face(&mut self, name: Option<&str>, face_geometry: EntityId, face_bounds: Vec<EntityId>) -> EntityId {
        let id = self.id_gen.next();
        let advanced_face = AdvancedFace {
            id,
            name: name.map(|s| s.to_string()),
            face_geometry,
            face_bounds,
        };
        self.entities.push((id, Box::new(advanced_face)));
        id
    }

    /// Add a closed shell and return its ID
    pub fn add_closed_shell(&mut self, name: Option<&str>, cfs_faces: Vec<EntityId>) -> EntityId {
        let id = self.id_gen.next();
        let closed_shell = ClosedShell {
            id,
            name: name.map(|s| s.to_string()),
            cfs_faces,
        };
        self.entities.push((id, Box::new(closed_shell)));
        id
    }

    /// Add a manifold solid B-rep and return its ID
    pub fn add_manifold_solid_brep(&mut self, name: Option<&str>, outer: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let manifold_solid_brep = ManifoldSolidBrep {
            id,
            name: name.map(|s| s.to_string()),
            outer,
        };
        self.entities.push((id, Box::new(manifold_solid_brep)));
        id
    }

    // ===== Product Structure Helpers =====

    /// Add an APPLICATION_CONTEXT
    pub fn add_application_context(&mut self, application: &str) -> EntityId {
        let id = self.id_gen.next();
        let ctx = ApplicationContext {
            id,
            application: application.to_string(),
        };
        self.entities.push((id, Box::new(ctx)));
        id
    }

    /// Add a PRODUCT_CONTEXT
    pub fn add_product_context(&mut self, name: &str, frame_of_reference: EntityId, discipline_type: &str) -> EntityId {
        let id = self.id_gen.next();
        let ctx = ProductContext {
            id,
            name: name.to_string(),
            frame_of_reference,
            discipline_type: discipline_type.to_string(),
        };
        self.entities.push((id, Box::new(ctx)));
        id
    }

    /// Add a PRODUCT
    pub fn add_product(&mut self, id_string: &str, name: &str, description: &str, frame_of_reference: Vec<EntityId>) -> EntityId {
        let id = self.id_gen.next();
        let product = Product {
            id,
            id_string: id_string.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            frame_of_reference,
        };
        self.entities.push((id, Box::new(product)));
        id
    }

    /// Add a PRODUCT_DEFINITION_FORMATION
    pub fn add_product_definition_formation(&mut self, id_string: &str, description: Option<String>, of_product: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let formation = ProductDefinitionFormation {
            id,
            id_string: id_string.to_string(),
            description,
            of_product,
        };
        self.entities.push((id, Box::new(formation)));
        id
    }

    /// Add a PRODUCT_DEFINITION_CONTEXT
    pub fn add_product_definition_context(&mut self, name: &str, frame_of_reference: EntityId, life_cycle_stage: &str) -> EntityId {
        let id = self.id_gen.next();
        let ctx = ProductDefinitionContext {
            id,
            name: name.to_string(),
            frame_of_reference,
            life_cycle_stage: life_cycle_stage.to_string(),
        };
        self.entities.push((id, Box::new(ctx)));
        id
    }

    /// Add a PRODUCT_DEFINITION
    pub fn add_product_definition(&mut self, id_string: &str, description: Option<String>, formation: EntityId, frame_of_reference: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let def = ProductDefinition {
            id,
            id_string: id_string.to_string(),
            description,
            formation,
            frame_of_reference,
        };
        self.entities.push((id, Box::new(def)));
        id
    }

    /// Add a PRODUCT_DEFINITION_SHAPE
    pub fn add_product_definition_shape(&mut self, name: Option<String>, description: Option<String>, definition: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let shape = ProductDefinitionShape {
            id,
            name,
            description,
            definition,
        };
        self.entities.push((id, Box::new(shape)));
        id
    }

    /// Add a SHAPE_REPRESENTATION
    pub fn add_shape_representation(&mut self, name: &str, items: Vec<EntityId>, context_of_items: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let rep = ShapeRepresentation {
            id,
            name: name.to_string(),
            items,
            context_of_items,
        };
        self.entities.push((id, Box::new(rep)));
        id
    }

    /// Add a SHAPE_DEFINITION_REPRESENTATION
    pub fn add_shape_definition_representation(&mut self, definition: EntityId, used_representation: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let sdr = ShapeDefinitionRepresentation {
            id,
            definition,
            used_representation,
        };
        self.entities.push((id, Box::new(sdr)));
        id
    }

    /// Add a GEOMETRIC_REPRESENTATION_CONTEXT
    pub fn add_geometric_representation_context(&mut self, context_identifier: &str, context_type: &str) -> EntityId {
        let id = self.id_gen.next();
        let ctx = GeometricRepresentationContext {
            id,
            context_identifier: context_identifier.to_string(),
            context_type: context_type.to_string(),
        };
        self.entities.push((id, Box::new(ctx)));
        id
    }

    // ===== GD&T Helpers =====

    /// Add a SHAPE_ASPECT
    pub fn add_shape_aspect(&mut self, name: &str, description: Option<String>, of_shape: EntityId, product_definitional: bool) -> EntityId {
        let id = self.id_gen.next();
        let aspect = ShapeAspect {
            id,
            name: name.to_string(),
            description,
            of_shape,
            product_definitional,
        };
        self.entities.push((id, Box::new(aspect)));
        id
    }

    /// Add a DATUM (datum feature for GD&T)
    pub fn add_datum(&mut self, name: &str, description: Option<String>, of_shape: EntityId, identification: &str) -> EntityId {
        let id = self.id_gen.next();
        let datum = Datum {
            id,
            name: name.to_string(),
            description,
            of_shape,
            product_definitional: false,
            identification: identification.to_string(),
        };
        self.entities.push((id, Box::new(datum)));
        id
    }

    /// Add a DATUM_REFERENCE
    pub fn add_datum_reference(&mut self, precedence: u32, referenced_datum: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let datum_ref = DatumReference {
            id,
            precedence,
            referenced_datum,
        };
        self.entities.push((id, Box::new(datum_ref)));
        id
    }

    /// Add a DATUM_SYSTEM
    pub fn add_datum_system(&mut self, name: &str, description: Option<String>, of_shape: EntityId, constituents: Vec<EntityId>) -> EntityId {
        let id = self.id_gen.next();
        let system = DatumSystem {
            id,
            name: name.to_string(),
            description,
            of_shape,
            product_definitional: true,
            constituents,
        };
        self.entities.push((id, Box::new(system)));
        id
    }

    /// Add a LENGTH_MEASURE_WITH_UNIT
    pub fn add_length_measure_with_unit(&mut self, value: f64, unit: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let measure = LengthMeasureWithUnit {
            id,
            value,
            unit,
        };
        self.entities.push((id, Box::new(measure)));
        id
    }

    /// Add a FLATNESS_TOLERANCE
    pub fn add_flatness_tolerance(&mut self, name: &str, magnitude: EntityId, toleranced_shape_aspect: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let tolerance = FlatnessTolerance {
            id,
            base: GeometricToleranceBase {
                name: name.to_string(),
                description: None,
                magnitude,
                toleranced_shape_aspect,
            },
        };
        self.entities.push((id, Box::new(tolerance)));
        id
    }

    /// Add a STRAIGHTNESS_TOLERANCE
    pub fn add_straightness_tolerance(&mut self, name: &str, magnitude: EntityId, toleranced_shape_aspect: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let tolerance = StraightnessTolerance {
            id,
            base: GeometricToleranceBase {
                name: name.to_string(),
                description: None,
                magnitude,
                toleranced_shape_aspect,
            },
        };
        self.entities.push((id, Box::new(tolerance)));
        id
    }

    /// Add a PERPENDICULARITY_TOLERANCE
    pub fn add_perpendicularity_tolerance(&mut self, name: &str, magnitude: EntityId, toleranced_shape_aspect: EntityId, datum_system: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let tolerance = PerpendicularityTolerance {
            id,
            base: GeometricToleranceBase {
                name: name.to_string(),
                description: None,
                magnitude,
                toleranced_shape_aspect,
            },
            datum_system,
        };
        self.entities.push((id, Box::new(tolerance)));
        id
    }

    /// Add a POSITION_TOLERANCE
    pub fn add_position_tolerance(&mut self, name: &str, description: Option<String>, magnitude: EntityId, toleranced_shape_aspect: EntityId, datum_system: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let tolerance = PositionTolerance {
            id,
            base: GeometricToleranceBase {
                name: name.to_string(),
                description,
                magnitude,
                toleranced_shape_aspect,
            },
            datum_system,
        };
        self.entities.push((id, Box::new(tolerance)));
        id
    }

    // ===== PMI Helpers =====

    /// Add a MATERIAL_DESIGNATION
    pub fn add_material_designation(&mut self, name: &str, specification: &str) -> EntityId {
        let id = self.id_gen.next();
        let mat = MaterialDesignation {
            id,
            name: name.to_string(),
            specification: specification.to_string(),
        };
        self.entities.push((id, Box::new(mat)));
        id
    }

    /// Add a PLUS_MINUS_TOLERANCE
    pub fn add_plus_minus_tolerance(&mut self, name: &str, nominal_value: f64, upper_bound: f64, lower_bound: f64, unit: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let tolerance = PlusMinusTolerance {
            id,
            name: name.to_string(),
            nominal_value,
            upper_bound,
            lower_bound,
            unit,
        };
        self.entities.push((id, Box::new(tolerance)));
        id
    }

    /// Add a DIMENSIONAL_SIZE
    pub fn add_dimensional_size(&mut self, name: &str, description: Option<String>, applies_to: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let dim = DimensionalSize {
            id,
            name: name.to_string(),
            description,
            applies_to,
        };
        self.entities.push((id, Box::new(dim)));
        id
    }

    /// Add a DIMENSIONAL_LOCATION
    pub fn add_dimensional_location(&mut self, name: &str, description: Option<String>, relating_shape_aspect: EntityId, related_shape_aspect: EntityId) -> EntityId {
        let id = self.id_gen.next();
        let dim = DimensionalLocation {
            id,
            name: name.to_string(),
            description,
            relating_shape_aspect,
            related_shape_aspect,
        };
        self.entities.push((id, Box::new(dim)));
        id
    }

    /// Add a box (8 points for demonstration)
    pub fn add_box(&mut self, min: [f64; 3], max: [f64; 3]) {
        // 8 corner points
        self.add_point(None, min[0], min[1], min[2]);
        self.add_point(None, max[0], min[1], min[2]);
        self.add_point(None, max[0], max[1], min[2]);
        self.add_point(None, min[0], max[1], min[2]);
        self.add_point(None, min[0], min[1], max[2]);
        self.add_point(None, max[0], min[1], max[2]);
        self.add_point(None, max[0], max[1], max[2]);
        self.add_point(None, min[0], max[1], max[2]);
    }

    /// Write complete STEP file
    pub fn write_to<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // Header
        writeln!(writer, "ISO-10303-21;")?;
        writeln!(writer, "HEADER;")?;
        writeln!(writer, "FILE_DESCRIPTION(('AutoCrate ASTM D6039 Crate'),'2;1');")?;
        writeln!(writer, "FILE_NAME('crate.step','2025-12-02T00:00:00',('AutoCrate'),('Antimony Labs'),'','','');")?;
        writeln!(writer, "FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF'));")?;
        writeln!(writer, "ENDSEC;")?;

        // Data section
        writeln!(writer, "DATA;")?;

        for (id, entity) in &self.entities {
            entity.write_entity(*id, &mut writer)?;
        }

        writeln!(writer, "ENDSEC;")?;
        writeln!(writer, "END-ISO-10303-21;")?;

        Ok(())
    }

    /// Get STEP content as string
    pub fn to_string(&self) -> String {
        let mut buf = Vec::new();
        self.write_to(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
}

impl Default for StepWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_writer_basic() {
        let mut writer = StepWriter::new();
        let p1 = writer.add_point(None, 0.0, 0.0, 0.0);
        let p2 = writer.add_point(None, 1.0, 1.0, 1.0);

        assert_eq!(p1, EntityId(1));
        assert_eq!(p2, EntityId(2));

        let output = writer.to_string();
        assert!(output.contains("ISO-10303-21"));
        assert!(output.contains("CARTESIAN_POINT"));
    }
}
