#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: sync-atlas-data.sh | DNA/STORAGE_SERVER/deploy/sync-atlas-data.sh
# PURPOSE: Sync ATLAS map data to storage server
# USAGE: ./sync-atlas-data.sh [server-ip]
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
ATLAS_ASSETS="$REPO_ROOT/ATLAS/assets"
# Use STORAGE_HOST env var if set, otherwise use argument or default
SERVER_IP="${STORAGE_HOST:-${1:-144.126.145.3}}"
SERVER_USER="${STORAGE_USER:-root}"
REMOTE_DATA_DIR="/opt/atlas/data/atlas"

echo "=== ATLAS Data Sync ==="
echo "Source: $ATLAS_ASSETS"
echo "Target: $SERVER_USER@$SERVER_IP:$REMOTE_DATA_DIR"
echo ""

# Check if assets exist
if [ ! -d "$ATLAS_ASSETS" ]; then
    echo "ERROR: ATLAS assets directory not found: $ATLAS_ASSETS"
    exit 1
fi

# List files to sync
echo "Files to sync:"
ls -lh "$ATLAS_ASSETS"/*.geojson 2>/dev/null || echo "  No .geojson files"
ls -lh "$ATLAS_ASSETS"/*.geo 2>/dev/null || echo "  No .geo files"
echo ""

# Create remote directory
echo "Creating remote directory..."
ssh "$SERVER_USER@$SERVER_IP" "mkdir -p $REMOTE_DATA_DIR"

# Sync with rsync (efficient delta transfer)
echo "Syncing data..."
rsync -avz --progress \
    --include='*.geojson' \
    --include='*.geo' \
    --exclude='*' \
    "$ATLAS_ASSETS/" \
    "$SERVER_USER@$SERVER_IP:$REMOTE_DATA_DIR/"

echo ""
echo "=== Sync Complete ==="
echo ""
echo "Restart the server to pick up new data:"
echo "  ssh root@$SERVER_IP 'systemctl restart atlas-storage'"
echo ""
echo "Test the endpoint:"
echo "  curl http://$SERVER_IP:3000/v1/atlas/layers"
echo ""
