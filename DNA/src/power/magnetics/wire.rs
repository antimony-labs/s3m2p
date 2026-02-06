//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: wire.rs | DNA/src/power/magnetics/wire.rs
//! PURPOSE: Wire properties database with skin effect and proximity effect calculations
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides wire specifications and AC resistance calculations:
//!
//! - **AWG Database**: Standard American Wire Gauge specifications
//! - **Skin Effect**: AC resistance increase due to current crowding
//! - **Proximity Effect**: Additional loss in multi-layer windings
//! - **Litz Wire**: Stranded wire for high-frequency applications

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Copper resistivity at 20°C (Ω·m)
pub const COPPER_RESISTIVITY_20C: f64 = 1.68e-8;

/// Copper temperature coefficient (per °C)
pub const COPPER_TEMP_COEFF: f64 = 0.00393;

/// Copper permeability (≈ µ0)
pub const COPPER_PERMEABILITY: f64 = 4.0 * PI * 1e-7;

/// Wire type classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WireType {
    /// Solid round magnet wire
    SolidRound,
    /// Rectangular/flat wire
    Rectangular,
    /// Litz wire (stranded for HF)
    Litz,
    /// Foil winding
    Foil,
}

/// Insulation class/grade
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsulationClass {
    /// Single build (thinnest)
    Single,
    /// Heavy build (standard)
    Heavy,
    /// Triple build (thick)
    Triple,
    /// Quad build (thickest)
    Quad,
}

impl InsulationClass {
    /// Typical insulation thickness per side as fraction of wire diameter
    pub fn thickness_fraction(&self) -> f64 {
        match self {
            InsulationClass::Single => 0.015,
            InsulationClass::Heavy => 0.030,
            InsulationClass::Triple => 0.045,
            InsulationClass::Quad => 0.060,
        }
    }
}

/// Solid round wire specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WireSpec {
    /// AWG gauge number
    pub awg: i8,
    /// Bare wire diameter (mm)
    pub diameter_mm: f64,
    /// Cross-sectional area (mm²)
    pub area_mm2: f64,
    /// DC resistance at 20°C (Ω/m)
    pub resistance_per_m: f64,
    /// Recommended max current for chassis wiring (A)
    pub current_capacity_chassis: f64,
    /// Recommended max current for bundled/transformer use (A)
    pub current_capacity_bundled: f64,
    /// Weight (g/m)
    pub weight_per_m: f64,
}

impl WireSpec {
    /// Calculate outer diameter with insulation
    pub fn outer_diameter(&self, insulation: InsulationClass) -> f64 {
        let insulation_thickness = self.diameter_mm * insulation.thickness_fraction();
        self.diameter_mm + 2.0 * insulation_thickness
    }

    /// Calculate resistance at temperature
    pub fn resistance_at_temp(&self, temp_c: f64) -> f64 {
        self.resistance_per_m * (1.0 + COPPER_TEMP_COEFF * (temp_c - 20.0))
    }

    /// Calculate skin depth at frequency
    pub fn skin_depth(&self, frequency: f64) -> f64 {
        skin_depth(frequency, COPPER_RESISTIVITY_20C, COPPER_PERMEABILITY)
    }

    /// Calculate AC/DC resistance ratio at frequency
    pub fn ac_resistance_factor(&self, frequency: f64) -> f64 {
        let delta = self.skin_depth(frequency);
        let radius = self.diameter_mm / 2000.0; // Convert to meters
        ac_resistance_factor_dowell(radius, delta)
    }

    /// Calculate AC resistance at frequency and temperature
    pub fn ac_resistance(&self, frequency: f64, temp_c: f64) -> f64 {
        self.resistance_at_temp(temp_c) * self.ac_resistance_factor(frequency)
    }
}

/// Litz wire specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitzWireSpec {
    /// Number of strands
    pub strand_count: u32,
    /// AWG of each strand
    pub strand_awg: i8,
    /// Individual strand specification
    pub strand_spec: WireSpec,
    /// Overall outer diameter (mm)
    pub outer_diameter: f64,
    /// Effective cross-sectional area (mm²)
    pub effective_area_mm2: f64,
    /// DC resistance at 20°C (Ω/m)
    pub dc_resistance_per_m: f64,
}

impl LitzWireSpec {
    /// Create Litz wire specification from strand count and gauge
    pub fn new(strand_count: u32, strand_awg: i8) -> Self {
        let strand_spec = awg_database()
            .into_iter()
            .find(|w| w.awg == strand_awg)
            .unwrap_or_else(|| awg_spec(strand_awg));

        let effective_area = strand_spec.area_mm2 * strand_count as f64;
        let dc_resistance = strand_spec.resistance_per_m / strand_count as f64;

        // Approximate outer diameter (packing factor ~0.7)
        let bundle_area = effective_area / 0.7;
        let outer_diameter = 2.0 * (bundle_area / PI).sqrt();

        Self {
            strand_count,
            strand_awg,
            strand_spec,
            outer_diameter,
            effective_area_mm2: effective_area,
            dc_resistance_per_m: dc_resistance,
        }
    }

    /// Calculate AC/DC resistance ratio
    /// Litz wire has much better HF performance than solid wire of same area
    pub fn ac_resistance_factor(&self, frequency: f64) -> f64 {
        // For well-designed Litz wire, each strand sees skin effect independently
        let strand_factor = self.strand_spec.ac_resistance_factor(frequency);

        // Add small penalty for strand proximity within bundle (~10-20%)
        strand_factor * 1.15
    }

    /// Calculate AC resistance at frequency and temperature
    pub fn ac_resistance(&self, frequency: f64, temp_c: f64) -> f64 {
        let dc_at_temp = self.dc_resistance_per_m * (1.0 + COPPER_TEMP_COEFF * (temp_c - 20.0));
        dc_at_temp * self.ac_resistance_factor(frequency)
    }

    /// Maximum effective frequency (where Litz advantage is lost)
    pub fn max_effective_frequency(&self) -> f64 {
        // Litz is effective when strand diameter < 2 × skin depth
        // δ = √(ρ / (π × f × µ))
        // f_max = ρ / (π × µ × (d/2)²)
        let d = self.strand_spec.diameter_mm / 1000.0; // Convert to meters
        COPPER_RESISTIVITY_20C / (PI * COPPER_PERMEABILITY * (d / 2.0).powi(2))
    }
}

// ============================================================================
// SKIN EFFECT AND PROXIMITY EFFECT CALCULATIONS
// ============================================================================

/// Calculate skin depth in a conductor
/// δ = √(ρ / (π × f × µ))
pub fn skin_depth(frequency: f64, resistivity: f64, permeability: f64) -> f64 {
    if frequency <= 0.0 {
        return f64::INFINITY;
    }
    (resistivity / (PI * frequency * permeability)).sqrt()
}

/// Calculate skin depth in copper at given frequency
pub fn copper_skin_depth(frequency: f64) -> f64 {
    skin_depth(frequency, COPPER_RESISTIVITY_20C, COPPER_PERMEABILITY)
}

/// Calculate AC/DC resistance ratio using Dowell's formula
/// For a single solid round wire
pub fn ac_resistance_factor_dowell(radius: f64, skin_depth: f64) -> f64 {
    if skin_depth <= 0.0 || radius <= 0.0 {
        return 1.0;
    }

    let x = radius / skin_depth;

    if x < 0.5 {
        // Low frequency approximation
        1.0 + x.powi(4) / 48.0
    } else if x < 10.0 {
        // Use exact Dowell formula
        // Fr = x × (ber(x)×bei'(x) - bei(x)×ber'(x)) / (ber'(x)² + bei'(x)²)
        // Simplified approximation:
        let x2 = x * x;
        let x4 = x2 * x2;

        // Polynomial approximation valid for 0.5 < x < 10
        1.0 + x4 / 48.0 + x2.powi(2) / 3840.0 + x4 * x2 / 322560.0 - (x4 * x4) / 129024000.0
    } else {
        // High frequency - resistance approaches x/√2
        x / (2.0_f64).sqrt()
    }
}

/// Calculate proximity effect factor for multi-layer windings
/// Based on Dowell's equations for multiple layers
pub fn proximity_factor(
    wire_diameter: f64,
    skin_depth: f64,
    num_layers: u32,
    layer_fill_factor: f64,
) -> f64 {
    if num_layers == 1 {
        return 1.0;
    }

    let x = wire_diameter / (2.0 * skin_depth);
    let eta = layer_fill_factor.sqrt(); // Porosity factor

    if x < 0.5 {
        // Low frequency approximation
        let m = num_layers as f64;
        1.0 + (m * m - 1.0) * eta.powi(2) * x.powi(4) / 180.0
    } else {
        // Higher frequency - proximity effect dominates
        let m = num_layers as f64;
        let g = (2.0 * m * m - 1.0) / 3.0;
        let x2 = x * x;

        // Combined skin + proximity
        1.0 + (x2 / 4.0) * (1.0 + g * eta.powi(2))
    }
}

/// Calculate total AC resistance factor for multi-layer winding
pub fn total_ac_resistance_factor(
    wire_diameter: f64,
    frequency: f64,
    num_layers: u32,
    layer_fill_factor: f64,
) -> f64 {
    let delta = copper_skin_depth(frequency);
    let fr_skin = ac_resistance_factor_dowell(wire_diameter / 2000.0, delta); // mm to m
    let fr_prox = proximity_factor(wire_diameter / 1000.0, delta, num_layers, layer_fill_factor);

    fr_skin * fr_prox
}

/// Recommend Litz wire strand gauge for frequency
/// Returns AWG for individual strands
pub fn recommended_litz_strand_awg(frequency: f64) -> i8 {
    let delta = copper_skin_depth(frequency);
    let ideal_diameter = delta * 2.0; // Wire diameter should be < 2× skin depth

    // Find closest AWG
    let diameter_mm = ideal_diameter * 1000.0;

    if diameter_mm >= 0.5 {
        24
    } else if diameter_mm >= 0.35 {
        26
    } else if diameter_mm >= 0.25 {
        28
    } else if diameter_mm >= 0.18 {
        30
    } else if diameter_mm >= 0.12 {
        32
    } else if diameter_mm >= 0.08 {
        36
    } else if diameter_mm >= 0.05 {
        40
    } else {
        44
    }
}

// ============================================================================
// AWG WIRE DATABASE
// ============================================================================

/// Calculate AWG specification from gauge number
pub fn awg_spec(awg: i8) -> WireSpec {
    // AWG formula: diameter (mm) = 0.127 × 92^((36-AWG)/39)
    let diameter = 0.127 * 92.0_f64.powf((36 - awg) as f64 / 39.0);
    let area = PI * (diameter / 2.0).powi(2);
    let resistance = COPPER_RESISTIVITY_20C * 1e6 / area; // Ω/m (area in mm²)

    // Current capacity: roughly 700 circular mils per amp for bundled
    // 1 circular mil = 0.0005067 mm²
    let circular_mils = area / 0.0005067;
    let current_bundled = (circular_mils / 700.0).max(0.1);
    let current_chassis = current_bundled * 1.5;

    // Weight: copper density 8.96 g/cm³
    let weight = area * 8.96 / 1000.0; // g/m

    WireSpec {
        awg,
        diameter_mm: diameter,
        area_mm2: area,
        resistance_per_m: resistance,
        current_capacity_chassis: current_chassis,
        current_capacity_bundled: current_bundled,
        weight_per_m: weight,
    }
}

/// Get the standard AWG wire database (10 to 44 AWG)
pub fn awg_database() -> Vec<WireSpec> {
    (10..=44).map(awg_spec).collect()
}

/// Find wire gauge for given current (bundled/transformer use)
pub fn wire_for_current(current: f64, current_density_a_mm2: f64) -> WireSpec {
    let required_area = current / current_density_a_mm2;

    // Find smallest AWG (thinnest wire) with sufficient area
    // AWG goes backwards: AWG 10 is thicker than AWG 40
    // Iterate from thin (40) to thick (10) and return first with enough area
    for awg in (10..=40).rev() {
        let spec = awg_spec(awg);
        if spec.area_mm2 >= required_area {
            return spec;
        }
    }

    // If nothing found, return AWG 10 (thickest in range)
    awg_spec(10)
}

/// Recommended current density for different applications (A/mm²)
#[derive(Clone, Copy, Debug)]
pub enum CurrentDensity {
    /// Conservative for enclosed transformer (3 A/mm²)
    Enclosed,
    /// Normal for ventilated design (4-5 A/mm²)
    Ventilated,
    /// Aggressive for forced cooling (6-8 A/mm²)
    ForcedCooling,
}

impl CurrentDensity {
    /// Get current density value in A/mm²
    pub fn value(&self) -> f64 {
        match self {
            CurrentDensity::Enclosed => 3.0,
            CurrentDensity::Ventilated => 4.5,
            CurrentDensity::ForcedCooling => 7.0,
        }
    }
}

// ============================================================================
// FOIL WINDING
// ============================================================================

/// Copper foil specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoilSpec {
    /// Foil thickness (mm)
    pub thickness_mm: f64,
    /// Foil width (mm)
    pub width_mm: f64,
    /// Cross-sectional area (mm²)
    pub area_mm2: f64,
    /// DC resistance per unit length (Ω/m)
    pub resistance_per_m: f64,
}

impl FoilSpec {
    /// Create foil specification from dimensions
    pub fn new(thickness_mm: f64, width_mm: f64) -> Self {
        let area = thickness_mm * width_mm;
        let resistance = COPPER_RESISTIVITY_20C * 1e6 / area;

        Self {
            thickness_mm,
            width_mm,
            area_mm2: area,
            resistance_per_m: resistance,
        }
    }

    /// Calculate AC/DC resistance ratio for foil
    /// Different formula than round wire
    pub fn ac_resistance_factor(&self, frequency: f64) -> f64 {
        let delta = copper_skin_depth(frequency);
        let d = self.thickness_mm / 1000.0; // Convert to meters

        let x = d / delta;

        if x < 0.5 {
            // Low frequency - minimal skin effect
            1.0 + x.powi(4) / 45.0
        } else if x < 5.0 {
            // Medium frequency
            x / ((2.0 * x).tanh())
        } else {
            // High frequency - current flows on surfaces
            x
        }
    }
}

// ============================================================================
// COMMON LITZ WIRE CONFIGURATIONS
// ============================================================================

/// Get common Litz wire configurations
pub fn common_litz_wires() -> Vec<LitzWireSpec> {
    vec![
        // Low power / high frequency
        LitzWireSpec::new(30, 38),  // 30 strands of AWG 38
        LitzWireSpec::new(50, 40),  // 50 strands of AWG 40
        LitzWireSpec::new(100, 40), // 100 strands of AWG 40
        // Medium power
        LitzWireSpec::new(60, 36),  // 60 strands of AWG 36
        LitzWireSpec::new(100, 36), // 100 strands of AWG 36
        LitzWireSpec::new(175, 38), // 175 strands of AWG 38
        // Higher power
        LitzWireSpec::new(100, 32), // 100 strands of AWG 32
        LitzWireSpec::new(200, 36), // 200 strands of AWG 36
        LitzWireSpec::new(400, 38), // 400 strands of AWG 38
        // High power
        LitzWireSpec::new(300, 32), // 300 strands of AWG 32
        LitzWireSpec::new(500, 36), // 500 strands of AWG 36
        LitzWireSpec::new(800, 38), // 800 strands of AWG 38
    ]
}

/// Find suitable Litz wire for current and frequency
pub fn find_litz_wire(
    current_rms: f64,
    frequency: f64,
    current_density: CurrentDensity,
) -> Option<LitzWireSpec> {
    let required_area = current_rms / current_density.value();
    let recommended_strand = recommended_litz_strand_awg(frequency);

    common_litz_wires()
        .into_iter()
        .filter(|litz| {
            litz.effective_area_mm2 >= required_area && litz.strand_awg >= recommended_strand - 4
        })
        .min_by(|a, b| {
            a.effective_area_mm2
                .partial_cmp(&b.effective_area_mm2)
                .unwrap()
        })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skin_depth() {
        // At 100kHz, copper skin depth ≈ 0.21mm
        let delta = copper_skin_depth(100e3);
        assert!((delta * 1000.0 - 0.21).abs() < 0.02);

        // At 1MHz, ≈ 0.066mm
        let delta_1mhz = copper_skin_depth(1e6);
        assert!((delta_1mhz * 1000.0 - 0.066).abs() < 0.01);
    }

    #[test]
    fn test_awg_spec() {
        // AWG 22 should be approximately 0.64mm diameter
        let awg22 = awg_spec(22);
        assert!((awg22.diameter_mm - 0.644).abs() < 0.01);

        // AWG 10 should be approximately 2.59mm diameter
        let awg10 = awg_spec(10);
        assert!((awg10.diameter_mm - 2.59).abs() < 0.05);
    }

    #[test]
    fn test_ac_resistance_factor() {
        let awg22 = awg_spec(22);

        // At DC (low frequency), factor should be ~1.0
        let fr_dc = awg22.ac_resistance_factor(60.0);
        assert!((fr_dc - 1.0).abs() < 0.01);

        // At 100kHz, factor should be > 1
        let fr_100k = awg22.ac_resistance_factor(100e3);
        assert!(fr_100k > 1.0);

        // At 1MHz, factor should be significant
        let fr_1m = awg22.ac_resistance_factor(1e6);
        assert!(fr_1m > 2.0);
    }

    #[test]
    fn test_litz_wire() {
        let litz = LitzWireSpec::new(100, 38);

        // Should have 100× the area of single strand
        let single = awg_spec(38);
        assert!((litz.effective_area_mm2 / single.area_mm2 - 100.0).abs() < 0.1);

        // DC resistance should be 1/100th
        assert!((litz.dc_resistance_per_m * 100.0 / single.resistance_per_m - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_wire_for_current() {
        // 5A at 4 A/mm² needs 1.25 mm²
        let wire = wire_for_current(5.0, 4.0);
        assert!(wire.area_mm2 >= 1.25);

        // Should be around AWG 16-18
        assert!(wire.awg >= 16 && wire.awg <= 20);
    }

    #[test]
    fn test_proximity_effect() {
        // Single layer should have factor = 1
        let fp1 = proximity_factor(0.5, 0.2, 1, 0.7);
        assert!((fp1 - 1.0).abs() < 0.01);

        // Multiple layers should have higher factor
        let fp4 = proximity_factor(0.5, 0.2, 4, 0.7);
        assert!(fp4 > 1.0);
    }

    #[test]
    fn test_recommended_litz_strand() {
        // At 100kHz, skin depth ≈ 0.21mm, so ideal strand ≈ 0.42mm ≈ AWG 26
        let awg_100k = recommended_litz_strand_awg(100e3);
        assert!(
            awg_100k >= 24 && awg_100k <= 30,
            "Expected AWG 24-30 at 100kHz, got {}",
            awg_100k
        );

        // At 1MHz, skin depth ≈ 0.066mm, so ideal strand ≈ 0.13mm ≈ AWG 32
        let awg_1m = recommended_litz_strand_awg(1e6);
        assert!(awg_1m >= 30, "Expected AWG >= 30 at 1MHz, got {}", awg_1m);
    }

    #[test]
    fn test_foil_ac_resistance() {
        let foil = FoilSpec::new(0.1, 10.0); // 0.1mm × 10mm

        // At low frequency, factor ≈ 1
        let fr_low = foil.ac_resistance_factor(1e3);
        assert!((fr_low - 1.0).abs() < 0.1);

        // At high frequency, factor increases
        let fr_high = foil.ac_resistance_factor(1e6);
        assert!(fr_high > 1.0);
    }
}
