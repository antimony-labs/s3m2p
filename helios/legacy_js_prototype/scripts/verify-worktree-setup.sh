#!/bin/bash
# Quick setup verification for worktree workflow

echo "🔍 Verifying worktree setup..."
echo ""

# Check if git is available
if ! command -v git &> /dev/null; then
    echo "❌ Git is not installed"
    exit 1
fi

# Check if we're in a git repo
if ! git rev-parse --git-dir &> /dev/null; then
    echo "❌ Not in a git repository"
    exit 1
fi

# Check if worktree command exists
if ! git worktree list &> /dev/null; then
    echo "❌ Git worktree command not available (Git 2.5+ required)"
    exit 1
fi

# Check if npm scripts exist
if [ ! -f "package.json" ]; then
    echo "❌ package.json not found"
    exit 1
fi

# Check for required scripts
if ! grep -q '"task"' package.json; then
    echo "❌ npm run task script not found"
    exit 1
fi

if ! grep -q '"worktrees"' package.json; then
    echo "❌ npm run worktrees script not found"
    exit 1
fi

# Check if scripts exist
if [ ! -f "scripts/cursor-agent.js" ]; then
    echo "❌ scripts/cursor-agent.js not found"
    exit 1
fi

if [ ! -f "scripts/worktree.js" ]; then
    echo "❌ scripts/worktree.js not found"
    exit 1
fi

# Check for GitHub token (optional warning)
if [ -z "$GITHUB_TOKEN" ]; then
    echo "⚠️  GITHUB_TOKEN not set (required for creating issues/PRs)"
    echo "   Set it with: export GITHUB_TOKEN=your_token"
    echo ""
fi

# List current worktrees
echo "✅ Setup verified!"
echo ""
echo "📋 Current worktrees:"
git worktree list
echo ""
echo "🚀 Ready to use:"
echo "   npm run task \"Your feature description\""
echo "   npm run worktrees"

