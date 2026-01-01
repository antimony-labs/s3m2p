//! Integration tests for STEP export with PMI/GD&T

use autocrate::*;

#[test]
fn test_step_export_with_pmi_gdt() {
    // Generate a simple Style B crate
    let spec = CrateSpec {
        product: ProductDimensions {
            length: 120.0,
            width: 120.0,
            height: 120.0,
            weight: 10000.0,
        },
        clearances: Clearances {
            side: 3.0,
            end: 3.0,
            top: 3.0,
        },
        style: CrateStyle::B,
        ..Default::default()
    };

    let assembly = generate_crate(&spec, CrateStyle::B);
    let step_writer = autocrate::step_converter::convert_assembly_to_step(&assembly);
    let content = step_writer.to_string();

    // Verify STEP file structure
    assert!(content.contains("ISO-10303-21"), "Missing STEP header");
    assert!(content.contains("AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF"), "Missing AP242 schema");

    // Verify Product Structure
    assert!(content.contains("APPLICATION_CONTEXT"), "Missing APPLICATION_CONTEXT");
    assert!(content.contains("PRODUCT_CONTEXT"), "Missing PRODUCT_CONTEXT");
    assert!(content.contains("PRODUCT"), "Missing PRODUCT");
    assert!(content.contains("PRODUCT_DEFINITION"), "Missing PRODUCT_DEFINITION");
    assert!(content.contains("PRODUCT_DEFINITION_SHAPE"), "Missing PRODUCT_DEFINITION_SHAPE");
    assert!(content.contains("SHAPE_REPRESENTATION"), "Missing SHAPE_REPRESENTATION");

    // Verify Datum Reference Frame (A|B|C)
    assert!(content.contains("DATUM"), "Missing DATUM entity");
    assert!(content.contains("Datum A"), "Missing Datum A");
    assert!(content.contains("Datum B"), "Missing Datum B");
    assert!(content.contains("Datum C"), "Missing Datum C");
    assert!(content.contains("DATUM_SYSTEM"), "Missing DATUM_SYSTEM");
    assert!(content.contains("Primary DRF"), "Missing Primary DRF");
    assert!(content.contains("A|B|C"), "Missing A|B|C reference");

    // Verify Geometric Tolerances
    assert!(content.contains("FLATNESS_TOLERANCE"), "Missing FLATNESS_TOLERANCE");
    assert!(content.contains("SHAPE_ASPECT"), "Missing SHAPE_ASPECT");
    assert!(content.contains("LENGTH_MEASURE_WITH_UNIT"), "Missing LENGTH_MEASURE_WITH_UNIT");

    // Verify PMI
    assert!(content.contains("MATERIAL_DESIGNATION"), "Missing MATERIAL_DESIGNATION");
    assert!(content.contains("ASTM"), "Missing ASTM material spec");

    // Verify B-rep geometry
    assert!(content.contains("MANIFOLD_SOLID_BREP"), "Missing MANIFOLD_SOLID_BREP");
    assert!(content.contains("CARTESIAN_POINT"), "Missing CARTESIAN_POINT");
    assert!(content.contains("ADVANCED_FACE"), "Missing ADVANCED_FACE");
    assert!(content.contains("CLOSED_SHELL"), "Missing CLOSED_SHELL");

    println!("STEP file length: {} bytes", content.len());
    assert!(content.len() > 10000, "STEP file seems too small");
}

#[test]
fn test_step_export_datum_system() {
    // Test that datum system is properly defined
    let spec = CrateSpec::default();
    let assembly = generate_crate(&spec, CrateStyle::B);

    let step_writer = autocrate::step_converter::convert_assembly_to_step(&assembly);
    let content = step_writer.to_string();

    // Verify all three datums are defined with proper precedence
    assert!(content.contains("DATUM_REFERENCE"), "Missing DATUM_REFERENCE");

    // Count datum references (should be at least 3 for A, B, C)
    let datum_ref_count = content.matches("DATUM_REFERENCE").count();
    assert!(datum_ref_count >= 3, "Should have at least 3 datum references (A, B, C)");

    // Verify datum system links them
    assert!(content.contains("DATUM_SYSTEM"), "Missing DATUM_SYSTEM");
    assert!(content.contains("Primary DRF"), "Missing Primary DRF name");
}

#[test]
fn test_step_export_skid_tolerances() {
    let spec = CrateSpec::default();
    let assembly = generate_crate(&spec, CrateStyle::B);

    // Verify assembly has skids
    let skid_count = assembly.nodes.iter()
        .filter(|n| matches!(n.component_type, ComponentType::Skid { .. }))
        .count();

    assert!(skid_count >= 3, "Assembly should have at least 3 skids");

    let step_writer = autocrate::step_converter::convert_assembly_to_step(&assembly);
    let content = step_writer.to_string();

    // Verify flatness tolerances for skid tops
    assert!(content.contains("Flatness 0.125\""), "Missing skid flatness tolerance");
    assert!(content.contains("Top Surface"), "Missing surface designation");
    assert!(content.contains("ASTM D245"), "Missing lumber material spec");
    assert!(content.contains("Southern Pine"), "Missing wood species");
}

#[test]
fn test_step_export_cleat_perpendicularity() {
    let spec = CrateSpec::default();
    let assembly = generate_crate(&spec, CrateStyle::B);

    // Verify assembly has cleats
    let cleat_count = assembly.nodes.iter()
        .filter(|n| matches!(n.component_type, ComponentType::Cleat { .. }))
        .count();

    assert!(cleat_count > 0, "Assembly should have cleats");

    let step_writer = autocrate::step_converter::convert_assembly_to_step(&assembly);
    let content = step_writer.to_string();

    // Verify perpendicularity tolerances for vertical cleats
    assert!(content.contains("PERPENDICULARITY_TOLERANCE"), "Missing PERPENDICULARITY_TOLERANCE");
    assert!(content.contains("Perp"), "Missing perpendicularity callout");
}

#[test]
fn test_step_export_panel_materials() {
    let spec = CrateSpec::default();
    let assembly = generate_crate(&spec, CrateStyle::B);

    // Verify assembly has panels
    let panel_count = assembly.nodes.iter()
        .filter(|n| matches!(n.component_type, ComponentType::Panel { .. }))
        .count();

    assert!(panel_count > 0, "Style B crate should have panels");

    let step_writer = autocrate::step_converter::convert_assembly_to_step(&assembly);
    let content = step_writer.to_string();

    // Verify panel materials
    assert!(content.contains("Plywood"), "Missing plywood specification");
    assert!(content.contains("ASTM D3043"), "Missing plywood standard");
}
