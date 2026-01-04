#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: config.sh | SCRIPTS/config.sh
# PURPOSE: Configuration file defining repository paths, project ports, and directory mappings
# MODIFIED: 2025-12-09
# ═══════════════════════════════════════════════════════════════════════════════
# S3M2P Configuration - Single Source of Truth
# Sourced by other scripts

# Repository Root
# If sourced from SCRIPTS dir, use parent. If from root, use .
if [[ -d "SCRIPTS" ]]; then
    REPO_ROOT="$(pwd)"
elif [[ -f "config.sh" ]]; then
    REPO_ROOT="$(cd .. && pwd)"
else
    REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi

# Log Directory
LOG_DIR="$REPO_ROOT/TESTS/logs"

# Project Port Assignments
declare -A PROJECT_PORTS=(
    ["welcome"]="8080"
    ["helios"]="8081"
    ["chladni"]="8082"
    ["sensors"]="8083"
    ["autocrate"]="8084"
    ["blog"]="8085"
    ["learn"]="8086"
    ["arch"]="8087"
    ["pll"]="8090"
    ["power"]="8091"
    ["ai"]="8100"
    ["ubuntu"]="8101"
    ["opencv"]="8102"
    ["arduino"]="8103"
    ["esp32"]="8104"
    ["swarm"]="8105"
    ["slam"]="8106"
    ["git"]="8107"
    ["coming_soon"]="8109"
    ["mcad"]="8088"

)

# Project Directory Mappings (Relative to REPO_ROOT)
declare -A PROJECT_DIRS=(
    ["welcome"]="WELCOME"
    ["helios"]="HELIOS"
    ["chladni"]="SIMULATION/CHLADNI"
    ["sensors"]="COMING_SOON"
    ["autocrate"]="TOOLS/AUTOCRATE"
    ["blog"]="BLOG"
    ["learn"]="LEARN"
    ["arch"]="ARCH"
    ["pll"]="TOOLS/PLL"
    ["power"]="COMING_SOON"
    ["ai"]="LEARN/AI"
    ["ubuntu"]="LEARN/UBUNTU"
    ["opencv"]="LEARN/OPENCV"
    ["arduino"]="COMING_SOON"
    ["esp32"]="LEARN/ESP32"
    ["swarm"]="LEARN/SWARM_ROBOTICS"
    ["slam"]="LEARN/SLAM"
    ["git"]="LEARN/GIT"
    ["coming_soon"]="COMING_SOON"
    ["mcad"]="MCAD"

)

# Cloudflare Pages Project Names (Mapping from config keys)
declare -A PAGES_PROJECTS=(
    ["welcome"]="too-foo"
    ["helios"]="helios-too-foo"
    ["blog"]="blog-too-foo"
    ["chladni"]="chladni-too-foo"
    ["sensors"]="sensors-too-foo"
    ["autocrate"]="autocrate-too-foo"
    ["pll"]="pll-too-foo"
    ["mcad"]="mcad-too-foo"
    ["coming_soon"]="coming-soon-too-foo"

    ["ai"]="ai-too-foo"
    ["arduino"]="arduino-too-foo"
    ["esp32"]="esp32-too-foo"
    ["ubuntu"]="ubuntu-too-foo"
    ["opencv"]="opencv-too-foo"
    ["swarm"]="swarm-too-foo"
    ["slam"]="slam-too-foo"
    ["git"]="git-too-foo"
    ["power"]="power-too-foo"
    ["arch"]="arch-too-foo"
)

# Static projects that don't need trunk build (deploy root dir directly)
declare -A STATIC_PROJECTS=(
    ["coming_soon"]=1
    ["arduino"]=1
    ["sensors"]=1
    ["power"]=1
)

# Export for sub-shells
export REPO_ROOT
export LOG_DIR
export PROJECT_PORTS
export PROJECT_DIRS
export PAGES_PROJECTS
export STATIC_PROJECTS
