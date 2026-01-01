# Start Work on Issue

Start work on GitHub issue #$ARGUMENTS using worktree isolation.

## Workflow (ALWAYS use worktrees)

1. **Check current location**:
   ```bash
   pwd
   git branch --show-current
   ```

2. **If on main branch** - create/switch to worktree:
   ```bash
   ./SCRIPTS/worktree.sh create $ARGUMENTS
   ```

   Then **STOP** and tell the user:
   ```
   Worktree created! To continue:

   cd ~/worktrees/{project}-issue-$ARGUMENTS
   claude

   Then say "work on issue $ARGUMENTS" again.
   ```

3. **If already in worktree** - proceed with work:
   - Fetch issue details: `gh issue view $ARGUMENTS --json title,body,labels`
   - Load BRAIN context: `cat .claude/BRAIN/{PROJECT}/context.md`
   - Load project CLAUDE.md if needed
   - Create implementation plan with TodoWrite

## Project Detection

Extract project from:
1. Issue label: `project:xxx`
2. Title prefix: `[xxx]`
3. Branch name: `{project}/issue-XX`

## Output Format

### If creating worktree:
```
## Worktree Created for Issue #$ARGUMENTS

**Path:** ~/worktrees/{project}-issue-$ARGUMENTS
**Branch:** {project}/issue-$ARGUMENTS

To start working:
  cd ~/worktrees/{project}-issue-$ARGUMENTS
  claude

Then say "work on issue $ARGUMENTS" to continue.
```

### If in worktree (ready to work):
```
## Issue #$ARGUMENTS: [Title]

**Project:** [project]
**Branch:** [current branch]
**Worktree:** [current path]

### Summary
[What needs to be done]

### Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

### Implementation Plan
1. Step 1
2. Step 2

Ready to proceed?
```

## Key Rules

- **NEVER work on main** for features/fixes
- **ALWAYS create worktree first**
- **PR back to main** when done using `/pr`
- Worktrees live in `~/worktrees/`
