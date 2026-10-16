# Deploying the prototype

One Rust binary serves the API and the web app (`ZIDARA_WEB_DIR`). It needs PostgreSQL 16 and,
optionally, an Anthropic API key for the assessor (without one, the rubric marks). Measured on
2 CPU cores: p95 30 ms for learner views, 70 ms for answers, at 150 views/s and 50 answers/s
from 300 learners (`make load`). A small VPS is enough.

## 1. Build (on any Linux machine with the toolchain)

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

```bash
sudo -u postgres createdb zidara
DATABASE_URL=postgres://postgres@/zidara ZIDARA_SECRET=x zidara-server migrate
sudo -u postgres psql -d zidara -c "alter role zidara_app password '<app password>'"
DATABASE_URL=postgres://postgres@/zidara ZIDARA_SECRET=x zidara-server seed   # the demo school
```

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

## Before real children use it

The demo sign-in lets anyone act as anyone; it is for showing the prototype only. Real use
needs OIDC sign-in, a privacy review, and the data-protection rules of the country of use.
