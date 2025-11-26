# 🚀 Worktree Quick Reference

## Start a Feature
```bash
npm run task "Feature description"
```

## Work on Feature
```bash
cd ~/.cursor/worktrees/too.foo/issue-{number}
# Make changes...
git add . && git commit -m "Add: feature" && git push
```

## List Worktrees
```bash
npm run worktrees
```

## Remove Worktree
```bash
node scripts/worktree.js remove issue-{number}
```

## Cleanup Merged
```bash
node scripts/worktree.js cleanup
```

## Verify Setup
```bash
npm run verify-worktree
```

---
**Full guide:** [WORKTREE_GUIDE.md](WORKTREE_GUIDE.md)

