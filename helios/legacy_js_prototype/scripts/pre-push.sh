#!/bin/bash
# Git pre-push hook to enforce local checks prior to pushing to GitHub

set -e

HOOK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(cd "$HOOK_DIR/.." && pwd)")"

cd "$REPO_ROOT"

echo "🔒 pre-push: verifying repository integrity via local checks..."

# Allow override to skip (not recommended)
if [ "${SKIP_PREPUSH_CHECKS:-0}" = "1" ]; then
  echo "⚠️  SKIP_PREPUSH_CHECKS=1 set; bypassing checks."
  exit 0
fi

# Snapshot guard: disallow committing Playwright screenshots directly to main
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
if [ "$CURRENT_BRANCH" = "main" ]; then
  echo "🔍 Checking for visual snapshot changes on main..."
  # Try to diff against origin/main; if fetch fails, fallback to recent history
  if git fetch -q origin main; then
    CHANGED_SNAPSHOTS=$(git diff --name-only origin/main...HEAD | rg "^tests/visual/__screenshots__/" || true)
  else
    CHANGED_SNAPSHOTS=$(git diff --name-only HEAD~20..HEAD | rg "^tests/visual/__screenshots__/" || true)
  fi
  if [ -n "$CHANGED_SNAPSHOTS" ]; then
    echo "❌ Visual snapshot changes detected on main. Keep snapshots on feature branches." >&2
    echo "   Files:" >&2
    echo "$CHANGED_SNAPSHOTS" >&2
    echo "   If this merge is intentional, push via SKIP_PREPUSH_CHECKS=1 (not recommended)." >&2
    exit 1
  fi
fi

# Run consolidated checks
bash "$REPO_ROOT/scripts/run-checks.sh"

echo "✅ pre-push checks passed. Proceeding with push."
exit 0
