#!/bin/bash
# Git commit message formatter with metadata
# Formats commits with Issue #, version, author, timestamp for GitHub display

set -e

COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2
SHA1=$3

# Get package version
VERSION=$(node -p "require('./package.json').version" 2>/dev/null || echo "1.0.0")

# Get current branch name
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")

# Extract issue number from branch name (e.g., issue-124, fix-124, feature-124)
ISSUE_NUM=$(echo "$BRANCH" | grep -oE '(issue|fix|feature|bug)-?([0-9]+)' | grep -oE '[0-9]+' | head -1)

# Get author name
AUTHOR=$(git config user.name || echo "unknown")

# Get timestamp
TIMESTAMP=$(date '+%m/%d/%Y, %I:%M:%S %p')

# Read the original commit message
ORIGINAL_MSG=$(cat "$COMMIT_MSG_FILE")

# If message already has metadata, don't modify it
if echo "$ORIGINAL_MSG" | grep -q "^Issue #"; then
    exit 0
fi

# Build formatted commit message
FORMATTED_MSG=""

# Add Issue tag if found
if [ -n "$ISSUE_NUM" ]; then
    FORMATTED_MSG="Issue #$ISSUE_NUM"
fi

# Add description
if [ -n "$ORIGINAL_MSG" ]; then
    if [ -n "$FORMATTED_MSG" ]; then
        FORMATTED_MSG="$FORMATTED_MSG • $ORIGINAL_MSG"
    else
        FORMATTED_MSG="$ORIGINAL_MSG"
    fi
fi

# Add references (issue numbers found in message)
REFS=$(echo "$ORIGINAL_MSG" | grep -oE '#[0-9]+' | tr '\n' ' ' | sed 's/ $//')
if [ -n "$REFS" ]; then
    FORMATTED_MSG="$FORMATTED_MSG ($REFS)"
fi

# Add version
FORMATTED_MSG="$FORMATTED_MSG • v$VERSION"

# Add commit hash (short)
if [ -n "$SHA1" ]; then
    SHORT_HASH=$(echo "$SHA1" | cut -c1-7)
    FORMATTED_MSG="$FORMATTED_MSG • $SHORT_HASH"
fi

# Add author
FORMATTED_MSG="$FORMATTED_MSG • by $AUTHOR"

# Add timestamp
FORMATTED_MSG="$FORMATTED_MSG • $TIMESTAMP"

# Write formatted message back
echo "$FORMATTED_MSG" > "$COMMIT_MSG_FILE"

exit 0

