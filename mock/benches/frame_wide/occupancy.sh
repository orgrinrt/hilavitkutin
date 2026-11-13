#!/bin/sh
# How many cores one arm of this bench actually runs on.
#
# A wall-clock median says how long a frame took, not how much machine it took
# to get there. Two arms at the same median are the same speed; if one of them
# used half the cores, that is the whole finding and the medians do not carry
# it. This prints cores for one arm, so the two numbers can be read together.
#
# The instrument is fields 14 and 15 of /proc/<tgid>/stat, utime and stime,
# which are process-wide: they include threads created and reaped inside the
# window. Per-thread sampling over /proc/<tgid>/task/*/stat is wrong for any
# arm that spawns per frame, because each sample sees a different short-lived
# set and a reaped thread's ticks are gone. On the run that first measured it,
# it put `hand_par` at 0.48 cores against this instrument's 2.16 on the same
# build, four and a half times low. Both are figures from before the arms were
# made fair to each other; the note's table carries the current ones.
#
# The arm is run as a harness timing worker, directly, with no orchestrator:
# that is one process running one arm's frame back to back, which is what the
# question is about. The worker sleeps about three seconds calibrating its
# counter before its first call, so the window opens after a settling delay
# rather than at the spawn.
#
# usage: ./occupancy.sh <arm> [sweep] [n] [window-seconds]
#   ./occupancy.sh fw_engine_par
#   ./occupancy.sh fw_hand_par frame_wide/light 65536 20
#
# Build the member first, from this directory:
#   CARGO_INCREMENTAL=0 cargo build --release --workspace

set -eu

arm="${1:?usage: ./occupancy.sh <arm> [sweep] [n] [window-seconds]}"
sweep="${2:-frame_wide/heavy}"
n="${3:-65536}"
window="${4:-15}"

# How long the worker gets to finish calibrating before the window opens.
settle=6
# Enough calls that the worker outlives the window at any arm's frame cost.
runs=200000

bin=$(cd "$(dirname "$0")" && cargo metadata --format-version 1 --no-deps \
    | sed 's/.*"target_directory":"\([^"]*\)".*/\1/')/release
driver="$bin/frame_wide"
dylib="$bin/lib$arm.so"

for f in "$driver" "$dylib"; do
    if [ ! -f "$f" ]; then
        echo "occupancy: $f is not built. Run:" >&2
        echo "  CARGO_INCREMENTAL=0 cargo build --release --workspace" >&2
        exit 1
    fi
done

"$driver" --worker "$dylib" --bench-name "$sweep" --mode warm \
    --runs "$runs" --batch 5 --n "$n" --threaded >/dev/null 2>&1 &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT INT TERM

sleep "$settle"
if ! [ -r "/proc/$pid/stat" ]; then
    echo "occupancy: the worker for $arm died before the window opened" >&2
    exit 1
fi

before=$(awk '{print $14 + $15}' "/proc/$pid/stat")
sleep "$window"
if ! [ -r "/proc/$pid/stat" ]; then
    echo "occupancy: the worker for $arm died inside the window" >&2
    exit 1
fi
after=$(awk '{print $14 + $15}' "/proc/$pid/stat")

awk -v b="$before" -v a="$after" -v w="$window" -v h="$(getconf CLK_TCK)" \
    -v arm="$arm" -v sweep="$sweep" -v n="$n" \
    'BEGIN { printf "%-18s %-18s n=%-8s %6d ticks / %ss = %.2f cores\n", \
             arm, sweep, n, a - b, w, (a - b) / (w * h) }'
