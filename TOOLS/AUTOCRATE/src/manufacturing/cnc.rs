//! CNC program generation for panel cutting

use crate::assembly::*;
use crate::geometry::Point3;
use serde::{Deserialize, Serialize};

/// CNC operation types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CncOperationType {
    RectangularPocket,
    Drilling,
    OutlineProfile,
}

/// CNC tool definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CncTool {
    pub tool_number: u32,
    pub tool_type: String,
    pub diameter: f32, // inches
}

/// CNC operation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CncOperation {
    pub operation_number: u32,
    pub operation_type: CncOperationType,
    pub tool: String,           // Tool reference (e.g., "T1")
    pub feed_rate: f32,         // inches per minute
    pub spindle_speed: u32,     // RPM
    pub cut_depth: f32,         // inches
    pub path: Vec<Point3>,      // Tool path points
    pub description: String,
}

/// Complete CNC cut program
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CncCutProgram {
    pub operations: Vec<CncOperation>,
    pub tool_list: Vec<CncTool>,
    pub material_setup: String,
}

/// Generate CNC program for panel cutting
pub fn generate_cnc_program(assembly: &CrateAssembly) -> CncCutProgram {
    let mut operations = Vec::new();

    // Extract panel components
    let panels: Vec<_> = assembly.nodes.iter()
        .filter(|n| matches!(n.component_type, ComponentType::Panel { .. }))
        .collect();

    // Generate cutting operations for each panel
    for (idx, panel_node) in panels.iter().enumerate() {
        if let ComponentType::Panel { thickness, width, height, panel_type } = &panel_node.component_type {
            // Create rectangular profile cut
            let path = vec![
                Point3 { x: 0.0, y: 0.0, z: 0.0 },
                Point3 { x: *width, y: 0.0, z: 0.0 },
                Point3 { x: *width, y: *height, z: 0.0 },
                Point3 { x: 0.0, y: *height, z: 0.0 },
                Point3 { x: 0.0, y: 0.0, z: 0.0 }, // Close the loop
            ];

            operations.push(CncOperation {
                operation_number: (idx + 1) as u32,
                operation_type: CncOperationType::OutlineProfile,
                tool: "T1".to_string(),
                feed_rate: 60.0,        // 60 in/min for plywood
                spindle_speed: 12000,   // 12000 RPM
                cut_depth: *thickness,
                path,
                description: format!("{:?} Panel - {:.1}\" x {:.1}\"", panel_type, width, height),
            });
        }
    }

    CncCutProgram {
        operations,
        tool_list: vec![
            CncTool {
                tool_number: 1,
                tool_type: "1/4\" End Mill".to_string(),
                diameter: 0.25,
            }
        ],
        material_setup: "3/4\" Plywood, secured to spoilboard with vacuum or clamps".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_cnc_program_generation() {
        let mut assembly = CrateAssembly::new();

        // Add a panel
        let panel_id = assembly.create_node(
            "Front Panel".to_string(),
            ComponentType::Panel {
                thickness: 0.75,
                width: 48.0,
                height: 36.0,
                panel_type: PanelType::Front,
            },
            LocalTransform::identity(),
            BoundingBox::default(),
        );
        assembly.add_child(assembly.root_id, panel_id);

        let program = generate_cnc_program(&assembly);

        // Should have 1 operation
        assert_eq!(program.operations.len(), 1);

        let op = &program.operations[0];
        assert_eq!(op.operation_type, CncOperationType::OutlineProfile);
        assert_eq!(op.cut_depth, 0.75);
        assert_eq!(op.feed_rate, 60.0);
        assert_eq!(op.spindle_speed, 12000);

        // Path should be a closed rectangle (5 points)
        assert_eq!(op.path.len(), 5);
        // First and last point should match (closed loop)
        assert_eq!(op.path[0].x, op.path[4].x);
        assert_eq!(op.path[0].y, op.path[4].y);
        assert_eq!(op.path[0].z, op.path[4].z);

        // Tool list should have 1 tool
        assert_eq!(program.tool_list.len(), 1);
        assert_eq!(program.tool_list[0].tool_number, 1);
    }

    #[test]
    fn test_rectangular_path() {
        let mut assembly = CrateAssembly::new();

        let panel_id = assembly.create_node(
            "Panel".to_string(),
            ComponentType::Panel {
                thickness: 0.75,
                width: 48.0,
                height: 36.0,
                panel_type: PanelType::Back,
            },
            LocalTransform::identity(),
            BoundingBox::default(),
        );
        assembly.add_child(assembly.root_id, panel_id);

        let program = generate_cnc_program(&assembly);
        let path = &program.operations[0].path;

        // Check corners
        assert_eq!(path[0].x, 0.0);
        assert_eq!(path[0].y, 0.0);
        assert_eq!(path[1].x, 48.0);
        assert_eq!(path[1].y, 0.0);
        assert_eq!(path[2].x, 48.0);
        assert_eq!(path[2].y, 36.0);
        assert_eq!(path[3].x, 0.0);
        assert_eq!(path[3].y, 36.0);
    }
}
