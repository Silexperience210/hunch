#!/usr/bin/env bash
# verify-hips.sh — confirm each HIP file has required sections
# Required sections: ## Abstract, ## Motivation, ## Specification, ## References
# CI-safe: silently passes if no HIPs exist yet (Plan 02 hasn't shipped them).
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if [ ! -d docs ] || ! ls docs/HIP-*.md >/dev/null 2>&1; then
  echo "INFO: no HIP files yet (docs/HIP-N.md) — skipping (Plan 02 deliverable)"
  exit 0
fi

required_sections=("## Abstract" "## Motivation" "## Specification" "## References")
fail=0

for hip in docs/HIP-*.md; do
  for section in "${required_sections[@]}"; do
    if ! grep -q "^${section}" "$hip"; then
      echo "FAIL: $hip missing section '$section'" >&2
      fail=1
    fi
  done
done

if [ "$fail" -eq 1 ]; then
  exit 1
fi

echo "PASS: verify-hips.sh — all HIPs have required sections"
