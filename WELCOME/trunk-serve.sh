#!/bin/bash
cd "$(dirname "$0")"

# Trunk reads NO_COLOR as a boolean env var; some environments set NO_COLOR=1 which breaks parsing.
if [[ "${NO_COLOR:-}" == "1" ]]; then export NO_COLOR=true; fi
if [[ "${NO_COLOR:-}" == "0" ]]; then export NO_COLOR=false; fi

# Filter out --no-color flag that Playwright may add
args=()
for arg in "$@"; do
  if [[ "$arg" != --no-color* ]]; then
    args+=("$arg")
  fi
done
exec trunk serve "${args[@]}"

