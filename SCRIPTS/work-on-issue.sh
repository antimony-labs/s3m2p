#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: work-on-issue.sh | SCRIPTS/work-on-issue.sh
# PURPOSE: Creates worktree and opens development environment for GitHub issue work
# MODIFIED: 2025-11-30
# ═══════════════════════════════════════════════════════════════════════════════
# Simple issue workflow
# Usage: work-on-issue.sh <issue-number>

ISSUE="$1"

if [ -z "$ISSUE" ]; then
    echo "Usage: work-on-issue.sh <issue-number>"
    echo ""
    echo "Example: work-on-issue.sh 42"
    echo ""
    echo "This will:"
    echo "  1. Create/check worktree for issue"
    echo "  2. Open terminator with:"
    echo "     - Terminal 1: btop (system monitor)"
    echo "     - Terminal 2: Dev server for issue"
    echo "     - Terminal 3: Claude Code (ready for commands)"
    exit 1
fi

WORKTREE_PATH="$HOME/worktrees/issue-$ISSUE"

# Check if worktree exists, create if not
if [ ! -d "$WORKTREE_PATH" ]; then
    echo "Creating worktree for issue $ISSUE..."
    cd ~/S3M2P
    git worktree add "$WORKTREE_PATH" -b "issue-$ISSUE"
    echo "✓ Worktree created at $WORKTREE_PATH"
else
    echo "✓ Worktree already exists at $WORKTREE_PATH"
fi

# Fetch issue details
echo ""
echo "Fetching issue #$ISSUE..."
gh issue view "$ISSUE" 2>/dev/null || echo "⚠️  Could not fetch issue (continue anyway)"

echo ""
echo "🚀 Opening development environment..."
echo ""
echo "Terminator layout:"
echo "  Top-left: btop (system monitor)"
echo "  Top-right: Dev server"
echo "  Bottom: Claude Code workspace"
echo ""

# Open terminator with layout
terminator -l issue-dev &

sleep 1

# Instructions for manual setup (terminator layouts are complex)
echo ""
echo "In terminator:"
echo "  1. Split horizontal (Ctrl+Shift+O)"
echo "  2. Split vertical (Ctrl+Shift+E)"
echo "  3. Run in each pane:"
echo "     - Pane 1: btop"
echo "     - Pane 2: cd $WORKTREE_PATH && run <project>"
echo "     - Pane 3: cd $WORKTREE_PATH"
echo ""
echo "Or just use separate tabs in your editor terminal."
