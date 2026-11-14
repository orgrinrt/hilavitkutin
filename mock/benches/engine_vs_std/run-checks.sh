#!/bin/sh
# Everything that has to pass before a timing from this member counts: the
# build of the driver and every arm, every arm of every workload against the
# reference digest (the support crate's tests), and the verdict's own tests.
#
# usage: ./run-checks.sh [-v]
#   -v  print each command and its output rather than only failures

set -eu

cd "$(dirname "$0")"
. ../checks.sh

run "the build" cargo build --release --workspace --all-targets
run "every arm against the reference" cargo test --release -p evs-support
run "the verdict" cargo test --release --bin engine_vs_std

checks_done
