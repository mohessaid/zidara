---
name: girth-wasm
description: Use when exposing the Zidara kernel to JavaScript through wasm-bindgen, building or sizing the .wasm, testing it from Node, or generating TypeScript types from Rust with ts-rs.
metadata:
  girth:
    sources:
      - path-or-repo: avbel/ai-skills
        sections: ["`#[wasm_bindgen]`", "JavaScript Interop and Memory", "Testing", "Performance", "Binary Size", "Panics"]
      - path-or-repo: pluginagentmarketplace/custom-plugin-rust
        sections: ["Structs and Complex Types", "Size Optimization", "Common Issues"]
      - path-or-repo: https://docs.rs/ts-rs/12.0.1
        sections: ["Deriving TS", "Environment variables", "Serde compatibility", "Feature flags"]
    crafted_for: G3 wasm bridge - stateful kernel shell in wasm, serde JSON across the boundary, Node tests, ts-rs types
    created: 2026-10-16
    last_used: 2026-10-16
---
## Exports <!-- src: rust-wasm-conventions#-wasm-bindgen- -->
- Annotate exported `pub struct` and `impl` blocks with `#[wasm_bindgen]`; constructors as `#[wasm_bindgen(constructor)] pub fn new(..) -> Self`.
- Keep Rust-only helpers in impl blocks without `#[wasm_bindgen]`.

## Crossing the boundary <!-- src: rust-wasm-conventions#javascript-interop-and-memory, rust-wasm#structs-and-complex-types -->
- Minimize copying across the boundary: keep large state (the event log, lesson, frameworks) in wasm memory; return only what the caller needs.
- Complex types: serde. Either JSON strings (`serde_json`) or `serde_wasm_bindgen::to_value`. Errors become `JsValue::from_str(&e.to_string())` via `Result<_, JsValue>`.
- Type mismatch on complex types → use serde_wasm_bindgen. Import errors → `--target` must match the runtime (`web` for browsers, `nodejs` for Node). <!-- src: rust-wasm#common-issues -->

## Testing <!-- src: rust-wasm-conventions#testing -->
- `#[wasm_bindgen_test]` for wasm-targeted tests. Structure tests with helper builders for known input and expected output states.

## Size and panics <!-- src: rust-wasm-conventions#binary-size, rust-wasm-conventions#panics, rust-wasm#size-optimization -->
- Release: `lto = true`, `opt-level = "s"` or `"z"`, `codegen-units = 1`; consider `panic = "abort"`.
- Always measure the release build, never a debug one. Post-process with `wasm-opt -Os`/`-Oz`, and use `twiggy` to see what takes space.
- Avoid `format!`/`to_string` in release hot paths.
- In production wasm paths, replace `unwrap()`/`expect()` with explicit errors, unless abort is intended.

## ts-rs 12 <!-- src: https://docs.rs/ts-rs/12.0.1 -->
- `#[derive(ts_rs::TS)]` plus `#[ts(export)]`. Running `cargo test` writes the files (export runs as generated tests); `TS::export_all` does it without tests.
- `TS_RS_EXPORT_DIR` sets the output directory (default `./bindings`); set it in `.cargo/config.toml`. `TS_RS_LARGE_INT` sets the type for i64/u64 (default `bigint`, `number` when values fit).
- `serde-compat` (default) honours `rename`, `rename_all`, `rename_all_fields`, `tag`, `content`, `untagged`, `skip`, `skip_serializing(_if)` (only with `#[serde(default)]`), `flatten` and `default`. `skip_deserializing` is ignored; use `#[ts(skip)]`.
- Types that don't implement `TS`, or serialize differently from their shape (`try_from`/`into`): `#[ts(type = "..")]` or `#[ts(as = "..")]`.
- Features: `uuid-impl`, `serde-json-impl`, `format`.

## Record
After each deliverable, run the taste context's `made` command with its defining choices.
