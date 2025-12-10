/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * FILE: generate_workspace_data.js | SCRIPTS/generate_workspace_data.js
 * PURPOSE: Parses Cargo workspace and generates dependency graph data for architecture visualization
 * MODIFIED: 2025-12-09
 * ═══════════════════════════════════════════════════════════════════════════════
 */

const fs = require('fs');
const path = require('path');

const ROOT_DIR = path.resolve(__dirname, '..');
const CARGO_TOML_PATH = path.join(ROOT_DIR, 'Cargo.toml');
const OUTPUT_FILE = path.join(ROOT_DIR, 'ARCH', 'src', 'workspace_data.json');

// --- Helper Functions ---

/**
 * Poor man's TOML parser for [workspace] members and [package] name/dependencies.
 * Regex-based is fragile but sufficient for this specific codebase's standard formatting.
 */
function parseCargoToml(content) {
    const lines = content.split('\n');
    let section = '';
    const members = [];
    const dependencies = [];
    let packageName = '';

    for (let line of lines) {
        line = line.trim();
        if (!line || line.startsWith('#')) continue;

        if (line.startsWith('[')) {
            section = line.replace(/[\[\]]/g, '');
            continue;
        }

        if (section === 'workspace') {
            if (line.startsWith('members')) {
                // Multiline array handling is tricky with regex, assuming standard format or line-by-line in this repo
                // This parser assumes members are listed inside [] or on subsequent lines if [] is open.
                // NOTE: The repo uses:
                // members = [
                //    "DNA",
                //    ...
                // ]
            }
        }

        // Simplified extraction logic below
    }
    return { members, packageName, dependencies };
}

// Robust TOML extraction for members array
function getWorkspaceMembers(content) {
    // Extract everything between members = [ and ]
    const match = content.match(/members\s*=\s*\[([\s\S]*?)\]/);
    if (!match) return [];

    const block = match[1];
    const members = [];

    // Find all strings in quotes. This assumes no escaped quotes for simplicity, 
    // and that comments don't contain "quoted pathways" which is true for this repo.
    const re = /"([^"]+)"/g;
    let m;
    while ((m = re.exec(block)) !== null) {
        members.push(m[1]);
    }
    return members;
}

function getPackageName(content) {
    const match = content.match(/\[package\][\s\S]*?name\s*=\s*"([^"]+)"/);
    return match ? match[1] : null;
}

function getDependencies(content) {
    const deps = [];
    let inDeps = false;

    const lines = content.split('\n');
    for (let line of lines) {
        line = line.trim();
        if (line.startsWith('[dependencies]')) {
            inDeps = true;
            continue;
        }
        if (line.startsWith('[') && !line.startsWith('[dependencies.')) {
            inDeps = false; // Exited dependencies section
        }

        if (inDeps) {
            // Handle multiple formats:
            // 1. name = { ... }
            // 2. name = "..."
            // 3. name.workspace = true (workspace dependency)
            // 4. name = { path = "...", ... }
            const matchStandard = line.match(/^([a-zA-Z0-9_\-]+)\s*=/);
            const matchWorkspace = line.match(/^([a-zA-Z0-9_\-]+)\.workspace\s*=/);

            if (matchWorkspace) {
                deps.push(matchWorkspace[1]);
            } else if (matchStandard) {
                deps.push(matchStandard[1]);
            }
        }
    }

    return deps;
}

function expandGlob(memberPath) {
    if (memberPath.endsWith('/*')) {
        const parentDir = path.join(ROOT_DIR, memberPath.slice(0, -2));
        if (fs.existsSync(parentDir)) {
            return fs.readdirSync(parentDir)
                .filter(f => fs.statSync(path.join(parentDir, f)).isDirectory())
                .map(f => path.join(memberPath.slice(0, -2), f));
        }
        return [];
    }
    return [memberPath];
}

// --- Main Logic ---

function main() {
    console.log('Reading root Cargo.toml...');
    const rootCargo = fs.readFileSync(CARGO_TOML_PATH, 'utf-8');
    const rawMembers = getWorkspaceMembers(rootCargo);

    let members = [];
    for (const m of rawMembers) {
        // Expand wildcards like SIMULATION/*
        const expanded = expandGlob(m);
        members = members.concat(expanded);
    }

    console.log(`Found ${members.length} workspace members.`);

    const crates = [];
    const crateNames = new Set();

    // Pass 1: Gather info
    for (const memberPath of members) {
        const fullPath = path.join(ROOT_DIR, memberPath);
        const cargoPath = path.join(fullPath, 'Cargo.toml');

        if (!fs.existsSync(cargoPath)) {
            console.warn(`Warning: No Cargo.toml found at ${memberPath}, skipping.`);
            continue;
        }

        const cargoContent = fs.readFileSync(cargoPath, 'utf-8');
        const name = getPackageName(cargoContent);

        if (!name) {
            console.warn(`Warning: Could not parse package name in ${cargoPath}, skipping.`);
            continue;
        }

        const deps = getDependencies(cargoContent);

        // Determine Layer
        let layer = 'Project'; // Default
        if (memberPath.startsWith('DNA')) {
            layer = 'Dna';
        } else if (memberPath.includes('/CORE/')) {
            layer = 'Core';
        } else if (memberPath.startsWith('TOOLS/') || memberPath.startsWith('SIMULATION/') || memberPath.startsWith('LEARN/')) {
            layer = 'Tool';
        } else if (memberPath === 'WELCOME' || memberPath === 'HELIOS' || memberPath === 'BLOG' || memberPath === 'ARCH') {
            layer = 'Project';
        }

        crates.push({
            name,
            path: memberPath,
            layer,
            dependencies: deps, // Will filter in Pass 2
            _rawPath: memberPath
        });

        crateNames.add(name);
    }

    // Pass 2: Filter dependencies to only include workspace members
    for (const crate of crates) {
        // We filter AND we need to handle specific cases where dependency name != crate name?
        // In Rust, dep name equals package name usually.
        // But internal deps might be re-mapped? Not common in this workspace.
        crate.dependencies = crate.dependencies.filter(d => crateNames.has(d));
    }

    // Sort crates for consistent output (and so 'dna' is likely early)
    // Dependencies sort?
    // ARCH/src/workspace_data.json seems partially sorted or arbitrary.
    // We'll sort by path or name for determinism.
    // Actually, ARCH/src/lib.rs doesn't care about order.

    // Manual override: Ensure 'dna' appears first or similar? Not strictly needed.

    const output = {
        crates: crates.map(c => ({
            name: c.name,
            path: c.path,
            layer: c.layer,
            dependencies: c.dependencies.sort()
        }))
    };

    fs.writeFileSync(OUTPUT_FILE, JSON.stringify(output, null, 2));
    console.log(`Generated workspace_data.json at ${OUTPUT_FILE} with ${crates.length} crates.`);
}

main();
