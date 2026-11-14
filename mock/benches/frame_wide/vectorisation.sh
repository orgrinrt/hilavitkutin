#!/bin/sh
# Which arms vectorise the kernel's multiply, read off the built code.
#
# "All three vectorise the multiply with SSE2 `pmuludq`" is a claim about
# emitted code, and prose asserting it with nothing behind it is worth less
# than nothing: a reader cannot check it and will not rebuild to try. This
# writes the artifact the claim rests on.
#
# Per arm it counts the SSE2 packed-multiply instructions in every symbol of
# the support crate, split by regime, then prints the instruction window around
# the first one so the artifact shows the shape and not only a tally. The body
# is behind a `dyn FrameArm` vtable, so it is its own symbol rather than inlined
# into `bench_entry`; the search is over the whole `.text` section rather than
# over a name guessed in advance. It sums over symbols rather than taking the
# busiest, because the collected shape puts part of its work in `Vec` iterator
# specialisations that land in symbols of their own. The regime is read off the
# symbol, which carries it: the kernel is a type parameter, so each regime is
# its own monomorphisation.
#
# The counts are of the arm as built, which means the build profile is part of
# the claim: release, opt-level 3, fat LTO, one codegen unit, no `target-cpu`
# flag, so baseline x86-64 and SSE2 without AVX2. A different profile is a
# different answer and this prints under which one it looked.
#
# usage: ./vectorisation.sh [output-file]
#   ./vectorisation.sh                      # to stdout
#   ./vectorisation.sh vectorisation.txt    # to a file, which is what is committed
#
# The arms are read from a staged directory (`../stage.sh`), never from the
# shared target directory another worktree can rebuild while this reads it.
# `BENCH_BIN=<dir>` names one already staged; without it this stages the member
# itself. The checksums are checked before the first arm and after the last,
# and the stage's `build.txt` goes into the report.

set -eu

out="${1:--}"
insn=pmuludq

here=$(cd "$(dirname "$0")" && pwd)
stage="$here/../stage.sh"
if [ -z "${BENCH_BIN:-}" ]; then
    BENCH_BIN=$("$stage" "$here")
fi
"$stage" --verify "$BENCH_BIN"
bin="$BENCH_BIN/release"

report() {
    echo "SSE2 packed multiply ($insn) in the frame body of each arm"
    echo
    echo "Built release, opt-level 3, lto fat, codegen-units 1, no target-cpu"
    echo "flag, so baseline x86-64: SSE2 is available and AVX2 is not used."
    echo "Produced by ./vectorisation.sh; rerun it to refresh."
    echo
    cat "$BENCH_BIN/build.txt"
    echo
    printf '%-16s %-7s %7s %8s\n' arm regime count symbols
    for arm in fw_hand fw_hand_par fw_rust_nostd fw_rust_std fw_rust_std_par \
               fw_engine fw_engine_par; do
        lib="$bin/lib$arm.so"
        if [ ! -f "$lib" ]; then
            echo "vectorisation: $lib is not built" >&2
            exit 1
        fi
        objdump -d --no-show-raw-insn "$lib" \
            | awk -v insn="$insn" -v arm="$arm" '
                /^[0-9a-f]+ </ { sym = $2 }
                # Only this bench's own code. A dependency emitting the
                # instruction somewhere else says nothing about the frame.
                $0 ~ insn && sym ~ /frame_wide_support/ {
                    regime = (sym ~ /Heavy/) ? "heavy" : ((sym ~ /Light/) ? "light" : "untagged")
                    c[regime]++
                    if (!(sym in seen)) { seen[sym] = 1; syms[regime]++ }
                }
                END { for (r in c) printf "%-16s %-7s %7d %8d\n", arm, r, c[r], syms[r] }' \
            | sort -k2,2
    done
    echo
    echo "An arm with no row for a regime emitted none of the instruction there."
    echo "An untagged row would be support-crate code carrying neither kernel in"
    echo "its symbol, which would mean the regime split above is incomplete."
    echo
    echo "The first occurrence in the heavy frame body of fw_hand, in context:"
    echo
    sym=$(objdump -d --no-show-raw-insn "$bin/libfw_hand.so" \
        | awk -v insn="$insn" '
            /^[0-9a-f]+ </ { sym = $2 }
            $0 ~ insn && sym ~ /frame_wide_support/ && sym ~ /Heavy/ { print sym; exit }')
    echo "  $sym"
    echo
    objdump -d --no-show-raw-insn "$bin/libfw_hand.so" \
        | awk -v insn="$insn" -v want="$sym" '
            /^[0-9a-f]+ </ { sym = $2 }
            sym == want { buf[NR] = $0; order[++n] = NR
                          if ($0 ~ insn && !hit) { hit = n } }
            END {
                if (!hit) exit
                lo = (hit - 6 > 1) ? hit - 6 : 1
                hi = (hit + 6 < n) ? hit + 6 : n
                for (i = lo; i <= hi; i++) print "  " buf[order[i]]
            }'
}

if [ "$out" = "-" ]; then
    report
else
    report > "$out"
fi
"$stage" --verify "$BENCH_BIN"
