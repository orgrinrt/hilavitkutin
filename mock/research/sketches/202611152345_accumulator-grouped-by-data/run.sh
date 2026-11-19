#!/usr/bin/env nutshell
# Builds the sketch and runs its tests, keeping what the compiler and the tests
# printed in `out/` beside it, with the build directory, the sketch's own path
# and the repository's path written as `<target>`, `<sketch>` and `<repo>`.
#
#   CARGO_TARGET_DIR=<dir> ./run.sh [<name>]
#
# `<name>` names the output files, `out/<name>-build.txt` and
# `out/<name>-test.txt`; `run` by default. Exits with the test run's status.

set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
name="${1:-run}"
target="${CARGO_TARGET_DIR:?set CARGO_TARGET_DIR}"
mkdir -p "$here/out"

repo="$(cd "$here/../../../.." && pwd)"
scrub() { sed -e "s#$target#<target>#g" -e "s#$here#<sketch>#g" -e "s#$repo#<repo>#g" -e "s#$HOME#<home>#g"; }

cd "$here" || exit 1
cargo +nightly-2026-05-28 test --no-run 2>&1 | scrub > "out/$name-build.txt"
build="${PIPESTATUS[0]}"
if [ "$build" -ne 0 ]; then
    echo "build failed, see out/$name-build.txt"
    exit "$build"
fi
cargo +nightly-2026-05-28 test -- --test-threads=1 --nocapture 2>&1 | scrub > "out/$name-test.txt"
status="${PIPESTATUS[0]}"
tail -n 3 "out/$name-test.txt"
exit "$status"
