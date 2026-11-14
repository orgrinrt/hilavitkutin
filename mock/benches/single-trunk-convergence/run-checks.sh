#!/bin/sh
# Everything that has to pass before a timing from this member counts: the
# build of the driver and every arm, and the support crate's tests, which hold
# every arm's fold and append to the sequential reference and its splits to
# the records they claim to cover, and the driver's own tests.
#
# usage: ./run-checks.sh [-v]
#   -v  print each command and its output rather than only failures

set -eu

cd "$(dirname "$0")"
. ../checks.sh

run "the build" cargo build --release --workspace --all-targets
run "the support crate's tests" cargo test --release -p stc-support
run "the driver's tests" cargo test --release --bin single-trunk-convergence

checks_done
