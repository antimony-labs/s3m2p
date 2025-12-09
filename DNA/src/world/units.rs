//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: units.rs | DNA/src/world/units.rs
//! PURPOSE: Type-safe physical units and conversions
//! LAYER: DNA → WORLD
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Unit conversions:
//!   Length: meter (m), kilometer (km), astronomical unit (AU)
//!   Time: second (s), hour (h), day (d), year (yr)
//!   Angle: radian (rad), degree (deg)
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use std::f32::consts::PI;

// Length conversions
pub const AU_TO_M: f64 = 1.495_978_707e11;
pub const AU_TO_KM: f64 = 1.495_978_707e8;
pub const KM_TO_M: f64 = 1000.0;

// Time conversions
pub const HOUR_TO_S: f64 = 3600.0;
pub const DAY_TO_S: f64 = 86400.0;
pub const YEAR_TO_S: f64 = 31557600.0; // Julian year

// Angle conversions
#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / PI
}

// TODO: Type-safe unit wrapper types (Meter, Second, Kilogram, etc.)
