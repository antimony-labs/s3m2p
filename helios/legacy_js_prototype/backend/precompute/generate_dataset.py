#!/usr/bin/env python3
"""
Heliosphere Dataset Precomputation Pipeline
Generates Zarr datasets for Sun-centric heliosphere visualization
"""

import numpy as np
import json
from pathlib import Path
from typing import Dict, List, Tuple
from dataclasses import dataclass, asdict
from datetime import datetime

# Optional: zarr for chunked storage (install via: pip install zarr)
try:
    import zarr
    HAS_ZARR = True
except ImportError:
    HAS_ZARR = False
    print("Warning: zarr not installed. Will generate JSON fallback only.")


@dataclass
class HeliosphereParameters:
    """Parameters for a single epoch"""
    R_HP_nose: float  # AU
    R_TS_over_HP: float  # ratio
    nose_vec: Tuple[float, float, float]  # unit vector in HEE_J2000
    ISM_rho: float  # particles/cm³
    ISM_T: float  # K
    ISM_B: float  # nT
    SW_Mdot: float  # Solar mass / year (proxy)
    SW_v: float  # km/s
    morphology: str  # 'cometary' | 'croissant' | 'bubble'
    shape_params: List[float]


class SolarEvolutionModel:
    """Simplified solar evolution model"""
    
    def __init__(self):
        # Solar lifetime phases (approximate)
        self.ZAMS_age_Gyr = 0.0
        self.present_age_Gyr = 4.6
        self.TAMS_age_Gyr = 10.0
        self.RGB_peak_Gyr = 12.0
        self.AGB_peak_Gyr = 12.3
        self.PN_onset_Gyr = 12.4
        self.WD_age_Gyr = 13.0
    
    def solar_wind_properties(self, age_Gyr: float) -> Tuple[float, float]:
        """
        Returns (mass_loss_rate_factor, wind_speed_km_s)
        Normalized to present day = 1.0
        """
        age = age_Gyr
        
        # Main sequence (ZAMS to TAMS): relatively stable
        if age <= self.TAMS_age_Gyr:
            # Young Sun had faster rotation -> stronger wind
            if age < 1.0:
                mdot = 100.0  # Up to 100x present
            elif age < self.present_age_Gyr:
                mdot = 10.0 ** (1.5 - 0.3 * age)  # Decay
            else:
                mdot = 1.0  # Present day
            
            v_sw = 400.0  # km/s (relatively constant)
        
        # Red giant branch (RGB): mass loss increases
        elif age <= self.RGB_peak_Gyr:
            mdot = 1.0 + 1000.0 * (age - self.TAMS_age_Gyr) / (self.RGB_peak_Gyr - self.TAMS_age_Gyr)
            v_sw = 400.0 + 200.0 * (age - self.TAMS_age_Gyr) / (self.RGB_peak_Gyr - self.TAMS_age_Gyr)
        
        # Asymptotic giant branch (AGB): extreme mass loss
        elif age <= self.AGB_peak_Gyr:
            mdot = 1000.0 + 10000.0 * (age - self.RGB_peak_Gyr) / (self.AGB_peak_Gyr - self.RGB_peak_Gyr)
            v_sw = 600.0 + 400.0 * (age - self.RGB_peak_Gyr) / (self.AGB_peak_Gyr - self.RGB_peak_Gyr)
        
        # Planetary nebula: rapid transition
        elif age <= self.PN_onset_Gyr:
            mdot = 0.1  # Mass loss stops
            v_sw = 1000.0  # Fast PN winds
        
        # White dwarf: minimal wind
        else:
            mdot = 1e-6
            v_sw = 100.0
        
        return mdot, v_sw


class ISMModel:
    """Interstellar medium properties"""
    
    @staticmethod
    def get_properties() -> Dict[str, float]:
        """
        Returns ISM properties (assumed constant for simplicity)
        In reality, the Sun moves through varying ISM
        """
        return {
            'rho': 0.1,  # particles/cm³ (local cloud)
            'T': 6300,  # K
            'B': 0.3,  # nT
            'v': 26.3,  # km/s (IBEX measurement)
        }
    
    @staticmethod
    def inflow_direction_HEE() -> Tuple[float, float, float]:
        """
        ISM inflow direction in HEE_J2000 frame
        From galactic l=255.4°, b=5.2° (IBEX)
        """
        # Simplified conversion (full version needs proper transform)
        lon_rad = np.radians(255.4)
        lat_rad = np.radians(5.2)
        
        # Direction towards Sun (inflow)
        x = -np.cos(lat_rad) * np.cos(lon_rad)
        y = -np.cos(lat_rad) * np.sin(lon_rad)
        z = -np.sin(lat_rad)
        
        # Normalize
        norm = np.sqrt(x**2 + y**2 + z**2)
        return (x/norm, y/norm, z/norm)


def compute_heliopause_radius(SW_Mdot: float, SW_v: float, ISM_rho: float, ISM_v: float) -> float:
    """
    Pressure balance estimate for heliopause nose radius
    R_HP ∝ sqrt(SW_ram / ISM_ram)
    """
    # Normalize to present-day values
    SW_ram = SW_Mdot * SW_v**2  # Proportional to ram pressure
    ISM_ram = ISM_rho * ISM_v**2
    
    # Present-day R_HP_nose ≈ 121 AU (Voyager 1 crossing)
    R_HP_present = 121.0
    
    # Scale
    R_HP = R_HP_present * np.sqrt(SW_ram / ISM_ram)
    
    # Clamp to reasonable range
    return np.clip(R_HP, 10.0, 2000.0)


def determine_morphology(age_Gyr: float) -> str:
    """Determine heliosphere morphology based on solar age"""
    if age_Gyr < 10.0:
        return 'cometary'
    elif age_Gyr < 12.0:
        return 'croissant'  # RGB phase: weaker magnetic field
    else:
        return 'bubble'  # Post-MS: nearly spherical


def generate_time_axis() -> np.ndarray:
    """
    Generate non-uniform time axis
    Dense sampling during rapid evolution phases
    """
    solar = SolarEvolutionModel()
    
    epochs = []
    
    # Main sequence: 0 to 10 Gyr, Δt ≈ 0.5 Myr
    epochs.extend(np.arange(0.0, solar.TAMS_age_Gyr, 0.5e-3))
    
    # RGB: 10 to 12 Gyr, Δt ≈ 10 kyr
    epochs.extend(np.arange(solar.TAMS_age_Gyr, solar.RGB_peak_Gyr, 10e-6))
    
    # AGB: 12 to 12.4 Gyr, Δt ≈ 1 kyr
    epochs.extend(np.arange(solar.RGB_peak_Gyr, solar.PN_onset_Gyr, 1e-6))
    
    # PN/WD: 12.4 to 13 Gyr, Δt ≈ 50 kyr
    epochs.extend(np.arange(solar.PN_onset_Gyr, solar.WD_age_Gyr, 50e-6))
    
    return np.array(sorted(set(epochs)))  # Remove duplicates and sort


def generate_heliosphere_parameters(epochs: np.ndarray) -> List[HeliosphereParameters]:
    """Generate heliosphere parameters for all epochs"""
    solar = SolarEvolutionModel()
    ism = ISMModel()
    
    ism_props = ism.get_properties()
    nose_vec = ism.inflow_direction_HEE()
    
    parameters = []
    
    for age_Gyr in epochs:
        # Solar wind properties
        SW_Mdot, SW_v = solar.solar_wind_properties(age_Gyr)
        
        # Heliopause radius
        R_HP_nose = compute_heliopause_radius(
            SW_Mdot, SW_v, 
            ism_props['rho'], ism_props['v']
        )
        
        # TS/HP ratio (relatively constant, slight variation)
        R_TS_over_HP = 0.75 + 0.05 * np.sin(age_Gyr * 10.0)
        
        # Morphology
        morphology = determine_morphology(age_Gyr)
        
        # Shape parameters (morphology-dependent)
        if morphology == 'cometary':
            shape_params = [1.0, 2.5, 0.5]
        elif morphology == 'croissant':
            shape_params = [1.5, 0.7, 0.3]
        else:  # bubble
            shape_params = [0.1]
        
        params = HeliosphereParameters(
            R_HP_nose=R_HP_nose,
            R_TS_over_HP=R_TS_over_HP,
            nose_vec=nose_vec,
            ISM_rho=ism_props['rho'],
            ISM_T=ism_props['T'],
            ISM_B=ism_props['B'],
            SW_Mdot=SW_Mdot,
            SW_v=SW_v,
            morphology=morphology,
            shape_params=shape_params
        )
        
        parameters.append(params)
    
    return parameters


def save_dataset_json(output_dir: Path, epochs: np.ndarray, parameters: List[HeliosphereParameters]):
    """Save dataset as JSON files (fallback if zarr unavailable)"""
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Metadata
    metadata = {
        'version': '1.0.0',
        'created': datetime.now().isoformat(),
        'units': {
            'distance': 'AU',
            'velocity': 'km/s',
            'time': 'MyrSinceZAMS'
        },
        'provenance': {
            'solar_model': 'SimplifiedEvolution',
            'ism_model': 'ConstantLocalCloud',
            'generator_version': '1.0.0'
        },
        'time_axis': {
            'n_epochs': len(epochs),
            't_min': float(epochs[0]),
            't_max': float(epochs[-1]),
            'epoch_file': 'time/epochs.json'
        }
    }
    
    with open(output_dir / 'meta.json', 'w') as f:
        json.dump(metadata, f, indent=2)
    
    # Epochs
    (output_dir / 'time').mkdir(exist_ok=True)
    with open(output_dir / 'time' / 'epochs.json', 'w') as f:
        json.dump(epochs.tolist(), f)
    
    # Heliosphere parameters (individual files per epoch)
    (output_dir / 'heliosphere').mkdir(exist_ok=True)
    for i, params in enumerate(parameters):
        with open(output_dir / 'heliosphere' / f'epoch_{i:06d}.json', 'w') as f:
            json.dump(asdict(params), f, indent=2)
    
    print(f"Saved {len(parameters)} epochs to {output_dir}")


def save_dataset_zarr(output_dir: Path, epochs: np.ndarray, parameters: List[HeliosphereParameters]):
    """Save dataset as Zarr arrays (preferred, chunked format)"""
    if not HAS_ZARR:
        print("Zarr not available, skipping...")
        return
    
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Create Zarr group
    store = zarr.DirectoryStore(str(output_dir / 'zarr'))
    root = zarr.group(store=store, overwrite=True)
    
    # Time axis
    time_group = root.create_group('time')
    time_group.create_dataset('epochs', data=epochs, chunks=(1024,), dtype='float64')
    
    # Heliosphere parameters (columnar storage)
    n = len(parameters)
    helio_group = root.create_group('heliosphere')
    
    R_HP_nose = np.array([p.R_HP_nose for p in parameters], dtype='float32')
    R_TS_over_HP = np.array([p.R_TS_over_HP for p in parameters], dtype='float32')
    nose_vecs = np.array([p.nose_vec for p in parameters], dtype='float32')
    ISM_rho = np.array([p.ISM_rho for p in parameters], dtype='float32')
    ISM_T = np.array([p.ISM_T for p in parameters], dtype='float32')
    ISM_B = np.array([p.ISM_B for p in parameters], dtype='float32')
    SW_Mdot = np.array([p.SW_Mdot for p in parameters], dtype='float32')
    SW_v = np.array([p.SW_v for p in parameters], dtype='float32')
    
    helio_group.create_dataset('R_HP_nose', data=R_HP_nose, chunks=(1024,))
    helio_group.create_dataset('R_TS_over_HP', data=R_TS_over_HP, chunks=(1024,))
    helio_group.create_dataset('nose_vec', data=nose_vecs, chunks=(1024, 3))
    helio_group.create_dataset('ISM_rho', data=ISM_rho, chunks=(1024,))
    helio_group.create_dataset('ISM_T', data=ISM_T, chunks=(1024,))
    helio_group.create_dataset('ISM_B', data=ISM_B, chunks=(1024,))
    helio_group.create_dataset('SW_Mdot', data=SW_Mdot, chunks=(1024,))
    helio_group.create_dataset('SW_v', data=SW_v, chunks=(1024,))
    
    print(f"Saved Zarr dataset to {output_dir}/zarr")


def main():
    """Main pipeline"""
    print("Generating heliosphere dataset...")
    
    # Generate time axis
    print("  - Generating time axis...")
    epochs = generate_time_axis()
    print(f"    Generated {len(epochs)} epochs")
    
    # Generate parameters
    print("  - Computing heliosphere parameters...")
    parameters = generate_heliosphere_parameters(epochs)
    
    # Output directory
    output_dir = Path(__file__).parent.parent.parent / 'public' / 'dataset'
    
    # Save as JSON (always)
    print("  - Saving JSON fallback...")
    save_dataset_json(output_dir, epochs, parameters)
    
    # Save as Zarr (if available)
    if HAS_ZARR:
        print("  - Saving Zarr dataset...")
        save_dataset_zarr(output_dir, epochs, parameters)
    
    print("Done!")
    print(f"\nDataset statistics:")
    print(f"  Epochs: {len(epochs)}")
    print(f"  Time range: {epochs[0]:.3f} - {epochs[-1]:.3f} Gyr")
    print(f"  R_HP range: {min(p.R_HP_nose for p in parameters):.1f} - {max(p.R_HP_nose for p in parameters):.1f} AU")


if __name__ == '__main__':
    main()

