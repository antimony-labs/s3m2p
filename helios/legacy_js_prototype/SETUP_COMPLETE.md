# ✅ Worktree Workflow Setup Complete!

## 🎉 What's Ready

Your automated **worktree-based** workflow is now set up! When you ask any agent (Cursor, ChatGPT, Grok, etc.) to make a change, it will automatically:

1. ✅ Create a GitHub issue
2. ✅ Create a branch (`issue-{number}`)
3. ✅ **Create a worktree** (isolated directory)
4. ✅ Create a PR
5. ✅ After you make changes and push → Vercel creates preview automatically
6. ✅ After you merge PR → Production deployment automatically

## 🌳 Key Feature: Multiple Agents Simultaneously

**Each task gets its own worktree**, so you can:
- ✅ Work on multiple changes at once
- ✅ Use different agents (Cursor, ChatGPT, Grok) simultaneously
- ✅ No conflicts between agents
- ✅ Each worktree is completely isolated

## 🚀 How to Use (Super Simple!)

### Just tell any agent:

> **"Create a new task: [your description]"**

**Example:**
> "Create a new task: Fix the Layers button - it's not working"

### The agent will automatically:

1. Run `npm run task "your description"`
2. Create issue, branch, **worktree**, and PR
3. Output the worktree path: `~/.cursor/worktrees/too.foo/issue-{number}`
4. **Change to worktree directory** and make changes
5. Commit and push from worktree
6. Tell you the preview URL is ready

### You just need to:

1. **Set GitHub token (one time):**
   ```bash
   export GITHUB_TOKEN=your_token_here
   ```
   Get token: https://github.com/settings/tokens (needs `repo` scope)

2. **That's it!** Just describe what you want and agents handle everything.

## 📝 Complete Example

**You (to Cursor):**
> "Create a new task: Fix the Layers button - it's not working"

**Cursor:**
1. Runs: `npm run task "Fix the Layers button - it's not working"`
2. Output:
   ```
   ✅ Issue created: #123
   ✅ Branch created: issue-123
   ✅ Worktree created: ~/.cursor/worktrees/too.foo/issue-123
   ✅ PR created: #456
   ```
3. Changes to worktree: `cd ~/.cursor/worktrees/too.foo/issue-123`
4. Makes the changes to fix the button
5. Commits: `git commit -m "Fix: Layers button click handler"`
6. Pushes: `git push origin issue-123`
7. Tells you: "✅ Fixed! Vercel preview is being created. Check PR #456 for preview URL."

**You:**
- Test in preview URL
- Merge PR when ready
- Production deploys automatically!

## 🌳 Multiple Agents Example

**You (to Cursor):**
> "Create a new task: Fix Layers button"

**You (to ChatGPT):**
> "Create a new task: Update documentation"

**You (to Grok):**
> "Create a new task: Add dark mode"

**All three agents work simultaneously:**
- Cursor → `~/.cursor/worktrees/too.foo/issue-123`
- ChatGPT → `~/.cursor/worktrees/too.foo/issue-124`
- Grok → `~/.cursor/worktrees/too.foo/issue-125`

**No conflicts! Each has its own isolated worktree!** 🎉

## 📚 Files Created

- ✅ `scripts/cursor-agent.js` - Main automation script (worktree-enabled)
- ✅ `scripts/worktree.js` - Worktree management script
- ✅ `.github/workflows/*.yml` - GitHub Actions workflows
- ✅ `.github/WORKTREE_WORKFLOW.md` - Complete worktree documentation
- ✅ `.cursor/AGENT_INSTRUCTIONS.md` - Instructions for agents

## 🔧 NPM Scripts Added

```bash
npm run task "Description"           # Create issue, branch, worktree, PR
npm run worktrees                    # List all worktrees
node scripts/worktree.js remove <branch>  # Remove worktree
node scripts/worktree.js cleanup     # Cleanup merged worktrees
```

## 🎯 Workflow Summary

```
You → Agent → npm run task → Issue → Branch → Worktree → PR → Changes → Push → Preview → Merge → Production
```

**Key difference:** Each task gets its own **worktree** (isolated directory)!

## 💡 Tips

1. **Always work in the worktree directory** - Don't work in main repo
2. **Each agent gets its own worktree** - No conflicts!
3. **Worktrees are at:** `~/.cursor/worktrees/too.foo/issue-{number}`
4. **List worktrees:** `npm run worktrees`
5. **Cleanup after merge:** `node scripts/worktree.js cleanup`

## 🐛 Troubleshooting

**Token not set:**
```bash
export GITHUB_TOKEN=your_token
```

**Want to run manually:**
```bash
npm run task "Your description"
```

**List all worktrees:**
```bash
npm run worktrees
```

**Remove a worktree:**
```bash
node scripts/worktree.js remove issue-123
```

**Check workflow status:**
- GitHub Actions: https://github.com/Shivam-Bhardwaj/too.foo/actions
- PR checks: Visible on each PR
- Vercel dashboard: Preview/production deployments

---

**You're all set!** Now you can work on multiple changes simultaneously with different agents! 🚀🌳
