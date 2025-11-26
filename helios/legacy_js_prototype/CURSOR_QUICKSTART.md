# 🚀 Cursor Agent - Quick Start Guide (Worktree Edition)

## How to Use

When you want to make a change, **just paste your task description** and Cursor will handle everything automatically!

### Simple Usage

1. **Tell Cursor what you want:**
   ```
   "Create a new task: Fix the Layers button - it's not working"
   ```

2. **Cursor will automatically:**
   - Create a GitHub issue
   - Create a branch (`issue-{number}`)
   - **Create a worktree** at `~/.cursor/worktrees/too.foo/issue-{number}`
   - Create a PR
   - Output the worktree path

3. **Work in the worktree directory:**
   ```bash
   cd ~/.cursor/worktrees/too.foo/issue-{number}
   # Make your changes...
   ```

4. **Commit and push:**
   ```bash
   git add .
   git commit -m "Fix: Layers button"
   git push origin issue-{number}
   ```

5. **Test in Vercel preview** (automatically created)

6. **Merge PR** → Production deployment happens automatically!

## 🌳 Multiple Agents Working Simultaneously

**The best part:** You can have multiple agents working on different tasks at the same time!

**Agent 1 (Cursor):**
```bash
npm run task "Fix Layers button"
# Worktree: ~/.cursor/worktrees/too.foo/issue-123
```

**Agent 2 (ChatGPT):**
```bash
npm run task "Update documentation"
# Worktree: ~/.cursor/worktrees/too.foo/issue-124
```

**Agent 3 (Grok):**
```bash
npm run task "Add new feature"
# Worktree: ~/.cursor/worktrees/too.foo/issue-125
```

**All three can work simultaneously!** No conflicts! 🎉

## 🎯 Example Conversation with Cursor

**You:**
> "Create a new task: Fix the Layers button - it's not working"

**Cursor:**
> I'll create the issue, branch, and PR for you. Running: `npm run task "Fix the Layers button - it's not working"`

**After Cursor runs it:**
```
✅ Issue created: #123
✅ Branch created: issue-123
✅ PR created: #456
💡 You are now on branch: issue-123
```

**You:**
> "Now fix the button issue"

**Cursor:**
> *Makes the changes, commits, and pushes*

**You:**
> "Test in preview"

**Cursor:**
> *Shows you the preview URL from the PR*

## 📝 Alternative: Manual Command

If you prefer to run it manually:

```bash
npm run task "Your task description here"
```

Or:

```bash
node scripts/cursor-agent.js "Your task description here"
```

## 🔧 Setup (One Time)

1. **Set GitHub token:**
   ```bash
   export GITHUB_TOKEN=your_token_here
   ```
   Get token at: https://github.com/settings/tokens (needs `repo` scope)

2. **That's it!** The agent handles the rest.

## 📚 Full Documentation

See `.github/CURSOR_AGENT.md` for complete documentation.

