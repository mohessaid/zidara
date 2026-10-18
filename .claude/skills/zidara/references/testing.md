# Testing: the gauntlet

Distilled from Trail of Bits' property-based and mutation-testing skills and the ECC
rust-testing skill, then checked against what this repo actually runs.

## The loop

1. Write the test. Name the production change that would make it fail before you write it.
2. Run it and read the failure: it must fail because the behaviour is missing, not because of
   a typo.
3. Write the least code that passes. Run the crate's whole suite, not just the file.
4. A test that passed before the code existed proves nothing. Fix the test.
5. A test written after the code has to be proven by sabotage. Break the code on purpose,
   watch the test fail, then restore. Check that the sabotage actually landed: `cargo fmt` can
   make a `sed` miss.

## Where tests live

- `crates/<crate>/tests/<module>.rs`: one file per module, testing the public API only. Each
  file starts with:
  ```rust
  #[cfg(target_arch = "wasm32")]
  use wasm_bindgen_test::wasm_bindgen_test as test;
  ```
  so the same tests run on wasm32.
- `crates/zidara-kernel/tests/support/mod.rs` holds fixtures (`user`, `learner`, `teacher`,
  `agent`, `event`, `evidence`, `lesson`, `env`, `TestFramework`) and `check`, the property
  runner.
- `features/*.feature` holds user stories, run by `crates/zidara-acceptance` (cucumber,
  native only). Every new behaviour a teacher or learner would notice gets a scenario.
- Async server tests (M2) use `#[tokio::test]`. Database tests run against real PostgreSQL in
  Docker, never a mock.

## Property tests (proptest)

Use `support::check(strategy, |value| { ...; Ok(()) })`. It runs with a fixed ChaCha seed, 256
cases and no failure file, so results are reproducible and it works on wasm32.

Assert the strongest property the code supports, weakest to strongest:
no crash < type preservation < invariant < idempotence < roundtrip/oracle.

| Property | Shape | Zidara examples |
|---|---|---|
| Roundtrip | `decode(encode(x)) == x` | events, commands, envelopes through JSON |
| Invariant | holds for every input | a mean lies between min and max; graph order puts prerequisites first |
| Commutativity | order does not matter | fold over a log in any arrival order (distinct times) |
| Idempotence | `f(f(x)) == f(x)` | replaying an envelope twice changes nothing |
| Oracle | `new(x) == reference(x)` | phone fold vs server fold after sync |

Two ways a property asserts nothing:
- **Tautology**: recomputing the implementation in the test.
- **Vacuity**: filtering away most inputs.

Put constraints in the strategy (`prop_flat_map`, ranges, `btree_set` for distinct values),
not in filters. Pin known edges with plain example tests: empty, one element, zero, maximum
(`u32::MAX`), exact boundaries (7 999 vs 8 000).

When a property fails, read the shrunk counterexample. First ask whether the property is
wrong, or whether the code is.

## Coverage

`make coverage` runs nightly `cargo llvm-cov --branch`, excluding `tests/` and the acceptance
crate. The gate is:
- 0 uncovered lines and 0 uncovered functions (`--fail-uncovered-*`);
- 0 untaken branches (`scripts/branch-gate.sh`).

The branch gate reads per-line BRDA records, because LLVM's BRF/BRH totals count each copy of
a generic function separately. A gap means a missing test. Add the test; never exclude the code.

## Mutation testing (cargo-mutants)

`make mutants` must report 0 missed. For a survivor:
1. Find the input whose result the mutation changes, and write the test that pins it.
2. Only if no such input exists is the mutant equivalent. Prove it with all five checks:
   - Types: e.g. `x > 0` vs `x != 0` on unsigned types.
   - Reachability: dead code differs from untested code. Untested code is a finding.
   - Downstream use: is the mutated value ever read?
   - Semantics: is the result identical for every input within the types?
   - Distinguishing input: you tried to construct one and could not.
3. Exclude a proven equivalent in `.cargo/mutants.toml`, with the reason on the line above,
   and add a JOURNAL entry.

Unreachable defensive code fails both gates by design. Prefer structure that makes the bad
state impossible (the permission table, typed states) over a check nobody can trigger.

## Portability

`make wasm targets`: the kernel and frameworks test suites run on wasm32 under
`wasm-bindgen-test-runner` 0.2.129 (Node), plus `cargo check` for Android and iOS.

Known traps:
- `uuid` with `v7` does not build on plain wasm32. Use it as a type only.
- Test-only `getrandom` needs the `wasm_js` feature.
- `proptest` needs `default-features = false, features = ["std"]` and the fixed-seed runner.

## This machine

- `/usr/bin/cargo` (1.75) comes before rustup's in some shells and fails with "`resolver`
  setting `3` is not valid". Prefix commands with `PATH=$HOME/.cargo/bin:$PATH`.
- The `rtk` wrapper compresses output and can hide Playwright's results and raw diffs. Read
  results from `PLAYWRIGHT_JSON_OUTPUT_NAME=<file> rtk proxy npx playwright test
  --reporter=json`; use `rtk proxy git diff` for a diff a tool will parse (`cargo mutants
  --in-diff`).
- Find a server to stop by its port (`ss -ltnp`), never with `pkill -f`.
