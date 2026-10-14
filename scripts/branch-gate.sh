#!/usr/bin/env bash
# Fails unless every source branch recorded in an lcov file was taken by some test.
# It reads the per-line BRDA records, which merge every compiled copy of a generic function.
# The BRF/BRH totals count each copy separately, so a branch one copy never takes would fail
# the build even when a test covers it through another copy.
set -euo pipefail
lcov="${1:?usage: branch-gate.sh <lcov.info>}"
untaken=$(awk -F'[:,]' '/^SF:/{file=$2} /^BRDA:/ && ($5=="-" || $5=="0"){print file":"$2}' "$lcov" | sort -u)
total=$(grep -c '^BRDA:' "$lcov" || true)
echo "branches: $(( total - $(printf '%s' "$untaken" | grep -c . || true) ))/${total} taken"
if [ -n "$untaken" ]; then
  echo "untaken branches:" >&2
  echo "$untaken" >&2
  exit 1
fi
