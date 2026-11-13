#!/usr/bin/env nutshell
# Runs the probes whose outputs FINDINGS.md quotes and writes each to a
# committed file next to this script, with the exit code on the last line.
#
#   output-tests.txt              the kernel tests as they stand
#   output-mutant-half-up.txt     the same tests with ties rounding up
#   output-omit-panic-handler.txt the wasm front end without a panic handler
#   output-panic-unwind.txt       the wasm front end with panic = "unwind"
#   output-wasm.txt               size, exports and imports of the module
set -u

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"
node=/opt/node22/bin/node

# Runs a command, keeps its output in a file, and ends the file with its exit code.
record() {
    local file="$1"; shift
    "$@" > "$file" 2>&1
    echo "exit=$?" >> "$file"
}

record output-tests.txt cargo test --release --color never -p wasm-digest-kernel
record output-mutant-half-up.txt cargo test --release --color never -p wasm-digest-kernel --features mutant-half-up
record output-omit-panic-handler.txt cargo build --release --color never -p wasm-digest-wasm --target wasm32-unknown-unknown --features omit-panic-handler
record output-panic-unwind.txt cargo build --release --color never -p wasm-digest-wasm --target wasm32-unknown-unknown --config 'profile.release.panic="unwind"'

# The module measured is the one the sketch ships: panic abort, handler present.
cargo build --release -q -p wasm-digest-wasm --target wasm32-unknown-unknown
target="$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory)"
record output-wasm.txt "$node" inspect.mjs "$target/wasm32-unknown-unknown/release/wasm_digest_wasm.wasm"
