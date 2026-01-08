#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: update-worktrees.sh | SCRIPTS/update-worktrees.sh
# PURPOSE: Merge latest main branch into all worktrees
# CREATED: 2026-01-04
# ═══════════════════════════════════════════════════════════════════════════════

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║            Update All Worktrees with Latest Main Branch               ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# Track statistics
total=0
updated=0
skipped=0
conflicts=0

# Get current main branch commit
main_commit=$(git -C "$REPO_ROOT" rev-parse main)
main_short=$(git -C "$REPO_ROOT" rev-parse --short main)

echo "Main branch is at: $main_short"
echo ""

# Get list of all worktrees (excluding main repo)
worktrees=$(git -C "$REPO_ROOT" worktree list | tail -n +2 | awk '{print $1}')

for worktree_path in $worktrees; do
    ((total++)) || true

    worktree_name=$(basename "$worktree_path")
    branch_name=$(git -C "$worktree_path" branch --show-current)

    echo "────────────────────────────────────────────────────────────────────────"
    echo "[$total] $worktree_name"
    echo "    Branch: $branch_name"

    # Check for uncommitted changes
    if [[ -n $(git -C "$worktree_path" status --porcelain 2>/dev/null) ]]; then
        echo "    ⚠️  Has uncommitted changes - skipping"
        git -C "$worktree_path" status --short | head -5
        ((skipped++)) || true
        continue
    fi

    # Check if already up to date with main
    merge_base=$(git -C "$worktree_path" merge-base HEAD main 2>/dev/null)
    if [[ "$merge_base" == "$main_commit" ]]; then
        echo "    ✓ Already up to date with main"
        ((updated++)) || true
        continue
    fi

    # Get the divergence
    ahead=$(git -C "$worktree_path" rev-list --count HEAD ^main 2>/dev/null || echo "0")
    behind=$(git -C "$worktree_path" rev-list --count main ^HEAD 2>/dev/null || echo "0")

    echo "    Divergence: $ahead commits ahead, $behind commits behind main"

    # Attempt merge
    echo "    Merging main..."
    if git -C "$worktree_path" merge main --no-edit 2>&1 | grep -q "CONFLICT"; then
        echo "    ❌ MERGE CONFLICT - needs manual resolution"
        echo ""
        echo "    To resolve:"
        echo "      cd $worktree_path"
        echo "      # Fix conflicts in the files listed below"
        git -C "$worktree_path" diff --name-only --diff-filter=U | sed 's/^/      #   /'
        echo "      git add <resolved-files>"
        echo "      git commit"
        echo ""
        ((conflicts++)) || true
    elif git -C "$worktree_path" merge main --no-edit > /dev/null 2>&1; then
        echo "    ✅ Merged successfully"
        ((updated++)) || true
    else
        # Check if merge already happened
        if git -C "$worktree_path" diff --quiet HEAD main 2>/dev/null; then
            echo "    ✓ Already merged"
            ((updated++)) || true
        else
            echo "    ⚠️  Merge attempt unclear - check manually"
            ((skipped++)) || true
        fi
    fi
done

echo ""
echo "════════════════════════════════════════════════════════════════════════"
echo "Update Summary:"
echo "  Total worktrees:     $total"
echo "  ✅ Updated/Current:  $updated"
echo "  ⚠️  Skipped:          $skipped"
echo "  ❌ Conflicts:        $conflicts"
echo "════════════════════════════════════════════════════════════════════════"

if [[ $conflicts -gt 0 ]]; then
    echo ""
    echo "⚠️  Some worktrees have merge conflicts that need manual resolution."
    echo "   See the instructions above for each conflicted worktree."
fi

if [[ $skipped -gt 0 ]]; then
    echo ""
    echo "⚠️  Some worktrees were skipped due to uncommitted changes."
    echo "   Commit or stash changes in those worktrees, then re-run this script."
fi

# Exit with error if there were conflicts
if [[ $conflicts -gt 0 ]]; then
    exit 1
fi
