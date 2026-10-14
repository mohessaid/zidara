#!/usr/bin/env bash
# Fails unless every branch recorded in an lcov file was taken.
set -euo pipefail
lcov="${1:?usage: branch-gate.sh <lcov.info>}"
read -r found hit < <(awk -F: '/^BRF:/{f+=$2} /^BRH:/{h+=$2} END{print f+0, h+0}' "$lcov")
echo "branches: ${hit}/${found}"
if [ "$found" -ne "$hit" ]; then
  echo "uncovered branches:" >&2
  awk -F'[:,]' '/^SF:/{file=$2} /^BRDA:/ && ($5=="-" || $5=="0"){print file":"$2}' "$lcov" | sort -u >&2
  exit 1
fi
