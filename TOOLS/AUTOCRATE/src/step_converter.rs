//! Converts CrateAssembly to STEP entities

use crate::assembly::{CrateAssembly, ComponentType};
use dna::export::step::{
    StepWriter,
    entities::EntityId,
    primitives::{CartesianPoint, Direction, Axis2Placement3D, Vector, Line, Plane},
    topology::{VertexPoint, EdgeCurve, OrientedEdge, FaceBound, AdvancedFace, ClosedShell, ManifoldSolidBrep},
};
use glam::f32::{Vec3, Mat4};

/// Creates a solid box (manifold_solid_brep) from min and max corner points and a transform.
/// Returns the EntityId of the MANIFOLD_SOLID_BREP.
pub fn create_box_brep(writer: &mut StepWriter, transform: Mat4, min: Vec3, max: Vec3) -> EntityId {
    // Calculate 8 corners in local space
    let local_corners = [
        Vec3::new(min.x, min.y, min.z), // 0
        Vec3::new(max.x, min.y, min.z), // 1
        Vec3::new(max.x, max.y, min.z), // 2
        Vec3::new(min.x, max.y, min.z), // 3
        Vec3::new(min.x, min.y, max.z), // 4
        Vec3::new(max.x, min.y, max.z), // 5
        Vec3::new(max.x, max.y, max.z), // 6
        Vec3::new(min.x, max.y, max.z), // 7
    ];

    // Transform corners to global space
    let p_vecs: Vec<Vec3> = local_corners.iter().map(|p| transform.transform_point3(*p)).collect();

    // 1. Create 8 Cartesian Points
    let p_ids: Vec<EntityId> = p_vecs.iter()
        .map(|p| writer.add_point(None, p.x as f64, p.y as f64, p.z as f64))
        .collect();

    // 2. Create directions for axes (X, Y, Z) transformed
    let axis_x = transform.transform_vector3(Vec3::X).normalize();
    let axis_y = transform.transform_vector3(Vec3::Y).normalize();
    let axis_z = transform.transform_vector3(Vec3::Z).normalize();

    let dir_x = writer.add_direction(None, axis_x.x as f64, axis_x.y as f64, axis_x.z as f64);
    let dir_y = writer.add_direction(None, axis_y.x as f64, axis_y.y as f64, axis_y.z as f64);
    let dir_z = writer.add_direction(None, axis_z.x as f64, axis_z.y as f64, axis_z.z as f64);
    
    let dir_neg_x = writer.add_direction(None, -axis_x.x as f64, -axis_x.y as f64, -axis_x.z as f64);
    let dir_neg_y = writer.add_direction(None, -axis_y.x as f64, -axis_y.y as f64, -axis_y.z as f64);
    // dir_neg_z not strictly needed if we rely on axis logic, but let's be consistent if needed

    // 3. Create vectors for edge lengths
    let len_x = (max.x - min.x) as f64;
    let len_y = (max.y - min.y) as f64;
    let len_z = (max.z - min.z) as f64;

    let vec_x = writer.add_vector(None, dir_x, len_x);
    let vec_y = writer.add_vector(None, dir_y, len_y);
    let vec_z = writer.add_vector(None, dir_z, len_z);

    // 4. Create 12 lines for the edges of the box
    // Bottom face
    let l01 = writer.add_line(None, p_ids[0], vec_x); // p0 -> p1
    let l12 = writer.add_line(None, p_ids[1], vec_y); // p1 -> p2
    let l23 = writer.add_line(None, p_ids[2], vec_x); // p2 -> p3 (reverse x)
    let l30 = writer.add_line(None, p_ids[3], vec_y); 

    // Top face
    let l45 = writer.add_line(None, p_ids[4], vec_x); 
    let l56 = writer.add_line(None, p_ids[5], vec_y); 
    let l67 = writer.add_line(None, p_ids[6], vec_x); 
    let l74 = writer.add_line(None, p_ids[7], vec_y); 

    // Vertical edges
    let l04 = writer.add_line(None, p_ids[0], vec_z); 
    let l15 = writer.add_line(None, p_ids[1], vec_z); 
    let l26 = writer.add_line(None, p_ids[2], vec_z); 
    let l37 = writer.add_line(None, p_ids[3], vec_z); 

    // 5. Create vertex points
    let vps: Vec<EntityId> = p_ids.iter()
        .map(|&pid| writer.add_vertex_point(None, pid))
        .collect();

    // 6. Create edge curves
    // Bottom
    let ec01 = writer.add_edge_curve(None, vps[0], vps[1], l01, true);
    let ec12 = writer.add_edge_curve(None, vps[1], vps[2], l12, true);
    let ec23 = writer.add_edge_curve(None, vps[2], vps[3], l23, false); // p2->p3 is opposite to X
    let ec30 = writer.add_edge_curve(None, vps[3], vps[0], l30, false); // p3->p0 is opposite to Y

    // Top
    let ec45 = writer.add_edge_curve(None, vps[4], vps[5], l45, true);
    let ec56 = writer.add_edge_curve(None, vps[5], vps[6], l56, true);
    let ec67 = writer.add_edge_curve(None, vps[6], vps[7], l67, false);
    let ec74 = writer.add_edge_curve(None, vps[7], vps[4], l74, false);

    // Vertical
    let ec04 = writer.add_edge_curve(None, vps[0], vps[4], l04, true);
    let ec15 = writer.add_edge_curve(None, vps[1], vps[5], l15, true);
    let ec26 = writer.add_edge_curve(None, vps[2], vps[6], l26, true);
    let ec37 = writer.add_edge_curve(None, vps[3], vps[7], l37, true);

    // 7. Create oriented edges
    // Bottom Face (Normal -Z, CCW: p0->p3->p2->p1)
    let oe_bottom = vec![
        writer.add_oriented_edge(None, ec30, false), // p0->p3 (reverse of ec30: p3->p0)
        writer.add_oriented_edge(None, ec23, false), // p3->p2 (reverse of ec23: p2->p3)
        writer.add_oriented_edge(None, ec12, false), // p2->p1 (reverse of ec12: p1->p2)
        writer.add_oriented_edge(None, ec01, false), // p1->p0 (reverse of ec01: p0->p1)
    ];

    // Top Face (Normal +Z, CCW: p4->p5->p6->p7)
    let oe_top = vec![
        writer.add_oriented_edge(None, ec45, true),
        writer.add_oriented_edge(None, ec56, true),
        writer.add_oriented_edge(None, ec67, true),
        writer.add_oriented_edge(None, ec74, true),
    ];

    // Front Face (Normal -Y, CCW: p0->p4->p5->p1)
    let oe_front = vec![
        writer.add_oriented_edge(None, ec04, true),  // p0->p4
        writer.add_oriented_edge(None, ec45, true),  // p4->p5
        writer.add_oriented_edge(None, ec15, false), // p5->p1 (reverse of ec15: p1->p5)
        writer.add_oriented_edge(None, ec01, false), // p1->p0 (reverse of ec01: p0->p1)
    ];

    // Back Face (Normal +Y, CCW: p3->p2->p6->p7)
    let oe_back = vec![
        writer.add_oriented_edge(None, ec23, false), // p3 -> p2 (reverse of ec23: p2->p3)
        writer.add_oriented_edge(None, ec26, true),  // p2 -> p6
        writer.add_oriented_edge(None, ec67, true),  // p6 -> p7
        writer.add_oriented_edge(None, ec37, false), // p7 -> p3 (reverse of ec37: p3->p7)
    ];

    // Left Face (Normal -X, CCW: p0->p3->p7->p4) - previous logic was p3->p7->p4->p0 which is also CCW if p0 is starting point
    // p0->p3->p7->p4
    let oe_left = vec![
        writer.add_oriented_edge(None, ec30, false), // p0 -> p3
        writer.add_oriented_edge(None, ec37, true),  // p3 -> p7
        writer.add_oriented_edge(None, ec74, true),  // p7 -> p4
        writer.add_oriented_edge(None, ec04, false), // p4 -> p0
    ];

    // Right Face (Normal +X, CCW: p1->p5->p6->p2)
    let oe_right = vec![
        writer.add_oriented_edge(None, ec15, true),  // p1 -> p5
        writer.add_oriented_edge(None, ec56, true),  // p5 -> p6
        writer.add_oriented_edge(None, ec26, false), // p6 -> p2
        writer.add_oriented_edge(None, ec12, false), // p2 -> p1
    ];

    // Planes
    // Bottom (at p0, Z)
    let bottom_axis = writer.add_axis2_placement_3d(None, p_ids[0], Some(dir_z), Some(dir_x));
    let bottom_plane = writer.add_plane(None, bottom_axis);

    // Top (at p4, Z)
    let top_axis = writer.add_axis2_placement_3d(None, p_ids[4], Some(dir_z), Some(dir_x));
    let top_plane = writer.add_plane(None, top_axis);

    // Front (at p0, -Y)
    let front_axis = writer.add_axis2_placement_3d(None, p_ids[0], Some(dir_neg_y), Some(dir_x));
    let front_plane = writer.add_plane(None, front_axis);

    // Back (at p2, Y) - p2 is max X, max Y, min Z.
    let back_axis = writer.add_axis2_placement_3d(None, p_ids[2], Some(dir_y), Some(dir_neg_x));
    let back_plane = writer.add_plane(None, back_axis);

    // Left (at p0, -X)
    let left_axis = writer.add_axis2_placement_3d(None, p_ids[0], Some(dir_neg_x), Some(dir_y));
    let left_plane = writer.add_plane(None, left_axis);

    // Right (at p1, X)
    let right_axis = writer.add_axis2_placement_3d(None, p_ids[1], Some(dir_x), Some(dir_y));
    let right_plane = writer.add_plane(None, right_axis);

    // 8. Create FaceBounds
    let fb_bottom = writer.add_face_bound(None, oe_bottom, true);
    let fb_top = writer.add_face_bound(None, oe_top, true);
    let fb_front = writer.add_face_bound(None, oe_front, true);
    let fb_back = writer.add_face_bound(None, oe_back, true);
    let fb_left = writer.add_face_bound(None, oe_left, true);
    let fb_right = writer.add_face_bound(None, oe_right, true);

    // 9. Create AdvancedFaces
    let af_bottom = writer.add_advanced_face(None, bottom_plane, vec![fb_bottom]);
    let af_top = writer.add_advanced_face(None, top_plane, vec![fb_top]);
    let af_front = writer.add_advanced_face(None, front_plane, vec![fb_front]);
    let af_back = writer.add_advanced_face(None, back_plane, vec![fb_back]);
    let af_left = writer.add_advanced_face(None, left_plane, vec![fb_left]);
    let af_right = writer.add_advanced_face(None, right_plane, vec![fb_right]);

    // 10. Create ClosedShell
    let closed_shell = writer.add_closed_shell(None, vec![af_bottom, af_top, af_front, af_back, af_left, af_right]);

    // 11. Create ManifoldSolidBrep
    writer.add_manifold_solid_brep(None, closed_shell)
}

pub fn convert_assembly_to_step(assembly: &CrateAssembly) -> StepWriter {
    let mut writer = StepWriter::new();

    // === 1. Create Product Structure (ISO 10303-242 hierarchy) ===
    let app_context = writer.add_application_context("mechanical_design");
    let product_context = writer.add_product_context("mechanical", app_context, "mechanical");
    let product = writer.add_product(
        "AUTOCRATE-001",
        "AutoCrate Assembly",
        "ASTM D6039 Shipping Crate",
        vec![product_context]
    );
    let product_formation = writer.add_product_definition_formation(
        "1.0",
        Some("Initial design".to_string()),
        product
    );
    let def_context = writer.add_product_definition_context(
        "design",
        app_context,
        "design"
    );
    let product_def = writer.add_product_definition(
        "design",
        Some("Design definition".to_string()),
        product_formation,
        def_context
    );
    let product_def_shape = writer.add_product_definition_shape(
        Some("Assembly Shape".to_string()),
        None,
        product_def
    );

    // === 2. Create Datum Reference Frame (A|B|C) ===
    // Datum A: Base plane (Z=0, bottom of skids) - PRIMARY
    let datum_a_aspect = writer.add_shape_aspect(
        "Datum A Feature",
        Some("Base plane at Z=0".to_string()),
        product_def_shape,
        true
    );
    let datum_a = writer.add_datum(
        "Datum A",
        Some("Base plane (Z=0, bottom of skids)".to_string()),
        datum_a_aspect,
        "A"
    );
    let datum_ref_a = writer.add_datum_reference(1, datum_a); // Precedence 1 = primary

    // Datum B: Width centerplane (YZ at X=0) - SECONDARY
    let datum_b_aspect = writer.add_shape_aspect(
        "Datum B Feature",
        Some("Width centerplane".to_string()),
        product_def_shape,
        true
    );
    let datum_b = writer.add_datum(
        "Datum B",
        Some("Width centerplane (YZ at X=0)".to_string()),
        datum_b_aspect,
        "B"
    );
    let datum_ref_b = writer.add_datum_reference(2, datum_b); // Precedence 2 = secondary

    // Datum C: Length centerplane (XZ at Y=0) - TERTIARY
    let datum_c_aspect = writer.add_shape_aspect(
        "Datum C Feature",
        Some("Length centerplane".to_string()),
        product_def_shape,
        true
    );
    let datum_c = writer.add_datum(
        "Datum C",
        Some("Length centerplane (XZ at Y=0)".to_string()),
        datum_c_aspect,
        "C"
    );
    let datum_ref_c = writer.add_datum_reference(3, datum_c); // Precedence 3 = tertiary

    // Create datum system (A|B|C)
    let datum_system = writer.add_datum_system(
        "Primary DRF",
        Some("A|B|C".to_string()),
        product_def_shape,
        vec![datum_ref_a, datum_ref_b, datum_ref_c]
    );

    // === 3. Iterate through components and add geometry + tolerances ===
    let mut brep_ids = Vec::new();
    let unit_inch = EntityId(99999); // Placeholder - should create proper SI_UNIT

    for node in &assembly.nodes {
        if node.id == assembly.root_id {
            continue;
        }

        let global_transform = Mat4::from_rotation_translation(
            node.transform.rotation,
            node.transform.translation.to_vec3()
        );

        // Create B-rep geometry
        let brep_id = create_box_brep(&mut writer, global_transform, node.bounds.min.to_vec3(), node.bounds.max.to_vec3());
        brep_ids.push(brep_id);

        // Add PMI/GD&T based on component type
        match &node.component_type {
            ComponentType::Skid { .. } => {
                // Skid top surface - flatness tolerance ±0.125"
                let aspect = writer.add_shape_aspect(
                    &format!("{} Top Surface", node.name),
                    Some("Skid mounting surface".to_string()),
                    product_def_shape,
                    true
                );
                let magnitude = writer.add_length_measure_with_unit(0.125, unit_inch);
                writer.add_flatness_tolerance("Flatness 0.125\"", magnitude, aspect);

                // Material designation
                writer.add_material_designation(&node.name, "ASTM D245 No.2 Southern Pine, Pressure Treated");
            },

            ComponentType::Floorboard { .. } => {
                // Floorboard top surface - flatness tolerance ±0.0625"
                let aspect = writer.add_shape_aspect(
                    &format!("{} Top Surface", node.name),
                    Some("Floor mounting surface".to_string()),
                    product_def_shape,
                    true
                );
                let magnitude = writer.add_length_measure_with_unit(0.0625, unit_inch);
                writer.add_flatness_tolerance("Flatness 0.0625\"", magnitude, aspect);

                writer.add_material_designation(&node.name, "ASTM D245 No.2 Southern Pine");
            },

            ComponentType::Cleat { is_vertical, .. } => {
                if *is_vertical {
                    // Vertical cleat - perpendicularity to Datum A ±0.5°
                    let aspect = writer.add_shape_aspect(
                        &format!("{} Vertical Face", node.name),
                        Some("Corner post face".to_string()),
                        product_def_shape,
                        true
                    );
                    let magnitude = writer.add_length_measure_with_unit(0.0625, unit_inch); // ~0.5° in linear
                    writer.add_perpendicularity_tolerance(
                        "Perp 0.0625\" | A",
                        magnitude,
                        aspect,
                        datum_system
                    );
                }

                writer.add_material_designation(&node.name, "ASTM D245 No.2 Southern Pine");
            },

            ComponentType::Panel { .. } => {
                writer.add_material_designation(&node.name, "ASTM D3043 Grade C-C Plywood, 3/4\"");
            },

            ComponentType::Nail { x, y, z, .. } => {
                // Nail position - position tolerance ±0.25" relative to A|B|C
                let aspect = writer.add_shape_aspect(
                    &format!("Nail at ({:.1}, {:.1}, {:.1})", x, y, z),
                    Some(format!("Nailing coordinate {:.1},{:.1},{:.1}", x, y, z)),
                    product_def_shape,
                    true
                );
                let magnitude = writer.add_length_measure_with_unit(0.25, unit_inch);
                writer.add_position_tolerance(
                    &format!("Pos 0.25\" | A|B|C"),
                    Some("Datum-referenced fastener location".to_string()),
                    magnitude,
                    aspect,
                    datum_system
                );

                writer.add_material_designation(&node.name, "ASTM F1667 16d Common Nail, Galvanized");
            },

            _ => {
                // Other components - generic material
                writer.add_material_designation(&node.name, "As specified");
            }
        }
    }

    // === 4. Create shape representation and link to product ===
    let geom_context = writer.add_geometric_representation_context("3D", "assembly");
    let shape_rep = writer.add_shape_representation(
        "AutoCrate Assembly Geometry",
        brep_ids,
        geom_context
    );
    writer.add_shape_definition_representation(product_def_shape, shape_rep);

    writer
}