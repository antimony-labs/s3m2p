//! Part numbering system for AutoCrate components
//!
//! Format: {TYPE}-{SIZE}-{INDEX}
//! Examples:
//! - SKD-4X4-001 (4x4 skid #1)
//! - FLR-2X6-001 (2x6 floorboard #1)
//! - CLT-2X4-V001 (vertical 2x4 cleat #1)
//! - PNL-PLY-FRONT (front plywood panel)
//! - NAIL-16D-001 (16d nail #1)

use crate::assembly::ComponentType;
use crate::geometry::PanelType;

/// Generate part number from component type and index
pub fn generate_part_number(component_type: &ComponentType, index: usize) -> String {
    match component_type {
        ComponentType::Skid { .. } => format!("SKD-4X4-{:03}", index + 1),

        ComponentType::Floorboard { .. } => format!("FLR-2X6-{:03}", index + 1),

        ComponentType::Cleat { is_vertical, .. } => {
            let prefix = if *is_vertical { "V" } else { "H" };
            format!("CLT-2X4-{}{:03}", prefix, index + 1)
        }

        ComponentType::Panel { panel_type, .. } => {
            format!("PNL-PLY-{}", panel_type_name(*panel_type))
        }

        ComponentType::Nail { .. } => format!("NAIL-16D-{:03}", index + 1),

        // Assemblies don't get part numbers
        ComponentType::CrateAssembly => "ASSY-CRATE".to_string(),
        ComponentType::BaseAssembly => "ASSY-BASE".to_string(),
        ComponentType::WallAssembly(panel_type) => {
            format!("ASSY-WALL-{}", panel_type_name(*panel_type))
        }
    }
}

/// Get human-readable panel type name
fn panel_type_name(panel_type: PanelType) -> &'static str {
    match panel_type {
        PanelType::Front => "FRONT",
        PanelType::Back => "BACK",
        PanelType::Left => "LEFT",
        PanelType::Right => "RIGHT",
        PanelType::Top => "TOP",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skid_part_number() {
        let component = ComponentType::Skid {
            dimensions: [3.5, 3.5, 120.0],
        };
        assert_eq!(generate_part_number(&component, 0), "SKD-4X4-001");
        assert_eq!(generate_part_number(&component, 1), "SKD-4X4-002");
        assert_eq!(generate_part_number(&component, 99), "SKD-4X4-100");
    }

    #[test]
    fn test_floorboard_part_number() {
        let component = ComponentType::Floorboard {
            dimensions: [1.5, 5.5, 48.0],
        };
        assert_eq!(generate_part_number(&component, 0), "FLR-2X6-001");
    }

    #[test]
    fn test_cleat_part_number() {
        let vert_cleat = ComponentType::Cleat {
            dimensions: [1.5, 3.5, 36.0],
            is_vertical: true,
        };
        assert_eq!(generate_part_number(&vert_cleat, 0), "CLT-2X4-V001");

        let horz_cleat = ComponentType::Cleat {
            dimensions: [1.5, 3.5, 36.0],
            is_vertical: false,
        };
        assert_eq!(generate_part_number(&horz_cleat, 0), "CLT-2X4-H001");
    }

    #[test]
    fn test_panel_part_number() {
        let front_panel = ComponentType::Panel {
            thickness: 0.75,
            width: 48.0,
            height: 36.0,
            panel_type: PanelType::Front,
        };
        assert_eq!(generate_part_number(&front_panel, 0), "PNL-PLY-FRONT");

        let top_panel = ComponentType::Panel {
            thickness: 0.75,
            width: 48.0,
            height: 48.0,
            panel_type: PanelType::Top,
        };
        assert_eq!(generate_part_number(&top_panel, 0), "PNL-PLY-TOP");
    }

    #[test]
    fn test_nail_part_number() {
        let nail = ComponentType::Nail {
            x: 12.0,
            y: 0.0,
            z: 4.0,
            diameter: 0.162,
            length: 3.5,
            direction: [0.0, 0.0, -1.0],
        };
        assert_eq!(generate_part_number(&nail, 0), "NAIL-16D-001");
        assert_eq!(generate_part_number(&nail, 99), "NAIL-16D-100");
    }
}
