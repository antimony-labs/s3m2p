//! STEP Part 21 file writer

use super::entities::*;
use super::primitives::{CartesianPoint, Direction, Axis2Placement3D, Vector, Line, Plane};
use super::topology::{VertexPoint, EdgeCurve, OrientedEdge, FaceBound, AdvancedFace, ClosedShell, ManifoldSolidBrep};
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
