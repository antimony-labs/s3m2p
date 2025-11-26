#!/bin/bash
# Record HTTP interactions using Keploy while running the app

set -e

if ! command -v keploy &>/dev/null; then
  echo "⚠️  Keploy CLI not found. Use docker-compose.keploy.yml or install keploy."
  echo "   Docs: docs/KEPLOY.md"
  exit 1
fi

export KEPLOY_MODE=record
export PORT=3000

echo "🏗️  Building app..."
npm run build

echo "🟢 Recording with Keploy on port $PORT..."
keploy record --command "npm start" -- --config keploy.yaml

