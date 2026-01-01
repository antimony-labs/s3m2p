#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FILE: watch_tree.sh | WELCOME/watch_tree.sh
# PURPOSE: Watch script for monitoring directory tree changes during development
# MODIFIED: 2025-11-30
# ═══════════════════════════════════════════════════════════════════════════════

watch -n 1 --color tree -C
