#!/usr/bin/env bash
# A fresh demo school for end-to-end tests: resets the zidara_e2e database in the compose
# PostgreSQL, migrates and seeds it, then runs the server on 127.0.0.1:18080 (rubric assessor).
set -euo pipefail
cd "$(dirname "$0")/.."
docker compose up -d --wait db > /dev/null
psql() { docker compose exec -T db psql -q -v ON_ERROR_STOP=1 -U postgres "$@"; }
psql -d postgres -c "drop database if exists zidara_e2e with (force)" -c "create database zidara_e2e"
owner="postgres://postgres:postgres@127.0.0.1:55432/zidara_e2e"
secret="end-to-end-test-secret-not-for-production"
cargo build -q -p zidara-server
DATABASE_URL="$owner" ZIDARA_SECRET="$secret" target/debug/zidara-server migrate
psql -d zidara_e2e -c "alter role zidara_app password 'app'"
DATABASE_URL="$owner" ZIDARA_SECRET="$secret" target/debug/zidara-server seed
exec env DATABASE_URL="postgres://zidara_app:app@127.0.0.1:55432/zidara_e2e" ZIDARA_SECRET="$secret" \
  ZIDARA_DEMO_LOGIN=1 ZIDARA_ADDR=127.0.0.1:18080 target/debug/zidara-server serve
