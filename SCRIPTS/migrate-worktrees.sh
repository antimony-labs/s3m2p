#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: migrate-worktrees.sh | SCRIPTS/migrate-worktrees.sh
# PURPOSE: Migrate existing worktrees to new naming convention
# CREATED: 2026-01-04
# ═══════════════════════════════════════════════════════════════════════════════

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKTREE_BASE="$HOME/worktrees"

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║  Worktree Migration: project-issue-XX → issue-XX-project-description  ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# Track statistics
total=0
migrated=0
skipped=0
failed=0

# Get list of all worktrees (excluding main repo)
worktrees=$(git -C "$REPO_ROOT" worktree list | tail -n +2 | awk '{print $1}')

for worktree_path in $worktrees; do
    ((total++)) || true

    worktree_name=$(basename "$worktree_path")

    # Extract issue number from old format (project-issue-XX)
    issue_num=$(echo "$worktree_name" | grep -oP 'issue-\K\d+' || true)

    if [[ -z "$issue_num" ]]; then
        echo "⚠️  Skipping $worktree_name (cannot extract issue number)"
        ((skipped++)) || true
        continue
    fi

    echo "────────────────────────────────────────────────────────────────────────"
    echo "Processing: $worktree_name (Issue #$issue_num)"

    # Check for uncommitted changes
    if [[ -n $(git -C "$worktree_path" status --porcelain 2>/dev/null) ]]; then
        echo "⚠️  HAS UNCOMMITTED CHANGES - skipping"
        git -C "$worktree_path" status --short
        ((skipped++)) || true
        continue
    fi

    # Check if ahead/behind remote
    branch_status=$(git -C "$worktree_path" status -sb 2>/dev/null | head -1)
    if echo "$branch_status" | grep -q '\[ahead'; then
        echo "⚠️  AHEAD OF REMOTE - skipping"
        echo "   $branch_status"
        ((skipped++)) || true
        continue
    fi

    echo "✓ Clean worktree - proceeding with migration"

    # Get branch name
    branch_name=$(git -C "$worktree_path" branch --show-current)
    echo "  Branch: $branch_name"

    # Fetch issue info from GitHub
    echo "  Fetching issue details..."
    issue_json=$(gh issue view "$issue_num" --json title,labels 2>/dev/null) || {
        echo "❌ Failed to fetch issue #$issue_num from GitHub"
        ((failed++)) || true
        continue
    }

    # Extract title and create slug
    title=$(echo "$issue_json" | jq -r '.title' | sed 's/\[.*\]//' | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9' '-' | sed 's/^-//;s/-$//' | cut -c1-30)

    # Extract project from old worktree name
    project=$(echo "$worktree_name" | sed "s/-issue-${issue_num}//")

    # Create new worktree name
    new_name="issue-${issue_num}-${project}-${title}"
    new_path="${WORKTREE_BASE}/${new_name}"

    echo "  Old: $worktree_name"
    echo "  New: $new_name"

    # Check if new path already exists
    if [[ -e "$new_path" ]]; then
        echo "⚠️  Target path already exists - skipping"
        ((skipped++)) || true
        continue
    fi

    # Remove old worktree
    echo "  Removing old worktree..."
    git -C "$REPO_ROOT" worktree remove "$worktree_path" 2>/dev/null || {
        echo "❌ Failed to remove old worktree"
        ((failed++)) || true
        continue
    }

    # Create new worktree with same branch
    echo "  Creating new worktree..."
    git -C "$REPO_ROOT" worktree add "$new_path" "$branch_name" 2>/dev/null || {
        echo "❌ Failed to create new worktree"
        # Try to restore old worktree
        git -C "$REPO_ROOT" worktree add "$worktree_path" "$branch_name" 2>/dev/null || true
        ((failed++)) || true
        continue
    }

    echo "✅ Migrated successfully"
    ((migrated++)) || true
done

echo ""
echo "════════════════════════════════════════════════════════════════════════"
echo "Migration Summary:"
echo "  Total worktrees:  $total"
echo "  ✅ Migrated:      $migrated"
echo "  ⚠️  Skipped:       $skipped"
echo "  ❌ Failed:        $failed"
echo "════════════════════════════════════════════════════════════════════════"

if [[ $skipped -gt 0 ]]; then
    echo ""
    echo "⚠️  Some worktrees were skipped due to uncommitted changes or unpushed commits."
    echo "   Review those worktrees manually and either:"
    echo "   - Commit and push changes, then re-run this script"
    echo "   - Manually migrate them after resolving the changes"
fi
