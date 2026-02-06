//! STEP B-rep topology

use super::entities::*;
use std::io::{self, Write};

/// VERTEX_POINT - a topological vertex defined by a CartesianPoint
pub struct VertexPoint {
    pub id: EntityId,
    pub name: Option<String>,
    pub vertex_geometry: EntityId, // CARTESIAN_POINT
}

impl StepEntity for VertexPoint {
    fn entity_name(&self) -> &'static str {
        "VERTEX_POINT"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.vertex_geometry)
    }

    fn references(&self) -> Vec<EntityId> {
        vec![self.vertex_geometry]
    }
}

/// EDGE_CURVE - a topological edge defined by two vertices and a curve
pub struct EdgeCurve {
    pub id: EntityId,
    pub name: Option<String>,
    pub edge_start: EntityId,    // VERTEX_POINT
    pub edge_end: EntityId,      // VERTEX_POINT
    pub edge_geometry: EntityId, // CURVE (e.g., LINE)
    pub same_sense: bool,        // Orientation relative to curve
}

impl StepEntity for EdgeCurve {
    fn entity_name(&self) -> &'static str {
        "EDGE_CURVE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(
            w,
            ",{},{},{},.{}",
            self.edge_start,
            self.edge_end,
            self.edge_geometry,
            if self.same_sense { "T" } else { "F" }
        )
    }

    fn references(&self) -> Vec<EntityId> {
        vec![self.edge_start, self.edge_end, self.edge_geometry]
    }
}

/// ORIENTED_EDGE - an edge with a specific orientation
pub struct OrientedEdge {
    pub id: EntityId,
    pub name: Option<String>,
    pub edge_element: EntityId, // EDGE_CURVE
    pub orientation: bool,      // TRUE for same sense, FALSE for opposite sense
}

impl StepEntity for OrientedEdge {
    fn entity_name(&self) -> &'static str {
        "ORIENTED_EDGE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(
            w,
            ",{},.{}",
            self.edge_element,
            if self.orientation { "T" } else { "F" }
        )
    }

    fn references(&self) -> Vec<EntityId> {
        vec![self.edge_element]
    }
}

/// FACE_BOUND - boundary of a face (outer or inner loop)
pub struct FaceBound {
    pub id: EntityId,
    pub name: Option<String>,
    pub bound: Vec<EntityId>, // List of ORIENTED_EDGEs forming the loop
    pub orientation: bool,    // TRUE for outer boundary, FALSE for inner
}

impl StepEntity for FaceBound {
    fn entity_name(&self) -> &'static str {
        "FACE_BOUND"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",(")?;
        for (i, edge) in self.bound.iter().enumerate() {
            write!(w, "{}", edge)?;
            if i < self.bound.len() - 1 {
                write!(w, ",")?;
            }
        }
        write!(w, "),.{}", if self.orientation { "T" } else { "F" })
    }

    fn references(&self) -> Vec<EntityId> {
        self.bound.clone() // All referenced ORIENTED_EDGEs
    }
}

/// ADVANCED_FACE - a face defined by a surface and its boundaries
pub struct AdvancedFace {
    pub id: EntityId,
    pub name: Option<String>,
    pub face_geometry: EntityId,    // SURFACE (e.g., PLANE)
    pub face_bounds: Vec<EntityId>, // List of FACE_BOUNDs
}

impl StepEntity for AdvancedFace {
    fn entity_name(&self) -> &'static str {
        "ADVANCED_FACE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{},(", self.face_geometry)?;
        for (i, bound) in self.face_bounds.iter().enumerate() {
            write!(w, "{}", bound)?;
            if i < self.face_bounds.len() - 1 {
                write!(w, ",")?;
            }
        }
        write!(w, ")")
    }

    fn references(&self) -> Vec<EntityId> {
        let mut refs = vec![self.face_geometry];
        refs.extend(self.face_bounds.iter().cloned());
        refs
    }
}

/// CLOSED_SHELL - a collection of faces forming a closed boundary
pub struct ClosedShell {
    pub id: EntityId,
    pub name: Option<String>,
    pub cfs_faces: Vec<EntityId>, // List of ADVANCED_FACEs
}

impl StepEntity for ClosedShell {
    fn entity_name(&self) -> &'static str {
        "CLOSED_SHELL"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",(")?;
        for (i, face) in self.cfs_faces.iter().enumerate() {
            write!(w, "{}", face)?;
            if i < self.cfs_faces.len() - 1 {
                write!(w, ",")?;
            }
        }
        write!(w, ")")
    }

    fn references(&self) -> Vec<EntityId> {
        self.cfs_faces.clone() // All referenced ADVANCED_FACEs
    }
}

/// MANIFOLD_SOLID_BREP - a solid defined by an outer closed shell
pub struct ManifoldSolidBrep {
    pub id: EntityId,
    pub name: Option<String>,
    pub outer: EntityId, // CLOSED_SHELL
}

impl StepEntity for ManifoldSolidBrep {
    fn entity_name(&self) -> &'static str {
        "MANIFOLD_SOLID_BREP"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{}", self.outer)
    }

    fn references(&self) -> Vec<EntityId> {
        vec![self.outer]
    }
}
