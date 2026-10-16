#!/usr/bin/env bash
# Builds the kernel bridge for the browser (web/src/lib/kernel/pkg) and for Node tests
# (target/wasm-node), optimises it with binaryen's wasm-opt, and prints the sizes.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
cargo build -q -p zidara-wasm --release --target wasm32-unknown-unknown
raw="target/wasm32-unknown-unknown/release/zidara_wasm.wasm"
web="web/src/lib/kernel/pkg"
node="target/wasm-node"
rm -rf "$web" "$node"
wasm-bindgen --target web --out-dir "$web" "$raw"
wasm-bindgen --target nodejs --out-dir "$node" "$raw"
for dir in "$web" "$node"; do
  npx -y -q -p binaryen@132 wasm-opt -Os --enable-bulk-memory --enable-nontrapping-float-to-int \
    "$dir/zidara_wasm_bg.wasm" -o "$dir/zidara_wasm_bg.wasm"
done
bytes=$(wc -c < "$web/zidara_wasm_bg.wasm")
gz=$(gzip -9 -c "$web/zidara_wasm_bg.wasm" | wc -c)
echo "zidara_wasm_bg.wasm: ${bytes} bytes, ${gz} bytes gzipped"
