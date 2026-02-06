//! Nailing coordinate generation with datum references

use super::part_numbers::generate_part_number;
use crate::assembly::*;
use crate::geometry::{BoundingBox, Point3};
use serde::{Deserialize, Serialize};

/// Datum reference frame
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatumReference {
    pub primary: String,   // "A" - Base plane (Z=0)
    pub secondary: String, // "B" - Width centerplane (YZ at X=0)
    pub tertiary: String,  // "C" - Length centerplane (XZ at Y=0)
}

impl Default for DatumReference {
    fn default() -> Self {
        Self {
            primary: "A".to_string(),
            secondary: "B".to_string(),
            tertiary: "C".to_string(),
        }
    }
}

/// Nailing coordinate with datum reference
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NailingCoordinate {
    pub nail_id: String,
    pub position: Point3,    // Absolute XYZ coordinates
    pub direction: [f32; 3], // Unit vector for nail direction
    pub datum_references: DatumReference,
    pub position_tolerance: f32, // ±0.25" per ASTM spec
    pub source_part: String,     // Part nail goes through
    pub target_part: String,     // Part nail fastens to
    pub notes: String,
}

/// Generate nailing coordinates from assembly
pub fn generate_nailing_coordinates(assembly: &CrateAssembly) -> Vec<NailingCoordinate> {
    let mut coordinates = Vec::new();

    // Extract component nodes by type
    let floorboards: Vec<_> = assembly
        .nodes
        .iter()
        .filter(|n| matches!(n.component_type, ComponentType::Floorboard { .. }))
        .collect();

    let skids: Vec<_> = assembly
        .nodes
        .iter()
        .filter(|n| matches!(n.component_type, ComponentType::Skid { .. }))
        .collect();

    let panels: Vec<_> = assembly
        .nodes
        .iter()
        .filter(|n| matches!(n.component_type, ComponentType::Panel { .. }))
        .collect();

    let cleats: Vec<_> = assembly
        .nodes
        .iter()
        .filter(|n| matches!(n.component_type, ComponentType::Cleat { .. }))
        .collect();

    // Generate floor-to-skid nails
    let mut nail_counter = 0;
    for (floor_idx, floorboard) in floorboards.iter().enumerate() {
        for (skid_idx, skid) in skids.iter().enumerate() {
            if bounding_boxes_intersect(&floorboard.bounds, &skid.bounds) {
                // Calculate intersection center
                let intersection_center =
                    calculate_intersection_center(&floorboard.bounds, &skid.bounds);

                // Place 2 nails at ±8" offset (16" spacing)
                for offset in [-8.0, 8.0] {
                    nail_counter += 1;

                    // Nail position: offset along floorboard direction (Y-axis)
                    let position = Point3 {
                        x: intersection_center.x,
                        y: intersection_center.y + offset,
                        z: intersection_center.z,
                    };

                    coordinates.push(NailingCoordinate {
                        nail_id: format!("NAIL-{:03}", nail_counter),
                        position,
                        direction: [0.0, 0.0, -1.0], // Downward into skid
                        datum_references: DatumReference::default(),
                        position_tolerance: 0.25,
                        source_part: generate_part_number(&floorboard.component_type, floor_idx),
                        target_part: generate_part_number(&skid.component_type, skid_idx),
                        notes: "Floor to skid".to_string(),
                    });
                }
            }
        }
    }

    // Generate cleat-to-panel nails (for Style B crates with panels)
    for (cleat_idx, cleat) in cleats.iter().enumerate() {
        for (panel_idx, panel) in panels.iter().enumerate() {
            if bounding_boxes_intersect(&cleat.bounds, &panel.bounds) {
                // Calculate vertical spacing (6-8" for cleats)
                let cleat_height = cleat.bounds.max.z - cleat.bounds.min.z;
                let num_nails = ((cleat_height / 8.0).ceil() as usize).max(2);
                let spacing = cleat_height / (num_nails as f32 - 1.0);

                for i in 0..num_nails {
                    nail_counter += 1;

                    let z_offset = cleat.bounds.min.z + (i as f32) * spacing;
                    let intersection_center =
                        calculate_intersection_center(&cleat.bounds, &panel.bounds);

                    coordinates.push(NailingCoordinate {
                        nail_id: format!("NAIL-{:03}", nail_counter),
                        position: Point3 {
                            x: intersection_center.x,
                            y: intersection_center.y,
                            z: z_offset,
                        },
                        direction: [1.0, 0.0, 0.0], // Horizontal into panel
                        datum_references: DatumReference::default(),
                        position_tolerance: 0.25,
                        source_part: generate_part_number(&cleat.component_type, cleat_idx),
                        target_part: generate_part_number(&panel.component_type, panel_idx),
                        notes: "Cleat to panel".to_string(),
                    });
                }
            }
        }
    }

    coordinates
}

/// Check if two bounding boxes intersect
fn bounding_boxes_intersect(a: &BoundingBox, b: &BoundingBox) -> bool {
    // Check for separation in each axis
    if a.max.x < b.min.x || a.min.x > b.max.x {
        return false;
    }
    if a.max.y < b.min.y || a.min.y > b.max.y {
        return false;
    }
    if a.max.z < b.min.z || a.min.z > b.max.z {
        return false;
    }
    true
}

/// Calculate intersection center of two bounding boxes
fn calculate_intersection_center(a: &BoundingBox, b: &BoundingBox) -> Point3 {
    Point3 {
        x: (a.min.x.max(b.min.x) + a.max.x.min(b.max.x)) / 2.0,
        y: (a.min.y.max(b.min.y) + a.max.y.min(b.max.y)) / 2.0,
        z: (a.min.z.max(b.min.z) + a.max.z.min(b.max.z)) / 2.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_bounding_box_intersection() {
        let box_a = BoundingBox {
            min: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Point3 {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
        };

        let box_b = BoundingBox {
            min: Point3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            max: Point3 {
                x: 15.0,
                y: 15.0,
                z: 15.0,
            },
        };

        assert!(bounding_boxes_intersect(&box_a, &box_b));

        let box_c = BoundingBox {
            min: Point3 {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            max: Point3 {
                x: 30.0,
                y: 30.0,
                z: 30.0,
            },
        };

        assert!(!bounding_boxes_intersect(&box_a, &box_c));
    }

    #[test]
    fn test_nailing_coordinate_generation() {
        let mut assembly = CrateAssembly::new();

        // Add a skid
        let skid_id = assembly.create_node(
            "Skid 1".to_string(),
            ComponentType::Skid {
                dimensions: [3.5, 3.5, 120.0],
            },
            LocalTransform::identity(),
            BoundingBox {
                min: Point3 {
                    x: -2.0,
                    y: -60.0,
                    z: 0.0,
                },
                max: Point3 {
                    x: 2.0,
                    y: 60.0,
                    z: 3.5,
                },
            },
        );
        assembly.add_child(assembly.root_id, skid_id);

        // Add a floorboard that crosses the skid
        let floor_id = assembly.create_node(
            "Floorboard 1".to_string(),
            ComponentType::Floorboard {
                dimensions: [1.5, 5.5, 48.0],
            },
            LocalTransform::identity(),
            BoundingBox {
                min: Point3 {
                    x: -24.0,
                    y: -3.0,
                    z: 3.5,
                },
                max: Point3 {
                    x: 24.0,
                    y: 2.5,
                    z: 5.0,
                },
            },
        );
        assembly.add_child(assembly.root_id, floor_id);

        let coords = generate_nailing_coordinates(&assembly);

        // Should generate 2 nails (±8" offset)
        assert_eq!(coords.len(), 2);

        // Check first nail
        assert_eq!(coords[0].datum_references.primary, "A");
        assert_eq!(coords[0].datum_references.secondary, "B");
        assert_eq!(coords[0].datum_references.tertiary, "C");
        assert_eq!(coords[0].position_tolerance, 0.25);
        assert_eq!(coords[0].direction, [0.0, 0.0, -1.0]);

        // Check spacing
        assert!((coords[1].position.y - coords[0].position.y).abs() > 15.0); // ~16" spacing
    }
}
