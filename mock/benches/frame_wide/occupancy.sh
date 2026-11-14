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
# The figure covers the whole worker loop, the untimed bridge as well as the
# frame, and the bridge is single-threaded, so every parallel arm's figure is
# pulled toward one and not pulled equally: the dilution is heavier where the
# frame is shorter. Each figure is a floor on that arm's in-frame spread rather
# than a measure of it, and the ratio between two arms is not the ratio of
# their in-frame parallelism.
#
# usage: ./occupancy.sh <arm> [sweep] [n] [window-seconds]
#        ./occupancy.sh all [output-file]
#   ./occupancy.sh fw_engine_par
#   ./occupancy.sh fw_hand_par frame_wide/light 65536 20
#   ./occupancy.sh all                 # every arm, both regimes, to stdout
#   ./occupancy.sh all occupancy.txt   # to a file, which is what is committed
#
# `all` takes several samples an arm so the committed artifact carries a
# spread rather than one reading, and prints the median beside them. One
# sample is not enough to tell an arm that uses two cores from one that was
# measured next to a busy neighbour.
#
# The binaries come from a staged directory (`../stage.sh`), never from the
# shared target directory, which another worktree can rebuild under a fifteen
# minute sweep. `BENCH_BIN=<dir>` names one already staged; without it this
# stages the member itself. The checksums are checked before the first reading
# and after the last, and `all` writes the stage's `build.txt` into its header.

set -eu

# How long the worker gets to finish calibrating before the window opens.
settle=6
# Enough calls that the worker outlives the window at any arm's frame cost.
runs=200000
# How many readings `all` takes per arm and regime.
samples=3
# The default window, long enough that a tick of rounding does not show.
default_window=15

arms="fw_hand fw_hand_par fw_rust_nostd fw_rust_std fw_rust_std_par \
      fw_engine fw_engine_par"

here=$(cd "$(dirname "$0")" && pwd)
stage="$here/../stage.sh"
if [ -z "${BENCH_BIN:-}" ]; then
    BENCH_BIN=$("$stage" "$here")
fi
"$stage" --verify "$BENCH_BIN"
bin="$BENCH_BIN/release"
driver="$bin/frame_wide"

pid=
trap 'kill "$pid" 2>/dev/null || true' EXIT INT TERM

# Refuse before measuring anything, so a missing build is one message rather
# than one per arm.
require_built() {
    for f in "$@"; do
        if [ ! -f "$f" ]; then
            echo "occupancy: $f is not in the stage $BENCH_BIN" >&2
            exit 1
        fi
    done
}

# Run one arm for the window and print its cores figure, nothing else, so the
# caller can take a median of several.
cores_of() {
    _arm="$1"
    _sweep="$2"
    _n="$3"
    _window="$4"

    "$driver" --worker "$bin/lib$_arm.so" --bench-name "$_sweep" --mode warm \
        --runs "$runs" --batch 5 --n "$_n" --threaded >/dev/null 2>&1 &
    pid=$!

    sleep "$settle"
    if ! [ -r "/proc/$pid/stat" ]; then
        echo "occupancy: the worker for $_arm died before the window opened" >&2
        exit 1
    fi
    _before=$(awk '{print $14 + $15}' "/proc/$pid/stat")

    sleep "$_window"
    if ! [ -r "/proc/$pid/stat" ]; then
        echo "occupancy: the worker for $_arm died inside the window" >&2
        exit 1
    fi
    _after=$(awk '{print $14 + $15}' "/proc/$pid/stat")

    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    pid=

    awk -v b="$_before" -v a="$_after" -v w="$_window" \
        -v h="$(getconf CLK_TCK)" \
        'BEGIN { printf "%.2f\n", (a - b) / (w * h) }'
}

sweep_all() {
    n=65536
    window="$default_window"

    set -- $arms
    for a in "$@"; do
        require_built "$bin/lib$a.so"
    done
    require_built "$driver"

    echo "# Cores per arm, frame_wide, n=$n"
    echo "#"
    echo "# Process-wide CPU over a ${window}s window on a worker running one"
    echo "# arm's frame back to back, ${samples} readings an arm and regime."
    echo "# The window covers the untimed bridge as well as the frame, so each"
    echo "# figure is a floor on that arm's in-frame spread, not a measure of"
    echo "# it. Read ./occupancy.sh's header before quoting one."
    echo "#"
    echo "# Built: release, opt-level 3, fat LTO, one codegen unit."
    echo "# Measured: $(date -u '+%Y-%m-%d %H:%M UTC') on $(nproc) cores."
    echo "#"
    sed 's/^/# /' "$BENCH_BIN/build.txt"
    echo

    printf '%-16s %-18s %-24s %s\n' arm regime readings median
    for a in $arms; do
        for regime in heavy light; do
            readings=
            for _ in $(seq "$samples"); do
                r=$(cores_of "$a" "frame_wide/$regime" "$n" "$window")
                readings="$readings $r"
            done
            median=$(printf '%s\n' $readings | sort -n \
                | awk -v k=$(( (samples + 1) / 2 )) 'NR == k { print }')
            printf '%-16s %-18s %-24s %s\n' \
                "$a" "frame_wide/$regime" "$(echo $readings)" "$median"
        done
    done
    "$stage" --verify "$BENCH_BIN"
}

if [ "${1:-}" = all ]; then
    out="${2:--}"
    if [ "$out" = "-" ]; then
        sweep_all
    else
        sweep_all > "$out"
        echo "occupancy: wrote $out" >&2
    fi
    exit 0
fi

arm="${1:?usage: ./occupancy.sh <arm> [sweep] [n] [window-seconds] | all [out]}"
sweep="${2:-frame_wide/heavy}"
n="${3:-65536}"
window="${4:-$default_window}"

require_built "$driver" "$bin/lib$arm.so"
cores=$(cores_of "$arm" "$sweep" "$n" "$window")
"$stage" --verify "$BENCH_BIN"
printf '%-18s %-18s n=%-8s %s cores\n' "$arm" "$sweep" "$n" "$cores"
