//! Bill of Materials (BOM) generation

use super::part_numbers::generate_part_number;
use super::{
    determine_lumber_grade, FastenerType, LumberGrade, MaterialSpec, PlywoodGrade, WoodSpecies,
};
use crate::assembly::*;
use crate::constants::LumberSize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bill of Materials entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BomEntry {
    pub item_number: u32,
    pub part_number: String,
    pub description: String,
    pub quantity: u32,
    pub unit: String, // "EA" (each), "LF" (linear feet), "SF" (square feet)
    pub material: MaterialSpec,
    pub dimensions: String, // Human-readable dimensions
    pub notes: Vec<String>,
}

/// Complete Bill of Materials
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillOfMaterials {
    pub entries: Vec<BomEntry>,
}

/// Generate Bill of Materials from assembly
pub fn generate_bom(assembly: &CrateAssembly, product_weight: f32) -> BillOfMaterials {
    let lumber_grade = determine_lumber_grade(product_weight);
    let mut parts_map: HashMap<String, (ComponentType, usize, Vec<String>)> = HashMap::new();

    // Count identical parts
    for node in &assembly.nodes {
        if node.id == assembly.root_id {
            continue; // Skip root assembly node
        }

        // Generate part number (using index 0 as template)
        let part_number = generate_part_number(&node.component_type, 0);

        // Group by part number
        parts_map
            .entry(part_number)
            .and_modify(|(_, count, _)| *count += 1)
            .or_insert((node.component_type.clone(), 1, Vec::new()));
    }

    // Convert to BOM entries
    let mut entries: Vec<BomEntry> = parts_map
        .into_iter()
        .enumerate()
        .map(|(idx, (part_number, (component_type, quantity, notes)))| {
            let (description, unit, material, dimensions, mut notes) = match &component_type {
                ComponentType::Skid { dimensions } => (
                    "Skid Member".to_string(),
                    "EA".to_string(),
                    MaterialSpec::Lumber {
                        size: LumberSize::L4x4,
                        grade: lumber_grade,
                        species: WoodSpecies::SouthernPine,
                    },
                    format!("3.5\"x3.5\"x{:.1}\"", dimensions[2]),
                    vec!["Pressure treated recommended".to_string()],
                ),

                ComponentType::Floorboard { dimensions } => (
                    "Floor Board".to_string(),
                    "EA".to_string(),
                    MaterialSpec::Lumber {
                        size: LumberSize::L2x6,
                        grade: lumber_grade,
                        species: WoodSpecies::SouthernPine,
                    },
                    format!("1.5\"x5.5\"x{:.1}\"", dimensions[2]),
                    vec![],
                ),

                ComponentType::Cleat {
                    dimensions,
                    is_vertical,
                } => {
                    let desc = if *is_vertical {
                        "Vertical Cleat"
                    } else {
                        "Horizontal Cleat"
                    };
                    (
                        desc.to_string(),
                        "EA".to_string(),
                        MaterialSpec::Lumber {
                            size: LumberSize::L2x4,
                            grade: lumber_grade,
                            species: WoodSpecies::SouthernPine,
                        },
                        format!("1.5\"x3.5\"x{:.1}\"", dimensions[2]),
                        vec![],
                    )
                }

                ComponentType::Panel {
                    thickness,
                    width,
                    height,
                    ..
                } => (
                    "Plywood Panel".to_string(),
                    "EA".to_string(),
                    MaterialSpec::Plywood {
                        thickness: *thickness,
                        grade: PlywoodGrade::CDXSheathing,
                    },
                    format!("{:.2}\"x{:.1}\"x{:.1}\"", thickness, width, height),
                    vec!["Exterior grade".to_string()],
                ),

                ComponentType::Nail {
                    diameter, length, ..
                } => (
                    "16d Common Nail".to_string(),
                    "EA".to_string(),
                    MaterialSpec::Fastener {
                        fastener_type: FastenerType::CommonNail,
                        size: "16d".to_string(),
                    },
                    format!("{:.3}\" dia x {:.2}\" length", diameter, length),
                    vec![
                        "Galvanized steel".to_string(),
                        "Ring shank preferred".to_string(),
                    ],
                ),

                // Skip assembly nodes
                _ => return None,
            };

            Some(BomEntry {
                item_number: (idx + 1) as u32,
                part_number,
                description,
                quantity: quantity as u32,
                unit,
                material,
                dimensions,
                notes,
            })
        })
        .filter_map(|x| x)
        .collect();

    // Sort by item number
    entries.sort_by_key(|e| e.item_number);

    BillOfMaterials { entries }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_bom_generation() {
        let mut assembly = CrateAssembly::new();

        // Add 3 identical skids
        for i in 0..3 {
            let id = assembly.create_node(
                format!("Skid {}", i + 1),
                ComponentType::Skid {
                    dimensions: [3.5, 3.5, 120.0],
                },
                LocalTransform::identity(),
                BoundingBox::default(),
            );
            assembly.add_child(assembly.root_id, id);
        }

        // Add 2 identical floorboards
        for i in 0..2 {
            let id = assembly.create_node(
                format!("Floorboard {}", i + 1),
                ComponentType::Floorboard {
                    dimensions: [1.5, 5.5, 48.0],
                },
                LocalTransform::identity(),
                BoundingBox::default(),
            );
            assembly.add_child(assembly.root_id, id);
        }

        let bom = generate_bom(&assembly, 5000.0);

        // Should have 2 entries (one for skids, one for floorboards)
        assert_eq!(bom.entries.len(), 2);

        // Find skid entry
        let skid_entry = bom
            .entries
            .iter()
            .find(|e| e.part_number.starts_with("SKD"))
            .expect("Skid entry not found");

        assert_eq!(skid_entry.quantity, 3);
        assert_eq!(skid_entry.unit, "EA");

        // Find floorboard entry
        let floorboard_entry = bom
            .entries
            .iter()
            .find(|e| e.part_number.starts_with("FLR"))
            .expect("Floorboard entry not found");

        assert_eq!(floorboard_entry.quantity, 2);
    }

    #[test]
    fn test_lumber_grade_selection() {
        let grade_light = determine_lumber_grade(5000.0);
        assert_eq!(grade_light, LumberGrade::No2);

        let grade_heavy = determine_lumber_grade(15000.0);
        assert_eq!(grade_heavy, LumberGrade::No1);
    }
}
