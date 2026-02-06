//! Assembly tree structure for crate components

use crate::{constants::LumberSize, geometry::*};
use glam::Quat;
use serde::{Deserialize, Serialize};

/// Unique identifier for assembly components
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub u32);

/// Transform relative to parent coordinate system
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LocalTransform {
    pub translation: Point3,
    pub rotation: Quat,
}

impl Default for LocalTransform {
    fn default() -> Self {
        Self {
            translation: Point3::default(),
            rotation: Quat::IDENTITY,
        }
    }
}

impl LocalTransform {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn from_translation(t: Point3) -> Self {
        Self {
            translation: t,
            rotation: Quat::IDENTITY,
        }
    }
}

/// Component types in the assembly
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComponentType {
    // Assemblies
    CrateAssembly,
    BaseAssembly,
    WallAssembly(PanelType),

    // Parts
    Skid {
        dimensions: [f32; 3],
    },
    Floorboard {
        dimensions: [f32; 3],
    },
    Cleat {
        dimensions: [f32; 3],
        is_vertical: bool,
    },
    Panel {
        thickness: f32,
        width: f32,
        height: f32,
        panel_type: PanelType,
    },

    // Fasteners
    Nail {
        x: f32,
        y: f32,
        z: f32,
        diameter: f32,
        length: f32,
        direction: [f32; 3],
    },
}

/// Assembly node in the tree
#[derive(Clone, Debug)]
pub struct AssemblyNode {
    pub id: ComponentId,
    pub name: String,
    pub component_type: ComponentType,
    pub transform: LocalTransform,
    pub bounds: BoundingBox,
    pub children: Vec<ComponentId>,
}

/// Complete assembly structure
#[derive(Clone, Debug)]
pub struct CrateAssembly {
    pub nodes: Vec<AssemblyNode>,
    pub root_id: ComponentId,
    next_id: u32,
}

impl CrateAssembly {
    pub fn new() -> Self {
        let root = AssemblyNode {
            id: ComponentId(0),
            name: "CrateAssembly".to_string(),
            component_type: ComponentType::CrateAssembly,
            transform: LocalTransform::identity(),
            bounds: BoundingBox::default(),
            children: Vec::new(),
        };

        Self {
            nodes: vec![root],
            root_id: ComponentId(0),
            next_id: 1,
        }
    }

    pub fn create_node(
        &mut self,
        name: String,
        component_type: ComponentType,
        transform: LocalTransform,
        bounds: BoundingBox,
    ) -> ComponentId {
        let id = ComponentId(self.next_id);
        self.next_id += 1;

        let node = AssemblyNode {
            id,
            name,
            component_type,
            transform,
            bounds,
            children: Vec::new(),
        };

        self.nodes.push(node);
        id
    }

    pub fn add_child(&mut self, parent: ComponentId, child: ComponentId) {
        if let Some(parent_node) = self.nodes.iter_mut().find(|n| n.id == parent) {
            parent_node.children.push(child);
        }
    }

    pub fn get_node(&self, id: ComponentId) -> Option<&AssemblyNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

impl Default for CrateAssembly {
    fn default() -> Self {
        Self::new()
    }
}
