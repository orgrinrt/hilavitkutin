#!/usr/bin/env nutshell
# Builds the native binary and the wasm module, runs both, and compares their
# output line by line. Exits non-zero on the first differing line.
#
#   run.sh                run the sketch as it stands
#   run.sh --control      run the negative control: the digest absorbs the
#                         line length as a pointer-width integer
set -eu

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"
node=/opt/node22/bin/node
features=()
if [ "${1:-}" = "--control" ]; then
    features=(--features control-usize-len)
fi

cargo build --release -q -p wasm-digest-native "${features[@]}"
cargo build --release -q -p wasm-digest-wasm --target wasm32-unknown-unknown "${features[@]}"

target="$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory)"
mkdir -p out
"$target/release/wasm-digest-native" > out/native.txt
"$node" run.mjs "$target/wasm32-unknown-unknown/release/wasm_digest_wasm.wasm" 2 > out/node.txt

exec 3< out/native.txt
exec 4< out/node.txt
n=0
while true; do
    got_a=0; got_b=0
    IFS= read -r a <&3 && got_a=1
    IFS= read -r b <&4 && got_b=1
    if [ "$got_a" = 0 ] && [ "$got_b" = 0 ]; then
        break
    fi
    n=$((n + 1))
    if [ "$got_a" != "$got_b" ] || [ "$a" != "$b" ]; then
        echo "DIFFER at line $n"
        echo "  native: ${a:-<end>}"
        echo "  node:   ${b:-<end>}"
        exit 1
    fi
done
echo "SAME: $n lines, native and node agree"
