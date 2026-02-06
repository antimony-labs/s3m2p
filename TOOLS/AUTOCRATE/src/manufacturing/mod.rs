//! Manufacturing data generation for AutoCrate
//!
//! Generates comprehensive manufacturing outputs from crate assembly:
//! - Bill of Materials (BOM)
//! - Lumber cut lists with tolerances
//! - Nailing coordinates (datum-referenced)
//! - CNC programs for panel cutting

use crate::{assembly::*, constants::LumberSize, geometry::Point3};
use serde::{Deserialize, Serialize};

pub mod bom;
pub mod cnc;
pub mod cut_list;
pub mod nailing;
pub mod part_numbers;

// Re-exports
pub use bom::{generate_bom, BillOfMaterials, BomEntry};
pub use cnc::{generate_cnc_program, CncCutProgram, CncOperation, CncOperationType};
pub use cut_list::{generate_cut_list, CutListEntry};
pub use nailing::{generate_nailing_coordinates, DatumReference, NailingCoordinate};

/// Complete manufacturing data package
#[derive(Clone, Debug)]
pub struct ManufacturingData {
    pub bom: BillOfMaterials,
    pub cut_list: Vec<CutListEntry>,
    pub nailing_coords: Vec<NailingCoordinate>,
    pub cnc_program: CncCutProgram,
}

/// Lumber grade classifications per ASTM D245
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LumberGrade {
    Select,       // Highest quality, minimal defects
    No1,          // #1 grade structural lumber
    No2,          // #2 grade structural lumber (most common)
    No3,          // #3 grade utility lumber
    Stud,         // Stud grade (2x4, 2x6)
    Construction, // Construction grade
}

impl LumberGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            LumberGrade::Select => "Select",
            LumberGrade::No1 => "No.1",
            LumberGrade::No2 => "No.2",
            LumberGrade::No3 => "No.3",
            LumberGrade::Stud => "Stud",
            LumberGrade::Construction => "Construction",
        }
    }
}

/// Wood species
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WoodSpecies {
    SouthernPine,
    DouglasFir,
    Spruce,
    Hemlock,
}

impl WoodSpecies {
    pub fn as_str(&self) -> &'static str {
        match self {
            WoodSpecies::SouthernPine => "Southern Pine",
            WoodSpecies::DouglasFir => "Douglas Fir",
            WoodSpecies::Spruce => "Spruce",
            WoodSpecies::Hemlock => "Hemlock",
        }
    }
}

/// Plywood grade classifications
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlywoodGrade {
    ACExterior,   // A-C Exterior grade
    BCExterior,   // B-C Exterior grade
    CDXSheathing, // CDX Sheathing grade
}

impl PlywoodGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlywoodGrade::ACExterior => "A-C Exterior",
            PlywoodGrade::BCExterior => "B-C Exterior",
            PlywoodGrade::CDXSheathing => "CDX Sheathing",
        }
    }
}

/// Fastener types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FastenerType {
    CommonNail,
    RingShankNail,
    LagScrew,
}

impl FastenerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FastenerType::CommonNail => "Common Nail",
            FastenerType::RingShankNail => "Ring Shank Nail",
            FastenerType::LagScrew => "Lag Screw",
        }
    }
}

/// Material specification for a component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MaterialSpec {
    Lumber {
        size: LumberSize,
        grade: LumberGrade,
        species: WoodSpecies,
    },
    Plywood {
        thickness: f32,
        grade: PlywoodGrade,
    },
    Fastener {
        fastener_type: FastenerType,
        size: String, // e.g., "16d", "3.5in"
    },
}

impl MaterialSpec {
    pub fn format_spec(&self) -> String {
        match self {
            MaterialSpec::Lumber {
                size,
                grade,
                species,
            } => {
                format!("{} {} {}", size.name(), grade.as_str(), species.as_str())
            }
            MaterialSpec::Plywood { thickness, grade } => {
                format!("{:.2}\" {} Plywood", thickness, grade.as_str())
            }
            MaterialSpec::Fastener {
                fastener_type,
                size,
            } => {
                format!("{} {}", size, fastener_type.as_str())
            }
        }
    }
}

/// Tolerance specification for dimensions
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tolerance {
    pub nominal: f32,                  // Nominal dimension (inches)
    pub plus: f32,                     // Upper tolerance (inches)
    pub minus: f32,                    // Lower tolerance (inches)
    pub straightness: Option<f32>,     // Straightness tolerance (in/ft)
    pub perpendicularity: Option<f32>, // Perpendicularity tolerance (degrees)
}

impl Tolerance {
    /// Create a symmetric tolerance
    pub fn symmetric(nominal: f32, tolerance: f32) -> Self {
        Self {
            nominal,
            plus: tolerance,
            minus: tolerance,
            straightness: None,
            perpendicularity: None,
        }
    }

    /// Create a tolerance with straightness and perpendicularity
    pub fn lumber(nominal: f32, tolerance: f32, straightness: f32, perp: f32) -> Self {
        Self {
            nominal,
            plus: tolerance,
            minus: tolerance,
            straightness: Some(straightness),
            perpendicularity: Some(perp),
        }
    }
}

/// Generate complete manufacturing data from assembly
pub fn generate_manufacturing_data(
    assembly: &CrateAssembly,
    product_weight: f32,
) -> ManufacturingData {
    ManufacturingData {
        bom: generate_bom(assembly, product_weight),
        cut_list: generate_cut_list(assembly, product_weight),
        nailing_coords: generate_nailing_coordinates(assembly),
        cnc_program: generate_cnc_program(assembly),
    }
}

/// Determine lumber grade based on product weight
pub fn determine_lumber_grade(product_weight: f32) -> LumberGrade {
    if product_weight > 10000.0 {
        LumberGrade::No1 // Heavy crates require stronger lumber
    } else {
        LumberGrade::No2 // Standard grade for typical crates
    }
}
