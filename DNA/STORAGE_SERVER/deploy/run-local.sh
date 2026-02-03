#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: run-local.sh | DNA/STORAGE_SERVER/deploy/run-local.sh
# PURPOSE: Run storage server locally for testing
# USAGE: ./run-local.sh
# ═══════════════════════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Create local data directory structure
LOCAL_DATA="$SCRIPT_DIR/../local-data"
mkdir -p "$LOCAL_DATA/atlas"

# Symlink ATLAS assets (or copy if you prefer)
if [ ! -L "$LOCAL_DATA/atlas" ] && [ -d "$REPO_ROOT/ATLAS/assets" ]; then
    rm -rf "$LOCAL_DATA/atlas"
    ln -s "$REPO_ROOT/ATLAS/assets" "$LOCAL_DATA/atlas"
    echo "Linked ATLAS assets to local data directory"
fi

echo "=== Starting Storage Server ==="
echo "Data dir: $LOCAL_DATA"
echo "Endpoints:"
echo "  Health:  http://127.0.0.1:3000/v1/health"
echo "  ATLAS:   http://127.0.0.1:3000/v1/atlas/layers"
echo "  Layer:   http://127.0.0.1:3000/v1/atlas/countries/110m"
echo ""

cd "$SCRIPT_DIR/.."
DATA_DIR="$LOCAL_DATA" RUST_LOG=info cargo run
