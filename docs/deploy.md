# Deploying the prototype

There are two ways to put Zidara online.

- **The demo link (static, free):** the whole school runs in each visitor's browser. Any
  static host works. See "The demo link" at the end.
- **The real server:** below. It needs a VPS with PostgreSQL.

One Rust binary serves the API and the web app (`ZIDARA_WEB_DIR`). It needs PostgreSQL 16 and,
optionally, an Anthropic API key for the assessor (without one, the rubric marks). Measured on
2 CPU cores: p95 16–32 ms for learner views, 33–71 ms for answers (across runs), at 150 views/s and 50 answers/s
from 300 learners (`make load`). A small VPS is enough.

## 1. Build (on any Linux machine with the toolchain)

Needs Rust 1.98 or newer from rustup (with the `wasm32-unknown-unknown` target), Node and pnpm.
A distribution's own `cargo` is usually older and fails with "`resolver` setting `3` is not
valid": put `~/.cargo/bin` first in `PATH`.

```bash
scripts/build-wasm.sh                      # the kernel for the browser
(cd web && pnpm install --frozen-lockfile && npx vite build)
cargo build --release -p zidara-server
# Ship: target/release/zidara-server and web/dist/
```

## 2. The database

Create a database, and an owner role for migrations. The app role `zidara_app` is created by
the first migration; it gets a password from you, and it never owns anything (row-level
security depends on that).

Every command needs `ZIDARA_SECRET` (at least 32 bytes, or it refuses to start); use the one
you will put in `/etc/zidara.env`.

```bash
export ZIDARA_SECRET=$(openssl rand -hex 32)   # keep it for /etc/zidara.env
sudo -u postgres createdb zidara
sudo -u postgres psql -c "alter role postgres password '<owner password>'"
export OWNER_URL='postgres://postgres:<owner password>@127.0.0.1:5432/zidara'
DATABASE_URL=$OWNER_URL zidara-server migrate
sudo -u postgres psql -d zidara -c "alter role zidara_app password '<app password>'"
DATABASE_URL=$OWNER_URL zidara-server seed   # the demo school
```

Rehearsed on 18 October 2026 against a fresh PostgreSQL 16 with the release build: migrate,
seed, serve, the two `curl` checks and `pnpm smoke` all passed (the owner role connected over
TCP without a password there; the password step above is the standard equivalent).

## 3. The service

`/etc/zidara.env` (mode 600):

```
DATABASE_URL=postgres://zidara_app:<app password>@127.0.0.1:5432/zidara
ZIDARA_SECRET=<at least 32 random bytes, e.g. openssl rand -hex 32>
ZIDARA_ADDR=127.0.0.1:8080
ZIDARA_WEB_DIR=/srv/zidara/web
ZIDARA_DEMO_LOGIN=1            # demo sign-in by picking a person; remove for real users
# ANTHROPIC_API_KEY=...        # optional: Claude marks open answers, the rubric otherwise
# ZIDARA_ASSESSOR_MODEL=claude-haiku-5-5   # optional: a cheaper model than the default
```

`/etc/systemd/system/zidara.service`:

```ini
[Unit]
Description=Zidara prototype
After=network-online.target postgresql.service

[Service]
EnvironmentFile=/etc/zidara.env
ExecStart=/srv/zidara/zidara-server serve
User=zidara
Restart=on-failure
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true

[Install]
WantedBy=multi-user.target
```

## 4. HTTPS (Caddy)

```
zidara.example.org {
    encode zstd gzip
    reverse_proxy 127.0.0.1:8080
}
```

The service worker needs HTTPS (any real domain works with Caddy's automatic certificates).

## 5. Check

```bash
curl -s -o /dev/null -w "%{http_code}\n" https://zidara.example.org/          # 200
curl -s -o /dev/null -w "%{http_code}\n" https://zidara.example.org/api/me    # 401
```

Then, from any machine with the repository, the browser check (it signs in as a demo learner,
reads, and reloads offline; it changes no data):

```bash
cd web && ZIDARA_URL=https://zidara.example.org pnpm smoke     # 4 passed
```

## On a shared server (how s1 runs it)

The hosted sandbox (ADR 0010) runs on a server that other projects share. The files are in
`deploy/s1/`. What differs from the steps above:

- **Build here, copy there.** The release binary and `web/dist` are built on the development
  machine (the same Ubuntu 24.04 and glibc 2.39) and copied with `scp`; nothing compiles on the
  shared server.
- **Its own roles.** Never change the `postgres` role's password; other projects use it. Create
  an owner role for Zidara instead:

  ```bash
  sudo -u postgres psql -c "create role zidara_owner login createrole bypassrls password '<owner password>'" \
                        -c "create database zidara owner zidara_owner"
  ```

  `bypassrls` is needed because every table forces row-level security, even on its owner: with
  a non-superuser owner, `seed` fails with "new row violates row-level security policy for
  table tenants". Only `migrate` and `seed`, run by hand, use this role; the app connects as
  `zidara_app`, which bypasses nothing.
- **Its own user, port and files:** system user `zidara`, `/opt/zidara`, `127.0.0.1:8850`, and
  `/etc/zidara.env` (root only). The unit caps memory (512 MB) and CPU (one core).
- **Caddy:** one file in `/etc/caddy/sites.d/`, then `caddy validate` and `systemctl reload
  caddy`. Never `restart`: it drops every other site's connections.
- **A clean school every night:** `zidara-reset.timer` runs `select private.reset_demo()` at
  03:00 UTC: events and guests go, the seeded people and classes stay (ADR 0012). The server
  keeps nothing in memory, so this is safe while it runs.
- **Never run `tools/load` against it.**

Updating it (a release with a new migration also runs `migrate` as the owner; to start the
school from scratch, empty it and `seed` again):

```bash
scp target/release/zidara-server s1.nasa9.com:/tmp/ && ssh s1.nasa9.com \
  'sudo install -m 755 /tmp/zidara-server /opt/zidara/ && sudo systemctl restart zidara'
cd web && ZIDARA_URL=https://zidara.ta9in.com pnpm smoke     # 4 passed
```

The address is a DNS-only A record (`zidara.ta9in.com` → the server), so Caddy obtains and
renews its own certificate as for the other sites.

## Before real children use it

The demo sign-in lets anyone act as anyone; it is for showing the prototype only. Real use
needs OIDC sign-in, a privacy review, and the data-protection rules of the country of use.

## The browser-only build

```bash
scripts/build-wasm.sh
(cd web && pnpm install --frozen-lockfile && pnpm build:demo)   # web/dist-demo
```

`web/dist-demo` is the whole school running in the browser, as static files. It is not
published: the one public address is https://zidara.ta9in.com (the owner dropped the
zidara.pages.dev copy on 5 November 2026). To serve it anyway, put it at the root of a domain
with HTTPS (the service worker expects `/`) and test it with
`cd web && ZIDARA_URL=<url> pnpm demo`.

