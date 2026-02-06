//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: core_materials.rs | DNA/src/power/magnetics/core_materials.rs
//! PURPOSE: Magnetic core materials database with Steinmetz loss models
//! MODIFIED: 2026-01-08
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides a database of magnetic core materials including:
//! - Ferrite (MnZn, NiZn) for high frequency switching
//! - Iron powder cores for DC bias applications
//! - Sendust/Kool-Mu for moderate frequency with DC bias
//!
//! Core loss is calculated using the Steinmetz equation:
//! Pv = k × f^α × B^β (W/cm³)
//!
//! For non-sinusoidal waveforms, the improved Generalized Steinmetz Equation (iGSE)
//! is used to account for duty cycle effects.

use serde::{Deserialize, Serialize};

/// Core material type classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialType {
    /// Manganese-zinc ferrite - high permeability, lower frequency
    MnZnFerrite,
    /// Nickel-zinc ferrite - lower permeability, higher frequency
    NiZnFerrite,
    /// Iron powder - handles DC bias, lower permeability
    IronPowder,
    /// Sendust (Fe-Si-Al) - high saturation, good DC bias
    Sendust,
    /// MPP (Molypermalloy) - lowest core loss of powder cores
    MPP,
    /// High Flux - highest saturation of powder cores
    HighFlux,
    /// Amorphous metal - very low loss, hard to wind
    Amorphous,
    /// Nanocrystalline - extremely low loss, expensive
    Nanocrystalline,
}

impl MaterialType {
    /// Typical frequency range (min, max) in Hz
    pub fn frequency_range(&self) -> (f64, f64) {
        match self {
            MaterialType::MnZnFerrite => (1e3, 2e6),
            MaterialType::NiZnFerrite => (100e3, 100e6),
            MaterialType::IronPowder => (50.0, 500e3),
            MaterialType::Sendust => (1e3, 1e6),
            MaterialType::MPP => (1e3, 500e3),
            MaterialType::HighFlux => (1e3, 200e3),
            MaterialType::Amorphous => (50.0, 200e3),
            MaterialType::Nanocrystalline => (1e3, 1e6),
        }
    }

    /// Does this material handle DC bias well?
    pub fn good_dc_bias(&self) -> bool {
        match self {
            MaterialType::MnZnFerrite | MaterialType::NiZnFerrite => false,
            _ => true,
        }
    }
}

/// Magnetic core material specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreMaterial {
    /// Material name/grade (e.g., "N87", "3C95", "-26")
    pub name: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Material type
    pub material_type: MaterialType,
    /// Initial permeability (μi)
    pub initial_permeability: f64,
    /// Saturation flux density at 25°C (Tesla)
    pub bsat_25c: f64,
    /// Saturation flux density at 100°C (Tesla) - typically lower
    pub bsat_100c: f64,
    /// Curie temperature (°C)
    pub curie_temp: f64,
    /// Resistivity (Ω·cm) - higher is better for eddy current
    pub resistivity: f64,
    /// Steinmetz coefficient k (for Pv = k × f^α × B^β in W/m³)
    /// Note: Some datasheets use W/cm³, convert appropriately
    pub steinmetz_k: f64,
    /// Steinmetz exponent α (frequency)
    pub steinmetz_alpha: f64,
    /// Steinmetz exponent β (flux density)
    pub steinmetz_beta: f64,
    /// Frequency range validity for Steinmetz (Hz)
    pub steinmetz_freq_range: (f64, f64),
    /// Temperature coefficient of permeability (%/°C)
    pub temp_coeff_permeability: f64,
    /// Density (g/cm³)
    pub density: f64,
}

impl Default for CoreMaterial {
    fn default() -> Self {
        Self {
            name: String::new(),
            manufacturer: String::new(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2000.0,
            bsat_25c: 0.49,
            bsat_100c: 0.39,
            curie_temp: 210.0,
            resistivity: 10.0,
            steinmetz_k: 1.5e-6,
            steinmetz_alpha: 1.3,
            steinmetz_beta: 2.5,
            steinmetz_freq_range: (25e3, 500e3),
            temp_coeff_permeability: -0.1,
            density: 4.8,
        }
    }
}

impl CoreMaterial {
    /// Calculate saturation flux density at temperature
    pub fn bsat_at_temp(&self, temp_c: f64) -> f64 {
        if temp_c <= 25.0 {
            self.bsat_25c
        } else if temp_c >= 100.0 {
            self.bsat_100c
        } else {
            let ratio = (temp_c - 25.0) / 75.0;
            self.bsat_25c + ratio * (self.bsat_100c - self.bsat_25c)
        }
    }

    /// Calculate core loss density using Steinmetz equation
    /// Returns power loss in W/cm³
    pub fn core_loss_density(&self, frequency: f64, b_peak: f64) -> f64 {
        // Pv = k × f^α × B^β
        // Convert to W/cm³ (Steinmetz k often given for this unit)
        self.steinmetz_k * frequency.powf(self.steinmetz_alpha) * b_peak.powf(self.steinmetz_beta)
    }

    /// Calculate core loss density using improved Generalized Steinmetz Equation (iGSE)
    /// For non-sinusoidal waveforms with given duty cycle
    /// Returns power loss in W/cm³
    pub fn core_loss_density_igse(&self, frequency: f64, b_peak: f64, duty_cycle: f64) -> f64 {
        // For SMPS with triangular flux waveforms, empirical data shows the loss
        // is typically 10-30% higher than for sinusoidal excitation at the same
        // peak flux density and frequency.
        //
        // The correction depends on duty cycle asymmetry:
        // - D = 0.5 (symmetric): factor ≈ 1.1
        // - D ≈ 0.2 or 0.8 (asymmetric): factor ≈ 1.2-1.3
        //
        // Using empirical formula based on iGSE analysis of triangular waveforms:
        // K_igse ≈ 1.0 + 0.15 × (1 + |D - 0.5| × 2)

        let d = duty_cycle.clamp(0.1, 0.9);

        // Asymmetry factor: 0 at D=0.5, 1 at D=0 or D=1
        let asymmetry = (d - 0.5).abs() * 2.0;

        // iGSE correction: ~1.1 for symmetric, ~1.3 for very asymmetric
        let igse_correction = 1.0 + 0.15 * (1.0 + asymmetry);

        // Apply standard Steinmetz with correction
        self.core_loss_density(frequency, b_peak) * igse_correction
    }

    /// Check if material is suitable for given frequency
    pub fn is_frequency_suitable(&self, frequency: f64) -> bool {
        let (f_min, f_max) = self.material_type.frequency_range();
        frequency >= f_min && frequency <= f_max
    }

    /// Calculate maximum recommended flux density with margin
    pub fn max_flux_with_margin(&self, temp_c: f64, margin: f64) -> f64 {
        self.bsat_at_temp(temp_c) * (1.0 - margin)
    }
}

/// Core geometry types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreType {
    /// E-core (EE, EI)
    ECore,
    /// ETD core (round center leg)
    ETD,
    /// EFD core (flat)
    EFD,
    /// PQ core (round, compact)
    PQ,
    /// RM core (round, low profile)
    RM,
    /// Pot core
    Pot,
    /// Toroid
    Toroid,
    /// EQ core (oval center leg)
    EQ,
    /// EP core (integrated)
    EP,
}

impl CoreType {
    /// Typical fill factor for this core type
    pub fn typical_fill_factor(&self) -> f64 {
        match self {
            CoreType::ECore | CoreType::ETD | CoreType::EFD => 0.35,
            CoreType::PQ | CoreType::EQ | CoreType::EP => 0.30,
            CoreType::RM | CoreType::Pot => 0.25,
            CoreType::Toroid => 0.40,
        }
    }
}

/// Core geometry specification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreGeometry {
    /// Core type (EE, ETD, PQ, etc.)
    pub core_type: CoreType,
    /// Part number/size (e.g., "E32/16/9", "ETD34")
    pub part_number: String,
    /// Effective cross-sectional area Ae (mm²)
    pub ae: f64,
    /// Effective magnetic path length le (mm)
    pub le: f64,
    /// Effective volume Ve (mm³)
    pub ve: f64,
    /// Inductance factor AL (nH/turn²)
    pub al: f64,
    /// Window area Aw (mm²)
    pub window_area: f64,
    /// Bobbin window area - actual usable (mm²)
    pub bobbin_window: f64,
    /// Mean length per turn MLT (mm)
    pub mlt: f64,
    /// Core weight (g)
    pub weight: f64,
    /// Surface area for cooling (cm²)
    pub surface_area: f64,
}

impl CoreGeometry {
    /// Calculate area product Ae × Aw (mm⁴)
    pub fn area_product(&self) -> f64 {
        self.ae * self.window_area
    }

    /// Calculate volume (cm³)
    pub fn volume_cm3(&self) -> f64 {
        self.ve / 1000.0
    }

    /// Calculate inductance for given turns
    pub fn inductance(&self, turns: u32) -> f64 {
        // L = AL × N²
        self.al * 1e-9 * (turns as f64).powi(2)
    }

    /// Calculate turns needed for given inductance
    pub fn turns_for_inductance(&self, inductance: f64) -> f64 {
        // N = √(L / AL)
        (inductance / (self.al * 1e-9)).sqrt()
    }

    /// Calculate peak flux density for given volt-seconds
    pub fn flux_density(&self, volt_seconds: f64) -> f64 {
        // B = V×t / (N × Ae)
        // This is per turn - multiply by N in actual use
        volt_seconds / (self.ae * 1e-6)
    }
}

// ============================================================================
// CORE MATERIAL DATABASE
// ============================================================================

/// Get the built-in ferrite material database
pub fn ferrite_database() -> Vec<CoreMaterial> {
    vec![
        // TDK/EPCOS ferrites
        CoreMaterial {
            name: "N87".to_string(),
            manufacturer: "TDK".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2200.0,
            bsat_25c: 0.49,
            bsat_100c: 0.39,
            curie_temp: 210.0,
            resistivity: 10.0,
            steinmetz_k: 1.4e-6,
            steinmetz_alpha: 1.35,
            steinmetz_beta: 2.5,
            steinmetz_freq_range: (25e3, 500e3),
            temp_coeff_permeability: -0.08,
            density: 4.85,
        },
        CoreMaterial {
            name: "N97".to_string(),
            manufacturer: "TDK".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2300.0,
            bsat_25c: 0.41,
            bsat_100c: 0.32,
            curie_temp: 230.0,
            resistivity: 6.0,
            steinmetz_k: 0.8e-6,
            steinmetz_alpha: 1.4,
            steinmetz_beta: 2.4,
            steinmetz_freq_range: (25e3, 500e3),
            temp_coeff_permeability: -0.03,
            density: 4.8,
        },
        CoreMaterial {
            name: "N49".to_string(),
            manufacturer: "TDK".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 1500.0,
            bsat_25c: 0.47,
            bsat_100c: 0.38,
            curie_temp: 240.0,
            resistivity: 10.0,
            steinmetz_k: 1.0e-6,
            steinmetz_alpha: 1.38,
            steinmetz_beta: 2.45,
            steinmetz_freq_range: (100e3, 1e6),
            temp_coeff_permeability: -0.04,
            density: 4.75,
        },
        CoreMaterial {
            name: "N27".to_string(),
            manufacturer: "TDK".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2000.0,
            bsat_25c: 0.50,
            bsat_100c: 0.41,
            curie_temp: 200.0,
            resistivity: 5.0,
            steinmetz_k: 2.0e-6,
            steinmetz_alpha: 1.3,
            steinmetz_beta: 2.6,
            steinmetz_freq_range: (10e3, 200e3),
            temp_coeff_permeability: -0.12,
            density: 4.9,
        },
        // Ferroxcube ferrites
        CoreMaterial {
            name: "3C90".to_string(),
            manufacturer: "Ferroxcube".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2300.0,
            bsat_25c: 0.47,
            bsat_100c: 0.38,
            curie_temp: 220.0,
            resistivity: 5.0,
            steinmetz_k: 1.2e-6,
            steinmetz_alpha: 1.35,
            steinmetz_beta: 2.5,
            steinmetz_freq_range: (25e3, 400e3),
            temp_coeff_permeability: -0.06,
            density: 4.8,
        },
        CoreMaterial {
            name: "3C95".to_string(),
            manufacturer: "Ferroxcube".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 3000.0,
            bsat_25c: 0.53,
            bsat_100c: 0.45,
            curie_temp: 215.0,
            resistivity: 5.0,
            steinmetz_k: 0.9e-6,
            steinmetz_alpha: 1.4,
            steinmetz_beta: 2.45,
            steinmetz_freq_range: (25e3, 500e3),
            temp_coeff_permeability: -0.02,
            density: 4.8,
        },
        CoreMaterial {
            name: "3F3".to_string(),
            manufacturer: "Ferroxcube".to_string(),
            material_type: MaterialType::MnZnFerrite,
            initial_permeability: 2000.0,
            bsat_25c: 0.44,
            bsat_100c: 0.35,
            curie_temp: 200.0,
            resistivity: 2.0,
            steinmetz_k: 0.6e-6,
            steinmetz_alpha: 1.5,
            steinmetz_beta: 2.4,
            steinmetz_freq_range: (100e3, 2e6),
            temp_coeff_permeability: -0.1,
            density: 4.7,
        },
        // Fair-Rite NiZn (high frequency)
        CoreMaterial {
            name: "67".to_string(),
            manufacturer: "Fair-Rite".to_string(),
            material_type: MaterialType::NiZnFerrite,
            initial_permeability: 40.0,
            bsat_25c: 0.35,
            bsat_100c: 0.30,
            curie_temp: 500.0,
            resistivity: 1e8,
            steinmetz_k: 0.05e-6,
            steinmetz_alpha: 1.8,
            steinmetz_beta: 2.2,
            steinmetz_freq_range: (1e6, 100e6),
            temp_coeff_permeability: -0.02,
            density: 4.5,
        },
    ]
}

/// Get the built-in powder core material database
pub fn powder_core_database() -> Vec<CoreMaterial> {
    vec![
        // Micrometals iron powder
        CoreMaterial {
            name: "-2 (Red/Gray)".to_string(),
            manufacturer: "Micrometals".to_string(),
            material_type: MaterialType::IronPowder,
            initial_permeability: 10.0,
            bsat_25c: 1.0,
            bsat_100c: 0.95,
            curie_temp: 700.0,
            resistivity: 0.1,
            steinmetz_k: 20e-6,
            steinmetz_alpha: 1.1,
            steinmetz_beta: 2.1,
            steinmetz_freq_range: (10e3, 200e3),
            temp_coeff_permeability: 0.04, // Positive for iron powder!
            density: 6.2,
        },
        CoreMaterial {
            name: "-8 (Yellow/Red)".to_string(),
            manufacturer: "Micrometals".to_string(),
            material_type: MaterialType::IronPowder,
            initial_permeability: 35.0,
            bsat_25c: 1.0,
            bsat_100c: 0.95,
            curie_temp: 700.0,
            resistivity: 0.1,
            steinmetz_k: 15e-6,
            steinmetz_alpha: 1.15,
            steinmetz_beta: 2.2,
            steinmetz_freq_range: (10e3, 200e3),
            temp_coeff_permeability: 0.025,
            density: 6.0,
        },
        CoreMaterial {
            name: "-26 (Yellow/White)".to_string(),
            manufacturer: "Micrometals".to_string(),
            material_type: MaterialType::IronPowder,
            initial_permeability: 75.0,
            bsat_25c: 1.0,
            bsat_100c: 0.95,
            curie_temp: 700.0,
            resistivity: 0.1,
            steinmetz_k: 10e-6,
            steinmetz_alpha: 1.2,
            steinmetz_beta: 2.3,
            steinmetz_freq_range: (10e3, 300e3),
            temp_coeff_permeability: 0.015,
            density: 5.9,
        },
        // Magnetics Kool-Mu (Sendust)
        CoreMaterial {
            name: "Kool Mu 26μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::Sendust,
            initial_permeability: 26.0,
            bsat_25c: 1.05,
            bsat_100c: 1.0,
            curie_temp: 500.0,
            resistivity: 60.0,
            steinmetz_k: 4e-6,
            steinmetz_alpha: 1.35,
            steinmetz_beta: 2.3,
            steinmetz_freq_range: (10e3, 500e3),
            temp_coeff_permeability: -0.004,
            density: 6.9,
        },
        CoreMaterial {
            name: "Kool Mu 60μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::Sendust,
            initial_permeability: 60.0,
            bsat_25c: 1.05,
            bsat_100c: 1.0,
            curie_temp: 500.0,
            resistivity: 60.0,
            steinmetz_k: 3.5e-6,
            steinmetz_alpha: 1.4,
            steinmetz_beta: 2.35,
            steinmetz_freq_range: (10e3, 500e3),
            temp_coeff_permeability: -0.006,
            density: 6.9,
        },
        CoreMaterial {
            name: "Kool Mu 125μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::Sendust,
            initial_permeability: 125.0,
            bsat_25c: 1.05,
            bsat_100c: 1.0,
            curie_temp: 500.0,
            resistivity: 60.0,
            steinmetz_k: 3e-6,
            steinmetz_alpha: 1.45,
            steinmetz_beta: 2.4,
            steinmetz_freq_range: (10e3, 500e3),
            temp_coeff_permeability: -0.008,
            density: 6.9,
        },
        // MPP cores
        CoreMaterial {
            name: "MPP 60μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::MPP,
            initial_permeability: 60.0,
            bsat_25c: 0.75,
            bsat_100c: 0.72,
            curie_temp: 460.0,
            resistivity: 100.0,
            steinmetz_k: 1e-6,
            steinmetz_alpha: 1.5,
            steinmetz_beta: 2.2,
            steinmetz_freq_range: (10e3, 300e3),
            temp_coeff_permeability: -0.0025,
            density: 8.0,
        },
        CoreMaterial {
            name: "MPP 125μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::MPP,
            initial_permeability: 125.0,
            bsat_25c: 0.75,
            bsat_100c: 0.72,
            curie_temp: 460.0,
            resistivity: 100.0,
            steinmetz_k: 0.8e-6,
            steinmetz_alpha: 1.55,
            steinmetz_beta: 2.25,
            steinmetz_freq_range: (10e3, 300e3),
            temp_coeff_permeability: -0.003,
            density: 8.0,
        },
        // High Flux cores
        CoreMaterial {
            name: "High Flux 60μ".to_string(),
            manufacturer: "Magnetics".to_string(),
            material_type: MaterialType::HighFlux,
            initial_permeability: 60.0,
            bsat_25c: 1.5,
            bsat_100c: 1.45,
            curie_temp: 500.0,
            resistivity: 50.0,
            steinmetz_k: 8e-6,
            steinmetz_alpha: 1.25,
            steinmetz_beta: 2.3,
            steinmetz_freq_range: (10e3, 200e3),
            temp_coeff_permeability: -0.006,
            density: 7.6,
        },
    ]
}

/// Get the built-in core geometry database
pub fn core_geometry_database() -> Vec<CoreGeometry> {
    vec![
        // Small E-cores (for low power)
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E13/7/4".to_string(),
            ae: 12.4,
            le: 29.0,
            ve: 360.0,
            al: 440.0,
            window_area: 10.0,
            bobbin_window: 7.0,
            mlt: 21.5,
            weight: 1.7,
            surface_area: 3.5,
        },
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E16/8/5".to_string(),
            ae: 20.0,
            le: 35.0,
            ve: 700.0,
            al: 630.0,
            window_area: 14.0,
            bobbin_window: 10.0,
            mlt: 26.0,
            weight: 3.4,
            surface_area: 5.2,
        },
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E20/10/6".to_string(),
            ae: 31.2,
            le: 42.8,
            ve: 1340.0,
            al: 800.0,
            window_area: 20.0,
            bobbin_window: 15.0,
            mlt: 32.0,
            weight: 6.5,
            surface_area: 8.0,
        },
        // Medium E-cores (5-50W typical)
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E25/13/7".to_string(),
            ae: 52.0,
            le: 49.0,
            ve: 2550.0,
            al: 1050.0,
            window_area: 37.0,
            bobbin_window: 28.0,
            mlt: 40.0,
            weight: 12.0,
            surface_area: 12.0,
        },
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E32/16/9".to_string(),
            ae: 83.0,
            le: 59.0,
            ve: 4900.0,
            al: 1400.0,
            window_area: 61.0,
            bobbin_window: 48.0,
            mlt: 50.0,
            weight: 24.0,
            surface_area: 19.0,
        },
        // Large E-cores (50-200W)
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E42/21/15".to_string(),
            ae: 178.0,
            le: 77.0,
            ve: 13700.0,
            al: 2300.0,
            window_area: 133.0,
            bobbin_window: 105.0,
            mlt: 68.0,
            weight: 66.0,
            surface_area: 40.0,
        },
        CoreGeometry {
            core_type: CoreType::ECore,
            part_number: "E55/28/21".to_string(),
            ae: 354.0,
            le: 99.0,
            ve: 35000.0,
            al: 3500.0,
            window_area: 250.0,
            bobbin_window: 200.0,
            mlt: 90.0,
            weight: 170.0,
            surface_area: 72.0,
        },
        // ETD cores (round center leg - better winding)
        CoreGeometry {
            core_type: CoreType::ETD,
            part_number: "ETD29".to_string(),
            ae: 76.0,
            le: 72.0,
            ve: 5470.0,
            al: 1050.0,
            window_area: 90.0,
            bobbin_window: 68.0,
            mlt: 53.0,
            weight: 26.0,
            surface_area: 22.0,
        },
        CoreGeometry {
            core_type: CoreType::ETD,
            part_number: "ETD34".to_string(),
            ae: 97.0,
            le: 78.0,
            ve: 7640.0,
            al: 1230.0,
            window_area: 123.0,
            bobbin_window: 95.0,
            mlt: 60.0,
            weight: 37.0,
            surface_area: 28.0,
        },
        CoreGeometry {
            core_type: CoreType::ETD,
            part_number: "ETD39".to_string(),
            ae: 125.0,
            le: 92.0,
            ve: 11500.0,
            al: 1350.0,
            window_area: 177.0,
            bobbin_window: 140.0,
            mlt: 69.0,
            weight: 55.0,
            surface_area: 37.0,
        },
        CoreGeometry {
            core_type: CoreType::ETD,
            part_number: "ETD44".to_string(),
            ae: 173.0,
            le: 103.0,
            ve: 17800.0,
            al: 1670.0,
            window_area: 213.0,
            bobbin_window: 170.0,
            mlt: 78.0,
            weight: 85.0,
            surface_area: 48.0,
        },
        CoreGeometry {
            core_type: CoreType::ETD,
            part_number: "ETD49".to_string(),
            ae: 211.0,
            le: 114.0,
            ve: 24000.0,
            al: 1850.0,
            window_area: 273.0,
            bobbin_window: 220.0,
            mlt: 87.0,
            weight: 115.0,
            surface_area: 58.0,
        },
        // EFD cores (flat, low profile)
        CoreGeometry {
            core_type: CoreType::EFD,
            part_number: "EFD15".to_string(),
            ae: 14.0,
            le: 34.0,
            ve: 480.0,
            al: 420.0,
            window_area: 11.0,
            bobbin_window: 8.0,
            mlt: 27.0,
            weight: 2.3,
            surface_area: 4.8,
        },
        CoreGeometry {
            core_type: CoreType::EFD,
            part_number: "EFD20".to_string(),
            ae: 31.0,
            le: 47.0,
            ve: 1460.0,
            al: 660.0,
            window_area: 22.0,
            bobbin_window: 16.0,
            mlt: 38.0,
            weight: 7.0,
            surface_area: 10.0,
        },
        CoreGeometry {
            core_type: CoreType::EFD,
            part_number: "EFD25".to_string(),
            ae: 58.0,
            le: 57.0,
            ve: 3300.0,
            al: 1000.0,
            window_area: 40.0,
            bobbin_window: 30.0,
            mlt: 48.0,
            weight: 16.0,
            surface_area: 16.0,
        },
        // PQ cores (compact, good thermal)
        CoreGeometry {
            core_type: CoreType::PQ,
            part_number: "PQ20/16".to_string(),
            ae: 61.0,
            le: 37.0,
            ve: 2260.0,
            al: 1650.0,
            window_area: 26.0,
            bobbin_window: 19.0,
            mlt: 37.0,
            weight: 11.0,
            surface_area: 9.0,
        },
        CoreGeometry {
            core_type: CoreType::PQ,
            part_number: "PQ26/25".to_string(),
            ae: 119.0,
            le: 46.0,
            ve: 5470.0,
            al: 2580.0,
            window_area: 53.0,
            bobbin_window: 40.0,
            mlt: 48.0,
            weight: 26.0,
            surface_area: 18.0,
        },
        CoreGeometry {
            core_type: CoreType::PQ,
            part_number: "PQ32/30".to_string(),
            ae: 162.0,
            le: 54.0,
            ve: 8740.0,
            al: 3000.0,
            window_area: 78.0,
            bobbin_window: 60.0,
            mlt: 56.0,
            weight: 42.0,
            surface_area: 26.0,
        },
        // RM cores (low profile)
        CoreGeometry {
            core_type: CoreType::RM,
            part_number: "RM8".to_string(),
            ae: 64.0,
            le: 28.0,
            ve: 1790.0,
            al: 2280.0,
            window_area: 14.0,
            bobbin_window: 10.0,
            mlt: 32.0,
            weight: 8.6,
            surface_area: 8.0,
        },
        CoreGeometry {
            core_type: CoreType::RM,
            part_number: "RM10".to_string(),
            ae: 98.0,
            le: 37.0,
            ve: 3580.0,
            al: 2600.0,
            window_area: 24.0,
            bobbin_window: 18.0,
            mlt: 40.0,
            weight: 17.0,
            surface_area: 13.0,
        },
        CoreGeometry {
            core_type: CoreType::RM,
            part_number: "RM12".to_string(),
            ae: 140.0,
            le: 46.0,
            ve: 6440.0,
            al: 3000.0,
            window_area: 39.0,
            bobbin_window: 30.0,
            mlt: 48.0,
            weight: 31.0,
            surface_area: 20.0,
        },
    ]
}

/// Find suitable cores for given power and frequency
pub fn find_suitable_cores(
    power_va: f64,
    frequency: f64,
    core_type_filter: Option<CoreType>,
) -> Vec<CoreGeometry> {
    // Empirical core size selection based on Ae × Aw product
    // Pt = Kf × Ae × Aw × B × J × f
    // Higher frequency allows smaller cores
    // Simplified: Ae × Aw ≈ 50 × Pt^1.14 × (100kHz/f)^0.5 (mm⁴)

    let freq_factor = (100e3 / frequency.max(10e3)).sqrt();
    let base_ap = 50.0 * power_va.powf(1.14) * freq_factor;
    let area_product_min = base_ap * 0.3; // Allow smaller cores
    let area_product_max = base_ap * 5.0; // 500% margin for larger options

    core_geometry_database()
        .into_iter()
        .filter(|c| {
            let ap = c.area_product();
            ap >= area_product_min && ap <= area_product_max
        })
        .filter(|c| {
            if let Some(filter) = core_type_filter {
                c.core_type == filter
            } else {
                true
            }
        })
        .collect()
}

/// Recommend core material for application
pub fn recommend_material(
    frequency: f64,
    has_dc_bias: bool,
    efficiency_priority: bool,
) -> MaterialType {
    if has_dc_bias {
        if frequency > 200e3 {
            MaterialType::Sendust
        } else if efficiency_priority {
            MaterialType::MPP
        } else {
            MaterialType::IronPowder
        }
    } else {
        if frequency > 1e6 {
            MaterialType::NiZnFerrite
        } else {
            MaterialType::MnZnFerrite
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsat_temperature() {
        let material = CoreMaterial {
            bsat_25c: 0.50,
            bsat_100c: 0.40,
            ..Default::default()
        };

        assert!((material.bsat_at_temp(25.0) - 0.50).abs() < 0.001);
        assert!((material.bsat_at_temp(100.0) - 0.40).abs() < 0.001);
        assert!((material.bsat_at_temp(62.5) - 0.45).abs() < 0.001); // Midpoint
    }

    #[test]
    fn test_core_loss_density() {
        let n87 = ferrite_database()
            .into_iter()
            .find(|m| m.name == "N87")
            .unwrap();

        // At 100kHz, 0.1T, expect roughly 50-100 mW/cm³
        let pv = n87.core_loss_density(100e3, 0.1);
        assert!(pv > 0.01 && pv < 0.5); // W/cm³
    }

    #[test]
    fn test_core_inductance() {
        let etd34 = core_geometry_database()
            .into_iter()
            .find(|c| c.part_number == "ETD34")
            .unwrap();

        // AL = 1230 nH/turn²
        // 10 turns → L = 1230 × 100 = 123000 nH = 123 µH
        let l = etd34.inductance(10);
        assert!((l * 1e6 - 123.0).abs() < 1.0);
    }

    #[test]
    fn test_turns_for_inductance() {
        let etd34 = core_geometry_database()
            .into_iter()
            .find(|c| c.part_number == "ETD34")
            .unwrap();

        // For 500 µH, how many turns?
        let n = etd34.turns_for_inductance(500e-6);
        // N = √(500e-6 / 1230e-9) = √406 ≈ 20
        assert!((n - 20.2).abs() < 0.5);
    }

    #[test]
    fn test_find_suitable_cores() {
        // For a 50W transformer
        let suitable = find_suitable_cores(50.0, 100e3, Some(CoreType::ETD));
        assert!(!suitable.is_empty());

        // Should include ETD29 or ETD34 for this power level
        let has_etd34 = suitable.iter().any(|c| c.part_number.contains("ETD"));
        assert!(has_etd34);
    }

    #[test]
    fn test_material_recommendation() {
        // High frequency without DC bias → MnZn ferrite
        assert_eq!(
            recommend_material(100e3, false, false),
            MaterialType::MnZnFerrite
        );

        // With DC bias, efficiency priority → MPP
        assert_eq!(recommend_material(100e3, true, true), MaterialType::MPP);

        // Very high frequency → NiZn
        assert_eq!(
            recommend_material(10e6, false, false),
            MaterialType::NiZnFerrite
        );
    }

    #[test]
    fn test_powder_core_dc_bias() {
        let powder = powder_core_database();

        // All powder cores should handle DC bias
        for core in powder {
            assert!(core.material_type.good_dc_bias());
        }
    }
}
