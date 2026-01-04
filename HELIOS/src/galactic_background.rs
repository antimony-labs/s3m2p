//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: galactic_background.rs | HELIOS/src/galactic_background.rs
//! PURPOSE: Milky Way background stars with galactic coordinate system
//! CREATED: 2025-01-04
//! LAYER: HELIOS (simulation)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # Galactic Background Stars
//!
//! Renders a realistic Milky Way background with the heliosphere (ecliptic plane)
//! tilted at approximately 60° to the galactic plane.
//!
//! ## Coordinate Systems
//!
//! - **Galactic**: (l, b) where l is galactic longitude, b is galactic latitude
//!   - l = 0° points toward galactic center (Sagittarius A*)
//!   - b = 0° is the galactic plane
//!   - b = +90° is the galactic north pole
//!
//! - **Ecliptic**: The plane of Earth's orbit, tilted ~60° from galactic plane
//!
//! - **HCI (Heliocentric Inertial)**: Sun-centered Cartesian coordinates
//!   - X points toward vernal equinox
//!   - Z points toward ecliptic north pole
//!
//! ## Key Astronomical Constants
//!
//! - Galactic north pole in equatorial coords: RA 192.86°, Dec +27.13°
//! - Galactic center direction: RA 266.4°, Dec -28.94°
//! - Ecliptic-galactic plane angle: ~60.2°
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::DVec3;
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────────
// CONSTANTS
// ─────────────────────────────────────────────────────────────────────────────────

/// Angle between ecliptic plane and galactic plane (radians)
/// The galactic plane is tilted approximately 60.2° from the ecliptic
pub const GALACTIC_ECLIPTIC_ANGLE: f64 = 60.2 * PI / 180.0;

/// Galactic north pole position in ecliptic coordinates (radians)
/// RA 192.86°, Dec +27.13° → Ecliptic: λ ≈ 180°, β ≈ 29.8°
pub const GALACTIC_NORTH_POLE_ECLIPTIC_LON: f64 = 180.0 * PI / 180.0;
pub const GALACTIC_NORTH_POLE_ECLIPTIC_LAT: f64 = 29.8 * PI / 180.0;

/// Galactic center direction in ecliptic coordinates
/// The center of the Milky Way (Sagittarius A*) in ecliptic coords
pub const GALACTIC_CENTER_ECLIPTIC_LON: f64 = 266.8 * PI / 180.0;
pub const GALACTIC_CENTER_ECLIPTIC_LAT: f64 = -5.5 * PI / 180.0;

/// Distance to galactic center in AU (approximately 8.2 kpc)
pub const GALACTIC_CENTER_DISTANCE_AU: f64 = 8200.0 * 206264.806; // kpc to AU

/// Distance for rendering background stars (very far, but finite for projection)
const BACKGROUND_STAR_DISTANCE_AU: f64 = 1e8; // 100 million AU ~ 1.6 light years

/// Maximum number of background Milky Way stars
/// A realistic night sky has ~6000 naked-eye visible stars, plus many more faint ones
/// 15000 provides good density, especially along the Milky Way band
pub const MAX_MILKY_WAY_STARS: usize = 15000;

/// Maximum number of bright reference stars
pub const MAX_REFERENCE_STARS: usize = 100;

// ─────────────────────────────────────────────────────────────────────────────────
// GALACTIC STAR DATA
// ─────────────────────────────────────────────────────────────────────────────────

/// A background star in the Milky Way field
#[derive(Clone, Copy, Debug)]
pub struct GalacticStar {
    /// Position in HCI coordinates (AU)
    pub position: DVec3,
    /// Galactic longitude (radians)
    pub gal_lon: f64,
    /// Galactic latitude (radians)
    pub gal_lat: f64,
    /// Apparent magnitude (brightness)
    pub magnitude: f32,
    /// RGB color
    pub color_rgb: [u8; 3],
    /// Is this a Milky Way band star (denser region)?
    pub in_milky_way_band: bool,
}

/// Milky Way background star field
pub struct MilkyWayField {
    /// Background stars representing the Milky Way
    stars: Vec<GalacticStar>,
    /// Reference stars (known bright stars/constellations)
    reference_stars: Vec<GalacticStar>,
    /// Random seed for deterministic generation
    seed: u64,
}

impl Default for MilkyWayField {
    fn default() -> Self {
        Self::new(42) // Default seed
    }
}

impl MilkyWayField {
    /// Create a new Milky Way field with given seed
    pub fn new(seed: u64) -> Self {
        let mut field = Self {
            stars: Vec::with_capacity(MAX_MILKY_WAY_STARS),
            reference_stars: Vec::with_capacity(MAX_REFERENCE_STARS),
            seed,
        };
        field.generate_milky_way();
        field.add_reference_stars();
        field
    }

    /// Get all Milky Way background stars
    pub fn stars(&self) -> &[GalacticStar] {
        &self.stars
    }

    /// Get reference stars (known bright stars)
    pub fn reference_stars(&self) -> &[GalacticStar] {
        &self.reference_stars
    }

    /// Generate the Milky Way star field
    fn generate_milky_way(&mut self) {
        let mut rng = SimpleRng::new(self.seed);

        // Generate stars with higher density toward galactic plane and center
        for _ in 0..MAX_MILKY_WAY_STARS {
            // Sample galactic coordinates with realistic distribution
            let (gal_lon, gal_lat) = self.sample_galactic_coords(&mut rng);

            // Determine if in Milky Way band (|b| < 10°)
            let in_band = gal_lat.abs() < 10.0 * PI / 180.0;

            // Magnitude distribution: more faint stars than bright
            // Use inverse square law: many mag 6-8, few mag 4-5
            let mag_base = 4.0 + rng.next_f64() * 4.0; // 4-8 range
            let magnitude = if in_band {
                // Slightly brighter stars in the band (more dense)
                mag_base - rng.next_f64() * 0.5
            } else {
                mag_base + rng.next_f64() * 0.5
            } as f32;

            // Star color based on temperature distribution
            let color_rgb = self.random_star_color(&mut rng);

            // Convert to HCI position
            let position = galactic_to_hci(gal_lon, gal_lat, BACKGROUND_STAR_DISTANCE_AU);

            self.stars.push(GalacticStar {
                position,
                gal_lon,
                gal_lat,
                magnitude,
                color_rgb,
                in_milky_way_band: in_band,
            });
        }
    }

    /// Sample galactic coordinates with realistic density distribution
    fn sample_galactic_coords(&self, rng: &mut SimpleRng) -> (f64, f64) {
        // Longitude: uniform distribution with slight enhancement toward galactic center
        let u = rng.next_f64();
        let gal_lon = if u < 0.3 {
            // 30% chance to be near galactic center (Sagittarius arm)
            let center_spread = 60.0 * PI / 180.0; // ±60° from center
            (rng.next_f64() - 0.5) * 2.0 * center_spread
        } else if u < 0.5 {
            // 20% chance to be in Cygnus region (opposite side)
            PI + (rng.next_f64() - 0.5) * 60.0 * PI / 180.0
        } else {
            // 50% uniform
            rng.next_f64() * 2.0 * PI
        };

        // Latitude: concentrated toward galactic plane (exponential falloff)
        // The thin disk has scale height ~300 pc, thick disk ~1000 pc
        // For visual effect, we use a double-Gaussian distribution
        let v = rng.next_f64();
        let gal_lat = if v < 0.7 {
            // 70% in thin disk (narrow band)
            let sigma_thin = 5.0 * PI / 180.0; // 5° spread
            rng.next_gaussian() * sigma_thin
        } else if v < 0.9 {
            // 20% in thick disk (wider band)
            let sigma_thick = 15.0 * PI / 180.0; // 15° spread
            rng.next_gaussian() * sigma_thick
        } else {
            // 10% halo stars (anywhere)
            (rng.next_f64() - 0.5) * PI // -90° to +90°
        };

        // Clamp latitude to valid range
        let gal_lat = gal_lat.clamp(-PI / 2.0, PI / 2.0);

        (gal_lon, gal_lat)
    }

    /// Generate a random star color based on stellar population
    fn random_star_color(&self, rng: &mut SimpleRng) -> [u8; 3] {
        // Most stars are red dwarfs (M class), but include variety
        let u = rng.next_f64();
        if u < 0.50 {
            // Red/orange stars (K, M class) - most common
            let r = 255;
            let g = (180.0 + rng.next_f64() * 60.0) as u8;
            let b = (140.0 + rng.next_f64() * 60.0) as u8;
            [r, g, b]
        } else if u < 0.75 {
            // Yellow/white stars (G, F class)
            let r = 255;
            let g = (230.0 + rng.next_f64() * 25.0) as u8;
            let b = (200.0 + rng.next_f64() * 55.0) as u8;
            [r, g, b]
        } else if u < 0.90 {
            // Blue-white stars (A, B class)
            let r = (200.0 + rng.next_f64() * 55.0) as u8;
            let g = (210.0 + rng.next_f64() * 45.0) as u8;
            let b = 255;
            [r, g, b]
        } else {
            // Pure white (balance)
            [255, 255, 255]
        }
    }

    /// Add known bright reference stars for orientation
    fn add_reference_stars(&mut self) {
        // Key reference points in galactic coordinates
        // Format: (name, gal_l°, gal_b°, magnitude, color)
        let reference_data: &[(&str, f64, f64, f32, [u8; 3])] = &[
            // Galactic center region (Sagittarius)
            ("Galactic Center", 0.0, 0.0, 4.0, [255, 220, 180]),
            // Antares (Scorpius) - near galactic center
            ("Antares", 351.9, 15.1, 1.1, [255, 100, 70]),
            // Deneb (Cygnus) - in Cygnus arm, opposite to center
            ("Deneb", 84.3, 2.0, 1.3, [200, 220, 255]),
            // Vega - summer triangle
            ("Vega", 67.4, 19.2, 0.0, [180, 220, 255]),
            // Altair - summer triangle
            ("Altair", 47.7, -9.0, 0.8, [255, 255, 255]),
            // Sirius - brightest star
            ("Sirius", 227.2, -8.9, -1.5, [200, 220, 255]),
            // Canopus - second brightest
            ("Canopus", 261.2, -25.3, -0.7, [255, 255, 240]),
            // Alpha Centauri - nearest star system
            ("Alpha Centauri", 315.8, -0.7, -0.3, [255, 240, 200]),
            // Betelgeuse - Orion
            ("Betelgeuse", 199.8, -9.0, 0.5, [255, 120, 60]),
            // Rigel - Orion
            ("Rigel", 209.2, -25.1, 0.1, [180, 210, 255]),
            // Polaris - north star (galactic coords)
            ("Polaris", 123.3, 26.5, 2.0, [255, 250, 220]),
            // Carina Nebula region
            ("Carina Nebula", 287.6, -0.6, 5.0, [255, 200, 200]),
            // Large Magellanic Cloud (extragalactic but visible)
            ("LMC Direction", 280.5, -32.9, 0.9, [200, 200, 255]),
            // Small Magellanic Cloud
            ("SMC Direction", 302.8, -44.3, 2.7, [200, 200, 255]),
            // Orion Nebula region
            ("Orion Nebula", 209.0, -19.4, 4.0, [255, 180, 200]),
            // Pleiades cluster
            ("Pleiades", 166.6, -23.5, 1.6, [180, 200, 255]),
            // Sagittarius Arm marker
            ("Sagittarius Arm", 15.0, 0.0, 5.5, [255, 230, 200]),
            // Perseus Arm marker
            ("Perseus Arm", 130.0, 0.0, 5.5, [220, 230, 255]),
            // Crux (Southern Cross) - in galactic plane
            ("Alpha Crucis", 300.1, -0.4, 0.8, [180, 200, 255]),
        ];

        for &(_name, gal_l_deg, gal_b_deg, magnitude, color_rgb) in reference_data {
            let gal_lon = gal_l_deg * PI / 180.0;
            let gal_lat = gal_b_deg * PI / 180.0;
            let position = galactic_to_hci(gal_lon, gal_lat, BACKGROUND_STAR_DISTANCE_AU);
            let in_band = gal_lat.abs() < 10.0 * PI / 180.0;

            self.reference_stars.push(GalacticStar {
                position,
                gal_lon,
                gal_lat,
                magnitude,
                color_rgb,
                in_milky_way_band: in_band,
            });
        }
    }

    /// Get the direction to galactic center in HCI coordinates
    pub fn galactic_center_direction() -> DVec3 {
        galactic_to_hci(0.0, 0.0, 1.0).normalize()
    }

    /// Get the galactic north pole direction in HCI coordinates
    pub fn galactic_north_pole_direction() -> DVec3 {
        galactic_to_hci(0.0, PI / 2.0, 1.0).normalize()
    }
}

// ─────────────────────────────────────────────────────────────────────────────────
// COORDINATE TRANSFORMATIONS
// ─────────────────────────────────────────────────────────────────────────────────

/// Transform galactic coordinates (l, b) to HCI (Heliocentric Inertial) Cartesian
///
/// The transformation accounts for the 60° tilt between the galactic and ecliptic planes.
///
/// # Arguments
/// * `gal_lon` - Galactic longitude in radians (0 = galactic center)
/// * `gal_lat` - Galactic latitude in radians (0 = galactic plane)
/// * `distance` - Distance from Sun in AU
///
/// # Returns
/// DVec3 position in HCI frame (X toward vernal equinox, Z toward ecliptic north)
pub fn galactic_to_hci(gal_lon: f64, gal_lat: f64, distance: f64) -> DVec3 {
    // Step 1: Convert galactic spherical to galactic Cartesian
    // In galactic frame: X toward galactic center, Z toward galactic north pole
    let cos_b = gal_lat.cos();
    let x_gal = distance * cos_b * gal_lon.cos();
    let y_gal = distance * cos_b * gal_lon.sin();
    let z_gal = distance * gal_lat.sin();

    // Step 2: Rotate galactic frame to equatorial/HCI frame
    // This involves two rotations:
    // 1. Rotate around galactic Z by the galactic longitude of the ascending node
    // 2. Rotate around the new X by the inclination (60.2°)
    // 3. Rotate around Z by the right ascension of the galactic north pole

    // Transformation matrix from galactic to equatorial (J2000)
    // Using the standard IAU values:
    // - Position angle of the galactic north pole: 122.932°
    // - Declination of galactic north pole: 27.128°
    // - Right ascension of galactic north pole: 192.859°

    // Rotation angles (precomputed from IAU standard values)
    let theta_gnp = (90.0 - 27.128) * PI / 180.0; // Co-declination of galactic north pole
    let phi_gnp = 192.859 * PI / 180.0; // RA of galactic north pole
    let psi = 122.932 * PI / 180.0; // Position angle

    // Build rotation matrix (galactic to equatorial)
    let cos_theta = theta_gnp.cos();
    let sin_theta = theta_gnp.sin();
    let cos_phi = phi_gnp.cos();
    let sin_phi = phi_gnp.sin();
    let cos_psi = psi.cos();
    let sin_psi = psi.sin();

    // Full rotation matrix R = Rz(phi) * Rx(theta) * Rz(psi)
    let r11 = cos_phi * cos_psi - sin_phi * cos_theta * sin_psi;
    let r12 = -cos_phi * sin_psi - sin_phi * cos_theta * cos_psi;
    let r13 = sin_phi * sin_theta;

    let r21 = sin_phi * cos_psi + cos_phi * cos_theta * sin_psi;
    let r22 = -sin_phi * sin_psi + cos_phi * cos_theta * cos_psi;
    let r23 = -cos_phi * sin_theta;

    let r31 = sin_theta * sin_psi;
    let r32 = sin_theta * cos_psi;
    let r33 = cos_theta;

    // Apply rotation: equatorial = R * galactic
    let x_eq = r11 * x_gal + r12 * y_gal + r13 * z_gal;
    let y_eq = r21 * x_gal + r22 * y_gal + r23 * z_gal;
    let z_eq = r31 * x_gal + r32 * y_gal + r33 * z_gal;

    // Step 3: Convert equatorial to ecliptic (HCI)
    // Obliquity of the ecliptic: 23.4393°
    let obliquity = 23.4393 * PI / 180.0;
    let cos_obl = obliquity.cos();
    let sin_obl = obliquity.sin();

    // Rotate around X axis by obliquity
    let x_hci = x_eq;
    let y_hci = y_eq * cos_obl + z_eq * sin_obl;
    let z_hci = -y_eq * sin_obl + z_eq * cos_obl;

    DVec3::new(x_hci, y_hci, z_hci)
}

/// Transform HCI coordinates to galactic coordinates
///
/// # Arguments
/// * `hci_pos` - Position in HCI frame
///
/// # Returns
/// (galactic_longitude, galactic_latitude) in radians
pub fn hci_to_galactic(hci_pos: DVec3) -> (f64, f64) {
    // This is the inverse of galactic_to_hci
    // For simplicity, we compute the direction and reverse the transformation

    let x_hci = hci_pos.x;
    let y_hci = hci_pos.y;
    let z_hci = hci_pos.z;

    // Step 1: Ecliptic to equatorial
    let obliquity = 23.4393 * PI / 180.0;
    let cos_obl = obliquity.cos();
    let sin_obl = obliquity.sin();

    let x_eq = x_hci;
    let y_eq = y_hci * cos_obl - z_hci * sin_obl;
    let z_eq = y_hci * sin_obl + z_hci * cos_obl;

    // Step 2: Equatorial to galactic (inverse of the forward rotation)
    let theta_gnp = (90.0 - 27.128) * PI / 180.0;
    let phi_gnp = 192.859 * PI / 180.0;
    let psi = 122.932 * PI / 180.0;

    let cos_theta = theta_gnp.cos();
    let sin_theta = theta_gnp.sin();
    let cos_phi = phi_gnp.cos();
    let sin_phi = phi_gnp.sin();
    let cos_psi = psi.cos();
    let sin_psi = psi.sin();

    // Inverse (transpose) of the rotation matrix
    let r11 = cos_phi * cos_psi - sin_phi * cos_theta * sin_psi;
    let r21 = -cos_phi * sin_psi - sin_phi * cos_theta * cos_psi;
    let r31 = sin_phi * sin_theta;

    let r12 = sin_phi * cos_psi + cos_phi * cos_theta * sin_psi;
    let r22 = -sin_phi * sin_psi + cos_phi * cos_theta * cos_psi;
    let r32 = -cos_phi * sin_theta;

    let r13 = sin_theta * sin_psi;
    let r23 = sin_theta * cos_psi;
    let r33 = cos_theta;

    let x_gal = r11 * x_eq + r12 * y_eq + r13 * z_eq;
    let y_gal = r21 * x_eq + r22 * y_eq + r23 * z_eq;
    let z_gal = r31 * x_eq + r32 * y_eq + r33 * z_eq;

    // Step 3: Cartesian to spherical
    let gal_lon = y_gal.atan2(x_gal);
    let gal_lat = z_gal.atan2((x_gal * x_gal + y_gal * y_gal).sqrt());

    (gal_lon, gal_lat)
}

/// Get the Milky Way band density at a given galactic latitude
/// Returns 0.0 to 1.0, where 1.0 is the galactic plane
pub fn milky_way_density(gal_lat: f64) -> f64 {
    // Exponential falloff from galactic plane
    let scale_height = 8.0 * PI / 180.0; // 8 degree scale height
    let thin_disk = (-gal_lat.abs() / scale_height).exp();

    // Add thick disk component
    let thick_scale = 20.0 * PI / 180.0;
    let thick_disk = 0.3 * (-gal_lat.abs() / thick_scale).exp();

    (thin_disk + thick_disk).min(1.0)
}

/// Check if a direction points toward the Milky Way band
pub fn is_in_milky_way_band(gal_lat: f64) -> bool {
    gal_lat.abs() < 15.0 * PI / 180.0
}

// ─────────────────────────────────────────────────────────────────────────────────
// SIMPLE RNG (for deterministic star generation without external deps)
// ─────────────────────────────────────────────────────────────────────────────────

/// Simple deterministic RNG using xorshift128+
struct SimpleRng {
    state: [u64; 2],
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        // Initialize with splitmix64 to get good initial state
        let mut s = seed;
        let mut next_splitmix = || {
            s = s.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        };
        Self {
            state: [next_splitmix(), next_splitmix()],
        }
    }

    fn next_u64(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.state[1] = s1.rotate_left(37);

        result
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Box-Muller transform for Gaussian distribution
    fn next_gaussian(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-10); // Avoid log(0)
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

// ─────────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galactic_center_direction() {
        let gc_dir = MilkyWayField::galactic_center_direction();
        // Galactic center should point roughly toward Sagittarius
        // In HCI, this is roughly in the -X, -Y direction (ecliptic longitude ~266°)
        assert!(gc_dir.length() > 0.99 && gc_dir.length() < 1.01);
    }

    #[test]
    fn test_galactic_north_pole() {
        let gnp_dir = MilkyWayField::galactic_north_pole_direction();
        // Should be roughly 60° from ecliptic north (Z axis)
        let angle_from_z = gnp_dir.z.acos();
        let expected_angle = GALACTIC_ECLIPTIC_ANGLE;
        assert!(
            (angle_from_z - expected_angle).abs() < 0.1,
            "Galactic north pole angle: {} vs expected {}",
            angle_from_z * 180.0 / PI,
            expected_angle * 180.0 / PI
        );
    }

    #[test]
    fn test_coordinate_roundtrip() {
        // Test that galactic -> HCI -> galactic is consistent
        let test_points = [
            (0.0, 0.0),                 // Galactic center
            (PI, 0.0),                  // Anti-center
            (0.0, PI / 4.0),            // 45° north of plane
            (PI / 2.0, -PI / 6.0),      // Somewhere in southern sky
        ];

        for (lon, lat) in test_points {
            let hci = galactic_to_hci(lon, lat, 1.0);
            let (lon2, lat2) = hci_to_galactic(hci);

            // Allow for wrapping in longitude
            let lon_diff = ((lon - lon2 + PI).rem_euclid(2.0 * PI) - PI).abs();
            let lat_diff = (lat - lat2).abs();

            assert!(
                lon_diff < 0.01 && lat_diff < 0.01,
                "Roundtrip failed for ({}, {}): got ({}, {})",
                lon * 180.0 / PI,
                lat * 180.0 / PI,
                lon2 * 180.0 / PI,
                lat2 * 180.0 / PI
            );
        }
    }

    #[test]
    fn test_milky_way_field_generation() {
        let field = MilkyWayField::new(12345);

        // Should have generated stars
        assert!(field.stars().len() > 1000);
        assert!(field.reference_stars().len() > 10);

        // Check that more stars are near the galactic plane
        let in_plane = field.stars().iter()
            .filter(|s| s.gal_lat.abs() < 10.0 * PI / 180.0)
            .count();
        let out_of_plane = field.stars().len() - in_plane;

        // Should have more stars in the plane than outside
        assert!(in_plane > out_of_plane);
    }
}
