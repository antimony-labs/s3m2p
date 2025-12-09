# Add Metadata Headers

Add or update metadata headers to source files in the codebase.

## Usage

```
/add-metadata [target]
```

**Arguments:**
- `[target]` - Optional. Can be:
  - A file path: `/add-metadata DNA/src/lib.rs`
  - A directory: `/add-metadata TOOLS/`
  - A glob pattern: `/add-metadata **/*.rs`
  - Omitted: Process current working directory
- `--dry-run` - Preview changes without writing files

## Instructions

### 1. Parse Target

Determine what to process from `$ARGUMENTS`:
- If empty → current working directory (recursive)
- If file path → single file
- If directory → all eligible files recursively
- If glob pattern → expand and process matching files

### 2. Get File List

Use `find` or `glob` to get all candidate files, then **EXCLUDE**:
- Directories: `target/`, `dist/`, `node_modules/`, `.git/`, `.claude/logs/`
- File types: `.json` (cannot have comments), binary files (`.wasm`, `.png`, `.jpg`, `.ico`)
- Files smaller than 10 bytes
- Files starting with `// Generated` or `# Generated`

### 3. For Each File

**a. Read the file content**

**b. Get file modification time:**
```bash
stat -c %Y <file>  # Linux: seconds since epoch
```
Convert to ISO date format (YYYY-MM-DD).

**c. Detect file type and comment syntax:**

| Extension | Comment Style | Line Prefix |
|-----------|---------------|-------------|
| `.rs` | Rust doc comment | `//!` |
| `.js`, `.ts` | JSDoc block | `/** ... */` |
| `.sh` | Shell comment | `#` |
| `.html` | HTML comment | `<!-- ... -->` |
| `.toml` | TOML comment | `#` |
| `.css` | CSS block | `/* ... */` |
| `.md` | YAML frontmatter | `---` |

**d. Check for existing metadata header:**
Look for the marker pattern `═══` in the first 10 lines.
- If found → UPDATE mode (replace existing header block)
- If not found → INSERT mode (prepend new header)

**e. Infer LAYER from path:**

| Path Pattern | Layer Value |
|--------------|-------------|
| `DNA/...` | `DNA (foundation)` |
| `*/CORE/*` | `CORE → {engine_name}` |
| `SIMULATION/...` | `SIMULATION → {project}` |
| `TOOLS/...` | `TOOLS → {tool_name}` |
| `LEARN/...` | `LEARN → {topic}` |
| `WELCOME/...` | `WELCOME (landing)` |
| `HELIOS/...` | `HELIOS (simulation)` |
| `SCRIPTS/...` | `SCRIPTS (automation)` |
| Other | Infer from directory structure |

**f. Generate PURPOSE:**
Analyze the file content and generate a concise (10-15 word) description:
- For Rust: Look at `//!` docs, struct/function names, imports
- For Scripts: Look at usage comments, main logic
- For HTML: Look at `<title>`, main content
- For Config: Describe what the configuration controls

**g. Build the metadata header** using the appropriate format (see templates below).

**h. Write the file:**
- In UPDATE mode: Replace the existing header block
- In INSERT mode: Prepend header (preserve shebang `#!/...` as line 1 if present)

### 4. Report Results

Output a summary table:

```markdown
## Metadata Header Results

### Processed Files
| File | Action | Purpose |
|------|--------|---------|
| DNA/src/lib.rs | UPDATED | Core simulation algorithms |
| TOOLS/PLL/src/lib.rs | INSERTED | PLL design tool entry |

### Skipped Files
| File | Reason |
|------|--------|
| package.json | JSON (no comments) |
| target/... | Generated directory |

### Summary
- **Processed:** N files
- **Updated:** N files
- **Inserted:** N files
- **Skipped:** N files
```

---

## Header Templates

### Rust (.rs)
```rust
//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: {filename} | {relative_path}
//! PURPOSE: {purpose}
//! MODIFIED: {modified_date}
//! LAYER: {layer}
//! ═══════════════════════════════════════════════════════════════════════════════
```

### Shell (.sh)
```bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: {filename} | {relative_path}
# PURPOSE: {purpose}
# MODIFIED: {modified_date}
# ═══════════════════════════════════════════════════════════════════════════════
```
**Note:** If file starts with shebang (`#!/bin/bash`), insert header AFTER the shebang line.

### JavaScript/TypeScript (.js/.ts)
```javascript
/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * FILE: {filename} | {relative_path}
 * PURPOSE: {purpose}
 * MODIFIED: {modified_date}
 * ═══════════════════════════════════════════════════════════════════════════════
 */
```

### HTML (.html)
```html
<!--
═══════════════════════════════════════════════════════════════════════════════
FILE: {filename} | {relative_path}
PURPOSE: {purpose}
MODIFIED: {modified_date}
═══════════════════════════════════════════════════════════════════════════════
-->
```

### TOML (.toml)
```toml
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: {filename} | {relative_path}
# PURPOSE: {purpose}
# MODIFIED: {modified_date}
# ═══════════════════════════════════════════════════════════════════════════════
```

### CSS (.css)
```css
/*
═══════════════════════════════════════════════════════════════════════════════
FILE: {filename} | {relative_path}
PURPOSE: {purpose}
MODIFIED: {modified_date}
═══════════════════════════════════════════════════════════════════════════════
*/
```

### Markdown (.md)
```markdown
---
file: {filename}
path: {relative_path}
purpose: {purpose}
modified: {modified_date}
---
```
**Note:** If YAML frontmatter already exists, merge the metadata fields into it.

---

## Idempotency Rules

1. **Marker detection:** The `═══` pattern (3+ box-drawing equals signs) identifies a metadata header
2. **UPDATE mode:** When marker found in first 10 lines, replace the entire header block (from first `═══` line to last `═══` line)
3. **INSERT mode:** When no marker found, prepend new header
4. **Never duplicate:** Running the command twice should produce identical output

## Edge Cases

| Situation | Handling |
|-----------|----------|
| Empty file | Skip with reason "Empty file" |
| Shebang line | Preserve as line 1, insert header after |
| Existing YAML frontmatter | Merge metadata into existing frontmatter |
| Read-only file | Skip with reason "Permission denied" |
| Very large file | Process normally (header is small) |
