# Update ARCH Data

Regenerate the architecture visualization data files for the ARCH project.

## Usage

```
/update-arch-data [target]
```

**Arguments:**
- `[target]` - Optional. Can be:
  - `workspace` - Regenerate only `workspace_data.json` (crates, dependencies)
  - `docs` - Regenerate only `db.json` (documentation database)
  - Omitted - Regenerate both files

## Instructions

### 1. Determine Target

Parse `$ARGUMENTS` to determine what to regenerate:
- If empty or "all" → regenerate both files
- If "workspace" → only `workspace_data.json`
- If "docs" → only `db.json`

### 2. For workspace_data.json

Run the existing script:
```bash
node /home/curious/S3M2P/SCRIPTS/generate_workspace_data.js
```

This script:
- Parses the root `Cargo.toml` workspace members
- Expands glob patterns (e.g., `SIMULATION/*`)
- Reads each crate's `Cargo.toml` for name and dependencies
- Determines layer based on path:
  - `DNA/*` → "Dna"
  - `*/CORE/*` → "Core"
  - `WELCOME`, `HELIOS`, `BLOG`, `ARCH` → "Project"
  - `TOOLS/*`, `SIMULATION/*`, `LEARN/*` → "Tool"
- Filters dependencies to only include workspace members
- Outputs to `/home/curious/S3M2P/ARCH/src/workspace_data.json`

### 3. For db.json

Run the existing script:
```bash
node /home/curious/S3M2P/SCRIPTS/scan_docs.js
```

This script:
- Scans `DNA`, `TOOLS`, `SIMULATION`, `HELIOS`, `BLOG`, `LEARN` directories
- Extracts purpose from doc comments (`//!`, `///`, `#`, `*`)
- Identifies main function/struct
- Outputs to `/home/curious/S3M2P/ARCH/src/db.json`

### 4. Verify the Output

After regeneration, verify the files are valid JSON:
```bash
node -e "JSON.parse(require('fs').readFileSync('/home/curious/S3M2P/ARCH/src/workspace_data.json'))"
node -e "JSON.parse(require('fs').readFileSync('/home/curious/S3M2P/ARCH/src/db.json'))"
```

### 5. Report Results

Output a summary:

```markdown
## ARCH Data Update Results

### workspace_data.json
- **Crates found:** N
- **Layers:** DNA (n), Core (n), Project (n), Tool (n)
- **Dependencies mapped:** N total

### db.json
- **Files scanned:** N
- **By type:** .rs (n), .md (n), .js (n), .ts (n), .html (n), .css (n), .sh (n)

### Verification
- [x] workspace_data.json is valid JSON
- [x] db.json is valid JSON
```

---

## Data Structures

### workspace_data.json Schema

```json
{
  "crates": [
    {
      "name": "crate-name",
      "path": "RELATIVE/PATH",
      "layer": "Dna|Core|Project|Tool",
      "dependencies": ["dep1", "dep2"]
    }
  ]
}
```

### db.json Schema

```json
{
  "RELATIVE/PATH/file.rs": {
    "path": "RELATIVE/PATH/file.rs",
    "name": "file.rs",
    "purpose": "Extracted description from doc comments",
    "main_function": "fn main()|struct Name|Module/Lib|N/A",
    "type": ".rs"
  }
}
```

---

## Layer Classification Rules

| Path Pattern | Layer |
|--------------|-------|
| `DNA` or `DNA/*` | Dna |
| `*/CORE/*` | Core |
| `WELCOME`, `HELIOS`, `BLOG`, `ARCH` | Project |
| `TOOLS/*`, `SIMULATION/*`, `LEARN/*` | Tool |

---

## Extending the Scripts

### Adding a new top-level directory to scan

Edit `/home/curious/S3M2P/SCRIPTS/scan_docs.js`:
```javascript
const dirsToScan = ['DNA', 'TOOLS', 'SIMULATION', 'HELIOS', 'BLOG', 'LEARN', 'NEW_DIR'];
```

### Adding a new layer type

Edit `/home/curious/S3M2P/SCRIPTS/generate_workspace_data.js` in the layer detection logic:
```javascript
if (memberPath.startsWith('NEW_DIR/')) {
    layer = 'NewLayer';
}
```

Also update `ARCH/src/graph.rs` to add the new `CrateLayer` variant.
