#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: run_tests.sh | COMING_SOON/run_tests.sh
# PURPOSE: Test suite runner for wave-particle ecosystem stability at multiple durations
# MODIFIED: 2025-12-09
# ═══════════════════════════════════════════════════════════════════════════════

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  Quantum Wave-Particle Ecosystem - Stability Test Suite      ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Test 1: 20 seconds (1200 frames @ 60fps)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 1: 20 second simulation (1200 frames)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
node test_wave_sim.js 1200 200
echo ""

# Test 2: 1 minute (3600 frames @ 60fps)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 2: 1 minute simulation (3600 frames)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
node test_wave_sim.js 3600 600
echo ""

# Test 3: 10 minutes (36000 frames @ 60fps)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 3: 10 minute simulation (36000 frames) - Long stability test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
node test_wave_sim.js 36000 3000
echo ""

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  All tests complete                                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
