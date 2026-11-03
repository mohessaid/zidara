#!/usr/bin/env bash
# Builds the web shell, serves it, runs the Lighthouse gate (performance >= 90, accessibility =
# 100, mobile), and stops the server it started, by PID.
set -euo pipefail
cd "$(dirname "$0")/../web"
npx vite build > /dev/null
node_modules/.bin/vite preview --port 4174 --strictPort --host 127.0.0.1 > /dev/null 2>&1 &
server=$!
trap 'kill $server 2> /dev/null || true' EXIT
for _ in $(seq 1 50); do curl -s -o /dev/null http://127.0.0.1:4174/ && break; sleep 0.2; done
# The front page (ADR 0014) and the game behind it are gated alike.
node scripts/lighthouse.mjs http://127.0.0.1:4174/
node scripts/lighthouse.mjs "http://127.0.0.1:4174/#play"
