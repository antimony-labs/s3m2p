# 🌳 Parallel Feature Development with Worktrees

**Work on multiple features simultaneously without conflicts!**

## 🚀 Quick Start

### 1. Start a New Feature

```bash
npm run task "Your feature description here"
```

**That's it!** This automatically:
- ✅ Creates a GitHub issue
- ✅ Creates a branch (`issue-{number}`)
- ✅ Creates a worktree at `~/.cursor/worktrees/too.foo/issue-{number}`
- ✅ Creates a PR
- ✅ Shows you where to work

### 2. Work on Your Feature

The script tells you the worktree path. Switch to it:

```bash
cd ~/.cursor/worktrees/too.foo/issue-{number}
```

Make your changes, then commit:

```bash
git add .
git commit -m "Add: your feature"
git push origin issue-{number}
```

### 3. Preview & Merge

- Vercel automatically creates a preview URL (check the PR)
- Test your changes
- Merge the PR when ready

## 💡 Working on Multiple Features

**You can work on multiple features at the same time!**

```bash
# Feature 1
npm run task "Fix button styling"
# Worktree: ~/.cursor/worktrees/too.foo/issue-123

# Feature 2 (in another terminal/agent)
npm run task "Add dark mode"
# Worktree: ~/.cursor/worktrees/too.foo/issue-124

# Feature 3 (in another terminal/agent)
npm run task "Update documentation"
# Worktree: ~/.cursor/worktrees/too.foo/issue-125
```

**All three worktrees exist simultaneously - no conflicts!**

## 📋 Common Commands

```bash
# Start a new feature
npm run task "Description"

# Start from an existing issue
npm run issue -- 123
npm run issue -- https://github.com/<owner>/<repo>/issues/123

# List all active worktrees
npm run worktrees

# Remove a worktree (after PR is merged)
node scripts/worktree.js remove issue-123

# Clean up all merged worktrees
node scripts/worktree.js cleanup
```

## 🎯 Best Practices

1. **One worktree per feature** - Don't reuse worktrees
2. **Work in the worktree directory** - Not in the main repo
3. **Clean up after merge** - Remove worktrees when done
4. **Use descriptive task names** - Makes it easier to find worktrees

## 🔧 Setup (One-Time)

**Verify your setup:**

```bash
npm run verify-worktree
```

**Set your GitHub token:**

```bash
export GITHUB_TOKEN=your_token_here
```

Get a token at: https://github.com/settings/tokens

## 📁 Where Are Worktrees?

Worktrees are stored at:
```
~/.cursor/worktrees/too.foo/issue-{number}
```

Each worktree is a complete copy of your repo on a specific branch.

## ❓ Troubleshooting

**Worktree already exists?**
- The script will use the existing worktree
- Or remove it: `node scripts/worktree.js remove issue-123`

**Can't find worktree?**
- List all worktrees: `npm run worktrees`
- Check the output from `npm run task` - it shows the path

**Need to switch between features?**
- Just `cd` to the worktree directory you want to work on
- Each worktree is independent

---

**Quick reference:** See [WORKTREE_CHEATSHEET.md](WORKTREE_CHEATSHEET.md) for a one-page cheat sheet.

**That's it!** You're ready to work on features in parallel. 🎉
