#!/bin/sh
# Everything that has to pass before a timing from this member counts.
#
# This member declares its own `[workspace]`, as every bench member here does,
# so it is not among `mock/Cargo.toml`'s members and `cargo mock test` never
# builds it. This is the entry point instead: one command, from this directory,
# silent on success, and `../stage.sh` runs it before it builds this member for
# timing.
#
# Four checks, in the order a failure is cheapest to read. The grouping test is
# compile-time analysis and runs in milliseconds. The support crate's unit tests
# are in-process. The unstaged-driver test runs the built driver once from the
# target directory and costs a second. The negative control spawns harness
# workers and takes a couple of minutes, nearly all of it the per-worker
# counter calibration (ticket 3ec38cb5c7c8).
#
# usage: ./run-checks.sh [-v]
#   -v  print each command and its output rather than only failures

set -eu

cd "$(dirname "$0")"
. ../checks.sh

run "the build" cargo build --release --workspace --all-targets
run "the grouping test" cargo test --release --test engine_grouping
run "the support crate's unit tests" cargo test --release -p frame-wide-support
run "the unstaged driver refused" cargo test --release --test unstaged_driver
run "the negative control" cargo test --release --test negative_control

checks_done
