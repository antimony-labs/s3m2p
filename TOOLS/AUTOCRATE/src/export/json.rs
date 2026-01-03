//! JSON export for assembly data

use crate::assembly::*;
use crate::manufacturing::*;
use crate::CrateSpec;
use serde::{Deserialize, Serialize};
use serde_json;

/// Assembly export structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssemblyExport {
    pub crate_specification: CrateSpecExport,
    pub overall_dimensions: OverallDimensions,
    pub components: Vec<ComponentExport>,
    pub bom: BillOfMaterials,
    pub manufacturing_data: ManufacturingDataExport,
}

/// Crate specification export
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrateSpecExport {
    pub product_length: f32,
    pub product_width: f32,
    pub product_height: f32,
    pub product_weight: f32,
    pub clearance_side: f32,
    pub clearance_end: f32,
    pub clearance_top: f32,
    pub standards: String,
}

/// Overall crate dimensions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverallDimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

/// Component export
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentExport {
    pub part_number: String,
    pub name: String,
    pub component_type: String,
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion
    pub bounds_min: [f32; 3],
    pub bounds_max: [f32; 3],
}

/// Manufacturing data export
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManufacturingDataExport {
    pub nailing_coordinates: Vec<NailingCoordinate>,
    pub datum_planes: DatumPlanesInfo,
}

/// Datum plane information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatumPlanesInfo {
    pub a: String,
    pub b: String,
    pub c: String,
}

impl Default for DatumPlanesInfo {
    fn default() -> Self {
        Self {
            a: "Base plane (Z=0, bottom of skids)".to_string(),
            b: "Width centerplane (YZ plane at X=0)".to_string(),
            c: "Length centerplane (XZ plane at Y=0)".to_string(),
        }
    }
}

/// Export assembly to JSON
pub fn export_assembly_json(
    assembly: &CrateAssembly,
    spec: &CrateSpec,
    manufacturing: &ManufacturingData,
) -> String {
    // Calculate overall dimensions from assembly bounds
    let overall_dims = calculate_overall_dimensions(assembly);

    // Convert assembly nodes to component exports
    let mut component_counter: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let components: Vec<ComponentExport> = assembly.nodes.iter()
        .filter(|n| n.id != assembly.root_id)
        .map(|node| {
            let type_name = get_component_type_name(&node.component_type);
            let counter = component_counter.entry(type_name.clone()).or_insert(0);
            let part_number = crate::manufacturing::part_numbers::generate_part_number(&node.component_type, *counter);
            *counter += 1;

            ComponentExport {
                part_number,
                name: node.name.clone(),
                component_type: type_name,
                position: [
                    node.transform.translation.x,
                    node.transform.translation.y,
                    node.transform.translation.z,
                ],
                rotation: [
                    node.transform.rotation.x,
                    node.transform.rotation.y,
                    node.transform.rotation.z,
                    node.transform.rotation.w,
                ],
                bounds_min: [node.bounds.min.x, node.bounds.min.y, node.bounds.min.z],
                bounds_max: [node.bounds.max.x, node.bounds.max.y, node.bounds.max.z],
            }
        })
        .collect();

    let export_data = AssemblyExport {
        crate_specification: CrateSpecExport {
            product_length: spec.product.length,
            product_width: spec.product.width,
            product_height: spec.product.height,
            product_weight: spec.product.weight,
            clearance_side: spec.clearances.side,
            clearance_end: spec.clearances.end,
            clearance_top: spec.clearances.top,
            standards: "ASTM D6039".to_string(),
        },
        overall_dimensions: overall_dims,
        components,
        bom: manufacturing.bom.clone(),
        manufacturing_data: ManufacturingDataExport {
            nailing_coordinates: manufacturing.nailing_coords.clone(),
            datum_planes: DatumPlanesInfo::default(),
        },
    };

    serde_json::to_string_pretty(&export_data)
        .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
}

/// Calculate overall dimensions from assembly
fn calculate_overall_dimensions(assembly: &CrateAssembly) -> OverallDimensions {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for node in &assembly.nodes {
        if node.id == assembly.root_id {
            continue;
        }
        min_x = min_x.min(node.bounds.min.x);
        max_x = max_x.max(node.bounds.max.x);
        min_y = min_y.min(node.bounds.min.y);
        max_y = max_y.max(node.bounds.max.y);
        min_z = min_z.min(node.bounds.min.z);
        max_z = max_z.max(node.bounds.max.z);
    }

    OverallDimensions {
        length: max_y - min_y,
        width: max_x - min_x,
        height: max_z - min_z,
    }
}

/// Get human-readable component type name
fn get_component_type_name(component_type: &ComponentType) -> String {
    match component_type {
        ComponentType::CrateAssembly => "CrateAssembly".to_string(),
        ComponentType::BaseAssembly => "BaseAssembly".to_string(),
        ComponentType::WallAssembly(_) => "WallAssembly".to_string(),
        ComponentType::Skid { .. } => "Skid".to_string(),
        ComponentType::Floorboard { .. } => "Floorboard".to_string(),
        ComponentType::Cleat { is_vertical, .. } => {
            if *is_vertical {
                "VerticalCleat".to_string()
            } else {
                "HorizontalCleat".to_string()
            }
        },
        ComponentType::Panel { .. } => "Panel".to_string(),
        ComponentType::Nail { .. } => "Nail".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_json_export_valid() {
        let mut assembly = CrateAssembly::new();

        let skid_id = assembly.create_node(
            "Skid 1".to_string(),
            ComponentType::Skid { dimensions: [3.5, 3.5, 120.0] },
            LocalTransform::identity(),
            BoundingBox {
                min: Point3 { x: -2.0, y: -60.0, z: 0.0 },
                max: Point3 { x: 2.0, y: 60.0, z: 3.5 },
            },
        );
        assembly.add_child(assembly.root_id, skid_id);

        let spec = CrateSpec::default();

        let manufacturing = generate_manufacturing_data(&assembly, spec.product.weight);
        let json = export_assembly_json(&assembly, &spec, &manufacturing);

        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json)
            .expect("JSON should be valid");

        assert!(parsed["crate_specification"].is_object());
        assert!(parsed["components"].is_array());
        assert!(parsed["bom"].is_object());
        assert!(parsed["manufacturing_data"].is_object());

        // Check datum planes
        assert_eq!(parsed["manufacturing_data"]["datum_planes"]["a"].as_str().unwrap(),
                   "Base plane (Z=0, bottom of skids)");
    }
}
