//! Lumber cut list generation with ASTM D6039 tolerances

use super::part_numbers::generate_part_number;
use super::{determine_lumber_grade, LumberGrade, Tolerance};
use crate::assembly::*;
use crate::constants::LumberSize;
use serde::{Deserialize, Serialize};

/// Cut list entry for a lumber component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutListEntry {
    pub part_number: String,
    pub description: String,
    pub lumber_size: LumberSize,
    pub length: Tolerance,
    pub quantity: u32,
    pub lumber_grade: LumberGrade,
    pub straightness_tolerance: f32,     // inches per foot
    pub perpendicularity_tolerance: f32, // degrees
    pub notes: Vec<String>,
}

/// Generate cut list from assembly
pub fn generate_cut_list(assembly: &CrateAssembly, product_weight: f32) -> Vec<CutListEntry> {
    let lumber_grade = determine_lumber_grade(product_weight);
    let mut cut_list = Vec::new();
    let mut counter_skid = 0;
    let mut counter_floorboard = 0;
    let mut counter_cleat_v = 0;
    let mut counter_cleat_h = 0;

    for node in &assembly.nodes {
        if node.id == assembly.root_id {
            continue;
        }

        let entry = match &node.component_type {
            ComponentType::Skid { dimensions } => {
                counter_skid += 1;
                Some(CutListEntry {
                    part_number: generate_part_number(&node.component_type, 0), // Use 0 for aggregation
                    description: "Skid".to_string(),
                    lumber_size: LumberSize::L4x4,
                    length: Tolerance::lumber(
                        dimensions[2], // length dimension
                        0.125,         // ±1/8" tolerance
                        0.0625,        // 1/16" per foot straightness
                        1.0,           // 1.0° perpendicularity
                    ),
                    quantity: 1,
                    lumber_grade,
                    straightness_tolerance: 0.0625,
                    perpendicularity_tolerance: 1.0,
                    notes: vec!["Run along length".to_string()],
                })
            }

            ComponentType::Floorboard { dimensions } => {
                counter_floorboard += 1;
                Some(CutListEntry {
                    part_number: generate_part_number(&node.component_type, 0), // Use 0 for aggregation
                    description: "Floor Board".to_string(),
                    lumber_size: LumberSize::L2x6,
                    length: Tolerance::lumber(
                        dimensions[2], // length dimension
                        0.0625,        // ±1/16" tolerance (tighter for floorboards)
                        0.0625,        // 1/16" per foot straightness
                        1.0,           // 1.0° perpendicularity
                    ),
                    quantity: 1,
                    lumber_grade,
                    straightness_tolerance: 0.0625,
                    perpendicularity_tolerance: 1.0,
                    notes: vec!["Spans across skids".to_string()],
                })
            }

            ComponentType::Cleat {
                dimensions,
                is_vertical,
            } => {
                let (counter, desc, note) = if *is_vertical {
                    counter_cleat_v += 1;
                    (counter_cleat_v, "Vertical Cleat", "Corner posts")
                } else {
                    counter_cleat_h += 1;
                    (counter_cleat_h, "Horizontal Cleat", "Horizontal support")
                };

                Some(CutListEntry {
                    part_number: generate_part_number(&node.component_type, 0), // Use 0 for aggregation
                    description: desc.to_string(),
                    lumber_size: LumberSize::L2x4,
                    length: Tolerance::lumber(
                        dimensions[2], // length dimension
                        0.0625,        // ±1/16" tolerance
                        0.0625,        // 1/16" per foot straightness
                        0.5,           // 0.5° perpendicularity (tighter for cleats)
                    ),
                    quantity: 1,
                    lumber_grade,
                    straightness_tolerance: 0.0625,
                    perpendicularity_tolerance: 0.5,
                    notes: vec![note.to_string()],
                })
            }

            // Panels and nails don't go in cut list (different manufacturing process)
            _ => None,
        };

        if let Some(entry) = entry {
            cut_list.push(entry);
        }
    }

    // Aggregate identical parts
    aggregate_cut_list(cut_list)
}

/// Aggregate identical cut list entries by part number
fn aggregate_cut_list(entries: Vec<CutListEntry>) -> Vec<CutListEntry> {
    use std::collections::HashMap;

    let mut aggregated: HashMap<String, CutListEntry> = HashMap::new();

    for entry in entries {
        aggregated
            .entry(entry.part_number.clone())
            .and_modify(|e| e.quantity += 1)
            .or_insert(entry);
    }

    let mut result: Vec<CutListEntry> = aggregated.into_values().collect();

    // Sort by part number
    result.sort_by(|a, b| a.part_number.cmp(&b.part_number));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_cut_list_generation() {
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

        let cut_list = generate_cut_list(&assembly, 5000.0);

        // Should have 1 entry (aggregated skids)
        assert_eq!(cut_list.len(), 1);

        let skid_entry = &cut_list[0];
        assert_eq!(skid_entry.quantity, 3);
        assert_eq!(skid_entry.lumber_size, LumberSize::L4x4);
        assert_eq!(skid_entry.length.nominal, 120.0);
        assert_eq!(skid_entry.length.plus, 0.125);
        assert_eq!(skid_entry.straightness_tolerance, 0.0625);
        assert_eq!(skid_entry.perpendicularity_tolerance, 1.0);
    }

    #[test]
    fn test_cleat_tolerances() {
        let mut assembly = CrateAssembly::new();

        let id = assembly.create_node(
            "Vertical Cleat".to_string(),
            ComponentType::Cleat {
                dimensions: [1.5, 3.5, 36.0],
                is_vertical: true,
            },
            LocalTransform::identity(),
            BoundingBox::default(),
        );
        assembly.add_child(assembly.root_id, id);

        let cut_list = generate_cut_list(&assembly, 5000.0);

        let cleat_entry = &cut_list[0];
        assert_eq!(cleat_entry.perpendicularity_tolerance, 0.5); // Tighter for cleats
        assert_eq!(cleat_entry.straightness_tolerance, 0.0625);
    }
}
