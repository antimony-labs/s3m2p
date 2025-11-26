#!/bin/bash
# Helper script to make commits with metadata formatting
# Usage: ./scripts/commit-with-metadata.sh "Your commit message" [issue-number]

set -e

COMMIT_MSG="$1"
ISSUE_NUM="$2"

if [ -z "$COMMIT_MSG" ]; then
    echo "Usage: $0 \"commit message\" [issue-number]"
    exit 1
fi

# Get version
VERSION=$(node -p "require('./package.json').version" 2>/dev/null || echo "1.0.0")

# Get current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Use provided issue number or extract from branch
if [ -z "$ISSUE_NUM" ]; then
    ISSUE_NUM=$(echo "$BRANCH" | grep -oE '(issue|fix|feature|bug)-?([0-9]+)' | grep -oE '[0-9]+' | head -1)
fi

# Get author
AUTHOR=$(git config user.name || echo "unknown")

# Get timestamp
TIMESTAMP=$(date '+%m/%d/%Y, %I:%M:%S %P')

# Build commit message with metadata
if [ -n "$ISSUE_NUM" ]; then
    FULL_MSG="Issue #$ISSUE_NUM • $COMMIT_MSG • v$VERSION • by $AUTHOR • $TIMESTAMP"
else
    FULL_MSG="$COMMIT_MSG • v$VERSION • by $AUTHOR • $TIMESTAMP"
fi

# Make the commit
git commit -m "$FULL_MSG"

echo "✅ Committed with metadata:"
echo "$FULL_MSG"

