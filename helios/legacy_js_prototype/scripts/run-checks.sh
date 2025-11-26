#!/bin/bash
# Consolidated local checks before pushing to GitHub (VCS-only mode)
# - Installs deps on first run
# - Runs unit/integration tests
# - Builds the app to ensure production parity
# - Optionally runs visual tests if ENABLE_VISUAL_TESTS=1

set -euo pipefail

echo "🚦 Running local checks (tests/build)..."

# Ensure Node dependencies
if [ ! -d node_modules ]; then
  echo "📦 Installing dependencies (node_modules missing)..."
  npm install --silent
fi

# Unit and integration tests
echo "🧪 Running unit/integration tests..."
npm run -s test

# Hydration error check (before build to catch issues early)
# Skip if Playwright browsers not installed (graceful degradation)
if [ "${SKIP_HYDRATION_CHECK:-0}" != "1" ]; then
  echo "🔍 Checking for hydration errors..."
  if npm run check:hydration 2>&1 | tee /tmp/hydration-check.log; then
    echo "✅ Hydration check passed"
  else
    EXIT_CODE=$?
    if grep -q "Executable doesn't exist\|browserType.launch" /tmp/hydration-check.log 2>/dev/null; then
      echo "⚠️  Playwright browsers not installed. Skipping hydration check."
      echo "   Install browsers: npx playwright install chromium"
      echo "   Or skip explicitly: SKIP_HYDRATION_CHECK=1"
    else
      echo "❌ Hydration check failed. Review errors above."
      echo "   To skip hydration check: SKIP_HYDRATION_CHECK=1"
      exit $EXIT_CODE
    fi
  fi
else
  echo "ℹ️  Skipping hydration check (SKIP_HYDRATION_CHECK=1)."
fi

# Build check
echo "🏗️  Building production bundle..."
npm run -s build

# Visual tests (optional)
if [ "${ENABLE_VISUAL_TESTS:-0}" = "1" ]; then
  echo "👀 Running visual tests..."
  npm run -s test:visual || {
    echo "❌ Visual tests failed. To skip, unset ENABLE_VISUAL_TESTS or set to 0." >&2
    exit 1
  }
else
  echo "ℹ️  Skipping visual tests (set ENABLE_VISUAL_TESTS=1 to enable)."
fi

echo "✅ All checks passed."

