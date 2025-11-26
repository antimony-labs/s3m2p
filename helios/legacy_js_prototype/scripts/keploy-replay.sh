#!/bin/bash
# Replay recorded testcases using Keploy

set -e

if ! command -v keploy &>/dev/null; then
  echo "⚠️  Keploy CLI not found. Use docker-compose.keploy.yml or install keploy."
  echo "   Docs: docs/KEPLOY.md"
  exit 1
fi

export KEPLOY_MODE=test
export PORT=3000

echo "🏗️  Building app..."
npm run build

echo "🧪 Replaying with Keploy on port $PORT..."
keploy test --command "npm start" -- --config keploy.yaml

