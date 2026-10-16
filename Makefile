# The gauntlet. Each target is one proof; `make gauntlet` runs them all.
# Server tests start labelled PostgreSQL containers (Docker required); they are removed after.
KERNEL := -p zidara-kernel -p zidara-frameworks
PORTABLE := $(KERNEL) -p zidara-wasm
WASM_RUNNER := CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
# main.rs is CLI wiring, verified by running the binary (make serve).
# shell.rs is the wasm-bindgen face; JsError exists only on wasm32, so it is proven by
# crates/zidara-wasm/tests/shell.rs under `make wasm`.
COVERAGE_SCOPE := --ignore-filename-regex 'zidara-acceptance|tests/|zidara-server/src/main.rs|zidara-wasm/src/shell.rs'

.PHONY: gauntlet lint test acceptance wasm targets bridge coverage mutants clean-containers db-up db-down serve

gauntlet: lint test acceptance wasm targets bridge coverage mutants

lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo nextest run --workspace --exclude zidara-acceptance; status=$$?; $(MAKE) -s clean-containers; exit $$status

acceptance:
	cargo test -p zidara-acceptance --test acceptance

wasm:
	$(WASM_RUNNER) cargo test $(PORTABLE) --target wasm32-unknown-unknown

targets:
	cargo check $(PORTABLE) --target aarch64-linux-android
	cargo check $(PORTABLE) --target aarch64-apple-ios

# The optimised wasm build, replayed in Node against the golden session.
bridge:
	scripts/build-wasm.sh
	node --test web/tests/bridge.test.mjs

coverage:
	cargo +nightly llvm-cov --workspace --branch $(COVERAGE_SCOPE) \
	  --fail-uncovered-lines 0 --fail-uncovered-functions 0 \
	  --lcov --output-path target/lcov.info; status=$$?; $(MAKE) -s clean-containers; exit $$status
	scripts/branch-gate.sh target/lcov.info

# Split so the Docker-backed server suite never runs once per mutant.
mutants:
	cargo mutants $(KERNEL) --test-package zidara-kernel,zidara-frameworks,zidara-acceptance -j 4
	cargo mutants -p zidara-server -f 'crates/zidara-server/src/{visibility,xapi,assessor,auth,config}.rs' -j 4 -- --test pure

clean-containers:
	docker ps -aq --filter label=org.zidara.test=1 | xargs -r docker rm -f > /dev/null

# Running the server locally. The owner migrates and seeds; the server connects as zidara_app.
DEV_OWNER := postgres://postgres:postgres@127.0.0.1:55432/zidara
DEV_APP := postgres://zidara_app:app@127.0.0.1:55432/zidara
DEV_SECRET := local-development-secret-not-for-production

db-up:
	docker compose up -d --wait db
	DATABASE_URL=$(DEV_OWNER) ZIDARA_SECRET=$(DEV_SECRET) cargo run -q -p zidara-server -- migrate
	docker compose exec -T db psql -q -U postgres -d zidara -c "alter role zidara_app password 'app'"
	-DATABASE_URL=$(DEV_OWNER) ZIDARA_SECRET=$(DEV_SECRET) cargo run -q -p zidara-server -- seed 2> /dev/null

serve: db-up
	DATABASE_URL=$(DEV_APP) ZIDARA_SECRET=$(DEV_SECRET) ZIDARA_DEMO_LOGIN=1 cargo run -q -p zidara-server -- serve

db-down:
	docker compose down
