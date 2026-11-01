# 0010. A hosted sandbox on a shared server

Date: 2026-11-01. Status: accepted.

## Context
The browser demo (0009) cannot show the server: PostgreSQL row-level security, several devices
syncing through one log, or a live class. The owner offered a server (`s1.nasa9.com`, Ubuntu
24.04, 2 cores, 8 GB) that already hosts other projects behind one Caddy. Its rules: no Docker,
and never disturb the projects already there. A public server with the demo sign-in lets anyone
act as anyone and write to the database.

## Decision
- The release binary and the web build are made on the development machine (same Ubuntu and
  glibc) and copied over; nothing is compiled on the shared server.
- It runs under systemd as its own system user `zidara`, files in `/opt/zidara`, on
  `127.0.0.1:8850`, with the environment in `/etc/zidara.env` (root only).
- It uses its own database `zidara` in the existing PostgreSQL 17 cluster, owned by its own role
  `zidara_owner`. No other project's role or database is touched.
- Caddy gets one file, `/etc/caddy/sites.d/zidara.caddy`, the convention the server already
  uses; changes are checked with `caddy validate` and applied with `reload`, never `restart`.
- The address is `zidara.72-61-103-134.sslip.io` until a DNS record exists (the Cloudflare
  login available can read the `nasa9.com` zone but not edit it).
- **No model marks open answers on the public sandbox**: no API key is set, so the rubric marks,
  as in the browser demo. A public sign-in that can trigger paid calls is a cost and abuse
  surface; the model path is proven by the server's tests.
- A systemd timer empties the school every night (`truncate events, applied_envelopes`; the
  people and classes stay), so each day starts clean.
- Load tests (`tools/load`) are never run against it.

## Consequences
- Judges get a second link that runs the real server: two phones in one class see the teacher's
  decisions, and the database keeps the classes apart.
- Anyone can still change what other visitors see during the day; the sign-in page says it is a
  demo with invented people, and the night reset limits how long it lasts.
- Moving to the owner's own domain is one DNS record and one line in the Caddy file.
