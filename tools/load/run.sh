#!/usr/bin/env bash
# Fresh demo school, release server pinned to 2 CPU cores, k6 load; stops the server after.
set -euo pipefail
cd "$(dirname "$0")/../.."
docker compose up -d --wait db > /dev/null
psql() { docker compose exec -T db psql -q -v ON_ERROR_STOP=1 -U postgres "$@"; }
psql -d postgres -c "drop database if exists zidara_load with (force)" -c "create database zidara_load"
owner="postgres://postgres:postgres@127.0.0.1:55432/zidara_load"
secret="load-test-secret-not-for-production-use"
cargo build -q --release -p zidara-server
DATABASE_URL="$owner" ZIDARA_SECRET="$secret" target/release/zidara-server migrate
psql -d zidara_load -c "alter role zidara_app password 'app'"
DATABASE_URL="$owner" ZIDARA_SECRET="$secret" target/release/zidara-server seed
# A school: 10 classes of 30 learners, the agent serving every class. Ids are md5-derived so
# k6 can compute them: classes md5("load-class-<c>"), learners md5("load-learner-<c>-<i>").
psql -d zidara_load -c "
  insert into cohorts (id, tenant_id, name, lesson_id, framework)
    select md5('load-class-' || c)::uuid, '00000000-0000-0000-0000-00007e4a0001', 'Load class ' || c, 'equivalent-fractions', 'mastery'
    from generate_series(1, 10) c;
  insert into users (id, tenant_id, display_name, kind)
    select md5('load-learner-' || c || '-' || i)::uuid, '00000000-0000-0000-0000-00007e4a0001', 'Learner ' || c || '-' || i, 'person'
    from generate_series(1, 10) c, generate_series(1, 30) i;
  insert into memberships (user_id, cohort_id, role)
    select md5('load-learner-' || c || '-' || i)::uuid, md5('load-class-' || c)::uuid, 'learner'
    from generate_series(1, 10) c, generate_series(1, 30) i;
  insert into memberships (user_id, cohort_id, role)
    select '00000000-0000-0000-0000-000000a60001', md5('load-class-' || c)::uuid, 'assessment_agent'
    from generate_series(1, 10) c;"
DATABASE_URL="postgres://zidara_app:app@127.0.0.1:55432/zidara_load" ZIDARA_SECRET="$secret" ZIDARA_DEMO_LOGIN=1 \
  ZIDARA_ADDR=127.0.0.1:18090 taskset -c 0,1 target/release/zidara-server serve > /dev/null &
server=$!
trap 'kill $server 2> /dev/null || true; docker compose stop db > /dev/null' EXIT
for _ in $(seq 1 50); do curl -s -o /dev/null http://127.0.0.1:18090/api/me && break; sleep 0.2; done
k6 run --quiet "${K6_SCRIPT:-tools/load/k6.js}"
