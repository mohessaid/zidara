# The gauntlet. Each target is one proof; `make gauntlet` runs them all.
KERNEL := -p zidara-kernel -p zidara-frameworks
WASM_RUNNER := CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
COVERAGE_SCOPE := --ignore-filename-regex 'zidara-acceptance|tests/'

.PHONY: gauntlet lint test acceptance wasm targets coverage mutants

gauntlet: lint test acceptance wasm targets coverage mutants

lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo nextest run --workspace --exclude zidara-acceptance

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
	cargo mutants $(KERNEL) --test-workspace=true
