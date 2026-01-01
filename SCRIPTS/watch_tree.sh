#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: watch_tree.sh | SCRIPTS/watch_tree.sh
# PURPOSE: Continuously displays directory tree structure with auto-refresh every second
# MODIFIED: 2025-11-30
# ═══════════════════════════════════════════════════════════════════════════════
watch -n 1 --color tree -L 2 -C
