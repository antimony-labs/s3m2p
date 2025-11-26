#!/bin/bash
# Keploy-based hydration error detection
# Records user sessions and replays them to detect hydration errors

set -e

if ! command -v keploy &>/dev/null; then
  echo "⚠️  Keploy CLI not found. Install keploy or use docker-compose.keploy.yml"
  echo "   See: https://docs.keploy.io/"
  exit 1
fi

export KEPLOY_MODE=test
export PORT=4173

echo "🧪 Keploy Hydration Error Detection"
echo "===================================="
echo ""

echo "🏗️  Building app..."
npm run build

echo ""
echo "🚀 Starting server on port $PORT..."
# Start server in background
npm run start:visual &
SERVER_PID=$!

# Wait for server to be ready
echo "⏳ Waiting for server to be ready..."
for i in {1..30}; do
  if curl -s http://127.0.0.1:$PORT > /dev/null 2>&1; then
    echo "✅ Server is ready"
    break
  fi
  if [ $i -eq 30 ]; then
    echo "❌ Server failed to start"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
  fi
  sleep 1
done

echo ""
echo "🔍 Running Playwright hydration tests with Keploy..."
# Run Playwright tests which will be recorded by Keploy
npx playwright test tests/visual/hydration-error.spec.ts || TEST_EXIT_CODE=$?

# Cleanup
echo ""
echo "🛑 Stopping server..."
kill $SERVER_PID 2>/dev/null || true

if [ -n "$TEST_EXIT_CODE" ]; then
  echo ""
  echo "❌ Hydration errors detected!"
  exit $TEST_EXIT_CODE
else
  echo ""
  echo "✅ No hydration errors detected"
  exit 0
fi

