# The gauntlet. Each target is one proof; `make gauntlet` runs them all.
KERNEL := -p zidara-kernel -p zidara-frameworks
WASM_RUNNER := CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
COVERAGE_SCOPE := --ignore-filename-regex 'zidara-acceptance|tests/'

.PHONY: gauntlet lint test acceptance wasm targets coverage mutants clean-containers

gauntlet: lint test acceptance wasm targets coverage mutants

lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

# Server tests start labelled PostgreSQL containers (Docker required); they are removed after.
test:
	cargo nextest run --workspace --exclude zidara-acceptance; status=$$?; $(MAKE) -s clean-containers; exit $$status

clean-containers:
	docker ps -aq --filter label=org.zidara.test=1 | xargs -r docker rm -f > /dev/null

acceptance:
	cargo test -p zidara-acceptance --test acceptance

wasm:
	$(WASM_RUNNER) cargo test $(KERNEL) --target wasm32-unknown-unknown

targets:
	cargo check $(KERNEL) --target aarch64-linux-android
	cargo check $(KERNEL) --target aarch64-apple-ios

coverage:
	cargo +nightly llvm-cov --workspace --branch $(COVERAGE_SCOPE) \
	  --fail-uncovered-lines 0 --fail-uncovered-functions 0 \
	  --lcov --output-path target/lcov.info
	scripts/branch-gate.sh target/lcov.info

mutants:
	cargo mutants $(KERNEL) --test-workspace=true -j 4
