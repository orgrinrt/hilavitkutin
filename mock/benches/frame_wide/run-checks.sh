#!/bin/sh
# Everything that has to pass before a timing from this member counts.
#
# This member declares its own `[workspace]`, as every bench member here does,
# so it is not among `mock/Cargo.toml`'s members and `cargo mock test` never
# builds it. Its tests passed because somebody ran them by hand, which is not a
# state anything can be checked in. This is the entry point instead: one
# command, from this directory, silent on success.
#
# Four checks, in the order a failure is cheapest to read. The grouping test is
# compile-time analysis and runs in milliseconds. The support crate's unit tests
# are in-process. The checks gate drives this script's own stub a few times and
# costs a second. The negative control spawns harness workers and takes a couple
# of minutes, nearly all of it the per-worker counter calibration (ticket
# 3ec38cb5c7c8).
#
# The gate test never runs this script: it points the driver at a stub in a
# directory of its own, so there is no recursion here.
#
# usage: ./run-checks.sh [-v]
#   -v  print each command and its output rather than only failures

set -eu

verbose=0
if [ "${1:-}" = "-v" ]; then
    verbose=1
fi

dir=$(cd "$(dirname "$0")" && pwd)
log=$(mktemp)
trap 'rm -f "$log"' EXIT INT TERM

run() {
    label="$1"
    shift
    if [ "$verbose" = 1 ]; then
        echo "+ $*"
        "$@" || { echo "run-checks: $label failed" >&2; exit 1; }
        return
    fi
    if ! "$@" >"$log" 2>&1; then
        echo "run-checks: $label failed" >&2
        cat "$log" >&2
        exit 1
    fi
}

cd "$dir"
CARGO_INCREMENTAL=0
export CARGO_INCREMENTAL

run "the build" cargo build --release --workspace --all-targets
run "the grouping test" cargo test --release --test engine_grouping
run "the support crate's unit tests" cargo test --release -p frame-wide-support
run "the checks gate" cargo test --release --test checks_gate
run "the negative control" cargo test --release --test negative_control

if [ "$verbose" = 1 ]; then
    echo "run-checks: ok"
fi
