# Full Worktree Automation Testing Guide

## Overview

This guide explains how to test the complete worktree automation system with commit metadata formatting for GitHub display.

## System Components

### 1. Worktree Automation (`.cursor/worktrees.json`)
When Cursor creates a new worktree, it automatically runs:
- `npm install` - Installs dependencies
- `bash scripts/setup-git-hooks.sh` - Sets up git hooks for commit formatting

### 2. Commit Metadata Formatting
Commits are automatically formatted with:
- **Issue #**: Extracted from branch name (e.g., `issue-124`, `fix-124`) or provided manually
- **Description**: Your commit message
- **Version**: From `package.json`
- **Commit Hash**: Short hash (7 characters)
- **Author**: Git user name
- **Timestamp**: Current date/time

### 3. Display Format on GitHub
Commits will display as:
```
Issue #124 • Issue number display not updating correctly • v1.0.0 • 035d34a • by author • 11/5/2025, 3:26:19 PM
```

## Testing Workflow

### Step 1: Create a Worktree (via Cursor)
When Cursor creates a worktree, the automation runs automatically:
1. Dependencies are installed (`npm install`)
2. Git hooks are configured
3. Worktree is ready for development

### Step 2: Make Changes
Work in your worktree independently:
```bash
cd /path/to/worktree
# Make your changes
```

### Step 3: Commit with Metadata

**Option A: Use the helper script (recommended)**
```bash
./scripts/commit-with-metadata.sh "Your commit message" [issue-number]
```

**Option B: Use regular git commit (hook formats automatically)**
```bash
git commit -m "Your commit message"
# Hook automatically adds metadata
```

**Option C: Branch with issue number (auto-detected)**
```bash
git checkout -b issue-124-feature
git commit -m "Fix the bug"
# Automatically formats as: Issue #124 • Fix the bug • v1.0.0 • ...
```

### Step 4: Verify Formatting
```bash
git log -1 --pretty=format:"%s"
```

### Step 5: Push to GitHub
```bash
git push origin branch-name
```

On GitHub, commits will display with full metadata.

## Multi-Agent Testing

### Scenario: Multiple Agents Working Simultaneously

1. **Agent 1** creates worktree:
   ```bash
   git worktree add /tmp/agent-1-worktree -b issue-124-fix
   ```

2. **Agent 2** creates worktree:
   ```bash
   git worktree add /tmp/agent-2-worktree -b issue-125-feature
   ```

3. **Both agents work independently:**
   - No conflicts
   - Each has own `node_modules`
   - Each has own git hooks
   - Each commits with proper metadata

4. **Merge when ready:**
   ```bash
   git checkout main
   git merge issue-124-fix
   git merge issue-125-feature
   ```

## Test Script

Run the full automation test:
```bash
./scripts/test-worktree-automation.sh
```

This script verifies:
- ✅ Worktree setup
- ✅ Automation execution (npm install)
- ✅ Git hooks installation
- ✅ Commit message formatting
- ✅ Worktree isolation
- ✅ Expected format display

## Commit Format Examples

### Example 1: Issue from branch name
```bash
git checkout -b issue-124
git commit -m "Fix display bug"
# Result: Issue #124 • Fix display bug • v1.0.0 • abc1234 • by user • 11/7/2025, 3:16:23 am
```

### Example 2: Manual issue number
```bash
./scripts/commit-with-metadata.sh "Add new feature" 170
# Result: Issue #170 • Add new feature • v1.0.0 • def5678 • by user • 11/7/2025, 3:16:23 am
```

### Example 3: Multiple references in message
```bash
git commit -m "Fix bug related to #124 and #170"
# Result: Issue #124 • Fix bug related to #124 and #170 (#124) (#170) • v1.0.0 • ...
```

## Files Created

- `scripts/format-commit-msg.sh` - Git hook script that formats commits
- `scripts/commit-with-metadata.sh` - Helper script for manual commits
- `scripts/setup-git-hooks.sh` - Sets up hooks in worktrees
- `scripts/test-worktree-automation.sh` - Full automation test
- `.cursor/worktrees.json` - Cursor automation configuration

## Troubleshooting

### Hooks not running?
```bash
# Manually install hooks
bash scripts/setup-git-hooks.sh
```

### Metadata not appearing?
- Check that hooks are installed: `ls -la .git/hooks/commit-msg`
- Verify script exists: `ls scripts/format-commit-msg.sh`
- Check hook permissions: `chmod +x .git/hooks/commit-msg`

### Worktree automation not running?
- Ensure `.cursor/worktrees.json` exists
- Check Cursor is creating the worktree (not manual git commands)
- Verify scripts are executable: `chmod +x scripts/*.sh`

## Next Steps

1. ✅ Test automation in worktree
2. ✅ Verify commit formatting
3. ✅ Test with multiple worktrees
4. ✅ Push to GitHub and verify display
5. ✅ Use in production workflow

## Benefits

- **Automatic Metadata**: No need to manually format commits
- **GitHub Display**: Commits show rich metadata on GitHub
- **Multi-Agent Safe**: Each worktree is isolated
- **Issue Tracking**: Automatic issue number extraction
- **Version Tracking**: Version included in every commit
- **Audit Trail**: Author and timestamp on every commit

