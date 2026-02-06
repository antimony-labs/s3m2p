//! CSV export for cut lists, BOM, and nailing coordinates

use crate::manufacturing::*;

/// Export cut list to CSV format
pub fn export_cut_list_csv(cut_list: &[CutListEntry]) -> String {
    let mut csv = String::from("Part Number,Description,Lumber Size,Length (in),Tolerance (+/-),Quantity,Grade,Straightness (in/ft),Perpendicularity (deg),Notes\n");

    for entry in cut_list {
        csv.push_str(&format!(
            "{},{},{},{:.3},{:.3},{},{},{:.4},{:.1},{}\n",
            entry.part_number,
            entry.description,
            entry.lumber_size.name(),
            entry.length.nominal,
            entry.length.plus,
            entry.quantity,
            entry.lumber_grade.as_str(),
            entry.straightness_tolerance,
            entry.perpendicularity_tolerance,
            escape_csv(&entry.notes.join("; ")),
        ));
    }

    csv
}

/// Export BOM to CSV format
pub fn export_bom_csv(bom: &BillOfMaterials) -> String {
    let mut csv = String::from(
        "Item,Part Number,Description,Quantity,Unit,Material Specification,Dimensions,Notes\n",
    );

    for entry in &bom.entries {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            entry.item_number,
            entry.part_number,
            entry.description,
            entry.quantity,
            entry.unit,
            escape_csv(&entry.material.format_spec()),
            escape_csv(&entry.dimensions),
            escape_csv(&entry.notes.join("; ")),
        ));
    }

    csv
}

/// Export nailing coordinates to CSV format
pub fn export_nailing_csv(coords: &[NailingCoordinate]) -> String {
    let mut csv = String::from("Nail ID,X (in),Y (in),Z (in),X Tol (+/-),Y Tol (+/-),Z Tol (+/-),Direction X,Direction Y,Direction Z,Datum Ref,Source Part,Target Part,Notes\n");

    for coord in coords {
        csv.push_str(&format!(
            "{},{:.3},{:.3},{:.3},{:.2},{:.2},{:.2},{:.3},{:.3},{:.3},{}|{}|{},{},{},{}\n",
            coord.nail_id,
            coord.position.x,
            coord.position.y,
            coord.position.z,
            coord.position_tolerance,
            coord.position_tolerance,
            coord.position_tolerance,
            coord.direction[0],
            coord.direction[1],
            coord.direction[2],
            coord.datum_references.primary,
            coord.datum_references.secondary,
            coord.datum_references.tertiary,
            coord.source_part,
            coord.target_part,
            escape_csv(&coord.notes),
        ));
    }

    csv
}

/// Escape CSV fields that contain commas, quotes, or newlines
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembly::*;
    use crate::constants::LumberSize;
    use crate::geometry::*;

    #[test]
    fn test_csv_escaping() {
        assert_eq!(escape_csv("simple"), "simple");
        assert_eq!(escape_csv("has,comma"), "\"has,comma\"");
        assert_eq!(escape_csv("has\"quote"), "\"has\"\"quote\"");
    }

    #[test]
    fn test_cut_list_csv_format() {
        let cut_list = vec![CutListEntry {
            part_number: "SKD-4X4-001".to_string(),
            description: "Skid".to_string(),
            lumber_size: LumberSize::L4x4,
            length: Tolerance::lumber(120.0, 0.125, 0.0625, 1.0),
            quantity: 3,
            lumber_grade: LumberGrade::No2,
            straightness_tolerance: 0.0625,
            perpendicularity_tolerance: 1.0,
            notes: vec!["Test note".to_string()],
        }];

        let csv = export_cut_list_csv(&cut_list);

        // Check header
        assert!(csv.starts_with("Part Number,Description"));

        // Check data row
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2); // Header + 1 data row
        assert!(lines[1].contains("SKD-4X4-001"));
        assert!(lines[1].contains("120.000"));
        assert!(lines[1].contains("0.125"));
    }

    #[test]
    fn test_bom_csv_format() {
        let bom = BillOfMaterials {
            entries: vec![BomEntry {
                item_number: 1,
                part_number: "SKD-4X4-001".to_string(),
                description: "Skid Member".to_string(),
                quantity: 3,
                unit: "EA".to_string(),
                material: MaterialSpec::Lumber {
                    size: LumberSize::L4x4,
                    grade: LumberGrade::No2,
                    species: WoodSpecies::SouthernPine,
                },
                dimensions: "3.5\"x3.5\"x120\"".to_string(),
                notes: vec![],
            }],
        };

        let csv = export_bom_csv(&bom);

        assert!(csv.starts_with("Item,Part Number"));
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("SKD-4X4-001"));
        assert!(lines[1].contains("3"));
        assert!(lines[1].contains("EA"));
    }

    #[test]
    fn test_nailing_csv_format() {
        let coords = vec![NailingCoordinate {
            nail_id: "NAIL-001".to_string(),
            position: Point3 {
                x: 12.0,
                y: 0.0,
                z: 4.25,
            },
            direction: [0.0, 0.0, -1.0],
            datum_references: DatumReference::default(),
            position_tolerance: 0.25,
            source_part: "FLR-2X6-001".to_string(),
            target_part: "SKD-4X4-001".to_string(),
            notes: "Floor to skid".to_string(),
        }];

        let csv = export_nailing_csv(&coords);

        assert!(csv.starts_with("Nail ID,X (in)"));
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("NAIL-001"));
        assert!(lines[1].contains("12.000"));
        assert!(lines[1].contains("A|B|C"));
    }
}
