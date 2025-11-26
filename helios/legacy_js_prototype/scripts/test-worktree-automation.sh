#!/bin/bash
# Full automation test for worktree with commit metadata
# This script tests the complete workflow: worktree creation, automation, and commit formatting

set -e

echo "🧪 Full Worktree Automation Test"
echo "================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test 1: Verify worktree setup
echo -e "${BLUE}Test 1: Worktree Setup${NC}"
echo "Current worktree:"
git worktree list
echo ""

# Test 2: Verify automation ran (node_modules exists)
echo -e "${BLUE}Test 2: Automation Execution${NC}"
if [ -d "node_modules" ]; then
    echo -e "${GREEN}✅ node_modules exists${NC}"
    PACKAGE_COUNT=$(ls node_modules 2>/dev/null | wc -l)
    echo "   Packages installed: $PACKAGE_COUNT"
else
    echo -e "${YELLOW}⚠️  node_modules missing - running npm install${NC}"
    npm install
fi
echo ""

# Test 3: Verify git hooks installed
echo -e "${BLUE}Test 3: Git Hooks Installation${NC}"
if [ -f ".git/hooks/commit-msg" ]; then
    echo -e "${GREEN}✅ Git hooks installed${NC}"
    if grep -q "format-commit-msg.sh" ".git/hooks/commit-msg"; then
        echo -e "${GREEN}✅ Commit message formatter configured${NC}"
    else
        echo -e "${YELLOW}⚠️  Installing hooks...${NC}"
        bash scripts/setup-git-hooks.sh
    fi
else
    echo -e "${YELLOW}⚠️  Installing hooks...${NC}"
    bash scripts/setup-git-hooks.sh
fi
echo ""

# Test 4: Test commit message formatting
echo -e "${BLUE}Test 4: Commit Message Formatting${NC}"
echo "Creating a test commit to verify metadata formatting..."
echo ""

# Get current branch and version
BRANCH=$(git rev-parse --abbrev-ref HEAD)
VERSION=$(node -p "require('./package.json').version" 2>/dev/null || echo "1.0.0")
AUTHOR=$(git config user.name || echo "test-user")

echo "Branch: $BRANCH"
echo "Version: $VERSION"
echo "Author: $AUTHOR"
echo ""

# Create a test file for commit
TEST_FILE="test-metadata-$(date +%s).txt"
echo "Test file for commit metadata" > "$TEST_FILE"
git add "$TEST_FILE"

# Test commit with metadata
echo "Making test commit..."
TEST_MSG="Test commit metadata formatting"
if [ -f "scripts/commit-with-metadata.sh" ]; then
    bash scripts/commit-with-metadata.sh "$TEST_MSG"
else
    git commit -m "$TEST_MSG"
fi

# Show the commit
echo ""
echo -e "${GREEN}✅ Commit created. Latest commit:${NC}"
git log -1 --pretty=format:"%s"
echo ""

# Test 5: Verify isolation
echo -e "${BLUE}Test 5: Worktree Isolation${NC}"
echo "Current worktree changes:"
git status --short
echo ""

# Test 6: Show commit format example
echo -e "${BLUE}Test 6: Expected Commit Format${NC}"
echo "Commits should display on GitHub as:"
echo ""
echo "  Issue #124 • Description text • v1.0.0 • 035d34a • by author • 11/5/2025, 3:26:19 PM"
echo ""
echo "Where:"
echo "  - Issue #124: Extracted from branch name or provided"
echo "  - Description: Your commit message"
echo "  - v1.0.0: Version from package.json"
echo "  - 035d34a: Short commit hash"
echo "  - by author: Git user name"
echo "  - Timestamp: Current date/time"
echo ""

# Cleanup test file
if [ -f "$TEST_FILE" ]; then
    git reset HEAD~1 > /dev/null 2>&1 || true
    rm -f "$TEST_FILE"
    echo -e "${GREEN}✅ Test commit cleaned up${NC}"
fi

echo ""
echo -e "${GREEN}✅ All tests completed!${NC}"
echo ""
echo "Next steps:"
echo "1. Make changes in this worktree"
echo "2. Commit using: ./scripts/commit-with-metadata.sh \"Your message\" [issue-number]"
echo "3. Or use regular git commit - the hook will format it automatically"
echo "4. Push to GitHub to see formatted metadata"

