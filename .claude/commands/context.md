# Load Project Context

Automatically detect and load context for the current project.

## Project Detection

Detect project from conversation using these patterns:

| Keyword | Project | Path |
|---------|---------|------|
| "WELCOME", "too.foo", "landing" | WELCOME | WELCOME/ |
| "HELIOS", "helios", "solar", "stars" | HELIOS | HELIOS/ |
| "CHLADNI", "chladni", "wave", "particles" | CHLADNI | SIMULATION/CHLADNI/ |
| "SLAM", "slam", "localization" | SLAM | LEARN/SLAM/ |
| "DNA", "dna", "foundation", "core" | DNA | DNA/ |
| "LEARN", "learn", "tutorial" | LEARN | LEARN/ |
| "ESP32", "esp32" | ESP32 | LEARN/ESP32/ |
| "ARDUINO", "arduino" | ARDUINO | LEARN/ARDUINO/ |
| "UBUNTU", "ubuntu", "linux" | UBUNTU | LEARN/UBUNTU/ |
| "AUTOCRATE", "autocrate", "crate" | AUTOCRATE | TOOLS/AUTOCRATE/ |
| "PLL", "pll", "phase-locked" | PLL | TOOLS/PLL/ |
| "BLOG", "blog" | BLOG | BLOG/ |

## Instructions

1. **Detect project** from user's message or current location:
   - Check for explicit project mention in conversation
   - Check current directory/branch if in worktree
   - Extract from GitHub issue URL if provided

2. **Load BRAIN context first** (fast cached context):
   ```bash
   cat .claude/BRAIN/${PROJECT}/context.md 2>/dev/null
   ```

3. **Fall back to project CLAUDE.md** if no BRAIN or need more detail:
   ```bash
   cat ${PROJECT_PATH}/CLAUDE.md
   ```

4. **Check for issue context** if working on an issue:
   ```bash
   gh issue view <number> --json title,body,labels
   ```

## Output Format

```
## Context: {PROJECT}

**Path:** {full path}
**Port:** {dev port}
**Type:** {WASM/Static/Library}

### Quick Reference
{from BRAIN context}

### Key Files
{list from BRAIN}

### Validation
{validation commands}

Ready to work on {PROJECT}.
```

## BRAIN Files Location

Pre-computed context files are in `.claude/BRAIN/`:
- WELCOME/context.md
- HELIOS/context.md
- CHLADNI/context.md
- SLAM/context.md
- DNA/context.md
- LEARN/context.md
