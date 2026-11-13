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
if [ "$n" = 0 ]; then
    echo "EMPTY: neither front end printed a line"
    exit 1
fi
echo "SAME: $n lines, native and node agree"

# Without the control, the lines must also be the committed ones, so two
# front ends that drifted together still fail.
if [ "${#features[@]}" = 0 ]; then
    if ! diff -q out/native.txt digests-native.txt > /dev/null; then
        echo "DRIFT: the lines differ from the committed digests-native.txt"
        diff out/native.txt digests-native.txt | head -4
        exit 1
    fi
    echo "GOLDEN: the lines equal the committed digests-native.txt"
fi
