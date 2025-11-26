#!/usr/bin/env node
/**
 * JavaScript Dataset Generator (no external dependencies)
 * Generates heliosphere parameters for demo purposes
 */

const fs = require('fs');
const path = require('path');

// Solar evolution model (simplified)
class SolarEvolution {
  constructor() {
    this.ZAMS_age_Gyr = 0.0;
    this.present_age_Gyr = 4.6;
    this.TAMS_age_Gyr = 10.0;
    this.RGB_peak_Gyr = 12.0;
    this.AGB_peak_Gyr = 12.3;
    this.PN_onset_Gyr = 12.4;
    this.WD_age_Gyr = 13.0;
  }

  getSolarWindProperties(age_Gyr) {
    const age = age_Gyr;
    let mdot, v_sw;

    if (age <= this.TAMS_age_Gyr) {
      // Main sequence
      if (age < 1.0) {
        mdot = 100.0;
      } else if (age < this.present_age_Gyr) {
        mdot = Math.pow(10, 1.5 - 0.3 * age);
      } else {
        mdot = 1.0;
      }
      v_sw = 400.0;
    } else if (age <= this.RGB_peak_Gyr) {
      // RGB
      const t = (age - this.TAMS_age_Gyr) / (this.RGB_peak_Gyr - this.TAMS_age_Gyr);
      mdot = 1.0 + 1000.0 * t;
      v_sw = 400.0 + 200.0 * t;
    } else if (age <= this.AGB_peak_Gyr) {
      // AGB
      const t = (age - this.RGB_peak_Gyr) / (this.AGB_peak_Gyr - this.RGB_peak_Gyr);
      mdot = 1000.0 + 10000.0 * t;
      v_sw = 600.0 + 400.0 * t;
    } else if (age <= this.PN_onset_Gyr) {
      // PN
      mdot = 0.1;
      v_sw = 1000.0;
    } else {
      // WD
      mdot = 1e-6;
      v_sw = 100.0;
    }

    return { mdot, v_sw };
  }
}

// ISM properties
const ISM = {
  rho: 0.1,
  T: 6300,
  B: 0.3,
  v: 26.3,
};

// ISM inflow direction (from IBEX)
const noseDirection = [-0.93, -0.26, 0.26]; // Simplified HEE_J2000

// Calculate heliopause radius
function computeHeliopauseRadius(SW_Mdot, SW_v, ISM_rho, ISM_v) {
  // Normalize to present-day values
  // Present day: SW_Mdot = 1.0, SW_v = 400 km/s, ISM_rho = 0.1, ISM_v = 26.3 km/s
  const SW_ram_present = 1.0 * 400 * 400; // 160000
  const ISM_ram_present = 0.1 * 26.3 * 26.3; // ~69.17
  const R_HP_present = 121.0; // AU (Voyager 1 crossing)
  
  // Current ram pressures
  const SW_ram = SW_Mdot * SW_v * SW_v;
  const ISM_ram = ISM_rho * ISM_v * ISM_v;
  
  // Scale from present day: R_HP ∝ sqrt(SW_ram / ISM_ram)
  // But normalize so present day = R_HP_present
  const ram_ratio = (SW_ram / ISM_ram) / (SW_ram_present / ISM_ram_present);
  const R_HP = R_HP_present * Math.sqrt(ram_ratio);
  
  return Math.max(10.0, Math.min(2000.0, R_HP));
}

// Determine morphology
function determineMorphology(age_Gyr) {
  if (age_Gyr < 10.0) return 'cometary';
  if (age_Gyr < 12.0) return 'croissant';
  return 'bubble';
}

// Generate time axis
function generateTimeAxis() {
  const epochs = [];
  
  // Main sequence: 0-10 Gyr, Δt ≈ 50 Myr (100 points)
  for (let t = 0; t <= 10.0; t += 0.1) {
    epochs.push(t);
  }
  
  // RGB: 10-12 Gyr, Δt ≈ 10 Myr (200 points)
  for (let t = 10.05; t <= 12.0; t += 0.01) {
    epochs.push(t);
  }
  
  // AGB/PN/WD: 12-13 Gyr, Δt ≈ 5 Myr (200 points)
  for (let t = 12.005; t <= 13.0; t += 0.005) {
    epochs.push(t);
  }
  
  return epochs;
}

// Main generation
function generateDataset() {
  console.log('🌟 Generating heliosphere dataset...');
  
  const outputDir = path.join(__dirname, '..', 'public', 'dataset');
  
  // Create directories
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }
  ['time', 'heliosphere'].forEach(dir => {
    const dirPath = path.join(outputDir, dir);
    if (!fs.existsSync(dirPath)) {
      fs.mkdirSync(dirPath, { recursive: true });
    }
  });
  
  // Generate time axis
  console.log('  📅 Generating time axis...');
  const epochs = generateTimeAxis();
  console.log(`    Generated ${epochs.length} epochs`);
  
  // Solar evolution model
  const solar = new SolarEvolution();
  
  // Generate parameters for each epoch
  console.log('  🌊 Computing heliosphere parameters...');
  const parameters = [];
  
  for (const age_Gyr of epochs) {
    const { mdot, v_sw } = solar.getSolarWindProperties(age_Gyr);
    
    const R_HP_nose = computeHeliopauseRadius(mdot, v_sw, ISM.rho, ISM.v);
    const R_TS_over_HP = 0.75 + 0.05 * Math.sin(age_Gyr * 10.0);
    const morphology = determineMorphology(age_Gyr);
    
    let shape_params;
    if (morphology === 'cometary') {
      shape_params = [1.0, 2.5, 0.5];
    } else if (morphology === 'croissant') {
      shape_params = [1.5, 0.7, 0.3];
    } else {
      shape_params = [0.1];
    }
    
    parameters.push({
      R_HP_nose,
      R_TS_over_HP,
      nose_vec: noseDirection,
      ISM_rho: ISM.rho,
      ISM_T: ISM.T,
      ISM_B: ISM.B,
      SW_Mdot: mdot,
      SW_v: v_sw,
      morphology,
      shape_params,
    });
  }
  
  // Write metadata
  console.log('  💾 Writing metadata...');
  const metadata = {
    version: '1.0.0',
    created: new Date().toISOString(),
    units: {
      distance: 'AU',
      velocity: 'km/s',
      time: 'GyrSinceZAMS',
    },
    provenance: {
      solar_model: 'SimplifiedEvolution',
      ism_model: 'ConstantLocalCloud',
      generator_version: '1.0.0-js',
    },
    time_axis: {
      n_epochs: epochs.length,
      t_min: epochs[0],
      t_max: epochs[epochs.length - 1],
      epoch_file: 'time/epochs.json',
    },
  };
  
  fs.writeFileSync(
    path.join(outputDir, 'meta.json'),
    JSON.stringify(metadata, null, 2)
  );
  
  // Write epochs
  console.log('  💾 Writing epochs...');
  fs.writeFileSync(
    path.join(outputDir, 'time', 'epochs.json'),
    JSON.stringify(epochs)
  );
  
  // Write parameters (individual files per epoch)
  console.log('  💾 Writing heliosphere parameters...');
  parameters.forEach((params, i) => {
    const filename = `epoch_${String(i).padStart(6, '0')}.json`;
    fs.writeFileSync(
      path.join(outputDir, 'heliosphere', filename),
      JSON.stringify(params, null, 2)
    );
  });
  
  console.log('\n✅ Dataset generation complete!');
  console.log(`\n📊 Statistics:`);
  console.log(`  Epochs: ${epochs.length}`);
  console.log(`  Time range: ${epochs[0].toFixed(3)} - ${epochs[epochs.length - 1].toFixed(3)} Gyr`);
  console.log(`  R_HP range: ${Math.min(...parameters.map(p => p.R_HP_nose)).toFixed(1)} - ${Math.max(...parameters.map(p => p.R_HP_nose)).toFixed(1)} AU`);
  console.log(`\n📁 Output: ${outputDir}`);
  console.log(`  Files: ${parameters.length + 2} (meta.json, epochs.json, ${parameters.length} parameter files)`);
  
  // Estimate size
  const totalSize = parameters.reduce((sum, p) => {
    return sum + JSON.stringify(p).length;
  }, 0) + JSON.stringify(metadata).length + JSON.stringify(epochs).length;
  console.log(`  Size: ~${(totalSize / 1024 / 1024).toFixed(2)} MB`);
}

// Run
try {
  generateDataset();
} catch (error) {
  console.error('❌ Error generating dataset:', error);
  process.exit(1);
}

