#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: audit.sh | SCRIPTS/audit.sh
# PURPOSE: Runs security audits with cargo audit, cargo deny, and unsafe code scanner
# MODIFIED: 2025-11-29
# ═══════════════════════════════════════════════════════════════════════════════
set -e

echo "=== S3M2P Security Audit ==="
echo ""

echo "=== cargo audit ==="
cargo audit || echo "Warning: cargo-audit not installed or failed"
echo ""

echo "=== cargo deny ==="
cargo deny check || echo "Warning: cargo-deny not installed or failed"
echo ""

echo "=== Unsafe code scan ==="
grep -rn "unsafe" --include="*.rs" DNA/ SIM/ SW/ HW/ TOOLS/ 2>/dev/null || echo "None found"
echo ""

echo "=== Security audit complete ==="
